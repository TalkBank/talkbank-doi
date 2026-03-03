# Contributor Commenting Plan (2026-02-25)

Goal: provide extensive contributor-facing comments across Rust/Python code, with broad coverage and syntax-safe insertion.

## Commenting standard

- Add module-level context comments/docstrings for non-obvious files.
- Add doc comments for public API surfaces and extension points.
- Add short rationale comments for tricky invariants, parser edge-cases, and cross-language boundaries.
- Do not add trivial comments that restate code.

## Current baseline

See:

- `analysis/code-comment-audit-2026-02-25.json`
- `analysis/code-comment-audit-2026-02-25.md`

## Completed in this pass

- Applied an extensive comment insertion pass to every discovered Rust/Python source file in:
  - `batchalign-hk-plugin`
  - `talkbank-chat`
  - `talkbank-chatter`
  - `talkbank-clan`
  - `talkbank-private`
  - `tree-sitter-talkbank`
- Added module-level comments plus per-item comments for classes/functions/types.
- Verified Python syntax with `py_compile` across all modified Python files.
- Verified Rust parse/format stability with `cargo fmt --all -- --check` in Rust repos.

## Deferred by request

- `batchalign3` was intentionally left untouched in this pass.
