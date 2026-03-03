# 18. Data and Generated Artifacts

## Findings

- Generated artifacts are central to parser/model correctness.
- `talkbank-chat` has good generated artifact policies and scripts.
- Drift risk remains high when generated files, grammar, tests, and downstream parsers are not validated together.
- Metrics output indicates corpus/reference counts may have changed (for example 343 files) while some docs still mention older counts.

## Recommendations

1. Treat generated artifacts as first-class build products with deterministic regeneration.
2. Add drift-check jobs across grammar/parser/spec/test outputs.
3. Add "doc-stat freshness" checks for key numeric claims (corpus size, test counts).
4. For large static data assets, move to versioned data packages with integrity checks.
5. Add explicit generated-file ownership and review rules.

## Tools to leverage

- Deterministic codegen checks (`git diff --exit-code` after generation)
- Metadata check scripts for docs consistency

## Artifacts checklist

- [ ] Add one command to regenerate all artifacts across repos
- [ ] Add CI gate for stale generated outputs in every affected repo
- [ ] Add documentation freshness checks for key published numbers
- [ ] Add checksums/signatures for major generated datasets
- [ ] Add contributor docs explaining regeneration flow end-to-end
