# Documentation Revision Matrix (2026-03-05)

## Scope

This plan covers the published docs in `batchalign3/book/src` (the pages listed in `book/src/SUMMARY.md`).

Inventory snapshot:
- Published pages in navigation: 90
- Archive pages: 37 (`docs/archive`)

Goal:
- Align published docs to current implementation (Rust control plane + Python ML workers).
- Move stale planning/proposal content to explicit historical context.
- Make user and contributor docs operationally correct as of March 5, 2026.
- Make `book/src/migration/` the authoritative public home for durable
  `batchalign2` -> `batchalign3` deltas, anchored to:
  - baseline `84ad500b09e52a82aca982c41a8ccd46b01f4f2c` (2026-01-09)
  - released master-branch point `e8f8bfada6170aa0558a638e5b73bf2c3675fe6d`
    (2026-02-09) only where that reflects the last shipped BA2 surface

## Canonical Truth Sources

Use these as ground truth during edits:

1. Runtime architecture and worker protocol:
   - `book/src/architecture/rust-server-migration.md`
   - `rust-next/crates/batchalign-worker/src/lib.rs`
   - `rust-next/crates/batchalign-types/src/worker.rs`
2. Command behavior and dispatch:
   - `rust-next/crates/batchalign-cli/`
   - `rust-next/crates/batchalign-server/`
   - `batchalign/worker/_main.py`, `batchalign/worker/_handlers.py`
3. Inference engine behavior:
   - `batchalign/inference/*.py`
4. Real behavior safety net:
   - `rust-next` integration/e2e tests
   - `batchalign/tests/`

Rule: if docs disagree with code/tests, update docs to code/tests unless a code bug is confirmed.

Migration-book rule:
- preserve durable differences in correctness, performance model, algorithms,
  and data structures relative to the Jan 9 BA2 baseline;
- do not document transient unreleased intermediate states;
- use the Feb 9 master-branch point only when it is the last shipped BA2 comparison
  point before BA3 divergence.

## Triage Labels

- `keep`: accurate and useful as-is (minor edits only).
- `revise`: keep page, rewrite sections to current state.
- `split`: page is too mixed (history + current); split and archive history.
- `archive`: historical/proposal content not suitable for live book navigation.
- `deprecate-link`: keep page but remove from high-visibility nav and point to successor.

Priority:
- `P0`: misleading for users/operators now.
- `P1`: contributor architecture drift.
- `P2`: reference depth/cleanup.

## P0 Backlog (Do First)

| File | Current issue | Action | Priority |
|---|---|---|---|
| `book/src/decisions/rust-migration-proposal.md` | Still framed as proposal/phases; partially superseded by implemented architecture | `split` into: short ADR outcome + historical archive link | P0 |
| `book/src/architecture/command-contracts.md` | Marked proposed/not enforced | `revise` with explicit current enforcement status and code links | P0 |
| `book/src/developer/pre-commit.md` | Proposal-style rollout phases | `revise` to actual current workflow; move roadmap text to archive note | P0 |
| `book/src/user-guide/server-mode.md` | Must reflect current server + worker behavior exactly | `revise` against `batchalign-server` endpoints and runtime behavior | P0 |
| `book/src/user-guide/rust-server.md` | Risk of stale setup/ops steps | `revise` with validated commands and expected outputs | P0 |
| `book/src/user-guide/cli-reference.md` | Needs parity with current Rust CLI | `revise` from `batchalign-cli` command tree | P0 |
| `book/src/user-guide/plugins.md` | Mentions transitional install caveats | `revise` with current plugin contract + install path | P0 |
| `book/src/architecture/server-known-issues.md` | Mixed fixed/open items, potentially stale mitigations | `revise` into `Open`, `Mitigated`, `Resolved` with dates | P0 |
| `book/src/SUMMARY.md` | Keeps proposal-era labels in top-level navigation | `revise` titles/order to emphasize current-state docs first | P0 |

Acceptance criteria for P0:
- No P0 page contains ambiguous proposal language without explicit historical framing.
- User guide command behavior matches executable CLI/server behavior.
- Every operational claim is anchored to a current code path.

## P1 Backlog (Architecture/Developer Truth Alignment)

| File | Current issue | Action | Priority |
|---|---|---|---|
| `book/src/architecture/overview.md` | Needs explicit “current architecture” framing | `revise` | P1 |
| `book/src/architecture/chat-parsing.md` | Still links migration proposal for core explanation | `revise` references to current pages | P1 |
| `book/src/architecture/python-rust-interface.md` | Needs consistency with worker protocol and `batch_infer` ownership | `revise` | P1 |
| `book/src/architecture/server-architecture.md` | Validate endpoint/feature matrix against server code | `revise` | P1 |
| `book/src/architecture/rust-server-migration.md` | Rename/reframe from migration narrative to implemented model | `deprecate-link` + add successor page or rewrite in place | P1 |
| `book/src/architecture/caching.md` | Phase-oriented sections mixed with current behavior | `revise` into current behavior + roadmap appendix | P1 |
| `book/src/architecture/validation.md` | In-progress phase text likely stale | `revise` with implemented warnings/errors and open gaps | P1 |
| `book/src/developer/rust-next.md` | Verify contributor setup exactness | `revise` | P1 |
| `book/src/developer/rust-workspace-map.md` | Validate crate map against current workspace | `revise` | P1 |
| `book/src/developer/testing.md` | Ensure test commands and scope match current suites | `revise` | P1 |
| `book/src/developer/tracing-and-debugging.md` | Might over-index on phased migration language | `revise` | P1 |

Acceptance criteria for P1:
- Architecture pages describe implemented behavior, not transition intent.
- Contributor docs allow a clean setup/test/debug flow without dead steps.

## P2 Backlog (Reference Consistency and Cleanup)

| File | Current issue | Action | Priority |
|---|---|---|---|
| `book/src/reference/utr-alignment.md` | Phase terminology could be algorithmic or historical; clarify | `revise` wording | P2 |
| `book/src/reference/per-word-language-routing.md` | Contains future-work sections; separate speculation | `split` | P2 |
| `book/src/reference/per-utterance-language-routing.md` | Contains pending/next-step language | `revise` | P2 |
| `book/src/reference/python-gra-generation-analysis.md` | Ensure clearly marked as historical/comparative if still needed | `split` or `archive` | P2 |
| `book/src/reference/language-handling.md` | Confirm future/deferred notes still accurate | `revise` | P2 |
| `book/src/reference/benchmarks.md` | Ensure benchmark provenance/date explicitly stated | `revise` | P2 |
| `book/src/decisions/lenient-parsing.md` | Proposal-heavy; needs status outcome section | `revise` | P2 |
| `book/src/decisions/google-translate-migration.md` | Deferred language may be stale | `revise` | P2 |
| `book/src/decisions/rust-morphosyntax-case.md` | Mark final outcomes and current boundaries | `revise` | P2 |

Acceptance criteria for P2:
- Reference pages distinguish implemented behavior vs ideas.
- Any future-work notes are brief, dated, and non-authoritative.

## Pages Likely Keep-Only (Quick Verification Pass)

Perform a quick factual pass, minimal edits only:
- `book/src/user-guide/installation.md`
- `book/src/user-guide/quick-start.md`
- `book/src/user-guide/rev-ai.md`
- `book/src/user-guide/troubleshooting.md`
- `book/src/architecture/error-handling.md`
- `book/src/architecture/type-driven-design.md`
- `book/src/reference/forced-alignment.md`
- `book/src/reference/dynamic-programming.md`
- `book/src/reference/wor-tier.md`
- `book/src/developer/documentation-scope.md`
- `book/src/developer/documentation-readiness.md`

## Execution Sequence

1. P0 rewrite sprint (user-facing correctness).
2. P1 architecture/developer synchronization.
3. P2 reference cleanup.
4. Navigation cleanup in `SUMMARY.md`.
5. Archive and cross-link historical docs.

## Progress Update (2026-03-05)

Completed in initial pass:
- `book/src/decisions/rust-migration-proposal.md` reclassified as historical/superseded and linked to current architecture docs.
- `book/src/architecture/command-contracts.md` status updated to partially implemented with concrete enforcement sources.
- `book/src/developer/pre-commit.md` reclassified as historical proposal draft.
- `book/src/architecture/server-known-issues.md` split into open issues vs resolved incidents.
- `book/src/SUMMARY.md` labels updated for proposal/historical clarity.
- `book/src/user-guide/cli-reference.md` corrected for current Rust CLI behavior on key flags and verbosity mapping.
- `book/src/user-guide/server-mode.md` and `book/src/user-guide/rust-server.md` now include status metadata headers.
- `book/src/user-guide/plugins.md` removed date-sensitive packaging wording.
- `book/src/architecture/chat-parsing.md` now links to current architecture pages (not proposal-era roadmap).
- `book/src/architecture/overview.md` and `book/src/architecture/python-rust-interface.md` now include status metadata headers.
- `book/src/architecture/server-architecture.md` rewritten to implementation-first current state (single-server + local daemon active; fleet fan-out explicitly marked disabled in this release); `validation.md` now includes status metadata and clearer historical labeling.
- `book/src/architecture/dispatch-system.md`, `book/src/user-guide/rust-server.md`, and `book/src/user-guide/cli-reference.md` now align with current dispatch reality (single `--server` + local daemon; no active multi-server fan-out).
- Migration/reference pages (`migration/index.md`, `migration/developer-migration.md`, `migration/user-migration.md`, `reference/filesystem-paths.md`, `reference/command-io.md`) now reflect current fleet status and avoid implying active fan-out dispatch.
- Contributor/developer docs updated for current module layout and routing reality (`developer/rust-manual-crosswalk.md`, `developer/rust-next.md`, `developer/rust-workspace-map.md`, `developer/python-versioning.md`, `developer/python-314t-migration.md`, `architecture/rust-server-migration.md`).
- Runtime policy update applied: 3.14t targeting marked paused across high-visibility docs (`developer/python-versioning.md`, `developer/python-314t-migration.md`, `developer/building.md`, `user-guide/rust-server.md`, `user-guide/server-mode.md`, nav label in `SUMMARY.md`), with historical context preserved.
- Residual 3.14t mentions in reference docs now explicitly read as experimental/historical (`reference/morphosyntax.md`, `reference/multi-file-optimization.md`); `user-guide/quick-start.md` no longer promises a fixed cold-start duration.
- Additional implementation-state corrections landed: `reference/proportional-fa-estimation.md` now marked implemented, `reference/textgrid.md` marked current export support (not TODO), `reference/python-gra-generation-analysis.md` marked historical, and `architecture/{caching,validation}.md` better distinguish live behavior from historical proposals.
- `book/src/architecture/rust-server-migration.md` and nav labels now explicitly mark this as implemented.
- Migration-book scope has been tightened: durable BA2 baseline-to-BA3 deltas
  stay public, while historical analysis pages should move private only after
  their migration-relevant substance is extracted into `book/src/migration/`.
- Migration pages now explicitly anchor comparisons to Jan 9 baseline
  `84ad500...`, optionally Feb 9 released master-branch point `e8f8bfa...`, and
  current `batchalign3`, while excluding transient unreleased branch states.
- Migration wording for UTR / DP has been tightened against code: released BA2
  still uses `bulletize_doc(...)` rough utterance timing recovery and DP-heavy
  Python FA mapping, while current BA3 uses ID-first and window-constrained
  monotonic UTR transfer plus deterministic `rust-next` FA response handling.
- Migration framing now explicitly centers the cross-command engineering shift
  from string/array repair and broad flattened-text DP recovery toward stable
  identity, explicit indexing/chunk maps, AST iteration, and narrower
  deterministic fallback policy.
- Migration chapters now also distinguish command-family orchestration changes:
  `transcribe` as raw-ASR-plus-Rust-assembly, `translate` as pure text infer
  plus Rust injection, `utseg` as tree-return plus Rust assignment, and
  `coref` as document-level sparse Rust-injected annotation.
- Migration pages now name concrete morphotag correctness fixes with current
  code/test backing instead of leaving the topic at architecture-level
  generalities.
- The morphotag section now also distinguishes older BA2 behaviors that already
  existed (`reflx`, special-form handling, `xbxxx` restoration) from more
  clearly current-BA3-specific hardening (ROOT/head/chunk validation and
  chunk-based `%gra` indexing).
- Migration pages now distinguish the public Jan 9 BA2 -> Feb 9 BA2 -> current
  BA3 story for `transcribe`, `translate`, `utseg`, and `coref`, and no longer
  overclaim that BA3 introduced document-level coref.
- Migration audit now also covers `benchmark`, `opensmile`, and `avqi`, with
  the same Jan 9 / Feb 9 / current separation and with emphasis on typed
  runtime boundaries rather than overstated algorithm changes where none
  occurred.
- Migration and plugin docs no longer treat HK-specific material as part of the
  core batchalign3 book, and the migration command-surface story now separates
  Jan 9 BA2, Feb 9 BA2, and current BA3 for utility commands as well as
  processing commands.
- The dead `fleet` CLI subcommand has been removed from the Rust command tree,
  binary wiring, tests, and public migration/architecture docs; a separate
  command-retention audit now classifies remaining post-Jan-9 additions against
  the Jan 9 BA2 compatibility floor.
- The untested `gui` CLI launcher has been removed from the first-release
  command surface, while the web dashboard remains documented as current and the
  Tauri desktop wrapper remains in-tree as deferred work.
- The migration book is now partitioned cleanly by responsibility:
  `migration/index.md` as summary layer, `user-migration.md` for user-visible
  command/runtime/output deltas, `developer-migration.md` for architectural and
  control-plane changes, and `algorithms-and-language.md` for mechanism-level
  alignment/retokenization/%gra/DP-policy detail.
- Residual ambiguous “old/new” wording has been removed from the migration
  chapters in favor of explicit Jan 9 BA2, Feb 9 BA2, and current BA3
  references.
- Outer-layer docs and navigation now better reflect the cleaned public story:
  `introduction.md` points migrators to the migration book, historical UTR and
  master-tier references are labeled as historical in navigation, and
  documentation meta-pages have been refreshed to current 2026-03-05 state.

Next focus:
- continue outer-layer and navigation cleanup so historical pages do not compete
  with current migration/reference pages, then finish any remaining P1
  architecture synchronization that still affects current readers.

## Working Conventions for This Revision

For each edited page:
- Add/update a top metadata block:
  - `Status: Current | Historical`
  - `Last verified: YYYY-MM-DD`
  - `Verified against: <paths/tests>`
- Replace ambiguous timeline language:
  - bad: “will”, “phase 2”, “not yet approved” (without context)
  - good: “as of 2026-03-05”, “historical plan (superseded)”
- Keep history, but move it below current behavior or to archive.

## Output Artifacts to Produce

- `analysis/docs-revision-matrix-2026-03-05.md` (this file)
- `analysis/docs-revision-log.md` (append-only, page edits + verification source)
- Updated `book/src/SUMMARY.md` navigation after P0/P1 stabilization
- `analysis/docs-public-private-triage-2026-03-05.md` (public vs private move plan)

## Public/Private Progress

Executed first-wave de-nav + stub pass:
- removed `reference/python-gra-generation-analysis.md` from public nav
- removed `reference/multi-file-optimization.md` from public nav
- removed `developer/pre-commit.md` from public nav
- replaced each with a short public stub for link stability
