# Error Code Audit Report

This report audits all `ErrorCode` variants defined in `talkbank-model`.
Scope: all enum codes, including deprecated and disabled/ignored paths.

## Summary

- Total codes audited: 171
- Coverage: `Strong 60`, `Partial 0`, `Disabled-only 0`, `None 111`
- Name/message fit needing review: 40
- Messages needing user-language improvement: 45
- Suggestions missing/weak: 141

## Method

- Inventory source: `rust/crates/talkbank-model/src/codes/error_code.rs`
- Emission evidence: static scan of `ParseError` creation/report sites in Rust source.
- Coverage evidence: assertions/snapshots/corpus mentions in Rust test code.
- Ratings are heuristic and intended to identify manual follow-up hotspots.

## E0xx-E1xx Internal/System

| Code | Name | Primary Construct | Referenced Constructs | Name/Message Fit | User-Friendly Message | Suggested Fixes | Coverage |
|---|---|---|---|---|---|---|---|
| `E001` | `InternalError` | Generic/Internal | %sin alignment | Good fit | Good | Sensible | None |
| `E002` | `TestError` | Generic/Internal | %sin alignment | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E003` | `EmptyString` | Generic/Internal | %sin alignment | Good fit | Good | Missing | None |
| `E101` | `InvalidLineFormat` | Direct parser | form-type markers | Good fit | Good | Missing | None |

### `E001` `InternalError`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `wrapped content`
- Suggested fix example: `Use only ASCII characters in speaker names`
- Emission refs: `rust/crates/talkbank-model/src/offset_adjusting_sink.rs:129`, `rust/crates/talkbank-model/src/offset_adjusting_sink.rs:154`, `rust/crates/talkbank-model/src/offset_adjusting_sink.rs:179`, `rust/crates/talkbank-model/src/tests.rs:13`, `rust/crates/talkbank-model/src/tests.rs:6`, `rust/crates/talkbank-parser/src/api/parser_impl/helpers.rs:243`
- Test refs: None

### `E002` `TestError`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: `rust/crates/talkbank-model/src/tests.rs:133`, `rust/crates/talkbank-model/src/tests.rs:144`, `rust/crates/talkbank-model/src/tests.rs:174`, `rust/crates/talkbank-model/src/tests.rs:81`, `rust/crates/talkbank-model/src/tests.rs:87`
- Test refs: None

### `E003` `EmptyString`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `{} cannot be empty`
- Emission refs: `rust/crates/talkbank-model/src/tests.rs:175`, `rust/crates/talkbank-model/src/model/non_empty_string.rs:172`
- Test refs: None

### `E101` `InvalidLineFormat`

- Primary construct: Direct parser
- Relevant referenced constructs: form-type markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Parse error: {:?}`
- Emission refs: `rust/crates/talkbank-direct-parser/src/file.rs:479`
- Test refs: None

## E2xx Word

| Code | Name | Primary Construct | Referenced Constructs | Name/Message Fit | User-Friendly Message | Suggested Fixes | Coverage |
|---|---|---|---|---|---|---|---|
| `E202` | `MissingFormType` | Main tier / Utterance | %sin alignment, form-type markers | Good fit | Good | Sensible | None |
| `E203` | `InvalidFormType` | Main tier / Utterance | %sin alignment, %wor alignment, form-type markers | Good fit | Good | Missing | None |
| `E207` | `UnknownAnnotation` | Generic/Internal | %gra alignment, %sin alignment, replacement annotations, scoped annotations | Good fit | Good | Sensible | Strong |
| `E208` | `EmptyReplacement` | Main tier / Utterance | %sin alignment, replacement annotations, scoped annotations | Good fit | Good | Needs review | Strong |
| `E209` | `EmptySpokenContent` | Word | %wor alignment, replacement annotations, scoped annotations | Good fit | Good | Needs review | Strong |
| `E210` (deprecated) | `IllegalReplacementForFragment` | No primary emission site found (manual review) | None identified from static references | Deprecated code; verify no active user-facing emission | No message evidence | Missing | None |
| `E211` (deprecated) | `OmissionInReplacement` | No primary emission site found (manual review) | None identified from static references | Deprecated code; verify no active user-facing emission | No message evidence | Missing | None |
| `E212` | `InvalidWordFormat` | Word | %sin alignment, %wor alignment, form-type markers | Good fit | Good | Sensible | None |
| `E213` (deprecated) | `UntranscribedInReplacement` | No primary emission site found (manual review) | None identified from static references | Deprecated code; verify no active user-facing emission | No message evidence | Missing | None |
| `E214` | `EmptyAnnotatedScopedAnnotations` | Generic/Internal | scoped annotations | Good fit | Needs improvement (internal implementation wording) | Missing | None |
| `E220` | `IllegalDigits` | Word | %wor alignment, language metadata | Good fit | Good | Missing | None |
| `E230` | `UnbalancedCADelimiter` | Main tier / Utterance | CA delimiters | Good fit | Good | Missing | Strong |
| `E231` | `UnbalancedShortening` | Word | %wor alignment, shortening markers | Good fit | Good | Missing | None |
| `E232` | `InvalidCompoundMarkerPosition` | Word | %wor alignment, compound markers | Good fit | Good | Sensible | None |
| `E233` | `EmptyCompoundPart` | Word | %wor alignment, compound markers | Good fit | Good | Sensible | None |
| `E241` | `IllegalUntranscribed` | Generic/Internal | %gra alignment, %sin alignment, %wor alignment | Good fit | Good | Missing | Strong |
| `E242` | `UnbalancedQuotation` | Main tier / Utterance | quotation/linkers | Good fit | Good | Sensible | Strong |
| `E243` | `IllegalCharactersInWord` | Word | %sin alignment, %wor alignment | Good fit | Good | Needs review | None |
| `E244` | `ConsecutiveStressMarkers` | Word | %wor alignment | Good fit | Good | Needs review | None |
| `E245` | `StressNotBeforeSpokenMaterial` | Word | %wor alignment | Good fit | Good | Needs review | None |
| `E246` | `LengtheningNotAfterSpokenMaterial` | Word | %wor alignment | Good fit | Good | Needs review | None |
| `E247` | `MultiplePrimaryStress` | Word | %wor alignment | Good fit | Good | Missing | None |
| `E248` | `TertiaryLanguageNeedsExplicitCode` | Word | %wor alignment, language metadata | Good fit | Good | Missing | None |
| `E249` | `MissingLanguageContext` | Word | %sin alignment, %wor alignment, language metadata | Good fit | Good | Sensible | None |
| `E250` | `SecondaryStressWithoutPrimary` | Word | %wor alignment | Good fit | Good | Needs review | None |
| `E251` | `EmptyWordContentText` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E252` | `SyllablePauseNotBetweenSpokenMaterial` | Word | %wor alignment | Good fit | Good | Sensible | Strong |
| `E253` | `EmptyWordContent` | Word | %wor alignment | Good fit | Good | Sensible | None |
| `E254` | `UndeclaredLanguageCode` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |

### `E202` `MissingFormType`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, form-type markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Missing form type after @`
- Suggested fix example: `Add a form type after @ (e.g., @b for babbling, @s:eng for L2 English, @n for neologism)`
- Emission refs: `rust/crates/talkbank-parser/src/parser/chat_file_parser/utterance_parser.rs:44`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:104`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:92`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/utterance.rs:26`
- Test refs: None

### `E203` `InvalidFormType`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, %wor alignment, form-type markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unknown word prefix kind: {}`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/category.rs:20`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/form.rs:27`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/form.rs:61`
- Test refs: None

### `E207` `UnknownAnnotation`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %gra alignment, %sin alignment, replacement annotations, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Quadruple nested brackets [[[[]]]] are invalid`
- Suggested fix example: `CHAT supports up to triple nested brackets [[[]]]. Use proper nesting for groups and annotations.`
- Emission refs: `rust/crates/talkbank-model/src/configurable_sink.rs:141`, `rust/crates/talkbank-model/src/model/annotation/annotated.rs:132`, `rust/crates/talkbank-model/src/model/annotation/replacement.rs:275`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:38`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:62`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/utterance.rs:57`
- Test refs: `rust/tests/mutation_tests/word.rs:30`, `rust/tests/mutation_tests/word.rs:41`

### `E208` `EmptyReplacement`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, replacement annotations, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: Strong
- Message example: `Valid annotations: [= explanation], [* error], [+ addition], [//] retracing, [<]/[>] overlap`
- Suggested fix example: `Valid annotations: [= explanation], [* error], [+ addition], [//] retracing, [<]/[>] overlap`
- Emission refs: `rust/crates/talkbank-model/src/model/annotation/replacement.rs:118`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/utterance_parser.rs:59`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:119`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:76`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/utterance.rs:43`
- Test refs: `rust/tests/mutation_tests/word.rs:75`

### `E209` `EmptySpokenContent`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment, replacement annotations, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: Strong
- Message example: `Word has no spoken content`
- Suggested fix example: `Words must have phonetic content or be marked as untranscribed (xxx, yyy, www)`
- Emission refs: `rust/crates/talkbank-model/src/model/annotation/replacement.rs:134`, `rust/crates/talkbank-model/src/model/content/word/types.rs:466`
- Test refs: `rust/tests/mutation_tests/main_tier.rs:59`, `rust/tests/mutation_tests/word.rs:77`

### `E210` `IllegalReplacementForFragment`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Deprecated code; verify no active user-facing emission
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E211` `OmissionInReplacement`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Deprecated code; verify no active user-facing emission
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E212` `InvalidWordFormat`

- Primary construct: Word
- Relevant referenced constructs: %sin alignment, %wor alignment, form-type markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `No words parsed from input`
- Suggested fix example: `Use @Options: CA (or CA-Unicode) for CA omissions, or use 0word for omissions in standard CHAT`
- Emission refs: `rust/crates/talkbank-model/src/model/content/word/types.rs:498`, `rust/crates/talkbank-model/src/model/content/word/types.rs:523`, `rust/crates/talkbank-model/src/model/content/word/types.rs:542`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/parse_word.rs:56`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/parse_word.rs:73`
- Test refs: None

### `E213` `UntranscribedInReplacement`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Deprecated code; verify no active user-facing emission
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E214` `EmptyAnnotatedScopedAnnotations`

- Primary construct: Generic/Internal
- Relevant referenced constructs: scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Annotated content must include at least one scoped annotation`
- Emission refs: `rust/crates/talkbank-model/src/model/annotation/annotated.rs:119`
- Test refs: None

### `E220` `IllegalDigits`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment, language metadata
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `\"{}\" is not a legal word in language(s) \"{}\": numeric digits not allowed`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/language/digits.rs:40`
- Test refs: None

### `E230` `UnbalancedCADelimiter`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: CA delimiters
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Unbalanced CA delimiter {} ({:?}): missing closing delimiter`
- Emission refs: `rust/crates/talkbank-model/src/validation/utterance/ca_delimiter.rs:43`
- Test refs: `rust/tests/mutation_tests/word.rs:107`

### `E231` `UnbalancedShortening`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment, shortening markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Closing parenthesis ')' without corresponding opening '('`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/structure.rs:115`, `rust/crates/talkbank-model/src/validation/word/structure.rs:89`
- Test refs: None

### `E232` `InvalidCompoundMarkerPosition`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment, compound markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Compound marker '+' cannot start a word`
- Suggested fix example: `Remove the leading '+' or attach it to the previous word`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/structure.rs:134`, `rust/crates/talkbank-model/src/validation/word/structure.rs:147`
- Test refs: None

### `E233` `EmptyCompoundPart`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment, compound markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Compound marker '+' cannot have empty parts (++)`
- Suggested fix example: `Remove one '+' or add content between compound markers`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/structure.rs:168`
- Test refs: None

### `E241` `IllegalUntranscribed`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %gra alignment, %sin alignment, %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `\"{}\" is not legal; did you mean to use \"{}\"?`
- Emission refs: `rust/crates/talkbank-model/src/configurable_sink.rs:112`, `rust/crates/talkbank-model/src/configurable_sink.rs:117`, `rust/crates/talkbank-model/src/configurable_sink.rs:161`, `rust/crates/talkbank-model/src/model/content/word/types.rs:563`, `rust/crates/talkbank-model/src/validation/async_helpers.rs:215`, `rust/crates/talkbank-model/src/validation/async_helpers.rs:229`
- Test refs: `rust/tests/test_validation_comprehensive/integration.rs:105`

### `E242` `UnbalancedQuotation`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: quotation/linkers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Quotation end (+\"/.) without corresponding begin (+\"/)`
- Suggested fix example: `Ensure each quotation end (+\`
- Emission refs: `rust/crates/talkbank-model/src/validation/utterance/quotation.rs:30`, `rust/crates/talkbank-model/src/validation/utterance/quotation.rs:50`
- Test refs: `rust/tests/mutation_tests/word.rs:106`

### `E243` `IllegalCharactersInWord`

- Primary construct: Word
- Relevant referenced constructs: %sin alignment, %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Word contains illegal whitespace characters`
- Suggested fix example: `Words must not contain spaces, tabs, or newlines. Check word boundaries in %wor tiers and main tier.`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/structure.rs:22`, `rust/crates/talkbank-model/src/validation/word/structure.rs:38`, `rust/crates/talkbank-model/src/validation/word/structure.rs:56`
- Test refs: None

### `E244` `ConsecutiveStressMarkers`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Multiple consecutive stress markers`
- Suggested fix example: `A syllable can only have one stress marker (primary ˈ or secondary ˌ)`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/structure.rs:267`
- Test refs: None

### `E245` `StressNotBeforeSpokenMaterial`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Stress marker not followed by spoken material`
- Suggested fix example: `Stress markers (ˈ ˌ) must precede the syllable they mark`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/structure.rs:286`
- Test refs: None

### `E246` `LengtheningNotAfterSpokenMaterial`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Lengthening marker (:) not after spoken material`
- Suggested fix example: `Lengthening marker (:) must follow the syllable it lengthens (e.g., bana:nas)`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/structure.rs:304`
- Test refs: None

### `E247` `MultiplePrimaryStress`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Word has {} primary stress markers, but only one is allowed`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/structure.rs:230`
- Test refs: None

### `E248` `TertiaryLanguageNeedsExplicitCode`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment, language metadata
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Language '{}' is tertiary, so @s shortcut needs explicit language code (e.g., @s:eng)`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/language/resolve.rs:95`
- Test refs: None

### `E249` `MissingLanguageContext`

- Primary construct: Word
- Relevant referenced constructs: %sin alignment, %wor alignment, language metadata
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `No secondary language available for @s shortcut`
- Suggested fix example: `Either add a second language to @Languages header or use explicit language code (e.g., @s:spa)`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/language/resolve.rs:113`, `rust/crates/talkbank-model/src/validation/word/language/resolve.rs:128`
- Test refs: None

### `E250` `SecondaryStressWithoutPrimary`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `A word can have at most one primary stress (ˈ)`
- Suggested fix example: `A word can have at most one primary stress (ˈ)`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/structure.rs:247`
- Test refs: None

### `E251` `EmptyWordContentText`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E252` `SyllablePauseNotBetweenSpokenMaterial`

- Primary construct: Word
- Relevant referenced constructs: %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Syllable pause marker (^) must be between spoken material`
- Suggested fix example: `Syllable pause (^) must occur between syllables (e.g., rhi^noceros)`
- Emission refs: `rust/crates/talkbank-model/src/validation/word/structure.rs:325`
- Test refs: `rust/tests/mutation_tests/main_tier.rs:58`, `rust/tests/mutation_tests/word.rs:76`

### `E253` `EmptyWordContent`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Word content cannot be empty`
- Suggested fix example: `Add at least one word content element`
- Emission refs: `rust/crates/talkbank-model/src/model/borrowed/word.rs:262`, `rust/crates/talkbank-model/src/model/content/word/types.rs:418`
- Test refs: None

### `E254` `UndeclaredLanguageCode`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

## E3xx Parser/Main-tier

| Code | Name | Primary Construct | Referenced Constructs | Name/Message Fit | User-Friendly Message | Suggested Fixes | Coverage |
|---|---|---|---|---|---|---|---|
| `E301` | `MissingMainTier` | Parser CST/Tree | %sin alignment | Potential mismatch: message uses implementation terms | Needs improvement (internal implementation wording) | Sensible | Strong |
| `E302` | `MissingNode` | Header | %sin alignment, speaker codes | Good fit | Good | Missing | None |
| `E303` | `SyntaxError` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | Strong |
| `E304` | `MissingSpeaker` | Main tier / Utterance | %sin alignment, speaker codes | Good fit | Good | Needs review | Strong |
| `E305` | `MissingTerminator` | Main tier / Utterance | %sin alignment | Good fit | Good | Sensible | Strong |
| `E306` | `EmptyUtterance` | Parser CST/Tree | %sin alignment | Good fit | Good | Sensible | Strong |
| `E307` | `InvalidSpeaker` | Header | speaker codes | Good fit | Good | Missing | None |
| `E308` | `UndeclaredSpeaker` | Generic/Internal | speaker codes | Good fit | Good | Sensible | Strong |
| `E309` | `UnexpectedSyntax` | Parser CST/Tree | %sin alignment | Good fit | Good | Needs review | None |
| `E310` | `ParseFailed` | Generic/Internal | %sin alignment, %wor alignment, form-type markers | Potential mismatch: message uses implementation terms | Needs improvement (internal implementation wording) | Missing | None |
| `E311` | `UnexpectedNode` | Generic/Internal | %sin alignment | Good fit | Good | Needs review | None |
| `E312` | `UnclosedBracket` | Parser CST/Tree | %sin alignment | Good fit | Good | Needs review | None |
| `E313` | `UnclosedParenthesis` | Parser CST/Tree | %sin alignment | Good fit | Good | Sensible | None |
| `E314` | `IncompleteAnnotation` | Parser CST/Tree | %sin alignment, scoped annotations | Good fit | Good | Sensible | None |
| `E315` | `InvalidControlCharacter` | Parser CST/Tree | %sin alignment | Good fit | Good | Needs review | None |
| `E316` | `UnparsableContent` | Parser CST/Tree | %sin alignment, %wor alignment | Good fit | Good | Sensible | Strong |
| `E317` | `UnparsableFileContent` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E318` | `UnparsableDependentTier` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E319` | `UnparsableLine` | Parser CST/Tree | %sin alignment | Good fit | Good | Missing | None |
| `E320` | `UnparsableHeader` | Parser CST/Tree | %sin alignment | Good fit | Good | Missing | None |
| `E321` | `UnparsableUtterance` | Parser CST/Tree | %sin alignment | Good fit | Good | Missing | None |
| `E322` | `EmptyColon` | Main tier / Utterance | %sin alignment | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E323` | `MissingColonAfterSpeaker` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E324` | `UnrecognizedUtteranceError` | Main tier / Utterance | None identified from static references | Good fit | Good | Sensible | None |
| `E325` | `UnexpectedUtteranceChild` | Main tier / Utterance | None identified from static references | Good fit | Needs improvement (internal implementation wording) | Missing | None |
| `E326` | `UnexpectedLineType` | Generic/Internal | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E330` | `TreeParsingError` | Parser CST/Tree | %pho alignment, %sin alignment, %wor alignment, language metadata, media bullets/timestamps, overlap markers, participants, quotation/linkers | Good fit | Good | Weak/placeholder | Strong |
| `E331` | `UnexpectedNodeInContext` | Parser CST/Tree | %sin alignment | Good fit | Good | Needs review | None |
| `E340` | `UnknownBaseContent` | Main tier / Utterance | %sin alignment | Good fit | Good | Weak/placeholder | None |
| `E341` | `UnbalancedQuotationCrossUtterance` | Generic/Internal | quotation/linkers | Good fit | Needs improvement (internal implementation wording) | Missing | Strong |
| `E342` | `MissingRequiredElement` | Parser CST/Tree | %sin alignment, %wor alignment | Good fit | Good | Missing | None |
| `E344` | `InvalidScopedAnnotationNesting` | Generic/Internal | quotation/linkers, scoped annotations | Good fit | Good | Missing | Strong |
| `E345` | `UnmatchedScopedAnnotationBegin` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E346` | `UnmatchedScopedAnnotationEnd` | Generic/Internal | scoped annotations | Good fit | Needs improvement (internal implementation wording) | Missing | Strong |
| `E347` | `UnbalancedOverlap` | Main tier / Utterance | %sin alignment, overlap markers | Good fit | Good | Missing | None |
| `E348` | `MissingOverlapEnd` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E350` | `GenericAnnotationError` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | Strong |
| `E351` | `MissingQuoteBegin` | Generic/Internal | %sin alignment | Good fit | Good | Missing | Strong |
| `E352` | `MissingQuoteEnd` | Generic/Internal | %sin alignment | Good fit | Good | Missing | Strong |
| `E353` | `MissingOtherCompletionContext` | Generic/Internal | %sin alignment | Good fit | Good | Missing | Strong |
| `E354` | `MissingTrailingOffTerminator` | Generic/Internal | %sin alignment | Good fit | Good | Missing | Strong |
| `E355` | `InterleavedScopedAnnotations` | Generic/Internal | scoped annotations | Good fit | Good | Missing | Strong |
| `E356` | `UnmatchedUnderlineBegin` | Generic/Internal | underline markers | Good fit | Good | Missing | None |
| `E357` | `UnmatchedUnderlineEnd` | Main tier / Utterance | underline markers | Good fit | Good | Sensible | None |
| `E358` | `UnmatchedLongFeatureBegin` | Generic/Internal | None identified from static references | Good fit | Good | Missing | None |
| `E359` | `UnmatchedLongFeatureEnd` | Generic/Internal | None identified from static references | Good fit | Good | Missing | None |
| `E360` | `InvalidMediaBullet` | Parser CST/Tree | %sin alignment, media bullets/timestamps | Good fit | Good | Missing | None |
| `E361` | `InvalidTimestamp` | Parser CST/Tree | %sin alignment, timestamps | Good fit | Good | Missing | None |
| `E362` | `TimestampBackwards` | Generic/Internal | media bullets/timestamps, timestamps | Good fit | Good | Missing | Strong |
| `E363` | `InvalidPostcode` | Main tier / Utterance | %sin alignment | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E364` | `MalformedWordContent` | Main tier / Utterance | %sin alignment, %wor alignment, form-type markers | Good fit | Good | Missing | None |
| `E365` | `MalformedTierContent` | Generic/Internal | form-type markers | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E366` | `LongFeatureLabelMismatch` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E367` | `UnmatchedNonvocalBegin` | Generic/Internal | None identified from static references | Good fit | Good | Missing | None |
| `E368` | `UnmatchedNonvocalEnd` | Generic/Internal | None identified from static references | Good fit | Good | Missing | None |
| `E369` | `NonvocalLabelMismatch` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E370` | `StructuralOrderError` | Main tier / Utterance | %sin alignment, retracing | Good fit | Good | Sensible | Strong |
| `E371` | `PauseInPhoGroup` | Main tier / Utterance | %pho alignment | Good fit | Good | Sensible | Strong |
| `E372` | `NestedQuotation` | Main tier / Utterance | quotation/linkers | Good fit | Good | Sensible | Strong |
| `E373` | `InvalidOverlapIndex` | Generic/Internal | %sin alignment, overlap markers, scoped annotations | Good fit | Good | Needs review | None |
| `E374` | `ErrorAnnotationParseError` | Main tier / Utterance | %sin alignment, scoped annotations | Good fit | Needs improvement (internal implementation wording) | Missing | None |
| `E375` | `ScopedAnnotationParseError` | Main tier / Utterance | %sin alignment, scoped annotations | Good fit | Good | Needs review | None |
| `E376` | `ReplacementParseError` | Main tier / Utterance | %sin alignment, replacement annotations, scoped annotations | Potential mismatch: message uses implementation terms | Good | Missing | None |
| `E377` | `RetraceParseError` | Main tier / Utterance | %sin alignment, retracing, scoped annotations | Good fit | Good | Missing | None |
| `E378` | `OverlapAnnotationParseError` | Main tier / Utterance | %sin alignment, overlap markers, scoped annotations | Good fit | Needs improvement (internal implementation wording) | Missing | None |
| `E380` | `UnknownSeparator` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E381` | `PhoParseError` | Direct parser | %pho alignment | Good fit | Good | Missing | None |
| `E382` | `MorParseError` | Direct parser | %mor alignment | Good fit | Good | Missing | None |
| `E383` | `GraParseError` | Direct parser | %gra alignment | Good fit | Good | Missing | None |
| `E384` | `SinParseError` | Direct parser | %sin alignment | Good fit | Good | Missing | None |
| `E385` | `WordParseError` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E386` | `TextTierParseError` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E387` | `ReplacementOnFragment` | Generic/Internal | replacement annotations, scoped annotations | Good fit | Good | Missing | None |
| `E388` | `ReplacementOnNonword` | Generic/Internal | %wor alignment, replacement annotations, scoped annotations | Good fit | Good | Missing | None |
| `E389` | `ReplacementOnFiller` | Generic/Internal | replacement annotations, scoped annotations | Good fit | Good | Missing | None |
| `E390` | `ReplacementContainsOmission` | Generic/Internal | replacement annotations, scoped annotations | Good fit | Good | Sensible | None |
| `E391` | `ReplacementContainsUntranscribed` | Generic/Internal | replacement annotations, scoped annotations | Good fit | Good | Sensible | None |

### `E301` `MissingMainTier`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Potential mismatch: message uses implementation terms
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Could not find main_tier node in parse tree`
- Suggested fix example: `Add a speaker code between * and : (e.g., *CHI:)`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/finder.rs:30`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs:53`
- Test refs: `rust/tests/mutation_tests/main_tier.rs:37`

### `E302` `MissingNode`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, speaker codes
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Speaker ID '{}' contains invalid character '{}'`
- Emission refs: `rust/crates/talkbank-model/src/tests.rs:209`, `rust/crates/talkbank-model/src/model/header/codes/speaker.rs:199`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/errors.rs:21`
- Test refs: None

### `E303` `SyntaxError`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: Strong
- Emission refs: None
- Test refs: `rust/tests/full_line_context_test.rs:165`, `rust/tests/full_line_context_test.rs:31`, `rust/tests/mutation_tests/headers.rs:144`, `rust/tests/mutation_tests/headers.rs:216`, `rust/tests/mutation_tests/headers.rs:50`, `rust/tests/mutation_tests/word.rs:40`

### `E304` `MissingSpeaker`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, speaker codes
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: Strong
- Message example: `Missing speaker in main tier`
- Suggested fix example: `Main tier should start with *SPEAKER:`
- Emission refs: `rust/crates/talkbank-model/src/model/content/main_tier.rs:419`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/mod.rs:41`
- Test refs: `rust/tests/mutation_tests/main_tier.rs:36`, `rust/tests/test_validation_comprehensive/integration.rs:110`, `rust/tests/test_validation_comprehensive/sad.rs:38`

### `E305` `MissingTerminator`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Unknown terminator type '{}'`
- Suggested fix example: `Add utterance content after the colon-tab (e.g., *CHI:\thello world .)`
- Emission refs: `rust/crates/talkbank-model/src/types.rs:451`, `rust/crates/talkbank-model/src/model/content/tier_content.rs:387`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/end.rs:74`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/end.rs:87`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/utterance_end.rs:66`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs:75`
- Test refs: `rust/tests/mutation_tests/main_tier.rs:56`

### `E306` `EmptyUtterance`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Utterance has no meaningful content (only separators)`
- Suggested fix example: `Add at least one word or content element to the utterance`
- Emission refs: `rust/crates/talkbank-model/src/model/content/tier_content.rs:406`, `rust/crates/talkbank-parser/src/parser/tree_parsing/freecode/mod.rs:61`
- Test refs: `rust/tests/mutation_tests/main_tier.rs:57`

### `E307` `InvalidSpeaker`

- Primary construct: Header
- Relevant referenced constructs: speaker codes
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Speaker ID '{}' exceeds maximum length of 7 characters`
- Emission refs: `rust/crates/talkbank-model/src/model/content/main_tier.rs:369`, `rust/crates/talkbank-model/src/model/content/main_tier.rs:383`, `rust/crates/talkbank-model/src/validation/header/checkers.rs:101`, `rust/crates/talkbank-model/src/validation/header/checkers.rs:88`
- Test refs: None

### `E308` `UndeclaredSpeaker`

- Primary construct: Generic/Internal
- Relevant referenced constructs: speaker codes
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Speaker '{}' is not in the participant list`
- Suggested fix example: `Add this speaker to the @Participants header`
- Emission refs: `rust/crates/talkbank-model/src/model/content/main_tier.rs:401`, `rust/crates/talkbank-model/src/model/header/codes/speaker.rs:174`
- Test refs: `rust/tests/mutation_tests/headers.rs:181`, `rust/tests/test_speaker_validation/invalid.rs:26`, `rust/tests/test_speaker_validation/invalid.rs:62`, `rust/tests/test_speaker_validation/valid.rs:27`, `rust/tests/test_speaker_validation/valid.rs:62`, `rust/tests/test_validation_comprehensive/sad.rs:28`

### `E309` `UnexpectedSyntax`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Unexpected syntax in {}`
- Suggested fix example: `Check for missing or malformed elements`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs:22`
- Test refs: None

### `E310` `ParseFailed`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment, %wor alignment, form-type markers
- Name/message assessment: Potential mismatch: message uses implementation terms
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Could not find standalone_word node in parse tree`
- Emission refs: `rust/crates/talkbank-transform/src/pipeline/parse.rs:120`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/chat_file/helpers.rs:37`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/chat_file/parse.rs:34`, `rust/crates/talkbank-parser/src/parser/tier_parsers/dependent_tier.rs:73`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/finder.rs:30`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/finder.rs:57`
- Test refs: None

### `E311` `UnexpectedNode`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Unclosed replacement bracket - replacements must be in format '[: correct form]' or '[* phonological form]'`
- Suggested fix example: `Complete the replacement: '[: target]' for word replacements, '[* phonology]' for phonological forms`
- Emission refs: `rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/helpers.rs:31`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:14`
- Test refs: None

### `E312` `UnclosedBracket`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Check for missing or malformed elements`
- Suggested fix example: `Check for missing or malformed elements`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs:36`
- Test refs: None

### `E313` `UnclosedParenthesis`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Add closing bracket ']' or check bracket nesting`
- Suggested fix example: `Add closing bracket ']' or check bracket nesting`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs:50`
- Test refs: None

### `E314` `IncompleteAnnotation`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Unclosed parenthesis in {}`
- Suggested fix example: `Add closing parenthesis ')' to complete the group`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs:62`
- Test refs: None

### `E315` `InvalidControlCharacter`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Incomplete annotation in {}`
- Suggested fix example: `Complete the annotation like [= comment] or [* error]`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs:74`
- Test refs: None

### `E316` `UnparsableContent`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment, %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Check freecode format. Freecodes should follow standard patterns like ‡code`
- Suggested fix example: `Remove or replace control characters (only tabs are allowed)`
- Emission refs: `rust/crates/talkbank-parser/src/parser/chat_file_parser/chat_file/helpers.rs:54`, `rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs:85`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:134`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/dependent_tier.rs:58`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs:163`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs:32`
- Test refs: `rust/tests/mutation_tests/headers.rs:145`, `rust/tests/mutation_tests/headers.rs:217`, `rust/tests/mutation_tests/headers.rs:51`

### `E317` `UnparsableFileContent`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E318` `UnparsableDependentTier`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E319` `UnparsableLine`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Syntax error in line`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/line.rs:52`
- Test refs: None

### `E320` `UnparsableHeader`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Syntax error in header`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/header.rs:97`
- Test refs: None

### `E321` `UnparsableUtterance`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unknown scoped annotation marker`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/utterance.rs:68`
- Test refs: None

### `E322` `EmptyColon`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/prefix.rs:82`
- Test refs: None

### `E323` `MissingColonAfterSpeaker`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E324` `UnrecognizedUtteranceError`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Add replacement text after [: , e.g., word [: corrected]`
- Suggested fix example: `Add replacement text after [: , e.g., word [: corrected]`
- Emission refs: `rust/crates/talkbank-parser/src/parser/chat_file_parser/utterance_parser.rs:75`
- Test refs: None

### `E325` `UnexpectedUtteranceChild`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unexpected child '{}' in utterance`
- Emission refs: `rust/crates/talkbank-parser/src/parser/chat_file_parser/utterance_parser.rs:120`
- Test refs: None

### `E326` `UnexpectedLineType`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: `rust/crates/talkbank-parser/src/parser/chat_file_parser/chat_file/helpers.rs:133`
- Test refs: None

### `E330` `TreeParsingError`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %pho alignment, %sin alignment, %wor alignment, language metadata, media bullets/timestamps, overlap markers, participants, quotation/linkers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Weak/placeholder
- Coverage: Strong
- Message example: `Thread-local parser failed: {err}`
- Suggested fix example: `Will be supported in future version`
- Emission refs: `rust/crates/talkbank-parser/src/lib.rs:124`, `rust/crates/talkbank-parser/src/lib.rs:144`, `rust/crates/talkbank-parser/src/lib.rs:164`, `rust/crates/talkbank-parser/src/lib.rs:184`, `rust/crates/talkbank-parser/src/lib.rs:220`, `rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/parse.rs:47`
- Test refs: `rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/tests/mod.rs:25`

### `E331` `UnexpectedNodeInContext`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Unexpected '{}' in {}`
- Suggested fix example: `This element is not valid in this context`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs:123`
- Test refs: None

### `E340` `UnknownBaseContent`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Weak/placeholder
- Coverage: None
- Message example: `Unknown base content type '{}'`
- Suggested fix example: `This may be a new grammar feature not yet supported`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/mod.rs:72`
- Test refs: None

### `E341` `UnbalancedQuotationCrossUtterance`

- Primary construct: Generic/Internal
- Relevant referenced constructs: quotation/linkers
- Name/message assessment: Good fit
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: ` linker
        if !has_quoted_linker(next_utt) {
            errors.push(
                ParseError::new(
                    ErrorCode::UnbalancedQuotationCrossUtterance,
                    Severity::Error,
                    SourceLocation::new(utterance.main.span),
                    ErrorContext::new(
                        format!(`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/quotation_follows.rs:27`, `rust/crates/talkbank-model/src/validation/cross_utterance/quotation_follows.rs:62`, `rust/crates/talkbank-model/src/validation/cross_utterance/quotation_follows.rs:83`
- Test refs: `rust/crates/talkbank-model/src/validation/cross_utterance/tests/edge_cases.rs:55`, `rust/crates/talkbank-model/src/validation/cross_utterance/tests/quotation_follows.rs:104`, `rust/crates/talkbank-model/src/validation/cross_utterance/tests/quotation_follows.rs:138`, `rust/crates/talkbank-model/src/validation/cross_utterance/tests/quotation_follows.rs:76`

### `E342` `MissingRequiredElement`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment, %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Invalid or incomplete word structure`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/content.rs:195`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:195`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:248`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:287`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_checking.rs:36`
- Test refs: None

### `E344` `InvalidScopedAnnotationNesting`

- Primary construct: Generic/Internal
- Relevant referenced constructs: quotation/linkers, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `*{}: ... +\". `
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/quotation_precedes.rs:29`
- Test refs: `rust/crates/talkbank-model/src/validation/cross_utterance/tests/quotation_precedes.rs:68`, `rust/crates/talkbank-model/src/validation/cross_utterance/tests/terminator_linker_pairing.rs:18`

### `E345` `UnmatchedScopedAnnotationBegin`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E346` `UnmatchedScopedAnnotationEnd`

- Primary construct: Generic/Internal
- Relevant referenced constructs: scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `.
    if !pattern_a_valid && !pattern_b_valid {
        errors.push(
            ParseError::new(
                ErrorCode::UnmatchedScopedAnnotationEnd,
                Severity::Error,
                SourceLocation::new(utterance.main.span),
                ErrorContext::new(
                    format!(`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/quoted_linker.rs:61`
- Test refs: `rust/crates/talkbank-model/src/validation/cross_utterance/tests/quotation_precedes.rs:91`, `rust/crates/talkbank-model/src/validation/cross_utterance/tests/terminator_linker_pairing.rs:36`

### `E347` `UnbalancedOverlap`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, overlap markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unknown overlap point marker '{}' (U+{:04X})`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/overlap_point.rs:66`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/overlap.rs:62`
- Test refs: None

### `E348` `MissingOverlapEnd`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E350` `GenericAnnotationError`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: Strong
- Emission refs: None
- Test refs: `rust/tests/mutation_tests/word.rs:42`

### `E351` `MissingQuoteBegin`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `self-completion linker`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/completion.rs:134`, `rust/crates/talkbank-model/src/validation/cross_utterance/completion.rs:77`
- Test refs: `rust/crates/talkbank-model/src/validation/cross_utterance/tests/self_completion.rs:42`

### `E352` `MissingQuoteEnd`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `self-completion linker`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/completion.rs:161`, `rust/crates/talkbank-model/src/validation/cross_utterance/completion.rs:55`
- Test refs: `rust/crates/talkbank-model/src/validation/cross_utterance/tests/self_completion.rs:71`

### `E353` `MissingOtherCompletionContext`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `other-completion linker`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/completion.rs:196`
- Test refs: `rust/crates/talkbank-model/src/validation/cross_utterance/tests/other_completion.rs:41`

### `E354` `MissingTrailingOffTerminator`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `other-completion linker`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/completion.rs:247`
- Test refs: `rust/crates/talkbank-model/src/validation/cross_utterance/tests/other_completion.rs:64`

### `E355` `InterleavedScopedAnnotations`

- Primary construct: Generic/Internal
- Relevant referenced constructs: scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `other-completion linker`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/completion.rs:218`
- Test refs: `rust/crates/talkbank-model/src/validation/cross_utterance/tests/other_completion.rs:88`

### `E356` `UnmatchedUnderlineBegin`

- Primary construct: Generic/Internal
- Relevant referenced constructs: underline markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unmatched underline begin: {} unclosed begin marker(s) (␂␁)`
- Emission refs: `rust/crates/talkbank-model/src/enhance.rs:287`, `rust/crates/talkbank-model/src/validation/utterance/underline.rs:30`
- Test refs: None

### `E357` `UnmatchedUnderlineEnd`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: underline markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Unmatched underline end (␂␂) without corresponding begin (␂␁)`
- Suggested fix example: `Ensure each underline end (␂␂) has a matching underline begin (␂␁) before it`
- Emission refs: `rust/crates/talkbank-model/src/validation/utterance/underline.rs:175`
- Test refs: None

### `E358` `UnmatchedLongFeatureBegin`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unmatched long feature begin marker: {} &{{l={} without matching &}}l={}`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs:83`
- Test refs: None

### `E359` `UnmatchedLongFeatureEnd`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unmatched long feature end marker for label '{}'`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs:41`, `rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs:60`
- Test refs: None

### `E360` `InvalidMediaBullet`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment, media bullets/timestamps
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Invalid internal bullet start time: '{}'`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/internal_bullet.rs:57`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/internal_bullet.rs:76`, `rust/crates/talkbank-parser/src/parser/tree_parsing/media_bullet.rs:132`, `rust/crates/talkbank-parser/src/parser/tree_parsing/media_bullet.rs:52`, `rust/crates/talkbank-parser/src/parser/tree_parsing/media_bullet.rs:69`, `rust/crates/talkbank-parser/src/parser/tree_parsing/media_bullet.rs:84`
- Test refs: None

### `E361` `InvalidTimestamp`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment, timestamps
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Malformed postcode content at byte {}..{}`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/freecode/mod.rs:38`, `rust/crates/talkbank-parser/src/parser/tree_parsing/postcode/mod.rs:34`
- Test refs: None

### `E362` `TimestampBackwards`

- Primary construct: Generic/Internal
- Relevant referenced constructs: media bullets/timestamps, timestamps
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Media bullet start time ({}ms) must be less than end time ({}ms)`
- Emission refs: `rust/crates/talkbank-model/src/validation/bullet.rs:15`, `rust/crates/talkbank-model/src/validation/bullet.rs:39`
- Test refs: `rust/tests/validation_gaps.rs:164`

### `E363` `InvalidPostcode`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/utterance_end.rs:50`
- Test refs: None

### `E364` `MalformedWordContent`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, %wor alignment, form-type markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Malformed multiple languages marker - expected language code, got {}`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/long_feature.rs:125`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/nonvocal.rs:168`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:55`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:65`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:80`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:94`
- Test refs: None

### `E365` `MalformedTierContent`

- Primary construct: Generic/Internal
- Relevant referenced constructs: form-type markers
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: `rust/crates/talkbank-parser/src/parser/chat_file_parser/header_dispatch/parse.rs:164`
- Test refs: None

### `E366` `LongFeatureLabelMismatch`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E367` `UnmatchedNonvocalBegin`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unmatched nonvocal begin marker: {} &{{n={} without matching &}}n={}`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs:165`
- Test refs: None

### `E368` `UnmatchedNonvocalEnd`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unmatched nonvocal end marker for label '{}'`
- Emission refs: `rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs:126`, `rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs:142`
- Test refs: None

### `E369` `NonvocalLabelMismatch`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E370` `StructuralOrderError`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, retracing
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Unexpected '{}' in contents`
- Suggested fix example: `Add content after the retrace marker, or remove the retrace if it's not needed`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/contents.rs:152`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/contents.rs:62`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/body.rs:103`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/body.rs:168`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/body.rs:38`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/body.rs:73`
- Test refs: `rust/tests/validation_gaps.rs:143`

### `E371` `PauseInPhoGroup`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %pho alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Pause cannot appear inside phonological group ‹...›`
- Suggested fix example: `Move the pause outside the phonological group, or remove it`
- Emission refs: `rust/crates/talkbank-model/src/validation/main_tier.rs:28`
- Test refs: `rust/tests/validation_gaps.rs:181`

### `E372` `NestedQuotation`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: quotation/linkers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Quotations cannot be nested inside other quotations`
- Suggested fix example: `Use separate quotations or reformulate without nesting`
- Emission refs: `rust/crates/talkbank-model/src/validation/main_tier.rs:65`
- Test refs: `rust/tests/validation_gaps.rs:215`

### `E373` `InvalidOverlapIndex`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment, overlap markers, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Invalid overlap marker index '{}'`
- Suggested fix example: `Overlap indices must be a single digit from 2 to 9`
- Emission refs: `rust/crates/talkbank-model/src/configurable_sink.rs:127`, `rust/crates/talkbank-model/src/configurable_sink.rs:132`, `rust/crates/talkbank-model/src/model/annotation/scoped/types.rs:326`, `rust/crates/talkbank-model/src/model/content/overlap.rs:234`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/overlap.rs:99`
- Test refs: None

### `E374` `ErrorAnnotationParseError`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unexpected child '{}' in error_marker_annotation`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/error_annotation.rs:100`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/error_annotation.rs:46`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/error_annotation.rs:67`
- Test refs: None

### `E375` `ScopedAnnotationParseError`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `Unclosed replacement bracket - replacements must be in format '[: correct form]' or '[* phonological form]'`
- Suggested fix example: `Complete the replacement: '[: target]' for word replacements, '[* phonology]' for phonological forms`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/scoped/list.rs:39`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/scoped/list.rs:67`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/scoped/single.rs:127`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/scoped/single.rs:54`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/scoped/single.rs:67`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:26`
- Test refs: None

### `E376` `ReplacementParseError`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, replacement annotations, scoped annotations
- Name/message assessment: Potential mismatch: message uses implementation terms
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Missing word in replacement at position {} (tree-sitter inserted placeholder)`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/replacement/helpers.rs:17`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/replacement/helpers.rs:42`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/replacement/parse.rs:117`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/replacement/parse.rs:59`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/replacement/parse.rs:75`
- Test refs: None

### `E377` `RetraceParseError`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, retracing, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Failed to access child at position 0 of retrace_marker`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/retrace.rs:34`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/retrace.rs:47`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/retrace.rs:64`
- Test refs: None

### `E378` `OverlapAnnotationParseError`

- Primary construct: Main tier / Utterance
- Relevant referenced constructs: %sin alignment, overlap markers, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unexpected child '{}' in overlap annotation`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/overlap.rs:44`, `rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/overlap.rs:54`
- Test refs: None

### `E380` `UnknownSeparator`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E381` `PhoParseError`

- Primary construct: Direct parser
- Relevant referenced constructs: %pho alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Pho word parse error: {}`
- Emission refs: `rust/crates/talkbank-direct-parser/src/pho_tier.rs:129`, `rust/crates/talkbank-direct-parser/src/pho_tier.rs:49`, `rust/crates/talkbank-direct-parser/src/pho_tier.rs:99`
- Test refs: None

### `E382` `MorParseError`

- Primary construct: Direct parser
- Relevant referenced constructs: %mor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Mor word parse error: {}`
- Emission refs: `rust/crates/talkbank-direct-parser/src/mor_tier.rs:111`, `rust/crates/talkbank-direct-parser/src/mor_tier.rs:141`, `rust/crates/talkbank-direct-parser/src/mor_tier.rs:64`
- Test refs: None

### `E383` `GraParseError`

- Primary construct: Direct parser
- Relevant referenced constructs: %gra alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Gra relation parse error: {}`
- Emission refs: `rust/crates/talkbank-direct-parser/src/gra_tier.rs:104`, `rust/crates/talkbank-direct-parser/src/gra_tier.rs:134`, `rust/crates/talkbank-direct-parser/src/gra_tier.rs:57`
- Test refs: None

### `E384` `SinParseError`

- Primary construct: Direct parser
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Sin tier parse error: {}`
- Emission refs: `rust/crates/talkbank-direct-parser/src/sin_tier.rs:29`
- Test refs: None

### `E385` `WordParseError`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E386` `TextTierParseError`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E387` `ReplacementOnFragment`

- Primary construct: Generic/Internal
- Relevant referenced constructs: replacement annotations, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Replacement [: text] is not allowed for fragments`
- Emission refs: `rust/crates/talkbank-model/src/model/annotation/replacement.rs:503`
- Test refs: None

### `E388` `ReplacementOnNonword`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %wor alignment, replacement annotations, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Replacement [: text] is not allowed for nonwords`
- Emission refs: `rust/crates/talkbank-model/src/model/annotation/replacement.rs:523`
- Test refs: None

### `E389` `ReplacementOnFiller`

- Primary construct: Generic/Internal
- Relevant referenced constructs: replacement annotations, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Replacement [: text] is not allowed for fillers`
- Emission refs: `rust/crates/talkbank-model/src/model/annotation/replacement.rs:543`
- Test refs: None

### `E390` `ReplacementContainsOmission`

- Primary construct: Generic/Internal
- Relevant referenced constructs: replacement annotations, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Remove empty word from replacement or fix parsing issue`
- Suggested fix example: `Remove empty word from replacement or fix parsing issue`
- Emission refs: `rust/crates/talkbank-model/src/model/annotation/replacement.rs:151`
- Test refs: None

### `E391` `ReplacementContainsUntranscribed`

- Primary construct: Generic/Internal
- Relevant referenced constructs: replacement annotations, scoped annotations
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Remove the 0 prefix from replacement words`
- Suggested fix example: `Remove the 0 prefix from replacement words`
- Emission refs: `rust/crates/talkbank-model/src/model/annotation/replacement.rs:168`
- Test refs: None

## E4xx Dependent-tier

| Code | Name | Primary Construct | Referenced Constructs | Name/Message Fit | User-Friendly Message | Suggested Fixes | Coverage |
|---|---|---|---|---|---|---|---|
| `E401` | `DuplicateDependentTier` | Direct parser | None identified from static references | Good fit | Good | Missing | Strong |
| `E404` | `OrphanedDependentTier` | Direct parser | None identified from static references | Good fit | Good | Missing | None |

### `E401` `DuplicateDependentTier`

- Primary construct: Direct parser
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Duplicate dependent tier: %{} appears more than once for this utterance`
- Emission refs: `rust/crates/talkbank-direct-parser/src/dependent_tier.rs:63`, `rust/crates/talkbank-model/src/validation/utterance/tiers.rs:23`
- Test refs: `rust/tests/validation_gaps.rs:97`

### `E404` `OrphanedDependentTier`

- Primary construct: Direct parser
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Dependent tier must follow a main tier`
- Emission refs: `rust/crates/talkbank-direct-parser/src/file.rs:194`
- Test refs: None

## E5xx Header/Metadata

| Code | Name | Primary Construct | Referenced Constructs | Name/Message Fit | User-Friendly Message | Suggested Fixes | Coverage |
|---|---|---|---|---|---|---|---|
| `E501` | `DuplicateHeader` | Direct parser | None identified from static references | Good fit | Good | Missing | Strong |
| `E502` | `MissingEndHeader` | Header | %sin alignment | Good fit | Good | Sensible | Strong |
| `E504` | `MissingRequiredHeader` | Header | %sin alignment | Good fit | Good | Missing | Strong |
| `E505` | `InvalidIDFormat` | Parser CST/Tree | %sin alignment, form-type markers | Good fit | Good | Needs review | Strong |
| `E506` | `EmptyParticipantsHeader` | Header | %sin alignment, participants | Good fit | Good | Missing | None |
| `E507` | `EmptyLanguagesHeader` | Header | %sin alignment, language metadata | Good fit | Good | Missing | None |
| `E508` | `EmptyDateHeader` | Parser CST/Tree | %sin alignment | Good fit | Good | Missing | None |
| `E509` | `EmptyMediaHeader` | Parser CST/Tree | %sin alignment | Good fit | Good | Missing | None |
| `E510` | `EmptyIDLanguage` | Header | language metadata | Good fit | Good | Missing | None |
| `E511` | `EmptyIDSpeaker` | Header | speaker codes | Good fit | Good | Missing | None |
| `E512` | `EmptyParticipantCode` | Header | participants | Good fit | Good | Missing | None |
| `E513` | `EmptyParticipantRole` | Header | participants | Good fit | Good | Missing | None |
| `E514` | `MissingLanguageCode` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E515` | `EmptyIDRole` | Parser CST/Tree | %sin alignment, media bullets/timestamps | Good fit | Good | Missing | None |
| `E516` | `EmptyDate` | Header | None identified from static references | Good fit | Good | Missing | None |
| `E517` | `InvalidAgeFormat` | Header | form-type markers | Good fit | Good | Missing | Strong |
| `E518` | `InvalidDateFormat` | Header | form-type markers | Good fit | Good | Sensible | Strong |
| `E519` | `InvalidLanguageCode` | Header | language metadata | Good fit | Good | Sensible | None |
| `E522` | `SpeakerNotDefined` | Generic/Internal | participants, speaker codes | Good fit | Good | Missing | Strong |
| `E523` | `OrphanIDHeader` | Generic/Internal | participants | Good fit | Good | Missing | Strong |
| `E524` | `BirthUnknownParticipant` | Generic/Internal | participants | Good fit | Good | Missing | Strong |
| `E525` | `UnknownHeader` | Generic/Internal | None identified from static references | Potential mismatch: message uses implementation terms | Needs improvement (internal implementation wording) | Missing | None |
| `E526` | `UnmatchedBeginGem` | Header | None identified from static references | Good fit | Good | Missing | None |
| `E527` | `UnmatchedEndGem` | Header | None identified from static references | Good fit | Good | Missing | None |
| `E528` | `GemLabelMismatch` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E529` | `NestedBeginGem` | Header | None identified from static references | Good fit | Good | Missing | Strong |
| `E530` | `LazyGemInsideScope` | Header | None identified from static references | Good fit | Good | Missing | Strong |
| `E531` | `MediaFilenameMismatch` | Generic/Internal | None identified from static references | Good fit | Good | Missing | Strong |
| `E532` | `InvalidParticipantRole` | Header | participants | Good fit | Good | Missing | Strong |
| `E533` | `EmptyOptionsHeader` | Header | None identified from static references | Good fit | Good | Missing | None |

### `E501` `DuplicateHeader`

- Primary construct: Direct parser
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Duplicate @{} header found ({} occurrences)`
- Emission refs: `rust/crates/talkbank-direct-parser/src/header.rs:1115`, `rust/crates/talkbank-direct-parser/src/header.rs:1130`, `rust/crates/talkbank-direct-parser/src/header.rs:1158`, `rust/crates/talkbank-direct-parser/src/header.rs:183`, `rust/crates/talkbank-model/src/validation/header/structure.rs:38`
- Test refs: `rust/tests/mutation_tests/headers.rs:40`, `rust/tests/mutation_tests/headers.rs:52`

### `E502` `MissingEndHeader`

- Primary construct: Header
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Add @{} header to the file`
- Suggested fix example: `Add @End header at the end of the file`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/structure.rs:73`
- Test refs: `rust/tests/mutation_tests/headers.rs:85`

### `E504` `MissingRequiredHeader`

- Primary construct: Header
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Missing required @{} header`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/structure.rs:59`
- Test refs: `rust/tests/mutation_tests/headers.rs:109`

### `E505` `InvalidIDFormat`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment, form-type markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: Strong
- Message example: `@Media header cannot be empty`
- Suggested fix example: `Format: @ID:\tlang|corpus|speaker|age|sex|group|SES|role|education|custom|`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs:147`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/header.rs:79`
- Test refs: `rust/tests/mutation_tests/headers.rs:134`, `rust/tests/mutation_tests/headers.rs:146`

### `E506` `EmptyParticipantsHeader`

- Primary construct: Header
- Relevant referenced constructs: %sin alignment, participants
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Missing participants_contents in @Participants header`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:123`, `rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:151`, `rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:183`, `rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:52`, `rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:90`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/header.rs:38`
- Test refs: None

### `E507` `EmptyLanguagesHeader`

- Primary construct: Header
- Relevant referenced constructs: %sin alignment, language metadata
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Missing languages_contents in @Languages header`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/languages.rs:114`, `rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/languages.rs:137`, `rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/languages.rs:164`, `rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/languages.rs:44`, `rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/languages.rs:75`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/header.rs:48`
- Test refs: None

### `E508` `EmptyDateHeader`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `@Languages header cannot be empty`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs:116`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/header.rs:58`
- Test refs: None

### `E509` `EmptyMediaHeader`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `@Date header cannot be empty`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs:131`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/header.rs:68`
- Test refs: None

### `E510` `EmptyIDLanguage`

- Primary construct: Header
- Relevant referenced constructs: language metadata
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `ID header language field cannot be empty`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/checkers.rs:11`
- Test refs: None

### `E511` `EmptyIDSpeaker`

- Primary construct: Header
- Relevant referenced constructs: speaker codes
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `ID header language field cannot be empty`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/checkers.rs:21`
- Test refs: None

### `E512` `EmptyParticipantCode`

- Primary construct: Header
- Relevant referenced constructs: participants
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Participant speaker code cannot be empty`
- Emission refs: `rust/crates/talkbank-model/src/model/header/codes/participant.rs:54`
- Test refs: None

### `E513` `EmptyParticipantRole`

- Primary construct: Header
- Relevant referenced constructs: participants
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Participant role cannot be empty`
- Emission refs: `rust/crates/talkbank-model/src/model/header/codes/participant.rs:70`
- Test refs: None

### `E514` `MissingLanguageCode`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E515` `EmptyIDRole`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %sin alignment, media bullets/timestamps
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Invalid bullet start timestamp: '{}'`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/checkers.rs:33`, `rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/inline_bullet.rs:104`, `rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/inline_bullet.rs:57`, `rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/inline_bullet.rs:73`, `rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/inline_bullet.rs:92`
- Test refs: None

### `E516` `EmptyDate`

- Primary construct: Header
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `@Date header should not be empty`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/checkers.rs:74`
- Test refs: None

### `E517` `InvalidAgeFormat`

- Primary construct: Header
- Relevant referenced constructs: form-type markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Use a valid CHAT role such as: {}`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/checkers.rs:58`
- Test refs: `rust/tests/mutation_tests/headers.rs:252`

### `E518` `InvalidDateFormat`

- Primary construct: Header
- Relevant referenced constructs: form-type markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Invalid @Date format (expected DD-MMM-YYYY with hyphens)`
- Suggested fix example: `Use format: 01-JAN-2024 (two-digit day, uppercase month, four-digit year)`
- Emission refs: `rust/crates/talkbank-model/src/model/header/codes/date.rs:107`, `rust/crates/talkbank-model/src/model/header/codes/date.rs:20`, `rust/crates/talkbank-model/src/model/header/codes/date.rs:39`, `rust/crates/talkbank-model/src/model/header/codes/date.rs:51`, `rust/crates/talkbank-model/src/model/header/codes/date.rs:63`, `rust/crates/talkbank-model/src/model/header/codes/date.rs:90`
- Test refs: `rust/tests/mutation_tests/headers.rs:206`, `rust/tests/mutation_tests/headers.rs:218`

### `E519` `InvalidLanguageCode`

- Primary construct: Header
- Relevant referenced constructs: language metadata
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `Language code '{}' should be 3 characters (got {})`
- Suggested fix example: `Use ISO 639-3 three-letter language codes (e.g., eng, spa, deu)`
- Emission refs: `rust/crates/talkbank-model/src/model/header/codes/language.rs:145`, `rust/crates/talkbank-model/src/model/header/codes/language.rs:163`
- Test refs: None

### `E522` `SpeakerNotDefined`

- Primary construct: Generic/Internal
- Relevant referenced constructs: participants, speaker codes
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Participant '{}' listed in @Participants but has no @ID header`
- Emission refs: `rust/crates/talkbank-direct-parser/src/file.rs:377`, `rust/crates/talkbank-model/src/model/file/utterance/validate.rs:15`, `rust/crates/talkbank-model/src/validation/header/structure.rs:88`, `rust/crates/talkbank-parser/src/parser/participants/builder.rs:87`
- Test refs: `rust/tests/full_line_context_test.rs:89`, `rust/tests/mutation_tests/headers.rs:278`, `rust/tests/mutation_tests/headers.rs:287`, `rust/tests/test_participant_errors/e522.rs:43`, `rust/tests/test_validation_comprehensive/integration.rs:25`, `rust/tests/test_validation_comprehensive/integration.rs:51`

### `E523` `OrphanIDHeader`

- Primary construct: Direct parser
- Relevant referenced constructs: participants
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `@ID header for '{}' but speaker not in @Participants`
- Emission refs: `rust/crates/talkbank-direct-parser/src/file.rs:401`, `rust/crates/talkbank-parser/src/parser/participants/builder.rs:114`
- Test refs: `rust/tests/mutation_tests/headers.rs:279`, `rust/tests/mutation_tests/headers.rs:287`, `rust/tests/test_participant_errors/e523.rs:39`

### `E524` `BirthUnknownParticipant`

- Primary construct: Direct parser
- Relevant referenced constructs: participants
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `@Birth header for '{}' but speaker not a declared participant`
- Emission refs: `rust/crates/talkbank-direct-parser/src/file.rs:425`, `rust/crates/talkbank-parser/src/parser/participants/builder.rs:147`
- Test refs: `rust/tests/test_participant_errors/e524.rs:44`

### `E525` `UnknownHeader`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Potential mismatch: message uses implementation terms
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `header choice node has no child`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/unknown.rs:17`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/dispatch/core.rs:18`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/dispatch/mod.rs:43`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/helpers.rs:49`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/pre_begin.rs:77`
- Test refs: None

### `E526` `UnmatchedBeginGem`

- Primary construct: Header
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unmatched @Bg: {} @Bg without matching @Eg`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/structure.rs:264`
- Test refs: None

### `E527` `UnmatchedEndGem`

- Primary construct: Header
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Unmatched @Eg (no matching @Bg)`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/structure.rs:202`, `rust/crates/talkbank-model/src/validation/header/structure.rs:230`
- Test refs: None

### `E528` `GemLabelMismatch`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E529` `NestedBeginGem`

- Primary construct: Header
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Nested @Bg: cannot open @Bg while already inside an unlabeled @Bg scope`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/structure.rs:134`
- Test refs: `rust/tests/validation_gaps.rs:60`

### `E530` `LazyGemInsideScope`

- Primary construct: Header
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `@G (lazy gem) cannot appear inside @Bg/@Eg scope`
- Emission refs: `rust/crates/talkbank-model/src/validation/header/structure.rs:170`
- Test refs: `rust/tests/validation_gaps.rs:76`

### `E531` `MediaFilenameMismatch`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Media filename '{}' does not match file name '{}' (case-insensitive comparison)`
- Emission refs: `rust/crates/talkbank-model/src/model/file/chat_file/validate.rs:361`
- Test refs: `rust/tests/validation_gaps.rs:120`

### `E532` `InvalidParticipantRole`

- Primary construct: Header
- Relevant referenced constructs: participants
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Invalid participant role: '{}'`
- Emission refs: `rust/crates/talkbank-model/src/model/header/codes/participant.rs:83`, `rust/crates/talkbank-model/src/validation/header/checkers.rs:44`
- Test refs: `rust/tests/validation_gaps.rs:198`

### `E533` `EmptyOptionsHeader`

- Primary construct: Header
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `@Options header cannot be empty`
- Emission refs: `rust/crates/talkbank-model/src/model/header/header_enum/header.rs:256`
- Test refs: None

## E6xx Tier Validation

| Code | Name | Primary Construct | Referenced Constructs | Name/Message Fit | User-Friendly Message | Suggested Fixes | Coverage |
|---|---|---|---|---|---|---|---|
| `E600` | `TierValidationError` | Generic/Internal | None identified from static references | Potential mismatch: message uses implementation terms | Needs improvement (internal implementation wording) | Missing | None |
| `E601` | `InvalidDependentTier` | Generic/Internal | None identified from static references | Potential mismatch: message uses implementation terms | Needs improvement (internal implementation wording) | Missing | None |
| `E602` | `MalformedTierHeader` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `E604` | `GraWithoutMor` | Generic/Internal | %gra alignment, %mor alignment | Good fit | Good | Sensible | None |

### `E600` `TierValidationError`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Potential mismatch: message uses implementation terms
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Header {} not found in CST (found {} headers)`
- Emission refs: `rust/crates/talkbank-parser/src/parser/chat_file_parser/header_dispatch/finder.rs:91`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/header_dispatch/parse.rs:42`
- Test refs: None

### `E601` `InvalidDependentTier`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Potential mismatch: message uses implementation terms
- User-language assessment: Needs improvement (internal implementation wording)
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `dependent_tier choice node has no child`
- Emission refs: `rust/crates/talkbank-parser/src/parser/chat_file_parser/dependent_tier_dispatch/parse.rs:41`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/dependent_tier_dispatch/parse.rs:74`, `rust/crates/talkbank-parser/src/parser/tier_parsers/dependent_tier.rs:175`, `rust/crates/talkbank-parser/src/parser/tier_parsers/dependent_tier.rs:98`
- Test refs: None

### `E602` `MalformedTierHeader`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `E604` `GraWithoutMor`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %gra alignment, %mor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: None
- Message example: `%gra tier requires %mor tier to be present`
- Suggested fix example: `Add %mor tier before %gra, or remove %gra tier`
- Emission refs: `rust/crates/talkbank-model/src/model/file/utterance/validate.rs:65`
- Test refs: None

## E7xx Alignment/Temporal

| Code | Name | Primary Construct | Referenced Constructs | Name/Message Fit | User-Friendly Message | Suggested Fixes | Coverage |
|---|---|---|---|---|---|---|---|
| `E700` | `UnexpectedTierNode` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | Strong |
| `E701` | `TierBeginTimeNotMonotonic` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | Strong |
| `E702` | `InvalidMorphologyFormat` | Parser CST/Tree | %mor alignment, %pho alignment, %sin alignment, form-type markers | Good fit | Good | Needs review | Strong |
| `E703` | `UnexpectedMorphologyNode` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | Strong |
| `E704` | `SpeakerSelfOverlap` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | Strong |
| `E705` | `MorCountMismatchTooFew` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | Strong |
| `E706` | `MorCountMismatchTooMany` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | Strong |
| `E708` | `MalformedGrammarRelation` | Parser CST/Tree | %gra alignment, %sin alignment, form-type markers | Good fit | Good | Needs review | Strong |
| `E709` | `InvalidGrammarIndex` | Parser CST/Tree | %gra alignment | Good fit | Good | Needs review | Strong |
| `E710` | `UnexpectedGrammarNode` | Parser CST/Tree | %gra alignment, %sin alignment | Good fit | Good | Needs review | Strong |
| `E712` | `GraInvalidWordIndex` | Alignment / Temporal | %gra alignment, %wor alignment | Good fit | Good | Missing | Strong |
| `E713` | `GraInvalidHeadIndex` | Alignment / Temporal | %gra alignment | Good fit | Good | Missing | Strong |
| `E714` | `PhoCountMismatchTooFew` | Alignment / Temporal | %pho alignment | Good fit | Good | Sensible | Strong |
| `E715` | `PhoCountMismatchTooMany` | Alignment / Temporal | %pho alignment | Good fit | Good | Missing | Strong |
| `E718` | `SinCountMismatchTooFew` | Alignment / Temporal | %sin alignment | Good fit | Good | Sensible | Strong |
| `E719` | `SinCountMismatchTooMany` | Alignment / Temporal | %sin alignment | Good fit | Good | Missing | Strong |
| `E720` | `MorGraCountMismatch` | Alignment / Temporal | %gra alignment, %mor alignment | Good fit | Good | Missing | Strong |

### `E700` `UnexpectedTierNode`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: Strong
- Emission refs: None
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:107`

### `E701` `TierBeginTimeNotMonotonic`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: Strong
- Emission refs: None
- Test refs: `rust/crates/talkbank-model/tests/temporal_validation_tests.rs:109`, `rust/crates/talkbank-model/tests/temporal_validation_tests.rs:143`, `rust/crates/talkbank-model/tests/temporal_validation_tests.rs:335`, `rust/tests/alignment_corpus_tests/helpers.rs:108`

### `E702` `InvalidMorphologyFormat`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %mor alignment, %pho alignment, %sin alignment, form-type markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: Strong
- Message example: `hello n|world`
- Suggested fix example: `MOR chunks must have format: pos|stem (e.g., v|hello, n|world)`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/dependent_tier.rs:34`
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:109`

### `E703` `UnexpectedMorphologyNode`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: Strong
- Emission refs: None
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:110`

### `E704` `SpeakerSelfOverlap`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: Strong
- Emission refs: None
- Test refs: `rust/crates/talkbank-model/tests/temporal_validation_tests.rs:177`, `rust/crates/talkbank-model/tests/temporal_validation_tests.rs:211`, `rust/crates/talkbank-model/tests/temporal_validation_tests.rs:245`, `rust/crates/talkbank-model/tests/temporal_validation_tests.rs:283`, `rust/crates/talkbank-model/tests/temporal_validation_tests.rs:335`, `rust/tests/alignment_corpus_tests/helpers.rs:111`

### `E705` `MorCountMismatchTooFew`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: Strong
- Emission refs: None
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:112`, `rust/tests/alignment_corpus_tests/sad_path.rs:25`, `rust/tests/mutation_tests/alignment.rs:38`

### `E706` `MorCountMismatchTooMany`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: Strong
- Emission refs: None
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:113`, `rust/tests/alignment_corpus_tests/sad_path.rs:51`, `rust/tests/mutation_tests/alignment.rs:66`

### `E708` `MalformedGrammarRelation`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %gra alignment, %sin alignment, form-type markers
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: Strong
- Message example: `MOR chunks must have format: pos|stem (e.g., v|hello, n|world)`
- Suggested fix example: `MOR chunks must have format: pos|stem (e.g., v|hello, n|world)`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:112`, `rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:124`, `rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:31`, `rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:62`, `rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:78`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/dependent_tier.rs:47`
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:114`

### `E709` `InvalidGrammarIndex`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %gra alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: Strong
- Message example: `Index cannot be 0 (indices are 1-indexed)`
- Suggested fix example: `Index must start at 1 for the first word`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:46`
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:115`

### `E710` `UnexpectedGrammarNode`

- Primary construct: Parser CST/Tree
- Relevant referenced constructs: %gra alignment, %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: Strong
- Message example: `Invalid GRA relation - non-numeric index`
- Suggested fix example: `GRA relation indices must be numbers (e.g., 1|2|SUBJ, not one|2|SUBJ)`
- Emission refs: `rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:93`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/dependent_tier.rs:18`, `rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs:17`
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:116`

### `E712` `GraInvalidWordIndex`

- Primary construct: Alignment / Temporal
- Relevant referenced constructs: %gra alignment, %wor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `%mor tier has more chunks than %gra tier: expected {} gra relations, found {} (Note: %mor chunks include clitics)`
- Emission refs: `rust/crates/talkbank-model/src/alignment/gra/align.rs:90`
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:117`, `rust/tests/mutation_tests/alignment.rs:96`

### `E713` `GraInvalidHeadIndex`

- Primary construct: Alignment / Temporal
- Relevant referenced constructs: %gra alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `%gra tier is longer than %mor tier: expected {} gra relations, found {}`
- Emission refs: `rust/crates/talkbank-model/src/alignment/gra/align.rs:120`
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:118`

### `E714` `PhoCountMismatchTooFew`

- Primary construct: Alignment / Temporal
- Relevant referenced constructs: %pho alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Main tier has more alignable content than %pho tier: expected {} phonological tokens, found {}`
- Suggested fix example: `Add phonological tokens to %pho tier to match main tier words`
- Emission refs: `rust/crates/talkbank-model/src/alignment/pho.rs:130`, `rust/crates/talkbank-model/src/alignment/pho.rs:76`
- Test refs: `rust/crates/talkbank-model/src/alignment/location_tests.rs:154`, `rust/crates/talkbank-model/src/alignment/location_tests.rs:32`, `rust/tests/alignment_corpus_tests/helpers.rs:119`, `rust/tests/alignment_corpus_tests/sad_path.rs:115`

### `E715` `PhoCountMismatchTooMany`

- Primary construct: Alignment / Temporal
- Relevant referenced constructs: %pho alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `%pho tier is longer than main tier: expected {} phonological tokens, found {}`
- Emission refs: `rust/crates/talkbank-model/src/alignment/pho.rs:148`, `rust/crates/talkbank-model/src/alignment/pho.rs:93`
- Test refs: `rust/crates/talkbank-model/src/alignment/location_tests.rs:64`, `rust/tests/alignment_corpus_tests/helpers.rs:120`, `rust/tests/alignment_corpus_tests/sad_path.rs:141`

### `E718` `SinCountMismatchTooFew`

- Primary construct: Alignment / Temporal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Sensible
- Coverage: Strong
- Message example: `Main tier has more alignable content than %sin tier: expected {} gesture/sign tokens, found {}`
- Suggested fix example: `Add gesture/sign tokens to %sin tier to match main tier words`
- Emission refs: `rust/crates/talkbank-model/src/alignment/sin.rs:76`
- Test refs: `rust/crates/talkbank-model/src/alignment/location_tests.rs:95`, `rust/tests/alignment_corpus_tests/helpers.rs:121`

### `E719` `SinCountMismatchTooMany`

- Primary construct: Alignment / Temporal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `%sin tier is longer than main tier: expected {} gesture/sign tokens, found {}`
- Emission refs: `rust/crates/talkbank-model/src/alignment/sin.rs:93`
- Test refs: `rust/crates/talkbank-model/src/alignment/location_tests.rs:124`, `rust/tests/alignment_corpus_tests/helpers.rs:122`

### `E720` `MorGraCountMismatch`

- Primary construct: Alignment / Temporal
- Relevant referenced constructs: %gra alignment, %mor alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `Tier type mismatch: %{} tier cannot align with %{} tier (use %mor↔%gra, %trn↔%grt, or %umor↔%ugra)`
- Emission refs: `rust/crates/talkbank-model/src/alignment/gra/align.rs:50`
- Test refs: `rust/tests/alignment_corpus_tests/helpers.rs:123`

## Other

| Code | Name | Primary Construct | Referenced Constructs | Name/Message Fit | User-Friendly Message | Suggested Fixes | Coverage |
|---|---|---|---|---|---|---|---|
| `E999` | `UnknownError` | Generic/Internal | %sin alignment | Good fit | Good | Missing | Strong |

### `E999` `UnknownError`

- Primary construct: Generic/Internal
- Relevant referenced constructs: %sin alignment
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: Strong
- Message example: `No dependent tier found in parsed output`
- Emission refs: `rust/crates/talkbank-model/src/enhance.rs:154`, `rust/crates/talkbank-model/src/enhance.rs:205`, `rust/crates/talkbank-model/src/enhance.rs:239`, `rust/crates/talkbank-model/src/enhance.rs:262`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/parse_tiers.rs:107`, `rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/parse_tiers.rs:91`
- Test refs: `rust/tests/full_line_context_test.rs:131`, `rust/tests/tui_file_ordering_test.rs:32`, `rust/tests/tui_file_ordering_test.rs:44`, `rust/tests/tui_file_ordering_test.rs:56`, `rust/tests/tui_realtime_display_test.rs:50`, `rust/tests/tui_realtime_display_test.rs:67`

## Wxxx Warnings

| Code | Name | Primary Construct | Referenced Constructs | Name/Message Fit | User-Friendly Message | Suggested Fixes | Coverage |
|---|---|---|---|---|---|---|---|
| `W001` | `GenericWarning` | Generic/Internal | None identified from static references | Good fit | Good | Missing | None |
| `W108` | `SpeakerNotFoundInParticipants` | Generic/Internal | participants, speaker codes | Good fit | Good | Needs review | None |
| `W210` | `MissingWhitespaceBeforeContent` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `W211` | `MissingWhitespaceAfterOverlap` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |
| `W601` | `EmptyUserDefinedTier` | Generic/Internal | None identified from static references | Good fit | Good | Needs review | None |
| `W602` | `UnknownUserDefinedTier` | Generic/Internal | None identified from static references | Good fit | Good | Missing | None |
| `W999` | `LegacyWarning` | No primary emission site found (manual review) | None identified from static references | No message found at emission site (manual review needed) | No message evidence | Missing | None |

### `W001` `GenericWarning`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Warning message`
- Emission refs: `rust/crates/talkbank-model/src/tests.rs:45`, `rust/crates/talkbank-model/src/tests.rs:97`
- Test refs: None

### `W108` `SpeakerNotFoundInParticipants`

- Primary construct: Generic/Internal
- Relevant referenced constructs: participants, speaker codes
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `*MO: how are you ?`
- Suggested fix example: `Did you mean 'MOT' (Mother)?`
- Emission refs: `rust/crates/talkbank-model/src/model/file/utterance/tests.rs:34`
- Test refs: None

### `W210` `MissingWhitespaceBeforeContent`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `W211` `MissingWhitespaceAfterOverlap`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

### `W601` `EmptyUserDefinedTier`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Needs review
- Coverage: None
- Message example: `User-defined tier %x{} has no content`
- Suggested fix example: `User-defined tiers should contain custom analysis/annotation data`
- Emission refs: `rust/crates/talkbank-model/src/validation/unparsed_tier.rs:11`
- Test refs: None

### `W602` `UnknownUserDefinedTier`

- Primary construct: Generic/Internal
- Relevant referenced constructs: None identified from static references
- Name/message assessment: Good fit
- User-language assessment: Good
- Suggested-fix assessment: Missing
- Coverage: None
- Message example: `Deprecated experimental tier %x{}: should be updated to %{}`
- Emission refs: `rust/crates/talkbank-model/src/validation/unparsed_tier.rs:45`
- Test refs: None

### `W999` `LegacyWarning`

- Primary construct: No primary emission site found (manual review)
- Relevant referenced constructs: None identified from static references
- Name/message assessment: No message found at emission site (manual review needed)
- User-language assessment: No message evidence
- Suggested-fix assessment: Missing
- Coverage: None
- Emission refs: None
- Test refs: None

