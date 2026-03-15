# TalkBank Release Plan

**Status:** Current
**Last updated:** 2026-03-15

Phased plan for making TalkBank toolchain available. Starts with **private
GitHub repos** for team use, then public release, then PyPI.

## Current State

All repos are on personal private GitHub (`FranklinChen/*-private`). The
TalkBank GitHub org (`github.com/TalkBank`) exists with 54 repos (older
projects) but none of the new Rust/Python toolchain repos are there yet.

### Repo consolidation (completed 2026-03-08)

The original 8-repo structure has been consolidated to 2 active repos:

| Repo | Purpose | Ships |
|------|---------|-------|
| `talkbank-tools` | Unified: grammar, spec, 10 Rust crates, corpus, VS Code extension | `chatter` CLI, `talkbank-lsp-server`, VS Code extension |
| `batchalign3` | NLP pipeline: Rust server + Python ML workers | `batchalign3` (PyPI) |

Archived (read-only): `tree-sitter-talkbank`, `talkbank-chat`, `talkbank-chatter`,
`talkbank-clan`, `batchalign-hk-plugin`

Supporting (private): `talkbank-dev` workspace (deploy scripts, internal docs, ops)

### Dependency Graph

```
talkbank-tools (self-contained)
    10 crates, grammar, spec, corpus, VS Code
    CI: no sibling clones needed
    Release: 5-platform GitHub Release (chatter + lsp-server)
        │
        │  path dependencies (../../talkbank-tools/crates/)
        ▼
batchalign3
    3 Rust crates + pyo3/ bridge + Python inference
    CI: clones talkbank-tools for Rust path deps (uses TALKBANK_TOOLS_TOKEN)
    Release: batchalign3 (PyPI wheel — CLI + Python runtime)
```

**Key insight**: talkbank-tools is fully self-contained. batchalign3 depends
on it for path dependencies. So talkbank-tools must be on GitHub first.

### Release metadata (current)

| Field | talkbank-tools | batchalign3 |
|-------|---------------|-------------|
| Version | `0.1.0` | `1.0.0` |
| Repository URL | `github.com/TalkBank/talkbank-tools` | `github.com/TalkBank/batchalign3` |
| License | BSD-3-Clause | BSD-3-Clause |
| Release trigger | Git tag `v*` | Git tag `v*` |
| Binaries | `chatter`, `talkbank-lsp-server` | `batchalign3` console command |
| Packages | (GitHub Release only) | `batchalign3` (py≥3.12) |

### Platform matrix

| Platform | talkbank-tools | batchalign3 |
|----------|---------------|-------------|
| macOS ARM (aarch64-apple-darwin) | GitHub Release | PyPI wheel |
| macOS Intel (x86_64-apple-darwin) | GitHub Release | PyPI wheel |
| Linux x86_64 (manylinux 2_28) | GitHub Release | PyPI wheel |
| Linux ARM (aarch64, manylinux 2_28) | — | PyPI wheel |
| Windows (x86_64-pc-windows-msvc) | GitHub Release | PyPI wheel |

### Release-readiness workspace gates

Run these from the workspace root before public release:

- `make verify-contract-gates`
- `make verify-coverage-gates`
- `make verify-release-gates`

`verify-contract-gates` is the focused release-facing gate. It runs the new
contract suites across both repos:

- `talkbank-tools` CLI manifests, legacy CLAN compatibility, stateful CLI
  integration, and VS Code runtime/service seams
- `batchalign3` CLI manifests, legacy batchalign2 compatibility, command
  matrices, runtime-environment seams, and worker-protocol matrices

`verify-coverage-gates` uses the repos' existing coverage entrypoints rather
than a new custom framework:

- `talkbank-tools` Rust `cargo llvm-cov` coverage plus VS Code `npm run test:coverage`
- `batchalign3` Python `pytest --cov` plus Rust `cargo llvm-cov` coverage for
  both the workspace and `pyo3/`

`verify-release-gates` chains both and is the top-level local release-readiness
entrypoint.

### Current release posture (2026-03-15)

- Finish doc cleanup and release-prep work before pushing anything to the
  TalkBank org.
- Keep `batchalign3` PyPI publication **on hold for now**. Metadata, workflow,
  and trusted-publishing prep should stay current, but no release tag or PyPI
  publish step should happen yet.
- Treat Apple code signing + notarization as a public-release requirement for
  direct macOS downloads of `chatter` and `talkbank-lsp-server`. The current
  `talkbank-tools` release workflow emits macOS `.tar.gz` archives, so it needs
  a notarizable `.zip` or `.dmg` path before the first public CLI release.

---

## Phase 0: Pre-flight (completed 2026-03-09)

All items complete:

- [x] **Audit for secrets/credentials** — no real credentials found in
  committed code. All secrets read from `~/.batchalign.ini` at runtime.
- [x] **Audit `.gitignore` files** — both repos updated with comprehensive
  exclusions (Python, Node, Rust, editor, OS, coverage, mutation testing).
  Removed 32,731 tracked build artifacts from talkbank-tools index.
- [x] **Audit CLAUDE.md files** — all 8 files clean. No private paths,
  personal info, or internal infrastructure details.
- [x] **Review commit history** — full `git log --all -p` scan of both
  repos: no secrets, no credential files, no API keys ever committed.
- [x] **Fix stale CI repo references in batchalign3** — `test.yml`
  updated: `talkbank-chat` + `tree-sitter-talkbank` → `talkbank-tools`.
- [x] **Fix `docs.yml` branch** in batchalign3 — `master` → `main`.
- [x] **Verify `repository` URLs** in all Cargo.toml and pyproject.toml
  files point to `github.com/TalkBank/<repo>`. Added missing
  `repository` field to batchalign3 workspace Cargo.toml and
  `repository.workspace = true` to all three workspace crates.
- [x] **Fix release workflows** — both `release.yml` and `release-cli.yml`
  updated: multi-platform matrix (5 targets including Windows),
  `maturin-action`, sibling clone with `TALKBANK_TOOLS_TOKEN`, sdist job.
- [x] **Documentation audit** — 193 pages reviewed across both repos,
  10 blocked pages fixed, 0 remaining blockers.

### Doc audit status (completed 2026-03-09)

| Repo | Verified | Revised | Blocked | Total |
|------|----------|---------|---------|-------|
| talkbank-tools | 64 | 23 | **0** | 89 |
| batchalign3 | 76 | 24 | **0** | 104 |

Detailed trackers: `docs/release-doc-audit/matrices/`

---

## Phase 1: Private repos on TalkBank GitHub org

Goal: team members can clone, build, run CI, and contribute. Both repos
stay **private** — no public exposure yet.

### 1a. Push talkbank-tools (private)

- [ ] Create `TalkBank/talkbank-tools` on GitHub (**private**)
- [ ] Push main branch
- [ ] Verify CI passes (`make verify` — gates G0–G10, ~2300 tests)
- [ ] Confirm `corpus/reference/` 74-file suite is included
- [ ] Add team members as collaborators

### 1b. Push batchalign3 (private)

Requires talkbank-tools to be on GitHub first (CI clones it).

- [ ] Create `TalkBank/batchalign3` on GitHub (**private**)
- [ ] Create a **fine-grained PAT** (or GitHub App token) with read access
  to `TalkBank/talkbank-tools`. Store as repo secret `TALKBANK_TOOLS_TOKEN`
  on the batchalign3 repo.
- [ ] Push main branch
- [ ] Verify CI passes (`test.yml` — Python tests, Rust tests, typecheck)
- [ ] Add team members as collaborators

### 1c. Team onboarding

- [ ] Verify team can clone both repos and build locally:
  ```bash
  git clone git@github.com:TalkBank/talkbank-tools.git
  git clone git@github.com:TalkBank/batchalign3.git
  cd batchalign3 && make sync && make build
  ```
- [ ] Verify `batchalign3 --help` runs after build
- [ ] Collect feedback on README, setup friction, missing docs

---

## Phase 2: Make talkbank-tools public + ship chatter CLI

talkbank-tools is self-contained — low risk to make public first.

### 2a. Make repo public

- [ ] Flip `TalkBank/talkbank-tools` from private → **public**
- [ ] Verify CI still passes
- [ ] Verify README installation instructions are correct

### 2b. Ship first chatter release

The `release.yml` workflow is already configured for multi-platform archives,
but macOS signing/notarization is not wired yet:

- [ ] Export the Apple Developer ID Application certificate as a portable `.p12`
- [ ] Create an App Store Connect API key for notarization
- [ ] Change the macOS release packaging from `.tar.gz` to a notarizable `.zip`
  or `.dmg`
- [ ] Add macOS codesign + notarization for both `chatter` and
  `talkbank-lsp-server`
- [ ] Verify `Cargo.toml` workspace version (`0.1.0`)
- [ ] Create and push `v0.1.0` tag
- [ ] Verify `release.yml` succeeds:
  - Builds on 5 platforms (macOS ARM, macOS Intel, Linux, Windows)
  - Creates GitHub Release with `chatter` + `talkbank-lsp-server` archives
  - Tag version matches workspace Cargo.toml (validated in workflow)
- [ ] Download and test each archive on available platforms, including a clean
  macOS machine for Gatekeeper/notarization behavior

### 2c. Announce

- [ ] Link from talkbank.org if appropriate
- [ ] Consider a Homebrew tap for easier macOS installation

---

## Phase 3: Make batchalign3 public

batchalign3 stays private until the team is confident it's ready.

### 3a. Pre-public checklist

- [ ] Team consensus that batchalign3 is ready for external users
- [ ] README covers all user-facing commands and setup
- [ ] Migration guide from batchalign2 is complete and tested
- [ ] Known issues documented

### 3b. Make repo public

- [ ] Flip `TalkBank/batchalign3` from private → **public**
- [ ] Update CI clone to drop token (or keep for resilience):
  once talkbank-tools is public, `TALKBANK_TOOLS_TOKEN` is optional
- [ ] Verify CI passes

---

## Phase 4: Publish batchalign3 on PyPI (on hold — prep only for now)

One PyPI package:
- `batchalign3` — Python package with Rust extension + CLI console command (ASR, alignment, morphosyntax)

Current posture: keep the release metadata, GitHub Actions workflow, and
trusted-publishing setup ready, but do **not** tag or publish to PyPI until the
hold is lifted after the private/public release hardening work.

### Old batchalign PyPI page (reference)

The `batchalign` 0.8.2 page on PyPI had:
- **Name**: batchalign (we use `batchalign3` — different package, no conflict)
- **Summary**: "Python Speech Language Sample Analysis" (reuse this)
- **Authors**: Brian MacWhinney, Houjun Liu (add Franklin Chen for v3)
- **Classifiers**: Development Status, Python versions, Topic :: Utilities
- **Description**: Overview, installation (UV), CLI usage (transcribe/align/morphotag),
  Python API (Quick Pipeline, Manual Pipeline), CHAT + TextGrid format support
- **NIH grant acknowledgment**: "supported by NIH grant HD082736"

### 4a. Update PyPI metadata

Current `pyproject.toml` is close but needs review:

- [ ] Update description to match batchalign2 style (overview, install, CLI,
  format support) while documenting v3 differences
- [ ] Bump classifiers: `Development Status :: 5 - Production/Stable`,
  add Python 3.14 when supported
- [ ] Add `keywords` if missing
- [ ] Verify `[project.urls]` links work

### 4b. Configure PyPI trusted publishing

- [ ] Create PyPI account (or use existing)
- [ ] Keep `release.yml` manual-only (`workflow_dispatch`) until the hold is lifted
- [ ] Set up OIDC trusted publishing for `batchalign3`:
  GitHub org `TalkBank`, repo `batchalign3`, workflow `release.yml`,
  environment `pypi`
- [ ] Register package name on PyPI (first publish claims it)

### 4c. Test end-to-end release (when the hold is lifted)

- [ ] Tag `v1.0.0`
- [ ] Verify `release.yml` triggers and succeeds:
  - `release.yml` → `batchalign3` wheels (5 platforms) on PyPI + sdist
- [ ] Test installation on each platform:
  ```bash
  uv tool install batchalign3
  batchalign3 --version
  ```
- [ ] Verify the CLI binary can dispatch to Python workers

### 4d. Announce

- [ ] Update talkbank.org documentation
- [ ] Consider deprecation notice on old `batchalign` (0.x) PyPI package
- [ ] Announce on relevant mailing lists / communities

---

## Phase 5: VS Code extension (marketplace)

The VS Code extension lives at `talkbank-tools/vscode/`.

- [ ] Publish to VS Code Marketplace
- [ ] Bundle or document how to install `talkbank-lsp-server` binary
- [ ] Set up CI for marketplace publishing on tag push

---

## Phase 6: Package registry publishing (eventual, after first public releases)

Publishing to crates.io enables third-party Rust consumers to depend on
TalkBank crates without cloning repos. We do want this eventually, but it is
not on the critical path for the first public `chatter` / `talkbank-lsp-server`
binary releases or the first `batchalign3` package release.

### 6a. Publish talkbank-tools crates to crates.io

9 publishable crates (talkbank-parser-tests is `publish = false`).

Required publish order (respecting dependencies):

```
1. talkbank-derive          (no talkbank deps)
2. talkbank-model           (depends on talkbank-derive)
3. talkbank-direct-parser   (depends on model)
4. talkbank-parser           (depends on model, direct-parser)
5. talkbank-transform       (depends on parser, direct-parser, model)
6. talkbank-clan            (depends on model, transform)
7. talkbank-cli             (depends on all above)
8. talkbank-lsp             (depends on model, parser, transform, clan)
9. send2clan-sys            (C FFI, depends on nothing)
```

- [ ] Verify each crate's Cargo.toml metadata (workspace inheritance)
- [ ] Run `cargo publish --dry-run -p <crate>` for each in order
- [ ] Publish each crate in order
- [ ] Add release workflow that publishes all crates on tag push

### 6b. Publish tree-sitter-talkbank to npm/crates.io

The grammar lives at `talkbank-tools/grammar/` but could also be published
as a standalone tree-sitter package for editor integrations.

- [ ] Decide if standalone package is needed (grammar is already in talkbank-tools)
- [ ] If yes: extract grammar/ into publishable form, publish to npm + crates.io

---

## Suggested Execution Order

```
Phase 0:  Pre-flight (DONE)
Phase 1:  Private repos on TalkBank org — team starts using
          1a: talkbank-tools private
          1b: batchalign3 private (needs TALKBANK_TOOLS_TOKEN secret)
          1c: Team onboarding + feedback
Phase 2:  talkbank-tools public + chatter GitHub Release
Phase 3:  batchalign3 public (when team agrees it's ready)
Phase 4:  PyPI publishing (batchalign3)
Phase 5:  VS Code marketplace
Phase 6:  crates.io (eventual, after the first public releases settle)
```

Phase 1 can happen immediately. Phase 2 can follow quickly since
talkbank-tools is self-contained and low risk. Phases 3–4 happen after
team has used batchalign3 privately and confirmed readiness. Phase 6 remains a
real target, but it should follow the initial binary/wheel release work rather
than block it.

---

## CI Authentication for Private Repos

batchalign3's CI (`test.yml`, `release.yml`) clones
talkbank-tools as a sibling dependency. While both repos are private,
this requires authentication:

```yaml
- name: Clone sibling dependencies
  run: git clone --depth 1 https://x-access-token:${{ secrets.TALKBANK_TOOLS_TOKEN }}@github.com/TalkBank/talkbank-tools ../talkbank-tools
```

**Setup**: Create a fine-grained PAT with `contents: read` on
`TalkBank/talkbank-tools`. Store as `TALKBANK_TOOLS_TOKEN` in
batchalign3's repo secrets. Once talkbank-tools goes public (Phase 2),
the token becomes optional but harmless to keep.

---

## Risk Checklist

| Risk | Mitigation |
|------|------------|
| Secrets in git history | Scanned — clean (Phase 0) |
| PyPI name conflict with old `batchalign` | Different name (`batchalign3`), no conflict |
| CI breaks when repos move to TalkBank org | talkbank-tools CI is self-contained; batchalign3 uses `TALKBANK_TOOLS_TOKEN` |
| Private cross-repo clone auth | Fine-grained PAT stored as repo secret |
| Multi-platform batchalign3 wheel | 5-platform matrix (incl. Windows) in release.yml |
| Unsigned macOS CLI downloads trigger Gatekeeper friction | Add Developer ID signing + notarization and ship notarizable macOS `.zip`/`.dmg` artifacts before public CLI release |
| Private paths in CLAUDE.md/docs | Audited — clean (Phase 0) |
| Path deps break in CI | `../talkbank-tools/crates/` resolves after shallow clone |
| Windows compatibility | Windows target in all release matrices; PowerShell-compatible clone commands |
| Team not ready for public | Private-first approach; public only after team consensus |

---
Last Updated: 2026-03-09
