# Framework Adoption Backlog (Server + Dashboard)

**Date:** 2026-02-24  
**Status:** Proposed execution backlog for ADR follow-through.

## Objective

Implement off-the-shelf libraries that reduce custom platform code while
preserving correctness and typed contracts.

## Principles

- No large rewrites; use incremental migrations.
- Keep orchestration behavior stable and test-gated.
- Prefer typed interfaces (`serde`/TypeScript types) over ad hoc maps.
- Land each phase with measurable acceptance criteria.

## Track A: Rust Server (control plane)

### Phase A1: Contract generation and drift gates

- [x] Add `utoipa` schema derivations for core route request/response types in `rust-next`.
- [x] Add a `rust-next` command to emit OpenAPI JSON deterministically.
- [x] Compare emitted schema against checked-in schema in CI.
- [x] Document regeneration workflow and ownership.

Acceptance criteria:

- `rust-next` can generate OpenAPI from source code.
- CI fails on schema drift.
- Rust and frontend generated types are built from one canonical schema artifact.

### Phase A2: Telemetry standardization

- [x] Integrate `tracing` -> `opentelemetry` OTLP export path.
- [x] Define metric names and labels for job lifecycle and worker health.
- [x] Add correlation IDs to job submission/polling/log events.
- [x] Add sample dashboard panels and alert thresholds for SLOs.

Acceptance criteria:

- One trace spans request -> job enqueue -> worker completion.
- Health counters are queryable in the chosen telemetry backend.
- Runbooks include metric names and alert thresholds (`developer/telemetry-runbook.md`).

### Phase A3: Traffic protection and resilience middleware

- [x] Introduce request-rate limiting for sensitive endpoints (`/jobs`, result fetch routes).
- [x] Add standard timeout middleware and explicit response mapping for timeout cases.
- [ ] Add backpressure behavior tests for overload scenarios (beyond deterministic 429 burst tests).

Acceptance criteria:

- Load tests show bounded latency under burst traffic.
- Rate-limit and timeout responses are deterministic and documented.

### Phase A4: Persistence strategy spike (`rusqlite` vs `sqlx`)

- [x] Write a short technical spike comparing current `rusqlite` approach vs `sqlx` migration cost.
- [x] Record recommendation in `decisions/rusqlite-vs-sqlx-spike-2026-02.md`.
- [ ] Prototype one representative query path in `sqlx` (optional branch).
- [x] Decide go/no-go with maintainability and runtime criteria.

Acceptance criteria:

- Written decision document with measured tradeoffs and explicit recommendation.

## Track B: Web Dashboard

### Phase B1: Adopt `@tanstack/react-query` for server state

- [x] Introduce `QueryClientProvider` in `frontend/src/main.tsx`.
- [x] Move `fetchJobs` and `fetchJob` flows to query hooks with typed results.
- [x] Keep websocket updates, but wire them to cache invalidation or cache patching.
- [x] Define stale times and retry policy by endpoint criticality.

Acceptance criteria:

- No direct page-level `fetch` orchestration for jobs/job details.
- Reconnect and retry behavior is consistent and test-covered.

### Phase B2: Error and offline UX hardening

- [ ] Add `react-error-boundary` around page roots and key panels.
- [ ] Add explicit offline/reconnecting states for websocket and REST failures.
- [ ] Add user-facing remediation text for common failure classes.

Acceptance criteria:

- Dashboard degrades gracefully during server disconnects.
- Error boundaries capture and display recoverable failure states.

### Phase B3: Testing baseline

- [ ] Add `vitest` + React Testing Library setup.
- [ ] Add component tests for: job list, job detail, action buttons, filter flows.
- [ ] Add `playwright` smoke scenarios for multi-server dashboard behavior.

Acceptance criteria:

- CI includes dashboard unit/component and e2e smoke gates.
- Critical operator workflows are regression protected.

## Track C: Dioxus Dashboard Migration (Rust-First UI)

### Phase C1: Scaffold and serving compatibility

- [x] Add `batchalign-dashboard-dioxus` scaffold crate in `rust-next`.
- [x] Keep existing static dashboard serving contract (`BATCHALIGN_DASHBOARD_DIR` / `~/.batchalign3/dashboard`).
- [x] Document Dioxus build and deployment workflow.

Acceptance criteria:

- Dioxus dashboard can be built independently and served by current server routes.
- No backend API contract changes are required for initial adoption.

### Phase C2: Feature parity migration

- [x] Port job detail/actions UI (cancel/restart/delete).
- [x] Port websocket update handling and reconnect behavior.
- [x] Add parity checklist against production operator flows.

Acceptance criteria:

- Dioxus dashboard covers all critical operator flows used in production.
- Reconnect and failure UX parity is demonstrated in staging.

### Phase C3: Fleet-aware operator ergonomics

- [x] Add multi-server discovery (`?servers=` and `GET /fleet`).
- [x] Add server filter tabs and per-server websocket status indicators.
- [x] Add browser smoke test harness for multi-server list/detail/action flows.
- [x] Wire smoke tests into CI execution.

Acceptance criteria:

- Operators can inspect and act on jobs from multiple servers in one dashboard.
- Server-scoped actions are verified against the correct backend target.
- Fleet behavior has automated smoke coverage before cutover.

## Suggested Sequence (first three sprints)

Sprint 1:

- A1 (OpenAPI generation/drift gate)
- B1 (react-query for job list/detail)

Sprint 2:

- A2 (telemetry bridge)
- B2 (error/offline UX)

Sprint 3:

- A3 (rate limiting/timeout middleware)
- B3 (test baseline)
- A4 (persistence spike write-up)

## Ownership Suggestion

- Server platform owner: A1-A4
- Dashboard owner: B1-B3
- CI/tooling owner: schema drift gates, test workflow expansion

## Exit Criteria for This Backlog

- Rust server and dashboard both rely on selected off-the-shelf libraries for
  contracts, state handling, and testing.
- CI enforces schema freshness and dashboard regression coverage.
- Operational telemetry is standardized enough for SLO-based alerting.
