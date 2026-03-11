# Validation Completion Audit

**Date**: 2026-02-12
**Prior Art**: public CHECK parity notes in this book, an earlier Java validation audit, and an internal normalized transcription of CLAN CHECK rules. Maintainer-only provenance notes for the local source artifacts are archived privately.

---

## Executive Summary

This audit consolidates the three prior validation reference documents and assesses the
current state of Rust CHAT validation in `talkbank-tools` as of February 2026.

| Metric | Count |
|--------|-------|
| CLAN CHECK rules total | 87 |
| CHECK rules fully covered in Rust | 70 |
| CHECK rules genuinely missing in Rust | 12 |
| CHECK rules handled by grammar implicitly | 5 |
| Java-only validations (gaps in Rust) | 4 minor |
| Rust-only enhancements beyond CHECK/Java | 50+ |
| Error codes defined in Rust | 107 |
| Test fixtures (error corpus) | 101+ |
| Spec-based error definitions | 62 |

**Overall Assessment**: based on this February 2026 audit, Rust validation
covers the same practical cases as the older Java validator while adding many
extra checks of its own. Against CLAN CHECK, there are still documented gaps:
12 genuinely unimplemented CHECK rules (mostly depfile-related or legacy format
constraints), plus 5 cases that the tree-sitter grammar appears to handle
implicitly rather than through dedicated validation code.

---

## Part 1: CLAN CHECK Rules — Gap Analysis

### 1.1 CHECK Rules Fully Covered (70 of 87)

These CHECK rules have full semantic parity in the Rust codebase. 13 of these have
intentional *behavioral* divergences where CHECK uses flawed counter/toggle algorithms
and Rust implements correct stack-based nesting (see CHECK-rules.md Anomaly 1–4).

| CHECK # | Message | Rust Codes | Notes |
|---:|---|---|---|
| 3 | Missing TAB/SPACE after tier | E243, W210, W211 | |
| 4 | Space instead of TAB after tier name | E243, W210, W211 | |
| 6 | @Begin missing | E501 | |
| 7 | @End missing | E502 | |
| 9 | Tier name too long (SPEAKERLEN) | E308, E522, E532 | |
| 12 | Missing speaker name/role | E308, E522, E532 | |
| 13 | Duplicate speaker declaration | E308, E522, E532 | |
| 14 | Spaces before tier code | E243, W210, W211 | |
| 16 | Extended chars in speaker names | E308, E522, E532 | |
| 18 | Speaker not in @Participants | E308, E522 | |
| 19 | Delimiter in word / missing space | E243, E304, E305, E360 | |
| 21 | Utterance delimiter expected | E304 | |
| 22–27 | Unmatched brackets `[]<>{}` | E345–E348, E230, E231, E242, E356, E357 | Intentional divergence: stack-based |
| 28–29 | Unmatched `()` | E230, E231, E345, E346 | CHECK disabled since 1998 |
| 31 | Missing text after colon | E305 | |
| 36 | Delimiter not at end | E305 | |
| 38 | Numbers should be written out | E220 | |
| 40 | Duplicate dependent tier | E401, E705, E706, E720 | |
| 41 | Parentheses around words | E212 | |
| 47 | Numbers inside words | E220 | |
| 50 | Redundant utterance delimiter | E305 | |
| 51 | `<>` not followed by `[]` | E347, E348 | |
| 52 | Item must be preceded by text | E370 | |
| 55–56 | Unmatched `()` in word | E231 | |
| 57 | Space between word and pause | E243 | |
| 60 | @ID missing | E522 | |
| 68 | CHI Target_Child missing | E522, E523, E524 | |
| 69 | UTF8 header missing | E507 | |
| 70 | Expected text or "0" on tier | E253 | |
| 81 | Bullet position rules | E360 | |
| 82 | Bullet BEG < END | E361 | |
| 83 | BEG time monotonicity | E362, E701 | |
| 84 | BEG < previous END overlap | E704 | |
| 85 | Gap between bullets | E700 | |
| 89–90 | Bullet format/time errors | E360, E361 | |
| 91 | Blank lines not allowed | E303 | |
| 92 | Item must be followed by space | E243, W210, W211 | |
| 93 | Item must be preceded by space | E243, W210, W211 | |
| 94 | %mor vs speaker delimiter mismatch | E705, E706, E714–E720 | |
| 110 | No bullet on tier | E360 | |
| 117 | CA chars must be in pairs | E230, E356, E357 | Intentional divergence: stack-based |
| 118 | Delimiter must precede bullet | E360 | |
| 120 | Use 3-letter language code | E248 | |
| 121 | Unknown language code | E254, E519 | |
| 122 | @ID language not in @Languages | E519 | |
| 128–131 | Unmatched `‹›〔〕` | E345, E346 | Intentional divergence: stack-based |
| 136–137 | Unmatched `""` | E242 | Intentional divergence: stack-based |
| 140 | %MOR size mismatch | E401, E705, E706, E720 | |
| 141 | `[: ...]` must follow single word | E387–E389 | |
| 142 | @ID vs @Participants role mismatch | E532 | |
| 143 | @ID needs 10 fields | E505 | |
| 151 | Word has only repetition | E370 | |
| 153 | Age month/day missing leading zero | E517 | |
| 155 | Use "0word" not "(word)" | E212 | |
| 156 | Replace `,,` with `‚` | E243 | |
| 159 | Pause after retrace, not before | E370 | |
| 160 | No space after `<` or before `>` | E243, W210, W211 | |
| 161 | Space required before `[` code | E243, W210, W211 | |

### 1.2 CHECK Rules Handled by Grammar (5 of 87)

These rules are enforced by the tree-sitter grammar rather than explicit validation code.
The grammar simply cannot produce a parse tree for these violations — they surface as parse
errors (E310, E316, E319, etc.).

| CHECK # | Message | How Grammar Handles It |
|---:|---|---|
| 1 | Expected `@` or `%` or `*` | Grammar only matches lines starting with `@`/`*`/`%`; anything else is ERROR node |
| 8 | Expected `@ % * TAB` | Same as CHECK 1 |
| 43 | File must start with @Begin | Grammar structure requires `@Begin` as first tier |
| 44 | File must end with @End | Grammar structure requires `@End` as last tier |
| 15 | Illegal role | Validation via E532 + grammar accepts only alpha roles |

### 1.3 CHECK Rules Genuinely Missing in Rust (12 of 87)

These are CHECK rules with no equivalent Rust validation. They need to be triaged
for whether to implement, skip (legacy), or handle differently.

| CHECK # | Message | Category | Assessment |
|---:|---|---|---|
| **2** | Missing `:` and argument | File Structure | **Already handled**: Grammar requires colon in tier structure |
| **5** | Colon is illegal (on non-colon tier) | File Structure | **Already handled**: Grammar handles tier structure |
| **10** | Tier text > UTTLINELEN (18000 chars) | File Structure | **Skip**: No length limit needed |
| **11** | Symbol not declared in depfile | Depfile | **Skip**: No depfile concept in Rust tooling |
| **17** | Tier not declared in depfile | Depfile | **Skip**: No depfile concept in Rust tooling |
| **20** | Undeclared suffix in depfile | Depfile | **Skip**: No depfile concept in Rust tooling |
| **30** | Text is illegal | Generic | **Already handled**: Grammar rejects invalid line structure |
| **32** | Code not declared in depfile | Depfile | **Skip**: No depfile concept in Rust tooling |
| **48** | Illegal character(s) found | Content | **Already handled**: `check_word_characters()` rejects control chars via E243 |
| **58** | Tier name > 8 characters | Structure | **Implement as warning**: `MAX_SPEAKER_ID_LENGTH=7` exists but unused |
| **102** | Italic markers illegal | Format | **Already handled**: Italic markers removed from grammar |
| **107** | Only single commas allowed | Format | **Implement**: Detect consecutive commas (should use `‚`) |

#### Depfile Rules (CHECK 11, 17, 20, 32)

CLAN's CHECK validates against a "depfile" (`depfile.cut`) which declares legal tiers,
symbols, suffixes, and codes. This is a CLAN-specific concept. The Rust tooling does not
use depfiles — it validates against the CHAT specification directly. These 4 rules are
**intentionally not applicable** to the Rust implementation.

### 1.4 Summary of CHECK Parity

| Status | Count |
|--------|-------|
| Fully implemented (semantic parity) | 70 |
| Handled by grammar | 7 |
| Depfile-specific (intentionally N/A) | 4 |
| Genuinely missing (should implement) | 2 (CHECK 58, 107) |
| Skipped (legacy/disabled/vague) | 4 (CHECK 5, 10, 28/29, 30) |

---

## Part 2: Java Chatter Gaps

From the `VALIDATION_AUDIT.md`, 4 minor Java-only validations were identified:

| Validation | Java Location | Rust Status | Assessment |
|---|---|---|---|
| Capital prefix whitelist (Mac, Mc, De, La, etc.) | `Languages.java:57–72` | Not implemented | **Low priority**: Affects only capitalization in proper names. Grammar handles compounds. |
| Colon after `@` form marker | `Chat.flex` | Not implemented | **Grammar handles**: Tree-sitter rejects this at parse level |
| User-defined tier name ≤ 7 chars | `Chat.flex`, `XmlParser.java` | Not implemented | **Same as CHECK 58**: Implement as warning |
| Minutes must not start with 0 | `ChatWalker.g` | Not implemented | **Trivial**: Edge case in time format |

**All 4 are low-priority.** One overlaps with CHECK 58 (tier name length).

---

## Part 3: Rust-Only Enhancements (Beyond CHECK and Java)

### 3.1 Structural Validation (Rust-only)

| Error Code | Name | Description |
|---|---|---|
| E341 | UnbalancedQuotationCrossUtterance | Quotation spans across utterance boundaries |
| E344 | InvalidScopedAnnotationNesting | Annotations nested incorrectly |
| E345/E346 | UnmatchedScopedAnnotation Begin/End | Stack-based (not counter-based) bracket matching |
| E355 | InterleavedScopedAnnotations | Overlapping but not nested annotations |
| E356/E357 | UnmatchedUnderlineBegin/End | Underline delimiter balance |
| E358/E359 | UnmatchedLongFeatureBegin/End | Long feature delimiter balance |
| E366 | LongFeatureLabelMismatch | Begin/end labels don't match |
| E367/E368/E369 | UnmatchedNonvocal + LabelMismatch | Nonvocal event matching |
| E373 | InvalidOverlapIndex | Overlap index validation |
| E404 | OrphanedDependentTier | Dependent tier without main tier |

### 3.2 Phonological Validation (Rust-only)

| Error Code | Name | Description |
|---|---|---|
| E244 | ConsecutiveStressMarkers | Two stress markers in a row |
| E245 | StressNotBeforeSpokenMaterial | Stress marker misplaced |
| E246 | LengtheningNotAfterSpokenMaterial | Lengthening marker misplaced |
| E247 | MultiplePrimaryStress | More than one primary stress per word |
| E250 | SecondaryStressWithoutPrimary | Secondary without primary stress |
| E252 | SyllablePauseNotBetweenSpokenMaterial | Syllable pause position |

### 3.3 Header Validation (Rust-only)

| Error Code | Name | Description |
|---|---|---|
| E525 | UnknownHeader | Unrecognized `@` header name |
| E533 | EmptyOptionsHeader | Empty `@Options` field |
| E526–E530 | Gem validation suite | @Bg/@Eg/@G scoping and label matching |

### 3.4 Alignment Validation (Rust-only)

| Error Code | Name | Description |
|---|---|---|
| E718/E719 | SinCountMismatch | `%sin` tier count alignment |
| E720 | MorGraCountMismatch | `%mor` vs `%gra` cross-check |
| E708/E709 | MalformedGrammarRelation/Index | Grammar tier structure validation |
| E712/E713 | GraInvalidWord/HeadIndex | Grammar relation index validation |

### 3.5 Temporal Validation (Rust-only)

| Error Code | Name | Description |
|---|---|---|
| E704 | SpeakerSelfOverlap | Same speaker overlapping > 500ms (CLAN Error 133) |
| E360 | InvalidMediaBullet | Comprehensive bullet format validation |

### 3.6 %wor Tier Alignment (NEW — Rust-only)

The `%wor` tier is a **new tier type** for word-level timing, developed in collaboration
with Python batchalign. This is entirely absent from both CLAN CHECK and Java Chatter.

**What %wor does differently from %mor and %pho**:

| Aspect | %wor | %mor | %pho |
|--------|------|------|------|
| Retraced words `[/]` | **Included** (were spoken) | Excluded | Included |
| Fillers `&-um` | **Included** | Excluded | Included |
| Nonwords `&~gaga` | Excluded | Excluded | Included |
| Fragments `&+fr` | Excluded | Excluded | Included |
| Timing metadata | Excluded (it's metadata, not content) | N/A | N/A |
| Groups `‹...›` | Flattened | Flattened | Counted as 1 |
| Alignment model | 1-to-1 word pairs | 1-to-1 word pairs | 1-to-1 word pairs |

**Implementation**: `talkbank-model/src/alignment/wor.rs` (168 lines)
**Rationale**: %wor aligns to what was actually **spoken** (including retraces — they were
vocalized), while %mor aligns to what was **meant** (excluding retraces).

### 3.7 Warnings (Rust-only)

| Warning Code | Name | Description |
|---|---|---|
| W210 | MissingWhitespaceBeforeContent | Spacing issues |
| W211 | MissingWhitespaceAfterOverlap | Spacing issues |
| W601 | EmptyUserDefinedTier | Empty `%xxx:` tier |
| W602 | UnknownUserDefinedTier | Unrecognized tier name |

---

## Part 4: Intentional Divergences

### 4.1 Bracket/Delimiter Matching

**CLAN CHECK** uses simple counters (Anomaly 1) and toggles (Anomaly 2) that allow
invalid nesting like `]text[` or `∆a ∆b` to pass. Rust uses proper stack-based
nesting validation. This is an **intentional improvement** — semantic intent is
preserved while fixing known CHECK bugs.

Affected CHECK rules: 22–29, 117, 128–131, 136–137

### 4.2 Disabled CHECK Rules

CHECK has disabled validations that Rust does NOT replicate:
- **CHECK 28/29**: Parenthesis balance disabled since 1998-11-02
- **CHECK 118**: Bullet-after-CA-delimiter disabled since 2025-07-04
- Uppercase in words disabled since 2019-04-23

### 4.3 `%com` Tier Exemptions

Since 2025-07-07, CHECK exempts `%com:` tiers from angle bracket validation
(errors 24/25). Rust currently validates all tiers uniformly.

**Decision**: Not needed — Rust doesn't validate content of unparsed dependent tiers at all.
This is a more principled approach than CHECK's "validate everything then carve out exceptions."

---

## Part 5: Test Coverage Assessment

### 5.1 Current Coverage

| Test Type | Count | Coverage |
|-----------|-------|----------|
| Error corpus fixtures (`.cha` files) | 101+ | E2xx, E3xx, E4xx, E5xx, E7xx, Wxx |
| Error specifications | 62 | 59 auto-generated, 3 manual |
| Validation unit tests (in-crate) | 6 modules | word, utterance, cross-utterance, header, retrace, language |
| Integration tests | 10+ files | comprehensive, speaker, participant, alignment, roundtrip |
| Mutation tests | 3 modules | headers, words, alignment |
| Reference corpus | 340 files | 100% roundtrip pass |

### 5.2 Coverage Gaps

**Missing test coverage for existing error codes**:
- E315 InvalidControlCharacter — spec exists, no test fixture
- E363 InvalidPostcode — no spec, no test
- E373 InvalidOverlapIndex — no spec, no test
- E533 EmptyOptionsHeader — no spec, no test

**New rules needing tests**:
- Double commas (CHECK 107) — new error code + tests
- Tier name length (CHECK 58) — new warning code + tests

---

## Part 6: Implementation Roadmap

### Phase 1: Close CHECK Parity Gaps (2 rules)

| Priority | CHECK # | Description | New Error Code | Effort |
|----------|---------|-------------|----------------|--------|
| High | 107 | Double commas | E258 (new) | Small |
| Medium | 58 | Speaker ID > 7 chars | W603 (new) | Small |

### Phase 2: Fill Test Gaps

Add dedicated test fixtures for existing error codes that lack them
(E315, E363, E373, E533).

---

## Appendix A: Error Code Registry (Complete)

### E0xx–E1xx: Internal
E001 InternalError, E002 TestError, E003 EmptyString, E101 InvalidLineFormat

### E2xx: Word
E202 MissingFormType, E203 InvalidFormType, E207 UnknownAnnotation,
E208 EmptyReplacement, E209 EmptySpokenContent, E210 (deprecated),
E211 (deprecated), E212 InvalidWordFormat, E213 (deprecated),
E214 EmptyAnnotatedScopedAnnotations, E220 IllegalDigits,
E230 UnbalancedCADelimiter, E231 UnbalancedShortening,
E232 InvalidCompoundMarkerPosition, E233 EmptyCompoundPart,
E241 IllegalUntranscribed, E242 UnbalancedQuotation,
E243 IllegalCharactersInWord, E244 ConsecutiveStressMarkers,
E245 StressNotBeforeSpokenMaterial, E246 LengtheningNotAfterSpokenMaterial,
E247 MultiplePrimaryStress, E248 TertiaryLanguageNeedsExplicitCode,
E249 MissingLanguageContext, E250 SecondaryStressWithoutPrimary,
E251 EmptyWordContentText, E252 SyllablePauseNotBetweenSpokenMaterial,
E253 EmptyWordContent, E254 UndeclaredLanguageCode

### E3xx: Parser/Annotation
E301 MissingMainTier, E302 MissingNode, E303 SyntaxError,
E304 MissingSpeaker, E305 MissingTerminator, E306 EmptyUtterance,
E307 InvalidSpeaker, E308 UndeclaredSpeaker, E309 UnexpectedSyntax,
E310 ParseFailed, E311 UnexpectedNode, E312 UnclosedBracket,
E313 UnclosedParenthesis, E314 IncompleteAnnotation,
E315 InvalidControlCharacter, E316 UnparsableContent,
E317 UnparsableFileContent, E318 UnparsableDependentTier,
E319 UnparsableLine, E320 UnparsableHeader, E321 UnparsableUtterance,
E322 EmptyColon, E323 MissingColonAfterSpeaker,
E324 UnrecognizedUtteranceError, E325 UnexpectedUtteranceChild,
E326 UnexpectedLineType, E330 TreeParsingError,
E331 UnexpectedNodeInContext, E340 UnknownBaseContent,
E341 UnbalancedQuotationCrossUtterance, E342 MissingRequiredElement,
E344 InvalidScopedAnnotationNesting,
E345 UnmatchedScopedAnnotationBegin, E346 UnmatchedScopedAnnotationEnd,
E347 UnbalancedOverlap, E348 MissingOverlapEnd,
E350 GenericAnnotationError, E351 MissingQuoteBegin, E352 MissingQuoteEnd,
E353 MissingOtherCompletionContext, E354 MissingTrailingOffTerminator,
E355 InterleavedScopedAnnotations,
E356 UnmatchedUnderlineBegin, E357 UnmatchedUnderlineEnd,
E358 UnmatchedLongFeatureBegin, E359 UnmatchedLongFeatureEnd,
E360 InvalidMediaBullet, E361 InvalidTimestamp,
E362 TimestampBackwards, E363 InvalidPostcode,
E364 MalformedWordContent, E365 MalformedTierContent,
E366 LongFeatureLabelMismatch,
E367 UnmatchedNonvocalBegin, E368 UnmatchedNonvocalEnd,
E369 NonvocalLabelMismatch, E370 StructuralOrderError,
E371 PauseInPhoGroup, E372 NestedQuotation,
E373 InvalidOverlapIndex, E374–E378 Parse errors,
E380 UnknownSeparator, E381–E386 Tier-specific parse errors,
E387 ReplacementOnFragment, E388 ReplacementOnNonword,
E389 ReplacementOnFiller, E390 ReplacementContainsOmission,
E391 ReplacementContainsUntranscribed

### E4xx: Dependent Tier
E401 DuplicateDependentTier, E404 OrphanedDependentTier

### E5xx: Header
E501 DuplicateHeader, E502 MissingEndHeader,
E504 MissingRequiredHeader, E505 InvalidIDFormat,
E506 EmptyParticipantsHeader, E507 EmptyLanguagesHeader,
E508 EmptyDateHeader, E509 EmptyMediaHeader,
E510–E515 Empty ID fields, E516 EmptyDate,
E517 InvalidAgeFormat, E518 InvalidDateFormat, E519 InvalidLanguageCode,
E522 SpeakerNotDefined, E523 OrphanIDHeader, E524 BirthUnknownParticipant,
E525 UnknownHeader, E526–E530 Gem validation,
E531 MediaFilenameMismatch, E532 InvalidParticipantRole,
E533 EmptyOptionsHeader

### E6xx: Tier
E600 TierValidationError, E601 InvalidDependentTier,
E602 MalformedTierHeader, E604 GraWithoutMor

### E7xx: Temporal/Alignment
E700 UnexpectedTierNode, E701 TierBeginTimeNotMonotonic,
E702 InvalidMorphologyFormat, E703 UnexpectedMorphologyNode,
E704 SpeakerSelfOverlap, E705/E706 MorCountMismatch,
E708/E709 GrammarRelation errors, E710 UnexpectedGrammarNode,
E712/E713 GraIndex errors,
E714/E715 PhoCountMismatch, E718/E719 SinCountMismatch,
E720 MorGraCountMismatch

### Warnings
W001 GenericWarning, W108 SpeakerNotFoundInParticipants,
W210 MissingWhitespaceBeforeContent, W211 MissingWhitespaceAfterOverlap,
W601 EmptyUserDefinedTier, W602 UnknownUserDefinedTier, W999 LegacyWarning

---

## Appendix B: Cross-Reference Tables

### CHECK Rule → Rust Error Code

See Part 1 tables above.

### Java Validation → Rust Error Code

The full legacy Java-to-Rust mapping matrix is preserved in maintainer archive notes.
All Java validations map to Rust codes except the 4 minor gaps listed in Part 2.

---

*Last updated: 2026-02-12*
