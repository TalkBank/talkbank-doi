# 21. LSP, VS Code, and Editor Integration

## Findings

- `talkbank-lsp` architecture is modular and feature-rich.
- Current compile failures in `talkbank-lsp` block confidence and should be treated as immediate stop-ship issues.
- VS Code extension has extensive command surface and test scripts, but integration CI with LSP binary health should be explicit.

## Immediate issues to fix

- `crates/talkbank-lsp/src/alignment/formatters/content.rs:29`
- `crates/talkbank-lsp/src/graph/labels.rs:42`
- Type mismatch (`SmolStr` vs `String`) from upstream model changes

## Recommendations

1. Add compatibility tests against current `talkbank-model` types for all formatter paths.
2. Add LSP protocol smoke tests in CI (initialize, open doc, diagnostics, hover, semantic tokens).
3. Add extension-to-LSP integration tests (binary startup, command wiring, graceful failures).
4. Reduce conversion boilerplate with explicit newtypes/traits for display labels.
5. Add editor-performance budget for large files.

## Tools/frameworks to leverage

- LSP integration harness via `tower-lsp` test patterns
- VS Code extension integration tests with `@vscode/test-electron`

## LSP/extension checklist

- [ ] Fix current compile break in `talkbank-lsp`
- [ ] Add LSP protocol smoke tests in CI
- [ ] Add extension integration tests for LSP start/fallback paths
- [ ] Add performance benchmarks for large transcript edits
- [ ] Add model-change compatibility checklist for LSP maintainers
