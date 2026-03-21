# DOI System Churn Report: cdcs-to-csv Git History Analysis

**Status:** Reference
**Last updated:** 2026-03-20 16:00

> This document analyzes the git history of `staging/repos/cdcs-to-csv/` to catalog the sources of instability in the DOI management system. It also explains what the script actually does and identifies the current breakage caused by the repo split.

---

## What cdcs-to-csv Actually Does

The `cdcs_to_csv.py` script is a batch DOI maintenance tool run manually by Franklin on `git.talkbank.org`. Its job:

1. **`git pull`** all 16 data repos (hardcoded in `metadatas.py`)
2. **Walk every repo** looking for `0metadata.cdc` files
3. **Parse each CDC file** — but only 5 fields: title, first creator only, year, publisher (hardcoded "TalkBank"), resource type (hardcoded "Dataset")
4. **Compare** against `output.csv` to detect new corpora or changed metadata
5. **Mint new DOIs** or update changed ones via DataCite MDS API (legacy, curl subprocess, XML payload)
6. **Write DOI back** into the `0metadata.cdc` file in place
7. **`git commit/push`** the updated CDC files in all repos
8. **Write `output.csv`** — the CSV is both the "database" and the change-detection state

The script's fragility stems from three design choices:
- **File system is identity**: a corpus exists if and only if a `0metadata.cdc` file is found by walking a hardcoded repo path. If the repo is renamed, moved, or split, corpora vanish silently.
- **output.csv is the state store**: there is no external database. The CSV is what the script diffs against. If it gets out of sync with DataCite, there is no recovery path.
- **Everything hardcoded**: `base = "/home/macw/staging/repos"` and a fixed list of 16 repo names, all of which assume the pre-split repo structure.

---

## Categories of Churn

### Category 1: Bank Renames (URL scheme changes)

These were sweeping changes where a bank domain or path segment changed, causing every DOI target URL for that bank to need updating.

**2021-11-02 — phonbank→phon, samtalebank→samtale** (large diff, ~40+ rows changed)

The banks `phonbank.talkbank.org` and `samtalebank.talkbank.org` were renamed to `phon.talkbank.org` and `samtale.talkbank.org`. Every DOI target URL for those banks had to be re-registered with DataCite.

In `metadatas.py`, the mapping entries changed from:
```python
"phon-data":    "phonbank",
"samtale-data": "samtalebank",
```
to:
```python
"phon-data":    "phon",
"samtale-data": "samtale",
```

Every corpus in those banks had its DOI target URL updated in DataCite and in `output.csv`. This is the most common form of churn — a domain decision cascades into hundreds of API calls.

**2025-05-07 — motorspeech→motor** (medium diff)

The `motorspeech-data` repo was renamed to `motor-data`, and its bank domain changed accordingly. All motor speech corpus DOI URLs updated. The `metadatas.py` entry changed:
```python
"motorspeech-data": "motorspeech",
# became:
"motor-data": "motor",
```

**2016-era — URL scheme change from zip access to HTML access**

Early DOI targets pointed at zip download endpoints. The scheme changed to HTML corpus pages at `https://{bank}.talkbank.org/access/{path}.html`. Every existing DOI target URL had to be re-registered — essentially a full re-mint of the entire corpus inventory.

**2021 — http→https**

All DOI target URLs migrated from `http://` to `https://`. Another full-sweep update.

---

### Category 2: Internal Directory Restructuring Within Banks

Brian frequently reorganizes directories within a corpus bank — moving corpora between language groups, renaming top-level folders, or consolidating. Because the DOI target URL is derived from the filesystem path relative to the repo root, any directory rename is a URL change.

**2025-01-21 — Major AphasiaBank restructure** (182-line diff in output.csv, largest single commit)

AphasiaBank's internal directory structure was reorganized. The change was large enough to touch ~90 corpus records. Key patterns observed:

- `Aphasia/{CorpusName}/` paths became `Protocol/{CorpusName}/` — the top-level `Aphasia/` directory was replaced with `Protocol/`
- `Control/{CorpusName}/` paths became `Protocol/Control{CorpusName}/` or similar — controls moved under Protocol
- Many corpora moved between subdirectories

This meant every affected corpus had its DOI target URL changed at DataCite. The script detected these as URL changes (same title, different target) and called `datacite.update()` for each one.

Example of the kind of change in output.csv:
```
# Before:
10.21415/XXXX,https://aphasia.talkbank.org/access/Aphasia/GR.html,...

# After:
10.21415/XXXX,https://aphasia.talkbank.org/access/Protocol/GR.html,...
```

**2023-07-11 — CHILDES Clinical renaming** (~30 rows changed)

Within CHILDES, the Clinical corpora were reorganized:
- `Clinical-MOR` directory renamed to `Clinical-Eng`
- `Clinical` directory renamed to `Clinical-Other`

All DOI target URLs for those corpora updated in DataCite.

**Pattern**: These restructurings happen several times per year. Each one generates a batch of DOI update API calls proportional to the number of affected corpora. The script cannot distinguish between "corpus was renamed" and "new corpus appeared while old one vanished" — it only compares output.csv against the current filesystem state.

---

### Category 3: API Migration (EZID→DataCite MDS, 2018)

Before DataCite, TalkBank used EZID (California Digital Library's DOI service) under a different prefix. In 2018 the system migrated to DataCite's MDS API directly. This was a one-time but large-scale change:

- DOI prefix changed (old EZID prefix retired)
- All existing DOIs had to be re-registered with DataCite
- The `credentials.py` and `datacite.py` modules were rewritten
- `output.csv` was rebuilt from scratch

This migration created a gap where corpora with EZID DOIs had DOI values in their CDC files that were not in DataCite, and vice versa — the kind of inconsistency that output.csv exists to track but cannot fully resolve.

---

### Category 4: Duplicate DOI Minting

The script has a known failure mode: if run twice without the CDC file write completing (e.g., a crash or git push failure between mint and commit), it mints a second DOI for the same corpus because the DOI is not yet in the CDC file. The first DOI becomes an orphan in DataCite — permanently registered but pointing nowhere.

Brian has also minted DOIs directly through DataCite Fabrica (the web UI), which the script does not know about. Those DOIs exist in DataCite but not in the CDC files, so the next time the script runs it mints a new one — a duplicate.

The `check-isbns.py` utility was written specifically to detect these duplicates by looking for multiple rows in output.csv with the same title+creator+year combination. It is described as "no longer used" in the current codebase.

---

### Category 5: The Current Breakage — Repo Split (2026)

**This is unresolved and actively wrong.**

In early 2026, the 16 monolithic data repos were split into 24 repos as part of the GitLab→GitHub migration:

| Old repo | New repos |
|----------|-----------|
| `childes-data` | `childes-eng-na-data`, `childes-eng-uk-data`, `childes-romance-germanic-data`, `childes-other-data` |
| `phon-data` | `phon-eng-french-data`, `phon-other-data` |
| `homebank-data` | `homebank-public-data`, `homebank-cougar-data`, `homebank-bergelson-data`, `homebank-password-data` |
| `ca-data` | `ca-data` (remainder), `ca-candor-data` |

`metadatas.py` was **never updated**. It still contains:

```python
base = "/home/macw/staging/repos"

urls = {
    "childes-data":  "childes",   # repo no longer exists
    "phon-data":     "phon",      # repo no longer exists
    "homebank-data": "talkbank",  # repo no longer exists + WRONG URL (see below)
    "ca-data":       "ca",        # split; ca-candor-data missing
    ...
}
```

The split repos (`childes-eng-na-data` etc.) are invisible to the script. If the script were run today:
- It would try to `git pull` the non-existent `childes-data`, `phon-data`, `homebank-data` repos and fail
- It would not discover any CDC files in the split repos
- It would see all those corpora as "deleted" and not update them
- Any new corpora added to split repos since the split would never get DOIs

---

### Category 6: The HomeBank URL Bug

Independently of the split, `metadatas.py` has always had a bug in the HomeBank entry:

```python
"homebank-data": "talkbank",
```

The value `"talkbank"` causes the URL to be constructed as:

```
https://talkbank/access/...
```

instead of:

```
https://homebank.talkbank.org/access/...
```

This was introduced when HomeBank was originally added to the script (the intent was probably `"homebank"`, not `"talkbank"` — the HomeBank bank domain is `homebank.talkbank.org`).

**24 HomeBank entries in the current `output.csv` have broken target URLs** — they resolve to `talkbank/access/...` which is not a real host. These have been registered with DataCite as broken landing URLs. The `talkbank-doi` Rust tool correctly maps `homebank-*-data` repos to `homebank.talkbank.org`, but these 24 existing records need to be corrected via `talkbank-doi sync` or a direct DataCite API update.

The commit in which this first became visible in output.csv: **2026-02-23** (the last time the script was run before the split).

---

## Summary Table

| Category | Frequency | Scope per event | Fixable by script? |
|----------|-----------|-----------------|-------------------|
| Bank domain renames | Rare (once every 1-3 years) | All corpora in that bank | Yes — update `metadatas.py` |
| Internal directory restructuring | Several times/year | 5–90 corpora per event | Yes — detected as URL change |
| API migration | Once (2018) | All corpora | Done |
| Duplicate minting | Occasional | 1-5 corpora | Manual cleanup at DataCite |
| Repo split breakage | Current (2026) | ~300+ corpora | **No — script is broken** |
| HomeBank URL bug | Present since inception | 24 corpora | **No — wrong URL in DataCite** |

---

## Root Cause

All of these failures share the same root: **corpus identity is defined by filesystem path**, and the script derives everything (DOI target URL, repo grouping, whether a corpus "exists") from that path. Any organizational change — rename, move, restructure, split — is invisible to the script until someone manually updates the hardcoded mappings.

The `talkbank-doi` Rust tool has better split-repo handling (it maps all split repo prefixes to the correct bank domain) but inherits the same fundamental design: it walks the filesystem and derives identity from path. The difference is that it reads env-configured repo roots rather than hardcoded paths, and it knows about the split repos.

The only durable fix is to move corpus identity out of the filesystem and into an explicit registry — a database, a web app, or DataCite itself as the source of truth — so that renames and moves do not silently break the DOI→URL mapping.

---

## What Needs Fixing Now

1. **`metadatas.py` in cdcs-to-csv**: Update repo list to reflect the 24 split repos. Fix `"homebank-data": "talkbank"` → `"homebank-public-data": "homebank"`, etc. (Or retire cdcs-to-csv in favor of `talkbank-doi sync`.)

2. **24 broken HomeBank DOIs in DataCite**: Run `talkbank-doi sync` against the homebank repos to push corrected URLs for those 24 records.

3. **`talkbank-doi` for new minting going forward**: The Rust tool correctly handles all 24 split repos. Any new minting should go through `talkbank-doi sync`, not cdcs-to-csv.

4. **Backfill all split-repo corpora**: Corpora added to split repos since the split (late 2025–present) have never been seen by any DOI tool. Run `talkbank-doi check` across all 24 data repos to find CDC files without DOIs.
