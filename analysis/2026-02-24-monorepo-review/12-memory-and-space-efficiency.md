# 12. Memory and Space Efficiency

## Findings

- `batchalign3` already models memory-aware worker limits and mode-specific budgets (free-threaded vs process-based).
- Large static datasets and duplicated crates increase artifact and memory footprint.
- Workspace contains large generated/test corpora and build outputs; hygiene controls should be explicit and automated.

## Recommendations

1. Formalize memory budgets per command and environment as testable policy.
2. Move large static Python data (for example huge in-module lists) into compressed data assets loaded lazily.
3. Add binary and wheel size budget checks in release pipelines.
4. Introduce duplicate-code/data detection for vendored crate copies.
5. Add memory-leak detection and long-run stability tests for daemon and server modes.

## Tools to leverage

- Memory profiling: `memray` (Python), `heaptrack`/`valgrind massif` (Rust targets as needed)
- Artifact size checks: custom CI step with thresholds

## Memory checklist

- [ ] Add explicit memory budget assertions for high-cost commands
- [ ] Refactor oversized static data modules to lazy-loaded resources
- [ ] Add release artifact size budget gate
- [ ] Add 6h/12h soak tests for server/daemon memory behavior
- [ ] Track memory usage trends per version in release notes
