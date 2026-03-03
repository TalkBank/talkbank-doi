# 02. Executive Summary

## Current state

The codebase is ambitious and technically strong in core parser/model areas, but quality maturity is uneven across repos. `talkbank-chat` has rigorous gates and architecture discipline; `talkbank-chatter`, `talkbank-clan`, and parts of `batchalign3` need consistency upgrades in CI, typing strictness, and interface stability.

## Top risks

1. Missing CI enforcement in some repos allows regressions to land.
2. Cross-repo duplication (`talkbank-chat` crates vendored in `batchalign3/rust`) creates drift risk.
3. Reliability-sensitive paths in `batchalign3` still use broad exception handling and dynamic typing.
4. Packaging metadata is inconsistent in multiple places (repo URLs/license metadata).
5. Grammar/query integration can pass parse suites while failing query validity.

## Highest-impact recommendations

1. Standardize quality gates across all repos (check, fmt, clippy/mypy, tests, generated artifact checks).
2. Introduce a synchronization strategy for shared Rust crates (subtree/submodule/publishing flow).
3. Tighten static typing in Python server/dispatch layers with protocol types and `TypedDict`/Pydantic models.
4. Add a compatibility matrix test layer for parser + grammar + LSP + extension queries.
5. Build one unified operational telemetry contract for CLI/server/LSP flows.

## 30-day success criteria

- `talkbank-chatter` and `talkbank-clan` have CI workflows and green baseline.
- Existing compile/query failures are fixed.
- Workspace-level `make verify-all` (or equivalent) exists and is documented.
- Metadata consistency issues are eliminated.
- Typed interfaces replace `Any` in priority reliability paths.

## Executive checklist

- [ ] Stabilize broken builds/tests now
- [ ] Implement minimum cross-repo CI parity
- [ ] Freeze and document shared crate sync policy
- [ ] Define typed API contracts for runtime-critical Python components
- [ ] Start quarterly performance and robustness scorecard
