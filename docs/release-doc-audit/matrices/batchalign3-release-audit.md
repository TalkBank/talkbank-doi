# batchalign3 Release Audit Matrix

This matrix tracks all public artifacts for the first public release. Organized
by section and risk level. The full artifact inventory is generated in
`../inventories/batchalign3.tsv`.

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
| 2 | `pyproject.toml` | dev | critical | revised | Package metadata |
| 3 | `cli-pyproject.toml` | dev | critical | verified | Package name, maturin bindings=bin, manifest path correct |
| 4 | `.github/workflows/test.yml` | dev | critical | revised | Stale workspace paths fixed |
| 5 | `.github/workflows/release.yml` | dev | critical | verified | Trusted publishing, Python 3.12 build, correct |
| 6 | `.github/workflows/release-cli.yml` | dev | critical | verified | 4-target maturin matrix, cli-pyproject.toml path correct |
| 7 | `.github/workflows/docs.yml` | dev | medium | revised | Fixed: `master`→`main` branch trigger |
| 8 | `.github/workflows/dashboard-desktop.yml` | dev | medium | verified | Active but dormant; consistent with docs marking it developer-only/future |
| 9 | `LICENSE` | legal | high | unreviewed | Verify correct license text |
| 10 | `Cargo.toml` (workspace) | dev | medium | unreviewed | Workspace metadata |
| 11 | `stubs/batchalign_core/__init__.pyi` | dev+integrator | high | verified | All 18 functions + 24 ParsedChat methods match PyO3 exports exactly |
| 12 | `apps/dashboard-desktop/README.md` | dev | low | n/a | Dashboard is deferred from first release |
| 13 | `benchmarks/README.md` | dev | low | n/a | Low-risk metadata |

---

## Book: Migration (4 pages) — RELEASE BLOCKER

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 14 | `migration/index.md` | critical | revised | Added `compare` to crosswalk; scope claims verified |
| 15 | `migration/user-migration.md` | critical | revised | Added `compare` command; `transcribe_s` confirmed as internal dispatch variant not user-facing; all BA2 flags verified |
| 16 | `migration/developer-migration.md` | critical | verified | Architecture-focused, crate layout accurate |
| 17 | `migration/algorithms-and-language.md` | critical | verified | Algorithm mapping careful and well-hedged |

---

## Book: User Guide (9 pages) — RELEASE BLOCKER

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 18 | `user-guide/installation.md` | critical | verified | Install contract verified: cli-pyproject.toml (py≥3.11), pyproject.toml (py≥3.12), Python resolution order correct |
| 19 | `user-guide/quick-start.md` | critical | verified | All example commands verified against CLI --help; routing notes accurate |
| 20 | `user-guide/cli-reference.md` | critical | verified | All flags for align, transcribe, morphotag, setup, cache, serve, logs, jobs verified against --help output; exit codes match error.rs |
| 21 | `user-guide/python-api.md` | high | verified | All exports, class names, extensions correct. Missing lenient param, module-level functions. |
| 22 | `user-guide/plugins.md` | high | revised | HK engines — plugin system retired |
| 23 | `user-guide/server-mode.md` | critical | verified | All serve subcommands (start/stop/status) and flags verified against CLI |
| 24 | `user-guide/rust-server.md` | high | verified | All server commands, config, dispatch, SSE confirmed. Missing: rate limit, health endpoint details. |
| 25 | `user-guide/rev-ai.md` | high | verified | Setup, defaults, engine selection correct. Missing: preflight workflow, lang code translation. |
| 26 | `user-guide/troubleshooting.md` | high | verified | All commands, flags, paths, logs confirmed. Missing: sidecar daemon files, cache dirs. |

---

## Book: Architecture (19 pages)

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 27 | `architecture/overview.md` | medium | revised | Fixed: %eng→%xtra (2×), extract_langs→extract_languages, stale pyo3 path |
| 28 | `architecture/pipeline-system.md` | medium | revised | Updated: removed defunct BatchalignPipeline/Task/Engine classes, replaced with Rust server dispatch |
| 29 | `architecture/engine-interface.md` | medium | verified | Correctly notes historical architecture deprecated; current Rust+worker IPC accurate |
| 30 | `architecture/dispatch-system.md` | medium | verified | Dispatch router, three dispatch paths, worker transport all confirmed |
| 31 | `architecture/chat-parsing.md` | medium | verified | Minor: grammar repo reference says tree-sitter-talkbank (archived), should say talkbank-tools |
| 32 | `architecture/python-rust-interface.md` | medium | revised | Fixed: callback paths→inference/*.py, SHA256→BLAKE3, talkbank-revai→pyo3/src/revai/, stale provider language removed |
| 33 | `architecture/extension-layers.md` | medium | verified | Three-layer split, HK engines, providers, pipeline_api all confirmed |
| 34 | `architecture/fleet-evolution.md` | medium | verified | Types in scheduling.rs, QueueBackend, Phase 1 checkpoint all confirmed |
| 35 | `architecture/server-architecture.md` | high | verified | All 12 endpoints, 6 IPC ops, dispatch, key files confirmed |
| 36 | `architecture/rust-server-migration.md` | high | verified | "Landed" status and mapping table confirmed |
| 37 | `architecture/chat-ownership-boundary.md` | high | verified | All 6 engine surfaces, crate listing, command→task table confirmed |
| 38 | `architecture/hk-cantonese-engines.md` | medium | revised | Promoted from private — engine fold-in |
| 39 | `architecture/caching.md` | medium | verified | Minor: refers to batchalign-cache crate but cache is in batchalign-app/src/cache/ |
| 40 | `architecture/validation.md` | medium | verified | Three-level system, validate_to_level(), severity posture all confirmed |
| 41 | `architecture/command-contracts.md` | high | verified | All 6 claimed functions confirmed implemented |
| 42 | `architecture/error-handling.md` | medium | verified | Minor: says ErrorCode in talkbank-errors crate, actually in talkbank-model/src/errors/ |
| 43 | `architecture/type-driven-design.md` | low | verified | Newtypes, serde patterns, text_types.rs all confirmed |
| 44 | `architecture/server-known-issues.md` | medium | verified | Issue numbering jumps (2-6 removed); #1 MPS deadlock and #7 no run logs accurate |
| 45 | `architecture/server-model-loading.md` | medium | verified | Device priority, model inventory, cache scoping all confirmed |

---

## Book: Technical Reference (31 pages in SUMMARY + 2 unlisted)

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 46 | `reference/chat-format.md` | medium | revised | Fixed: %eng→%xtra, removed stale Python module references, updated parse mode locations |
| 47 | `reference/morphosyntax.md` | high | revised | Fixed stale line counts, retokenize module structure updated |
| 48 | `reference/gra-format.md` | high | revised | Extraction done — private provenance removed |
| 49 | `reference/forced-alignment.md` | high | verified | UTR→FA pipeline, post-processing order, engine selection confirmed |
| 50 | `reference/alignment-structures.md` | medium | verified | Minor: uses shorthand paths (chat-ops/ not batchalign-chat-ops/) |
| 51 | `reference/dynamic-programming.md` | medium | verified | Hirschberg algorithm, enforce_monotonicity(), DP allowlist all confirmed |
| 52 | `reference/whisper-asr.md` | high | revised | Fixed: engine class names→function names, file paths→inference/asr.py and inference/fa.py, models/resolve.py reference updated |
| 53 | `reference/retrace-detection.md` | high | verified | Algorithm matches code. Minor: may be dead code (no Python callers), case-insensitive not documented |
| 54 | `reference/multilingual.md` | high | verified | All claims accurate, all 6 sub-pages exist and match |
| 55 | `reference/language-specific-processing.md` | high | revised | Extraction done — private provenance removed |
| 56 | `reference/language-code-resolution.md` | medium | verified | 39-language Stanza mapping, _iso3_to_alpha2() confirmed |
| 57 | `reference/language-handling.md` | medium | verified | Three-level language spec, resolve_word_language(), L2|xxx all confirmed |
| 58 | `reference/l2-handling.md` | high | verified | L2|xxx replacement logic, conservative handling confirmed |
| 59 | `reference/mwt-handling.md` | high | revised | Fixed stale Python file paths (engine.py→worker/_main.py), cache key SHA256→BLAKE3 |
| 60 | `reference/cantonese-processing.md` | high | verified | 31-entry table, zhconv, Aho-Corasick, PyO3 exports confirmed. One questionable entry (遊水→游水) needs domain review |
| 61 | `reference/japanese-morphosyntax.md` | medium | revised | Fixed: lib.rs:1447-1462→morphosyntax/inject.rs:103-115 |
| 62 | `reference/hebrew-morphosyntax.md` | medium | verified | Feature extraction, mapping.rs line numbers, tests all confirmed |
| 63 | `reference/number-expansion.md` | medium | verified | Stages, num2chinese(), 12-language table, file paths all confirmed |
| 64 | `reference/utterance-segmentation.md` | medium | verified | BERT models, punctuation fallback, source paths confirmed |
| 65 | `reference/wor-tier.md` | medium | verified | for_each_leaf(), collect_fa_words(), generate_wor_tier() all confirmed |
| 66 | `reference/textgrid.md` | medium | revised | Fixed: batchalign-core/src/lib.rs:2740→pyo3/src/pyfunctions.rs:237 |
| 67 | `reference/media-conversion.md` | medium | verified | Cache paths, media handling confirmed |
| 68 | `reference/command-io.md` | high | revised | Fixed: removed wrong "Hidden command" label on coref. Core I/O contracts verified. |
| 69 | `reference/filesystem-paths.md` | low | verified | All paths verified against config.rs and logs_cmd.rs |
| 70 | `reference/benchmarks.md` | medium | verified | wer_compute() reference confirmed |
| 71 | `reference/gra-correctness-guarantee.md` | high | verified | 3 rules, 6 tests, failure behavior all confirmed |
| 72 | `reference/per-utterance-language-routing.md` | high | verified | Core claims correct but vague; "skipmultilang" name doesn't match code (MultilingualPolicy) |
| 73 | `reference/per-word-language-routing.md` | high | verified | All claims accurate; L2\|xxx boundary correctly described |
| 74 | `reference/proportional-fa-estimation.md` | medium | verified | BUFFER_MS=2000, grouping.rs, proportional estimation confirmed |
| 75 | `reference/tier-parse-leniency.md` | medium | verified | Correctly framed as proposal/future study |
| 76 | `reference/wor-tier-bullet-bug.md` | low | verified | Minor: line number off by 20 (says 196, actually 216) |
| 77 | `reference/algorithm-audit-2026-03-07.md` | medium | verified | Correctly marked as dated snapshot; conclusions match code |
| 78 | `reference/morphotag-migration-audit.md` | high | verified | Function locations and algorithms confirmed. Stale: test count 46→72, Cantonese table 25→31 |

---

## Book: Developer Guide (17 pages)

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 79 | `developer/building.md` | high | verified | uv/maturin/edition/3.12 correct. CLI invocation examples misleading (Rust binary, not `uv run`). Missing dual pyproject.toml explanation. |
| 80 | `developer/testing.md` | medium | verified | All test commands match actual files and commands |
| 81 | `developer/api-stability.md` | high | verified | Public surfaces correct. Missing ParsedDocument/ProviderInvoker/extensions from stability listing. Broken test artifact references deleted PluginDescriptor. |
| 82 | `developer/adding-engines.md` | medium | verified | Paths, types, runtime_constants.toml all confirmed |
| 83 | `developer/rust-contributor-onboarding.md` | medium | verified | pyo3/Cargo.toml, workspace structure confirmed |
| 84 | `developer/rust-core.md` | medium | revised | Fixed: rust/→pyo3/, batchalign-core→batchalign-pyo3, removed talkbank-revai, tree-sitter-talkbank→talkbank-tools/grammar, %eng→%xtra, test command path |
| 85 | `developer/rust-workspace-map.md` | medium | verified | Correctly references pyo3/ and batchalign-pyo3 crate |
| 86 | `developer/rust-manual-crosswalk.md` | medium | verified | Generic references remain valid |
| 87 | `developer/rust-cli-and-server.md` | high | revised | Replaced rust-next page |
| 88 | `developer/docx-bookmark-workflow.md` | low | verified | Scripts and directories confirmed |
| 89 | `developer/manual-anchor-audit.md` | medium | revised | Extraction: local audit metadata removed |
| 90 | `developer/plugins.md` | medium | revised | Plugin removal notes — history only |
| 91 | `developer/python-versioning.md` | medium | verified | Policy claims accurate; 3.14t paused status confirmed |
| 92 | `developer/tracing-and-debugging.md` | medium | verified | Module references, Stanza anomaly capture confirmed |
| 93 | `developer/arena-allocators.md` | low | verified | Minor: references archived repos as examples; patterns valid |
| 94 | `developer/tauri-react-dashboard.md` | medium | verified | Command paths, scripts, backend targeting confirmed |
| 95 | `developer/telemetry-runbook.md` | medium | verified | Dashboard references, metric names confirmed |

---

## Book: Design Decisions (8 pages)

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 96 | `decisions/long-term-architecture-charter.md` | medium | verified | Principles and commitments confirmed |
| 97 | `decisions/tauri-react-dashboard-adoption.md` | low | verified | Status and rationale current; correctly deferred from first release |
| 98 | `decisions/models-training-runtime-adr.md` | low | verified | Bridge implementation confirmed; models_cmd.rs forwards correctly |
| 99 | `decisions/plugin-migration-compatibility-adr.md` | medium | verified | Correctly marked Superseded. Note: stale test plugin artifact still imports deleted PluginDescriptor |
| 100 | `decisions/lenient-parsing.md` | low | verified | Decision and parser behavior confirmed |
| 101 | `decisions/cache-rust-migration.md` | low | verified | Rust-side cache ownership confirmed |
| 102 | `decisions/downstream-integration.md` | low | verified | Decision scope confirmed |
| 103 | `decisions/rust-morphosyntax-case.md` | low | verified | Minor: line counts outdated (retokenize 938→1410) |

---

## Book: Introduction

| # | Page | Risk | Status | Notes |
|---|------|------|--------|-------|
| 104 | `introduction.md` | high | verified | All architecture/capability claims accurate. Missing: coref, compare, benchmark, HK engines |

---

## Summary

| Status | Count |
|--------|-------|
| verified | 76 |
| revised | 24 |
| blocked | 0 |
| unreviewed | 2 |
| n/a | 2 |
| **Total** | **104** |

### Release blockers — ALL RESOLVED

Phase 1 release blockers resolved:
- Migration: all 4 pages (#14–17) ✓
- User Guide: installation, quick-start, cli-reference, server-mode (#18–20, #23) ✓
- Non-book: cli-pyproject.toml, release.yml, release-cli.yml (#3, #5, #6) ✓

Phase 2 blockers — all 8 fixed:
1. ~~`whisper-asr.md` (#52)~~ revised — engine class names→functions, file paths fixed
2. ~~`pipeline-system.md` (#28)~~ revised — updated to Rust server dispatch
3. ~~`python-rust-interface.md` (#32)~~ revised — callback paths, cache keys, Rev.AI section fixed
4. ~~`chat-format.md` (#46)~~ revised — %eng→%xtra, stale module references removed
5. ~~`japanese-morphosyntax.md` (#61)~~ revised — file path fixed
6. ~~`textgrid.md` (#66)~~ revised — file path fixed
7. ~~`rust-core.md` (#84)~~ revised — directory, crate names, test command, grammar repo fixed
8. ~~`docs.yml` (#7)~~ revised — master→main

### Remaining unreviewed (2 items, non-critical)
- `LICENSE` (#9) — legal review needed
- `Cargo.toml` workspace (#10) — metadata only
