# 19. Parser and Grammar Quality

## Findings

- The parser/grammar/spec ecosystem is a major strength.
- Current detected issue: `tree-sitter-talkbank` query validation failure (`mor_category` node type), despite full parse test success.
- This indicates a gap between grammar node evolution and query maintenance gates.

## Recommendations

1. Add explicit query validation as required CI gate (separate from parse corpus pass).
2. Add compatibility matrix tests:
   - grammar node-types
   - Rust parser node dispatch
   - highlights/folds/tags queries
   - LSP semantic features
3. Add change-impact automation that flags affected downstream components when grammar changes.
4. Improve release checklists to include real-file roundtrip and query compatibility steps.

## Tools/frameworks to leverage

- Query validation scripts integrated into CI
- Node-type snapshot comparison tooling

## Parser quality checklist

- [ ] Fix current `mor_category` query mismatch in grammar repo
- [ ] Add query compatibility CI gate before merge
- [ ] Add grammar-change impact report in PR checks
- [ ] Add end-to-end parser+LSP smoke tests on representative corpora
- [ ] Version and document grammar node-type change policy
