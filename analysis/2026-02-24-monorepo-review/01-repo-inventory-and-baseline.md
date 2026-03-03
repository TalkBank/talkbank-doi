# 01. Repo Inventory and Baseline

## Workspace composition

- `batchalign3`: Python + Rust + React dashboard
- `talkbank-chat`: Core Rust model/parser/spec toolchain
- `talkbank-chatter`: CLI + LSP + VS Code extension
- `talkbank-clan`: Rust CLAN command reimplementation
- `tree-sitter-talkbank`: grammar + multi-language bindings
- `talkbank-private`: internal docs/ops scripts

## Scale snapshot

- Approx tracked files (excluding `.git`, `target`, build/dist, venv caches): **5,271**
- Approx source/doc mix:
  - Rust: 1,858 `.rs`
  - Python: 234 `.py`
  - TypeScript: 29 `.ts` + 19 `.tsx`
  - Markdown: 940 `.md`

## Health check results captured

- Root `make check`:
  - `talkbank-chat` and `talkbank-chat/spec/tools`: pass
  - `talkbank-chatter`: fail
    - `crates/talkbank-lsp/src/alignment/formatters/content.rs:29`
    - `crates/talkbank-lsp/src/graph/labels.rs:42`
    - Cause: `SmolStr` vs `String` mismatch
- `talkbank-clan`: `cargo check --all-targets` pass
- `tree-sitter-talkbank`: `npx tree-sitter test` parses pass, query check fails
  - `highlights.scm` invalid node type `mor_category`
- `batchalign3`: pass
  - `uv run mypy batchalign/cli/ batchalign/pipelines/ batchalign/serve/`
  - `cargo check --manifest-path rust/Cargo.toml --all-targets`

## CI/workflow baseline

- `talkbank-chat`: strong CI with multi-gate checks
- `tree-sitter-talkbank`: CI present but currently allows query breakage to surface in test run
- `batchalign3`: CI present but mostly Python test/release/docs focused
- `talkbank-chatter`: no `.github/workflows`
- `talkbank-clan`: no `.github/workflows`

## Immediate baseline actions checklist

- [ ] Fix current `talkbank-chatter` compile break before further feature work
- [ ] Fix `tree-sitter-talkbank` query-node mismatch (`mor_category`)
- [ ] Add first-class CI workflows for `talkbank-chatter` and `talkbank-clan`
- [ ] Establish one shared workspace health script that runs all mandatory checks
- [ ] Track baseline metrics in a versioned dashboard (build pass rate, test durations, warning counts)
