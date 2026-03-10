# Doctest Assessment

## Current State

73 doctests across the workspace: 54 pass, 1 fails (missing import), 18 ignored/skipped.

| Crate | Pass | Fail | Ignored | Notes |
|-------|------|------|---------|-------|
| talkbank-model | 30 | 0 | 12 | Participant construction, CA elements, validation trait |
| talkbank-errors | 11 | 0 | 0 | ValidationConfig methods, severity handling |
| send2clan-sys | 10 | 0 | 0 | 5 are `no_run` (need CLAN installed) |
| talkbank-clan | 3 | 0 | 1 | |
| talkbank-derive | 0 | 0 | 4 | Macro expansion examples, can't run in doctest context |
| talkbank-parser-api | 0 | 1 | 0 | Missing `use ParseOutcome` import in example |
| talkbank-json | 0 | 0 | 1 | |
| 6 other crates | 0 | 0 | 0 | No doctests at all |

### Crates with zero doctests

talkbank-tree-sitter-parser, talkbank-direct-parser, talkbank-lsp, talkbank-highlight, batchalign-core, doc-tools.

## Assessment

### What doctests are good for

- **API documentation that stays correct**: The compiler verifies examples compile. This is valuable for public APIs that external consumers use.
- **Simple constructor/accessor demos**: The talkbank-model and talkbank-errors doctests show how to create objects and call methods. These are genuinely useful as living documentation.

### What doctests are bad for

- **They're slow**: Each doctest compiles as a separate binary. With 73 doctests, `cargo test --doc` takes significant time and is the main thing `cargo nextest` cannot parallelize (nextest doesn't support doctests at all).
- **They test the wrong things**: Our real correctness comes from the reference corpus (339 files), parser equivalence tests, and roundtrip tests. Doctests test trivial constructor calls.
- **Maintenance burden**: The 1 failing doctest (talkbank-parser-api) and 18 ignored doctests show that doctests rot when APIs evolve. Ignored doctests are dead weight.
- **Internal crates don't benefit**: Most of our crates are internal (no external consumers). The "documentation stays correct" benefit only matters for public APIs.

### Who are our consumers?

- **talkbank-model**: Used by both parsers, transform, CLI, LSP, and potentially external Python bindings. Doctests have some value here.
- **talkbank-errors**: Used by everything. Doctests have some value.
- **talkbank-parser-api**: Trait consumed by both parsers. The 1 failing doctest should be fixed or removed.
- **Everything else**: Internal implementation crates. Doctests provide minimal value.

## Recommendation

**Keep doctests in talkbank-model and talkbank-errors** where they serve as living API documentation for cross-crate consumers. Fix or remove broken/ignored ones.

**Don't add doctests to internal crates**. Use `#[test]` functions instead — they're faster to compile, easier to maintain, and nextest can parallelize them.

**Fix the 1 failing doctest** in talkbank-parser-api (add missing import) or convert it to a `#[test]`.

**Clean up ignored doctests**: The 18 ignored doctests should either be converted to `#[test]` functions (if they test something useful) or deleted (if they're just aspirational examples that never ran).

**For CI/dev speed**: Use `cargo nextest run` for unit/integration tests (fast, parallel). Run `cargo test --doc` separately and less frequently (or only in CI), since doctests are inherently sequential and slow.

## Impact on nextest adoption

`cargo nextest run` does not run doctests — it only runs `#[test]` functions and integration tests. This is fine because:
1. Our doctests are mostly trivial usage examples, not critical regression tests.
2. Critical correctness is covered by `#[test]` functions and the reference corpus.
3. We can run `cargo test --doc` as a separate CI step if desired.

---
Last Updated: 2026-02-20
