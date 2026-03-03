---
name: bump
description: Bump versions across TalkBank repos with cross-repo coordination. Handles workspace inheritance, multi-source sync (Cargo.toml + package.json), and downstream path dependency version fields. Use when bumping versions.
disable-model-invocation: true
allowed-tools: Bash, Read, Write, Edit, Glob, Grep
---

# Version Bump with Cross-Repo Coordination

Bump version numbers across one or more TalkBank repos. `$ARGUMENTS` should specify the repo and version (e.g., `/bump talkbank-chat 0.2.0` or `/bump all 0.2.0`).

## Version Source Map

| Repo | Primary Source | Inheritors | Secondary Sources |
|------|---------------|------------|-------------------|
| **talkbank-chat** | `Cargo.toml` `[workspace.package] version` | 14 crates via `version.workspace = true` | Workspace dep version fields (lines ~120-129) |
| **talkbank-chatter** | `Cargo.toml` `[workspace.package] version` | 4 crates | `vscode/package.json` `"version"` |
| **talkbank-clan** | `Cargo.toml` `[package] version` | (standalone) | — |
| **tree-sitter-talkbank** | `Cargo.toml` `[package] version` | — | `package.json` `"version"` (must match!) |
| **batchalign3** | `pyproject.toml` `version` | — | `rust-next/pyproject.toml`, `rust-next/crates/batchalign-bin/Cargo.toml` (all 3 must match) |

## Step 1: Read Current Versions

```bash
echo "=== talkbank-chat ===" && grep -A1 '\[workspace.package\]' /Users/chen/talkbank/talkbank-chat/Cargo.toml | grep version
echo "=== talkbank-chatter ===" && grep -A1 '\[workspace.package\]' /Users/chen/talkbank/talkbank-chatter/Cargo.toml | grep version
echo "=== talkbank-clan ===" && head -5 /Users/chen/talkbank/talkbank-clan/Cargo.toml | grep version
echo "=== tree-sitter-talkbank (Cargo) ===" && head -5 /Users/chen/talkbank/tree-sitter-talkbank/Cargo.toml | grep version
echo "=== tree-sitter-talkbank (npm) ===" && grep '"version"' /Users/chen/talkbank/tree-sitter-talkbank/package.json | head -1
echo "=== talkbank-chatter vscode ===" && grep '"version"' /Users/chen/talkbank/talkbank-chatter/vscode/package.json | head -1
echo "=== batchalign3 ===" && grep '^version' /Users/chen/talkbank/batchalign3/pyproject.toml | head -1
echo "=== batchalign3-cli (pyproject) ===" && grep '^version' /Users/chen/talkbank/batchalign3/rust-next/pyproject.toml | head -1
echo "=== batchalign3-cli (Cargo) ===" && grep '^version' /Users/chen/talkbank/batchalign3/rust-next/crates/batchalign-bin/Cargo.toml | head -1
```

## Step 2: Determine Bump Scope

### Single repo bump
If bumping just one repo, only update that repo's version sources.

### talkbank-chat bump (cascading)
When talkbank-chat bumps, the `version` field in downstream path dependencies may need updating:

**talkbank-chatter `Cargo.toml`** workspace dependencies section:
```toml
talkbank-errors = { path = "../talkbank-chat/crates/talkbank-errors", version = "0.1.0" }
talkbank-model = { path = "../talkbank-chat/crates/talkbank-model", version = "0.1.0" }
# ... other talkbank-chat crates
```

These `version` fields are ignored for local builds (path deps always use the local code), but they matter if crates are ever published to crates.io. Update them to match the new talkbank-chat version.

**talkbank-clan `Cargo.toml`** dependencies:
```toml
talkbank-errors = { path = "../talkbank-chat/crates/talkbank-errors" }
# (no version field — less strict, but consider adding for consistency)
```

**batchalign3 `rust/crates/batchalign-core/Cargo.toml`** and **`rust-next/`** crates:
```toml
talkbank-model = { path = "../../../../talkbank-chat/crates/talkbank-model" }
# (path deps, no version field typically)
```

### Coordinated bump (all repos)
Bump all repos to the same version for a major coordinated release.

## Step 3: Execute the Bump

For each repo being bumped:

### Workspace repos (talkbank-chat, talkbank-chatter)
Only edit the root `Cargo.toml` `[workspace.package] version` line. All member crates inherit automatically.

Also update the version fields in `[workspace.dependencies]` for any workspace member cross-references.

### Standalone repos (talkbank-clan)
Edit `Cargo.toml` `[package] version` directly.

### Dual-source repos (tree-sitter-talkbank)
Edit BOTH:
- `Cargo.toml` `[package] version`
- `package.json` `"version"`

Verify they match:
```bash
cargo_v=$(grep '^version' /Users/chen/talkbank/tree-sitter-talkbank/Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
npm_v=$(grep '"version"' /Users/chen/talkbank/tree-sitter-talkbank/package.json | head -1 | sed 's/.*"\(.*\)".*/\1/')
[ "$cargo_v" = "$npm_v" ] && echo "MATCH: $cargo_v" || echo "MISMATCH: Cargo=$cargo_v npm=$npm_v"
```

### Triple-source repos (batchalign3)
Edit ALL THREE:
- `pyproject.toml` `version`
- `rust-next/pyproject.toml` `version`
- `rust-next/crates/batchalign-bin/Cargo.toml` `version`

Verify:
```bash
cd /Users/chen/talkbank/batchalign3 && uv run python scripts/check_cli_version_sync.py
```

## Step 4: Update Cargo.lock Files

```bash
# For each repo with Cargo.toml changes
cd /Users/chen/talkbank/<repo> && cargo check 2>/dev/null
```

This regenerates `Cargo.lock` with the new versions.

## Step 5: Verify

```bash
# Quick compile check across repos
cd /Users/chen/talkbank/talkbank-chat && cargo check --all-targets
cd /Users/chen/talkbank/talkbank-chatter && cargo check --all-targets
cd /Users/chen/talkbank/talkbank-clan && cargo check --all-targets
```

## Step 6: Report

Summarize:
- Old version → new version (per repo)
- Files modified
- Any cascading updates in downstream repos
- Reminder: version bump is separate from release — use `/release` to tag and publish
