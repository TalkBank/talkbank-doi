# Phase 3: Decommission git-talkbank

**Status:** Draft
**Last updated:** 2026-03-24 13:24 EDT

**Prerequisites:** Phases 1-2 complete and stable (all 24 data repos on GitHub, deploy pipeline working with GitHub remotes on git-talkbank).

**Goal:** Eliminate git-talkbank VM entirely. Replace the `deploy` script with pre-commit hooks (on user machines) and GitHub Actions (on talkbank.org). Move John's TBB app to talkbank.org.

---

## What `deploy` Does Today (and What Replaces It)

| Current (`deploy` on git-talkbank) | Replacement | Where it runs |
|-------------------------------------|-------------|---------------|
| `git pull` data repo | GitHub Actions `git pull` | talkbank.org (self-hosted runner) |
| rsync to `build/{bank}/data-orig/` | Eliminated — John reads repo clones directly | — |
| Update `@Types` headers, commit+push | Pre-commit hook on user machine | Brian/Davida's machine |
| Check duplicate DOIs (log only) | Pre-commit hook on user machine | Brian/Davida's machine |
| `generate_chat_data.py` → create ZIPs | Eliminated — John's app generates ZIPs dynamically | — |
| `generate_chat_data.py` → inject DOIs into `*-bank` HTML | Pre-commit hook on user machine (cross-repo) | Brian/Davida's machine |
| Push `*-bank` HTML changes to GitHub | Pre-commit hook auto-pushes `*-bank` repo | Brian/Davida's machine |
| TBB reads `build/{bank}/data-orig/` | TBB reads repo clones in `/var/data/` | talkbank.org |

---

## Standardized Workspace Layout

Everyone (Brian, Davida, Chen) has the same workspace structure:

```
~/0data/                          # All 24 data repos (flat)
    aphasia-data/
    childes-eng-na-data/
    childes-eng-uk-data/
    ...

~/0web/                           # All 17 web repos (flat)
    talkbank-web/
    childes-bank/
    aphasia-bank/
    ...
```

Pre-commit hooks in each `*-data` repo know where to find sibling repos:
- Sibling data repos: `../*-data/` (for duplicate DOI checking)
- Corresponding web repo: `~/0web/{bank}-bank/` (for DOI injection into HTML)

The bank name is derived from the repo name using the `DATA_REPO_TO_BANK` mapping (or a local equivalent).

**Setup script** (`scripts/setup-data-workspace.sh`) already clones all 24 data repos. Extend it to also clone web repos into `~/0web/`.

---

## Pre-commit Hooks

All hooks run on `git push` (pre-push hook, not pre-commit) so they execute once, not on every commit. This matches the current `deploy` behavior where the work happens on push.

### Hook 1: Update @Types

**What:** Read `0types.txt` files in the data repo, update `@Types:` headers in `.cha` files.

**Based on:** `staging/scripts/update_chat_types.py` (already works per-repo, no cross-repo dependency).

**Behavior:**
1. Walk repo tree, find `0types.txt` files
2. Update `@Types:` in all `.cha` files per inheritance rules
3. If any files changed: `git add` + `git commit --amend` (amend the push commit with @Types fixes)

**Cross-repo:** No — purely local to the data repo being pushed.

### Hook 2: Check Duplicate DOIs

**What:** Scan all `0metadata.cdc` files across ALL sibling `*-data` repos for duplicate DOI values.

**Based on:** `staging/hooks/tools/check-doi-duplicates` (already works with glob `*-data`, auto-discovers split repos).

**Behavior:**
1. Find parent dir of current repo
2. Glob `../*-data/` to find all sibling data repos
3. Grep for `^DOI:` in all `0metadata.cdc` files
4. If duplicates found: reject the push with error message

**Cross-repo:** Yes — reads (but does not modify) sibling data repos.

### Hook 3: Inject DOIs into Bank HTML

**What:** Read DOI values from `0metadata.cdc` files in the data repo, update corresponding HTML files in the `*-bank` web repo.

**Based on:** `staging/repos/generate-from-chat/generate_chat_data.py` DOI injection logic (lines 363-426).

**Behavior:**
1. Determine bank name from repo name (using `DATA_REPO_TO_BANK` equivalent)
2. Find corresponding web repo at `~/0web/{bank}-bank/`
3. For each corpus dir with `0metadata.cdc`:
   - Parse DOI value
   - Compute HTML path: `~/0web/{bank}-bank/site/access/{relative_path}.html`
   - Update `<td> DOI:</td> <td>...</td>` in the HTML
4. If any HTML files changed:
   - `git -C ~/0web/{bank}-bank add -A`
   - `git -C ~/0web/{bank}-bank commit -m "Update DOI from {repo_name}"`
   - `git -C ~/0web/{bank}-bank push`

**Cross-repo:** Yes — reads data repo, modifies+pushes web repo.

### Hook 4: Check for Large Files

**What:** Reject pushes containing files over a size threshold (e.g., 50 MB) to prevent accidental media commits.

**Based on:** `staging/hooks/tools/check-large-files` (already repo-agnostic).

**Behavior:**
1. Check staged files for size
2. Reject if any exceed threshold
3. Print helpful message about `.gitignore`

**Cross-repo:** No.

### Hook 5: Media Validation (`check-media`)

**What:** Validate CHAT transcript ↔ media file correspondence. Detects missing media, missing CHAT, case mismatches, wrong corpus names, bullet inconsistencies, missing `%pic` files.

**Replaces:** `staging/scripts/chatmedia.py` (Python, SSH-based, broken for split repos).

**Implementation:** Rust binary, currently at `scripts/check-media/` in talkbank-dev. Will be extracted to its own repo (`TalkBank/check-media`) before deployment. Already written, compiles, has typed diagnostics and JSON output. Key design change: uses a **cached media manifest** instead of SSH `find` on every run.

**Architecture:**
1. `check-media refresh-manifest` — SSHes to `net` once, builds `~/.cache/talkbank/media-manifest.json` listing all media files by bank. Run periodically (e.g., weekly cron) or manually before audits.
2. `check-media check <paths>` — reads CHAT files locally, cross-references against the cached manifest. No SSH. Fast enough for pre-push.
3. `check-media fix add-unlinked` / `fix fix-corpus` — mutation subcommands with `--dry-run`. Separated from checking (no implicit writes).

**Checks (7 kinds):**
| Check | What it detects |
|-------|-----------------|
| `missing-media` | CHAT references media not in manifest |
| `missing-chat` | Media exists with no CHAT file |
| `case-mismatch` | Media or CHAT filename differs only in case |
| `filename-match` | `@Media` name doesn't match CHAT basename |
| `bullet-consistency` | Bullets present but marked unlinked/notrans, or vice versa |
| `corpus-name` | `@ID` corpus field doesn't match directory structure |
| `pic` | `%pic` references nonexistent file |

**Cross-repo:** Yes — reads media manifest (covers all banks). But only checks CHAT files in the paths you give it, so as a pre-push hook it checks only the repo being pushed.

**Deployment:**
- Build: `cargo build --release -p check-media` (in `scripts/check-media/`)
- Install binary on Brian/Davida's machines and on talkbank.org
- Pre-push hook calls `check-media check . --manifest ~/.cache/talkbank/media-manifest.json --fail-on-error`
- GitHub Actions workflow can also run it post-push on talkbank.org (where manifest is refreshed by cron)

**Current status:** Core logic written. Needs: integration testing against real corpus data, manifest refresh validation, deployment scripts.

### Hook 6 (future): CHAT Validation

**What:** Validate `.cha` files using `talkbank-cli validate`.

**Not for Phase 3** — can be added later once talkbank-tools CLI is stable and distributed. Would run `chatter validate` on changed `.cha` files.

---

## GitHub Actions Workflows (on talkbank.org)

### Data Repo Workflows

Same pattern as existing web repo workflows. For each of the 24 data repos, add `.github/workflows/deploy.yml`:

```yaml
name: Deploy {repo}
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: [self-hosted, talkbank]
    steps:
      - name: Deploy
        run: |
          set -euo pipefail
          REPO=/var/data/{repo}
          git -C "$REPO" fetch --prune origin
          if ! git -C "$REPO" diff --quiet || ! git -C "$REPO" diff --cached --quiet; then
            git -C "$REPO" stash push -m "auto-deploy $(date -u +%Y-%m-%dT%H:%M:%SZ)" || true
          fi
          git -C "$REPO" checkout -f main
          git -C "$REPO" reset --hard origin/main
```

Use the talkbank-web style (fetch + reset --hard) rather than the simpler `git pull`, since pre-commit hooks may have amended commits that cause fast-forward issues.

### Web Repo Workflows

Already exist — 17 workflows doing `git pull` on talkbank.org. No changes needed.

---

## John's TBB App Migration

### Current State (git-talkbank)

- Reads from `~/staging/build/{bank}/data-orig/` and `~/staging/build/{bank}/data/`
- Dynamic ZIP generation ready but not enabled
- Auth via `sla2.talkbank.org:443`

### Target State (talkbank.org)

- Reads from `/var/data/*-data/` (repo clones, flat layout)
- Dynamic ZIP generation enabled (no pre-built ZIPs)
- Auth unchanged (still `sla2.talkbank.org:443`)
- Behind nginx reverse proxy at `talkbank.org/...`

### What John Needs

1. **Config mapping:** Bank name → list of repo directories (see `data-repo-structure-for-john.md`)
2. **Clone path:** `/var/data/` on talkbank.org (or wherever we decide)
3. **Nginx config:** New `location` block proxying to his Node app
4. **Password-protected corpora:** HomeBank split repos (`homebank-cougar-data`, etc.) may simplify access control since each access tier is now a separate repo

### Migration Steps

1. Clone all 24 data repos to `/var/data/` on talkbank.org
2. John deploys his Node app on talkbank.org, configured to read from `/var/data/`
3. Test internally (Tailscale access)
4. Add nginx reverse proxy config
5. Test with John's auth system
6. Add GitHub Actions workflows to data repos (push → `git pull` on talkbank.org)
7. Verify end-to-end: Brian pushes → Actions pulls → John's app sees update

---

## HTML URL Rewrite

After John's app is live on talkbank.org:

1. Determine exact URL scheme with John (e.g., `talkbank.org/TBB/childes/...`)
2. Bulk find-and-replace across all `*-bank` HTML files:
   - `https://git.talkbank.org/{bank}/data/` → `https://talkbank.org/TBB/{bank}/data/` (or whatever John's routes are)
3. Commit and push all `*-bank` repos
4. GitHub Actions pulls on talkbank.org → nginx serves updated HTML

---

## DOI System Rebuild

After the deploy pipeline is decommissioned, the DOI system needs a clean replacement.

### New DOI Tool (replaces `cdcs_to_csv.py` + 3 copies of `cdcfile.py`)

Single tool that:
1. Discovers all `0metadata.cdc` files across all 24 data repos
2. Compares against DataCite records (modern REST API, not curl)
3. Mints new DOIs for corpora without them
4. Updates DataCite when metadata or URLs change
5. Stores credentials securely (not plaintext Python)
6. Writes `output.csv` inventory

**Can be run from any machine** with access to the data repos (no longer requires git-talkbank).

### URL Changes at DataCite

DOI landing pages currently registered as `https://{bank}.talkbank.org/access/{path}.html`. These redirect to `https://talkbank.org/{bank}/access/{path}.html`. The redirects work, but eventually update the registered URLs at DataCite to use the canonical `talkbank.org` paths.

---

## Decommission git-talkbank VM

### Pre-checks

- [ ] All 24 data repos on GitHub, all remotes switched
- [ ] Brian/Davida pushing to GitHub, pre-commit hooks working
- [ ] GitHub Actions workflows pulling to talkbank.org
- [ ] John's app live on talkbank.org, reading repo clones
- [ ] HTML URLs rewritten, no links pointing to git.talkbank.org
- [ ] DOI tool working from a non-git-talkbank machine
- [ ] `output.csv` backed up
- [ ] No remaining services on git-talkbank that aren't replicated

### What's on git-talkbank that needs archiving

| What | Action |
|------|--------|
| `~/staging/` | Already in meta-repo (this workspace) |
| `~/staging/repos/` | Data repo clones — replaced by GitHub |
| `~/staging/build/` | Generated artifacts — no longer needed |
| `~/deploy.log` | Archive to `docs/legacy/` if useful |
| GitLab data (port 8929) | Repos migrated to GitHub; GitLab can be purged |
| John's Node app | Moved to talkbank.org |
| SSL certs | Let expire (already happening) |

### Shutdown

```bash
# After all pre-checks pass:
ssh macw@git-talkbank
sudo systemctl stop gitlab-runsvdir  # or however GitLab is managed
sudo apt purge gitlab-ee  # or gitlab-ce
# Request VM decommission from CMU
```

---

## Ordering / Dependencies

```
Phase 3 Step Dependencies:

1. Set up pre-commit hooks          ← can start immediately after Phase 2
   ├── Hook 1: @Types               ← independent
   ├── Hook 2: DOI duplicates       ← independent
   ├── Hook 3: DOI → HTML injection ← needs web repos cloned on user machines
   ├── Hook 4: Large file check     ← independent
   └── Hook 5: Media validation     ← needs cached manifest from net

2. GitHub Actions for data repos    ← needs repos cloned on talkbank.org
   └── Clone 24 repos to /var/data/ on talkbank.org

3. John's app migration             ← needs repos on talkbank.org (Step 2)
   ├── Deploy Node app
   ├── Nginx config
   └── Test

4. HTML URL rewrite                 ← needs John's app live (Step 3)

5. DOI system rebuild               ← independent, can start anytime

6. Decommission git-talkbank        ← needs ALL above complete
```

Steps 1, 2, and 5 can proceed in parallel. Step 3 depends on 2. Step 4 depends on 3. Step 6 is last.

---

## Risk Summary

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Pre-commit hook fails on Brian's machine | Medium | Low | Hook prints clear error, push is rejected, Brian asks for help |
| Cross-repo push from hook fails (network, auth) | Medium | Medium | Hook catches error, tells user to push `*-bank` repo manually |
| John's app migration breaks TBB | Low | High | Run in parallel (old app on git-talkbank, new on talkbank.org) until verified |
| DOI injection race condition (two pushers) | Low | Low | Only Brian/Davida push; split repos reduce overlap further |
| Something on git-talkbank we forgot about | Low | Medium | Keep VM running (powered off, not deleted) for 30 days after migration |
