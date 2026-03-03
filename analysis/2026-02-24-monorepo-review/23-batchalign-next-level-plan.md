# 23. Batchalign Next-Level Plan (`batchalign3`)

## Strategic direction

Focus on making `batchalign3` a highly reliable typed orchestration platform with predictable performance and safer distributed execution.

## Current strengths

- Strong domain functionality and breadth.
- Hybrid concurrency model and daemon/server capabilities.
- Rust core integration for parser correctness and performance.
- Typed frontend with generated API types.

## Priority gaps

- High complexity in dispatch/server modules.
- Broad exception usage in runtime-critical paths.
- Typing strictness not yet at desired level for critical modules.
- Some metadata and release-link drift.

## Recommended workstreams

1. Runtime reliability hardening
   - Narrow exception handling and enrich error taxonomy
   - Add idempotency/retry safety for remote dispatch
2. Typing and contracts
   - Strict typing tiers for CLI/serve modules
   - Replace dynamic payload dicts with typed models
3. Module decomposition
   - Split `dispatch_server.py` and `job_store.py` into cohesive subpackages
4. Observability
   - Structured telemetry across job lifecycle
5. Performance
   - Baseline and guard p95 latency/throughput

## Batchalign checklist

- [ ] Refactor top 3 complexity hotspots into composable submodules
- [ ] Enforce strict typing in `batchalign/serve/*` and dispatch paths
- [ ] Implement unified error model and user-facing remediation hints
- [ ] Add robust integration tests for daemon/fleet/server modes
- [ ] Add operational dashboard metrics and alert thresholds
