# Ten-Year Platform Roadmap (2026-2036)

## Date

February 24, 2026

## Status

Proposed

## Purpose

Define a long-term architecture roadmap for `batchalign3` that prioritizes
correctness, durability, and principled evolution over short-term shipping
constraints.

This roadmap operationalizes the
[Long-Term Architecture Charter](long-term-architecture-charter.md).

## Scope

In scope:

1. server/control plane
2. web dashboard
3. CLI
4. typed contracts and plugin boundary governance

Out of scope:

- CHAT internal linguistic model design

In-scope CHAT concern:

- interface boundaries between `batchalign3` and CHAT processing components

## Target End-State (2036)

1. `batchalign3` is a stable host platform with versioned extension contracts.
2. server behavior is observable, testable, and multi-tenant ready.
3. dashboard is an operational control plane with strong reliability UX.
4. CLI is a durable automation API with explicit compatibility policy.
5. provider/domain integrations (for example HK cloud engines) are external,
   independently versioned plugins.

## Architecture Workstreams

## A. Server / Control Plane

### A1. Contract-first platform

- single canonical API schema generated from server source
- strict schema drift gates in CI
- versioned API compatibility policy

### A2. Reliability and operability

- standardized traces/metrics/log correlation IDs
- documented SLOs and error-budget policy
- deterministic overload handling (rate limiting, timeout, backpressure)

### A3. Scheduling and execution model

- explicit queueing/scheduling abstraction with pluggable policies
- workload-class isolation (CPU/GPU/IO-heavy classes)
- policy-driven worker placement and admission control

### A4. Persistence and recovery

- explicit storage strategy with migration lifecycle
- crash-safe job state transitions with replay semantics
- auditability for long-lived deployments
- **2026-02-26:** preparing to decouple persistence from SQLite-specific
  assumptions; likely direction is `sqlx` with Postgres for shared fleet state
  and SQLite retained for dev/test (see
  `book/src/decisions/rusqlite-vs-sqlx-spike-2026-02.md`)

## B. Dashboard

### B1. State architecture

- formal split of server-state vs local UI-state ownership
- typed API clients generated from canonical schema
- websocket + query cache coherence policy

### B2. Operator UX

- degraded/offline modes are first-class
- actionable, typed error surfaces (not generic failure toasts)
- fleet and multi-server operational workflows supported

### B3. Quality gates

- component + E2E baseline for critical operator flows
- visual/interaction regressions gated in CI for high-risk views

## C. CLI

### C1. Stable automation contract

- explicit deprecation policy and migration windows
- compatibility aliases with sunset tracking
- machine-readable output modes for automation workflows

### C2. Discoverability and safety

- strict, typed command option semantics
- high-risk operations guarded by explicit confirmation/policy flags
- clear error classes mapped to deterministic exit codes

### C3. Extensibility

- plugin engine selection without core hacks
- stable CLI-to-server command semantics across versions

## D. Typed Boundary and Plugin Governance

### D1. Compatibility contracts

- semantic versioning for host/plugin interfaces
- compatibility matrix and CI checks for supported plugin ranges
- explicit lifecycle states (experimental, supported, deprecated)

### D2. Ecosystem model

- core remains minimal host platform
- domain/provider plugins live externally with independent release cadence
- no re-forking as primary extension strategy

## Timeline and Milestones

## Phase 0 (2026): Contract and boundary hardening

1. lock host/plugin boundary (v1 contract)
2. complete schema generation + CI drift gates
3. establish telemetry baseline and correlation IDs
4. document CLI compatibility/deprecation policy

Exit criteria:

- API/contract drift is CI-blocking
- plugin engine selection works via explicit CLI flags
- operational telemetry has minimum dashboard/runbook coverage

## Phase 1 (2027-2028): Reliability platform

1. implement deterministic overload controls
2. formalize scheduler abstraction and workload classes
3. productionize dashboard offline/degraded modes
4. complete typed error/exit-code mapping across CLI and server

Exit criteria:

- overload behavior is test-verified and documented
- dashboard handles disconnect/retry without operator ambiguity

## Phase 2 (2029-2031): Multi-node and governance maturity

1. evolve fleet orchestration model with policy-driven routing
2. add stronger audit/recovery semantics for long-running deployments
3. enforce plugin compatibility matrix in CI
4. add long-horizon migration tooling for contract changes

Exit criteria:

- multi-node operation is operationally predictable
- plugin compatibility is machine-checked, not ad hoc

## Phase 3 (2032-2036): Platform longevity

1. periodic architecture simplification to reduce accumulated complexity
2. succession-friendly documentation and maintainership playbooks
3. scheduled replacement cycles for high-risk dependencies
4. formal architecture review every release train

Exit criteria:

- no single subsystem is knowledge-siloed
- major upgrades are incremental rather than emergency rewrites

## Decision Gates (Annual)

Each year, evaluate roadmap progress with these gates:

1. Contract gate: any breaking boundary change must include migration path + compatibility window.
2. Reliability gate: SLO and failure mode evidence must exist before expanding feature surface.
3. Complexity gate: remove/replace accidental complexity before adding new control-plane layers.
4. Ecosystem gate: plugin boundaries remain external and versioned.

## Metrics

Track at minimum:

1. schema drift incidents and time-to-resolution
2. rollback-worthy production incidents by subsystem
3. CLI compatibility breakages per release
4. dashboard operator error/recovery success metrics
5. plugin compatibility failures caught in CI vs runtime

## Radical Change Policy

Radical architectural changes are valid when they improve long-term
correctness and maintainability and meet all criteria:

1. explicit problem statement and alternatives analysis
2. compatibility and migration plan
3. testability and observability plan
4. rollback strategy

## First-Year Execution Mapping (2026)

This roadmap maps immediately to existing planning docs:

1. [Framework Adoption Backlog](../developer/framework-adoption-backlog.md)
2. [Off-the-Shelf Framework ADR](off-the-shelf-framework-adoption-adr.md)
3. [HK Pluginization Assessment](hk-pluginization-assessment.md)
4. [One-Year Execution Plan (2026-2027)](one-year-execution-plan-2026-2027.md)
