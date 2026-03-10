# Documentation Scope

**Status:** Current  
**Last verified:** 2026-03-05

This project keeps three documentation tiers:

- **Published docs** in `book/src/` and `mdbook`
- **Public archive/history docs** in `docs/archive/` when public link stability
  or compact historical context still matters
- **Private/internal history** in `talkbank-private` for report-style
  archaeology, campaign notes, and process material that should not live in the
  public book

## What stays in `book/src`

- user-facing usage and operational guidance
- stable architecture and technical reference material
- contributor onboarding and active developer workflows
- ADRs that still shape current implementation
- durable migration comparison anchored to:
  - Jan 9, 2026 BA2 baseline
    `84ad500b09e52a82aca982c41a8ccd46b01f4f2c`
  - Feb 9, 2026 released BA2 master point
    `e8f8bfada6170aa0558a638e5b73bf2c3675fe6d` where needed
  - current BA3

## What should not stay in the public book

- dated execution sheets, sign-off templates, and cutover logs
- superseded proposals, one-off campaign notes, and temporary plans
- internal audits and comparison snapshots kept only for institutional memory
- dated standalone reports and root planning notes
- implementation spike reports
- provider-specific rollout or migration material that belongs in plugin repos

Rule of thumb:

- keep current behavior and active contributor truth in `book/src`
- keep migration-relevant historical material in `book/src/migration/`
- keep only short historical stubs publicly when they protect old links or
  explain current architecture
- move deep archaeology and internal process artifacts to `talkbank-private`

## Public vs private boundary

The public book should optimize for:

- current users
- contributors working on the active codebase
- `batchalign2` users migrating to `batchalign3`

If a page mainly answers "what did we once try, benchmark, or debug?" rather
than "how does it work now?" or "what changed for migration?", it belongs in
private history, not the public book.

## Example private moves

Suggested private destination root:
`talkbank-private/docs/batchalign3-reports/`

| Source (moved from public book) | Private destination |
|---|---|
| `book/src/reference/align-correctness-fixes.md` | `docs/batchalign3-reports/reference/align-correctness-fixes.md` |
| `book/src/reference/align-monotonicity.md` | `docs/batchalign3-reports/reference/align-monotonicity.md` |
| `book/src/reference/benchmark-methodology.md` | `docs/batchalign3-reports/reference/benchmark-methodology.md` |
| `book/src/reference/benchmark-results.md` | `docs/batchalign3-reports/reference/benchmark-results.md` |
| `book/src/reference/morphotag-throughput.md` | `docs/batchalign3-reports/reference/morphotag-throughput.md` |
| `book/src/reference/throughput-analysis.md` | `docs/batchalign3-reports/reference/throughput-analysis.md` |
| `book/src/reference/utr-auto-add-history.md` | `docs/batchalign3-reports/reference/utr-auto-add-history.md` |
| `book/src/reference/utr-in-align-pipeline.md` | `docs/batchalign3-reports/reference/utr-in-align-pipeline.md` |
| `book/src/reference/wor-terminator-bullet-behavior.md` | `docs/batchalign3-reports/reference/wor-terminator-bullet-behavior.md` |
| `book/src/reference/wor-tier-python-vs-rust.md` | `docs/batchalign3-reports/reference/wor-tier-python-vs-rust.md` |
| `book/src/developer/documentation-audit-2026-02-25.md` | `docs/batchalign3-reports/developer/documentation-audit-2026-02-25.md` |
| `book/src/decisions/align-improvements.md` | `docs/batchalign3-reports/decisions/align-improvements.md` |
| `book/src/decisions/bug-fixes-backport.md` | `docs/batchalign3-reports/decisions/bug-fixes-backport.md` |
| `book/src/decisions/rust-implementation-coverage.md` | `docs/batchalign3-reports/decisions/rust-implementation-coverage.md` |
| `book/src/decisions/talkbank-batchalign-transition.md` | `docs/batchalign3-reports/decisions/talkbank-batchalign-transition.md` |
| `book/src/decisions/rusqlite-vs-sqlx-spike-2026-02.md` | `docs/batchalign3-reports/decisions/rusqlite-vs-sqlx-spike-2026-02.md` |

## Why this split

- keeps published docs focused for newcomers
- preserves historical context without deleting it
- avoids exposing internal/private process artifacts in public docs
- keeps the migration book authoritative instead of competing with archaeology

## Audit record

The current classification and move list is tracked in:

- `docs/archive/README.md`
- `analysis/docs-audit-2026-02-25.json`
- `analysis/docs-public-private-triage-2026-03-05.md`
