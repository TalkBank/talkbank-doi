# Portfolio Reorg Plan for First Public Releases (Chatter CLI first, Batchalign3 second)

## Summary
Keep 5 GitHub repos (no merges now), but impose hard contracts so releases are deterministic:

1. `tree-sitter-talkbank` (grammar, independent lifecycle)
2. `talkbank-chat` (core Rust model/parser/transform + spec tooling)
3. `talkbank-clan` (analysis library)
4. `talkbank-chatter` (CLI/LSP/VS Code)
5. `batchalign3` (Python product + Rust internals)

Primary public milestone sequence:
1. Foundation releases needed for transitive installability.
2. `chatter` as installable CLI (`cargo install talkbank-cli --bin chatter`).
3. `batchalign3` release immediately after, pinned to published crates.

## Target Repo/Crate Boundaries

### Repo boundaries (final)
- Keep `tree-sitter-talkbank` separate.
- Keep `talkbank-clan` separate.
- Keep `batchalign3` as one repo with a `pyo3/` bridge crate and the root `crates/` workspace.

### Crate boundaries (final)
- `talkbank-chat`: keep current crate split (no merge now), but classify:
  - Public crates: `talkbank-errors`, `talkbank-model`, `talkbank-parser-api`, `talkbank-pipeline`, `talkbank-json`, `talkbank-tree-sitter-parser`, `talkbank-transform`, `talkbank-derive`
  - Private crates: `talkbank-parser-tests` (`publish = false`), `talkbank-tools` (`publish = false`), `talkbank-direct-parser` (decide public/private now; default private for v0.1 unless external consumer exists)
- `talkbank-chatter`: public crates are `talkbank-cli` (required), optionally `talkbank-lsp`/`talkbank-highlight`; keep `send2clan-sys` as crate.
- `batchalign3`: no crates.io publication requirement for its internal Rust crates in v1 release path.

## Dependency Contract (enforced)
Allowed direction only:

`tree-sitter-talkbank`
-> `talkbank-chat` public crates
-> (`talkbank-clan`, `talkbank-chatter`, `batchalign3`)

Rules:
- No repo may depend on `talkbank-parser-tests`.
- No production crate may depend on test harness crates.
- No path deps across repos on release branches; only semver deps.
- `talkbank-chatter` must not depend on `batchalign3`; `batchalign3` must not depend on `talkbank-chatter`/`talkbank-clan`.

## Required Interface/API Changes

1. `talkbank-chatter`:
- Replace all cross-repo path deps with versioned crates.io deps.
- Remove runtime dependency on `talkbank-parser-tests` from `talkbank-cli`.
- Move `MinimalChatFile` (currently coming from parser-tests) into a non-test public crate (`talkbank-model` or `talkbank-transform`) and update imports.
- Keep binary interface stable: `chatter` command name unchanged.

2. `talkbank-lsp` manifest:
- Replace direct `path` dep (`talkbank-clan`) with workspace version dependency.

3. `batchalign3` (`pyo3` and `crates`):
- Replace `../../talkbank-chat/...` path deps with crates.io versions.
- Document role split:
  - `pyo3/`: PyO3 packaging and Python-facing bridge.
  - `crates/`: CLI, app, and shared Rust workspace crates.

4. `talkbank-chat`:
- Ensure public crates have complete publish metadata (`description`, `readme`, `license`, keywords/categories already mostly present).
- Confirm `publish = false` where intended for private crates.

## Release Plan (decision-complete)

### Phase 0: Contract Hardening (all repos, no public release yet)
- Add dependency-policy checks in CI:
  - fail on cross-repo path deps in release mode
  - fail on non-dev dependency on `talkbank-parser-tests`
- Add `RELEASING.md` in each repo with exact command sequence.

### Phase 1: Foundation releases (prerequisites)
1. Release `tree-sitter-talkbank` (`npm` + `crates.io`).
2. Release `talkbank-chat` public crates (ordered by dependency graph).
3. Release `talkbank-clan`.

### Phase 2: Chatter CLI first public milestone
- Update `talkbank-chatter` to only semver deps.
- Publish `talkbank-cli`.
- Validate install path: `cargo install talkbank-cli --bin chatter`.
- Tag and publish GitHub release with changelog + minimal install docs.

### Phase 3: Batchalign3 immediate follow-up
- Update `batchalign3/pyo3` and the root `crates/` workspace to semver deps where release policy requires it.
- Build and publish Python package (`batchalign3`) with pinned Rust crate versions.
- Verify clean install in isolated environment without sibling checkouts.

## CI/Test Scenarios (must pass)

### Cross-repo installability
1. Fresh machine (no sibling repos):
   - `cargo install talkbank-cli --bin chatter` succeeds.
2. Fresh Python env:
   - `pip install batchalign3` (or project release command) succeeds without local path deps.

### Runtime smoke
1. `chatter validate <valid.cha>` works.
2. `chatter new-file` produces valid minimal CHAT file via non-test crate API.
3. `batchalign3` basic command path runs and loads Rust extension.

### Contract checks
1. No `path = "../talkbank-*"` in release manifests.
2. No production dependency on `talkbank-parser-tests`.
3. Dependency direction check passes (no forbidden upward/cyclic edges).

### Existing quality gates
- `tree-sitter-talkbank`: generate/test/query checks.
- `talkbank-chat`: parser equivalence/reference corpus gates.
- `talkbank-chatter`: Rust tests + CLI tests.
- `batchalign3`: Python + Rust test suites already defined.

## Assumptions and Defaults Chosen
- Compatibility with unpublished historical layouts is not required.
- Repos remain separate (5 repos total).
- `tree-sitter-talkbank` remains separate.
- `talkbank-clan` remains separate.
- `batchalign3` uses the current `pyo3/` + `crates/` layout.
- Primary `chatter` distribution is crates.io (`cargo install`).
- `batchalign3` consumes published crates.io dependencies (not vendored, not path-based) for release.
