# `build_text_utterance`: Tree-Sitter Misuse and Dead Code

**Status:** Current
**Last updated:** 2026-03-18

## Summary

`build_text_utterance()` in `batchalign3/crates/batchalign-chat-ops/src/build_chat.rs:330-367`
fabricates a complete CHAT document to parse a single utterance through tree-sitter. This is
the wrong parser for the job, and the function has **zero production callers**.

## The Problem

### What `build_text_utterance` does

Given raw text like `"hello world ."`, a speaker code, and optional timing, it:

1. Constructs a fake mini CHAT document with `@UTF8`, `@Begin`, `@Languages`,
   `@Participants`, `@ID`, one `*SPK:` line, and `@End`
2. Parses the entire fake document through **tree-sitter** (`parse_strict()`)
3. Extracts and returns the single `Utterance` from the parsed file

### Why this is wrong

**Tree-sitter is the wrong parser.** Tree-sitter is designed to parse complete
CHAT transcripts with file-level context (`@Options: CA`, language codes, etc.).
Using it for one utterance requires fabricating a throwaway CHAT document — a
fragile workaround.

The **direct parser** (`talkbank-direct-parser`) exists specifically for parsing
CHAT fragments in isolation. It already has:

- `parse_main_tier_impl()` — parses `*SPK:\thello world .` directly
- `parse_utterance_impl()` — parses a main tier + any dependent tiers
- `parse_word()` — parses individual words (already used by `build_word_utterance`)

These require no fake document wrapping.

**Parsing text at all is questionable** when the data model provides direct
constructors. `build_word_utterance()` (the sibling function at line 374-461)
demonstrates the correct pattern: it uses `DirectParser::parse_word()` for
individual words and `MainTier::new()` / `Utterance::new()` model constructors
to assemble the result. No document-level parsing needed.

## Who Uses It

### Production callers: none

`build_text_utterance()` is called from exactly one place: `build_chat()` at
line 193-202, when an `UtteranceDesc` has `.text` (a raw string) but no
`.words` (word-level tokens).

**No production code path ever provides `.text` without `.words`:**

- **Rust transcribe pipeline** (`batchalign-app/src/transcribe.rs`,
  `pipeline/transcribe.rs`): Uses `transcript_from_asr_utterances()`, which
  always sets `words: Some(words)` and `text: None` (line 290-295).

- **Python compat layer** (`batchalign/compat.py:272-278`): `Document.new()`
  splits text into individual words and passes them as `WordDesc`s via the
  `words` field. It does **not** use the `text` field.

- **All Python tests** (`test_rust_build_chat.py`): Every test case uses the
  `words` path with individual word tokens. None pass `text`.

### Only caller: one Rust unit test

The sole exerciser of the `text` path is `test_build_chat_text_utterance` at
line 585-609 in `build_chat.rs` — a unit test that exists specifically to test
`build_text_utterance`.

## Recommended Fix

### Immediate: replace tree-sitter with DirectParser

If the `text` path is kept for API completeness, replace the fake-document
approach with a direct `parse_main_tier_impl()` call:

```rust
let line = format!("*{speaker}:\t{text}{bullet}");
let parser = DirectParser::new().unwrap();
let errors = NullErrorSink;
match parser.parse_main_tier(&line, 0, &errors) { ... }
```

Or add a `parse_main_tier_content()` method to `DirectParser` that accepts just
the content without the `*SPK:\t` prefix (analogous to how `parse_mor_tier_content()`
works for `%mor`).

### Long-term: remove the `text` path entirely

Since no production code uses it, the `text` field on `UtteranceDesc` and the
`build_text_utterance()` function could be removed. All callers already provide
structured word-level input.

## Related

- `build_word_utterance()` (same file, line 374-461) — the correct pattern
- `talkbank-direct-parser` (`talkbank-tools/crates/talkbank-direct-parser/`) —
  fragment parser designed for exactly this use case
- `DirectParser::parse_main_tier()` — parses `*SPK:\t...` in isolation
- `MainTier::new()` / `Utterance::new()` — model constructors for direct assembly
