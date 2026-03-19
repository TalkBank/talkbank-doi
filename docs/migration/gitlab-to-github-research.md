# GitLab-to-GitHub Migration: Research & Facts

**Status:** Historical (migration complete 2026-03-19)
**Last updated:** 2026-03-19

Research document for the migration of 16 data repos from GitLab to 24 repos on GitHub. **Migration completed 2026-03-19.** Phases 1+2 executed in a single window. GitLab purged. See `implementation-plan.md` for what was done, `phase3-plan.md` for next steps.

---

## Machines & Services

### git-talkbank (DECOMMISSIONING)

- **Cloud VM** on CMU Campus Cloud (Ubuntu)
- **Tailscale access:** `macw@git-talkbank`
- **Two distinct services on the same machine:**
  - **Port 8929:** ~~GitLab EE~~ **DELETED 2026-03-19** (`sudo apt purge gitlab-ee`)
  - **Port 80/443 (`https://git.talkbank.org`):** John Kowalski's Node.js web app — dynamic file serving, NOT GitLab. Intercepts web requests and serves data from `~/staging/build/`.
- **Also runs:** deploy pipeline (`~/staging/`), build output (`~/staging/build/`)
- **Status:** GitLab deleted. Deploy pipeline still runs here. VM still needed for John's app + deploy until Phase 3 completes.

**IMPORTANT:** `https://git.talkbank.org` (no port) is John's Node app, NOT GitLab. `https://git.talkbank.org:8929` is GitLab. These are two completely separate services.

### talkbank.org (CONSOLIDATION TARGET)

- **Formerly known as:** `homebank.talkbank.org`
- **Official name now:** `talkbank.org`
- **All bank subdomains** (`childes.talkbank.org`, `homebank.talkbank.org`, etc.) are nginx virtual hosts pointing here
- **Currently runs:** nginx (web repos), GitHub Actions self-hosted runner
- **Post-migration will also run:**
  - John's Node.js app (behind nginx reverse proxy, served at `talkbank.org/...`)
  - Clones of all GitHub data repos (for John's app to read)
- **Hardware:** Ubuntu VM on CMU Campus Cloud

### Other Machines (unchanged)

| Machine | Role | Notes |
|---------|------|-------|
| `net.talkbank.org` | Media source, batchalign server | Mac Studio M3 Ultra, 256 GB. Stays. |
| `media.talkbank.org` | Media file server (20 TB) | Red Hat, CMU Campus Cloud Plus. Stays. |
| `sla.talkbank.org` / `sla2.talkbank.org` | John's TalkBankDB + auth service | John-managed. Stays. |

---

## John Kowalski's Node App

### Current State (on git-talkbank)

- Runs on port 80/443 at `https://git.talkbank.org`
- Reads filesystem at `/home/macw/staging/build/{bank}/`:
  - `data-orig/` — raw CHAT files (rsync'd from data repos)
  - `data/` — pre-zipped files created by `generate_chat_data.py`
- Provides dynamic browsing, search, download of corpus data
- Auth backend at `https://sla2.talkbank.org:443`

### Post-Migration State (on talkbank.org)

- Will run behind nginx on `talkbank.org`, reverse-proxied at `talkbank.org/...`
- Reads GitHub data repo clones directly — **no `build/` directory, no `data-orig/` vs `data/` split**
- Repos can be in whatever structure they exist on GitHub
- **ZIP generation is dynamic** — John has already implemented on-the-fly ZIP creation in his Node app, ready to turn on
- **Our pre-built ZIPs are no longer needed**

### Implications

1. **`generate_chat_data.py` ZIP generation becomes unnecessary** — the entire ZIP pipeline goes away
2. **The `~/staging/build/` directory structure goes away**
3. **The deploy pipeline simplifies to:** `git push` → GitHub Actions → `git pull` on talkbank.org
4. **All download URLs** in `*-bank` HTML files must change from `https://git.talkbank.org/{bank}/data/*.zip` to `talkbank.org/...` (exact URL scheme TBD with John)

---

## Current Data Repos (16 on GitLab)

| Bank | Repo | Working Tree | .git | Notes |
|------|------|-------------|------|-------|
| aphasia | `aphasia-data` | — | — | |
| asd | `asd-data` | 95 MB | 52 MB | |
| biling | `biling-data` | 273 MB | 167 MB | |
| ca | `ca-data` | 5.1 GB | — | Split planned: CANDOR separate |
| childes | `childes-data` | 4 GB | — | Split planned: 4 language groups |
| class | `class-data` | 119 MB | 55 MB | |
| dementia | `dementia-data` | 1.0 GB | 937 MB | |
| fluency | `fluency-data` | 561 MB | 350 MB | |
| homebank | `homebank-data` | 13.6 GB | — | Split planned: 4 by access tier |
| motor | `motor-data` | 156 KB | 152 KB | |
| phon | `phon-data` | 5.5 GB | — | Split planned: 2 language groups |
| psychosis | `psychosis-data` | — | 2.4 GB | Bloated .git from historical media commits |
| rhd | `rhd-data` | 232 MB | 205 MB | |
| samtale | `samtale-data` | 5 MB | 2.6 MB | |
| slabank | `slabank-data` | 403 MB | 199 MB | |
| tbi | `tbi-data` | 144 MB | 87 MB | |

### Post-Split Plan (16 → 24 repos on GitHub)

**12 repos migrate 1:1:** aphasia, asd, biling, class, dementia, fluency, motor, psychosis, rhd, samtale, slabank, tbi

**4 repos split:**

| Original | Split Into | Reason |
|----------|-----------|--------|
| `childes-data` | `childes-eng-na-data`, `childes-eng-uk-data`, `childes-romance-germanic-data`, `childes-other-data` | 4 GB, 362 corpora across 23 language groups |
| `ca-data` | `ca-candor-data`, `ca-data` | CANDOR is 4.8 GB alone |
| `phon-data` | `phon-eng-french-data`, `phon-other-data` | 5.5 GB total |
| `homebank-data` | `homebank-public-data`, `homebank-cougar-data`, `homebank-bergelson-data`, `homebank-password-data` | 13.6 GB, access-tier separation |

---

## URL Changes Required

### Download URLs (in *-bank HTML files)

**Before:** `https://git.talkbank.org/{bank}/data/{corpus_path}.zip`
**After:** `talkbank.org/...` (exact scheme TBD — coordinate with John)

These URLs appear in:
- All `*-bank/site/access/**/*.html` files across 16 banks
- `generate-from-chat/test/pre/` and `test/post/` fixtures
- Possibly cached/indexed externally

### Other URL Changes

- All references to `git.talkbank.org` (non-port-8929) need to become `talkbank.org`
- GitLab clone URLs (`gitlab.talkbank.org:talkbank/*.git`) become GitHub (`github.com/TalkBank/*.git`)

---

## Systems Affected by Migration

### Must Be Updated (code changes required)

| System | Files | What Changes |
|--------|-------|-------------|
| **Makefile** | `GITLAB_HOST`, `DATA_REPOS`, `clone-data` | URL → GitHub, list → 24 repos |
| **staging/scripts/config.py** | `GIT_HOST`, `GIT_REPOS` | URL changes; may become obsolete |
| **staging/scripts/tasks.py** | `data_repo_name()`, `build_dir()`, `force_build()`, `build_data()` | **Cannot handle split repos.** Entire deploy pipeline replaced by GitHub Actions. |
| **generate-from-chat** | `generate_chat_data.py`, test fixtures | ZIP generation may become unnecessary; URL template changes |
| **cdcs-to-csv/metadatas.py** | Hardcoded 16-repo list | Needs 24-repo list or dynamic discovery |
| **webdev/config.toml** | Bank names list | May need sub-bank entries for splits |
| **All *-bank HTML files** | Download URLs | `git.talkbank.org` → `talkbank.org` |

### May Become Unnecessary

| System | Why |
|--------|-----|
| **deploy-1.py** | Replaced by GitHub Actions `git pull` |
| **generate_chat_data.py ZIP creation** | John's app generates ZIPs dynamically |
| **~/staging/build/ directory** | No longer needed; app reads repos directly |
| **ourlock.py** | No deploy lock needed if Actions handles it |

### Unaffected

| System | Why |
|--------|-----|
| **sync-media** | Media server is separate infrastructure |
| **gra-cgi** | CGI service, no data repo dependency |
| **update-chat-types** | Works on local repos regardless of remote |
| **talkbank-tools** | Self-contained; data paths are relative |
| **batchalign3** | No direct data repo dependency |
| **deploy/ansible/** | Dead code — never used in production. Aspirational Ansible config, out of scope for now. |

### Existing GitHub Actions Pattern (proven, reusable for data repos)

17 web repo workflows already deployed on `[self-hosted, talkbank]` runner on talkbank.org:

**Bank repos** (16 identical workflows, e.g. `web/banks/childes-bank/.github/workflows/deploy.yml`):
```yaml
name: Deploy childes
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: [self-hosted, talkbank]
    steps:
      - name: Deploy
        run: git -C /var/www/web/banks/childes-bank pull
```

**talkbank-web** (slightly more robust — stashes local edits, resets to origin/main):
```yaml
run: |
  set -euo pipefail
  REPO=/var/www/web/talkbank-web
  git -C "$REPO" fetch --prune origin
  if ! git -C "$REPO" diff --quiet || ! git -C "$REPO" diff --cached --quiet; then
    git -C "$REPO" stash push -m "auto-deploy $(date -u +%Y-%m-%dZ)" || true
  fi
  git -C "$REPO" checkout -f main
  git -C "$REPO" reset --hard origin/main
```

**For data repos post-migration:** Same pattern. Push to GitHub → self-hosted runner does `git pull` into a path John's app reads from. No Actions minute limits (self-hosted). No build step needed (John reads repos directly).

---

## Naming Convention: The 1:N Problem

The current system assumes **1 data repo per bank**. Functions that derive repo name from bank name by appending `-data` break when a bank has multiple repos.

### Already split-aware (partially updated)

- `config.DATA_REPO_TO_BANK` — maps split repo names to parent bank (e.g., `"childes-eng-na"` → `"childes"`)
- `config.data_repo_bank()` — lookup function for the above
- `tasks.force_build(repo_name)` (line 252) — strips `-data`, calls `data_repo_bank()` to get parent bank, then calls `build_data(bank)`. This part works.

### Breaks on splits (in `staging/scripts/tasks.py`)

| Function | Line | 1:1 Assumption | What Breaks |
|----------|------|----------------|-------------|
| `data_repo_name(site_name)` | 66 | Returns `f"{site_name}-data"` (ONE name) | After split, childes has 4 repo names |
| `data_repo_dir(has_data, site_name)` | 79 | Calls `data_repo_name()` → ONE path | Same |
| `build_data(name)` | 340 | `data_repo_dir(True, name)` → rsyncs ONE repo to `build/{bank}/data-orig/` | With splits, must rsync MULTIPLE repos into same `data-orig/` |
| `site_repo_name(name)` | 61 | Returns `f"{name}-site"` | Dead code (`*-site` repos no longer exist) but still called at line 376 |

### The core problem: `build_data()` merge step

`build_data(bank)` rsyncs a single data repo into `build/{bank}/data-orig/`. After splits:
- `build_data("childes")` needs to rsync 4 repos (`childes-eng-na-data`, `childes-eng-uk-data`, `childes-romance-germanic-data`, `childes-other-data`) into the same `build/childes/data-orig/`
- The rest of the pipeline (`generate_chat_data.py`, DOI injection) operates on the merged `data-orig/` dir, so those parts should work IF the merge is correct

### Other scripts with 1:1 assumptions

| Script | Assumption | Impact |
|--------|-----------|--------|
| `staging/scripts/dois.py` | Walks `config.GIT_REPOS` filtering `endswith("-data")` | Needs split repo names in GIT_REPOS |
| `cdcs-to-csv/metadatas.py` | Hardcoded 16-repo `urls` dict | Needs 24-repo dict with correct URL mapping |
| `Makefile` | `DATA_REPOS` list has 16 entries | Needs 24 entries |
| `Makefile clone-data` | Clones into `data/{repo}` flat | Split repos still need a flat structure |

### Complete Dependency Map

See **[dependency-map.md](dependency-map.md)** for the full function-by-function breakdown of what breaks, what works, what temporarily goes offline, dead code to remove, and new conventions/decisions needed.

### File-size audit (GitHub readiness)

- **100 MB hard limit**: Only one file is close — `childes-data/German/Leo/00errors.cex` at 94 MB. This is a CLAN error log, **should be .gitignored** (confirmed by user), not corpus data. Must be removed before migration.
- **All other files**: Under 25 MB. All repos are GitHub-ready from a file-size perspective with fresh `git init`.

---

## DOI System

### DOI Lifecycle Events

| Event | What Triggers It | What Code Runs | When |
|-------|-----------------|----------------|------|
| **New DOI minted** | Brian creates new corpus dir with `0metadata.cdc` that has no `DOI:` field | `cdcs_to_csv.py` → `datacite.mint()` → writes DOI back to `0metadata.cdc` → commits+pushes data repo | Chen manually runs on git-talkbank (infrequent) |
| **DOI URL updated at DataCite** | Brian moves/renames a corpus directory (path changes → URL changes) | `cdcs_to_csv.py` compares `record.target` vs `output.csv` → `datacite.update_url()` | Chen manually runs on git-talkbank |
| **DOI metadata updated at DataCite** | Title, creator, or date changes in `0metadata.cdc` | `cdcs_to_csv.py` compares fields vs `output.csv` → `datacite.update()` | Chen manually runs on git-talkbank |
| **DOI injected into HTML** | Any data repo deploy | `generate_chat_data.py` reads DOI from `0metadata.cdc`, regex-replaces `<td> DOI:</td> <td>...</td>` in `*-bank` HTML, commits+pushes web repo | **Every deploy** (automatic) |
| **Duplicate DOI check** | Any data repo deploy | `staging/scripts/dois.py` scans all repos for duplicate DOIs | **Every deploy** (automatic, log-only, never halts) |

**Critical insight:** The deploy pipeline touches DOIs on **every single deploy**, not just when `cdcs_to_csv.py` is run manually. HTML DOI injection via `generate_chat_data.py` happens automatically whenever `deploy` runs on git-talkbank.

### URL Construction (determines DOI landing pages)

`metadatas.py` maps repo name → domain, then derives URL from directory path:

```
{repo}/path/to/corpus/ → https://{domain}/access/path/to/corpus.html
```

Example: `aphasia-data/English/GR/` → `https://aphasia.talkbank.org/access/English/GR.html`

**Known bug:** `homebank-data` maps to `"talkbank"` (missing `.org`), producing malformed URLs in `output.csv`.

### Migration Impact

During migration, if we disable the deploy pipeline:
- No DOI-to-HTML propagation (OK if Brian doesn't add/move corpora)
- No duplicate checking on deploy (OK if pre-commit bash hook survives)
- Manual `cdcs_to_csv.py` still works independently (it does its own git pull/push)

**Post-migration DOI needs (must be rebuilt):**
1. DOI minting for new corpora (currently `cdcs_to_csv.py`)
2. DOI URL updates when corpora move (currently `cdcs_to_csv.py`)
3. DOI injection into `*-bank` HTML (currently `generate_chat_data.py` during deploy)
4. Duplicate DOI checking (currently `dois.py` + bash pre-commit hook)
5. URL mapping must be updated: `{bank}.talkbank.org` → `talkbank.org/{bank}` for DOI landing pages registered at DataCite

### Three Divergent Copies of cdcfile.py

| Location | Interface | Notes |
|----------|-----------|-------|
| `staging/scripts/cdcfile.py` | Pydantic `Info` model, `target` key | Deploy-time duplicate checking. `replace_doi()` is dead code here. |
| `staging/repos/generate-from-chat/cdcfile.py` | Dict, `_target` key (older) | HTML DOI injection. 2-space indent, no type hints. |
| `staging/repos/cdcs-to-csv/cdcfile.py` | Pydantic `Info` model, `target` key | DOI minting. **Local clone is stale; live code on git-talkbank.** |

### Known Bugs & Issues

- `cdcfile.py` line 82: `raise Exception("{input_path}: has no Date")` — missing `f` prefix
- `homebank-data` → `"talkbank"` URL mapping missing `.org`
- DOI `doi:` prefix stripped on parse, re-added on write (fragile round-trip across 3 codebases)
- XML built via f-string with no escaping (special chars in creator/title break XML)
- Bare `except:` in `cdcs_to_csv.py` silently skips corrupted CDC files
- DataCite API via `curl` subprocess with plaintext credentials
- `output.csv` is only inventory of historical DOI state (no backup/recovery mechanism)

**Plan:** Consolidate to single clean DOI tool post-migration. Modern DataCite REST API, credentials in secret store, proper XML escaping.

---

## Open Questions

1. ~~**Exact URL scheme for John's app on talkbank.org**~~ — Coordinate with John when the time comes. Depends on his Node app route structure.
2. ~~**GitHub Campus Program**~~ — Not eligible (CMU doesn't participate). All repos private, self-hosted runner (no minute limits). Already proven pattern with 17 web repo workflows.
3. ~~**Pilot repo**~~ — Moot. Can't do one repo at a time because even one split breaks all scripts. Must do all splits at once.
4. ~~**Timeline pressure**~~ — No hard deadline. SSL cert for GitLab web UI (port 8929) expires in days but SSH (port 29418) continues to work indefinitely. Nobody uses the web UI. Two-phase approach is viable: splits on GitLab first, then remote switch to GitHub.
5. ~~**Pre-commit hooks / @Types**~~ — `update_chat_types.py` is purely local (walks one directory tree, inherits `0types.txt` top-down). Splits are by top-level language dirs, so each split repo has its own complete subtree. No cross-repo dependency. Currently called during deploy (`tasks.py` line 359), not as a pre-commit hook — pre-commit is aspirational.
6. ~~**Deploy frequency**~~ — Mostly Brian and Davida. Chen can ask them to pause during migration windows. Easy to coordinate.
7. ~~**Git server software**~~ — GitLab. config.py comment saying "GitBucket" is 10-year-old stale comment. GitLab SSH on port 29418, web UI on port 8929.

---

## Decision Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-03-18 | talkbank.org is the official consolidated machine name | homebank.talkbank.org renamed; all banks are virtual hosts |
| 2026-03-18 | John's app will be reverse-proxied at talkbank.org/... | Nginx integration; all URLs become talkbank.org |
| 2026-03-18 | Pre-built ZIPs no longer needed post-migration | John's app does dynamic ZIP generation |
| 2026-03-18 | `generate_chat_data.py` ZIP pipeline can be retired | Follows from above |
| 2026-03-18 | DOI system: disable during migration, clean up after | DOI data is stable in `0metadata.cdc` files and at DataCite. Only operator (Chen) can simply not run `cdcs_to_csv.py` during migration. Cleaning up 3 tangled codebases while simultaneously replacing the deploy pipeline = two moving targets. Post-migration: build single clean DOI tool, modern DataCite REST API, GitHub Actions for duplicate checking, credentials out of plaintext. |
| 2026-03-18 | `staging/repos/cdcs-to-csv/` local clone is stale | Live code is on `macw@git-talkbank:~/staging/repos/cdcs-to-csv/`. Manual process: Chen SSHs in, runs `./cdcs_to_csv.py`. Does 16 pulls + 16 commit/push attempts even when nothing changed. |
| 2026-03-18 | HTML URL rewrite depends on John's app being live | Can't rewrite `git.talkbank.org` → `talkbank.org` URLs in `*-bank` HTML until John's Node app is actually running on talkbank.org. Ordering: (1) deploy John's app on talkbank.org, (2) rewrite URLs, (3) decommission git-talkbank. |
