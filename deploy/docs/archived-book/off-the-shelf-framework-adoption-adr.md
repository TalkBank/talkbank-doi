# ADR: Off-the-Shelf Framework Strategy for Server and Dashboard

**Date:** 2026-02-24
**Status:** Accepted
**Scope:** Rust control-plane server framework choices and Web dashboard framework/library choices.

> **Update (2026-03):** Most items implemented: `utoipa` (OpenAPI generation),
> `governor` (rate limiting), `opentelemetry` (tracing export),
> `@tanstack/react-query` (dashboard). `vitest` and `playwright` adoption
> is pending.

## Context

Batchalign currently has:

- a production Python server/runtime (`batchalign/serve/*`) with strong test coverage,
- a Rust control-plane workspace (`rust-next/`) built on `axum`/`tokio`,
- a React + TypeScript dashboard (`frontend/`) using generated OpenAPI types, `zustand`, and `wouter`.

We want to reduce custom infrastructure code where mature libraries can improve
reliability, maintainability, and typing guarantees.

## Decision

1. Keep `axum` + `tower-http` as the Rust server foundation.
2. Do not adopt Shuttle as a deployment framework/platform.
3. Adopt targeted Rust ecosystem libraries for missing concerns:
   - OpenAPI contract generation from Rust handlers/types (`utoipa` stack),
   - standardized telemetry export (`opentelemetry` + OTLP bridge for `tracing`),
   - request-rate protection (`governor`/tower-compatible limiter).
4. Keep the current React dashboard foundation, but adopt more off-the-shelf
   server-state and testing tooling:
   - `@tanstack/react-query` for fetch/cache/retry/invalidation,
   - `vitest` + React Testing Library for component/state behavior,
   - `playwright` for end-to-end dashboard regressions.

## Rationale

### Rust server

- `axum` + `tower-http` already matches the architecture (typed handlers,
  middleware, websocket support, composable routers).
- Replacing foundational server framework now adds migration risk without clear
  functional gain.
- The main reliability gaps are around observability, contracts, and traffic
  guards, which are better solved by additive libraries than by framework replacement.

### Shuttle

- Shuttle documents a shutdown timeline (operations ending on January 16, 2026),
  so it is not a viable strategic dependency for new platform commitments.

### Dashboard

- Current code uses manual `fetch` orchestration and websocket-driven updates.
- `react-query` reduces bespoke cache/poll/retry/invalidation logic and provides
  proven primitives for stale/fresh state and failure handling.
- Stronger test tooling is needed before increasing dashboard operational complexity.

## Consequences

### Positive

- Lower custom code in infrastructure-critical areas.
- Better typed contract governance between backend and UI.
- Faster incident diagnosis with standardized telemetry.
- Reduced regression risk in UI behavior.

### Negative / Costs

- Short-term dependency and integration overhead.
- Team learning curve for new library conventions.
- Need to define clear boundaries so state ownership is not split arbitrarily
  between `zustand` and `react-query`.

## Guardrails

- Keep custom code only for domain-specific orchestration or CHAT/linguistics logic.
- Prefer additive migration with feature flags over large rewrites.
- Keep API schema generation deterministic and CI-gated.
- Enforce strict typing for runtime-critical modules as adoption progresses.

## Revisit Triggers

Revisit this ADR if:

- Rust server throughput/reliability needs exceed what `axum` + selected
  middleware can provide,
- OpenAPI and typed client generation diverge repeatedly despite CI gates,
- Dashboard complexity outgrows current routing/state architecture.

## Source Evidence (accessed 2026-02-24)

- Shuttle docs notice: <https://docs.shuttle.dev/>
- Shuttle roadmap update: <https://www.shuttle.dev/roadmap/2026>
- Axum docs: <https://docs.rs/axum/latest/axum/>
- tower-http docs: <https://docs.rs/tower-http/latest/tower_http/>
- utoipa docs: <https://docs.rs/utoipa/latest/utoipa/>
- OpenTelemetry Rust docs: <https://docs.rs/opentelemetry/latest/opentelemetry/>
- TanStack Query docs: <https://tanstack.com/query/latest/docs/framework/react/overview>
- Vitest docs: <https://vitest.dev/guide/>
- Playwright docs: <https://playwright.dev/docs/intro>

## Implementation Link

Execution plan and checklists are archived at:
`docs/archive/book-src/developer/framework-adoption-backlog.md`
