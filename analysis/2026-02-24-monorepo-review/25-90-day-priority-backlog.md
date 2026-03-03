# 25. 90-Day Priority Backlog

## Days 0-15: Stabilize

- [ ] Fix `talkbank-lsp` compile failures (`SmolStr` to `String` conversions)
- [ ] Fix `tree-sitter-talkbank` query node mismatch (`mor_category`)
- [ ] Add CI workflows for `talkbank-chatter` and `talkbank-clan`
- [ ] Correct package metadata inconsistencies (repo URLs, license fields)

## Days 16-30: Standardize quality gates

- [ ] Introduce uniform `check/test/verify` commands for every repo
- [ ] Add pre-commit lint/type/format hooks
- [ ] Add workspace-level integration check command
- [ ] Add baseline metrics collection and publication

## Days 31-60: Improve architecture and typing

- [ ] Refactor `batchalign3` dispatch/server hotspots into smaller modules
- [ ] Enforce strict typing in critical Python modules
- [ ] Introduce typed API payload models and contract checks
- [ ] Add cross-repo compatibility tests for grammar/parser/model/LSP

## Days 61-90: Performance and operations maturity

- [ ] Add benchmark trend CI for parser and batch processing performance
- [ ] Add memory/soak tests for daemon and server modes
- [ ] Add structured telemetry schema and SLO dashboard
- [ ] Add release governance: SBOM, dependency audits, signed artifacts

## Outcome targets by day 90

- [ ] All repos have passing CI with minimum quality parity
- [ ] No active compile/query breakages in default branches
- [ ] Typed contract coverage for critical runtime surfaces
- [ ] Observable, benchmarked, and regression-guarded production pathways
- [ ] Documented and automated shared-crate synchronization policy
