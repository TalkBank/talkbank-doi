# Rust Sentinel/Dummy Audit

Date: 2026-02-06
Scope: `rust/**/*.rs` (production + tests)

## Executive Summary

This audit found sentinel/dummy usage concentrated in `talkbank-model` and `talkbank-parser`, with the highest runtime risk in tree-sitter error recovery and model validation context plumbing.

Key counts from direct code scan:

- `Span::DUMMY`: 67 occurrences
- `Span::from_usize(0, 0)`: 13 occurrences
- `SourceLocation::from_offsets(0, 0)`: 5 occurrences
- `ErrorContext::new("", 0..0, ...)`: 27 occurrences
- Sentinel union (deduplicated across strong patterns): 159 locations
- Production vs test split (union set): 133 production, 26 test

By crate (union set):

- `talkbank-model`: 108
- `talkbank-parser`: 35
- `talkbank-model`: 11
- `talkbank-direct-parser`: 2
- `talkbank-transform`: 1
- `talkbank-lsp`: 1

Interpretation:

- The parser currently uses explicit error reporting for many tree-sitter recovery cases, but still synthesizes dummy objects/spans in some critical paths.
- The model layer frequently uses dummy spans as default constructor values and validation fallback context.
- Diagnostics still have multiple empty-context constructions, which encode "unknown" via sentinel values rather than type-level states.

## Taxonomy Used

This document classifies sentinel patterns into these categories:

1. `Location Sentinel`
- Meaning: fabricated source location (`0..0`, `DUMMY`, fake line/column) used to represent unknown source.

2. `Context Sentinel`
- Meaning: empty source/context payloads (`ErrorContext::new("", 0..0, "")`) used to represent unavailable context.

3. `Data Sentinel`
- Meaning: synthetic model values used as placeholders (for example dummy `%mor` words with borrowed literals).

4. `Control-Flow Sentinel`
- Meaning: `unwrap_or_default()`/fallback behavior that suppresses explicit unknown/error states.

5. `Alignment Placeholder`
- Meaning: placeholder index semantics represented as `None`; this is explicit and mostly acceptable, but still audited for leakage.

## Critical Findings (Runtime Paths)

## 1) Tree-sitter parser creates synthetic diagnostics on parser init failure

Files:

- `rust/crates/talkbank-parser/src/api/file.rs`
- `rust/crates/talkbank-parser/src/api/header.rs`
- `rust/crates/talkbank-parser/src/api/main_tier.rs`

Current pattern:

- `SourceLocation::from_offsets(0, 0)`
- `ErrorContext::new("", 0..0, "TreeSitterParser::new")`

Risk:

- Unknown source is encoded as fake source offsets and empty context.
- Downstream renderer must infer this was unknown; type information is lost.

Proposed replacement:

- Introduce explicit construction for no-source failures:
  - `ParseError::new_no_source(code, severity, message)`
  - Internally: `SourceLocation { span, line: None, column: None }` + `context: None`
- Optional: `enum DiagnosticOrigin { SourceBacked { span: Span }, ParserInit, ValidationDerived, RecoveredNode }`

Migration steps:

1. Add no-source constructor in `talkbank-model`.
2. Replace three parser init callsites.
3. Add render tests for no-source diagnostics in CLI/TUI.

## 2) Tree-sitter recovery paths still materialize dummy `%mor` words

File:

- `rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs`

Current pattern:

- On missing main chunk/clitic/compound: creates `MorWordBorrowed` with synthetic fields (`v`, `missing`, `clitic`, `error`).

Risk:

- Fabricated linguistic content may leak into later stages.
- Distinguishing true parse content from recovery artifacts requires heuristics.

Proposed replacement:

- Replace dummy word construction with explicit recovered variant:
  - `enum ChunkBorrowed<'a> { Word(MorWordBorrowed<'a>), Compound(...), RecoveredMissingMor { reason: RecoveredMorReason, source_span: Option<Span> } }`
  - `enum RecoveredMorReason { MissingMainChunk, MissingCliticChunk, MissingCompoundWord }`

Migration steps:

1. Add new enum variant + reason enum.
2. Update parser recovery sites to emit recovered variant.
3. Update consumers/serializers/validators to handle recovered variant explicitly.
4. Add roundtrip and validation tests that confirm recovered variants never serialize as normal lexical items.

## 3) Prefix parser uses `Span::DUMMY` as mutable default

File:

- `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/prefix.rs`

Current pattern:

- `let mut speaker_span = Span::DUMMY;` then overwritten only if speaker found.

Risk:

- Missing speaker is represented by dummy span instead of an explicit missing state.

Proposed replacement:

- Change `PrefixData` to explicit state:
  - `enum SpeakerField { Present { code: String, span: Span }, Missing { reason: MissingSpeakerReason } }`
- Remove default mutable dummy span entirely.

Migration steps:

1. Update `PrefixData` shape.
2. Replace downstream checks on `speaker` + `speaker_span` with `match SpeakerField`.
3. Add tests for missing speaker and malformed prefix order.

## 4) Finder APIs return errors with fake zero spans/context

File:

- `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/finder.rs`

Current pattern:

- Missing main_tier error uses `SourceLocation::from_offsets(0, 0)` and `ErrorContext::new("", 0..0, "")`.

Proposed replacement:

- Return no-source parse error (same strategy as parser init failures).
- Optionally include tree-level metadata field in error extension (`root_kind`, `searched_kind`).

## 5) Model validation defaults unknown field span to `DUMMY`/`0..0`

Key files:

- `rust/crates/talkbank-model/src/model/annotation/annotated.rs`
- `rust/crates/talkbank-model/src/model/annotation/replacement.rs`
- `rust/crates/talkbank-model/src/model/content/tier_content.rs`
- `rust/crates/talkbank-model/src/model/non_empty_string.rs`
- `rust/crates/talkbank-model/src/model/content/word/types.rs`
- `rust/crates/talkbank-model/src/validation/retrace/mod.rs`

Current pattern:

- `context.field_span.unwrap_or(Span::DUMMY)` and related `Span::from_usize(0, 0)` fallbacks.

Risk:

- Unknown source location represented as a concrete span value.
- Validators and diagnostics can accidentally treat unknown as real position.

Proposed replacement:

- Replace `ValidationContext` span fields with explicit known/unknown enum:
  - `enum FieldSpanRef { Known(Span), Unknown(UnknownSpanReason) }`
  - `enum UnknownSpanReason { SyntheticNode, ParserRecovery, ContextOmitted, BuilderConstruction }`
- Update validator APIs to accept `FieldSpanRef` and branch explicitly.

Migration steps:

1. Add `FieldSpanRef` in validation context.
2. Update all callsites currently writing `Some(Span::from_usize(0,0))`.
3. Prohibit conversion from unknown span to concrete `SourceLocation::new(span)` without explicit policy.

## 6) Empty `ErrorContext` sentinels used in runtime validator paths

High-frequency files:

- `rust/crates/talkbank-model/src/validation/header/structure.rs`
- `rust/crates/talkbank-model/src/validation/utterance/underline.rs`
- `rust/crates/talkbank-model/src/validation/utterance/quotation.rs`
- `rust/crates/talkbank-model/src/validation/utterance/tiers.rs`
- `rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs`

Current pattern:

- `ErrorContext::new("", 0..0, "")`

Risk:

- Conflates "no context available" with "empty context string at span 0..0".

Proposed replacement:

- Use `context: None` for unavailable context.
- Add explicit helper for contextualized errors only when source is real:
  - `ParseError::with_source_context(source_line, rel_span, found)`.

Migration steps:

1. Replace empty-context constructor calls with `None` when source is unavailable.
2. Add targeted source capture where genuinely available.
3. Ensure renderers handle context absence without fallback fabrication.

## 7) `unwrap_or_default()` in tree-sitter extraction suppresses parse state

Files:

- `rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs`
- `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs`
- `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/dependent_tier.rs`
- `rust/crates/talkbank-parser/src/parser/tier_parsers/pho/cst.rs`
- `rust/crates/talkbank-parser/src/parser/tier_parsers/pho/unparsed.rs`
- `rust/crates/talkbank-parser/src/parser/tree_parsing/header/id/fields.rs`

Current pattern:

- UTF-8 extraction fallback silently collapses to `""`.

Risk:

- Information loss and hard-to-debug behavior under invalid UTF-8 or unexpected CST byte slices.

Proposed replacement:

- Replace with typed helper:
  - `fn utf8_text_or_error<'a>(node: Node<'a>, source: &'a str, errors: &impl ErrorSink) -> Result<&'a str, Utf8ExtractError>`
- If caller must continue, explicitly map `Err` to `RecoveredText::Unavailable`.

## 8) Dummy span used as model default constructors

Scope:

- Broadly across `talkbank-model` constructors and derived defaults.

Risk:

- Constructor APIs allow building semantically valid objects with unknown location indistinguishable from real span 0..0.

Proposed replacement:

- For source-tracked types, switch field type from `Span` to explicit source state:
  - `enum SourceSpan { Known(Span), Unknown(UnknownSpanReason) }`
- For types where source is optional by design, use `Option<Span>` and document invariants.

## 9) Alignment placeholders (`None` indexes) are explicit and mostly good

Files:

- `rust/crates/talkbank-model/src/alignment/types.rs`
- `rust/crates/talkbank-model/src/alignment/gra/types.rs`

Assessment:

- This placeholder strategy is type-safe already (explicit `Option`), unlike dummy spans.
- Keep this approach; do not regress to sentinel indices.

Action:

- Confirm no conversion path serializes placeholders ambiguously.

## Medium/Low Findings

- `talkbank-direct-parser` uses dummy `%mor` word/POS placeholders in omission and compound handling (`mor_tier.rs`).
- `talkbank-lsp` checks `is_dummy()` (`validation_cache.rs`) to suppress spans; this should move to explicit unknown span semantics.
- Multiple tests intentionally use `Span::DUMMY`; acceptable in tests but should be tagged as `test-only sentinel`.

## Proposed Target Type Design

These changes should be introduced in phases; they are not implemented in this document.

## A) Source position model

- Add:

```rust
pub enum SourceSpan {
    Known(Span),
    Unknown(UnknownSpanReason),
}

pub enum UnknownSpanReason {
    ParserRecovery,
    SyntheticConstruction,
    ContextUnavailable,
    DerivedValue,
}
```

- Use `SourceSpan` in parser/model paths where `Span::DUMMY` is currently assigned.

## B) Validation context model

- Replace `Option<Span>` sentinel-heavy usage with:

```rust
pub enum FieldSpanRef {
    Known(Span),
    Unknown(UnknownSpanReason),
}
```

- Replace `field_text: Option<String>` fallback-by-empty with:

```rust
pub enum FieldTextRef<'a> {
    Known(&'a str),
    Unknown,
}
```

## C) Recovery objects in parser model

- Introduce recovered variants instead of dummy value injection.
- Attach `reason` + source metadata to each recovered variant.

## D) Diagnostics constructors

- Split constructors into explicit modes:

```rust
ParseError::new_with_source(...)
ParseError::new_without_source(...)
```

- Disallow `ErrorContext::new("", 0..0, "")` in runtime paths via lint/test guard.

## Migration Priority

1. Parser init + finder fake location/context paths (`critical`)
2. `%mor` dummy word recovery (`critical`)
3. Prefix `Span::DUMMY` default and similar parser defaults (`high`)
4. Validation context fallback conversion (`high`)
5. Model constructor `Span::DUMMY` defaults (`high`, broad)
6. LSP/transform consumers of `is_dummy()` (`medium`)
7. Test cleanup/standardization (`low`)

## Test and Acceptance Matrix

1. No runtime diagnostic may include fabricated `0..0` location unless it is a real source span.
2. Unknown source must remain unknown through pipeline, not coerced into dummy values.
3. Parser recovery variants must be distinguishable from valid parsed data in serialized output.
4. `grep`-based guard tests:
- Fail CI on runtime `ErrorContext::new("", 0..0, ...)`.
- Fail CI on runtime `Span::DUMMY` assignment in parser/model modules.
5. Golden tests for parser recovery to ensure recovered variants are emitted and displayed correctly.

## Inventory: Highest-Density Files

(Count = number of deduplicated sentinel hits in this audit)

- `rust/crates/talkbank-model/tests/temporal_validation_tests.rs` (23)
- `rust/crates/talkbank-model/src/validation/header/structure.rs` (7)
- `rust/crates/talkbank-model/src/enhance.rs` (7)
- `rust/crates/talkbank-model/src/alignment/gra/align.rs` (6)
- `rust/crates/talkbank-parser/src/lib.rs` (5)
- `rust/crates/talkbank-model/src/model/header/header_enum/options.rs` (4)
- `rust/crates/talkbank-model/src/model/file/chat_file/core.rs` (4)
- `rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs` (3)
- `rust/crates/talkbank-model/src/validation/utterance/underline.rs` (3)
- `rust/crates/talkbank-model/src/validation/retrace/mod.rs` (3)
- `rust/crates/talkbank-model/src/validation/async_helpers.rs` (3)
- `rust/crates/talkbank-model/src/model/header/header_enum/header.rs` (3)
- `rust/crates/talkbank-model/src/model/dependent_tier/wor.rs` (3)
- `rust/crates/talkbank-model/src/model/annotation/replacement.rs` (3)
- `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/finder.rs` (2)
- `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/finder.rs` (2)
- `rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/tests/mod.rs` (2)
- `rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/parse_word.rs` (2)
- `rust/crates/talkbank-parser/src/api/main_tier.rs` (2)
- `rust/crates/talkbank-parser/src/api/header.rs` (2)
- `rust/crates/talkbank-parser/src/api/file.rs` (2)
- `rust/crates/talkbank-model/src/validation/utterance/quotation.rs` (2)
- `rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs` (2)
- `rust/crates/talkbank-model/src/model/semantic_diff/source_utils.rs` (2)
- `rust/crates/talkbank-model/src/model/semantic_diff/context.rs` (2)
- `rust/crates/talkbank-model/src/model/dependent_tier/text/mod.rs` (2)
- `rust/crates/talkbank-model/src/model/dependent_tier/sin/tier.rs` (2)
- `rust/crates/talkbank-model/src/model/dependent_tier/pho/tier.rs` (2)
- `rust/crates/talkbank-model/src/model/dependent_tier/cod.rs` (2)
- `rust/crates/talkbank-model/src/model/dependent_tier/act.rs` (2)
- `rust/crates/talkbank-model/src/model/content/word/types.rs` (2)
- `rust/crates/talkbank-model/src/model/content/tier_content.rs` (2)
- `rust/crates/talkbank-model/src/model/content/main_tier.rs` (2)
- `rust/crates/talkbank-model/src/model/content/long_feature.rs` (2)
- `rust/crates/talkbank-model/src/model/borrowed/word.rs` (2)
- `rust/crates/talkbank-model/src/model/annotation/annotated.rs` (2)
- `rust/crates/talkbank-model/src/alignment/mor.rs` (2)
- `rust/crates/talkbank-model/src/lib.rs` (2)
- `rust/crates/talkbank-direct-parser/src/mor_tier.rs` (2)
- `rust/tests/full_line_context_test.rs` (1)
- `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/dependent_tier.rs` (1)
- `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/prefix.rs` (1)
- `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs` (1)
- `rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs` (1)
- `rust/crates/talkbank-parser/src/parser/tree_parsing/header/id/fields.rs` (1)
- `rust/crates/talkbank-parser/src/parser/tier_parsers/pho/unparsed.rs` (1)
- `rust/crates/talkbank-parser/src/parser/tier_parsers/pho/cst.rs` (1)
- `rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/helpers.rs` (1)
- `rust/crates/talkbank-parser/src/parser/chat_file_parser/header_dispatch/parse.rs` (1)
- `rust/crates/talkbank-parser/src/parser/chat_file_parser/header_dispatch/finder.rs` (1)
- `rust/crates/talkbank-parser/src/parser/chat_file_parser/chat_file/parse.rs` (1)
- `rust/crates/talkbank-parser/src/parser/chat_file_parser/chat_file/helpers.rs` (1)
- `rust/crates/talkbank-parser/src/api/parser_impl/helpers.rs` (1)
- `rust/crates/talkbank-transform/src/unified_cache/types.rs` (1)
- `rust/crates/talkbank-model/src/validation/utterance/tiers.rs` (1)
- `rust/crates/talkbank-model/src/model/non_empty_string.rs` (1)
- `rust/crates/talkbank-model/src/model/file/line.rs` (1)
- `rust/crates/talkbank-model/src/model/dependent_tier/mor/tier.rs` (1)
- `rust/crates/talkbank-model/src/model/dependent_tier/gra/tier.rs` (1)
- `rust/crates/talkbank-model/src/model/content/postcode.rs` (1)
- `rust/crates/talkbank-model/src/model/content/pause.rs` (1)
- `rust/crates/talkbank-model/src/model/content/nonvocal/simple.rs` (1)
- `rust/crates/talkbank-model/src/model/content/nonvocal/end.rs` (1)
- `rust/crates/talkbank-model/src/model/content/nonvocal/begin.rs` (1)
- `rust/crates/talkbank-model/src/model/content/group.rs` (1)
- `rust/crates/talkbank-model/src/model/content/event.rs` (1)
- `rust/crates/talkbank-model/src/model/content/bullet.rs` (1)
- `rust/crates/talkbank-model/src/model/borrowed/gra.rs` (1)
- `rust/crates/talkbank-lsp/src/backend/validation_cache.rs` (1)
- `rust/crates/talkbank-model/src/tests.rs` (1)
- `rust/crates/talkbank-model/src/span_shift.rs` (1)


## Appendix Files

- Raw strong-pattern inventory: `docs/audits/sentinel-audit-appendix-a.md`
- Tree-sitter placeholder/recovery keyword inventory: `docs/audits/sentinel-audit-appendix-b.md`
- Global sentinel keyword inventory: `docs/audits/sentinel-audit-appendix-c.md`


---

# Sentinel Audit Appendix A

Strong sentinel patterns inventory from `rust/**/*.rs`.

Patterns included:

- `Span::DUMMY`
- `Span::from_usize(0, 0)`
- `SourceLocation::from_offsets(0, 0)`
- `ErrorContext::new("", 0..0, ...)`
- tree-sitter parser `unwrap_or_default()` UTF-8/fallback points
- `SourceLocation::from_offsets(0, input.len()/source.len())` broad range defaults
- explicit `dummy` variable/object creation sites

rust/crates/talkbank-direct-parser/src/mor_tier.rs:231:        let dummy_pos = PartOfSpeech::new(PosCategory::new("omission"));
rust/crates/talkbank-direct-parser/src/mor_tier.rs:232:        let dummy_word = MorWord::new(dummy_pos, MorStem::new("0"));
rust/crates/talkbank-model/src/enhance.rs:118:                .get_or_insert_with(|| crate::ErrorContext::new("", 0..0, ""));
rust/crates/talkbank-model/src/enhance.rs:131:                .get_or_insert_with(|| crate::ErrorContext::new("", 0..0, ""));
rust/crates/talkbank-model/src/enhance.rs:157:            ErrorContext::new("", Span::from_usize(0, 0), ""),
rust/crates/talkbank-model/src/enhance.rs:242:            ErrorContext::new("", Span::from_usize(0, 0), ""),
rust/crates/talkbank-model/src/enhance.rs:265:            ErrorContext::new("", Span::from_usize(0, 0), ""),
rust/crates/talkbank-model/src/enhance.rs:290:            ErrorContext::new("", Span::from_usize(0, 0), ""),
rust/crates/talkbank-model/src/enhance.rs:320:            ErrorContext::new("", Span::from_usize(0, 0), ""),
rust/crates/talkbank-model/src/lib.rs:48:    /// Dummy span for programmatic construction (tests, builders).
rust/crates/talkbank-model/src/lib.rs:77:    pub fn is_dummy(&self) -> bool {
rust/crates/talkbank-model/src/span_shift.rs:9:        if self.is_dummy() {
rust/crates/talkbank-model/src/tests.rs:283:        ErrorContext::new("", Span::from_usize(0, 0), ""),
rust/crates/talkbank-lsp/src/backend/validation_cache.rs:57:    if span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:100:        if !mor.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:105:        if !gra.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:130:        if !mor.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:135:        if !gra.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:60:        if !mor.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:66:        if !gra.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/mor.rs:201:    if !main.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/mor.rs:206:    if !mor.span.is_dummy() {
rust/crates/talkbank-model/src/model/annotation/annotated.rs:111:        let span = context.field_span.unwrap_or(crate::error::Span::DUMMY);
rust/crates/talkbank-model/src/model/annotation/annotated.rs:239:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/annotation/replacement.rs:108:            None => crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/annotation/replacement.rs:259:            None => crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/annotation/replacement.rs:457:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/borrowed/gra.rs:138:            crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/borrowed/word.rs:243:                Span::from_usize(0, 0)
rust/crates/talkbank-model/src/model/borrowed/word.rs:44:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/bullet.rs:72:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/content/event.rs:67:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/content/group.rs:56:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/content/long_feature.rs:108:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/long_feature.rs:186:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/main_tier.rs:115:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/main_tier.rs:116:            speaker_span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/nonvocal/begin.rs:60:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/nonvocal/end.rs:62:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/nonvocal/simple.rs:60:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/pause.rs:133:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/content/postcode.rs:53:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/content/tier_content.rs:376:            None => Span::from_usize(0, 0),
rust/crates/talkbank-model/src/model/content/tier_content.rs:504:        context.field_span = Some(Span::from_usize(0, 0));
rust/crates/talkbank-model/src/model/content/word/types.rs:257:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/content/word/types.rs:405:                None => crate::error::Span::from_usize(0, 0),
rust/crates/talkbank-model/src/model/dependent_tier/act.rs:111:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/act.rs:98:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/cod.rs:110:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/cod.rs:123:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/gra/tier.rs:81:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/mor/tier.rs:102:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/pho/tier.rs:109:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/pho/tier.rs:89:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/sin/tier.rs:102:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/sin/tier.rs:83:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/text/mod.rs:37:                    span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/text/mod.rs:45:                    span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/wor.rs:102:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/wor.rs:62:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/wor.rs:80:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/file/chat_file/core.rs:57:///     Line::Header { header: Header::Utf8, span: Span::DUMMY },
rust/crates/talkbank-model/src/model/file/chat_file/core.rs:58:///     Line::Header { header: Header::Begin, span: Span::DUMMY },
rust/crates/talkbank-model/src/model/file/chat_file/core.rs:59:///     Line::Header { header: Header::Languages { codes: vec![LanguageCode::new("eng")].into() }, span: Span::DUMMY },
rust/crates/talkbank-model/src/model/file/chat_file/core.rs:61:///     Line::Header { header: Header::End, span: Span::DUMMY },
rust/crates/talkbank-model/src/model/file/line.rs:145:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/header/header_enum/header.rs:175:                crate::error::ErrorContext::new("", 0..0, "participants"),
rust/crates/talkbank-model/src/model/header/header_enum/header.rs:259:                crate::error::ErrorContext::new("", 0..0, "options"),
rust/crates/talkbank-model/src/model/header/header_enum/header.rs:91:                crate::error::ErrorContext::new("", 0..0, "languages"),
rust/crates/talkbank-model/src/model/header/header_enum/options.rs:105:            "dummy" => Some(Self::Dummy),
rust/crates/talkbank-model/src/model/header/header_enum/options.rs:115:            Self::Dummy => "dummy",
rust/crates/talkbank-model/src/model/header/header_enum/options.rs:66:/// ## Dummy
rust/crates/talkbank-model/src/model/header/header_enum/options.rs:96:    Dummy,
rust/crates/talkbank-model/src/model/non_empty_string.rs:161:            None => Span::from_usize(0, 0),
rust/crates/talkbank-model/src/model/semantic_diff/context.rs:39:    if span.is_dummy() { None } else { Some(span) }
rust/crates/talkbank-model/src/model/semantic_diff/context.rs:43:    span.filter(|s| !s.is_dummy())
rust/crates/talkbank-model/src/model/semantic_diff/source_utils.rs:13:    if span.is_dummy() {
rust/crates/talkbank-model/src/model/semantic_diff/source_utils.rs:93:    if span.is_dummy() {
rust/crates/talkbank-model/src/validation/async_helpers.rs:186:                span: Span::DUMMY,
rust/crates/talkbank-model/src/validation/async_helpers.rs:190:                span: Span::DUMMY,
rust/crates/talkbank-model/src/validation/async_helpers.rs:194:                span: Span::DUMMY,
rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs:168:                    ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs:86:                    ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/header/structure.rs:137:                                    ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/header/structure.rs:173:                            ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/header/structure.rs:205:                                ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/header/structure.rs:233:                            ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/header/structure.rs:267:                    ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/header/structure.rs:62:                    ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/header/structure.rs:76:                ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/retrace/mod.rs:52:            None => Span::from_usize(0, 0),
rust/crates/talkbank-model/src/validation/retrace/mod.rs:54:        let absolute_span = if !main_tier.span.is_dummy() {
rust/crates/talkbank-model/src/validation/retrace/mod.rs:72:        if !absolute_span.is_dummy() {
rust/crates/talkbank-model/src/validation/utterance/quotation.rs:33:                        ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/utterance/quotation.rs:53:                ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/utterance/tiers.rs:26:                    ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/utterance/underline.rs:35:                            ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/utterance/underline.rs:61:                                        ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/src/validation/utterance/underline.rs:87:                ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:104:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:128:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:132:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:138:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:162:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:166:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:172:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:196:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:200:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:206:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:230:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:234:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:249:                span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:250:                speaker_span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:258:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:34:                span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:38:        span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:39:        speaker_span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:60:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:64:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:70:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:94:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:98:            span: Span::DUMMY,
rust/crates/talkbank-transform/src/unified_cache/types.rs:64:            ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-parser/src/api/file.rs:56:        SourceLocation::from_offsets(0, 0),
rust/crates/talkbank-parser/src/api/file.rs:57:        ErrorContext::new("", 0..0, "TreeSitterParser::new"),
rust/crates/talkbank-parser/src/api/header.rs:36:        SourceLocation::from_offsets(0, 0),
rust/crates/talkbank-parser/src/api/header.rs:37:        ErrorContext::new("", 0..0, "TreeSitterParser::new"),
rust/crates/talkbank-parser/src/api/main_tier.rs:49:        SourceLocation::from_offsets(0, 0),
rust/crates/talkbank-parser/src/api/main_tier.rs:50:        ErrorContext::new("", 0..0, "TreeSitterParser::new"),
rust/crates/talkbank-parser/src/api/parser_impl/helpers.rs:245:            SourceLocation::from_offsets(0, 1),
rust/crates/talkbank-parser/src/lib.rs:126:                SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/lib.rs:146:                SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/lib.rs:166:                SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/lib.rs:186:                SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/lib.rs:222:                SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/parser/chat_file_parser/chat_file/helpers.rs:39:                SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/parser/chat_file_parser/chat_file/parse.rs:36:                    SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/parser/chat_file_parser/header_dispatch/finder.rs:94:        ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-parser/src/parser/chat_file_parser/header_dispatch/parse.rs:44:                        SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/helpers.rs:33:                SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/parse_word.rs:58:            SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/parse_word.rs:75:                SourceLocation::from_offsets(0, input.len()),
rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/tests/mod.rs:27:        SourceLocation::from_offsets(0, 0),
rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/tests/mod.rs:28:        ErrorContext::new("", 0..0, "TreeSitterParser::new"),
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:123:            let dummy = MorWordBorrowed {
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:162:    let dummy = MorWordBorrowed {
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:64:            let dummy_word = MorWordBorrowed {
rust/crates/talkbank-parser/src/parser/tier_parsers/pho/cst.rs:121:    let text = node.utf8_text(source.as_bytes()).unwrap_or_default();
rust/crates/talkbank-parser/src/parser/tier_parsers/pho/unparsed.rs:57:        .unwrap_or_default();
rust/crates/talkbank-parser/src/parser/tree_parsing/header/id/fields.rs:22:            .unwrap_or_default();
rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs:14:    let error_text = node.utf8_text(source.as_bytes()).unwrap_or_default();
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:8:    let error_text = error_node.utf8_text(source.as_bytes()).unwrap_or_default();
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/prefix.rs:16:    let mut speaker_span = Span::DUMMY;
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/finder.rs:32:        SourceLocation::from_offsets(0, 0),
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/finder.rs:33:        ErrorContext::new("", 0..0, ""),
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/finder.rs:32:        SourceLocation::from_offsets(0, source.len()),
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/finder.rs:59:        SourceLocation::from_offsets(0, source.len()),
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/dependent_tier.rs:9:    let error_text = error_node.utf8_text(source.as_bytes()).unwrap_or_default();
rust/tests/full_line_context_test.rs:134:        ErrorContext::new("", Span::from_usize(0, 0), ""),


---

# Sentinel Audit Appendix B

Tree-sitter placeholder/recovery keyword hits.

rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs:83:    // Generic fallback: Show what was found
rust/crates/talkbank-parser/src/parser/tier_parsers/dependent_tier.rs:63:         *CHI:\tdummy .\n\
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/word.rs:31:            // CRITICAL: Check for MISSING nodes before processing
rust/crates/talkbank-parser/src/parser/tier_parsers/pho/groups.rs:8:use super::cst::{build_group_from_words, fallback_group_as_text};
rust/crates/talkbank-parser/src/parser/tier_parsers/pho/groups.rs:53:                        fallback_group_as_text(node, source)
rust/crates/talkbank-parser/src/parser/tier_parsers/pho/groups.rs:59:            _ => fallback_group_as_text(node, source),
rust/crates/talkbank-parser/src/parser/tree_parsing/media_bullet.rs:191:            // DEFAULT: Invalid UTF-8 should still surface a placeholder in error reporting.
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:179:/// Returns `None` if child doesn't exist, kind doesn't match, or node is MISSING (error already reported)
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:181:/// **CRITICAL**: This function checks for MISSING nodes (tree-sitter error recovery placeholders)
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:182:/// and reports them as errors. MISSING nodes have the expected `kind()` but zero-length span.
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:192:        // CRITICAL: Check for MISSING nodes first - these have the expected kind but are placeholders
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:200:                    "Tree-sitter error recovery: MISSING '{}' node inserted at {} position {}",
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:235:/// **CRITICAL**: This function checks for MISSING nodes (tree-sitter error recovery placeholders)
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:245:        // CRITICAL: Check for MISSING nodes - these are placeholders from error recovery
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:253:                    "Tree-sitter error recovery: MISSING '{}' node inserted at {} position {}",
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:279:/// Check if a node is a MISSING placeholder and report error if so
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:281:/// **Purpose:** Inline check for MISSING nodes when not using expect_child helpers
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:283:/// Returns `true` if node is valid (not MISSING), `false` if MISSING (error already reported)
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:292:                "Tree-sitter error recovery: MISSING '{}' node inserted in {}",
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:307:/// **Purpose:** Replace silent fallback extraction with proper error handling
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:314:/// * `fallback` - Fallback text if UTF-8 extraction fails
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:319:/// // If UTF-8 fails, error is reported and fallback is returned
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:326:    fallback: &'a str,
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:343:            fallback
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:64:            let dummy_word = MorWordBorrowed {
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:73:            AnnotatedChunkBorrowed::new(ChunkBorrowed::Word(dummy_word))
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:123:            let dummy = MorWordBorrowed {
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:132:            AnnotatedChunkBorrowed::new(ChunkBorrowed::Word(dummy))
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:162:    let dummy = MorWordBorrowed {
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:171:    AnnotatedChunkBorrowed::new(ChunkBorrowed::Word(dummy))
rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:81:        // CRITICAL: Check for MISSING nodes before processing
rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:113:            // CRITICAL: Check for MISSING nodes
rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:142:            // CRITICAL: Check for MISSING nodes
rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:172:            // CRITICAL: Check for MISSING nodes
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_checking.rs:29:    // Check for MISSING nodes (tree-sitter inserted placeholder for required element)
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/helpers.rs:52:                    // Unknown but non-error node - extract text as fallback
rust/crates/talkbank-parser/src/parser/tier_parsers/gra/tier.rs:77:            // CRITICAL: Check for MISSING nodes before processing
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/tier.rs:50:            // CRITICAL: Check for MISSING nodes before processing
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/word.rs:46:    // CRITICAL: Use expect_child to check for MISSING nodes - prevents fake Word objects
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/replacement/parse.rs:55:                    // CRITICAL: Check for MISSING nodes - tree-sitter error recovery
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/replacement/parse.rs:56:                    // can insert placeholder nodes that still have the expected kind
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/replacement/parse.rs:64:                                "Missing word in replacement at position {} (tree-sitter inserted placeholder)",
rust/crates/talkbank-parser/src/parser/tier_parsers/sin/parse.rs:49:            // CRITICAL: Check for MISSING nodes before processing
rust/crates/talkbank-parser/src/parser/tier_parsers/pho/cst.rs:90:            // CRITICAL: Check for MISSING nodes before processing
rust/crates/talkbank-parser/src/parser/tier_parsers/pho/cst.rs:117:pub(crate) fn fallback_group_as_text<'a>(
rust/crates/talkbank-parser/src/parser/tier_parsers/pho/cst.rs:122:    // DEFAULT: If the CST node contains invalid UTF-8, treat the fallback as empty.
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/ca.rs:42:            // Return a default - BlockedSegments as fallback
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/ca.rs:84:            // Return a default - Faster as fallback
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:130:    // E316: Unparsable content (LOWEST PRIORITY fallback)
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:195:    // CRITICAL: Check for MISSING nodes - tree-sitter error recovery inserts these
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:196:    // as placeholders. A MISSING standalone_word is an internal error.
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:204:                "Internal error: attempted to convert MISSING tree-sitter node at byte {}",
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:208:        // Return empty word - caller should have checked for MISSING
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/contents.rs:37:            // Backward-compatible fallback: accept direct core_content/overlap/separator
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/mod.rs:40:    // CRITICAL: Use expect_child_at to check for MISSING nodes - prevents fake objects
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/content.rs:120:    // CRITICAL: Check for MISSING nodes - prevents creating fake word content from grammar bugs
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/content.rs:127:        // Translate technical MISSING node into user-friendly message


---

# Sentinel Audit Appendix C

Global sentinel keyword scan (`dummy`, `DUMMY`, `is_dummy`, `placeholder`, `sentinel`).
Includes comments/tests/docs mentions for broad coverage.

rust/talkbank-parser-tests/tests/generated/reference_corpus.rs:184:#[case::case_176("corpus/reference/options-dummy.cha")]
rust/crates/talkbank-model/src/validation/context.rs:54:/// - ❌ Standalone validation of Word requires dummy context (use public functions instead)
rust/src/bin/parse-tree.rs:69:            // DEFAULT: Invalid UTF-8 is rendered as a fixed placeholder in the tree dump.
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:34:                span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:38:        span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:39:        speaker_span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:60:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:64:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:70:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:94:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:98:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:104:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:128:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:132:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:138:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:162:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:166:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:172:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:196:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:200:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:206:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:230:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:234:            span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:249:                span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:250:                speaker_span: Span::DUMMY,
rust/crates/talkbank-model/tests/temporal_validation_tests.rs:258:            span: Span::DUMMY,
rust/crates/talkbank-model/src/validation/mod.rs:22://! Currently, validation errors use placeholder source locations `(1, 1)` because
rust/crates/talkbank-model/src/validation/async_helpers.rs:186:                span: Span::DUMMY,
rust/crates/talkbank-model/src/validation/async_helpers.rs:190:                span: Span::DUMMY,
rust/crates/talkbank-model/src/validation/async_helpers.rs:194:                span: Span::DUMMY,
rust/crates/talkbank-model/src/span_shift.rs:9:        if self.is_dummy() {
rust/crates/talkbank-model/src/validation/word/language/resolve.rs:134:                // Return a dummy language to allow validation to continue (it will be reported as error above)
rust/crates/talkbank-model/src/validation/word/language/resolve.rs:157:                // Return a dummy language to allow validation to continue
rust/crates/talkbank-model/src/codes/error_code.rs:88:    MissingRequiredElement, // Tree-sitter inserted MISSING placeholder
rust/crates/talkbank-model/src/validation/retrace/mod.rs:54:        let absolute_span = if !main_tier.span.is_dummy() {
rust/crates/talkbank-model/src/validation/retrace/mod.rs:72:        if !absolute_span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/tests.rs:79:    assert_eq!(alignment.pairs.len(), 3); // 1 valid + 2 placeholders
rust/crates/talkbank-model/src/alignment/gra/tests.rs:84:    // First pair valid, next two are placeholders
rust/crates/talkbank-model/src/alignment/gra/tests.rs:86:    assert!(alignment.pairs[1].is_placeholder());
rust/crates/talkbank-model/src/alignment/gra/tests.rs:87:    assert!(alignment.pairs[2].is_placeholder());
rust/crates/talkbank-model/src/alignment/gra/tests.rs:105:    assert_eq!(alignment.pairs.len(), 3); // 1 valid + 2 placeholders
rust/crates/talkbank-model/src/alignment/gra/tests.rs:110:    // First pair valid, next two are placeholders
rust/crates/talkbank-model/src/alignment/gra/tests.rs:112:    assert!(alignment.pairs[1].is_placeholder());
rust/crates/talkbank-model/src/alignment/gra/tests.rs:113:    assert!(alignment.pairs[2].is_placeholder());
rust/crates/talkbank-model/src/config.rs:141:        // This is a placeholder - in real usage, you'd list actual warning codes
rust/crates/talkbank-model/src/lib.rs:49:    pub const DUMMY: Span = Span { start: 0, end: 0 };
rust/crates/talkbank-model/src/lib.rs:75:    /// Check if this is a dummy span
rust/crates/talkbank-model/src/lib.rs:77:    pub fn is_dummy(&self) -> bool {
rust/crates/talkbank-model/src/alignment/gra/align.rs:9:/// Continues alignment even on mismatch, creating error placeholders.
rust/crates/talkbank-model/src/alignment/gra/align.rs:23:/// 4. Create error placeholders for extras
rust/crates/talkbank-model/src/alignment/gra/align.rs:60:        if !mor.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:66:        if !gra.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:100:        if !mor.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:105:        if !gra.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:113:        // Add placeholders for extra %mor chunks
rust/crates/talkbank-model/src/alignment/gra/align.rs:130:        if !mor.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:135:        if !gra.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/gra/align.rs:143:        // Add placeholders for extra %gra relations
rust/crates/talkbank-model/src/alignment/gra/types.rs:10:    /// `None` in either position indicates a placeholder due to misalignment.
rust/crates/talkbank-model/src/alignment/gra/types.rs:50:    /// Index in %mor chunks (0-indexed, None = placeholder for extra %gra)
rust/crates/talkbank-model/src/alignment/gra/types.rs:53:    /// Index in %gra relations (0-indexed, None = placeholder for extra %mor chunk)
rust/crates/talkbank-model/src/alignment/gra/types.rs:70:    /// Check if this is a placeholder due to misalignment
rust/crates/talkbank-model/src/alignment/gra/types.rs:71:    pub fn is_placeholder(&self) -> bool {
rust/crates/talkbank-model/src/alignment/mor.rs:22:    /// `None` in either position indicates a placeholder due to misalignment.
rust/crates/talkbank-model/src/alignment/mor.rs:128:        // Add placeholders for extra main tier items
rust/crates/talkbank-model/src/alignment/mor.rs:162:        // Add placeholders for extra %mor items
rust/crates/talkbank-model/src/alignment/mor.rs:201:    if !main.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/mor.rs:206:    if !mor.span.is_dummy() {
rust/crates/talkbank-model/src/alignment/sin.rs:59:/// 4. Create error placeholders for extras
rust/crates/talkbank-model/src/alignment/sin.rs:73:    // Handle length mismatch with error placeholders
rust/crates/talkbank-model/src/alignment/sin.rs:165:        assert_eq!(alignment.pairs.len(), 3); // 2 matched + 1 placeholder
rust/crates/talkbank-model/src/alignment/sin.rs:186:        assert_eq!(alignment.pairs.len(), 2); // 1 matched + 1 placeholder
rust/crates/talkbank-model/src/alignment/pho.rs:59:/// 4. Create error placeholders for extras
rust/crates/talkbank-model/src/alignment/pho.rs:73:    // Handle length mismatch with error placeholders
rust/crates/talkbank-model/src/alignment/types.rs:9:/// `None` indicates a placeholder due to misalignment (extra or missing content).
rust/crates/talkbank-model/src/alignment/types.rs:12:    /// Index in the source tier (e.g., main tier), or None for placeholder
rust/crates/talkbank-model/src/alignment/types.rs:14:    /// Index in the target tier (e.g., mor tier), or None for placeholder
rust/crates/talkbank-model/src/alignment/types.rs:31:    /// Check if this is a placeholder due to misalignment
rust/crates/talkbank-model/src/alignment/types.rs:32:    pub fn is_placeholder(&self) -> bool {
rust/crates/talkbank-parser-tests/tests/generated/reference_corpus.rs:184:#[case::case_176("corpus/reference/options-dummy.cha")]
rust/crates/talkbank-cli/tests/cache_tests.rs:3://! Note: Early placeholder tests have been replaced by full implementations below.
rust/crates/talkbank-lsp/src/backend/validation_cache.rs:57:    if span.is_dummy() {
rust/crates/talkbank-model/src/model/annotation/replacement.rs:108:            None => crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/annotation/replacement.rs:259:            None => crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/annotation/replacement.rs:457:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/annotation/annotated.rs:110:        // DEFAULT: Missing span indicates unknown location; use dummy span for diagnostics.
rust/crates/talkbank-model/src/model/annotation/annotated.rs:111:        let span = context.field_span.unwrap_or(crate::error::Span::DUMMY);
rust/crates/talkbank-model/src/model/annotation/annotated.rs:239:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/header/header_enum/options.rs:105:            "dummy" => Some(Self::Dummy),
rust/crates/talkbank-model/src/model/header/header_enum/options.rs:115:            Self::Dummy => "dummy",
rust/crates/talkbank-model/src/model/semantic_diff/context.rs:39:    if span.is_dummy() { None } else { Some(span) }
rust/crates/talkbank-model/src/model/semantic_diff/context.rs:43:    span.filter(|s| !s.is_dummy())
rust/crates/talkbank-model/src/model/dependent_tier/sin/tier.rs:83:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/sin/tier.rs:102:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/pho/tier.rs:89:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/pho/tier.rs:109:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/header/header_enum/header.rs:437:    /// @Options:\toptions (CA, CA-Unicode, bullets, dummy)
rust/crates/talkbank-model/src/model/borrowed/word.rs:44:            span: Span::DUMMY,
rust/crates/talkbank-parser/src/parser/chat_file_parser/chat_file/tests.rs:224:    // Check that span is not DUMMY (0..0)
rust/crates/talkbank-parser/src/parser/chat_file_parser/chat_file/tests.rs:299:    // Check that error spans are not DUMMY
rust/crates/talkbank-model/src/model/semantic_diff/source_utils.rs:13:    if span.is_dummy() {
rust/crates/talkbank-model/src/model/semantic_diff/source_utils.rs:93:    if span.is_dummy() {
rust/crates/talkbank-model/src/model/dependent_tier/cod.rs:110:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/cod.rs:123:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/act.rs:98:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/act.rs:111:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/wor.rs:62:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/wor.rs:80:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/wor.rs:102:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/borrowed/gra.rs:138:            crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/gra/tier.rs:81:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/mor/tier.rs:102:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/mor/tests.rs:32:    let word = MorWord::new(PartOfSpeech::new("v"), "dummy");
rust/crates/talkbank-model/src/model/dependent_tier/text/mod.rs:37:                    span: Span::DUMMY,
rust/crates/talkbank-model/src/model/dependent_tier/text/mod.rs:45:                    span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/group.rs:56:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/content/event.rs:67:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-direct-parser/src/mor_tier.rs:230:        // Create a dummy word for omission
rust/crates/talkbank-direct-parser/src/mor_tier.rs:554:                    // Use a dummy POS - will be replaced by compound's overall POS
rust/crates/talkbank-model/src/model/content/pause.rs:133:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/file/chat_file/core.rs:57:///     Line::Header { header: Header::Utf8, span: Span::DUMMY },
rust/crates/talkbank-model/src/model/file/chat_file/core.rs:58:///     Line::Header { header: Header::Begin, span: Span::DUMMY },
rust/crates/talkbank-model/src/model/file/chat_file/core.rs:59:///     Line::Header { header: Header::Languages { codes: vec![LanguageCode::new("eng")].into() }, span: Span::DUMMY },
rust/crates/talkbank-model/src/model/file/chat_file/core.rs:61:///     Line::Header { header: Header::End, span: Span::DUMMY },
rust/crates/talkbank-direct-parser/src/dependent_tier.rs:20:    // empty content gracefully by using a space placeholder
rust/crates/talkbank-model/src/model/content/long_feature.rs:108:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/long_feature.rs:186:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/nonvocal/simple.rs:60:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/file/line.rs:145:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/postcode.rs:53:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:123:            let dummy = MorWordBorrowed {
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:132:            AnnotatedChunkBorrowed::new(ChunkBorrowed::Word(dummy))
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:162:    let dummy = MorWordBorrowed {
rust/crates/talkbank-parser/src/parser/tier_parsers/mor/item.rs:171:    AnnotatedChunkBorrowed::new(ChunkBorrowed::Word(dummy))
rust/crates/talkbank-model/src/model/content/nonvocal/begin.rs:60:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/nonvocal/end.rs:62:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/main_tier.rs:115:            span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/main_tier.rs:116:            speaker_span: Span::DUMMY,
rust/crates/talkbank-model/src/model/content/bullet.rs:72:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-model/src/model/content/word/types.rs:257:            span: crate::error::Span::DUMMY,
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_checking.rs:29:    // Check for MISSING nodes (tree-sitter inserted placeholder for required element)
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/prefix.rs:16:    let mut speaker_span = Span::DUMMY;
rust/crates/talkbank-parser/src/parser/tree_parsing/media_bullet.rs:191:            // DEFAULT: Invalid UTF-8 should still surface a placeholder in error reporting.
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:181:/// **CRITICAL**: This function checks for MISSING nodes (tree-sitter error recovery placeholders)
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:192:        // CRITICAL: Check for MISSING nodes first - these have the expected kind but are placeholders
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:235:/// **CRITICAL**: This function checks for MISSING nodes (tree-sitter error recovery placeholders)
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:245:        // CRITICAL: Check for MISSING nodes - these are placeholders from error recovery
rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:279:/// Check if a node is a MISSING placeholder and report error if so
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:196:    // as placeholders. A MISSING standalone_word is an internal error.
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/replacement/parse.rs:56:                    // can insert placeholder nodes that still have the expected kind
rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/replacement/parse.rs:64:                                "Missing word in replacement at position {} (tree-sitter inserted placeholder)",
