# Test/Doctest Coverage Audit (2026-02-25)

Scope: non-`batchalign3` repos updated in this pass.

| Repo | Rust src-like | Rust test-like | Rust files w/ `#[cfg(test)]` | Rust files w/ doctest hints | Python src-like | Python test-like |
|---|---:|---:|---:|---:|---:|---:|
| `batchalign-hk-plugin` | 0 | 0 | 0 | 0 | 36 | 0 |
| `talkbank-chat` | 561 | 345 | 119 | 175 | 3 | 0 |
| `talkbank-chatter` | 102 | 3 | 9 | 9 | 0 | 0 |
| `talkbank-clan` | 37 | 2 | 20 | 13 | 0 | 0 |
| `talkbank-private` | 0 | 0 | 0 | 0 | 10 | 2 |
| `tree-sitter-talkbank` | 2 | 0 | 1 | 1 | 2 | 1 |

## Findings

### batchalign-hk-plugin

- Add Python pytest smoke tests for key modules and scripts.

### talkbank-chat

- Add Python pytest smoke tests for key modules and scripts.

### talkbank-chatter

- Increase Rust integration/unit tests for src modules with minimal direct test coverage.

### talkbank-clan

- No immediate structural gaps detected from file-level audit heuristics.

### talkbank-private

- No immediate structural gaps detected from file-level audit heuristics.

### tree-sitter-talkbank

- Increase Rust integration/unit tests for src modules with minimal direct test coverage.

## Changes made in this pass

- Expanded `tree-sitter-talkbank` Python binding tests with attribute-surface checks and error-path checks.
- Added/expanded code comments/docstrings across all non-`batchalign3` Rust/Python files to improve maintainability and onboarding.

## Recommended next wave

1. Add doctest examples to top-level Rust crate docs in `talkbank-chatter` and `talkbank-clan` for key public entry points.
2. Add Python smoke tests for `batchalign-hk-plugin` core adapters (`aliyun_asr`, `tencent_asr`, `funaudio_asr`) with mocked external clients.
3. Add script-level regression tests in `talkbank-private/batchalign/scripts` for cache migration and warning parsers.
