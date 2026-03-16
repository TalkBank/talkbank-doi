# talkbank-dev GEMINI Workspace Guide

## Project Overview
This is the **talkbank-dev** private development workspace for [TalkBank](https://talkbank.org/). It serves as the single home for all TalkBank project assets, including code repositories, corpus data, deploy infrastructure, websites, legacy tools, and documentation.

The workspace orchestrates multiple independent git repositories into a unified sibling layout. This layout is structurally load-bearing (e.g., `batchalign3` relies on Rust path dependencies pointing to `talkbank-tools`).

### Core Data Flow
**spec** (source of truth) → **grammar** (tree-sitter) → **Rust** (parsers, model, validation) → **applications** (CLI, LSP, VS Code, Python pipeline)

## Directory Overview & Key Projects

- `talkbank-tools/`: Core logic including Grammar, spec, Rust crates, CLI, LSP, and VS Code extension.
- `batchalign3/`: NLP pipeline featuring a Rust server and Python ML model server.
- `data/`: 16 corpus data repos (e.g., childes, phon, aphasia, dementia).
- `deploy/`: Deployment infrastructure, Ansible playbooks, and scripts.
- `docs/`: Internal docs, build notes, and reports. (See `docs/inventory.md` for a complete list of all sub-projects).
- `analysis/`, `ops/`, `scripts/`: Workspace-wide audits, operational, and maintenance scripts.
- `known-issues/`: Validation baselines for external corpora.

*Note: The code and data repositories are gitignored sub-repositories with their own independent histories.*

## Building and Running

Workspace orchestration is managed via `make`.

### Setup & Cloning
- `make clone` — Clone ALL repos (code, data, web, and collaborator).
- `make clone-minimal` — Clone just the core repos (`talkbank-tools` and `batchalign3`).
- `make clone-data` — Clone corpus data repos.

### Git Operations
- `make status` — Check git status across all sub-repos.
- `make pull` — Pull latest changes for all repos.

### Build, Test, & Validation
- `make check` — Run Cargo check across all Rust workspaces.
- `make test` — Run tests across repos (`cargo nextest`, `pytest`, `vitest`).
- `make verify-all` — Full cross-repo verification gate (formatting, linting, tests).
- `make verify-release-gates` — Runs comprehensive contract suites and coverage generation.

## Development Conventions

- **Language Stack:** Heavy use of **Rust** (`cargo`, `clippy`, `rustfmt`, `llvm-cov`) for core libraries and APIs, **Python** (`uv`, `pytest`) for ML pipelines, and **TypeScript/Node.js** for editor tooling.
- **Dependencies:** `talkbank-tools` is self-contained. `batchalign3` (both Rust crates and PyO3 bridge) depends on local paths pointing to `talkbank-tools`.
- **Testing:** Large scale corpus testing is done via the `talkbank-cli` (e.g., `cargo run --release -p talkbank-cli -- validate ../data/ --force`). Compare validation errors against baselines in `known-issues/`.
- **Documentation Standard:** All markdown documents modified or created must include a date header right below the title. Do not update dates on files you haven't reviewed.
  ```markdown
  **Status:** Current | Historical | Reference | Draft
  **Last updated:** YYYY-MM-DD
  ```
- **Repo-Specific Context:** Always check individual repositories for their own `CLAUDE.md` or `GEMINI.md` files (e.g., inside `talkbank-tools/` and `batchalign3/`) for fine-grained development rules.

---
**Status:** Current
**Last updated:** 2026-03-16