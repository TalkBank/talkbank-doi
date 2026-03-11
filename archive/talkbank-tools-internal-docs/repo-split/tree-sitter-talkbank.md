# Repo: tree-sitter-talkbank

**GitHub**: `talkbank/tree-sitter-talkbank`
**Phase**: 1 (extract first)

## Files to Move

### From `grammar/` (becomes repo root)

```
grammar.js              -> grammar.js
src/                    -> src/
bindings/               -> bindings/
queries/                -> queries/
test/                   -> test/
Cargo.toml              -> Cargo.toml
Cargo.lock              -> Cargo.lock
package.json            -> package.json
package-lock.json       -> package-lock.json
pyproject.toml          -> pyproject.toml
setup.py                -> setup.py
tree-sitter.json        -> tree-sitter.json
binding.gyp             -> binding.gyp
CMakeLists.txt          -> CMakeLists.txt
Package.swift           -> Package.swift
Package.resolved        -> Package.resolved
go.mod                  -> go.mod
highlight.py            -> highlight.py
Makefile                -> Makefile
LICENSE                 -> LICENSE
README.md               -> README.md
CHANGELOG.md            -> CHANGELOG.md
CLAUDE.md               -> CLAUDE.md
.editorconfig           -> .editorconfig
.gitattributes          -> .gitattributes
.gitignore              -> .gitignore
scripts/                -> scripts/
```

### From `spec/` (becomes `spec/` in new repo)

```
spec/constructs/        -> spec/constructs/
spec/errors/            -> spec/errors/
spec/symbols/           -> spec/symbols/
spec/tools/             -> spec/tools/
spec/docs/              -> spec/docs/
spec/CLAUDE.md          -> spec/CLAUDE.md
spec/Cargo.toml         -> spec/Cargo.toml
spec/Cargo.lock         -> spec/Cargo.lock
```

### Do NOT Move

```
grammar/docs/           -> DELETE (session notes, not reference docs)
grammar/AGENTS.md       -> DELETE
grammar/CA_MODE_VALIDATION_TODO.md -> DELETE
grammar/CHAT_ERRORS_REPORT.md -> DELETE
grammar/CHAT-SPECIAL-CHARACTERS.md -> DELETE
grammar/CONTRIBUTING.md -> DELETE
grammar/GRAMMAR_AUDIT.md -> DELETE
grammar/GRAMMAR_INTERNALS.md -> DELETE
grammar/target/         -> .gitignore
grammar/.claude/        -> Keep if useful, otherwise delete
grammar/.kiro/          -> DELETE
grammar/.github/        -> REPLACE with new CI
```

## Dependency Changes

### `spec/tools/Cargo.toml`

Before (path deps to monorepo):
```toml
talkbank-errors = { path = "../rust/crates/talkbank-errors" }
talkbank-model = { path = "../rust/crates/talkbank-model" }
talkbank-tree-sitter-parser = { path = "../rust/crates/talkbank-tree-sitter-parser" }
```

After (crates.io deps -- requires Phase 2 first):
```toml
talkbank-errors = "0.1"
talkbank-model = "0.1"
talkbank-tree-sitter-parser = "0.1"
```

Bootstrap: For the initial extraction before Phase 2, use git deps:
```toml
talkbank-errors = { git = "https://github.com/talkbank/talkbank", tag = "v0.1.0" }
talkbank-model = { git = "https://github.com/talkbank/talkbank", tag = "v0.1.0" }
talkbank-tree-sitter-parser = { git = "https://github.com/talkbank/talkbank", tag = "v0.1.0" }
```

### `Cargo.toml` (grammar crate itself)

No changes needed -- the grammar crate has no Rust crate dependencies.

## Publishing

| Target | Package | Command |
|--------|---------|---------|
| npm | `tree-sitter-talkbank` | `npm publish` |
| PyPI | `tree-sitter-talkbank` | `python -m build && twine upload dist/*` |
| crates.io | `tree-sitter-talkbank` | `cargo publish` |

## CI Configuration

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  grammar:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: tree-sitter/setup-action@v2
      - run: tree-sitter generate
      - run: tree-sitter test

  spec-tools:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cd spec/tools && cargo test
      - run: cd spec/tools && cargo clippy -- -D warnings
```

## Verification After Extraction

1. `tree-sitter generate` succeeds
2. `tree-sitter test` passes
3. `cd spec/tools && cargo test` passes
4. `npm pack --dry-run` succeeds
5. `cargo publish --dry-run` succeeds
6. Grammar repo is standalone -- no references back to monorepo
