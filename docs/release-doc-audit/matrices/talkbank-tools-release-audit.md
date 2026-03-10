# talkbank-tools Release Audit Matrix

This matrix tracks all public artifacts for the first public release. Organized
by section and risk level. The full artifact inventory is generated in
`../inventories/talkbank-tools.tsv`.

Current pause/resume summary: `../STATUS-2026-03-09.md`

## Status key

- `unreviewed` — no manual review yet
- `in_review` — partially checked
- `revised` — corrected during this audit and rechecked
- `verified` — evidence-backed and safe to ship
- `extracted` — private/historical detail moved out of public docs
- `blocked` — release blocker remains
- `n/a` — not applicable or low-risk metadata only

---

## Non-Book Artifacts

| # | Artifact | Audience | Risk | Status | Notes |
|---|---------|----------|------|--------|-------|
| 1 | `README.md` | user+dev | critical | revised | Root public entry point |
| 2 | `crates/talkbank-cli/README.md` | dev | critical | revised | Crate install and usage |
| 3 | `.github/workflows/ci.yml` | dev | critical | revised | Fixed stale 339→73 corpus count |
| 4 | `.github/workflows/release.yml` | dev | critical | verified | 4-target matrix, tag validation, correct |
| 5 | `CONTRIBUTING.md` | dev | high | verified | Contributor guidelines confirmed |
| 6 | `LICENSE` | legal | high | unreviewed | Verify correct license text |
| 7 | `SECURITY.md` | user+dev | medium | verified | Security policy confirmed |
| 8 | `vscode/README.md` | user | high | revised | Fixed shortcut (F5→Shift+F5), command count |
| 9 | `vscode/package.json` | user | high | revised | Fixed stale repo URLs (talkbank-chatter→talkbank-tools) |
| 10 | `grammar/README.md` | dev | medium | verified | Grammar contributor entry confirmed |
| 11 | `spec/README.md` | dev | medium | verified | Spec contributor entry confirmed |
| 12 | `corpus/README.md` | dev | medium | verified | 73-file corpus structure confirmed |
| 13 | `examples/README.md` | user+dev | medium | verified | Examples entry confirmed |
| 14 | `Cargo.toml` (workspace) | dev | medium | verified | Workspace metadata confirmed |
| 15 | Crate READMEs (7 remaining) | dev | low | n/a | Low-risk metadata |

---

## Book: User Guide (7 pages) — RELEASE BLOCKER

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 16 | `user-guide/installation.md` | critical | verified | Correct: cargo install path, binary name, prereqs |
| 17 | `user-guide/cli-reference.md` | critical | revised | Must stay exact — spot-check against evidence |
| 18 | `user-guide/migrating-from-clan.md` | critical | revised | New migration page — spot-check CLAN manual claims |
| 19 | `user-guide/validation-errors.md` | high | revised | Fixed: E301 "Missing @Participants"→"Empty speaker code" (actual spec), added E504 note |
| 20 | `user-guide/vscode-extension.md` | high | verified | Architecture, features, build commands all confirmed |
| 21 | `user-guide/batch-workflows.md` | high | revised | Fixed: 339→73 corpus count, talkbank-tools→talkbank-chat cache path |
| 22 | `user-guide/chat-processing-playbook.md` | high | verified | General editorial guidance, no code mismatches |

---

## Book: CHAT Format (7 pages)

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 23 | `chat-format/overview.md` | high | verified | Tab separation, terminators, speaker codes all match grammar |
| 24 | `chat-format/headers.md` | high | verified | Required/optional headers, ordering match grammar |
| 25 | `chat-format/utterances.md` | high | verified | Main tier structure, terminators table, content items confirmed |
| 26 | `chat-format/dependent-tiers.md` | high | revised | %cod semantic-layer note promoted from private |
| 27 | `chat-format/mor-tier.md` | high | revised | Fixed 339→73 corpus count; data model structures verified |
| 28 | `chat-format/word-syntax.md` | medium | verified | Compounds, forms, markers, error marking all match grammar |
| 29 | `chat-format/symbols.md` | medium | verified | Symbol registry reference, CA delimiters, terminators confirmed |

---

## Book: Architecture (22 pages)

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 30 | `architecture/overview.md` | medium | verified | Crate map, data flow, directory layout correct |
| 31 | `architecture/spec-system.md` | medium | verified | Spec types, test generation, workflow correct |
| 32 | `architecture/grammar.md` | medium | revised | Fixed 339→73 corpus count |
| 33 | `architecture/parsing.md` | medium | revised | Fixed 339→73 corpus count |
| 34 | `architecture/data-model.md` | medium | verified | ChatFile, dependent tiers, content walker confirmed |
| 35 | `architecture/transform-pipeline.md` | medium | verified | Pipeline stages and cache lifecycle confirmed |
| 36 | `architecture/error-system.md` | medium | verified | Error code ranges, severity tiers, ErrorSink confirmed |
| 37 | `architecture/crate-reference.md` | medium | verified | All 10 crate names and purposes confirmed |
| 38 | `architecture/repo-architecture.md` | medium | verified | Directory structure and ownership rules confirmed |
| 39 | `architecture/grammar-governance.md` | low | verified | Symbol registry governance confirmed |
| 40 | `architecture/grammar-stakeholders.md` | low | verified | Stakeholder analysis confirmed |
| 41 | `architecture/grammar-redesign.md` | medium | revised | Fixed: grammar.js 2,457→1,990, parser.c 46,393→26,687, conflicts 11→5, node types ~441→~370 |
| 42 | `architecture/parser-model-contracts.md` | medium | verified | Parser contract model confirmed |
| 43 | `architecture/parser-error-recovery.md` | medium | verified | Recovery comparison confirmed |
| 44 | `architecture/leniency-policy.md` | medium | verified | Three-tier classification confirmed |
| 45 | `architecture/error-diagnostics-ux.md` | medium | verified | Diagnostic schema confirmed |
| 46 | `architecture/spec-tooling.md` | medium | verified | Spec tool pipeline confirmed |
| 47 | `architecture/symbol-registry.md` | medium | verified | Symbol registry governance confirmed |
| 48 | `architecture/alignment.md` | medium | verified | Alignment module, traits, walker confirmed |
| 49 | `architecture/memory-and-ownership.md` | low | verified | String representation, interning confirmed |
| 50 | `architecture/algorithms.md` | low | verified | Parsing strategies, alignment algorithms confirmed |
| 51 | `architecture/concurrency.md` | low | verified | Threading, async, worker pool confirmed |
| 52 | `architecture/performance-optimizations.md` | low | verified | Optimization decisions confirmed |

---

## Book: Contributing (16 pages)

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 53 | `contributing/setup.md` | high | verified | Dev setup and toolchain confirmed |
| 54 | `contributing/grammar-workflow.md` | medium | revised | Fixed 339→73 corpus count; 4-step workflow correct |
| 55 | `contributing/spec-workflow.md` | medium | verified | Spec change workflow confirmed |
| 56 | `contributing/testing.md` | medium | revised | Fixed 339→73 corpus count; nextest-vs-doctest caveat promoted; gate numbering uses old G0-G7 scheme |
| 57 | `contributing/coding-standards.md` | medium | verified | Standards confirmed |
| 58 | `contributing/coding-standards-extended.md` | medium | verified | Extended standards confirmed |
| 59 | `contributing/ci-and-release.md` | high | revised | Fixed 339→73 corpus count; gate numbering inconsistent with Makefile (G0-G7 vs G0-G10) |
| 60 | `contributing/quality-gates.md` | medium | verified | Quality gates confirmed |
| 61 | `contributing/documentation-architecture.md` | low | verified | Doc structure confirmed |
| 62 | `contributing/chat-processing-playbook.md` | medium | verified | Developer playbook confirmed |
| 63 | `contributing/error-testing-consolidation.md` | low | verified | Error test plan confirmed |
| 64 | `contributing/open-source-governance.md` | high | verified | Governance model confirmed |
| 65 | `contributing/compile-times.md` | low | verified | Build perf notes confirmed |
| 66 | `contributing/dev-checks.md` | medium | verified | Dev check commands confirmed; uses subset of gates |
| 67 | `contributing/branch-protection.md` | medium | verified | Branch rules confirmed |
| 68 | `contributing/reference-corpus.md` | medium | verified | Corpus docs confirmed; describes 73-file structure |

---

## Book: Integrating (4 pages) — RELEASE BLOCKER

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 69 | `integrating/library-usage.md` | critical | revised | Fixed TreeSitterParser::new() Result, WriteChat Vec→String |
| 70 | `integrating/json-output.md` | critical | revised | MAJOR rewrite: fixed utterance/word/tier/alignment structures |
| 71 | `integrating/json-schema.md` | critical | verified | Schema URL, --url flag, generate test all confirmed |
| 72 | `integrating/diagnostic-contract.md` | critical | revised | Added cached field to single-file output |

---

## Book: Technical Reference (16 pages)

Historical audits and design decisions retained for developer reference.

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 73 | `architecture/executive-summary.md` | medium | verified | High-level summary confirmed |
| 74 | `architecture/end-user-experience.md` | medium | verified | UX contract confirmed |
| 75 | `architecture/risk-register.md` | low | verified | Risk tracking confirmed |
| 76 | `architecture/option-result-audit.md` | low | verified | Code quality audit confirmed |
| 77 | `architecture/permissiveness-regression.md` | medium | verified | Regression tracking confirmed |
| 78 | `architecture/passive-stub-migration.md` | low | verified | Stub migration plan confirmed |
| 79 | `architecture/check-parity-audit.md` | high | revised | Extraction: local provenance removed |
| 80 | `architecture/sentinel-audit.md` | low | verified | Sentinel value audit confirmed |
| 81 | `architecture/utf8-audit.md` | low | verified | UTF-8 correctness confirmed |
| 82 | `architecture/validation-completion-audit.md` | high | revised | Extraction: local provenance removed |
| 83 | `architecture/batchalign-warnings-audit.md` | medium | verified | Cross-repo warnings confirmed |
| 84 | `architecture/async-rust-analysis.md` | low | verified | Async analysis confirmed |
| 85 | `architecture/performance-future-work.md` | low | verified | Future perf work confirmed |
| 86 | `architecture/talkbank-clan-PORTING-ROADMAP.md` | high | verified | CLAN parity claims confirmed |
| 87 | `contributing/acceptance-test-matrix.md` | medium | verified | Test coverage matrix confirmed |
| 88 | `contributing/maintenance-model.md` | low | verified | Maintenance plan confirmed |

---

## Book: Introduction

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 89 | `introduction.md` | high | revised | Fixed 339→73 corpus count; all other claims verified |

---

## Summary

| Status | Count |
|--------|-------|
| verified | 64 |
| revised | 23 |
| blocked | 0 |
| unreviewed | 1 |
| n/a | 1 |
| **Total** | **89** |

### Release blockers — ALL RESOLVED

Phase 1 release blockers resolved:
1. ~~User Guide: `installation.md` (#16)~~ verified
2. ~~Integrating: all 4 pages (#69–72)~~ 3 revised + 1 verified
3. ~~CI workflow: `ci.yml` (#3)~~ revised
4. ~~Release workflow: `release.yml` (#4)~~ verified
5. ~~VS Code: `vscode/README.md` + `package.json` (#8–9)~~ revised

Phase 3 blockers — both fixed:
1. ~~`validation-errors.md` (#19)~~ revised — E301 "Missing @Participants"→"Empty speaker code"
2. ~~`grammar-redesign.md` (#41)~~ revised — all metrics updated to current values

### Phase 3 fixes applied

- Fixed 339→73 corpus count in 8 book pages (#21, #27, #32, #33, #54, #56, #59, #89)
- Fixed cache path talkbank-tools→talkbank-chat (#21)
- Fixed E301 error code description (#19)
- Updated grammar metrics to current values (#41)

### Remaining

- `LICENSE` (#6) — legal review needed
- Gate numbering inconsistency across contributing pages (G0-G7 vs G0-G10 in Makefile) — noted, not blocking
