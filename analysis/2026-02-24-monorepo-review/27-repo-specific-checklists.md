# 27. Repo-Specific Checklists

## `talkbank-chat`

- [ ] Keep parser equivalence and roundtrip gates mandatory
- [ ] Add downstream compatibility CI jobs (`talkbank-chatter`, `batchalign3`)
- [ ] Automate docs freshness checks for corpus/test metric counts
- [ ] Reduce remaining production `unwrap` instances where practical

## `talkbank-chatter`

- [ ] Fix current `talkbank-lsp` compile errors
- [ ] Add CI workflow with `check`, `fmt`, `clippy`, `test`
- [ ] Add LSP protocol smoke tests
- [ ] Add extension integration tests with LSP binary startup/failure scenarios

## `talkbank-clan`

- [ ] Add CI workflow with full quality gates
- [ ] Create command parity matrix against legacy CLAN
- [ ] Add deterministic golden output corpus tests
- [ ] Benchmark high-cost commands and set regression budgets

## `tree-sitter-talkbank`

- [ ] Fix query node mismatch in `highlights.scm`
- [ ] Add strict query compatibility gate in CI
- [ ] Add change-impact reporting for downstream parser/LSP consumers
- [ ] Ensure manifest license metadata stays consistent across ecosystems

## `batchalign3`

- [ ] Split large dispatch/server modules into cohesive packages
- [ ] Tighten typing and reduce `Any`/broad catches in critical paths
- [ ] Add daemon/fleet/server integration stress tests
- [ ] Add standardized operational telemetry and SLO dashboard
- [ ] Resolve stale repository metadata references (`batchalign2` links)
