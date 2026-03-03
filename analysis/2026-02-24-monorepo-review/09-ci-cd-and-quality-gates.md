# 09. CI/CD and Quality Gates

## Findings

- CI maturity is inconsistent:
  - Strong in `talkbank-chat`
  - Basic in `batchalign3`
  - Present but currently not preventing query-level regressions in `tree-sitter-talkbank`
  - Missing in `talkbank-chatter` and `talkbank-clan`

## Concrete current issues

- Current compile failure in `talkbank-chatter` would be caught by basic CI.
- Query compatibility issue in `tree-sitter-talkbank` indicates test matrix should include query validation as an explicit gate.

## Recommendations

1. Define mandatory gates for every repo:
   - Format check
   - Compile/type check
   - Unit/integration tests
   - Generated artifact consistency
2. Add repository-specific deploy/release hardening:
   - Signed tags/releases
   - Reproducible build metadata
3. Add root-level meta CI that checks cross-repo compatibility at workspace level.
4. Publish quality gate status badge dashboard.

## CI checklist

- [ ] Add `.github/workflows/ci.yml` to `talkbank-chatter`
- [ ] Add `.github/workflows/ci.yml` to `talkbank-clan`
- [ ] Expand `tree-sitter-talkbank` gate to fail fast on query mismatches
- [ ] Strengthen `batchalign3` CI with Rust checks + typing strictness tiers
- [ ] Add workspace integration workflow (`verify-all`)
