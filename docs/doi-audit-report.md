# DOI Audit Report

**Status:** Current
**Last updated:** 2026-03-21 15:48 EDT

This report documents the current state of TalkBank's DOI system based on a
full scan of all 24 data repos (806 corpus directories) and all 16 web bank
repos (814 HTML pages with DOI references). It is intended as a reference for
Franklin and as a basis for discussion with Brian.

---

## Summary

- **806** corpus directories with `0metadata.cdc` files across 24 data repos
- **814** HTML pages in web repos contain DOI references
- **9** "ghost" corpus directories contain *only* a `0metadata.cdc` file and
  zero data (no `.cha` files, no media, no subdirectories)
- **67** HTML pages reference DOIs outside the generated `site/access/` pages —
  all are citations of published papers using external journal DOIs, **none**
  reference our `10.21415` TalkBank prefix
- **24** HomeBank DOI records in DataCite have broken landing URLs (the
  `talkbank/access/...` bug from `metadatas.py`)
- The old `cdcs-to-csv` script is **broken** for all split repos and has not
  been updated

## Three Data Sources for DOIs

DOIs appear in three independent systems that can disagree:

| Source | What it contains | Count |
|--------|-----------------|-------|
| **DataCite** | The authoritative DOI registry for `10.21415` | ~806 registered DOIs |
| **CDC files** | `DOI:` field in `0metadata.cdc` in each `*-data` repo | 806 files scanned |
| **HTML pages** | `<td> doi:10.21415/XXXX </td>` injected by `generate-from-chat` into `site/access/*.html` | 814 files with DOI references |

When these disagree — a DOI exists in DataCite but not in the CDC file, or the
HTML has a DOI that DataCite doesn't know about — the result is either a broken
citation link or an orphaned DOI that costs money and pollutes search results.

## Ghost Corpus Directories

These 9 directories contain nothing except a `0metadata.cdc` file. They exist
solely to anchor a DOI so a contributor can cite the corpus:

| Directory | Bank | Title |
|-----------|------|-------|
| `ca-data/Mesolex` | CA | CABank Mesolex Corpus |
| `homebank-password-data/Password/deBarbaroChaos` | HomeBank | deBarbaro Chaos Corpus |
| `homebank-password-data/Password/deBarbaroCry` | HomeBank | deBarbaro Cry Corpus |
| `homebank-password-data/Password/IDSLabel` | HomeBank | IDSLabel Corpus |
| `homebank-password-data/Password/MendozaMusic` | HomeBank | MendozaMusic Password Corpus |
| `homebank-public-data/Public/Challenge` | HomeBank | VanDam Challenge Corpus |
| `homebank-public-data/Public/FauseyTrio-Public` | HomeBank | Fausey Trio Public Corpus |
| `homebank-public-data/Public/MendozaMusic-Public` | HomeBank | Mendoza Music Public Corpus |
| `homebank-public-data/Public/VanDam-Validation` | HomeBank | VanDam Validation Corpus |

All 9 already have DOIs minted. 7 of 9 are HomeBank — these are likely
audio-only corpora whose actual recordings live on the media server (`net`),
not in git. The CDC files in the data repos exist solely as DOI anchors.

This is a known design quirk, not a bug: corpus identity is defined by the
presence of a `0metadata.cdc` file in the directory tree, so "corpora" that
have no transcripts but need a DOI must still have a directory with a CDC file.

## DOIs in HTML: Two Distinct Populations

All 814 HTML files containing `doi:` fall into exactly two categories:

### 1. Generated corpus access pages (814 files in `site/access/`)

These are produced by `generate-from-chat`. Each contains a machine-injected
line like:

```html
<td> DOI:</td> <td> doi:10.21415/T5FX1S </td>
```

Every one uses our `10.21415` prefix. The DOI is displayed as plain text —
not hyperlinked. This is the primary user-facing DOI display mechanism.

### 2. Hand-authored publication/reference pages (67 files outside `site/access/`)

These appear in:
- `aphasia-bank/site/publications/methods.html` (21 references)
- `aphasia-bank/site/publications/discourse.html` (31 references)
- `homebank-bank/site/pubs/index.html`, `projects.html`, `rules.html`
- `rhd-bank/site/GR/index.html`
- `slabank-bank/site/access/` (a few hand-authored access pages)

Every one of these 67 references is a **citation of a published journal
article** using the publisher's DOI (prefixes like `10.1044`, `10.1080`,
`10.1016`, `10.1055`, `10.1371`). **None** reference our `10.21415` prefix.

**Conclusion:** Brian is not manually embedding TalkBank DOIs in hand-authored
HTML. The only place TalkBank DOIs appear in HTML is the generated
`site/access/` pages.

## Near-Empty CDC Files

Beyond the 9 ghost directories, many corpora have minimal metadata in their
CDC files. The typical "thin" CDC file has 7 lines:

```
Title:	SomeBank English SomeName Corpus
Creator:	LastName, FirstName
Language:	eng
Description:	one line
Date:		2018
Country:	United States
DOI:	doi:10.21415/XXXX
```

This is adequate for minting a DOI (title, creator, and date are required by
DataCite), but the descriptions are often generic ("HomeBank recording",
"narratives in Nahuatl") and would not help a researcher evaluate the corpus
from search results alone.

Brian has indicated that richer metadata is not a priority for him:

> "I don't really need more info in the DOI record. The main purpose of DOI is
> so contributors can point to their work."

## Known Bugs

### Broken HomeBank URLs in DataCite

The old `cdcs-to-csv` script has `"homebank-data": "talkbank"` in
`metadatas.py`, which produces landing URLs of `https://talkbank/access/...`
instead of `https://homebank.talkbank.org/access/...`. **24 HomeBank DOI
records** in DataCite currently have non-resolving landing URLs. These need to
be corrected by running `talkbank-doi sync` against the HomeBank repos.

### Split repo breakage

`metadatas.py` still references the old monolithic repo names (`childes-data`,
`phon-data`, `homebank-data`) which no longer exist after the GitLab → GitHub
split. The script cannot run at all in its current state. Any new corpora added
since the split (late 2025) have never been seen by the DOI tooling.

The replacement tool `talkbank-doi` handles all 24 split repo names correctly.

### Mint produces draft DOIs (fixed)

The `talkbank-doi` Rust tool was missing the `event: "publish"` field when
minting new DOIs via the DataCite REST API. Without this, new DOIs are created
in `draft` state — they don't resolve at doi.org and aren't indexed in
DataCite search. This has been fixed: `mint()` now sends `event: "publish"` so
new DOIs go straight to `findable`.

## What Brian Actually Wants

From his email (2026-03-21):

1. **Every corpus access page should display a DOI.** This is handled by
   `generate-from-chat` injecting DOIs into `site/access/*.html`. No change
   needed.

2. **A simple command to mint or update DOIs** when contributors ask. The
   command already exists:

   ```
   talkbank-doi sync --data-dir ~/talkbank/data/aphasia-data
   ```

   This discovers all CDC files in the specified repo, mints DOIs for any that
   lack one, updates changed metadata for existing ones, and writes the DOI
   back into the CDC file.

3. **Nothing else.** He does not want richer metadata, does not want an audit
   workflow, and does not care about DOI lifecycle management.

## Recommended Actions

### Immediate (fix existing breakage)

1. **Fix 24 broken HomeBank URLs.** Run `talkbank-doi sync` against all four
   HomeBank repos to push corrected landing URLs to DataCite.

2. **Backfill split repos.** Run `talkbank-doi check` across all 24 data repos
   to identify any corpora added since the split that need DOIs minted.

3. **Retire `cdcs-to-csv`.** It is broken, uses the legacy MDS API, hardcodes
   paths to `git.talkbank.org` (being decommissioned), and is replaced by
   `talkbank-doi`.

### Ongoing (for Franklin)

4. **Periodically run `talkbank-doi audit`** to generate a three-source
   reconciliation report (DataCite vs CDC files vs HTML). This catches drift
   before it becomes visible to users.

5. **Use `talkbank-doi review --verify`** (the new TUI) to identify and resolve
   orphaned DOIs, manually-minted records, and URL mismatches with Brian when
   convenient.

### Not needed (per Brian)

6. **LLM-enriched metadata** — shelved. Brian doesn't want richer descriptions.
7. **DOI Manager web app** — shelved. The CLI meets Brian's stated needs.
8. **Self-service minting UI for Brian/Davida** — shelved. Brian is happy to
   ask Franklin to run a command.
