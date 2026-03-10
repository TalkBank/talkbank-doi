# Private Promotion Candidates

This matrix tracks private material that contained public-facing technical
detail worth promoting back into the public repos after verification.

| Source private doc | Public destination | Status | Notes |
|---|---|---|---|
| `docs/batchalign3-reports/root/BATCHALIGN2_DELTA.md` | `batchalign3/book/src/migration/user-migration.md` | promoted 2026-03-09 | Used to add the explicit BA2 compatibility no-op flag list and cache delta (`cache clear --all`, no `cache warm`) |
| `archive/batchalign-plugin-system.md` | `batchalign3/book/src/architecture/hk-cantonese-engines.md` and `batchalign3/book/src/developer/plugins.md` | promoted 2026-03-09 | Used only for historical provenance and removal rationale; current docs rewritten around built-in HK engines |
| `batchalign/docs/hk-plugin.md` | `batchalign3/book/src/user-guide/plugins.md` and `batchalign3/book/src/architecture/hk-cantonese-engines.md` | promoted 2026-03-09 | Private doc itself was rewritten as historical after fold-in; durable install/runtime details were re-verified against code before promotion |
| `archive/talkbank-tools-docs/dependent-tier-semantics-audit.md` | `talkbank-tools/book/src/chat-format/dependent-tiers.md` | promoted 2026-03-09 | Added the `%cod` semantic-layer note so public docs no longer imply `%cod` is only free text in command semantics |
| `archive/talkbank-tools-internal/audits/doctests-assessment.md` | `talkbank-tools/book/src/contributing/testing.md` | promoted 2026-03-09 | Added the missing nextest-vs-doctest execution note |
