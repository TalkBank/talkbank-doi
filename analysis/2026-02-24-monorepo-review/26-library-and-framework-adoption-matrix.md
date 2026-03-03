# 26. Library and Framework Adoption Matrix

## Goal

Use mature, well-supported libraries where they reduce custom complexity and reliability risk, while keeping core domain logic in-house.

## Recommended adoption matrix

### Python (`batchalign3`)

- `pydantic-settings` for typed config layering
- `tenacity` for explicit retry policies (if replacing ad hoc retries)
- `orjson` for high-volume JSON serialization paths
- `structlog` or strict JSON logging formatter for operational consistency
- `hypothesis` for property tests in parsing/transformation edge cases

### Rust (`talkbank-chat`, `talkbank-chatter`, `talkbank-clan`)

- `cargo-deny` for dependency policy and license compliance
- `cargo-audit` for vulnerability scanning
- `criterion` for performance benchmarks
- `tokio-util` cancellation primitives where async cancellation complexity grows

### Frontend/Extension

- `@tanstack/react-query` for robust async state management in dashboard
- React Testing Library + `vitest` for UI behavior reliability
- `playwright` for end-to-end dashboard regression tests

### Cross-ecosystem

- `pre-commit` for multi-language hooks
- `syft` for SBOM generation

## Adoption sequencing checklist

- [ ] Start with low-risk tooling additions (`cargo-deny`, `cargo-audit`, pre-commit)
- [ ] Add query/cache framework in dashboard (`react-query`)
- [ ] Add typed config framework in Python runtime-critical services
- [ ] Add benchmark framework baselines (`criterion`, Python perf harness)
- [ ] Review and retire custom code that overlaps mature library capabilities
