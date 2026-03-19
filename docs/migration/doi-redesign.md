# DOI System Redesign

**Status:** Draft — Research phase
**Last updated:** 2026-03-19

---

## Current State: What's Wrong

### 1. Only 5 of 40 metadata fields reach DataCite

`0metadata.cdc` files contain rich metadata — language (ISO 639-3), descriptions, OLAC/IMDI archival fields, country, multiple creators, subjects — but the current tool sends only:
- Title
- First creator (additional creators ignored)
- Publication year
- Publisher (hardcoded "TalkBank")
- Resource type (hardcoded "Dataset")

**35+ fields are thrown away.** This includes language codes, descriptions, subjects, and geographic information that would make corpora discoverable in DataCite search, Google Dataset Search, and other aggregators.

### 2. Only corpora get DOIs

Currently DOIs map 1:1 to corpus directories (identified by `0metadata.cdc`). But users want DOIs for:
- **Papers and publications** about corpora
- **Tools** (CLAN, batchalign)
- **The banks themselves** (CHILDES as a collection)
- **Subcorpora or individual recordings** (finer granularity)

### 3. DOIs are not actionable

In the HTML, DOIs appear as plain text: `<td> doi:10.21415/T56W31 </td>`. They are never hyperlinked. A user can't click to resolve the DOI. The DOIs do resolve at `https://doi.org/10.21415/T56W31`, but nobody knows that because the HTML doesn't tell them.

### 4. Legacy MDS API via curl

The current tool uses DataCite's legacy MDS API with `curl` subprocess calls. The modern REST API (`https://api.datacite.org`) is more capable, supports JSON, and has proper error handling.

### 5. Three divergent copies of the parser

Three copies of `cdcfile.py` with different interfaces (pydantic vs dict, different key names). One has a bug on line 82 (missing f-string prefix).

### 6. XML built with f-strings

The DataCite XML is built by string interpolation with no escaping. A creator name containing `&` or `<` produces malformed XML. The modern REST API accepts JSON, eliminating this class of bugs entirely.

### 7. `output.csv` is the only inventory

If `output.csv` is lost, DOI state must be reconstructed from CDC files + DataCite API queries. No backup mechanism.

---

## Design Goals

1. **Send ALL metadata to DataCite** — language, description, subjects, all creators, country, IMDI fields where they map to DataCite schema
2. **Support DOIs for non-corpus resources** — papers, tools, collections
3. **Make DOIs clickable** in HTML (link to `https://doi.org/{doi}`)
4. **Use modern DataCite REST API** (JSON, not XML)
5. **Single tool** replacing 3 codebases
6. **Credentials in environment variables**, not plaintext Python
7. **Idempotent** — safe to run repeatedly, only updates what changed
8. **Works from any machine** — no git-talkbank dependency
9. **DataCite as source of truth** — query API to reconcile, not just CSV

---

## DataCite Metadata Schema 4.5 — What We Can Send

DataCite supports rich metadata. Here's how `0metadata.cdc` fields map:

| CDC Field | DataCite Field | Currently Sent | Notes |
|-----------|---------------|----------------|-------|
| `Title` | `titles[].title` | Yes | |
| `Creator` (all) | `creators[].name` | Only first | **Fix: send ALL creators** |
| `Date` | `publicationYear` + `dates[].date` | Year only | Can send full date |
| `Language` | `language` | No | **Easy win** — ISO 639-3 |
| `Description` | `descriptions[].description` | No | **Easy win** — type "Abstract" |
| `Country` | `geoLocations[].geoLocationPlace` | No | **Easy win** |
| `Subject` | `subjects[].subject` | No | **Easy win** |
| `Subject.olac:*` | `subjects[].subject` + `subjectScheme: "OLAC"` | No | With scheme attribution |
| `IMDI_Genre` | `subjects[].subject` (subjectScheme: "IMDI") | No | |
| `Contributor` | `contributors[].name` | No | With contributorType |
| `CMDI_PID` | `relatedIdentifiers[]` | No | Type: "IsIdenticalTo" |
| (new) | `rightsList[].rights` | No | Could indicate access level |
| (hardcoded) | `publisher` | Yes | "TalkBank" |
| (hardcoded) | `types.resourceTypeGeneral` | Yes | "Dataset" |

### What DataCite supports that we don't use yet

- **`relatedIdentifiers`** — link a corpus DOI to a paper DOI, to a tool DOI, to a parent collection DOI
- **`fundingReferences`** — link to grant numbers (NIH, NSF funding)
- **`rightsList`** — indicate open access vs password-protected
- **`sizes`** — number of files, total size
- **`formats`** — "CHAT", "Phon XML"
- **`version`** — corpus version tracking

---

## Architecture

### Single CLI Tool: `talkbank-doi`

```
talkbank-doi sync              # Discover all CDC files, sync with DataCite
talkbank-doi sync --dry-run    # Show what would change
talkbank-doi mint <path>       # Mint DOI for a specific corpus dir
talkbank-doi check             # Check for duplicates, missing DOIs, stale URLs
talkbank-doi export            # Export inventory CSV
talkbank-doi query <doi>       # Query DataCite for a specific DOI
```

### Configuration

```bash
# Environment variables (no plaintext credentials)
export DATACITE_CLIENT_ID="SML.TALKBANK"
export DATACITE_CLIENT_SECRET="..."
export DATACITE_PREFIX="10.21415"
export DATACITE_API="https://api.datacite.org"  # or https://api.test.datacite.org for testing
```

### Data Flow

```
0metadata.cdc files (in 24 data repos)
    │
    ▼
talkbank-doi sync
    │
    ├── Parse ALL fields from CDC files
    ├── Query DataCite for existing DOIs
    ├── Compare: what's new, what changed, what's stale
    │
    ├── Mint new DOIs (POST /dois)
    ├── Update changed metadata (PUT /dois/{doi})
    ├── Update changed URLs (PUT /dois/{doi})
    │
    ├── Write DOI back to CDC file (if newly minted)
    ├── Write inventory CSV (backup)
    └── Report: what was done, what needs attention
```

### CDC Parsing: Send Everything

New parser extracts ALL fields, not just 5:

```python
@dataclass
class CorpusMetadata:
    # Required
    title: str
    creators: list[str]          # ALL creators, not just first
    publication_year: int
    date: str | None             # Full date if available

    # Identification
    doi: str | None
    cmdi_pid: str | None

    # Rich metadata (currently ignored, now sent)
    language: str | None         # ISO 639-3
    description: str | None
    country: str | None
    subjects: list[str]          # From Subject, Subject.olac:*, Subject.childes:*

    # Archival
    imdi: dict[str, str]         # All IMDI_* fields preserved

    # Computed
    target_url: str              # Derived from repo path + bank domain
    resource_type: str = "Dataset"
    publisher: str = "TalkBank"
```

### DataCite JSON (replaces XML)

```json
{
  "data": {
    "type": "dois",
    "attributes": {
      "prefix": "10.21415",
      "creators": [
        {"name": "Fromm, Davida"},
        {"name": "MacWhinney, Brian"}
      ],
      "titles": [{"title": "AphasiaBank GR Corpus"}],
      "publisher": "TalkBank",
      "publicationYear": 2022,
      "types": {"resourceTypeGeneral": "Dataset"},
      "url": "https://aphasia.talkbank.org/access/English/GR.html",
      "language": "eng",
      "descriptions": [
        {"description": "materials for collaborative commentary", "descriptionType": "Abstract"}
      ],
      "subjects": [
        {"subject": "child language development"},
        {"subject": "language_acquisition", "subjectScheme": "OLAC"}
      ],
      "geoLocations": [
        {"geoLocationPlace": "United States"}
      ]
    }
  }
}
```

### URL Construction (updated for splits)

Uses `BANK_TO_DATA_REPOS` reverse lookup:

```
aphasia-data/English/GR/  →  https://aphasia.talkbank.org/access/English/GR.html
childes-eng-na-data/Eng-NA/Bates/  →  https://childes.talkbank.org/access/Eng-NA/Bates.html
```

Multiple split repos map to the same bank domain — same logic as in `config.py`.

---

## Open Questions

### Q1: Language for the tool?

Options:
- **Python** — fastest to write, DataCite has a Python client library
- **Rust** — consistent with talkbank-tools, better error handling, publishable on crates.io

Recommendation: **Python** for the initial version (it's a batch admin tool, not a hot path), with the option to rewrite in Rust later if it becomes a frequently-run tool.

### Q2: What about non-corpus DOIs?

If Brian wants DOIs for papers, tools, or collections, the tool needs a metadata source that isn't `0metadata.cdc` (which is corpus-specific). Options:
- A new metadata file format (e.g., `0doi.toml`) that lives alongside papers/tools
- A central registry file listing non-corpus DOIs
- Manual minting via CLI: `talkbank-doi mint --title "..." --creator "..." --type Software`

### Q3: Should we update existing DOIs at DataCite with rich metadata?

We have 806 DOIs registered with only 5 fields. We could do a one-time backfill to send language, description, subjects, and country for all existing DOIs. This would dramatically improve discoverability.

**Risk:** Low — we're adding metadata, not changing it. DataCite accepts updates.

### Q4: What about the HTML DOI display?

Currently: `<td> DOI:</td> <td> doi:10.21415/T56W31 </td>`
Should be: `<td> DOI:</td> <td><a href="https://doi.org/10.21415/T56W31">10.21415/T56W31</a></td>`

This is a separate HTML fix (can be a bulk find-and-replace) but should be planned alongside the DOI tool work.

### Q5: DataCite test environment?

DataCite has a test API (`https://api.test.datacite.org`) for development. Should we test there first? Do we have test credentials?

### Q6: What happens to `0metadata.cdc` format?

The current format is fragile (tab-separated, no schema, inconsistent field names). Options:
- Keep as-is (don't change what works for 806 files)
- Migrate to TOML/YAML (better tooling support, schema validation)
- Keep the format but add a schema validator

---

## Implementation Phases

### Phase A: Core tool + backfill (highest impact)

1. Write `talkbank-doi` CLI with `sync` and `check` commands
2. Parse all CDC fields (not just 5)
3. Use modern DataCite REST API (JSON)
4. Backfill all 806 existing DOIs with rich metadata
5. Verify with `--dry-run` first

### Phase B: HTML integration

1. Make DOIs clickable in HTML (bulk find-and-replace)
2. DOI injection into HTML becomes part of the pre-push hook or the tool itself

### Phase C: Non-corpus DOIs

1. Define metadata format for papers, tools, collections
2. Support `relatedIdentifiers` (link corpus → paper, corpus → collection)
3. Mint collection-level DOIs for banks

### Phase D: Automation

1. Pre-push hook: validate CDC files, check for duplicates
2. GitHub Actions: run `talkbank-doi check` on push
3. Periodic sync: cron or scheduled Actions to keep DataCite in sync
