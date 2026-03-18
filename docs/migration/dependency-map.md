# Complete Dependency Map: Data Repo Split & GitHub Migration

**Status:** Draft
**Last updated:** 2026-03-18

Every script, function, config entry, and hardcoded assumption that references data repos, bank names, or the 1:1 bank-to-repo mapping. Organized by what breaks, what works, and what temporarily goes offline.

---

## The Deploy Call Graph

```
User runs `deploy` in a data repo working directory (e.g., childes-data/)
    │
    ▼
staging/deploy (bash wrapper)
    │  Extracts repo name from: git config --get remote.origin.url | basename
    │  e.g., "childes-data" or (post-split) "childes-eng-na-data"
    │
    ▼
ssh macw@git-talkbank deploy-1.py --repo {repo_name}
    │
    ▼
deploy-1.py:main()
    │  Validates: repo_name in config.GIT_REPOS     ← must contain split repo names
    │
    ▼
tasks.do_git_repo(repo_name)
    ├── pull_got_new(repo_name)                      ← works (just git pull)
    ├── force_build(repo_name)                       ← PARTIALLY works (see below)
    │       │
    │       │  Strips "-data" suffix
    │       │  bank = config.data_repo_bank(name)    ← works (DATA_REPO_TO_BANK)
    │       │
    │       ▼
    │   build_data(bank)                             ← ✗ BROKEN (assumes 1 repo per bank)
    │       ├── data_repo_dir(True, bank)            ← ✗ returns ONE path
    │       ├── rsync ONE repo → build/{bank}/data-orig/  ← ✗ only one repo
    │       ├── dois.check_duplicates()              ← ✓ works (scans all *-data in GIT_REPOS)
    │       ├── update_chat_types(local_data_dir)    ← ✗ only one repo's tree
    │       ├── generate_chat_data.py --base --access ← ✓ works IF data-orig/ is correctly populated
    │       └── site_repo_dir(True, bank)            ← ✓ works (web repos stay 1:1)
    │
    └── force_deploy(repo_name)                      ← ✓ works (data repos: "no deploy needed")
```

---

## Category 1: BREAKS on Split — Must Fix Before Migration

### 1.1 `tasks.data_repo_name(site_name)` — tasks.py:66

```python
def data_repo_name(site_name: str) -> str:
    return f"{site_name}{config.DATA_SUFFIX}"
    # "childes" → "childes-data" — WRONG: should be 4 repos
```

**Called by:** `data_repo_dir()` (line 82)
**Fix needed:** Replace with a function that returns a LIST of repo names for a bank.

### 1.2 `tasks.data_repo_dir(has_data, site_name)` — tasks.py:79

```python
def data_repo_dir(has_data: bool, site_name: str) -> str:
    if has_data:
        repo_name = data_repo_name(site_name)  # ONE name
        return repo_path(repo_name)             # ONE path
```

**Called by:** `build_data()` (line 340), `chatmedia.check_bank()` (line 305)
**Fix needed:** Return a LIST of paths, or accept a specific repo name instead of bank name.

### 1.3 `tasks.build_data(name)` — tasks.py:323

The core of the problem. Takes a bank name, operates on ONE repo.

| Line | What it does | 1:1 assumption |
|------|-------------|----------------|
| 340 | `local_data_dir = data_repo_dir(True, name)` | Gets ONE repo path |
| 341 | `data_orig_clone = build_dir(name) + "/data-orig"` | ONE build dir (OK — bank-level) |
| 344-353 | rsync `local_data_dir/` → `data_orig_clone/` | Copies ONE repo |
| 359 | `update_chat_types(local_data_dir, False)` | Updates ONE repo |
| 362-364 | git commit+push if @Types changed | Commits to ONE repo |
| 373 | `site_repo_dir(True, name)` → web repo path | OK (web repos stay 1:1) |
| 379-383 | Run `generate_chat_data.py --base={build_dir} --access={site_dir}/access` | OK (operates on merged data-orig/) |

**Fix needed:** For split banks, iterate over all data repos for that bank, rsync each into the same `data-orig/` directory, run @Types on each repo individually, commit+push each.

### 1.4 `tasks.site_repo_name(name)` — tasks.py:61

```python
def site_repo_name(site_name: str) -> str:
    return f"{site_name}{config.SITE_SUFFIX}"  # "childes" → "childes-site"
```

**Dead code** — `*-site` repos no longer exist. Still called at line 376. Should be removed.

### 1.5 `chatmedia.check_bank(bank, path, ...)` — chatmedia.py:293

```python
repo_path: str = tasks.data_repo_dir(True, config.host_to_site_name(bank))
# Gets ONE data repo path — misses split repos
```

**Impact:** `check_media --bank childes` only validates media for `childes-data`, ignoring all 4 split repos.
**Fix needed:** Iterate over all data repos for the bank.

### 1.6 `cdcs-to-csv/metadatas.py` — lines 13-30

Hardcoded 16-repo dict:
```python
urls: dict[str, str] = {
    "childes-data": "childes.talkbank.org",
    # ... 15 more, NO split repos
}
```

**Impact:** DOI minting/updating completely ignores split repos. `get_dict()` won't discover `0metadata.cdc` files in split repos. `update_git()` and `commit_git()` won't pull/push split repos.
**Fix needed:** Add split repo entries with correct URL mapping. Multiple split repos map to the same bank domain (e.g., `"childes-eng-na-data": "childes.talkbank.org"`).

### 1.7 `Makefile` `DATA_REPOS` list — line 32

```makefile
DATA_REPOS = aphasia-data asd-data ... tbi-data  # 16 repos, hardcoded
```

**Impact:** `make clone-data` won't clone split repos.
**Fix needed:** Update to 24 repos. Also update `GITLAB_HOST` → GitHub URL when remotes switch.

### 1.8 `config.GIT_REPOS` dict — config.py:153

Must add entries for all split repos:
```python
"childes-eng-na-data": RepoInfo(url="...", root_dir="", git_clean=True),
"childes-eng-uk-data": RepoInfo(url="...", root_dir="", git_clean=True),
# etc.
```

And remove the old unsplit entries (`"childes-data"`, `"ca-data"`, `"phon-data"`, `"homebank-data"`).

### 1.9 `config.HOSTS` dict — config.py:380

Currently: `"childes": HostInfo(repos=["childes-data", "childes-site"], ...)`
Needs: `"childes": HostInfo(repos=["childes-eng-na-data", "childes-eng-uk-data", "childes-romance-germanic-data", "childes-other-data"], ...)`

Note: The `repos` field is already a list — it just needs to be populated correctly. Remove `*-site` entries (dead).

---

## Category 2: WORKS with Split Repos — No Changes Needed

### 2.1 `deploy` bash wrapper — staging/deploy

Extracts repo name from git origin URL. If a user is in `childes-eng-na-data/` and pushes, the wrapper gets `"childes-eng-na-data"` and passes it through. No change needed.

### 2.2 `deploy-1.py` — staging/scripts/deploy-1.py

Receives `--repo` argument, validates against `config.GIT_REPOS`, calls `do_git_repo()`. Works as long as split repos are added to `GIT_REPOS`.

### 2.3 `tasks.do_git_repo(repo_name)` — tasks.py:132

Pulls the specific repo and calls `force_build()`. Works with any repo name.

### 2.4 `tasks.force_build(repo_name)` — tasks.py:242

Already split-aware! Strips `-data`, calls `config.data_repo_bank()` to get parent bank. The problem is downstream in `build_data()`, not here.

### 2.5 `config.data_repo_bank()` — config.py:633

Designed for splits. `DATA_REPO_TO_BANK` already has all 12 split mappings.

### 2.6 `dois.check_duplicates()` — dois.py:17

Iterates ALL repos in `config.GIT_REPOS` that end with `-data`. Auto-discovers split repos if they're added to `GIT_REPOS`.

### 2.7 `generate_chat_data.py` — generate-from-chat

Takes `--base` and `--access` paths. Walks directory tree looking for `0metadata.cdc` files. **No bank name or repo name awareness at all.** Works as long as `data-orig/` is correctly populated with the merged contents.

### 2.8 `update_chat_types.py` — staging/scripts/update_chat_types.py

Takes a directory path, walks it. No repo name awareness. Works on any directory tree.

### 2.9 Pre-commit hooks — staging/hooks/tools/

- `check-doi-duplicates`: Uses glob `*-data` to discover sibling repos. **Auto-discovers splits.**
- `validate-metadata`: Walks `REPO_ROOT`. Repo-agnostic.
- `generate-manifest`: Walks `REPO_ROOT`. Repo-agnostic.
- `check-large-files`: Checks staged files. Repo-agnostic.

### 2.10 `tasks.pull_got_new(repo_name)` — tasks.py:175

Git pull/clone for any repo name. No naming assumptions.

### 2.11 `tasks.build_dir(site_name)` — tasks.py:93

Returns `~/staging/build/{bank}/`. Bank-level, not repo-level. Correct — the build output is per-bank regardless of how many repos feed it.

### 2.12 `tasks.site_repo_dir(has_data, site_name)` — tasks.py:71

Returns `/mnt/homebank_web/banks/{bank}-bank/site`. Web repos stay 1:1. No change needed.

### 2.13 Web repo workflows — web/banks/*/deploy.yml

GitHub Actions `git pull` on talkbank.org. No data repo awareness. Unaffected.

### 2.14 Nginx config — webdev/

Regex-based redirects. Bank names only, no data repo awareness. `config.toml` lists bank names (not repo names). Unaffected by data repo splits.

---

## Category 3: TEMPORARILY BREAKS During Migration

These systems will be offline or degraded during the migration window. They can be restored afterward.

### 3.1 DOI Minting (`cdcs_to_csv.py`)

**When it breaks:** As soon as repos are split on GitLab (or moved to GitHub).
**What breaks:** `metadatas.py` has hardcoded 16-repo list. Split repos invisible. Cannot mint DOIs for corpora in split repos, cannot update URLs when corpora move.
**Duration:** Until `metadatas.py` is updated with 24-repo list (or replaced entirely post-migration).
**Risk:** LOW if Chen doesn't run `cdcs_to_csv.py` during migration. Brian must not add new corpora that need DOIs during this window.
**Mitigation:** Tell Brian not to create new corpus dirs during migration.

### 3.2 DOI Injection into HTML (`generate_chat_data.py` via deploy)

**When it breaks:** When the deploy pipeline on git-talkbank is disabled or `build_data()` fails for split repos.
**What breaks:** DOI values from `0metadata.cdc` won't be injected into `*-bank` HTML access pages.
**Duration:** Until deploy pipeline is replaced by GitHub Actions.
**Risk:** LOW. Existing DOI values in HTML are already correct. Only new/changed DOIs would be missed.
**Mitigation:** Same as above — no new corpus dirs during migration.

### 3.3 Duplicate DOI Checking on Deploy (`dois.py`)

**When it breaks:** When deploy pipeline is disabled.
**What breaks:** No automated duplicate DOI detection on push.
**Duration:** Until GitHub Actions or pre-commit hooks replace it.
**Risk:** LOW. The bash `check-doi-duplicates` pre-commit hook is independent and works with splits.

### 3.4 @Types Header Updates on Deploy (`update_chat_types.py`)

**When it breaks:** When `build_data()` fails for split repos.
**What breaks:** `@Types` headers in `.cha` files won't be auto-updated on push.
**Duration:** Until deploy pipeline is fixed for splits or replaced.
**Risk:** MEDIUM. Brian/Davida expect `@Types` to be updated on deploy. They need to know this won't happen during migration.
**Mitigation:** Run `update_chat_types.py --chatdir {repo}` manually if needed, or tell Brian to wait.

### 3.5 Media Validation (`check_media`)

**When it breaks:** When `chatmedia.check_bank()` can't resolve split repos.
**What breaks:** `check_media --bank childes` only checks the old unsplit repo path.
**Duration:** Until `check_bank()` is updated to iterate split repos.
**Risk:** LOW. Media validation is already a known mess and isn't run frequently.

### 3.6 ZIP File Generation (`generate_chat_data.py`)

**When it breaks:** When the deploy pipeline is disabled.
**What breaks:** New/changed corpus data won't be zipped for download.
**Duration:** Until John's app on talkbank.org generates ZIPs dynamically (Phase 2).
**Risk:** LOW during migration. Existing ZIPs in `build/{bank}/data/` continue to be served.

---

## Category 4: DEAD CODE — Can Be Removed

| Code | Location | Why Dead |
|------|----------|----------|
| `site_repo_name()` | tasks.py:61 | `*-site` repos don't exist |
| `*-site` entries in `GIT_REPOS` | config.py | Dead repos, never pulled/deployed |
| `*-site` entries in `HOSTS.repos` | config.py | Same |
| `SITE_SUFFIX` constant | config.py:114 | Only used by dead `site_repo_name()` |
| `cdcs-to-csv` entry in `GIT_REPOS` | config.py:209 | Points to Bitbucket (stale decade-old reference) |
| `"GitBucket"` comments | config.py:53,56 | GitBucket replaced by GitLab ~10 years ago |
| `ADMINISTRATORS_GITBUCKET_BASE` | config.py:58 | Same |
| `CapVid` entry in `GIT_REPOS` | config.py:154 | Verify if still used |
| `check-chat-bookmarks` entry | config.py:215 | Verify if still used |
| Apache conf generation code | Various | Nginx replaced Apache |

---

## Category 5: NEEDS NEW CONVENTIONS / DECISIONS

### Q1: Where do split repos live on git-talkbank during Phase 1?

Currently data repos are at `~/staging/repos/{bank}-data/`. After splits:
- Option A: `~/staging/repos/childes-eng-na-data/` (flat, same as today)
- Option B: `~/staging/repos/childes/eng-na-data/` (nested under bank)

The flat approach (A) is simpler and matches how `GIT_REPOS` keys work. **Recommend A.**

**DECIDED 2026-03-18:** Flat layout. All data repos as siblings in the same directory, everywhere (git-talkbank, talkbank.org, local dev). Enables `*-data` glob discovery.

### Q2: How does `build_data()` know which repos belong to a bank?

Currently uses `data_repo_name(bank)` → one name. Needs a new function like:

```python
def data_repo_names(bank: str) -> List[str]:
    """Return all data repo names for a bank."""
    return [name for name, b in DATA_REPO_TO_BANK.items() if b == bank] + \
           ([f"{bank}-data"] if bank not in DATA_REPO_TO_BANK.values() else [])
```

Or add a forward mapping to config:
```python
BANK_TO_DATA_REPOS: Dict[str, List[str]] = {
    "childes": ["childes-eng-na-data", "childes-eng-uk-data", "childes-romance-germanic-data", "childes-other-data"],
    "ca": ["ca-candor-data", "ca-data"],
    "phon": ["phon-eng-french-data", "phon-other-data"],
    "homebank": ["homebank-public-data", "homebank-cougar-data", "homebank-bergelson-data", "homebank-password-data"],
    # unsplit banks: auto-generate from name
}
```

**DECIDED 2026-03-18:** Explicit forward mapping (option B). `BANK_TO_DATA_REPOS` dict listing all repos per bank, including single-repo banks.

### Q3: How does `build_data()` merge multiple repos into `data-orig/`?

Currently: rsync ONE repo → `build/{bank}/data-orig/`
After split: rsync EACH split repo → `build/{bank}/data-orig/{language-group}/`

But wait — the split repos' top-level directories ARE the language groups. So rsync each split repo's contents into `data-orig/` and they naturally merge (no collisions since each split repo contains different top-level dirs).

Example for childes:
```
childes-eng-na-data/Eng-NA/...     → build/childes/data-orig/Eng-NA/...
childes-eng-uk-data/Eng-UK/...     → build/childes/data-orig/Eng-UK/...
childes-romance-germanic-data/...  → build/childes/data-orig/French/... German/... etc.
childes-other-data/Biling/...      → build/childes/data-orig/Biling/... Chinese/... etc.
```

**DECIDED 2026-03-18:** Yes — split repos contain language-group directories at top level. rsync merge into `data-orig/` produces identical structure to what exists today. John's app sees no difference.

### Q4: What happens to the old unsplit repos on GitLab?

After split repos are created and verified:
- Archive the old `childes-data`, `ca-data`, `phon-data`, `homebank-data` on GitLab?
- Delete them immediately?
- Keep them as read-only mirrors temporarily?

**DECIDED 2026-03-18:** Keep old unsplit repos read-only on GitLab as safety net until GitHub migration is complete and verified. Then delete.

### Q5: How do Brian and Davida know which split repo to push to?

Today: Brian works in `childes-data/Eng-NA/MacWhinney/`, pushes to `childes-data`.
After split: Brian works in `childes-eng-na-data/Eng-NA/MacWhinney/`, pushes to `childes-eng-na-data`.

They need to know:
- Each language group is now its own repo
- `git push` from the right directory

**DECIDED 2026-03-18:** Create a standardized setup script that clones all 24 data repos into a workspace (currently people use `~/0data/`). Run the script on each person's machine to set them up fresh. Inform everyone of the split. They already know the directory structure — just new repo boundaries.
- They cannot `cd` from `Eng-NA/` to `French/` within the same repo anymore

**Decision needed:** How to communicate this to Brian and Davida. A short email? A meeting?

### Q6: `sync-media/sync_media.py` — Does it need split awareness?

Has hardcoded bank list (17 banks). Syncs media by bank name, not by repo name. Since media organization is by bank (not by language group within a bank), this probably doesn't need to change.

**DECIDED 2026-03-18:** Confirmed — media on `net` is organized by bank (`/Users/macw/media/childes/`, etc.). `sync-media` is unaffected by data repo splits.

### Q7: What URL does `cdcs-to-csv/metadatas.py` construct for split repos?

Currently: `"childes-data"` → `https://childes.talkbank.org/access/{relative_path}.html`

After split: `"childes-eng-na-data"` should still map to `https://childes.talkbank.org/access/{relative_path}.html` (same bank domain).

So multiple split repos map to the same bank domain. The URL construction just needs the bank domain, which comes from `DATA_REPO_TO_BANK`.

**No decision needed** — the mapping is straightforward. Just needs implementation.

### Q8: Git clone URLs for GitHub repos

Currently: `ssh://git@git-talkbank:29418/TalkBank/{repo}.git`
After: `git@github.com:TalkBank/{repo}.git` (or `https://github.com/TalkBank/{repo}.git`)

Both `config.GIT_REPOS` URLs and `Makefile` `GITLAB_HOST` need updating.

**DECIDED 2026-03-18:** SSH (`git@github.com:TalkBank/{repo}.git`). talkbank.org already uses SSH keys for web repos. Verified: `childes-bank` remote is `git@github.com:TalkBank/childes-bank`. Same pattern for data repos.

---

## Complete Inventory: Every Hardcoded Repo/Bank Reference

| File | Line(s) | What | Type | Breaks on Split |
|------|---------|------|------|-----------------|
| `Makefile` | 31 | `GITLAB_HOST` | URL | Yes (remote switch) |
| `Makefile` | 32-34 | `DATA_REPOS` (16 repos) | List | Yes (need 24) |
| `config.py` | 49 | `GIT_HOST = "git-talkbank"` | String | Yes (remote switch) |
| `config.py` | 52 | `GIT_SSH_SERVER` | URL | Yes (remote switch) |
| `config.py` | 153-351 | `GIT_REPOS` dict | Dict | Yes (add split entries) |
| `config.py` | 380-553 | `HOSTS` dict | Dict | Yes (update repos lists) |
| `config.py` | 560-562 | `HOMEBANK_SITES` | Derived | No (uses HOSTS) |
| `config.py` | 602-604 | `REPO_HOSTS` | Derived | No (auto-derived from HOSTS) |
| `config.py` | 607-610 | `HOSTS_WITH_DATA` | Derived | No (uses HOSTS) |
| `config.py` | 616-629 | `DATA_REPO_TO_BANK` | Dict | No (already has splits) |
| `tasks.py` | 61-63 | `site_repo_name()` | Function | Dead code |
| `tasks.py` | 66-68 | `data_repo_name()` | Function | Yes |
| `tasks.py` | 71-76 | `site_repo_dir()` | Function | No (web repos 1:1) |
| `tasks.py` | 79-85 | `data_repo_dir()` | Function | Yes |
| `tasks.py` | 93-95 | `build_dir()` | Function | No (bank-level) |
| `tasks.py` | 242-259 | `force_build()` | Function | No (uses data_repo_bank) |
| `tasks.py` | 323-413 | `build_data()` | Function | Yes (core problem) |
| `tasks.py` | 376 | `site_repo_name(name)` call | Call | Dead code |
| `chatmedia.py` | 305 | `data_repo_dir()` call | Call | Yes |
| `dois.py` | 19 | Iterates `GIT_REPOS` `*-data` | Loop | No (auto-discovers) |
| `metadatas.py` | 13-30 | `urls` dict (16 repos) | Dict | Yes (need 24) |
| `metadatas.py` | 33-64 | `get_dict()` | Function | Yes (uses hardcoded urls) |
| `metadatas.py` | 67-78 | `update_git()` | Function | Yes (iterates hardcoded repos) |
| `metadatas.py` | 81-104 | `commit_git()` | Function | Yes (iterates hardcoded repos) |
| `sync-media` | 69-72 | `valid_banks` | String | No (bank names, not repo names) |
| `webdev/config.toml` | 14-18 | `names` list | List | No (bank names only) |
| `staging/hooks/check-doi-duplicates` | 47 | Glob `*-data` | Pattern | No (auto-discovers) |

---

## Category 6: Meta-Repo References (~/talkbank/)

### Hardcoded — Must Update

| File | What | Impact |
|------|------|--------|
| `Makefile:31` | `GITLAB_HOST = git@gitlab.talkbank.org:talkbank` | Needs GitHub URL |
| `Makefile:32-34` | `DATA_REPOS` (16 repos) | Needs 24 repos |
| `scripts/adopt-repos.sh:96` | Loops over 16 unsplit repo names | Needs 24 repos |
| `CLAUDE.md:68-74` | Workspace layout lists 16 repos with sizes | Update for 24 repos |
| `CLAUDE.md:113,123-126` | Example validation commands: `../data/phon-data/` | Update paths |
| `docs/inventory.md:156-171` | Lists 16 repos with GitLab URLs | Update for 24 repos on GitHub |
| `deploy/scripts/fix_underline_markers.py:31-42` | **11 hardcoded paths** to `ca-data/Jefferson/NB*` files | Paths move to `ca-data/` (post-split, Jefferson stays in ca-data, not ca-candor-data) — verify |
| `known-issues/phon-data-validation-baseline.txt` | 128 file paths starting with repo-relative paths | Paths split across `phon-eng-french-data/` and `phon-other-data/` — need 2 baseline files |
| `known-issues/README.md` | References `phon-data` as single baseline | Update for split repos |

### Documentation Only — Update When Convenient

| File | What |
|------|------|
| `docs/migration/*.md` | Migration docs (self-referential, update as we go) |
| `docs/legacy/*.md` | Historical docs — leave as-is (dated "Historical") |
| `docs/batchalign3-public-archive/` | Archived reports — leave as-is |
| `analysis/per-speaker-utr-experiment-2026-03-16/` | Experiment provenance — update PROVENANCE files |
| `deploy/docs/ssh-key-migration.md` | SSH key docs — many git-talkbank references, update post-decommission |
| `deploy/docs/fleet-management-plan.md:105` | Media mappings by repo name — verify still accurate |

---

## Category 7: batchalign3 References

### Good News: No Hardcoded Assumptions in Code

batchalign3's media mapping is **entirely configuration-driven** via `~/.batchalign3/server.yaml`:

```yaml
media_mappings:
  childes-data: /Users/macw/media/childes
  aphasia-data: /Users/macw/media/aphasia
  # ...
```

The CLI auto-detects the mapping key from the input path by extracting the `*-data` component. This is pattern-based (suffix extraction), not a hardcoded lookup.

### Must Update: server.yaml on Deployed Machines

| Location | What | Impact |
|----------|------|--------|
| `~/.batchalign3/server.yaml` on `net` | `media_mappings` dict | Keys like `childes-data` must become `childes-eng-na-data`, etc. — OR keep `childes-data` as a single key mapping to the same media dir (media is organized by bank, not by split) |
| `book/src/developer/server-yaml-template.yaml:36-52` | Template with 16 repo keys | Update to show split repo keys (or bank-level keys) |
| Test fixtures in `crates/batchalign-cli/tests/command_matrix.rs:383` | `"childes-data".into()` | Update test string |
| Test fixtures in `crates/batchalign-app/tests/json_compat.rs:29,150-155` | Test config parsing | Update test strings |

### Key Design Question: Media Mapping Keys Post-Split

The CLI extracts the mapping key from the input path. If a user runs:
```bash
batchalign3 align /path/to/childes-eng-na-data/Eng-NA/MacWhinney/file.cha
```
The CLI extracts `childes-eng-na-data` as the key and looks it up in `media_mappings`.

But media on `net` is organized by **bank** (`/Users/macw/media/childes/`), not by split repo. So we need EITHER:
- **Option A:** 4 mapping entries all pointing to the same media dir:
  ```yaml
  childes-eng-na-data: /Users/macw/media/childes
  childes-eng-uk-data: /Users/macw/media/childes
  childes-romance-germanic-data: /Users/macw/media/childes
  childes-other-data: /Users/macw/media/childes
  ```
- **Option B:** Change the CLI's key extraction to resolve split repos to bank names (using `DATA_REPO_TO_BANK` equivalent logic)

**Option A is simpler and requires no code changes.** Just more config entries.

---

## Category 8: talkbank-tools References

### No Breaking Code Changes Needed

talkbank-tools uses:
- **Embedded reference corpus** (`corpus/reference/`) for all tests — no external data dependency
- **3-tier path resolution** for OSX-CLAN: env var → workspace sibling → home dir
- **`../data/` relative convention** for data access from workspace root — works with any repo names

### Documentation & Examples — Update When Convenient

| File | What |
|------|------|
| `spec/errors/E502_auto.md:18-61` | Error specs cite `ca-data`, `childes-data` etc. as example sources |
| `spec/errors/E600_auto.md:23,44,66` | Same — example file paths |
| `Makefile:62,66` | Default `DATA_DIR=../data` — convention unchanged, but examples mention `childes-data` |
| `spec/tools/src/bin/perturb_corpus.rs:20` | Docstring: `--mine ../data/childes-data/Eng-NA` |
| `spec/tools/src/bin/extract_corpus_candidates.rs` | Example usage |
| `book/src/architecture/alignment.md:513` | Example: `chatter debug overlap-audit data/ca-data/` |
| `src/bin/clear-cache-prefix.rs:16` | Example: `/path/to/childes-data/EastAsian/Indonesian/Jakarta` |
| `spec/CLAUDE.md` | `--data-dir ../data` convention |

**Key insight:** The `../data/` convention itself doesn't change — the data directory still contains `*-data` repos, just 24 instead of 16. Commands like `chatter validate ../data/ --force` will automatically pick up all repos. Only examples that name specific repos need updating.

---

## Minimum Viable Changes for Phase 1 (Splits on GitLab)

To get the deploy pipeline working with split repos while still on GitLab:

1. **config.py:** Add split repo entries to `GIT_REPOS`. Remove old unsplit entries. Update `HOSTS[].repos` lists. (Already has `DATA_REPO_TO_BANK`.)

2. **tasks.py:** Rewrite `build_data(bank)` to iterate all data repos for a bank:
   - New helper: `data_repo_names(bank) → List[str]`
   - rsync each repo into the shared `data-orig/` directory
   - Run `update_chat_types()` on each repo individually
   - Commit+push each repo individually

3. **tasks.py:** Remove dead `site_repo_name()` and references.

4. **metadatas.py:** Add split repo entries to `urls` dict with correct bank domains.

5. **Makefile:** Update `DATA_REPOS` list to 24 repos.

6. **chatmedia.py:** Update `check_bank()` to iterate split repos (or defer — media validation is low priority).

**That's it for Phase 1.** Everything else either works already or is deferred.

## Phase 2: Remote Switch (GitLab → GitHub)

After splits are working on GitLab:

1. Create 24 repos on GitHub (fresh `git init`, push working trees)
2. Update `config.py` `GIT_REPOS` URLs from GitLab SSH to GitHub SSH
3. Update `Makefile` `GITLAB_HOST` → GitHub
4. Update git remotes on git-talkbank (`~/staging/repos/`)
5. Update git remotes on Brian/Davida's machines
6. Test deploy pipeline end-to-end
7. Archive old GitLab repos

## Phase 3: Pipeline Replacement (decommission git-talkbank)

1. Set up data repo clones on talkbank.org (like web repos)
2. Add GitHub Actions workflows to each data repo (same `git pull` pattern)
3. Deploy John's Node app on talkbank.org
4. Rewrite HTML download URLs
5. Rebuild DOI system as single clean tool
6. Shut down git-talkbank VM
