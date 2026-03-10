# Documentation Audit (2026-02-25)

This audit classified all book documentation into:

- **kept in published mdBook** (`book/src`)
- **archived historical/internal** (`docs/archive/book-src`)

## Coverage

- audited docs: 140
- kept in `book/src`: 107
- archived to `docs/archive/book-src`: 33
- published in `SUMMARY.md`: 107

## Archive split

- architecture archived: 2
- developer archived: 5
- decisions archived: 21
- reference archived: 5

## Decision rule

Kept in published docs:

- stable user/developer guidance
- active architecture/reference docs
- decision records that still guide implementation

Archived:

- dated execution sheets and sign-off templates
- one-off campaign notes and temporary planning docs
- internal audit/comparison snapshots retained for provenance

## Source of truth

- archive index: `docs/archive/README.md`
- machine-readable audit: `analysis/docs-audit-2026-02-25.json`

## Project-wide markdown inventory (excluding vendored/build dirs)

This broader inventory tracks markdown docs outside mdBook too (crate docs, reports, and private/internal docs):

- total markdown docs: 183
- published mdBook pages: 107
- archived mdBook pages: 33
- archived reports: 2
- archived root notes: 1
- component docs (`rust/`, `rust-next/`): 26
- historical reports still in active tree (`benchmarks/`): 1
- internal/private guidance (`CLAUDE.md`, `.kiro`): 9
- top-level repo docs: 3

Machine-readable inventory:

- `analysis/docs-audit-repo-2026-02-25.json`
