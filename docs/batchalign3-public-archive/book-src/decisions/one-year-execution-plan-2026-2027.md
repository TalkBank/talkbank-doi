# One-Year Execution Plan (March 2026 - February 2027)

## Date

February 24, 2026

## Status

Proposed

## Purpose

Convert the long-horizon architecture roadmap into a concrete 12-month program
with delivery sequence, acceptance gates, and measurable outcomes.

This plan is the execution layer for:

1. [Long-Term Architecture Charter](long-term-architecture-charter.md)
2. [Ten-Year Platform Roadmap](ten-year-platform-roadmap.md)
3. [Off-the-Shelf Framework ADR](off-the-shelf-framework-adoption-adr.md)
4. [Framework Adoption Backlog](../developer/framework-adoption-backlog.md)

## Planning Assumptions

1. Primary product surfaces: Rust CLI (`batchalign3`), Rust server control plane,
   and Web dashboard.
2. Python remains only model/runtime support where Rust replacement is not yet
   complete.
3. Provider/domain integration continues through external plugins (for example,
   HK plugin), not core forks.
4. No release automation or GitHub publication changes are part of this plan.

## Program Structure

Execution is organized into four quarters:

1. Q1 (Mar-May 2026): contract hardening and baseline reliability
2. Q2 (Jun-Aug 2026): control-plane resilience and dashboard operability
3. Q3 (Sep-Nov 2026): migration closure and performance scaling
4. Q4 (Dec 2026-Feb 2027): compatibility governance and platform readiness

Each quarter has mandatory acceptance gates. Work does not advance if gates fail.

## Q1 (Mar-May 2026): Contract Hardening

## Goals

1. Eliminate schema drift risk.
2. Establish typed command/API boundaries.
3. Start dashboard server-state modernization.

## Workstream A: Server

Checklist:

- [x] Implement `utoipa` schema derivations for core server request/response types.
- [x] Add deterministic schema generation command in `rust-next`.
- [x] Add CI schema drift gate against committed OpenAPI artifact.
- [x] Add correlation ID propagation across request/job lifecycle logs.

Acceptance criteria:

1. CI fails on uncommitted schema drift.
2. Correlation IDs visible in logs for submit -> queue -> completion path.
3. Generated schema is canonical input for frontend type generation.

## Workstream B: Dashboard

Checklist:

- [x] Introduce `@tanstack/react-query` provider and defaults.
- [x] Migrate job list/detail fetch flows to typed query hooks.
- [x] Wire websocket updates to cache invalidation/patching.
- [x] Define endpoint-specific retry and stale-time policy.

Acceptance criteria:

1. No page-level ad hoc `fetch` logic for job list/detail.
2. Reconnect behavior is deterministic and documented.
3. Typed API usage is enforced in the migrated flows.

## Workstream C: CLI

Checklist:

- [x] Audit command names/help text to ensure only `batchalign3` is documented.
- [x] Validate deterministic binary resolution (`BATCHALIGN_RUST_BIN` and install path).
- [x] Document and test `~/.batchalign.ini` compatibility behavior.
- [x] Standardize exit-code mapping for top-level command failures.

Acceptance criteria:

1. User docs and help text consistently reference `batchalign3`.
2. Config compatibility path is explicitly documented and tested.
3. Exit-code behavior is stable for automation scripts.

## Q2 (Jun-Aug 2026): Resilience and Operability

## Goals

1. Make overload behavior predictable.
2. Improve operator UX under failure/degraded conditions.
3. Expand automated regression safety nets.

## Workstream A: Server

Checklist:

- [x] Add rate limiting middleware for critical endpoints.
- [x] Add timeout middleware with explicit timeout error mapping.
- [ ] Add overload/backpressure tests under burst conditions.
- [x] Add OTLP export path for traces/metrics via `tracing`.

Acceptance criteria:

1. Burst traffic does not produce unbounded latency growth.
2. Timeout and rate-limit responses are deterministic and documented.
3. Core service metrics and traces are queryable in the telemetry backend.

## Workstream B: Dashboard

Checklist:

- [x] Add explicit offline/reconnecting UI states.
- [ ] Add Dioxus route-level failure boundaries around key page regions.
- [x] Implement failure remediation messaging for common REST/WS error classes.
- [x] Add dashboard runbook section for reconnect/failure behavior.

Acceptance criteria:

1. Operator can distinguish offline, retrying, and hard-failure states.
2. Recoverable failures do not collapse entire page state.
3. Runbook matches actual UX behavior during outage simulation.

## Workstream C: Quality Gates

Checklist:

- [ ] Add Dioxus-focused unit/component tests for job list/detail/actions/filter flows.
- [x] Add `playwright` smoke tests for critical operator paths.
- [x] Gate CI on dashboard smoke pass.

Acceptance criteria:

1. CI blocks merges on critical dashboard regressions.
2. At least one end-to-end operator path is fully automated.

## Q3 (Sep-Nov 2026): Migration Closure and Scale

## Goals

1. Close remaining Python CLI/server dependencies.
2. Improve scheduling and performance behavior.
3. Lock plugin contract and compatibility matrix.

## Workstream A: Rust-First Migration

Checklist:

- [ ] Identify any remaining Python CLI/server command routing and remove it.
- [ ] Ensure Rust server can run as primary path for all supported workflows.
- [ ] Keep Python only for model/runtime modules not yet ported.
- [ ] Publish migration matrix: old path -> Rust path -> status.

Acceptance criteria:

1. No production workflow depends on legacy Python CLI/server routing.
2. Migration matrix is complete and reviewable.
3. Any Python residual usage is explicitly bounded and justified.

## Workstream B: Performance and Concurrency

Checklist:

- [ ] Define workload classes (CPU-heavy, IO-heavy, model-heavy) and queue policy.
- [ ] Add scheduler abstraction boundaries for future pluggable policies.
- [ ] Add benchmarks for queue wait, throughput, and p95/p99 latency.
- [x] Execute persistence spike (`rusqlite` vs `sqlx`) and capture decision.

Acceptance criteria:

1. Workload-class behavior is test-covered and documented.
2. Performance regressions are detectable via benchmark baselines.
3. Persistence decision has written rationale and migration/deferral plan.

## Workstream C: Plugin Governance

Checklist:

- [ ] Declare host/plugin interface v1 with semantic version policy.
- [ ] Add compatibility matrix for host version vs supported plugin ranges.
- [ ] Add CI checks for plugin discovery and compatibility contract assertions.
- [ ] Confirm HK plugin remains external (`~/talkbank/batchalign-hk-plugin`).

Acceptance criteria:

1. Plugin compatibility is machine-checked, not manual.
2. Core does not re-inline provider-specific HK logic.

## Workstream D: Dioxus Dashboard Migration

Checklist:

- [x] Create Rust-native Dioxus dashboard scaffold in `rust-next`.
- [x] Document static build/deploy path via existing dashboard hosting contract.
- [x] Port job detail/actions and websocket updates to Dioxus UI.
- [x] Define parity gate and deprecation plan for React dashboard.

Acceptance criteria:

1. Dioxus dashboard can render live job list from production server API.
2. Migration can proceed incrementally without changing server route contracts.
3. Cutover is blocked until parity checks pass.

## Q4 (Dec 2026-Feb 2027): Governance and Readiness

## Goals

1. Institutionalize compatibility and deprecation policy.
2. Strengthen maintainership and succession readiness.
3. Prepare for multi-node evolution in the next annual cycle.

## Workstream A: Contract and Deprecation Policy

Checklist:

- [ ] Publish CLI/API/plugin deprecation windows and policy.
- [ ] Add release checklist requiring migration notes for breaking changes.
- [ ] Add architecture review template for major subsystem changes.
- [ ] Add rollback playbook template for control-plane changes.

Acceptance criteria:

1. Breaking changes cannot land without migration guidance.
2. Architecture decisions are documented with alternatives and rollback plan.

## Workstream B: Documentation and Operability

Checklist:

- [ ] Complete user guide updates for server mode, plugins, and config.
- [ ] Complete developer runbooks for telemetry, outages, and plugin debugging.
- [ ] Add onboarding docs for maintainers (ownership map + escalation path).
- [ ] Run a docs validation pass against live commands and examples.

Acceptance criteria:

1. New maintainer can execute core workflows from docs only.
2. Operational runbooks cover top incident categories.

## Workstream C: Annual Readiness Review

Checklist:

- [ ] Run annual architecture review against charter decision rules.
- [ ] Score roadmap progress against all 12-month acceptance gates.
- [ ] Identify carry-over risks and lock next-year objectives.
- [ ] Publish annual platform report with metrics and outcomes.

Acceptance criteria:

1. Annual report contains objective evidence, not narrative-only status.
2. Next-year backlog is derived from measured gaps.

## Metrics and Targets (for this 12-month cycle)

Track and review monthly:

1. Schema drift incidents: target 0 unreviewed drifts merged.
2. CLI compatibility regressions: target 0 unannounced breaking changes.
3. Dashboard critical workflow test pass rate: target >= 99% on main branch.
4. p95 submit-to-start queue latency under defined baseline load: target
   stable quarter-over-quarter.
5. Plugin compatibility failures detected in CI vs runtime: target > 90% caught
   pre-runtime.

## Risk Register

1. Dependency risk: telemetry/schema/test libraries increase integration surface.
   Mitigation: phased rollout + feature flags + strict CI.
2. Migration risk: residual Python paths hidden in scripts/docs.
   Mitigation: migration matrix + grep-based CI checks for deprecated routing.
3. Ownership risk: server/dashboard/plugin work fragmented across maintainers.
   Mitigation: explicit owner per workstream and quarterly gate reviews.
4. Credential-gated plugin risk (HK cloud providers) limits full E2E CI.
   Mitigation: split contract tests (public CI) and credential-gated private CI.

## Immediate 30-Day Start Plan

1. Week 1: land schema generation command + drift gate scaffolding.
2. Week 2: migrate dashboard job list/detail to `react-query`.
3. Week 3: add correlation IDs and base telemetry export wiring.
4. Week 4: finalize CLI/config compatibility audit (`batchalign3`,
   `~/.batchalign.ini`) and publish migration matrix draft.

## Completion Definition for This Plan

This 12-month plan is complete when all quarterly acceptance criteria are
either:

1. met with evidence in CI/docs/metrics, or
2. explicitly deferred with a written rationale and updated annual objective.
