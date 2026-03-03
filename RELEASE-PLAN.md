# TalkBank Public Release Plan

Phased plan for making TalkBank toolchain components available to external
users. Priority: **chatter CLI** and **batchalign3**.

## Current State

All repos are on personal private GitHub (`FranklinChen/*-private`). The
TalkBank GitHub org (`github.com/TalkBank`) exists with 54 repos (older
projects) but none of the new Rust/Python toolchain repos are public yet.

CI workflows already reference the intended public URLs:
- `github.com/TalkBank/talkbank-chat`
- `github.com/TalkBank/talkbank-chatter` (binary repo name in Cargo.toml: `chatter`)
- `github.com/TalkBank/talkbank-clan` (binary repo name in Cargo.toml: `clan`)
- `github.com/TalkBank/tree-sitter-talkbank`
- `github.com/TalkBank/batchalign3`

Package names are all available on their respective registries (crates.io,
npm, PyPI for `batchalign3`/`batchalign3-cli`). The old `batchalign` (0.x)
exists on PyPI but the `batchalign3` name is free.

## Dependency Graph (what blocks what)

```
tree-sitter-talkbank          ← independent (npm + crates.io)
        │
talkbank-chat (9 crates)      ← depends on tree-sitter-talkbank
        │
   ┌────┴────┐
   │         │
talkbank-clan    batchalign3 (PyO3 + rust-next)
   │                │
talkbank-chatter    batchalign3-cli
(chatter CLI)
```

**Key insight**: chatter and batchalign3 binaries can ship via GitHub
Releases and PyPI wheels (which bundle compiled code) *without* publishing
any crates to crates.io first. Crates.io publishing is a separate,
lower-priority track that enables third-party Rust consumers.

---

## Phase 0: Pre-flight (do before anything goes public)

These are one-time tasks that apply across all repos.

- [ ] **Audit for secrets/credentials** — grep all repos for API keys,
  tokens, passwords, private paths. Check `.env` files, config files,
  CI workflow secrets.
- [ ] **Audit `.gitignore` files** — ensure build artifacts, caches,
  `.DS_Store`, editor configs, and any local-only files are excluded.
- [ ] **Audit CLAUDE.md files** — remove any private paths, internal
  server names, or personal infrastructure details that shouldn't be
  public. The per-crate CLAUDE.md files are great for contributors but
  review them for private info.
- [ ] **Review commit history** — check for accidentally committed
  secrets in git history. If found, use `git filter-repo` to scrub
  before pushing to public repos.
- [ ] **Create `talkbank-chatter/README.md`** — this is the only repo
  missing a root README (the `vscode/` subdirectory has one but the
  repo root does not).
- [ ] **Decide on repo naming** — Cargo.toml `repository` fields
  currently point to `TalkBank/chatter` and `TalkBank/clan` but the
  local directory names are `talkbank-chatter` and `talkbank-clan`.
  Pick one convention and make it consistent.

---

## Phase 1: Make foundational repos public on GitHub

These repos have no user-facing installable — they're libraries and
build dependencies. Making them public first enables CI for downstream
repos.

### 1a. `tree-sitter-talkbank`

- [ ] Create `TalkBank/tree-sitter-talkbank` on GitHub (public)
- [ ] Push main branch
- [ ] Verify CI passes (232 tree-sitter tests)
- [ ] Confirm `package.json` and `Cargo.toml` metadata are correct

### 1b. `talkbank-chat`

- [ ] Create `TalkBank/talkbank-chat` on GitHub (public)
- [ ] Push main branch
- [ ] Verify CI passes (`make verify` — all gates G0–G7, 1422 tests)
- [ ] Confirm the `corpus/reference/` 73-file suite is included
  (this is essential for contributors to run tests)

**Once both are public, all downstream repos can clone them in CI.**

---

## Phase 2: Ship chatter CLI (GitHub Releases)

chatter is the user-facing CLI tool for CHAT validation, conversion, and
analysis. Users download a pre-built binary — no Rust toolchain needed.

### 2a. `talkbank-clan` (dependency of chatter)

- [ ] Create `TalkBank/talkbank-clan` on GitHub (public)
  - Verify the repo URL matches what `talkbank-chatter` CI clones
    (`TalkBank/talkbank-clan` — currently CI references this)
- [ ] Push main branch
- [ ] Verify CI passes (129 tests)

### 2b. `talkbank-chatter`

- [ ] Create `TalkBank/talkbank-chatter` on GitHub (public)
  - Or `TalkBank/chatter` — settle the naming question from Phase 0
- [ ] Push main branch
- [ ] Verify CI passes (113 tests, requires cloning 3 siblings)
- [ ] **Test the release workflow**:
  1. Verify `Cargo.toml` workspace version is set to desired release version
  2. Create and push a `v0.1.0` tag
  3. Verify `release.yml` succeeds:
     - Builds on all 4 platforms (macOS ARM, macOS Intel, Linux, Windows)
     - Creates GitHub Release with archives attached
     - Tag version matches `Cargo.toml` version (validated in workflow)
  4. Download and test each archive on available platforms

### 2c. Announce availability

- [ ] Update `talkbank-chatter/README.md` with installation instructions
  (download from GitHub Releases, put on PATH)
- [ ] Link from talkbank.org if appropriate
- [ ] Consider a Homebrew tap or similar for easier installation later

---

## Phase 3: Ship batchalign3 (PyPI)

batchalign3 has two PyPI packages:
- `batchalign3` — Python package with Rust extension (ASR, alignment, morphosyntax)
- `batchalign3-cli` — standalone Rust binary (server/client, no ML models)

### 3a. Make `batchalign3` repo public

- [ ] Create `TalkBank/batchalign3` on GitHub (public)
- [ ] Push main branch
- [ ] Verify CI passes (`test.yml` — 878 Python tests, Rust tests, typecheck)

### 3b. Fix release workflow for multi-platform wheels

The current `release.yml` builds on `ubuntu-latest` only. Since
`batchalign3` includes a compiled Rust extension (`batchalign_core`),
a single-platform build produces only a Linux wheel.

**Options** (pick one):
- [ ] **Option A: Multi-platform matrix** — add macOS ARM, macOS Intel,
  Windows to `release.yml` using `maturin-action`, similar to `release-cli.yml`
- [ ] **Option B: Separate the Rust extension** — publish `batchalign-core`
  as a separate PyPI package with multi-platform wheels, make `batchalign3`
  a pure Python package that depends on it
- [ ] **Option C: sdist + build-on-install** — publish only an sdist that
  compiles Rust during `pip install` (requires users to have Rust installed;
  not recommended for end users)

**Recommendation**: Option A is simplest and matches the existing
`release-cli.yml` pattern. The main complication is that `batchalign3`
has path deps to `talkbank-chat` crates — the release workflow needs
to clone sibling repos (same pattern as chatter's release workflow).

### 3c. Configure PyPI trusted publishing

- [ ] Set up OIDC trusted publishing on PyPI for `batchalign3` package
  (GitHub Actions → PyPI, no API token needed)
- [ ] Set up OIDC trusted publishing for `batchalign3-cli` package
- [ ] Register the package names on PyPI (first publish claims them)

### 3d. Test end-to-end release

- [ ] Tag `v1.0.0` (versions are already 1.0.0 in both pyproject.toml files)
- [ ] Verify both workflows trigger and succeed:
  - `release.yml` → `batchalign3` wheel(s) on PyPI
  - `release-cli.yml` → `batchalign3-cli` wheels on PyPI
- [ ] Test installation:
  ```bash
  pip install batchalign3           # Python package
  pip install batchalign3-cli       # Standalone binary
  batchalign3 --version
  ```
- [ ] Verify the CLI binary finds Python and can dispatch to workers

### 3e. Announce availability

- [ ] Update `batchalign3/README.md` installation section
  (currently says `uv tool install batchalign3` — verify this works from PyPI)
- [ ] Update talkbank.org documentation
- [ ] Consider deprecation notice on old `batchalign` (0.x) PyPI package

---

## Phase 4: Package registry publishing (optional, lower priority)

Publishing to crates.io and npm enables third-party Rust/JS consumers to
depend on TalkBank crates without cloning repos. This is **not required**
for chatter or batchalign3 distribution — those ship as pre-built binaries.

### 4a. Publish `tree-sitter-talkbank` to npm and crates.io

Both `package.json` and `Cargo.toml` are already configured.

- [ ] `npm publish` (from `tree-sitter-talkbank/`)
- [ ] `cargo publish` (from `tree-sitter-talkbank/`)
- [ ] Add a release workflow to automate this on tags

### 4b. Publish `talkbank-chat` crates to crates.io

The workspace already has `path + version` deps set up (line 119–142 of
`Cargo.toml`), so crates.io will use the version spec while local dev
uses the path. **BUT**: all talkbank-chat crates depend on
`tree-sitter-talkbank`, which must be on crates.io first (Phase 4a).

Required publish order (respecting internal dependencies):

```
1. talkbank-derive          (no talkbank deps)
2. talkbank-json            (no talkbank deps)
3. talkbank-errors          (depends on talkbank-derive)
4. talkbank-model           (depends on talkbank-derive, talkbank-errors)
5. talkbank-parser-api      (depends on talkbank-errors, talkbank-model)
6. talkbank-pipeline        (depends on talkbank-errors, talkbank-model)
7. talkbank-direct-parser   (depends on errors, model, parser-api)
8. talkbank-tree-sitter-parser (depends on errors, model, parser-api,
                                direct-parser, tree-sitter-talkbank)
9. talkbank-transform       (depends on all of the above)
```

(`talkbank-parser-tests` is `publish = false` — dev-only, not published.)

- [ ] Verify each crate's `Cargo.toml` has description, license, repository
  (currently: all set via workspace inheritance)
- [ ] Run `cargo publish --dry-run -p <crate>` for each in order
- [ ] Publish each crate in order (sequential — each must be on crates.io
  before the next can reference it)
- [ ] Add a release workflow that publishes all crates in order on tag push

### 4c. Publish `talkbank-clan` to crates.io

- [ ] Update path deps to version deps (comments in Cargo.toml already
  show the target state)
- [ ] `cargo publish --dry-run`
- [ ] `cargo publish`

### 4d. Publish `talkbank-chatter` crates to crates.io (optional)

The CLI and LSP are primarily distributed as binaries. Publishing to
crates.io lets users `cargo install chatter` but is lower priority.

- [ ] Update path deps to version deps
- [ ] Decide which crates to publish (all 4, or just the binaries?)
- [ ] `cargo publish` in order: send2clan-sys → talkbank-highlight →
  talkbank-lsp → talkbank-cli

---

## Phase 5: VS Code extension (marketplace)

- [ ] Publish `talkbank-chatter/vscode/` to VS Code Marketplace
- [ ] Bundle the `talkbank-lsp-server` binary (or document how to install it)
- [ ] Set up CI for marketplace publishing on tag push

---

## Suggested Execution Order

For the fastest path to "users can install chatter and batchalign3":

```
Week 1:  Phase 0 (audit, prep)
Week 2:  Phase 1 (tree-sitter-talkbank + talkbank-chat public)
Week 2:  Phase 2a (talkbank-clan public)
Week 2:  Phase 2b (talkbank-chatter public + first GitHub Release)
Week 3:  Phase 3a–3c (batchalign3 public + PyPI setup)
Week 3:  Phase 3d (first PyPI release)
Later:   Phase 4 (crates.io — when there's demand from Rust consumers)
Later:   Phase 5 (VS Code marketplace)
```

Phases 2 and 3 can run in parallel since they're independent.

---

## Risk Checklist

| Risk | Mitigation |
|------|------------|
| Secrets in git history | Run `git log --all -p` search before going public |
| crates.io name squatting | Names are currently free; publish placeholder if concerned |
| PyPI name conflict with old `batchalign` | Different name (`batchalign3`), no conflict |
| CI breaks when repos go public | CI already references public URLs; test by pushing to TalkBank org |
| Cross-repo CI clones fail | Ensure all sibling repos are public before testing CI |
| `release.yml` needs sibling repos | Release workflow already clones them; just need them public |
| Multi-platform batchalign3 wheel | Must fix `release.yml` before first PyPI publish (Phase 3b) |
| Private paths in CLAUDE.md/docs | Audit before pushing (Phase 0) |

---
Last Updated: 2026-03-01
