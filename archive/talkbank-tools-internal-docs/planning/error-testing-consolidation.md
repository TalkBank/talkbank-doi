# Error Code Testing: Consolidation Plan

**Date**: 2026-02-12
**Status**: Not started — plan only

---

## The Problem

We have **three overlapping test systems** for error codes, none of which is complete:

### 1. `spec/errors/*.md` — The intended source of truth
- 62 markdown specs exist
- Generators defined (`gen_rust_tests`, `gen_validation_tests`) but produce **empty output files**
- The pipeline is declared but not wired up

### 2. `rust/tests/error_corpus/` — What actually runs today
- 130 hand-maintained `.cha` fixtures with `expectations.json`
- Many fixtures don't trigger the code their filename claims (e.g., `E312_unclosed_bracket.cha` emits E304)
- 15 fixtures emit zero errors
- Only 78 of 172 codes exercised

### 3. `rust/crates/talkbank-parser-tests/tests/generated/` — Empty placeholders
- `generated_error_tests_body.rs` and `generated_validation_tests_body.rs` contain only comments
- Supposed to be generated from `spec/errors/` but never were

**Net result**: 172 error codes defined, 78 exercised, ~17 dead, and the spec→test pipeline doesn't function.

---

## The Principled Approach

**One source of truth (`spec/errors/`), one generator (`make test-gen`), one test suite (generated).**

### Step 1: Fix the generator pipeline

The `gen_rust_tests` and `gen_validation_tests` binaries in `spec/tools/` need to produce working Rust tests. Currently they output empty files. The work:

- **Parser-layer errors** (`layer: parser`): Generate a test that parses the CHAT example via `TreeSitterParser`, collects errors with `VecErrorSink`, asserts expected code appears.
- **Validation-layer errors** (`layer: validation`): Generate a test that parses successfully, runs `validate_with_alignment`, asserts expected code.
- Output goes to `rust/crates/talkbank-parser-tests/tests/generated/`.
- `make test-gen` regenerates everything.

This is the highest-leverage step — once the pipeline works, adding coverage is just adding specs.

### Step 2: Audit and consolidate `spec/errors/`

For every error code in `error_code.rs`:

| Situation | Action |
|-----------|--------|
| Spec exists, example correct | Verify layer/category, done |
| Spec exists, example wrong/missing | Fix the CHAT example |
| No spec, fixture exists in `error_corpus/` | Run `corpus_to_specs` or create spec manually |
| No spec, no fixture, code is emitted | Write a new spec with a CHAT example that triggers the code |
| Code is dead (never emitted) | Mark deprecated or remove from `error_code.rs` |

**Target**: One `spec/errors/E###_*.md` per living error code, with a valid CHAT example that triggers exactly that code.

Current inventory:
- 62 specs exist (but only ~3 are hand-written quality; 59 are auto-generated stubs)
- ~77 emitted codes have no spec at all
- ~17 dead codes to remove/deprecate

### Step 3: Retire `rust/tests/error_corpus/`

Once spec-driven tests cover all codes:
- Delete `rust/tests/error_corpus/` entirely, OR
- Keep as a secondary regression suite but freeze it (no new fixtures)

### Step 4: CI gate

Add a coverage check (can be a test or a `make` target):
- Every non-deprecated error code in `error_code.rs` must have a corresponding spec in `spec/errors/`
- Every spec must produce a generated test
- `make test-gen && cargo test` must pass

---

## Dead Codes (candidates for removal)

These are defined in `error_code.rs` but never emitted in production code:

```
E210 IllegalReplacementForFragment    (deprecated → E387)
E211 OmissionInReplacement            (deprecated → E390)
E213 UntranscribedInReplacement       (deprecated → E391)
E258 ConsecutiveCommas                (added but validation logic never implemented)
E303 SyntaxError                      (superseded by more specific codes)
E317 UnparsableFileContent
E318 UnparsableDependentTier
E345 UnmatchedScopedAnnotationBegin
E348 MissingOverlapEnd
E350 GenericAnnotationError
E366 LongFeatureLabelMismatch
E369 NonvocalLabelMismatch
E380 UnknownSeparator
E385 WordParseError
E386 TextTierParseError
E514 MissingLanguageCode
E720 MorGraCountMismatch
```

---

## Also Pending (from earlier session work, separate concerns)

- **Span overhaul**: Linker, BracketedContent, BracketedItems still need `span: Span`; `Spanned` trait has zero implementations
- **E258 ConsecutiveCommas**: Error code added but validation logic not implemented (CHECK rule 107)
- **W603 TierNameTooLong**: Not yet added at all (CHECK rule 58)

---

## Execution Order

1. **Fix generator pipeline** (Step 1) — highest leverage, unblocks everything
2. **Backfill specs** (Step 2) — ~110 specs to create/verify
3. **Run `make test-gen`**, verify all codes covered
4. **Clean up dead codes** and retire error corpus (Steps 3–4)
5. **Resume span overhaul** (separate concern)

---
Last Updated: 2026-02-12
