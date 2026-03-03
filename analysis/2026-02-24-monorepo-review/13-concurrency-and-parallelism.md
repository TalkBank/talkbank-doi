# 13. Concurrency and Parallelism

## Findings

- `batchalign3` has the most advanced concurrency system in the workspace, including process/thread mode switching and daemon/server orchestration.
- LSP uses async and incremental logic, but current compile break blocks confidence.
- Cross-repo concurrency policy is not standardized; approaches vary by repo.

## Recommendations

1. Document one concurrency model per executable system:
   - Unit of parallelism
   - Shared-state policy
   - Cancellation semantics
   - Backpressure strategy
2. Add cancellation and timeout contract tests for:
   - HTTP jobs
   - local daemon jobs
   - fleet-dispatched jobs
3. Standardize queue and worker instrumentation (queued/running/failed/retry states).
4. Add structured retry policies with idempotency guarantees for remote dispatch paths.
5. Use typed state machines for lifecycle transitions to avoid invalid states.

## Libraries/frameworks to leverage

- Python task/state modeling: `transitions` (if needed) or explicit typed enums/dataclasses
- Rust async robustness: `tokio-util` cancellation tokens, bounded channels where appropriate

## Concurrency checklist

- [ ] Create concurrency design docs for batchalign server/daemon/fleet
- [ ] Add deterministic cancellation tests and race condition tests
- [ ] Add idempotency keys and safe retry semantics for remote job submission
- [ ] Add state-transition invariants and property tests
- [ ] Add queue depth and worker saturation telemetry
