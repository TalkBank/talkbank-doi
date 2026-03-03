# Workspace Documentation Audit (2026-02-25)

Scope: `batchalign-hk-plugin`, `batchalign3`, `talkbank-chat`, `talkbank-chatter`, `talkbank-clan`, `talkbank-private`, `tree-sitter-talkbank`.

## Per-Project Totals

| Project | Total docs | Published book | Archived docs (all archive tiers) | Internal/private | Historical candidates not archived |
|---|---:|---:|---:|---:|---:|
| `batchalign-hk-plugin` | 1 | 0 | 0 | 0 | 0 |
| `batchalign3` | 186 | 109 | 37 | 9 | 4 |
| `talkbank-chat` | 747 | 78 | 24 | 9 | 18 |
| `talkbank-chatter` | 25 | 0 | 3 | 2 | 0 |
| `talkbank-clan` | 5 | 0 | 0 | 1 | 0 |
| `talkbank-private` | 25 | 0 | 8 | 1 | 1 |
| `tree-sitter-talkbank` | 5 | 0 | 0 | 1 | 0 |

## Moves Applied In This Pass

### talkbank-chat

- Archived `book/src/architecture/legacy/*.md` (22 files) to `docs/archive/book-src/architecture/legacy/`.
- Archived `book/src/architecture/flat-mor-model-impact.md` to `docs/archive/book-src/architecture/`.
- Replaced large legacy TOC block in `book/src/SUMMARY.md` with `architecture/legacy-archive.md`.

### talkbank-chatter

- Archived `docs/java-chatter-vs-rust-comparison.md` and `docs/java-chatter-vs-rust-report.md` to `docs/archive/reports/`.

### talkbank-private

- Archived root notes to `docs/archive/batchalign/root-notes/`: `BOSS_REPORT.md`, `EMAIL_TO_BRIAN.md`, `OVERNIGHT_PLAN*.md`.
- Archived historical branch snapshots to `docs/archive/batchalign/reports/`: `master-branch-audit.md`, `master-branch-status.md`.
- Updated `docs/DOCS_MAP.md` to reflect archived paths.

## Remaining Historical Candidates (Not Yet Archived)

### batchalign3 (4)

- `book/src/decisions/rust-migration-proposal.md`
- `book/src/developer/documentation-audit-2026-02-25.md`
- `book/src/developer/manual-anchor-audit.md`
- `rust/crates/talkbank-parser-tests/PARSER_TEST_AUDIT.md`

### talkbank-chat (18)

- `book/src/architecture/batchalign-warnings-audit.md`
- `book/src/architecture/check-parity-audit.md`
- `book/src/architecture/error-code-audit.md`
- `book/src/architecture/option-result-audit.md`
- `book/src/architecture/reorganization-roadmap.md`
- `book/src/architecture/sentinel-audit.md`
- `book/src/architecture/talkbank-clan-PORTING-ROADMAP.md`
- `book/src/architecture/talkbank-utils-PORTING-ROADMAP.md`
- `book/src/architecture/utf8-audit.md`
- `book/src/architecture/validation-completion-audit.md`
- `book/src/contributing/execution-checklist.md`
- `crates/talkbank-parser-tests/PARSER_TEST_AUDIT.md`
- `docs/audits/performance-audit-2026-02-24.md`
- `docs/doctests-assessment.md`
- `docs/validation/STRICT_TIMELINE_MODE_PROPOSAL.md`
- `...` (3 more in JSON report)

### talkbank-private (1)

- `batchalign/docs/fleet-management-plan.md`

## Notes

- `talkbank-chat` mdBook still builds after archive split (pre-existing warning in `architecture/ui-wishlist.md`).
- `batchalign3` already had an archive split from the prior pass and is included here for workspace completeness.
- Full machine-readable detail is in `analysis/docs-audit-workspace-2026-02-25.json`.
