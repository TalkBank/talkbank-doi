---
name: release
description: Coordinate a release across TalkBank repos. Covers tagging, CI checks, GitHub Releases, and PyPI publishing. Use when the user wants to cut a release.
disable-model-invocation: true
allowed-tools: Bash, Read, Glob, Grep
---

# Release Coordination

Guide the release process for one or more TalkBank repos. `$ARGUMENTS` should specify the repo and version (e.g., `/release talkbank-tools 0.2.0` or `/release batchalign3 1.1.0`).

## Release Targets

| Repo | Artifacts | Distribution |
|------|-----------|--------------|
| **talkbank-tools** | `chatter` CLI binary, `talkbank-lsp-server` binary | GitHub Releases (cross-platform binaries) |
| **talkbank-tools** | VS Code extension `.vsix` | VS Code Marketplace (future) |
| **batchalign3** | `batchalign3` Python wheel (with Rust extension) | PyPI (5-platform manylinux/macOS/Windows) |
| **batchalign3** | `batchalign3` console command (via `[project.scripts]`) | Part of the `batchalign3` PyPI wheel |

## Pre-Release Checklist

### 1. Verify everything passes

```bash
cd /Users/chen/talkbank/talkbank-tools && make verify
cd /Users/chen/talkbank/batchalign3 && make test
```

### 2. Check version numbers

```bash
echo "=== talkbank-tools ===" && grep -A1 '\[workspace.package\]' /Users/chen/talkbank/talkbank-tools/Cargo.toml | grep version
echo "=== batchalign3 ===" && grep '^version' /Users/chen/talkbank/batchalign3/pyproject.toml | head -1
```

If versions need bumping, use `/bump` first.

### 3. Check for uncommitted changes

```bash
cd /Users/chen/talkbank/talkbank-tools && git status --short
cd /Users/chen/talkbank/batchalign3 && git status --short
```

### 4. Review recent commits since last tag

```bash
cd /Users/chen/talkbank/talkbank-tools && git log --oneline $(git describe --tags --abbrev=0 2>/dev/null || echo HEAD~20)..HEAD
cd /Users/chen/talkbank/batchalign3 && git log --oneline $(git describe --tags --abbrev=0 2>/dev/null || echo HEAD~20)..HEAD
```

## Release Process

### talkbank-tools

1. Ensure version is bumped in `Cargo.toml`
2. Commit and push to main
3. Create and push a version tag:
   ```bash
   cd /Users/chen/talkbank/talkbank-tools
   git tag -a v<VERSION> -m "Release v<VERSION>"
   git push origin v<VERSION>
   ```
4. GitHub Actions `release.yml` builds cross-platform binaries and creates a GitHub Release
5. Verify the release at `https://github.com/TalkBank/talkbank-tools/releases`

### batchalign3

1. Ensure version is bumped in `pyproject.toml` + `Cargo.toml`
2. Commit and push to main
3. Create and push a version tag:
   ```bash
   cd /Users/chen/talkbank/batchalign3
   git tag -a v<VERSION> -m "Release v<VERSION>"
   git push origin v<VERSION>
   ```
4. GitHub Actions `release.yml` builds 5-platform wheels via maturin-action and publishes to PyPI (OIDC trusted publishing)
5. Verify: `uv tool install batchalign3==<VERSION>` and check `https://pypi.org/project/batchalign3/`

## Post-Release

- Update CHANGELOG if maintained
- Announce to team
- Check that CI on the tag passed (GitHub Actions)
- For batchalign3: verify wheel installs correctly on a clean machine

## CI Authentication (Private Repos)

While repos are private, CI needs `TALKBANK_TOOLS_TOKEN` (fine-grained PAT) for cross-repo clones. This is configured as a GitHub Actions secret. When repos go public, this can be removed.

## Troubleshooting

| Problem | Fix |
|---------|-----|
| CI fails on tag push | Check the workflow file for the correct trigger (`push: tags: ["v*"]`) |
| PyPI publish fails | Verify OIDC trusted publisher is configured for the GitHub repo |
| Cross-platform build fails | Check maturin-action matrix and target triples |
| Wheel won't install | Check Python version constraints in pyproject.toml |
