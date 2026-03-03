# 05. Modularity and Boundaries

## Findings

- The architectural split is directionally good:
  - `talkbank-chat` core parser/model
  - `talkbank-chatter` user tools (CLI/LSP)
  - `talkbank-clan` analysis command reimplementation
  - `batchalign3` pipeline + serving
- The largest boundary risk is duplicated shared crates between `talkbank-chat` and `batchalign3/rust`.

## Concrete evidence

- Shared crate names exist in both repos (`talkbank-model`, `talkbank-parser-api`, `talkbank-tree-sitter-parser`, etc.).
- Diff counts show drift across duplicated crates (examples):
  - `talkbank-model`: high divergence
  - `talkbank-parser-tests`: high divergence
  - `talkbank-tree-sitter-parser`: significant divergence

## Recommendations

1. Pick one source-of-truth strategy for shared crates:
   - Option A: publish versioned crates and consume semver versions
   - Option B: git subtree with explicit sync tooling
   - Option C: workspace federation with locked commit references
2. Introduce compatibility tests in `batchalign3` against upstream `talkbank-chat` versions.
3. Reduce direct path coupling where possible; prefer explicit interfaces and versioning.
4. Add architecture decision records for each boundary policy.

## Modularity checklist

- [ ] Decide and document shared crate synchronization model
- [ ] Implement automated drift detection between duplicated crates
- [ ] Add compatibility test matrix for upstream parser/model versions
- [ ] Move cross-repo contracts into versioned API docs
- [ ] Define deprecation policy for cross-repo API changes
