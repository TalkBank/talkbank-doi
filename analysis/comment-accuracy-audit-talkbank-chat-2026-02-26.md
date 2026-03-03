# talkbank-chat comment accuracy audit (2026-02-26)

## scope

- `talkbank-chat/crates/talkbank-tree-sitter-parser`
- `talkbank-chat/crates/talkbank-model`

## findings fixed in this pass

1. `parser/chat_file_parser/chat_file/normalize.rs`
   - corrected CA omission comments:
   - removed incorrect `[= ...]` framing.
   - removed incorrect example implying `//` is the user-facing omission form.
   - clarified this module only normalizes CA-omission token shape and does **not** implement broader CA validation policy.
   - clarified CA mode has broader downstream effects (including terminator policy) handled in validation modules.
2. `parser/chat_file_parser/single_item/tests/utterance.rs`
   - replaced mechanical comments (`Runs ...`) with concrete behavior-focused comments.
3. `parser/tree_parsing/main_tier/content/base/long_feature.rs`
   - removed incorrect claim that begin/end matching is verified in this parser.
   - clarified this file validates local token shape only; cross-token pairing belongs to validation.
4. `parser/tree_parsing/main_tier/structure/convert/body.rs`
   - corrected linker description (`utterance linkers`) and removed incorrect `%mor` framing.
5. `parser/tree_parsing/main_tier/structure/convert/mod.rs`
   - corrected "optional CA bullet" phrasing to "optional media bullet".
6. `parser/tree_parsing/main_tier/content/base/nonvocal.rs`
   - corrected marker examples to actual grammar tokens (`&{n=...}`, `&}n=...`, `&{n=...}`).
7. `parser/tree_parsing/main_tier/content/base/long_feature.rs`
   - corrected marker examples to actual grammar tokens (`&{l=...}`, `&}l=...`).
8. `tests/test_parse_health_recovery.rs` (tree-sitter + direct parser)
   - replaced mechanical helper/test comments with behavior-specific phrasing for parse-health taint and recovery policy.
9. `tests/test_debug_error_group.rs`
   - clarified that `<...> [*]` cases are expected to parse as annotated groups with inner words.
10. `talkbank-direct-parser/tests/golden_unit_tests.rs`
   - replaced placeholder helper comments with explicit golden-reference equivalence intent (word/main/%com/%mor/%pho).
11. `parser/chat_file_parser/single_item/tests/**`
   - replaced low-signal `Runs ...` comments in `mod.rs`, `word/basic.rs`, `word/shortening.rs`,
     `word/cleaned_text_overlap.rs`, `word/cleaned_text_markers.rs`, `word/categories.rs`,
     and `word/untranscribed.rs` with parser-behavior descriptions.
12. `parser/tier_parsers/dependent_tier.rs` (snapshot test module)
   - replaced placeholder test comments with explicit behaviors (custom label retention, empty-content rejection, whitespace tolerance).
13. `talkbank-direct-parser/tests/snapshot_tests.rs`
   - replaced all placeholder snapshot helper/test comments with explicit snapshot intent (word/file, marker family, tier coverage).
14. `talkbank-parser-tests/tests/property_tests_modules/**`
   - replaced placeholder property/round-trip comments with behavior-specific language across:
     `error_scenarios.rs`, `raw_text.rs`, `round_trip/{mor,headers,word,main_tier,utterance,dependent_tiers}.rs`,
     `combinations.rs`, `cleaned_text.rs`, `shortening.rs`, `mod.rs`, `categories.rs`,
     `error_messages.rs`, `word_parsing.rs`, and `form_types.rs`.
15. `talkbank-parser-tests/tests/{validation_error_corpus,warning_corpus,parse_error_corpus,golden_words_validation,offset_tests,direct_parser_roundtrip_corpus,error_words_validation,parser_suite,dev_equivalence}.rs`
   - replaced placeholder comments in corpus harnesses, parser-suite helpers, and dev-equivalence tests with backend-accurate intent.
16. `parser/tree_parsing/main_tier/content/errors.rs`
   - replaced offset-helper test placeholder comments with explicit detection expectations.
17. repository-wide non-generated sweep (`talkbank-chat/crates/**`, excluding generated tests)
   - confirmed no remaining `/// Runs ...` comments via grep.

## high-risk areas to audit next

1. `talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/**`
   - verify comments mentioning CA semantics or terminator policy are scoped to parsing only.
2. `talkbank-tree-sitter-parser/src/parser/chat_file_parser/**`
   - verify comments about normalization versus validation responsibilities.
3. `talkbank-model/src/model/content/word/**` and `validation/**`
   - ensure comments about CA omission and terminator policy match actual enforcement points.

## audit rule

- Comments must describe only what the function/module actually does.
- If behavior lives elsewhere (validation vs normalization), comment must say so explicitly.
- CA-related comments must distinguish:
  - representation (`(word)` / `WordCategory::CAOmission` / `WordContent::Shortening`)
  - policy (e.g., missing terminator allowed in CA mode) enforced in validation layers.
