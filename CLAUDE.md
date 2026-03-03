# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

TalkBank development workspace — parent directory for all TalkBank repositories. TalkBank provides tools for linguistic research on conversational data in CHAT format (Codes for the Human Analysis of Transcripts).

Data flows: **spec** (source of truth) → **grammar** (tree-sitter) → **Rust** (parsers, model, validation) → **applications** (CLI, LSP, VS Code, Python pipeline).

## Repositories

| Directory | Purpose | Language |
|-----------|---------|----------|
| `tree-sitter-talkbank/` | Tree-sitter grammar for CHAT format (`grammar.js`) | JavaScript/C |
| `talkbank-chat/` | CHAT spec, core Rust crates (parsing, model, validation, transform) | Rust |
| `talkbank-chatter/` | CLI (`chatter`), LSP server, VS Code extension | Rust/TypeScript |
| `talkbank-clan/` | CLAN analysis commands reimplemented in Rust (FREQ, MLU, MLT, etc.) | Rust |
| `batchalign3/` | ASR, forced alignment, morphosyntax pipeline | Python/Rust (PyO3) |
| `talkbank-private/` | Internal archive, CLAUDE.md templates, copilot instructions | |

## Cross-Repo Commands (from this directory)

```bash
make status   # Git status across all repos
make check    # Cargo check all Rust repos
make test     # Run tests across repos
make pull     # Pull all repos
```

## Per-Repo Commands

### talkbank-chat (core libraries)
```bash
cd talkbank-chat && make build          # Build everything
cd talkbank-chat && make test           # Run all tests (nextest + parser-tests + doctests)
cd talkbank-chat && make verify         # Pre-merge gates (G0–G7)
cd talkbank-chat && make test-gen       # Regenerate tests from specs (always run after spec changes)
cd talkbank-chat && cargo nextest run   # Unit/integration tests (all crates including parser-tests)
cd talkbank-chat && cargo clippy --all-targets -- -D warnings  # Periodic lint check (not required on every change)
```

### talkbank-chatter (CLI/LSP/VS Code)
```bash
cd talkbank-chatter && cargo nextest run   # 113 tests
cd talkbank-chatter && cargo check --all-targets
cd talkbank-chatter && cargo clippy --all-targets -- -D warnings  # Periodic lint check
# VS Code extension:
cd talkbank-chatter/vscode && npm install && npm run compile
```

### tree-sitter-talkbank (grammar)
```bash
cd tree-sitter-talkbank && tree-sitter generate
cd tree-sitter-talkbank && tree-sitter test    # 232 tests
cd tree-sitter-talkbank && tree-sitter parse path/to/file.cha
```

### talkbank-clan (analysis commands)
```bash
cd talkbank-clan && cargo nextest run   # 129 tests
cd talkbank-clan && cargo nextest run -- --test clan_golden  # Golden snapshot tests
```

### batchalign3 (Python + Rust pipeline)
```bash
cd batchalign3 && uv sync --extra dev         # Install deps (never use pip)
cd batchalign3 && uv run pytest               # Python tests (560 default, 9 golden)
cd batchalign3 && uv run pytest batchalign/tests/path/test_file.py::test_name -v  # Single test
cd batchalign3 && uv run mypy batchalign/pipelines/
cd batchalign3 && uv run batchalign3 --help   # CLI
# Rebuild Rust extension after Rust changes:
cd batchalign3 && uv run maturin develop
cd batchalign3 && cargo test --manifest-path rust/Cargo.toml  # Rust tests (use cargo test, not nextest — custom harnesses in batchalign-core)
# Build wheels for deployment (three-wheel system: Python + Rust×2):
cd batchalign3 && uv build --wheel                                            # Pure Python (version-agnostic)
cd batchalign3 && uv run maturin build --release -i python3.12                # Rust core for 3.12 (clients)
cd batchalign3 && uv run maturin build --release -i $(uv python find 3.14t)  # Rust core for 3.14t (server)
# Deploy: bash talkbank-private/batchalign/scripts/deploy_server.sh (Net, 3.14t) / deploy_clients.sh (lab, 3.12)
```

## Architecture

### Crate Dependency Flow (Rust)

```
talkbank-errors              ← Error types, ErrorSink trait, Span
    ↓
talkbank-model               ← Data model (ChatFile AST), validation, alignment
    ↓
talkbank-parser-api          ← ChatParser trait abstraction
    ↓
talkbank-tree-sitter-parser  ← Canonical parser (tree-sitter CST → model)
talkbank-direct-parser       ← Experimental parser (chumsky combinators)
    ↓
talkbank-transform           ← Pipelines: parse+validate, CHAT↔JSON roundtrip, caching
    ↓
talkbank-cli (chatter)       ← CLI tool (in talkbank-chatter repo)
talkbank-lsp                 ← Language server (in talkbank-chatter repo)
talkbank-clan                ← CLAN analysis commands (separate repo)
batchalign-core              ← PyO3 bridge to Python (in batchalign3 repo)
```

Supporting crates: `talkbank-derive` (proc macros), `talkbank-json` (schema validation), `talkbank-pipeline` (config types), `talkbank-highlight` (syntax highlighting), `talkbank-parser-tests` (equivalence tests), `send2clan-sys` (FFI to CLAN).

### JSON Schema

`talkbank-chat/schema/chat-file.schema.json` is a JSON Schema (2020-12 dialect) auto-generated from `talkbank-model` types via schemars. It describes the full `ChatFile` AST as JSON. Canonical URL: `https://talkbank.org/schemas/v0.1/chat-file.json`. Regenerate after model changes: `cd talkbank-chat && cargo test --test generate_schema`. The `chatter schema` command prints the schema to stdout; `chatter schema --url` prints just the URL. See `talkbank-chat/book/src/integrating/json-schema.md` for the full guide.

### Cross-Repo Path Dependencies

All Rust repos use local path dependencies. The expected layout is:

```
~/talkbank/
├── tree-sitter-talkbank/       # Grammar (referenced by talkbank-chat and talkbank-chatter)
├── talkbank-chat/              # Core crates (referenced by all other Rust repos)
│   └── crates/                 # talkbank-errors, talkbank-model, talkbank-transform, etc.
├── talkbank-chatter/           # CLI, LSP (depends on talkbank-chat crates + tree-sitter-talkbank)
├── talkbank-clan/              # Analysis (depends on talkbank-chat crates)
└── batchalign3/
    └── rust/crates/            # batchalign-core (path deps to talkbank-chat for shared crates)
```

### Two Cargo Workspaces in talkbank-chat

1. **Root workspace** (`talkbank-chat/Cargo.toml`) — all Rust crates
2. **Spec tools** (`talkbank-chat/spec/tools/Cargo.toml`) — generators for tests/docs from specs

Always `cd spec/tools` before running cargo commands for spec tooling.

### Spec-Driven Development

Specs in `talkbank-chat/spec/` are the **authoritative source of truth** for CHAT format:
- `spec/constructs/` — valid CHAT examples with expected parse trees
- `spec/errors/` — invalid CHAT examples with expected error codes and metadata
- `spec/symbols/` — shared symbol registry (JSON → Rust/JS generators)
- `spec/tools/` — Rust binaries that generate tree-sitter corpus tests, Rust tests, error docs

**Error spec metadata fields** (in `spec/errors/*.md`):
- `Layer: parser` — error caught by `parse_chat_file()` (returns `Err`)
- `Layer: validation` — error caught by `validate_with_alignment()` after successful parse
- `Status: not_implemented` — generates `#[ignore]` tests (validation not yet coded)
- `Expected Error Codes` — per-example override of the spec's error code

**Workflow after any spec change:**
```bash
cd talkbank-chat && make test-gen   # Regenerates: tree-sitter corpus, Rust tests, error docs
cd talkbank-chat && make verify     # Pre-merge gates (G0–G7)
```

**Legacy error corpus** (`talkbank-chat/tests/error_corpus/`): Supplementary test fixtures. The `expectations.json` manifest maps `.cha` files to expected outcomes. Files not in the manifest are skipped. Spec-generated tests are the canonical coverage mechanism.

### batchalign3 Rust Dependencies

`batchalign3/rust/crates/batchalign-core/` is the only local crate. All talkbank-chat crates (`talkbank-model`, `talkbank-direct-parser`, etc.) are referenced via path dependencies to `../../talkbank-chat/crates/`. Changes to shared crates in talkbank-chat are picked up automatically on rebuild.

### Two-Parser Strategy

1. **Tree-sitter parser** (canonical): GLR-based, error-recovering, produces CST. Used by LSP and CLI.
2. **Direct parser** (experimental): chumsky combinators, fail-fast. Used for batch processing of well-formed input.

Both must agree on the 73-file reference corpus at `talkbank-chat/corpus/reference/`.

## Critical Policies

### Reference Corpus
`talkbank-chat/corpus/reference/` (73 files) must pass parser equivalence at 100%. If a grammar/parser change breaks even one file, revert immediately.

### Grammar Change Workflow
`parser.c` is generated from `grammar.js` — never edit it directly. After any `grammar.js` change:
1. `tree-sitter generate` (mandatory, including after reverts)
2. `tree-sitter test`
3. `cargo test -p talkbank-tree-sitter-parser && cargo test -p talkbank-parser-tests`
4. Verify reference corpus

### CHAT Handling — No Text Hacking
All CHAT parsing and serialization must go through AST manipulation (Rust crates or `batchalign_core`), never ad-hoc string/regex manipulation. This applies to both Rust and Python code.

### Generated Files
Never hand-edit generated artifacts (`parser.c`, `tree-sitter-talkbank/test/corpus/`, generated Rust tests). Regenerate from their source inputs.

### Cache Policy
The validation cache (`~/.cache/talkbank-utils/talkbank-cache.db`) contains results for 95,000+ files. Never delete it. Use `--force` to refresh specific paths.

### Error Code Testing
All error code tests flow through `spec/errors/`. Every error code must have a corresponding spec file. Test fixtures are generated via `make test-gen` — never hand-written.

## Coding Standards

### Rust
- Edition 2024
- `cargo fmt` before committing
- `cargo clippy` periodically (not required on every change; run when doing dedicated lint passes)
- Library code uses `tracing` (never `println!`/`eprintln!`); CLI binaries use stdout/stderr
- Prefer `cargo nextest run` over `cargo test` (except for doctests and custom harnesses)
- No panics for recoverable conditions — use `thiserror`/`miette`
- No clippy silencing without explicit approval
- Conventional Commits: `<type>[scope]: <description>`
- Preferred crates: `clap` (CLI), `serde` (serialization), `miette` (diagnostics), `insta` (snapshots), `tracing` (logging), `rayon`/`crossbeam` (concurrency)

### Python (batchalign3)
- Use `uv` for dependency management (never `pip install`); `uv run` to execute
- TDD mandatory; `unittest.mock` is banned — use test doubles from `batchalign/tests/doubles.py`
- Full type annotations required; `Any` is not allowed; run `mypy` before committing
- All CHAT operations via `batchalign_core` Rust functions (no regex/string hacking)

### TypeScript (VS Code extension)
- Located in `talkbank-chatter/vscode/`
- LSP integration via stdio transport to `talkbank-lsp` binary

## Environment
- macOS (Darwin), Apple Silicon
- Rust (stable), Node.js, Python 3.11+ (via `uv`)
- `python` does not exist on macOS — always use `uv run`

## Per-Repo CLAUDE.md Files

Each repo and major crate has detailed CLAUDE.md guidance in `talkbank-private/claude-md/`. Key ones:
- `talkbank-private/claude-md/talkbank-chat/CLAUDE.md` — main talkbank-chat guidance
- `talkbank-private/claude-md/talkbank-chat/crates/CLAUDE.md` — Rust coding standards
- `talkbank-private/claude-md/talkbank-chat/spec/CLAUDE.md` — spec workflows
- `talkbank-private/claude-md/talkbank-clan/CLAUDE.md` — CLAN analysis commands
- `talkbank-private/claude-md/batchalign-core/CLAUDE.md` — PyO3 bridge rules
- `batchalign3/CLAUDE.md` — Python pipeline (lives in the repo itself)
