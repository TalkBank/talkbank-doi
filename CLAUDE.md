# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

TalkBank development workspace — a git meta-repo that tracks cross-repo coordination files while sub-repos are gitignored with independent histories. TalkBank provides tools for linguistic research on conversational data in CHAT format (Codes for the Human Analysis of Transcripts).

Data flows: **spec** (source of truth) → **grammar** (tree-sitter) → **Rust** (parsers, model, validation) → **applications** (CLI, LSP, VS Code, Python pipeline).

## Workspace Layout

This meta-repo tracks:
- `Makefile` — cross-repo build/test/status commands
- `RELEASE-PLAN.md` — coordinated release planning
- `analysis/` — workspace-wide audits and reports
- `scripts/` — one-off maintenance scripts
- `.claude/skills/` — 8 workspace skills: bump, cascade, clan-command, debug-ui, doc-reorg, doc-search, doc-update, verify

Sub-repos are gitignored. Each has its own git history, CLAUDE.md, and build system.

## Repositories

| Directory | Purpose | Language |
|-----------|---------|----------|
| `tree-sitter-talkbank/` | Tree-sitter grammar for CHAT format | JavaScript/C |
| `talkbank-chat/` | CHAT spec, core Rust crates (parsing, model, validation, transform) | Rust |
| `talkbank-chatter/` | CLI (`chatter`), LSP server, VS Code extension | Rust/TypeScript |
| `talkbank-clan/` | CLAN analysis commands (FREQ, MLU, MLT, etc.) | Rust |
| `batchalign3/` | ASR, forced alignment, morphosyntax pipeline | Python/Rust (PyO3) |
| `batchalign-hk-plugin/` | HK deployment plugin for batchalign | Python |
| `talkbank-private/` | Internal archive, deploy scripts, ops docs | |

## Cross-Repo Commands

```bash
make status      # Git status across all repos
make check       # Cargo check all Rust repos
make test        # Run tests across repos
make verify-all  # Full cross-repo verification gate
make clone       # Clone all repos fresh (for new machines)
make pull        # Pull all repos
```

For per-repo commands, see each repo's CLAUDE.md.

## Cross-Repo Architecture

### Crate Dependency Flow (Rust)

```
talkbank-errors              <- Error types, ErrorSink trait, Span
    |
talkbank-model               <- Data model (ChatFile AST), validation, alignment
    |
talkbank-parser-api          <- ChatParser trait abstraction
    |
talkbank-tree-sitter-parser  <- Canonical parser (tree-sitter CST -> model)
talkbank-direct-parser       <- Experimental parser (chumsky combinators)
    |
talkbank-transform           <- Pipelines: parse+validate, CHAT<->JSON roundtrip, caching
    |
talkbank-cli (chatter)       <- CLI tool (in talkbank-chatter repo)
talkbank-lsp                 <- Language server (in talkbank-chatter repo)
talkbank-clan                <- CLAN analysis commands (in talkbank-clan repo)
batchalign-core              <- PyO3 bridge to Python (in batchalign3 repo)
```

Supporting crates: `talkbank-derive` (proc macros), `talkbank-json` (schema validation), `talkbank-pipeline` (config types), `talkbank-highlight` (syntax highlighting), `talkbank-parser-tests` (equivalence tests), `send2clan-sys` (FFI to CLAN).

### Cross-Repo Path Dependencies

All Rust repos use local path dependencies. The sibling directory layout is load-bearing:

```
~/talkbank/
├── tree-sitter-talkbank/       # Grammar (referenced by talkbank-chat and talkbank-chatter)
├── talkbank-chat/              # Core crates (referenced by all other Rust repos)
│   └── crates/                 # talkbank-errors, talkbank-model, talkbank-transform, etc.
├── talkbank-chatter/           # CLI, LSP (depends on talkbank-chat crates + tree-sitter-talkbank)
├── talkbank-clan/              # Analysis (depends on talkbank-chat crates)
├── batchalign3/
│   ├── rust/crates/            # batchalign-core (path deps to talkbank-chat)
│   └── rust-next/crates/       # Standalone Rust server (path deps to talkbank-chat)
└── batchalign-hk-plugin/       # HK deployment plugin
```

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

## Per-Repo Guidance

Each repo has its own CLAUDE.md (37 files, ~3,800 lines total). Key entry points:

| Repo | CLAUDE.md | Lines | Sub-files |
|------|-----------|-------|-----------|
| `tree-sitter-talkbank/` | `CLAUDE.md` | 95 | — |
| `talkbank-chat/` | `CLAUDE.md` | 254 | 13 (crates, spec, fuzz) |
| `talkbank-chatter/` | `CLAUDE.md` | 147 | 5 (crates, vscode) |
| `talkbank-clan/` | `CLAUDE.md` | 75 | — |
| `batchalign3/` | `CLAUDE.md` | 394 | 10 (rust, rust-next, frontend) |
| `batchalign-hk-plugin/` | `CLAUDE.md` | 155 | — |
| `talkbank-private/` | — | — | — |
