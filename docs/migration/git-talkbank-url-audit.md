# git.talkbank.org URL Audit

**Status:** Current
**Last updated:** 2026-03-23 14:52 EDT

## Summary

807 HTML files across the live web content on talkbank.org contain hardcoded links
to `git.talkbank.org`. These must be rewritten before git.talkbank.org is
decommissioned.

## Link Inventory (884 total)

Counts by bank:

| Bank | Links |
|------|------:|
| childes | 365 |
| phon | 160 |
| aphasia | 86 |
| ca | 61 |
| slabank | 58 |
| class | 34 |
| dementia | 23 |
| fluency | 20 |
| homebank | 15 |
| asd | 14 |
| biling | 12 |
| samtale | 10 |
| tbi | 9 |
| rhd | 8 |
| psychosis | 8 |
| motor | 1 |
| **Total** | **884** |

## URL Patterns

Two patterns observed:

1. **`https://git.talkbank.org/{bank}/data/{path}.zip`** (808 links) — ZIP download
   links served by John's Node app. Example:
   ```
   https://git.talkbank.org/childes/data/Eng-NA/Bates.zip
   ```

2. **`https://git.talkbank.org/phon/phon/{path}.zip`** (76 links) — phon-bank has a
   second URL path (`/phon/phon/` in addition to `/phon/data/`). Example:
   ```
   https://git.talkbank.org/phon/phon/German/Bracci.zip
   ```

## Current State

- `/{bank}/data/` on talkbank.org currently returns **404** (nginx has no proxy config
  deployed yet; the template exists in `webdev/` but hasn't been pushed to the server)
- git.talkbank.org is still serving these URLs but is scheduled for decommission
- The nginx template in `webdev/src/templates/nginx.conf.j2` already has the proxy
  block for `/{bank}/data(-orig)?/` to John's Node app on port 4000

## Next Steps

### 1. Deploy nginx config with Node app proxy

Generate and deploy the nginx config from `webdev/`:

```bash
cd webdev && uv run generate-web-confs
# Then on talkbank.org:
sudo cp conf.d/talkbank.conf /etc/nginx/conf.d/
sudo nginx -t && sudo systemctl reload nginx
```

This will route `/{bank}/data/` and `/{bank}/data-orig/` to port 4000. Until John's
app is running, these will return 502 (no worse than the current 404).

### 2. Investigate phon-bank `/phon/` path

76 links use `/{bank}/phon/{path}.zip` instead of `/{bank}/data/{path}.zip`. Determine
whether John's app serves this path too or if the nginx proxy regex needs updating.
The current regex `^/(\w+)/data(-orig)?/` does NOT match `/phon/phon/`.

### 3. Global find-and-replace in web source repos

Replace `git.talkbank.org` with `talkbank.org` in all bank web repos. Do this in the
**source repos on GitHub** (not on the server), so changes deploy cleanly via GitHub
Actions.

The repos to update (under `web/` in the workspace, or `TalkBank/*-bank` on GitHub):

```
aphasia-bank    asd-bank      biling-bank   ca-bank
childes-bank    class-bank    dementia-bank fluency-bank
homebank-bank   motor-bank    phon-bank     psychosis-bank
rhd-bank        samtale-bank  slabank-bank  tbi-bank
```

Replacement: `https://git.talkbank.org/` -> `https://talkbank.org/`

This is a straight substitution — the URL path structure (`/{bank}/data/{path}.zip`)
is the same on both hosts.

**Verify before committing:** spot-check a few transformed URLs to make sure they
resolve correctly once the Node app is running.

### 4. Set up redirect on git.talkbank.org

After the URL rewrite is deployed, add a catch-all redirect on git.talkbank.org:

```nginx
server {
    listen 443 ssl;
    server_name git.talkbank.org;
    return 301 https://talkbank.org$request_uri;
}
```

This catches straggler bookmarks and crawlers. Keep the redirect running for a few
weeks before decommissioning the VM.

### 5. Decommission git.talkbank.org VM

After redirect period with no issues, delete the VM from CMU Campus Cloud.
