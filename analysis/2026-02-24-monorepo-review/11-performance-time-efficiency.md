# 11. Performance and Time Efficiency

## Findings

- The workspace already includes many performance-oriented choices:
  - Generated parser tests and corpus regression gates
  - Batch processing and cache systems in `batchalign3`
  - Incremental parsing logic in LSP
- Remaining bottlenecks are concentrated in large dispatch/server modules and duplicated parsing stacks.

## Notable hotspots

- Python runtime-heavy modules:
  - `batchalign/cli/dispatch_server.py`
  - `batchalign/serve/job_store.py`
  - `batchalign/serve/app.py`
- Rust hotspots:
  - `batchalign-core/src/lib.rs`
  - parser-heavy direct parser modules

## Recommendations

1. Add cross-repo benchmark baselines and trend tracking:
   - Parser throughput
   - Validation throughput
   - Batch job throughput per worker mode
2. Split control-plane and data-plane logic in `batchalign3` dispatch/server code.
3. Add latency budgets for critical endpoints (`/health`, `/jobs`, `/jobs/{id}`).
4. Profile startup and first-request cold paths for server and LSP processes.
5. Move expensive repeated conversions into memoized or precomputed structures where safe.

## Tools to leverage

- Rust benchmarking: `criterion`
- Python profiling: `py-spy`, `scalene`
- API benchmarking: `k6` or `vegeta`

## Performance checklist

- [ ] Establish baseline benchmark suite with CI artifact publication
- [ ] Add p50/p95/p99 latency tracking for server endpoints
- [ ] Break up top Python hotspot files into testable submodules
- [ ] Add performance regression gate for parser/validation throughput
- [ ] Add cold-start benchmark for daemon/server/LSP startup
