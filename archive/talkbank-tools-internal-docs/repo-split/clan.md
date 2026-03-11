# Repo: clan (CLAN Analysis)

**GitHub**: `talkbank/clan`
**Phase**: 4

## Files to Move

```
rust/crates/talkbank-clan/      -> src/ (or crates/talkbank-clan/)
rust/crates/talkbank-clan/tests/ -> tests/
```

## Dependency Changes

```toml
# Before (workspace deps)
talkbank-errors = { workspace = true }
talkbank-model = { workspace = true }
talkbank-pipeline = { workspace = true }
talkbank-transform = { workspace = true }

# After (crates.io deps)
talkbank-errors = "0.1"
talkbank-model = "0.1"
talkbank-pipeline = "0.1"
talkbank-transform = "0.1"
```

## Publishing

| Target | Package | Notes |
|--------|---------|-------|
| crates.io | `talkbank-clan` | Library crate for analysis commands |

## Golden Tests

Golden tests compare output against legacy CLAN C binaries.
These tests need CLAN binaries (resolved via `CLAN_BIN_DIR` env var, `../OSX-CLAN/src/unix/bin/` workspace sibling, or `~/OSX-CLAN/src/unix/bin/` fallback).
In CI, golden tests are skipped when binaries aren't found.

## Verification

1. `cargo test -p talkbank-clan` passes
2. Golden tests pass (when CLAN binaries available)
3. `cargo publish --dry-run` succeeds
