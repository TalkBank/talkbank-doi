# Code Comment Coverage Audit (2026-02-25)

Scope: Rust (`.rs`) and Python (`.py`) files across workspace projects, excluding vendored/build directories.

## Coverage Summary

| Project | Rust files | Rust comment ratio | Rust public fn doc ratio | Python files | Python comment ratio | Python def/class docstring ratio |
|---|---:|---:|---:|---:|---:|---:|
| `batchalign-hk-plugin` | 0 | 0.0000 | - | 36 | 0.3242 | 0.2711 |
| `batchalign3` | 823 | 0.1666 | 0.6863 | 180 | 0.2011 | 0.4501 |
| `talkbank-chat` | 906 | 0.2245 | 0.925 | 3 | 0.5844 | 0.35 |
| `talkbank-chatter` | 105 | 0.1916 | 0.8098 | 0 | 0.0000 | - |
| `talkbank-clan` | 39 | 0.2623 | 0.9737 | 0 | 0.0000 | - |
| `talkbank-private` | 0 | 0.0000 | - | 12 | 0.1811 | 0.439 |
| `tree-sitter-talkbank` | 2 | 0.3214 | - | 3 | 0.2148 | 0.6923 |

## Low-Comment Hotspots (Top 5 per language/project)

### batchalign3

- Rust:
  - `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/dispatch/special.rs` (ratio=0.0, nonempty=303)
  - `rust/crates/talkbank-tree-sitter-parser/tests/test_parse_health_recovery.rs` (ratio=0.0, nonempty=228)
  - `rust/crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/mor/item.rs` (ratio=0.0, nonempty=162)
  - `rust/crates/talkbank-direct-parser/tests/test_parse_health_recovery.rs` (ratio=0.0, nonempty=128)
  - `rust/crates/talkbank-model/src/validation/cross_utterance/tests/quotation_follows.rs` (ratio=0.0, nonempty=127)
- Python:
  - `batchalign/utils/names.py` (ratio=0.0, nonempty=6684)
  - `batchalign/utils/abbrev.py` (ratio=0.0, nonempty=425)
  - `batchalign/pipelines/asr/num2lang/ell.py` (ratio=0.0, nonempty=292)
  - `batchalign/pipelines/asr/num2lang/hrv.py` (ratio=0.0, nonempty=115)
  - `batchalign/pipelines/asr/num2lang/eng.py` (ratio=0.0, nonempty=111)

### talkbank-chat

- Rust:
  - `crates/talkbank-parser-tests/tests/generated/reference_corpus.rs` (ratio=0.0144, nonempty=348)
  - `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/dispatch/special.rs` (ratio=0.0162, nonempty=308)
  - `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/base/nonvocal.rs` (ratio=0.0201, nonempty=199)
  - `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/dispatch/simple.rs` (ratio=0.0247, nonempty=365)
  - `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/base/long_feature.rs` (ratio=0.0258, nonempty=155)

## Interpretation

- Comment density improved substantially in non-`batchalign3` repos after the extensive comment insertion pass.
- `batchalign3` values are included for workspace visibility but were not modified in this pass.
- Remaining hotspots are mostly test/generated-heavy Rust modules and data-heavy Python utility files.
