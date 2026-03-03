# 14. Robustness and Error Handling

## Findings

- Error modeling in Rust core is strong (`talkbank-errors`, structured diagnostics).
- `batchalign3` still has many broad exception catch points across CLI/server/pipeline paths.
- Current active compile/query failures indicate need for stronger fail-fast gates earlier in workflow.

## Recommendations

1. Classify all recoverable vs non-recoverable errors per subsystem.
2. Replace broad catch blocks with targeted exception classes and contextual wrapping.
3. Standardize user-visible error taxonomy across CLI, HTTP, dashboard, and LSP.
4. Add panic and unwrap reduction plan in non-test Rust production code, especially in `batchalign-core` hotspots.
5. Add resilience tests for partial failures (network interruption, worker crash, cache corruption).

## Libraries/frameworks to leverage

- Python: typed error hierarchies + structured logging contexts
- Rust: `thiserror` + `miette` consistently at boundaries

## Robustness checklist

- [ ] Inventory all broad exception handlers and prioritize top 20 by runtime criticality
- [ ] Define and publish global error code taxonomy
- [ ] Add chaos-style tests for worker/process failure scenarios
- [ ] Reduce non-test `unwrap`/`expect` in critical modules
- [ ] Add incident runbook mapping error category to remediation step
