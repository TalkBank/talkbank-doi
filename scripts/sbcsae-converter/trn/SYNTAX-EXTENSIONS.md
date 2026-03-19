# TRN Syntax Extensions

**Last updated:** 2026-03-19

This document records extensions and corrections to the original SBCSAE TRN
transcription format. The goal is to make the TRN files unambiguous and
self-documenting, so the converter is a straightforward translation with no
heuristics needed.

## Philosophy

The original UCSB TRN files contain transcriber errors and formatting
inconsistencies (documented in `FIXES.md`). Rather than relying on a heuristic
algorithm to guess the intended structure, we fix the source files directly.
Each fix is documented, traceable, and version-controlled.

## Conventions

### 1. Bracket numbering

The original TRN format uses numbered brackets `[2...2]` through `[9...9]` for
concurrent overlap groups, with unnumbered `[...]` for the first group. We
extend this with explicit numbering where the original was ambiguous:

- **`[1...1]`**: Explicitly marks the first overlap group. The original TRN
  convention left this unnumbered, but we add `1` when disambiguation is needed.
  This is a TalkBank extension — the original UCSB format does not use `[1`.

- **Index assignment**: Every overlap group in the file should have a unique
  index within its local run. Indices reset between non-overlapping stretches
  of speech.

### 2. Source corrections (documented in FIXES.md)

All corrections to the original TRN files are documented in `FIXES.md` with:
- The original text
- The corrected text
- The reason for the correction
- A reference to the hand-edited CHAT where available

Categories of corrections:
- **Bracket placement**: Brackets embedded inside `((COMMENT))` names or
  `<<NONVOCAL>>` spans
- **NUL byte corruption**: Lines joined by `\x00` bytes, split back into
  separate lines
- **Missing punctuation**: Missing `)` in vocalisms, missing `]` in brackets
- **Case inconsistency**: Lowercase long feature close labels (`x>` → `X>`)
- **Format errors**: Missing tabs, missing speaker colons
- **Encoding**: DEL bytes stripped, Windows-1252 smart quotes normalized

### 3. Heuristic directives (not used)

The Java DT parser recognized `{+}` (OVERLAP_CONTINUE) and `{-}`
(OVERLAP_DISCONTINUE) as inline markers to override the overlap inference
heuristic. These do not appear in the SBCSAE source files and we do not use
them. All disambiguation is achieved through explicit bracket numbering instead.

## Compatibility

The extended TRN files remain valid for the original UCSB format specification
with one exception: the `[1...1]` index notation is a TalkBank extension. Any
tool that processes TRN files should treat `[1` the same as `[` (unnumbered,
first overlap group).

All other corrections fix data errors that any parser would need to handle.
