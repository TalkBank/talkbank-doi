# Validation & Span Infrastructure: Remaining Work

**Date**: 2026-02-12
**Status**: In progress — builds and tests pass, but span overhaul is partial and validation coverage incomplete.

---

## 1. Span Infrastructure (Partially Complete)

### What's done

The `text-size` crate is integrated. A `Spanned` trait is defined in `talkbank-errors`. The derive macros (`SemanticEq`, `SemanticDiff`, `SpanShift`) were fixed to emit `_` bindings for `#[semantic_eq(skip)]` fields, eliminating unused-variable warnings.

**Types that now carry `span: Span` with real byte offsets from the tree-sitter parser:**

| Type | Variants/Fields | Real spans from TS parser? |
|------|----------------|---------------------------|
| `Separator` | 13 enum variants | ✅ Yes |
| `Terminator` | 21 enum variants | ✅ Yes |
| `Freecode` | struct field | ✅ Yes |
| `Quotation` | struct field | ✅ Yes |
| `OtherSpokenEvent` | struct field | ✅ Yes |
| `Action` | struct field | ✅ Yes |
| `Word` | struct field | ✅ (pre-existing) |
| `Event` | struct field | ✅ (pre-existing) |
| `Pause` | struct field | ✅ (pre-existing) |
| `Group` | struct field | ✅ (pre-existing) |
| `PhoGroup` | struct field | ✅ (pre-existing) |
| `SinGroup` | struct field | ✅ (pre-existing) |
| `Bullet` | struct field | ✅ (pre-existing) |
| `Postcode` | struct field | ✅ (pre-existing) |
| `MainTier` | struct field | ✅ (pre-existing) |
| `NonvocalSimple/Begin/End` | struct fields | ✅ (pre-existing) |
| `LongFeatureBegin/End` | struct fields | ✅ (pre-existing) |
| `OverlapPoint` | `Option<Span>` | ✅ (pre-existing) |
| `UnderlineMarker` | struct field | ✅ (pre-existing) |

### What's left — source-located types still missing span

| Type | Kind | Est. Sites | Notes |
|------|------|-----------|-------|
| `Linker` | enum, 7 unit variants | ~20 match arms, ~12 constructions | Same pattern as Separator/Terminator |
| `BracketedContent` | struct | ~17 constructions | Container — span covers the full bracket expression |
| `BracketedItems` | newtype `Vec<BracketedItem>` | ~15 constructions | Wrapper — span = union of items |

### What's left — delegating enums (no own span, delegate to inner type)

| Type | Variants | Notes |
|------|----------|-------|
| `UtteranceContent` | 23 variants | Each variant's inner type has span; implement `Spanned` by delegating |
| `BracketedItem` | 19 variants | Same pattern |

### Types that should NOT get span (pure value/metadata)

`FormType`, `WordCategory`, `WordLanguageMarker`, `UntranscribedStatus`, `PhoItemAlignment`, `SinItemAlignment`, `WordMorphologyAlignment`, `WordTimingAlignment`, `WordTiming`, `NonvocalLabel`.

### `Spanned` trait

Defined in `talkbank-errors/src/lib.rs` but **zero implementations** exist. Once the remaining span fields are added, implement `Spanned` for all types with `span: Span` fields, and for `UtteranceContent`/`BracketedItem` by delegation. Consider a derive macro to auto-generate `Spanned` for structs with a `span` field.

### Direct parser

Currently uses `Span::DUMMY` everywhere for Separator, Terminator, Freecode, etc. Real span threading in the direct parser is lower priority since it's experimental.

---

## 2. Missing CHECK Rules (2 rules)

### E258 ConsecutiveCommas (CHECK 107)

**Status**: Error code variant added to `error_code.rs`, but validation logic NOT implemented.

**What to do**: In `validation/main_tier.rs`, add a check that walks `main_tier.content.content.iter()` and detects adjacent `UtteranceContent::Separator(Separator::Comma { .. })` items. Report E258 with the span of the second comma. The CHAT manual says use `‚` (single low-9 quotation mark, U+201A) for double-comma semantics; consecutive ASCII commas are an error.

**Test fixture needed**: `E2xx_word_errors/E258_consecutive_commas.cha` (or `E3xx_main_tier_errors/`).

### W603 TierNameTooLong (CHECK 58)

**Status**: Error code variant NOT yet added.

**What to do**: Add `W603` variant to `error_code.rs`. In `validation/unparsed_tier.rs` (function `check_tier_label`), check if user-defined tier labels exceed 8 characters (constant `MAX_SPEAKER_ID_LENGTH=7` exists but is unused). Report as warning.

**Test fixture needed**: `W6xx_warnings/W603_tier_name_too_long.cha`.

---

## 3. Error Code Test Coverage

### Current state

- **172** error/warning codes defined in `error_code.rs`
- **130** test fixture files in `rust/tests/error_corpus/`
- **68** distinct codes exercised by tree-sitter parser in `expectations.json`
- **104** codes with no test coverage at all

### 104 untested codes — triage by category

Many of these are tested indirectly through other means (roundtrip corpus, parser unit tests, etc.) or are parser-internal error recovery codes that are hard to trigger via `.cha` fixtures. They need triage into:

1. **Should have fixture tests** — validation-layer errors reachable with crafted `.cha` input
2. **Tested elsewhere** — parser unit tests, insta snapshots, roundtrip corpus covers them
3. **Dead/unreachable** — error codes defined but never emitted; candidates for removal

**Full list of untested codes** (tree-sitter parser):

```
E001 E002 E003 E101
E202 E208 E209 E210 E211 E212 E213 E214
E230 E232 E233 E241 E242 E244 E245 E247 E248 E250 E251 E253 E258
E302 E303 E307 E309 E310 E312 E313 E314 E315 E317 E318 E319 E320
E321 E322 E323 E324 E325 E326 E330 E331 E340 E341 E344 E345 E346
E347 E348 E350 E351 E352 E353 E354 E355 E356 E357 E361 E363 E364
E365 E366 E369 E373 E374 E377 E378 E380 E381 E382 E383 E384 E385
E386 E388 E404
E501 E502 E506 E508 E510 E511 E512 E514 E518 E531
E700 E701 E702 E703 E704 E709 E712 E720
W001 W108 W210 W211 W601 W602
```

Note: Some have `.cha` fixture files but the test harness doesn't check them (the `expectations.json` may mark them as `kind: "error"` without specifying expected codes, or the fixture may test a different code than its filename suggests).

### Recommended approach

1. **Batch 1 — Quick wins**: Codes with existing fixtures that just need expectations added (many E3xx, E5xx files exist)
2. **Batch 2 — Header/tier validation**: E5xx codes — straightforward to craft `.cha` fixtures
3. **Batch 3 — Word-level errors**: E2xx codes — need careful construction of malformed words
4. **Batch 4 — Parser-internal**: E3xx tree-parsing errors — may require intentionally malformed tree-sitter output
5. **Batch 5 — Dead code triage**: Audit whether codes like E001-E003, E101, E212, E214 are actually emitted anywhere

---

## 4. Other Infrastructure Work

### Test harness improvements

The error corpus test harness (`rust/tests/error_corpus/`) was fixed to use the canonical `ChatParser` API (via `VecErrorSink`) instead of the legacy `parse_chat_file() -> Result` path. This was critical — the legacy path discarded the `ChatFile` on any error, making validation-layer errors unreachable.

### Legacy API removal

The `TreeSitterParser::parse_chat_file(&str) -> Result<ChatFile, ParseErrors>` method was rewritten to delegate to the `ChatParser` trait implementation. The headerless fallback was removed. The generated construct tests (`generated_construct_tests_body.rs`, 162 tests) still use the `Result`-returning API but pass since their inputs are valid CHAT.

### Dead annotation/group directory

Deleted `rust/crates/talkbank-model/src/model/annotation/group/` — it contained duplicate type definitions (Group, PhoGroup, SinGroup, Quotation) that were never referenced from any module declaration.

---

## 5. Priority Order

1. **Finish span overhaul** — Linker (7 variants), BracketedContent, BracketedItems, then `Spanned` trait impls for UtteranceContent/BracketedItem delegation
2. **Implement E258 ConsecutiveCommas** — validation logic + test fixture
3. **Implement W603 TierNameTooLong** — error code + validation logic + test fixture
4. **Error code coverage triage** — audit 104 untested codes, add fixtures for reachable ones, mark dead ones for removal
5. **Spanned trait implementations** — implement for all types with span fields (consider derive macro)

---

## 6. Files Modified in This Session

### Commits

| SHA | Description |
|-----|-------------|
| `e490fb5d` | Validation completion audit document |
| `6c74dc87` | Migrate CHAT parser to unified ChatParser trait interface |
| `96773785` | Update error corpus expectations for unified parser API |
| `050b52c5` | Add span to Separator/Freecode/Quotation/OtherSpokenEvent/Action + derive macro fixes |
| `626f6cc3` | Thread real spans from TS parser for Freecode/Quotation/OtherSpokenEvent/Action |
| `0918b8da` | Add span to Terminator (21 variants) |

### Key files

- `rust/crates/talkbank-errors/src/lib.rs` — `Span`, `Spanned` trait, `text-size` re-exports
- `rust/crates/talkbank-errors/src/codes/error_code.rs` — E258 added (W603 not yet)
- `rust/crates/talkbank-derive/src/{semantic_eq,semantic_diff,span_shift,helpers}.rs` — fixed skipped-field bindings
- `rust/crates/talkbank-model/src/model/content/{separator,terminator,freecode,action,other_spoken,group}.rs` — span fields added
- `rust/crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/separator.rs` — real spans
- `rust/crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/terminator.rs` — real spans
- `rust/tests/error_corpus/` — 130 test fixtures, expectations.json, fixed harness
