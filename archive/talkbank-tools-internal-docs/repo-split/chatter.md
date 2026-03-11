# Repo: chatter (CLI + LSP + VS Code)

**GitHub**: `talkbank/chatter`
**Phase**: 5

## Files to Move

```
rust/crates/talkbank-cli/       -> crates/talkbank-cli/
rust/crates/talkbank-lsp/       -> crates/talkbank-lsp/
rust/crates/talkbank-highlight/ -> crates/talkbank-highlight/
rust/crates/send2clan-sys/      -> crates/send2clan-sys/
rust/crates/talkbank-revai/     -> crates/talkbank-revai/
rust/vscode/                    -> vscode/
```

## Dependency Changes

All `talkbank-*` path/workspace deps become crates.io deps:

```toml
# Before (workspace deps)
talkbank-errors = { workspace = true }
talkbank-model = { workspace = true }
talkbank-transform = { workspace = true }
talkbank-clan = { workspace = true }
# etc.

# After (crates.io deps)
talkbank-errors = "0.1"
talkbank-model = "0.1"
talkbank-transform = "0.1"
talkbank-clan = "0.1"
# etc.
```

## New Cargo.toml (workspace root)

```toml
[workspace]
resolver = "2"
members = [
    "crates/talkbank-cli",
    "crates/talkbank-lsp",
    "crates/talkbank-highlight",
    "crates/send2clan-sys",
    "crates/talkbank-revai",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["TalkBank Team"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/talkbank/chatter"
```

## Publishing

| Target | Package | Notes |
|--------|---------|-------|
| crates.io | `talkbank-cli` | `cargo install talkbank-cli` gives `chatter` binary |
| VS Code Marketplace | `talkbank-chat` | Extension bundles LSP binary |
| Homebrew (future) | `chatter` | Formula for CLI |

## Verification

1. `cargo build -p talkbank-cli` succeeds with crates.io deps
2. `cargo test -p talkbank-cli` passes
3. `chatter validate` works on reference corpus
4. VS Code extension builds and installs
5. `cargo publish --dry-run -p talkbank-cli` succeeds
