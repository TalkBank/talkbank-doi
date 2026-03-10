# TalkBank Portfolio LOC Snapshot (Generated 2026-03-05)

## Scope and Method
- Repos covered: `tree-sitter-talkbank`, `talkbank-chat`, `talkbank-chatter`, `talkbank-clan`, `batchalign3`
- Counts are from **git-tracked files only**.
- Excluded: build/dependency artifacts (`target/`, `node_modules/`, `dist/`, `build/`, `.venv/`, caches), and common generated paths (`generated/`, `tests/generated/`, `spec/tools/src/generated/`) plus well-known generated files (`src/parser.c`, `src/grammar.json`, `src/node-types.json`, etc.).
- These are approximate engineering LOC metrics, not SLOC by compiler semantics.
- Rust crate rows below are crate-local Rust LOC (including crate tests/benches/examples when present).

## Executive Summary (Per Repo)
| Repo | Approx code LOC (strict) | Primary stack |
|---|---:|---|
| `talkbank-chat` | 125,273 | Rust core parser/model/validation + support scripts |
| `batchalign3` | 68,185 | Rust + Python + frontend TS |
| `talkbank-chatter` | 25,769 | Rust CLI/LSP + VS Code extension |
| `talkbank-clan` | 22,319 | Rust analysis library |
| `tree-sitter-talkbank` | 3,242 | Grammar package + bindings scaffolding |

## Repo Language Rollups

### tree-sitter-talkbank
- Total: **3,242**
- Breakdown: Rust 130, Python 157, JavaScript 2,056, TypeScript 60, C/C++ 756, Go 30, Swift 53

### talkbank-chat
- Total: **125,273**
- Breakdown: Rust 122,638, Python 552, JavaScript 645, Shell 807, C/C++ 631

### talkbank-chatter
- Total: **25,769**
- Breakdown: Rust 19,090, TypeScript 4,566, JavaScript 1,119, C/C++ 994

### talkbank-clan
- Total: **22,319**
- Breakdown: Rust 22,319

### batchalign3
- Total: **68,185**
- Breakdown: Rust 52,364, Python 11,870, TypeScript 2,556, JavaScript 623, Shell 772

## Rust Crates (with short descriptions)

### talkbank-chat crates
| Crate | Rust LOC | Description |
|---|---:|---|
| `talkbank-model` | 42,734 | TalkBank data model and validation |
| `talkbank-tree-sitter-parser` | 21,196 | Parsing implementations for CHAT format (tree-sitter and hand-written) |
| `talkbank-tools` | 13,961 | Root utility/tooling binaries (no crate description string) |
| `talkbank-parser-tests` | 12,377 | Shared parser test harness and fixtures |
| `talkbank-direct-parser` | 10,013 | Direct CHAT parser (non-CST), optimized for batch parsing |
| `talkbank-errors` | 4,569 | Shared error types and codes for TalkBank tools |
| `talkbank-transform` | 3,689 | Transformation pipelines for CHAT format (CHAT ↔ JSON, normalization) |
| `talkbank-derive` | 1,480 | Derive macros for talkbank-model |
| `talkbank-parser-api` | 636 | Parser API traits and shared contracts |
| `talkbank-json` | 200 | JSON serialization helpers for TalkBank models |
| `talkbank-pipeline` | 165 | Shared pipeline options and helpers |

### talkbank-chatter crates
| Crate | Rust LOC | Description |
|---|---:|---|
| `talkbank-cli` | 9,233 | Command-line interface for CHAT format validation, conversion, and normalization |
| `talkbank-lsp` | 8,734 | Language Server Protocol implementation for CHAT format |
| `send2clan-sys` | 795 | Rust FFI bindings for send2clan (open file in CLAN) |
| `talkbank-highlight` | 328 | Syntax highlighting for CHAT format using tree-sitter-highlight |

### talkbank-clan crate
| Crate | Rust LOC | Description |
|---|---:|---|
| `talkbank-clan` | 22,319 | CLAN analysis command reimplementation for CHAT transcripts |

### batchalign3 Rust crates
| Crate | Rust LOC | Description |
|---|---:|---|
| `batchalign-chat-ops` | 14,821 | Shared CHAT manipulation functions (extract, inject, cache, parse, serialize) |
| `batchalign-server` | 12,592 | Axum server, WebSocket, job store, SQLite persistence |
| `batchalign-cli` | 11,178 | CLI dispatch and daemon lifecycle |
| `batchalign-core` | 5,705 | Rust-powered CHAT handling for Batchalign |
| `batchalign-types` | 3,400 | Shared API/config/worker IPC types |
| `batchalign-worker` | 2,274 | Python worker process manager |
| `batchalign-bin` | 861 | Binary entry point for batchalign3 |
| `batchalign-cache` | 909 | Cache crate (no manifest description string) |

### tree-sitter-talkbank Rust crate
| Crate | Rust LOC | Description |
|---|---:|---|
| `tree-sitter-talkbank` | 130 | TalkBank CHAT transcription format |

## Python / TypeScript / JS Packages (non-Cargo units)

### tree-sitter-talkbank
| Package/unit | LOC | Description |
|---|---:|---|
| `tree-sitter-talkbank` npm package surface (`bindings/node` + `grammar.js`) | 2,117 (JS 2,038, TS 60, C/C++ 19) | Node distribution for CHAT tree-sitter grammar |
| `bindings/python/tree_sitter_talkbank` | 101 (Python 66, C/C++ 35) | Python binding package for the grammar |

### talkbank-chatter
| Package/unit | LOC | Description |
|---|---:|---|
| `vscode/` extension package (`talkbank-chat`) | 5,685 (TS 4,566, JS 1,119) | VS Code extension for CHAT language tooling |

### batchalign3
| Package/unit | LOC | Description |
|---|---:|---|
| Python package `batchalign3` (`batchalign/`) | 9,879 Python | Python speech/language sample analysis package |
| Python extension wrapper/stubs (`batchalign_core/`, `stubs/batchalign_core/`) | 239 Python | PyO3 wrapper exports and typing stubs |
| Frontend package `batchalign-dashboard` (`frontend/`) | 3,179 (TS 2,556, JS 623) | Dashboard/frontend app package |

## Notes for Leadership
- The largest code concentration is in `talkbank-chat` (core semantics + parser stack).
- `batchalign3` is the main polyglot repo (Rust/Python/frontend).
- `talkbank-chatter` is moderate-size, split between Rust backend and VS Code TypeScript frontend.
- `tree-sitter-talkbank` is intentionally small at the Rust layer, with most code in grammar/JS/C binding surfaces.
