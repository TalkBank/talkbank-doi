# 08. Testing Strategy and Coverage

## Findings

- `talkbank-chat` has high test rigor and domain-driven generated tests.
- Snapshot-driven testing is widely used in parser/model repos.
- `talkbank-chatter` and `talkbank-clan` have tests but less visible gate enforcement due missing CI workflows.
- `batchalign3` test suite exists and mypy passes, but stronger runtime regression gates are needed for distributed/daemon/server behavior.

## Recommendations

1. Define testing pyramid per repo:
   - Unit, property, integration, corpus/regression, performance
2. Add contract tests between repos:
   - grammar -> parser -> model -> LSP -> extension query compatibility
3. Add concurrency and failure-mode tests for `batchalign3` job execution and daemon fallback logic.
4. Add reproducible performance benchmark suites in CI (nightly if needed).
5. Track flake rates and quarantine policy.

## Libraries/frameworks to leverage

- Rust: `proptest`, `rstest`, `criterion` for benchmarking
- Python: `pytest-xdist`, `pytest-timeout`, `hypothesis` for property testing
- TS/VSCode extension: `vitest` + integration harness

## Testing checklist

- [ ] Add CI-required test matrix for `talkbank-chatter`
- [ ] Add CI-required tests for `talkbank-clan`
- [ ] Add cross-repo compatibility test suite
- [ ] Add benchmark CI job with trend artifacts
- [ ] Add flaky-test tracking and enforcement policy
