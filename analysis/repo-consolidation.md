# Repository Consolidation Options

Assessed 2026-03-06. This document evaluates options for reducing the number of git repositories in the TalkBank workspace.

## Current State

8 git repositories, 28+ cross-repo path dependencies, 30+ Rust crates.

```
tree-sitter-talkbank/     grammar (JS/C)           1 crate
talkbank-chat/            core library (Rust)      10 crates
talkbank-chatter/         CLI + LSP + VS Code       4 crates
talkbank-clan/            CLAN commands              1 crate
batchalign3/              Python + Rust (PyO3)       8 crates (rust/ + rust-next/)
batchalign-hk-plugin/     Python plugin              0 crates
talkbank-private/         ops, deploy, docs          0 crates
```

### Dependency graph

```
tree-sitter-talkbank ─(1 path dep)─→ talkbank-chat
                                          │
                     ┌────(4 deps)────────┤
                     │                    ├────(8 deps)────→ talkbank-chatter
                     │                    │                        ↑
                talkbank-clan ──(1 dep)───────────────────────────┘
                                          │
                                          └───(~13 deps)──→ batchalign3/rust + rust-next
                                                                   ↑
                                              batchalign-hk-plugin (Python runtime)
```

### Key facts

- **Zero crates published to crates.io.** Distribution is via PyPI (batchalign3, batchalign3-cli) and VS Code Marketplace.
- **Path dependencies are purely local.** No external consumers depend on sibling directory layout.
- **talkbank-clan has exactly one consumer:** talkbank-chatter.
- **tree-sitter-talkbank has two consumers:** talkbank-chat and talkbank-chatter (highlight crate).
- **talkbank-chat has three consumers:** talkbank-chatter, talkbank-clan, batchalign3.
- **batchalign3's Rust crates are built by maturin** (PyO3 bridge) and cargo (rust-next server). Maturin expects Cargo.toml and pyproject.toml in the same repo.

### Pain points driving this analysis

1. **Two-commit changes.** Adding a field to `EvalConfig` (talkbank-clan) and wiring it in the CLI/LSP (talkbank-chatter) requires commits in two repos. This happens frequently during active development.
2. **Makefile coordination.** `make status`, `make test`, `make verify-all` exist solely to manage multi-repo friction.
3. **CLAUDE.md proliferation.** 37 CLAUDE.md files across repos, with duplicated Rust coding standards in each.
4. **CI fragmentation.** Each repo needs its own CI, but changes in talkbank-chat can break talkbank-chatter and talkbank-clan. No unified gate.

---

## Option A: Status Quo (7 active repos)

No changes.

### Pros
- Each repo has clear ownership boundary and independent git history
- Familiar — all tooling, CLAUDE.md, CI already configured
- Repos can evolve at different cadences
- Smaller `cargo check` / `cargo test` per repo

### Cons
- 28+ cross-repo path deps remain
- Two-commit tax for cross-boundary changes
- Makefile coordination overhead
- Duplicated coding standards across CLAUDE.md files
- No unified CI gate

### Verdict
Acceptable if cross-repo changes are rare. They are not — talkbank-clan ↔ talkbank-chatter changes happen almost every session.

---

## Option B: Fold talkbank-clan into talkbank-chatter (6 repos)

Move `talkbank-clan/` as `crates/talkbank-clan/` inside talkbank-chatter workspace.

```
tree-sitter-talkbank/     grammar                    1 crate
talkbank-chat/            core library              10 crates
talkbank-chatter/         CLI + LSP + VS Code + commands  5 crates  ← was 4
batchalign3/              Python + Rust              8 crates
batchalign-hk-plugin/     Python plugin
talkbank-private/         ops/deploy
```

### What moves
- `talkbank-clan/src/` → `talkbank-chatter/crates/talkbank-clan/src/`
- `talkbank-clan/Cargo.toml` → `talkbank-chatter/crates/talkbank-clan/Cargo.toml`
- `talkbank-clan/tests/` → `talkbank-chatter/crates/talkbank-clan/tests/`
- `talkbank-clan/book/` → `talkbank-chatter/book/clan/` (or `talkbank-chatter/crates/talkbank-clan/book/`)
- `talkbank-clan/CLAUDE.md` → absorbed into talkbank-chatter's CLAUDE.md

### Mechanical changes
- Add `"crates/talkbank-clan"` to talkbank-chatter `workspace.members`
- Change talkbank-clan path deps from `../talkbank-chat/crates/...` to `../../talkbank-chat/crates/...`
- Remove `talkbank-clan` from talkbank-chatter's `[dependencies]` path and use workspace dep
- Update meta-repo Makefile, .gitignore, CLAUDE.md references
- Archive talkbank-clan repo (or redirect)

### Pros
- Eliminates the most active pain point (clan ↔ chatter coordination)
- Single commit for config struct changes + CLI wiring + LSP integration
- One `cargo test` covers commands + CLI + LSP
- Unified CI for the "chatter tool" product
- talkbank-clan remains a separate crate (publishable to crates.io if ever needed)
- Low effort — one directory move + Cargo.toml edits

### Cons
- talkbank-chatter repo grows (~15K lines Rust + 102-page book)
- Golden tests for CLAN commands now live alongside CLI tests
- Git history for clan commands starts fresh in the new repo (old history stays in archived repo)

### Effort
Low. Half a day, mostly mechanical.

---

## Option C: Unify Rust side (4 repos)

Merge tree-sitter-talkbank + talkbank-chat + talkbank-clan + talkbank-chatter into one Rust workspace.

```
talkbank/                 grammar + core + commands + CLI/LSP   16 crates
batchalign3/              Python + Rust (PyO3 + server)          8 crates
batchalign-hk-plugin/     Python plugin
talkbank-private/         ops/deploy
```

### What the unified workspace looks like

```
talkbank/
├── grammar/                  ← was tree-sitter-talkbank/
│   ├── grammar.js
│   ├── src/parser.c
│   └── Cargo.toml            (tree-sitter-talkbank crate)
├── crates/
│   ├── talkbank-errors/
│   ├── talkbank-model/
│   ├── talkbank-tree-sitter-parser/
│   ├── talkbank-direct-parser/
│   ├── talkbank-transform/
│   ├── talkbank-pipeline/
│   ├── talkbank-json/
│   ├── talkbank-derive/
│   ├── talkbank-parser-api/
│   ├── talkbank-parser-tests/
│   ├── talkbank-clan/
│   ├── talkbank-cli/
│   ├── talkbank-lsp/
│   ├── talkbank-highlight/
│   └── send2clan-sys/
├── spec/                     ← was talkbank-chat/spec/
├── corpus/                   ← was talkbank-chat/corpus/
├── book/                     ← merged books
├── vscode/                   ← VS Code extension
├── fuzz/                     ← fuzzing workspace
└── Cargo.toml                (workspace root)
```

### Pros
- Eliminates ~20 of 28 cross-repo path deps
- Grammar change → parser fix → model update → command fix → CLI wire-up: one commit
- One `cargo test` / `cargo clippy` / `cargo fmt` for everything
- One CI pipeline with unified gate
- Spec-driven test generation writes directly into the same repo
- Single CLAUDE.md (no more duplicated coding standards)
- The meta-repo Makefile becomes unnecessary for Rust

### Cons
- Large workspace (~40K lines Rust + grammar + spec + corpus + 2 books)
- `cargo check` touches more code on every change (workspace-level feature unification)
- tree-sitter-talkbank is published to npm — npm publishing workflow needs to work from a subdirectory
- Merging 4 git histories is complex (or we accept history reset)
- batchalign3 still has 8+ cross-repo deps pointing at this unified repo
- Different concerns (grammar maintenance vs CLI features) share one issue tracker

### Effort
Medium. 1-2 days for the merge, plus CI/CD reconfiguration. npm publishing from a subdirectory needs testing.

### Risk: batchalign3 path deps
batchalign3 currently points at `../../talkbank-chat/crates/...`. After unification, paths change to `../../talkbank/crates/...` (or `../../talkbank-core/crates/...` depending on naming). This is a one-time find-and-replace but requires a coordinated commit in batchalign3.

---

## Option D: Monorepo without batchalign (3 repos)

Same as Option C but using the meta-repo (`~/talkbank/`) directly as the unified Rust workspace.

```
~/talkbank/               grammar + core + commands + CLI/LSP   16 crates
  batchalign3/            Python + Rust (subdir, own workspace)  8 crates
  batchalign-hk-plugin/   Python plugin (subdir)
  talkbank-private/        ops/deploy (subdir)
```

### Key difference from Option C
Instead of creating a new repo, promote the existing meta-repo. Sub-repos become subdirectories tracked by the parent git. batchalign3 remains its own git repo but lives as a gitignored subdirectory (as today).

### Pros
- Same as Option C
- No new repo needed — directory layout barely changes
- batchalign3 path deps don't change (still `../../talkbank-chat/crates/...` → adjust to `../crates/...`)

### Cons
- The meta-repo currently tracks coordination files (Makefile, CLAUDE.md, analysis/). Mixing these with 16 Rust crates requires reorganization.
- Unusual git structure: some subdirectories are gitignored independent repos (batchalign3, talkbank-private), others are tracked Rust crates.
- `cargo workspace` at the repo root means `cargo test` in the meta-repo runs all Rust tests but ignores batchalign3's Rust tests (separate workspace).

### Verdict
Mechanically possible but conceptually messy. Option C (fresh unified repo) is cleaner.

---

## Option E: Full monorepo (2 repos)

Everything in one repo except talkbank-private.

```
~/talkbank/               grammar + core + commands + CLI/LSP + batchalign (Python+Rust)
  talkbank-private/        ops/deploy (gitignored subdir)
```

### Hard constraint: maturin
batchalign3's PyO3 bridge (`rust/`) is built by maturin, which expects `Cargo.toml` and `pyproject.toml` to coexist. Maturin discovers the Rust crate from the Python package root. Moving the Rust crates out of batchalign3 breaks `uv run maturin develop`.

Workaround: maturin supports `manifest-path` in pyproject.toml, pointing to a Cargo.toml elsewhere. This is supported but less common and may interact poorly with workspace resolution.

### Pros
- Zero cross-repo path deps
- Truly unified versioning and CI
- One git log for all project history

### Cons
- Maturin integration requires careful configuration
- Mixes Python (uv, pytest, mypy) and Rust (cargo, nextest) build systems in one CI
- batchalign3 has genuinely different release cadence (Python package releases, model updates)
- batchalign-hk-plugin is a separate PyPI package — must remain independently publishable
- Very large repo with mixed concerns
- Git history noise: Rust crate changes and Python ML pipeline changes interleaved

### Verdict
Not recommended. The batchalign boundary is real — different build system, different release cadence, different audience. Forcing them together creates more friction than it eliminates.

---

## Comparison Matrix

| Criterion | A (status quo) | B (fold clan) | C (unify Rust) | D (meta monorepo) | E (full mono) |
|-----------|:-:|:-:|:-:|:-:|:-:|
| Cross-repo deps | 28 | 23 | 8 | 8 | 0 |
| Two-commit tax | High | Low | None | None | None |
| Effort | 0 | Low | Medium | Medium | High |
| Risk | 0 | Low | Medium | Medium | High |
| Build system complexity | Simple | Simple | Simple | Messy | Complex |
| CI unification | No | Partial | Rust: Yes | Rust: Yes | Full |
| Preserves git history | Yes | Partial | No* | No* | No* |

*Git history can be preserved with `git subtree` or filter-repo, at the cost of complexity.

---

## Recommendation

**Do Option B now.** Fold talkbank-clan into talkbank-chatter. This addresses the most active pain point with minimal risk and effort. talkbank-clan remains a distinct crate within the workspace.

**Evaluate Option C later.** After Option B stabilizes, assess whether the tree-sitter-talkbank and talkbank-chat boundaries cause enough friction to justify the larger merge. Grammar changes are infrequent, so this boundary is less costly than the clan ↔ chatter one.

**Keep batchalign3 separate.** The Python/maturin build coupling and different release cadence make it a natural boundary.
