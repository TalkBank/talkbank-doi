# Commenting Next Steps (2026-02-26)

## Purpose

Continue the contributor-facing documentation effort that was scoped in `analysis/commenting-plan-2026-02-25.md`. This follow-up plan drills down into the remaining work, especially in `batchalign3`, so the broader talkbank ecosystem eventually has high-quality, non-mechanical doc comments covering all public APIs and data models.

## Progress so far

- A cross-repo doc-comment pass landed for the non-`batchalign3` Rust/Python directories as documented in `analysis/commenting-plan-2026-02-25.md`.
- The latest audit (`analysis/code-comment-audit-2026-02-25.md`) shows coverage improvements but highlights `batchalign3` files and a few Python helpers with near-zero doc density.

## Constraints to keep in mind

- `batchalign3` is intentionally untouched in the prior pass, so all current work must respect its existing codebase and avoid regressions.
- There is no git metadata at `/Users/chen/talkbank`, so rely on manual file tracking for this wave.
- Anchoring comments to canonical CHAT manual references requires double-checking the bookmarks against the regenerated HTML/PDF artifacts kept under `~/save-word-html-pdf` rather than editing generated HTML directly.

## Next steps

1. Tackle the `batchalign3/rust/crates/talkbank-tree-sitter-parser` modules that still have the lowest comment ratios in the audit (e.g., `header_parser/dispatch/special.rs` and the test snapshots). Add thoughtful doc comments that explain the domain intent and cross-reference the relevant anchors from `https://talkbank.org/0info/manuals/CHAT.html`.
2. Do the same for the data-model-heavy crates under `batchalign3/rust/crates/talkbank-model`, ensuring every public type links to the CHAT manual anchor describing that annotation type, as recorded in `analysis/code-comment-audit-2026-02-25.md` and the `talkbank` repo docs.
3. For the low-comment Python helpers in `batchalign3` (e.g., `batchalign/utils/names.py` and the `num2lang` modules), insert module-level docstrings explaining their responsibilities and algorithmic constraints.
4. Validate each Rust file with `cargo fmt --all -- --check` and each Python file with `python -m py_compile` after commenting to catch syntax mistakes introduced during doc addition.
5. Prepare an updated audit and summary note (similar to `analysis/code-comment-audit-2026-02-25.md`) once this round finishes so the next wave knows what remains.

## Data references

- Current coverage snapshot: `analysis/code-comment-audit-2026-02-25.md`
- Commenting standards and previous scope: `analysis/commenting-plan-2026-02-25.md`
- CHAT manual anchors and regenerated docs in `~/save-word-html-pdf`
