# 10. Dependency and Supply Chain

## Findings

- Multiple ecosystems are in play: Rust, Python, Node, tree-sitter bindings.
- High-value lockfiles exist (`Cargo.lock`, `uv.lock`, `package-lock.json`) but policy is not unified across repos.
- Metadata inconsistency exists:
  - `batchalign3` points to `batchalign2` repository/issues in `pyproject.toml`
  - `talkbank-chat` workspace repository field points to `TalkBank/talkbank`
  - `talkbank-chatter/vscode` repository/homepage points to `talkbank-utils`
  - `tree-sitter-talkbank` has license mismatch risk (`pyproject.toml` says MIT, repo/package say BSD-3-Clause)

## Recommendations

1. Add automated metadata consistency checks in CI.
2. Enforce SBOM generation per release artifact.
3. Add vulnerability scanning for all package ecosystems.
4. Reduce path-based dependency coupling where possible using published versions or pinned git refs.
5. Create a release manifest documenting exact dependency roots and transitive risk posture.

## Libraries/tools to leverage

- Rust: `cargo audit`, `cargo deny`
- Python: `pip-audit` (or `uv` compatible audit flow)
- Node: `npm audit --production` + lockfile policy
- SBOM: `syft` (or equivalent)

## Supply-chain checklist

- [ ] Fix repository/license metadata inconsistencies
- [ ] Add dependency vulnerability scans in each repo CI
- [ ] Add SBOM generation and artifact retention
- [ ] Define lockfile update cadence and review policy
- [ ] Add signed release workflow requirements
