# 07. Code Quality and Style

## Findings

- `talkbank-chat` shows strong engineering discipline (generated artifact checks, parser guardrails, documentation of invariants).
- `batchalign3` has quality intent but still has broad exception usage and high-complexity modules.
- Large files indicate maintainability hotspots (examples: `dispatch_server.py`, `job_store.py`, `batchalign-core/src/lib.rs`).

## Observed quality debt signals

- Broad `except Exception` patterns across runtime-critical Python modules.
- Metadata and naming inconsistencies (multiple stale repository URLs).
- Some compiled/build-time issues currently present in active repos (`talkbank-chatter`, `tree-sitter-talkbank` query mismatch).

## Recommendations

1. Set complexity and size budgets:
   - Python: split files > 800 LOC unless mostly static data
   - Rust: split files > 1,200 LOC unless generated
2. Add lint baselines:
   - Rust `clippy -D warnings` in all repos
   - Python `ruff` + stricter mypy on critical paths
   - TypeScript `eslint` in frontend/dashboard code
3. Replace broad catch blocks with typed exception handling and context-rich errors.
4. Add code ownership map for high-risk modules.

## Libraries/frameworks to leverage

- Python lint/format/type stack: `ruff`, `mypy`, optionally `pyright` dual validation
- Rust linting: `clippy`, `cargo-deny` for policy checks
- JS/TS: `eslint`, `typescript-eslint`, `vitest` quality plugins

## Quality checklist

- [ ] Introduce complexity budgets and report violations in CI
- [ ] Add strict lints to all repos (with staged adoption)
- [ ] Refactor top 5 largest runtime-critical files into submodules
- [ ] Replace broad catches in server/dispatch paths with typed handling
- [ ] Add metadata consistency lint (license, URLs, package names)
