# Repository and Crate Consolidation

Assessed 2026-03-06. Revised 2026-03-08 with artifact-driven analysis, full crate audit, and stakeholder impact.

## Decisions Made

- **4 repos** (down from 8): talkbank-tools (unified workspace including grammar), batchalign3, batchalign-hk-plugin, talkbank-private
- **Fresh repos** with fresh git history; old repos archived read-only
- **`chatter lsp`** subcommand (single binary, no separate LSP binary)
- **No `talkbank` facade crate** — external Rust devs import the crate they need directly
- **tree-sitter-talkbank folded into talkbank-tools** (`grammar/` subdirectory) — eliminates cross-repo spec→grammar test generation. Can be extracted later if tree-sitter-grammars org reopens submissions.
- **talkbank-direct-parser** keeps its name — it's a full alternative CHAT parser, not just MOR
- **GitHub Actions** for CI
- **talkbank-tools unified workspace: 11 crates** (down from 17 across 5 repos, including tree-sitter-talkbank)
- **batchalign3: 3 crates** (down from 9): batchalign-chat-ops (shared CHAT manipulation), batchalign-app (everything server-side), batchalign-pyo3 (Python bridge, Python package name: `batchalign_native`)
- **Total: 14 crates** (down from 26)
- **Repo name: `talkbank-tools`**
- **batchalign3 directory renames**: `rust/` → `pyo3/`, `rust-next/` → removed (crates move to `crates/`)
- **batchalign-chat-ops loses batchalign-types dep** — two trivial lang converter functions move to batchalign-app
- **Docs**: mdBook in talkbank-tools (`book/`); useful analysis material migrates into books; transient analysis archived to talkbank-private
- **CLAUDE.md**: 37 files → ~3 (talkbank-tools root, talkbank-tools vscode/, batchalign3). No information lost — all unique guidance preserved.

## Execution Status (2026-03-08)

| Phase | Status | Notes |
|-------|--------|-------|
| Phase 1: talkbank-chat internal merges | Complete | errors→model, parser-api→model, pipeline→model, json→transform |
| Phase 1.5: talkbank-chatter highlight merge | Complete | highlight→lsp |
| Phase 2: Cross-repo consumer updates | Complete | chatter, clan, batchalign3 all updated |
| Phase 3: batchalign3 internal merges | Complete | 9→3 crates, rust/→pyo3/, rust-next/→crates/ |
| Phase 4: Create talkbank-tools | Complete | 10 workspace members, grammar intra-repo, parser renamed |
| Phase 5: Update batchalign3 path deps | Complete | All paths point to talkbank-tools/crates/ |
| Phase 6.1: CLAUDE.md consolidation | Complete | 16→5 (talkbank-tools), 10→3 (batchalign3) |
| Phase 6.2: mdBook updates | Complete | Existing book updated for new structure |
| Phase 6.3: CI consolidation | Complete | Unified CI + release workflows |
| Phase 7: Archive old repos | Pending | tree-sitter-talkbank, talkbank-chat, talkbank-chatter, talkbank-clan |

Test verification: talkbank-tools 2253 passed (120 skipped, 9 pre-existing legacy_mor failures); batchalign3 967 all passing.

## Current State

### Repositories (8)

```
tree-sitter-talkbank/     grammar (JS/C)             1 crate
talkbank-chat/            core library (Rust)        10 crates
talkbank-chatter/         CLI + LSP + VS Code         4 crates
talkbank-clan/            CLAN commands               1 crate
batchalign3/              Python + Rust (PyO3)        9 crates  (rust/ + rust-next/)
batchalign-hk-plugin/     Python plugin               0 crates
talkbank-private/         ops, deploy, docs           0 crates
~/talkbank/               meta-repo (coordination)    0 crates
```

26 Rust crates, 6 Rust workspaces, 8 git repositories.

### Full crate inventory (26 crates)

**talkbank-chat (10 crates, ~91K lines):**

| Crate | Lines | What it does | Consumers |
|-------|-------|-------------|-----------|
| talkbank-errors | 4,568 | Error types, ErrorSink, Span, diagnostics | Everything |
| talkbank-derive | 1,480 | Proc macros: SemanticEq, SpanShift, etc. | model, errors |
| talkbank-model | 42,813 | ChatFile AST, validation, alignment | Everything |
| talkbank-parser-api | 1,001 | ChatParser trait (37 methods), ParseOutcome | parsers, transform, batchalign |
| talkbank-pipeline | 165 | ParseValidateOptions config | transform, batchalign |
| talkbank-json | 200 | Schema-validated JSON serialization | transform, CLI |
| talkbank-tree-sitter-parser | 21,080 | Canonical parser (tree-sitter → model) | transform, batchalign |
| talkbank-direct-parser | 8,826 | Full alternative parser (chumsky → model) | batchalign-core, parser-tests |
| talkbank-transform | 3,766 | Orchestration: parse+validate, caching, roundtrip | CLI, LSP, batchalign |
| talkbank-parser-tests | 7,314 | Test harness: golden words, 73-file equivalence | dev-only |

**talkbank-chatter (4 crates, ~21K lines):**

| Crate | Lines | What it does |
|-------|-------|-------------|
| talkbank-cli | 8,210 | `chatter` binary (13+ subcommands, 33+ CLAN commands) |
| talkbank-lsp | 11,816 | LSP server binary (diagnostics, completions, hover) |
| talkbank-highlight | 328 | Semantic tokens from tree-sitter CST |
| send2clan-sys | 752 | C library: send files to CLAN desktop app (Apple Events/WM_APP) |

**talkbank-clan (1 crate, ~28K lines):**

| Crate | Lines | What it does |
|-------|-------|-------------|
| talkbank-clan | 28,234 | 33+ CLAN analysis commands + framework (463 tests) |

**tree-sitter-talkbank (1 crate, 130 lines Rust binding):**

Grammar (grammar.js → parser.c). Published to npm.

**batchalign3/rust (2 crates, ~6K lines):**

| Crate | Lines | What it does |
|-------|-------|-------------|
| batchalign-core | 5,749 | PyO3 bridge to Python |
| talkbank-revai | ~300 | Rev.AI client (optional feature of core) |

**batchalign3/rust-next (7 crates, ~47K lines):**

| Crate | Lines | What it does |
|-------|-------|-------------|
| batchalign-types | 3,378 | Wire format types (zero workspace deps) |
| batchalign-cache | 909 | Caching layer |
| batchalign-chat-ops | 17,548 | CHAT manipulation, FA injection, word extraction |
| batchalign-worker | 1,730 | Worker pool management, IPC protocol |
| batchalign-server | 14,041 | HTTP server, job dispatch, orchestration |
| batchalign-cli | 9,249 | CLI arg parsing and dispatch |
| batchalign-bin | 240 | `fn main()` entry point |

### Pain points

1. **Two-commit changes.** Adding a field to `EvalConfig` (talkbank-clan) and wiring it in the CLI (talkbank-chatter) requires commits in two repos. Happens almost every session.
2. **Makefile coordination.** `make status`, `make test`, `make verify-all` exist solely to manage multi-repo friction.
3. **CLAUDE.md proliferation.** 37 CLAUDE.md files with duplicated Rust coding standards.
4. **CI fragmentation.** Changes in talkbank-chat can break talkbank-chatter and talkbank-clan with no unified gate.
5. **Crate sprawl.** Several crates under 200 lines. `talkbank-pipeline` is 165 lines, `talkbank-json` is 200 lines, `talkbank-highlight` is 328 lines, `batchalign-bin` is 240 lines. These are artificial boundaries.

---

## Public Artifacts and Stakeholders

### Artifacts

| Artifact | Channel | What it provides |
|----------|---------|-----------------|
| **`chatter` CLI** | GitHub releases, homebrew, cargo install, (maybe PyPI) | `chatter validate`, `chatter clan`, `chatter lsp`, normalize, json, watch, lint |
| **VS Code extension** | VS Code Marketplace | One-click editor integration (bundles `chatter lsp`) |
| **`tree-sitter-talkbank`** | npm, crates.io | CHAT grammar for any tree-sitter editor |
| **`batchalign3`** | PyPI | ASR, forced alignment, morphosyntax pipeline + CLI |

### Audiences

**Chatter users (primary).** Former users of the Java CLAN tools. Corpus maintainers, clinical researchers, phonologists, language acquisition researchers. They want `chatter validate` and `chatter clan mlu/freq/eval/dss`. They care about specific tiers — one collaborator only uses %pho and %mod, others focus on %mor/%gra. These users may not have Python or any development toolchain. They need **standalone binary downloads**: GitHub releases with pre-built binaries for macOS/Linux/Windows, homebrew on macOS, possibly `cargo install` for Rust developers.

**VS Code extension users.** Researchers who edit CHAT files. They install from Marketplace and expect it to just work. The extension bundles `chatter lsp`.

**Editor plugin developers.** People adding CHAT syntax highlighting to Neovim, Helix, Zed, etc. They consume `tree-sitter-talkbank` from npm or source. Small audience but real — tree-sitter grammars have broad editor reach.

**External Rust developers.** People building their own CHAT analysis tools in Rust. They `cargo add talkbank-model` or `cargo add talkbank-transform` on crates.io. Currently impossible — nothing is published.

**Batchalign users.** Python-native ML researchers. They install from PyPI. **Completely different audience from chatter users** — different toolchain, different use cases, different release cadence.

### Distribution plan for `chatter`

| Channel | Command | Audience |
|---------|---------|----------|
| **GitHub releases** | Download pre-built binary | Primary — everyone |
| **Homebrew** | `brew install talkbank/tap/chatter` | macOS users |
| **cargo install** | `cargo install chatter` | Rust developers |
| **PyPI** (optional) | `pip install talkbank-cli` | Users who already have Python |

---

## Crate Consolidation: Unified Workspace

### Merges (16 → 10 crates)

| Absorbed | Into | Rationale |
|----------|------|-----------|
| `talkbank-errors` (4,568 lines) | `talkbank-model` | Every consumer of errors already depends on model. Zero dependency isolation benefit. |
| `talkbank-parser-api` (1,001 lines) | `talkbank-model` | ChatParser trait + ParseOutcome belong with the types they parse into. Parsers already depend on model. |
| `talkbank-pipeline` (165 lines) | `talkbank-model` | 165 lines of config. Every consumer already depends on model. |
| `talkbank-json` (200 lines) | `talkbank-transform` | JSON schema validation is part of the serialization pipeline. 200 lines. |
| `talkbank-highlight` (328 lines) | `talkbank-lsp` | Only consumer is LSP. 328 lines. |

### What stays separate and why

| Crate | Lines | Why separate |
|-------|-------|-------------|
| `talkbank-derive` | 1,480 | Rust requires proc-macro crates in separate compilation units. Non-negotiable. |
| `talkbank-model` | ~49K after merges | Core types, errors, validation, alignment, parser-api. Lightweight — no tree-sitter, no tokio, no sqlx. batchalign-chat-ops depends on just this. |
| `talkbank-parser` | 21,080 | Depends on tree-sitter C code. Different compilation profile from pure-Rust model. Renamed from talkbank-tree-sitter-parser. |
| `talkbank-direct-parser` | 8,826 | Full alternative CHAT parser (chumsky combinators). Different deps from tree-sitter parser. Merging would force both dep trees on every consumer. |
| `talkbank-transform` | ~4K after merge | Heavy deps (sqlx, tokio, blake3). Orchestration layer above parsers. |
| `talkbank-clan` | 28,234 | Large, distinct concern. Optional for consumers who don't need analysis. |
| `talkbank-lsp` | ~12K after merge | Library: LSP protocol + highlight. Only consumer is chatter, but separating keeps the binary crate focused on arg parsing and wiring. |
| `send2clan-sys` | 752 | C FFI with its own build.rs. Platform-specific (macOS/Windows). Mixing C compilation into chatter's build.rs would be messy. |
| `chatter` | 8,210 | Binary crate. |
| `talkbank-parser-tests` | 7,314 | Dev-only test infrastructure. Needs both parsers. |

### Result: 10 crates in unified workspace

```
Required separate (Rust constraint):
  talkbank-derive              proc-macro

Core libraries:
  talkbank-model               types + errors + parser-api + pipeline config + validation + alignment (~49K lines)
  talkbank-parser              canonical tree-sitter parser (21K lines)
  talkbank-direct-parser       full alternative parser, chumsky combinators (9K lines)
  talkbank-transform           orchestration + json + caching (~4K lines)

Application libraries:
  talkbank-clan                33+ CLAN analysis commands + framework (28K lines)
  talkbank-lsp                 LSP protocol + semantic tokens + highlight (~12K lines)

Platform/FFI:
  send2clan-sys                C library for CLAN desktop integration (752 lines)

Binaries:
  chatter                      CLI + LSP subcommand, single binary (8K lines)

Dev-only:
  talkbank-parser-tests        test infrastructure (7K lines)
```

Note: `tree-sitter-talkbank` is in its own standalone repo.

---

## Crate Consolidation: batchalign3

### 9 → 3 crates

batchalign3's Rust code does two things: expose CHAT parsing to Python (PyO3 bridge), and run a standalone HTTP server that dispatches NLP jobs. The shared CHAT manipulation library is used by both.

**Current 9 crates:** batchalign-types, batchalign-cache, batchalign-chat-ops, batchalign-worker, batchalign-server, batchalign-cli, batchalign-bin (in rust-next/), batchalign-core, talkbank-revai (in rust/).

**Target 3 crates:**

| Crate | What | Why separate |
|-------|------|-------------|
| `batchalign-chat-ops` | Pure CHAT manipulation: extract, inject, retokenize, mor/gra, FA, ASR postprocess. 17.5K lines. | Shared by both pyo3/ and app. Depends only on talkbank crates — zero batchalign deps. |
| `batchalign-app` | Types + worker pool + cache + HTTP server + CLI + binary. ~30K lines after merges. | The entire application. One binary (`batchalign3`). |
| `batchalign-pyo3` | PyO3 bridge. Python package name: `batchalign_native`. ~6K lines after absorbing revai. | Different build system (maturin). Must be separate crate. |

**What gets absorbed:**

| Absorbed | Into | Rationale |
|----------|------|-----------|
| `batchalign-types` (3,378 lines) | `batchalign-app` | chat-ops only depended on types for 2 trivial lang converter functions. Moving those into app breaks the circular dep, freeing types to merge into app. |
| `batchalign-cache` (909 lines) | `batchalign-app` | Only used by server. No cross-crate consumers. |
| `batchalign-worker` (1,730 lines) | `batchalign-app` | All consumers already depend on server. |
| `batchalign-server` (14,041 lines) | `batchalign-app` | Core of the application. |
| `batchalign-cli` (9,249 lines) | `batchalign-app` | CLI is the application frontend. |
| `batchalign-bin` (240 lines) | `batchalign-app` | Just `fn main()`. |
| `talkbank-revai` (~300 lines) | `batchalign-pyo3` | Only consumer. Module behind `#[cfg(feature = "native-revai")]`. |

**Key enabler:** batchalign-chat-ops's dependency on batchalign-types was only two functions (`lang3_to_model`, `lang_model_to_wire`). Moving those into batchalign-app eliminates the dependency, making chat-ops a pure talkbank-only library with zero batchalign deps. This breaks the circular dep that forced types to be separate.

### Directory structure after consolidation

```
batchalign3/
├── crates/
│   ├── batchalign-chat-ops/      shared CHAT manipulation (depends only on talkbank crates)
│   └── batchalign-app/           types + worker + cache + HTTP + CLI + binary
└── pyo3/
    └── batchalign-pyo3/          Python bridge (package name: batchalign_native)
```

Renames: `rust/` → `pyo3/`, `rust-next/` → removed (crates move to `crates/` and `crates/batchalign-app/`).

---

## Total Crate Count

| Scope | Before | After | Merges |
|-------|--------|-------|--------|
| talkbank-tools (5 repos → 1) | 17 | 11 | errors→model, parser-api→model, pipeline→model, json→transform, highlight→lsp; tree-sitter-talkbank folded in |
| batchalign3 | 9 | 3 | types→app, cache→app, worker→app, server→app, cli→app, bin→app, revai→pyo3 |
| **Total** | **26** | **14** | **12 merges** |

Each of the 14 remaining crates exists for one of these reasons:
- **Rust constraint** — proc-macros must be separate (talkbank-derive)
- **Dep isolation** — different compilation profiles prevent merging (model vs parser vs transform; tree-sitter vs chumsky)
- **Genuine size** — large enough to warrant its own module boundary (clan 28K, chat-ops 18K, parser 21K, model 49K, app 30K, lsp 12K)
- **Build isolation** — C FFI (send2clan-sys), PyO3 (batchalign-pyo3)
- **Shared by multiple workspaces** — batchalign-chat-ops (used by both pyo3/ and app)
- **Dev-only** — test infrastructure (talkbank-parser-tests)
- **Standalone ecosystem** — tree-sitter-talkbank (npm, editor plugins)

No crate under 750 lines survives. No crate exists solely as a "convenience wrapper" or "maybe someone will want just this."

---

## Repo Structure: Target State (5 repos)

### talkbank-tools (unified Rust workspace)

```
talkbank-tools/
├── Cargo.toml                    workspace root
├── grammar/                      tree-sitter-talkbank (npm publishable from subdirectory)
│   ├── grammar.js
│   ├── package.json              own semver for npm
│   ├── src/parser.c              generated
│   └── test/corpus/              generated by make test-gen
├── crates/
│   ├── talkbank-derive/          proc-macro
│   ├── tree-sitter-talkbank/     grammar Rust binding (thin wrapper around grammar/)
│   ├── talkbank-model/           core types + errors + validation + alignment + parser-api
│   ├── talkbank-parser/          tree-sitter parser (CST → AST)
│   ├── talkbank-direct-parser/   full alternative parser (chumsky)
│   ├── talkbank-transform/       orchestration + json + caching
│   ├── talkbank-clan/            CLAN analysis commands
│   ├── talkbank-lsp/             LSP protocol + highlight
│   ├── send2clan-sys/            C library (CLAN desktop integration)
│   ├── chatter/                  single binary (CLI + LSP subcommand)
│   └── talkbank-parser-tests/    dev-only test infrastructure
├── spec/                         CHAT specification (source of truth)
├── corpus/                       reference corpus (73 files)
├── vscode/                       VS Code extension (TypeScript)
├── book/                         documentation (mdBook)
└── .github/workflows/            GitHub Actions CI
```

### All repos

```
talkbank-tools/               unified workspace + grammar + VS Code        11 crates
batchalign3/                  Python + Rust ML pipeline (PyPI)              3 crates
batchalign-hk-plugin/         Python HK plugin (PyPI)                       0 crates
talkbank-private/             ops, deploy, credentials                      0 crates
```

**Archived (read-only):** tree-sitter-talkbank, talkbank-chat, talkbank-chatter, talkbank-clan.

**Retired:** `~/talkbank/` meta-repo (its coordination role becomes unnecessary).

### What gets published

| Target | What |
|--------|------|
| **crates.io** | talkbank-model, talkbank-parser, talkbank-direct-parser, talkbank-transform, talkbank-clan, talkbank-derive (transitive dep) |
| **crates.io (binary)** | chatter |
| **npm** | tree-sitter-talkbank |
| **VS Code Marketplace** | talkbank-chatter extension |
| **GitHub Releases** | chatter binaries (macOS/Linux/Windows) |
| **Homebrew** | talkbank/tap/chatter |

External Rust devs import the layer they need — no facade:

```toml
# Types only (what batchalign-chat-ops does)
talkbank-model = "1.0"

# Parse + validate CHAT files
talkbank-transform = "1.0"

# Run CLAN analysis commands
talkbank-clan = "1.0"
```

### Versioning

All workspace crates share **one version**, bumped together. Tightly coupled — a model change cascades everywhere.

Exception: `tree-sitter-talkbank` has independent npm versioning (grammar evolves at a different pace from the Rust crates). The npm package is published from `grammar/` subdirectory with its own `package.json` and semver.

---

## Impact on batchalign3

The talkbank crate merges simplify batchalign's dependency list:

**batchalign-chat-ops:**
- Before: talkbank-errors, talkbank-model, talkbank-parser-api, talkbank-direct-parser, talkbank-tree-sitter-parser, batchalign-types
- After: **talkbank-model, talkbank-direct-parser, talkbank-parser** (3 deps instead of 6, zero batchalign deps)

**batchalign-pyo3 (was batchalign-core):**
- Before: talkbank-errors, talkbank-model, talkbank-parser-api, talkbank-pipeline, talkbank-transform, talkbank-direct-parser, talkbank-tree-sitter-parser, batchalign-chat-ops, talkbank-revai
- After: **talkbank-model, talkbank-transform, talkbank-direct-parser, talkbank-parser, batchalign-chat-ops** (5 deps instead of 9, revai absorbed)

Path deps change once: `../../talkbank-chat/crates/X` → `../../talkbank-tools/crates/X`.

---

## Open Questions

None. All decisions have been made. Ready for execution.
