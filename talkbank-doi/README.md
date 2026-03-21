# talkbank-doi

**Status:** Current
**Last updated:** 2026-03-20 14:00

---

TalkBank DOI management tool. Replaces the old `cdcs-to-csv` Python system.

**User-facing workflow guide:** `docs/doi-workflow.md`
**Design rationale:** `docs/migration/doi-redesign.md`

---

## What this replaces

| Old | New |
|-----|-----|
| `staging/repos/cdcs-to-csv/` — Python, legacy DataCite MDS API, curl subprocesses | `talkbank-doi` — Rust, modern DataCite REST API, JSON |
| Sent only 5 of 40+ metadata fields (title, first creator, year, publisher, type) | Sends all fields: all creators, language, description, country, subjects, OLAC fields, geo |
| Required running on a specific staging machine | Works from any machine with env vars |
| Three divergent copies of `cdcfile.py` | Single parser in `src/cdc.rs` |
| XML built with f-strings (no escaping — `&` in names broke it) | JSON serialization via serde |

The last known CSV snapshot from the old system is `staging/repos/cdcs-to-csv/output.csv`. Do not run the old tool for new minting.

---

## Building

```bash
cd talkbank-doi
cargo build --release
# Binary: target/release/talkbank-doi
```

Or run directly with cargo:

```bash
cargo run --release -- check --data-dir ../data
```

---

## Environment variables

`talkbank-doi sync` (the only command that talks to the DataCite write API) requires credentials via environment variables. Never commit credentials to any repo.

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATACITE_CLIENT_ID` | Yes | — | DataCite client ID (e.g., `SML.TALKBANK`) |
| `DATACITE_CLIENT_SECRET` | Yes | — | DataCite client secret |
| `DATACITE_API` | No | `https://api.datacite.org` | API base URL; set to `https://api.test.datacite.org` for testing |
| `DATACITE_PREFIX` | No | `10.21415` | DOI prefix for minting new DOIs |

Store credentials in `~/.config/talkbank/env` (not tracked by git) and source before running:

```bash
source ~/.config/talkbank/env
talkbank-doi sync --data-dir ../data
```

`check`, `query`, and `export` do not require credentials. `query` makes a public unauthenticated GET to `https://api.datacite.org`.

---

## Commands

### `check` — Discover and validate all CDC files

```bash
talkbank-doi check [--data-dir DIR]
```

Walks the data directory, finds all `0metadata.cdc` files, and reports:

- **NEW** — CDC file exists but has no `DOI:` field (needs minting)
- **SPARSE** — missing recommended fields: `language`, `description`, or `country`
- **DUPLICATE** — same DOI value appears in two different CDC files (error)
- **INVALID** — missing required fields: `Title`, `Creator`, or `Date` (error)
- **WARN** — can't construct a landing page URL from the repo path (unusual)

Prints a summary: with DOI / without DOI / errors / total.

`--data-dir` defaults to `.` (current directory). When running from the workspace root, pass `--data-dir data/`.

### `sync` — Mint new DOIs and update changed ones

```bash
talkbank-doi sync [--data-dir DIR] [--dry-run]
```

For each valid CDC file:

- If no `DOI:` field: mints a new DOI via `POST /dois`, then writes `DOI:\tdoi:10.21415/XXXX` back into the CDC file.
- If `DOI:` field present: updates metadata at DataCite via `PUT /dois/{doi}`.

Requires `DATACITE_CLIENT_ID` and `DATACITE_CLIENT_SECRET`.

`--dry-run` shows what would happen without making any API calls or file writes.

**Always run `--dry-run` first.**

### `query` — Look up a DOI at DataCite

```bash
talkbank-doi query 10.21415/T56W31
```

Fetches and pretty-prints the full DataCite JSON record for a DOI. No credentials required (public API endpoint).

### `export` — Write a CSV inventory

```bash
talkbank-doi export [--data-dir DIR] [--output FILE]
```

Writes a CSV with columns: `doi`, `target`, `title`, `creators`, `publisher`, `year`, `language`, `description`, `country`.

`--output` defaults to `dois.csv`. Useful for auditing or handing to Brian.

---

## Split-repo to bank URL mapping

The data repos are split: multiple repos map to a single bank domain. `talkbank-doi` knows this mapping and constructs landing page URLs accordingly:

| Repo prefix | Bank domain |
|-------------|------------|
| `childes-eng-na-data`, `childes-eng-uk-data`, `childes-romance-germanic-data`, `childes-other-data` | `childes.talkbank.org` |
| `ca-data`, `ca-candor-data` | `ca.talkbank.org` |
| `phon-eng-french-data`, `phon-other-data` | `phon.talkbank.org` |
| `homebank-public-data`, `homebank-cougar-data`, `homebank-bergelson-data`, `homebank-password-data` | `homebank.talkbank.org` |
| `aphasia-data` | `aphasia.talkbank.org` |
| `talkbank-data` | `talkbank.org` |
| any other `<name>-data` | `<name>.talkbank.org` |

URL construction: `https://{domain}/access/{corpus-path-within-repo}.html`

Example: `data/childes-eng-na-data/Eng-NA/Brown/Adam/0metadata.cdc` → `https://childes.talkbank.org/access/Eng-NA/Brown/Adam.html`

The mapping is implemented in `src/main.rs` (`repo_to_bank()` and `bank_domain()`).

---

## Workflow for minting a new corpus DOI

1. **Check the CDC file has required fields:**
   ```bash
   talkbank-doi check --data-dir data/
   ```
   Fix any INVALID or DUPLICATE errors before proceeding.

2. **Dry run to preview:**
   ```bash
   source ~/.config/talkbank/env
   talkbank-doi sync --data-dir data/ --dry-run
   ```
   Confirm the corpus you expect to see listed as `WOULD MINT`.

3. **Live run:**
   ```bash
   talkbank-doi sync --data-dir data/
   ```
   The tool mints the DOI and writes `DOI:\tdoi:10.21415/XXXX` back into the CDC file.

4. **Commit the updated CDC file:**
   ```bash
   cd data/<repo-name>
   git add 0metadata.cdc   # (or the specific CDC files that were updated)
   git commit -m "doi: mint DOI for <corpus name>"
   git push
   ```

5. **Verify:**
   ```bash
   talkbank-doi query 10.21415/XXXX
   ```

---

## Pre-push hook integration

`scripts/hooks/pre-push` is installed into each `*-data` repo (by `scripts/setup-data-workspace.sh` or equivalent) as `.git/hooks/pre-push`. It runs three checks before every `git push`:

1. `check-large-files` — blocks commits with files over the size limit
2. **`check-doi-duplicates`** — scans all sibling `*-data` repos for duplicate `DOI:` values; blocks push if any found
3. `update-types` — updates `@Types` headers in CHAT files

`check-doi-duplicates` (`scripts/hooks/check-doi-duplicates`) is a bash script that greps all `0metadata.cdc` files across siblings and reports conflicts. It does **not** mint or modify DOIs.

The hooks read from `scripts/hooks/` at the path stored in `talkbank-hooks` (a sibling symlink in the repo). If the `talkbank-hooks` directory is not present, the hook exits silently.

---

## Legacy: cdcs-to-csv

The old tool lives at `staging/repos/cdcs-to-csv/`. Do not use it for new minting. Its last output is `staging/repos/cdcs-to-csv/output.csv` — the pre-migration DOI inventory.

The three copies of `cdcfile.py` that existed before this tool:
- `staging/scripts/cdcfile.py`
- `staging/repos/generate-from-chat/cdcfile.py`
- `staging/repos/cdcs-to-csv/cdcfile.py`

All three are superseded by `src/cdc.rs`. The Rust parser handles the same format plus double-tab separators, `doi:` prefix stripping, and the `IMDI_*` field namespace.

---

## DataCite test environment

For development or testing changes to the sync logic, use the DataCite test API:

```bash
export DATACITE_API=https://api.test.datacite.org
export DATACITE_CLIENT_ID=<test-client-id>
export DATACITE_CLIENT_SECRET=<test-secret>
export DATACITE_PREFIX=10.5072   # test prefix, not permanent
```

DOIs minted on the test environment use prefix `10.5072` and are not permanent or resolvable in production. Use `--dry-run` to validate the logic without hitting any API.

---

## CDC file format reference

`0metadata.cdc` is a tab-separated key-value file:

```
@UTF8
Title:	Brown Corpus
Creator:	Brown, Roger
Date:	2004
Language:	eng
Description:	Longitudinal study of three children.
Country:	United States
Subject:	child language acquisition
DOI:	doi:10.21415/T56W31
```

**Required:** `Title`, `Creator` (one or more), `Date`

**Recommended:** `Language`, `Description`, `Country`

The parser (`src/cdc.rs`) accepts:
- `Key:\tValue` (single tab)
- `Key:\t\tValue` (double tab — some legacy files use this)
- `DOI:\tdoi:10.21415/XXXX` or `DOI:\t10.21415/XXXX` (strips `doi:` prefix either way)
- `IMDI_*` keys are stored as IMDI metadata
- `Subject.olac:*` and `Type.olac:*` keys map to OLAC subject fields
- Unknown keys are silently ignored

Lines starting with `@UTF8` or `@Window:` are skipped.

---

## Future work (Phase B/C/D from design doc)

- **HTML DOI injection** — make DOIs clickable links in corpus HTML pages (currently plain text); to be handled in `generate-from-chat/` or as a post-sync step
- **GitHub Actions** — run `talkbank-doi check` on data repo PRs automatically
- **Backfill sync** — after GitHub migration completes, run `sync` to push rich metadata for all ~806 existing DOIs (previously only 5 fields were sent)
- **Non-corpus DOIs** — papers, tools, collections (needs a metadata format beyond `0metadata.cdc`)

Full design rationale: `docs/migration/doi-redesign.md`
