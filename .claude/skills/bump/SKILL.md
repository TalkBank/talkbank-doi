---
name: bump
description: Bump versions across TalkBank repos with cross-repo coordination. Handles workspace inheritance and multi-source sync (Cargo.toml + pyproject.toml + package.json). Use when bumping versions.
disable-model-invocation: true
allowed-tools: Bash, Read, Write, Edit, Glob, Grep
---

# Version Bump with Cross-Repo Coordination

Bump version numbers across one or more TalkBank repos. `$ARGUMENTS` should specify the repo and version (e.g., `/bump talkbank-tools 0.2.0` or `/bump all 0.2.0`).

## Version Source Map

| Repo | Primary Source | Inheritors | Secondary Sources |
|------|---------------|------------|-------------------|
| **talkbank-tools** | `Cargo.toml` `[workspace.package] version` | 10 crates via `version.workspace = true` | `vscode/package.json` `"version"`, `grammar/package.json` `"version"` |
| **batchalign3** | `pyproject.toml` `version` | — | `Cargo.toml` `[workspace.package] version` (Rust workspace), `crates/batchalign-cli/Cargo.toml` (CLI binary version) |

## Step 1: Read Current Versions

```bash
echo "=== talkbank-tools (Cargo) ===" && grep -A1 '\[workspace.package\]' /Users/chen/talkbank/talkbank-tools/Cargo.toml | grep version
echo "=== talkbank-tools (VS Code) ===" && grep '"version"' /Users/chen/talkbank/talkbank-tools/vscode/package.json | head -1
echo "=== talkbank-tools (grammar npm) ===" && grep '"version"' /Users/chen/talkbank/talkbank-tools/grammar/package.json | head -1
echo "=== batchalign3 (pyproject) ===" && grep '^version' /Users/chen/talkbank/batchalign3/pyproject.toml | head -1
echo "=== batchalign3 (Cargo workspace) ===" && grep -A1 '\[workspace.package\]' /Users/chen/talkbank/batchalign3/Cargo.toml | grep version
```

## Step 2: Determine Bump Scope

### Single repo bump
If bumping just one repo, only update that repo's version sources.

### talkbank-tools bump (cascading)
When talkbank-tools bumps, batchalign3 has path dependencies into talkbank-tools crates. The `version` fields in path deps are ignored locally but matter for crates.io. Check:

```bash
grep -n 'version.*=.*"0\.' /Users/chen/talkbank/batchalign3/crates/*/Cargo.toml /Users/chen/talkbank/batchalign3/pyo3/Cargo.toml 2>/dev/null | grep talkbank
```

### Coordinated bump (all repos)
Bump all repos to the same version for a major coordinated release.

## Step 3: Execute the Bump

### talkbank-tools
Edit root `Cargo.toml` `[workspace.package] version`. All member crates inherit automatically.

Also update:
- `vscode/package.json` version
- `grammar/package.json` version (if they should stay in sync)

### batchalign3
Edit ALL (must match):
- `pyproject.toml` `version`
- `Cargo.toml` `[workspace.package] version`

Verify sync:
```bash
cd /Users/chen/talkbank/batchalign3 && uv run python scripts/check_cli_version_sync.py
```

## Step 4: Update Cargo.lock Files

```bash
# For each repo with Cargo.toml changes
cd /Users/chen/talkbank/talkbank-tools && cargo check 2>/dev/null
cd /Users/chen/talkbank/batchalign3 && cargo check 2>/dev/null
```

This regenerates `Cargo.lock` with the new versions.

## Step 5: Verify

```bash
cd /Users/chen/talkbank/talkbank-tools && cargo check --workspace --all-targets
cd /Users/chen/talkbank/batchalign3 && cargo check --workspace --all-targets
```

## Step 6: Report

Summarize:
- Old version → new version (per repo)
- Files modified
- Any cascading updates in downstream repos
- Reminder: version bump is separate from release — use `/release` to tag and publish
