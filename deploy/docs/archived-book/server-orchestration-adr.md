# ADR: Server Orchestration and Migration Strategy

**Date:** 2026-02-19
**Status:** Accepted
**Scope:** Batchalign server/daemon orchestration, worker model, cache integration, and Rust migration path.

> **Update (2026-03):** All stages complete. CHAT divorce phases 0–6 are
> done — the Rust server owns CHAT lifecycle for morphosyntax, utseg,
> translate, coref, and forced alignment. Workers are stateless inference
> endpoints.

## Context

The system started as local sequential commands and is now a multi-layer execution runtime:

- local CLI execution (sequential/threads/processes),
- local daemon mode for warm pipelines,
- remote server mode (FastAPI + `JobStore` + SQLite recovery),
- optional fleet fan-out across multiple servers.

This is no longer a thin wrapper around pipeline calls. It is a custom orchestration platform.

Constraints that materially affect architecture choices:

- Key ML components (especially Stanza-dependent paths) remain Python-bound.
- Model load/warm reuse is required for acceptable throughput.
- Worker crashes and memory pressure must be handled without corrupting job state.
- Free-threaded Python 3.14 can improve some concurrency behavior, but does not remove
  process-level fault isolation needs for native ML stacks.

## Decision

Do not replace orchestration with a framework immediately.  
Do not attempt a full Rust rewrite immediately.

Instead:

1. Keep the current custom orchestrator and harden correctness invariants.
2. Consolidate and simplify architecture boundaries (job lifecycle, retry policy, cache story).
3. Migrate to a Rust control plane in stages, with Python ML workers as sidecars behind a stable IPC contract.

## Rationale

Framework migration now adds infrastructure complexity but does not
remove the hardest part: model lifecycle and Python ML dependency boundaries.

A full Rust rewrite now is blocked by Python-only model dependencies; forcing that path
immediately would stall delivery and increase risk.

The highest-risk correctness issue in current orchestration was job finalization on early-abort
paths. That has been fixed in `batchalign/serve/job_store.py` with regression tests in
`batchalign/tests/serve/test_job_store.py`.

## Staged Plan

### Stage 0 (completed on 2026-02-19): correctness invariant baseline

- Enforce: non-cancelled jobs cannot finish until all files are terminal (`done`/`error`).
- Convert unfinished files to system errors during finalization.
- Add regressions for thread-path and process-path partial completion.

Exit criteria:

- `test_job_store.py` passes with explicit invariant tests.

### Stage 1 (completed on 2026-02-19): reliability consistency baseline

- Local and server process-pool paths now share a no-auto-retry crash policy.
- Local crash handling now marks unfinished files as terminal failures.
- Local run-log `file_error` events now carry aligned error categories.
- Added operational counters (worker crashes, forced terminal errors, memory-gate aborts) exposed via `/health`.

Exit criteria:

- One documented crash policy used by both local and server routes.
- No path can report success/completion with unfinished files.

### Stage 2 (target: 2026-03-05 to 2026-03-26): architecture simplification

- Extract shared executor policy used by local and server code paths.
- Redis/hybrid cache wiring completed:
  - `redis_url` now activates `HybridCacheManager` through default cache wiring,
  - process workers configure the same backend on init,
  - `/health` now exposes backend + Redis connectivity status.
- Mark historical design docs clearly when superseded.

Exit criteria:

- Single source of truth for executor routing.
- No "paper architecture" features exposed but inactive.

### Stage 3 (target: 2026-03-26 to 2026-05-01): Rust control-plane pilot

- Implement Rust service for:
  - job ingestion/state machine/progress stream,
  - queueing and worker lease management,
  - persistence APIs.
- Keep Python sidecar workers for ML execution.
- Define versioned IPC contract (JSON/msgpack or gRPC) with strict schemas.

Exit criteria:

- One production command path runs through Rust control plane and Python sidecar.
- Feature parity maintained for cancellation, status, and per-file errors.

### Stage 4 (target: after 2026-05-01): incremental cutover

- Move additional commands to Rust control plane.
- Keep Python ML execution boundary until dependencies are replaced or wrapped.
- Reassess framework adoption only if operational requirements outgrow current control plane.

Exit criteria:

- Majority of orchestration logic is Rust-side.
- Python scope is primarily ML inference/adapters.

## Non-Goals (for this ADR period)

- Full Rust replacement of Stanza-dependent processing.
- Immediate adoption of a framework-centric architecture.
- New concurrency features before lifecycle correctness and consistency are stable.

## Revisit Triggers

Revisit this ADR if any of the following occur:

- Frequent multi-host scheduling failures where framework-level guarantees become mandatory.
- Operational burden from custom orchestration exceeds team maintenance capacity.
- ML dependency constraints materially change (for example, Rust-native replacement for current Python-bound engines).

## 2026-02-26 Note: Persistence and Fleet Coordination

The CLI-as-coordinator model (static file distribution, per-server SQLite, no
shared job queue) is insufficient for confident fleet deployment. Users face
private cache duplication, manual load balancing, and no cross-server visibility.

The emerging direction is to move fleet job state to **shared Postgres on Net**
(via `sqlx`), enabling a shared job queue with `SKIP LOCKED` work-pulling,
crash-safe re-queuing, and fleet-wide observability. This does not change the
staged plan above — Stages 0-3 proceed as written — but it informs the
persistence strategy underlying Stage 3 (Rust control-plane pilot) and the
framework reassessment in Stage 4.

See private report archive:
`talkbank-private/docs/batchalign3-reports/decisions/rusqlite-vs-sqlx-spike-2026-02.md`
for the full evaluation of database options.
