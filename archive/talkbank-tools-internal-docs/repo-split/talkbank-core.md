# Repo: talkbank (Core Rust Library)

**GitHub**: `talkbank/talkbank`
**Phase**: 2 (publish to crates.io) + Phase 6 (rename monorepo)

## What Stays (After All Extractions)

```
rust/crates/
  talkbank-errors/
  talkbank-derive/
  talkbank-model/
  talkbank-parser-api/
  talkbank-tree-sitter-parser/
  talkbank-direct-parser/
  talkbank-parser-tests/
  talkbank-json/
  talkbank-pipeline/
  talkbank-transform/
rust/corpus/
  reference/                  # 339 sacred reference files
rust/tests/                   # Integration tests
Cargo.toml                    # Workspace root
```

## What Gets Removed (After Other Phases)

| Current | Destination | Phase |
|---------|-------------|-------|
| `grammar/` | `talkbank/tree-sitter-talkbank` | 1 |
| `spec/` | `talkbank/tree-sitter-talkbank` | 1 |
| `rust/crates/batchalign-core/` | `batchalign` repo | 3 |
| `rust/crates/talkbank-clan/` | `talkbank/clan` | 4 |
| `rust/crates/talkbank-cli/` | `talkbank/chatter` | 5 |
| `rust/crates/talkbank-lsp/` | `talkbank/chatter` | 5 |
| `rust/crates/talkbank-highlight/` | `talkbank/chatter` | 5 |
| `rust/crates/send2clan-sys/` | `talkbank/chatter` | 5 |
| `rust/crates/talkbank-revai/` | `talkbank/chatter` | 5 |
| `rust/crates/doc-tools/` | DELETE | any |
| `rust/vscode/` | `talkbank/chatter` | 5 |

## Dependency Changes

### `tree-sitter-talkbank` (grammar crate)

Before (path dep):
```toml
tree-sitter-talkbank = { path = "grammar" }
```

After (crates.io dep):
```toml
tree-sitter-talkbank = "0.1"
```

## Publishing Order (crates.io)

Must publish in dependency order:

```
1. talkbank-derive        (no talkbank deps)
2. talkbank-errors         (depends on: talkbank-derive)
3. talkbank-model          (depends on: talkbank-errors, talkbank-derive)
4. talkbank-parser-api     (depends on: talkbank-errors, talkbank-model)
5. talkbank-json           (depends on: none in workspace)
6. talkbank-pipeline       (depends on: talkbank-errors, talkbank-model)
7. talkbank-direct-parser  (depends on: talkbank-errors, talkbank-model, talkbank-parser-api)
8. tree-sitter-talkbank    (from grammar repo, already published in Phase 1)
9. talkbank-tree-sitter-parser (depends on: tree-sitter-talkbank, talkbank-errors, talkbank-model, talkbank-parser-api, talkbank-direct-parser)
10. talkbank-transform     (depends on: all of the above)
```

### Pre-publish checklist per crate:

```bash
# For each crate in order:
cargo publish --dry-run -p <crate-name>
cargo publish -p <crate-name>
# Wait for crates.io to index before publishing dependents
```

## Workspace Versioning

All crates share version `0.1.0` via `[workspace.package]`.
Use `cargo-release` or `release-plz` for future version bumps.

## CI Configuration

```yaml
name: CI
on: [push, pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - run: cargo fmt --check
      - run: cargo check --all-targets
      - run: cargo clippy --all-targets -- -D warnings

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@nextest
      - run: cargo nextest run --workspace
      - run: cargo test --doc

  corpus:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: bash rust/no-cache-ref-test.sh
```

## Verification After Phase 2

1. All crates published to crates.io
2. `cargo install talkbank-tree-sitter-parser` works
3. External project can `cargo add talkbank-model` and build
4. Reference corpus tests pass (339/339)
5. `make verify` equivalent passes
