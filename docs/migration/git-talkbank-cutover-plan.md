# Plan: Migrate git.talkbank.org to talkbank.org and Decommission

**Status:** Draft
**Last updated:** 2026-03-23 14:52 EDT

## Context

git.talkbank.org is a CMU cloud VM we're paying for that ran two services:
1. **GitLab EE** (port 8929) — deleted 2026-03-19, data repos migrated to GitHub
2. **John's Node.js app** (ports 80/443) — file browsing and on-the-fly ZIP downloads for corpus data

With GitLab gone, the only reason git.talkbank.org exists is John's Node app. Moving it to talkbank.org and updating URLs lets us decommission the VM and stop paying for it.

**Machine topology:**
- **net** = Mac Studio in someone's office, private CMU LAN only. Media drives, batchalign, ML processing. NOT publicly accessible.
- **talkbank.org** = CMU Cloud VM, public-facing Ubuntu. Runs nginx (managed via webdev repo), serves all TalkBank websites.
- **git.talkbank.org** = CMU Cloud VM, being decommissioned. Currently runs only John's Node app.
- **talkbank-02** = CMU Campus Cloud Plus VM. Media server (`media.talkbank.org`).

These are four different machines. net is NOT talkbank.org.

**What John's app serves:**
- `https://git.talkbank.org/{bank}/data/{path}.zip` — on-the-fly ZIP downloads (e.g., `/childes/data/Biling/Bailleul.zip`)
- `https://git.talkbank.org/{bank}/data-orig/{path}` — raw CHAT file browsing/download

These URLs are embedded in bank website HTML pages (e.g., `talkbank.org/childes/access/Biling/Bailleul.html` has a "Download transcript" link to `git.talkbank.org/childes/data/Biling/Bailleul.zip`).

**On-the-fly ZIP generation is ready** — `generate_chat_data.py` and the pre-built `data/` directory are no longer needed. John's app generates ZIPs dynamically.

**Franklin already tested nginx proxying** (May 2025): Node app on port 3000 proxied at `talkbank.org/app/` via upstream + proxy_pass. Trivial.

---

## Solution: mergerfs virtual filesystem

John wants a flat `/{bank}/` directory per bank regardless of how many GitHub repos
back it. The old approach was rsync-copying all repos into a merged
`build/{bank}/data-orig/` tree — but that doubles disk space, requires a full
delete+rebuild on every deploy (creating a serving window where John's app 404s), and
can't handle renames/deletes without that rebuild.

**mergerfs** solves all of this. It's a FUSE (Filesystem in Userspace) union filesystem
that presents multiple source directories as a single merged mount point. Zero copies,
zero extra disk space, and changes from `git pull` are immediately visible (mergerfs
reads through to the source on every access — no caching).

```bash
# Install on talkbank.org
sudo apt install mergerfs

# Example: mount childes (4 repos merged into 1 view)
mergerfs \
  /var/data/repos/childes-eng-na-data:\
  /var/data/repos/childes-eng-uk-data:\
  /var/data/repos/childes-romance-germanic-data:\
  /var/data/repos/childes-other-data \
  /var/data/view/childes \
  -o defaults,allow_other,use_ino,cache.files=off,dropcacheonclose=true
```

John points his app at `/var/data/view/{bank}/` and sees:
```
/var/data/view/childes/
  Eng-NA/MacWhinney/foo.cha      <-- from childes-eng-na-data
  French/Lyon/bar.cha             <-- from childes-romance-germanic-data
  Chinese/Zhou/baz.cha            <-- from childes-other-data
```

**John changes zero lines of app code.** He just updates his data root path config.

One caveat: `.git` and `.gitignore` are visible in the merged view (the old rsync
approach excluded them). John is aware he may need to filter these from directory
listings.

### Why not OverlayFS?

OverlayFS is kernel-native (no FUSE) but its docs say "changes to the underlying
filesystems while part of a mounted overlay filesystem are not allowed." Since `git pull`
changes the underlying repos, OverlayFS would serve stale or undefined data. mergerfs is
designed for live, changing source directories.

### Why not rsync merge?

The old staging system merged split repos via `shutil.rmtree()` then rsync:
- **Doubles disk space** — full copy, not hard-linked to source (rsync `-H` only
  preserves hard links *within* a source tree, doesn't create links *to* the source)
- **Serving window** — between the delete and the rsync completing, John's app 404s
- **Can't handle renames/deletes** without a full rebuild (which causes the serving window)
- **Build step for successor** to understand and maintain

mergerfs has none of these problems.

---

## Step-by-Step Cutover Plan

### Phase 1: Prepare talkbank.org (Franklin)

**1.1 Clone data repos on talkbank.org**

Clone all 24 data repos to `/var/data/repos/` on talkbank.org:
```bash
ssh macw@talkbank.org
sudo mkdir -p /var/data/repos
cd /var/data/repos
for repo in aphasia-data asd-data biling-data ca-candor-data ca-data \
    childes-eng-na-data childes-eng-uk-data childes-romance-germanic-data \
    childes-other-data class-data dementia-data fluency-data \
    homebank-public-data homebank-cougar-data homebank-bergelson-data \
    homebank-password-data motor-data phon-eng-french-data phon-other-data \
    psychosis-data rhd-data samtale-data slabank-data tbi-data; do
  git clone git@github.com:TalkBank/$repo.git
done
```

**1.2 Install mergerfs and create mount points**

```bash
sudo apt install mergerfs
sudo mkdir -p /var/data/view/{aphasia,asd,biling,ca,childes,class,dementia,fluency,homebank,motor,open,phon,psyling,psychosis,rhd,samtale,slabank,tbi}
```

**1.3 Create mount script and fstab entries**

Create `/var/data/mount-views.sh` for initial setup and debugging:
```bash
#!/bin/bash
set -euo pipefail

REPOS=/var/data/repos
VIEW=/var/data/view
OPTS="defaults,allow_other,use_ino,cache.files=off,dropcacheonclose=true"

# Unsplit banks (1 repo each)
for bank in aphasia asd biling class dementia fluency motor open psyling psychosis rhd samtale slabank tbi; do
    mergerfs "$REPOS/${bank}-data" "$VIEW/$bank" -o "$OPTS"
done

# Split banks (multiple repos merged into one view)
mergerfs "$REPOS/ca-candor-data:$REPOS/ca-data" "$VIEW/ca" -o "$OPTS"
mergerfs "$REPOS/childes-eng-na-data:$REPOS/childes-eng-uk-data:$REPOS/childes-romance-germanic-data:$REPOS/childes-other-data" "$VIEW/childes" -o "$OPTS"
mergerfs "$REPOS/phon-eng-french-data:$REPOS/phon-other-data" "$VIEW/phon" -o "$OPTS"
mergerfs "$REPOS/homebank-public-data:$REPOS/homebank-cougar-data:$REPOS/homebank-bergelson-data:$REPOS/homebank-password-data" "$VIEW/homebank" -o "$OPTS"
```

Add to `/etc/fstab` for persistence across reboots:
```
# Split banks — mergerfs union mounts
/var/data/repos/ca-candor-data:/var/data/repos/ca-data /var/data/view/ca fuse.mergerfs defaults,allow_other,use_ino,cache.files=off,dropcacheonclose=true 0 0
/var/data/repos/childes-eng-na-data:/var/data/repos/childes-eng-uk-data:/var/data/repos/childes-romance-germanic-data:/var/data/repos/childes-other-data /var/data/view/childes fuse.mergerfs defaults,allow_other,use_ino,cache.files=off,dropcacheonclose=true 0 0
/var/data/repos/phon-eng-french-data:/var/data/repos/phon-other-data /var/data/view/phon fuse.mergerfs defaults,allow_other,use_ino,cache.files=off,dropcacheonclose=true 0 0
/var/data/repos/homebank-public-data:/var/data/repos/homebank-cougar-data:/var/data/repos/homebank-bergelson-data:/var/data/repos/homebank-password-data /var/data/view/homebank fuse.mergerfs defaults,allow_other,use_ino,cache.files=off,dropcacheonclose=true 0 0

# Unsplit banks (one line per bank, same pattern with single source)
/var/data/repos/aphasia-data /var/data/view/aphasia fuse.mergerfs defaults,allow_other,use_ino,cache.files=off,dropcacheonclose=true 0 0
# ... (14 more unsplit banks)
```

John's app reads from `/var/data/view/{bank}/` — a flat merged view per bank.

**1.4 Set up GitHub Actions self-hosted runner**

On talkbank.org, install a GitHub Actions self-hosted runner. On push to any data repo:
1. `git pull` the updated repo in `/var/data/repos/`
2. mergerfs immediately shows the changes (reads through to source, no rebuild needed)

**1.5 Add nginx proxy route**

Update `webdev/src/templates/nginx.conf.j2`:

```nginx
upstream node_data_backend {
    server 127.0.0.1:4000 max_fails=3 fail_timeout=30s;
}
```

John's app handles `/{bank}/data/...` and `/{bank}/data-orig/...`. These paths need
to be proxied to his Node app without conflicting with the existing static bank website
routes (which serve `/{bank}/...` from the filesystem).

The nginx routing needs to distinguish:
- `talkbank.org/childes/access/...` — static files (existing, served by nginx)
- `talkbank.org/childes/data/...` — John's Node app (new proxy)
- `talkbank.org/childes/data-orig/...` — John's Node app (new proxy)

```nginx
# Proxy data/data-orig requests to John's Node app
location ~ ^/(\w+)/data(-orig)?/ {
    proxy_pass http://node_data_backend;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_connect_timeout 60s;
    proxy_send_timeout 300s;
    proxy_read_timeout 300s;
}
```

Longer timeouts for `proxy_read_timeout` because ZIP generation may take time for
large corpora.

Regenerate and deploy:
```bash
cd ~/webdev && uv run generate-web-confs
sudo cp conf.d/talkbank.conf /etc/nginx/conf.d/
sudo nginx -t && sudo systemctl reload nginx
```

### Phase 2: John deploys his app on talkbank.org (John, with Franklin support)

**2.1** John clones his app repo on talkbank.org, installs Node.js dependencies.

**2.2** John configures the app:
- Data root: `/var/data/view/` (mergerfs-mounted, flat merged view per bank)
- Port: 4000 (proxied by nginx)

**2.3** Set up as a systemd service so it survives reboots.

**2.4** John verifies auth (sla2.talkbank.org) still works from the new location.

**2.5** Test all critical flows:
- Browse public corpus data-orig (e.g., `/childes/data-orig/Eng-NA/MacWhinney/`)
- Download ZIP (e.g., `/childes/data/Biling/Bailleul.zip`)
- Password-protected corpora (homebank)
- All 16 banks

### Phase 3: URL cutover (Franklin)

**3.1** Find all `git.talkbank.org` URLs in bank web repos:
```bash
grep -r "git.talkbank.org" web/banks/*/site/
```

**3.2** Replace `https://git.talkbank.org/` with `https://talkbank.org/` in all HTML
files. The paths stay the same (`/childes/data/Biling/Bailleul.zip`), only the hostname
changes. This is a bulk find-and-replace across all bank web repos.

**3.3** Commit and push all affected bank repos.

**3.4** Set up temporary redirect on git.talkbank.org:
```nginx
server {
    listen 80;
    listen 443 ssl;
    server_name git.talkbank.org;
    return 301 https://talkbank.org$request_uri;
}
```

### Phase 4: Decommission git.talkbank.org (Franklin)

**4.1** Let the redirect run for 2-4 weeks. Monitor access logs for stragglers.

**4.2** Verify nothing else depends on git.talkbank.org:
- Remove firewall rule: `git.talkbank.org → talkbank-02 SSH`
- Update `staging/` deploy scripts to not reference git.talkbank.org
- Search all repos for remaining references

**4.3** Delete the VM. Remove DNS records. Update docs.

---

## Files to Create/Modify

| File | Action |
|------|--------|
| `/var/data/mount-views.sh` (on talkbank.org) | Create — mergerfs mount script |
| `/etc/fstab` (on talkbank.org) | Add mergerfs entries for all 16 banks |
| `webdev/src/templates/nginx.conf.j2` | Modify — add data proxy route |
| `webdev/conf.d/talkbank.conf` | Regenerated |
| All `*-bank` web repos (HTML files) | Find-and-replace: `git.talkbank.org` → `talkbank.org` |
| `staging/scripts/config.py` | Remove git-talkbank host references |
| `staging/deploy` | Update to not SSH to git-talkbank |
| `docs/inventory.md` | Update: git.talkbank.org decommissioned |
| `CLAUDE.md` | Fix server table (net and talkbank.org are different machines) |

---

## Verification

1. `https://talkbank.org/childes/data/Biling/Bailleul.zip` downloads correctly
2. `https://talkbank.org/childes/data-orig/Eng-NA/MacWhinney/` browses correctly
3. All 16 banks work (both data/ and data-orig/ paths)
4. Password-protected corpora require auth
5. `https://git.talkbank.org/childes/data/...` redirects to `https://talkbank.org/childes/data/...`
6. Existing static bank pages (`talkbank.org/childes/access/...`) still work (no nginx conflict)
7. After 2-4 weeks: delete git.talkbank.org VM, clean up DNS and firewall rules
8. All docs updated with correct machine topology
