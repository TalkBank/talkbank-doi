# CHECK Parity Audit (CLAN CHECK vs TalkBank)

Reference basis: an internal normalized transcription of CLAN CHECK rule tables plus manual review against the published CLAN manual and current Rust behavior. Maintainer-only provenance notes for the local source artifacts are archived privately.

## Executive Summary

- CHECK rules parsed: `87`
- Overlap with TalkBank codes: `70`
- CHECK rules missing direct TalkBank mapping: `17`
- Semantic parity `full`: `70`
- Behavioral parity `full`: `57`
- Intentional divergence (semantic full + behavioral partial due to CHECK anomalies): `13`
- TalkBank enhancements beyond CHECK (no mapped CHECK rule): `122`

## Method

- Parsed CHECK rules from markdown error tables (`| # | Message | ... |`).
- Mapped CHECK rules to TalkBank codes via explicit ID mapping plus keyword fallback.
- Reported two parity dimensions:
  - `semantic`: intended rule meaning parity.
  - `behavioral`: literal CHECK runtime behavior parity (including documented anomalies).
- Strictness policy: TalkBank should be at least as strict semantically.

## Master Mapping (CHECK -> TalkBank)

| CHECK # | CHECK Message | Category | TalkBank Codes | Semantic | Behavioral | Strictness | Divergence | Action | Priority |
|---:|---|---|---|---|---|---|---|---|---|
| 1 | Expected characters are: @ or % or *. | File Structure Errors (1-10) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 2 | Missing ':' character and argument. | File Structure Errors (1-10) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 3 | Missing either TAB or SPACE character. | File Structure Errors (1-10) | `E243`, `W210`, `W211` | full | full | equal | none | no action | P3 |
| 4 | Found a space character instead of TAB character after Tier name | File Structure Errors (1-10) | `E243`, `W210`, `W211` | full | full | equal | none | no action | P3 |
| 5 | Colon (:) character is illegal. | File Structure Errors (1-10) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 6 | "@Begin" is missing at the beginning of the file. | File Structure Errors (1-10) | `E501` | full | full | equal | none | no action | P3 |
| 7 | "@End" is missing at the end of the file. | File Structure Errors (1-10) | `E502` | full | full | equal | none | no action | P3 |
| 8 | Expected characters are: @ % * TAB. | File Structure Errors (1-10) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 9 | Tier name is longer than [SPEAKERLEN]. | File Structure Errors (1-10) | `E308`, `E522`, `E532` | full | full | equal | none | no action | P3 |
| 10 | Tier text is longer than [UTTLINELEN]. | File Structure Errors (1-10) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 11 | Symbol is not declared in the depfile. | Declaration Errors (11-20) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 12 | Missing speaker name and/or role. | Declaration Errors (11-20) | `E308`, `E522`, `E532` | full | full | equal | none | no action | P3 |
| 13 | Duplicate speaker declaration. | Declaration Errors (11-20) | `E308`, `E522`, `E532` | full | full | equal | none | no action | P3 |
| 14 | Spaces before tier code. | Declaration Errors (11-20) | `E243`, `W210`, `W211` | full | full | equal | none | no action | P3 |
| 15 | Illegal role. Please see "depfile.cut" for list of roles. | Declaration Errors (11-20) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 16 | Illegal use of extended characters in speaker names. | Declaration Errors (11-20) | `E308`, `E522`, `E532` | full | full | equal | none | no action | P3 |
| 17 | Tier is not declared in depfile file. | Declaration Errors (11-20) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 18 | Speaker [X] is not specified in a participants list. | Declaration Errors (11-20) | `E308`, `E522` | full | full | equal | none | no action | P3 |
| 19 | Illegal use of delimiter in a word. Or a SPACE should be added after it. | Declaration Errors (11-20) | `E243`, `E304`, `E305`, `E360`, `W210`, `W211` | full | full | equal | none | no action | P3 |
| 20 | Undeclared suffix in depfile. | Declaration Errors (11-20) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 21 | Utterance delimiter expected. | Bracket/Delimiter Errors (21-30) | `E304` | full | full | equal | none | no action | P3 |
| 22 | Unmatched [ found on the tier. | Bracket/Delimiter Errors (21-30) | `E345` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 23 | Unmatched ] found on the tier. | Bracket/Delimiter Errors (21-30) | `E346` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 24 | Unmatched < found on the tier. | Bracket/Delimiter Errors (21-30) | `E347` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 25 | Unmatched > found on the tier. | Bracket/Delimiter Errors (21-30) | `E348` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 26 | Unmatched { found on the tier. | Bracket/Delimiter Errors (21-30) | `E230`, `E231`, `E242`, `E345`, `E346`, `E356`, `E357` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 27 | Unmatched } found on the tier. | Bracket/Delimiter Errors (21-30) | `E230`, `E231`, `E242`, `E345`, `E346`, `E356`, `E357` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 28 | Unmatched ( found on the tier. | Bracket/Delimiter Errors (21-30) | `E230`, `E231`, `E242`, `E345`, `E346`, `E356`, `E357` | full | full | equal | none | no action | P3 |
| 29 | Unmatched ) found on the tier. | Bracket/Delimiter Errors (21-30) | `E230`, `E231`, `E242`, `E345`, `E346`, `E356`, `E357` | full | full | equal | none | no action | P3 |
| 30 | Text is illegal. | Bracket/Delimiter Errors (21-30) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 31 | Missing text after the colon. | Content Errors (31-50) | `E305` | full | full | equal | none | no action | P3 |
| 32 | Code is not declared in depfile. | Content Errors (31-50) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 36 | Utterance delimiter must be at the end of the utterance. | Content Errors (31-50) | `E305` | full | full | equal | none | no action | P3 |
| 38 | Numbers should be written out in words. | Content Errors (31-50) | `E220` | full | full | equal | none | no action | P3 |
| 40 | Duplicate code tiers per one main tier are NOT allowed. | Content Errors (31-50) | `E401`, `E705`, `E706`, `E720` | full | full | equal | none | no action | P3 |
| 41 | Parentheses around words are illegal. | Content Errors (31-50) | `E212` | full | full | equal | none | no action | P3 |
| 43 | The file must start with "@Begin" tier. | Content Errors (31-50) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 44 | The file must end with "@End" tier. | Content Errors (31-50) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 47 | Numbers are not allowed inside words. | Content Errors (31-50) | `E220` | full | full | equal | none | no action | P3 |
| 48 | Illegal character(s) [X] found. | Content Errors (31-50) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 50 | Redundant utterance delimiter. | Content Errors (31-50) | `E305` | full | full | equal | none | no action | P3 |
| 51 | expected [ ]; < > should be followed by [ ] | Scope/Structure Errors (51-70) | `E347`, `E348` | full | full | equal | none | no action | P3 |
| 52 | This item must be preceded by text. | Scope/Structure Errors (51-70) | `E370` | full | full | equal | none | no action | P3 |
| 55 | Unmatched ( found in the word. | Scope/Structure Errors (51-70) | `E231` | full | full | equal | none | no action | P3 |
| 56 | Unmatched ) found in the word. | Scope/Structure Errors (51-70) | `E231` | full | full | equal | none | no action | P3 |
| 57 | Please add space between word and pause symbol. | Scope/Structure Errors (51-70) | `E243` | full | full | equal | none | no action | P3 |
| 58 | Tier name is longer than 8 characters. | Scope/Structure Errors (51-70) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 60 | "@ID:" tier is missing in the file. | Scope/Structure Errors (51-70) | `E522` | full | full | equal | none | no action | P3 |
| 68 | PARTICIPANTS TIER IS MISSING "CHI Target_Child". | Scope/Structure Errors (51-70) | `E522`, `E523`, `E524` | full | full | equal | none | no action | P3 |
| 69 | The UTF8 header is missing. | Scope/Structure Errors (51-70) | `E507` | full | full | equal | none | no action | P3 |
| 70 | Expected either text or "0" on this tier. | Scope/Structure Errors (51-70) | `E253` | full | full | equal | none | no action | P3 |
| 81 | Bullet must follow utterance delimiter or be followed by end-of-line. | Media/Bullet Errors (81-90) | `E360` | full | full | equal | none | no action | P3 |
| 82 | BEG mark of bullet must be smaller than END mark. | Media/Bullet Errors (81-90) | `E361` | full | full | equal | none | no action | P3 |
| 83 | Current BEG time is smaller than previous' tier BEG time. | Media/Bullet Errors (81-90) | `E362`, `E701` | full | full | equal | none | no action | P3 |
| 84 | Current BEG time is smaller than previous' tier END time by [X] msec. | Media/Bullet Errors (81-90) | `E704` | full | full | equal | none | no action | P3 |
| 85 | Gap found between current BEG time and previous' tier END time. | Media/Bullet Errors (81-90) | `E700` | full | full | equal | none | no action | P3 |
| 89 | Missing or extra or wrong characters found in bullet. | Media/Bullet Errors (81-90) | `E360`, `E361` | full | full | equal | none | no action | P3 |
| 90 | Illegal time representation inside a bullet. | Media/Bullet Errors (81-90) | `E360`, `E361` | full | full | equal | none | no action | P3 |
| 91 | Blank lines are not allowed. | Format Errors (91-120) | `E303` | full | full | equal | none | no action | P3 |
| 92 | This item must be followed by space or end-of-line. | Format Errors (91-120) | `E243`, `W210`, `W211` | full | full | equal | none | no action | P3 |
| 93 | This item must be preceded by SPACE. | Format Errors (91-120) | `E243`, `W210`, `W211` | full | full | equal | none | no action | P3 |
| 94 | Mismatch of speaker and %mor: utterance delimiters. | Format Errors (91-120) | `E705`, `E706`, `E714`, `E715`, `E718`, `E719`, `E720` | full | full | equal | none | no action | P3 |
| 102 | Italic markers are no longer legal in CHAT. | Format Errors (91-120) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 107 | Only single commas are allowed in tier. | Format Errors (91-120) | None | none | none | TalkBank looser | bug-risk | add rule | P1 |
| 110 | No bullet found on this tier. | Format Errors (91-120) | `E360` | full | full | equal | none | no action | P3 |
| 117 | This character must be used in pairs. | Format Errors (91-120) | `E230`, `E356`, `E357` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 118 | Utterance delimiter must precede final bullet. | Format Errors (91-120) | `E360` | full | full | equal | none | no action | P3 |
| 120 | Please use [X] language code instead. | Language/Code Errors (120-140) | `E248` | full | full | equal | none | no action | P3 |
| 121 | Language code [X] not found in ISO-639.cut | Language/Code Errors (120-140) | `E254`, `E519` | full | full | equal | none | no action | P3 |
| 122 | Language on @ID tier is not defined on "@Languages:" | Language/Code Errors (120-140) | `E519` | full | full | equal | none | no action | P3 |
| 128 | Unmatched ‹ found on the tier. | Language/Code Errors (120-140) | `E345` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 129 | Unmatched › found on the tier. | Language/Code Errors (120-140) | `E346` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 130 | Unmatched 〔 found on the tier. | Language/Code Errors (120-140) | `E345` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 131 | Unmatched 〕 found on the tier. | Language/Code Errors (120-140) | `E346` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 136 | Unmatched " found on the tier. | Language/Code Errors (120-140) | `E242` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 137 | Unmatched " found on the tier. | Language/Code Errors (120-140) | `E242` | full | partial | TalkBank stricter | intentional | no action | P2 |
| 140 | Tier "%MOR:" does not link in size to its speaker tier. | Language/Code Errors (120-140) | `E401`, `E705`, `E706`, `E720` | full | full | equal | none | no action | P3 |
| 141 | [: ...] has to be preceded by only one word and nothing else. | Role/ID Errors (141-161) | `E387`, `E388`, `E389` | full | full | equal | none | no action | P3 |
| 142 | Speaker's role on @ID tier does not match role on @Participants: | Role/ID Errors (141-161) | `E532` | full | full | equal | none | no action | P3 |
| 143 | The @ID line needs 10 fields. | Role/ID Errors (141-161) | `E505` | full | full | equal | none | no action | P3 |
| 151 | This word has only repetition segments. | Role/ID Errors (141-161) | `E370` | full | full | equal | none | no action | P3 |
| 153 | Age's month or day are missing initial zero. | Role/ID Errors (141-161) | `E517` | full | full | equal | none | no action | P3 |
| 155 | Please use "0word" instead of "(word)". | Role/ID Errors (141-161) | `E212` | full | full | equal | none | no action | P3 |
| 156 | Please replace ,, with F2-t (‚) character. | Role/ID Errors (141-161) | `E243` | full | full | equal | none | no action | P3 |
| 159 | Pause markers should appear after retrace markers. | Role/ID Errors (141-161) | `E370` | full | full | equal | none | no action | P3 |
| 160 | Space character is not allowed after '<' or before '>' character. | Role/ID Errors (141-161) | `E243`, `W210`, `W211` | full | full | equal | none | no action | P3 |
| 161 | Space character is required before '[' code item. | Role/ID Errors (141-161) | `E243`, `W210`, `W211` | full | full | equal | none | no action | P3 |

## Gaps: CHECK Rules Missing in TalkBank

- CHECK `1`: Expected characters are: @ or % or *. (`File Structure Errors (1-10)`) -> action: `add rule` (P1)
- CHECK `2`: Missing ':' character and argument. (`File Structure Errors (1-10)`) -> action: `add rule` (P1)
- CHECK `5`: Colon (:) character is illegal. (`File Structure Errors (1-10)`) -> action: `add rule` (P1)
- CHECK `8`: Expected characters are: @ % * TAB. (`File Structure Errors (1-10)`) -> action: `add rule` (P1)
- CHECK `10`: Tier text is longer than [UTTLINELEN]. (`File Structure Errors (1-10)`) -> action: `add rule` (P1)
- CHECK `11`: Symbol is not declared in the depfile. (`Declaration Errors (11-20)`) -> action: `add rule` (P1)
- CHECK `15`: Illegal role. Please see "depfile.cut" for list of roles. (`Declaration Errors (11-20)`) -> action: `add rule` (P1)
- CHECK `17`: Tier is not declared in depfile file. (`Declaration Errors (11-20)`) -> action: `add rule` (P1)
- CHECK `20`: Undeclared suffix in depfile. (`Declaration Errors (11-20)`) -> action: `add rule` (P1)
- CHECK `30`: Text is illegal. (`Bracket/Delimiter Errors (21-30)`) -> action: `add rule` (P1)
- CHECK `32`: Code is not declared in depfile. (`Content Errors (31-50)`) -> action: `add rule` (P1)
- CHECK `43`: The file must start with "@Begin" tier. (`Content Errors (31-50)`) -> action: `add rule` (P1)
- CHECK `44`: The file must end with "@End" tier. (`Content Errors (31-50)`) -> action: `add rule` (P1)
- CHECK `48`: Illegal character(s) [X] found. (`Content Errors (31-50)`) -> action: `add rule` (P1)
- CHECK `58`: Tier name is longer than 8 characters. (`Scope/Structure Errors (51-70)`) -> action: `add rule` (P1)
- CHECK `102`: Italic markers are no longer legal in CHAT. (`Format Errors (91-120)`) -> action: `add rule` (P1)
- CHECK `107`: Only single commas are allowed in tier. (`Format Errors (91-120)`) -> action: `add rule` (P1)

## Intentional Divergences (Behavioral Mismatch, Semantic Match)

- CHECK `22` Unmatched [ found on the tier. -> TalkBank E345. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `23` Unmatched ] found on the tier. -> TalkBank E346. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `24` Unmatched < found on the tier. -> TalkBank E347. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `25` Unmatched > found on the tier. -> TalkBank E348. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `26` Unmatched { found on the tier. -> TalkBank E230, E231, E242, E345, E346, E356, E357. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `27` Unmatched } found on the tier. -> TalkBank E230, E231, E242, E345, E346, E356, E357. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `117` This character must be used in pairs. -> TalkBank E230, E356, E357. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `128` Unmatched ‹ found on the tier. -> TalkBank E345. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `129` Unmatched › found on the tier. -> TalkBank E346. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `130` Unmatched 〔 found on the tier. -> TalkBank E345. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `131` Unmatched 〕 found on the tier. -> TalkBank E346. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `136` Unmatched " found on the tier. -> TalkBank E242. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.
- CHECK `137` Unmatched " found on the tier. -> TalkBank E242. Rationale: CHECK rule is known to have counter/toggle anomaly; TalkBank should match semantic intent, not flawed literal behavior.

## TalkBank Enhancements Beyond CHECK

- `E001` `InternalError`
- `E002` `TestError`
- `E003` `EmptyString`
- `E101` `InvalidLineFormat`
- `E301` `MissingMainTier`
- `E302` `MissingNode`
- `E306` `EmptyUtterance`
- `E307` `InvalidSpeaker`
- `E309` `UnexpectedSyntax`
- `E310` `ParseFailed`
- `E311` `UnexpectedNode`
- `E312` `UnclosedBracket`
- `E313` `UnclosedParenthesis`
- `E314` `IncompleteAnnotation`
- `E315` `InvalidControlCharacter`
- `E316` `UnparsableContent`
- `E317` `UnparsableFileContent`
- `E318` `UnparsableDependentTier`
- `E319` `UnparsableLine`
- `E320` `UnparsableHeader`
- `E321` `UnparsableUtterance`
- `E322` `EmptyColon`
- `E323` `MissingColonAfterSpeaker`
- `E324` `UnrecognizedUtteranceError`
- `E325` `UnexpectedUtteranceChild`
- `E326` `UnexpectedLineType`
- `E330` `TreeParsingError`
- `E331` `UnexpectedNodeInContext`
- `E340` `UnknownBaseContent`
- `E341` `UnbalancedQuotationCrossUtterance`
- `E342` `MissingRequiredElement`
- `E344` `InvalidScopedAnnotationNesting`
- `E350` `GenericAnnotationError`
- `E351` `MissingQuoteBegin`
- `E352` `MissingQuoteEnd`
- `E353` `MissingOtherCompletionContext`
- `E354` `MissingTrailingOffTerminator`
- `E355` `InterleavedScopedAnnotations`
- `E358` `UnmatchedLongFeatureBegin`
- `E359` `UnmatchedLongFeatureEnd`
- `E363` `InvalidPostcode`
- `E364` `MalformedWordContent`
- `E365` `MalformedTierContent`
- `E366` `LongFeatureLabelMismatch`
- `E367` `UnmatchedNonvocalBegin`
- `E368` `UnmatchedNonvocalEnd`
- `E369` `NonvocalLabelMismatch`
- `E371` `PauseInPhoGroup`
- `E372` `NestedQuotation`
- `E373` `InvalidOverlapIndex`
- `E374` `ErrorAnnotationParseError`
- `E375` `ScopedAnnotationParseError`
- `E376` `ReplacementParseError`
- `E377` `RetraceParseError`
- `E378` `OverlapAnnotationParseError`
- `E380` `UnknownSeparator`
- `E381` `PhoParseError`
- `E382` `MorParseError`
- `E383` `GraParseError`
- `E384` `SinParseError`
- `E385` `WordParseError`
- `E386` `TextTierParseError`
- `E390` `ReplacementContainsOmission`
- `E391` `ReplacementContainsUntranscribed`
- `E202` `MissingFormType`
- `E203` `InvalidFormType`
- `E207` `UnknownAnnotation`
- `E208` `EmptyReplacement`
- `E209` `EmptySpokenContent`
- `E210` `IllegalReplacementForFragment` (deprecated)
- `E211` `OmissionInReplacement` (deprecated)
- `E213` `UntranscribedInReplacement` (deprecated)
- `E214` `EmptyAnnotatedScopedAnnotations`
- `E232` `InvalidCompoundMarkerPosition`
- `E233` `EmptyCompoundPart`
- `E241` `IllegalUntranscribed`
- `E244` `ConsecutiveStressMarkers`
- `E245` `StressNotBeforeSpokenMaterial`
- `E246` `LengtheningNotAfterSpokenMaterial`
- `E247` `MultiplePrimaryStress`
- `E249` `MissingLanguageContext`
- `E250` `SecondaryStressWithoutPrimary`
- `E251` `EmptyWordContentText`
- `E252` `SyllablePauseNotBetweenSpokenMaterial`
- `E404` `OrphanedDependentTier`
- `E504` `MissingRequiredHeader`
- `E506` `EmptyParticipantsHeader`
- `E508` `EmptyDateHeader`
- `E509` `EmptyMediaHeader`
- `E510` `EmptyIDLanguage`
- `E511` `EmptyIDSpeaker`
- `E512` `EmptyParticipantCode`
- `E513` `EmptyParticipantRole`
- `E514` `MissingLanguageCode`
- `E515` `EmptyIDRole`
- `E516` `EmptyDate`
- `E518` `InvalidDateFormat`
- `E525` `UnknownHeader`
- `E526` `UnmatchedBeginGem`
- `E527` `UnmatchedEndGem`
- `E528` `GemLabelMismatch`
- `E529` `NestedBeginGem`
- `E530` `LazyGemInsideScope`
- `E531` `MediaFilenameMismatch`
- `E533` `EmptyOptionsHeader`
- `E600` `TierValidationError`
- `E601` `InvalidDependentTier`
- `E602` `MalformedTierHeader`
- `E604` `GraWithoutMor`
- `E702` `InvalidMorphologyFormat`
- `E703` `UnexpectedMorphologyNode`
- `E708` `MalformedGrammarRelation`
- `E709` `InvalidGrammarIndex`
- `E710` `UnexpectedGrammarNode`
- `E712` `GraInvalidWordIndex`
- `E713` `GraInvalidHeadIndex`
- `W001` `GenericWarning`
- `W108` `SpeakerNotFoundInParticipants`
- `W601` `EmptyUserDefinedTier`
- `W602` `UnknownUserDefinedTier`
- `W999` `LegacyWarning`
- `E999` `UnknownError`

## Reverse Mapping (TalkBank -> CHECK)

| TalkBank Code | Variant | CHECK Rules |
|---|---|---|
| `E001` | `InternalError` | None |
| `E002` | `TestError` | None |
| `E003` | `EmptyString` | None |
| `E101` | `InvalidLineFormat` | None |
| `E301` | `MissingMainTier` | None |
| `E302` | `MissingNode` | None |
| `E303` | `SyntaxError` | 91 |
| `E304` | `MissingSpeaker` | 19, 21 |
| `E305` | `MissingTerminator` | 19, 31, 36, 50 |
| `E306` | `EmptyUtterance` | None |
| `E307` | `InvalidSpeaker` | None |
| `E308` | `UndeclaredSpeaker` | 9, 12, 13, 16, 18 |
| `E309` | `UnexpectedSyntax` | None |
| `E310` | `ParseFailed` | None |
| `E311` | `UnexpectedNode` | None |
| `E312` | `UnclosedBracket` | None |
| `E313` | `UnclosedParenthesis` | None |
| `E314` | `IncompleteAnnotation` | None |
| `E315` | `InvalidControlCharacter` | None |
| `E316` | `UnparsableContent` | None |
| `E317` | `UnparsableFileContent` | None |
| `E318` | `UnparsableDependentTier` | None |
| `E319` | `UnparsableLine` | None |
| `E320` | `UnparsableHeader` | None |
| `E321` | `UnparsableUtterance` | None |
| `E322` | `EmptyColon` | None |
| `E323` | `MissingColonAfterSpeaker` | None |
| `E324` | `UnrecognizedUtteranceError` | None |
| `E325` | `UnexpectedUtteranceChild` | None |
| `E326` | `UnexpectedLineType` | None |
| `E330` | `TreeParsingError` | None |
| `E331` | `UnexpectedNodeInContext` | None |
| `E340` | `UnknownBaseContent` | None |
| `E341` | `UnbalancedQuotationCrossUtterance` | None |
| `E342` | `MissingRequiredElement` | None |
| `E344` | `InvalidScopedAnnotationNesting` | None |
| `E345` | `UnmatchedScopedAnnotationBegin` | 22, 26, 27, 28, 29, 128, 130 |
| `E346` | `UnmatchedScopedAnnotationEnd` | 23, 26, 27, 28, 29, 129, 131 |
| `E347` | `UnbalancedOverlap` | 24, 51 |
| `E348` | `MissingOverlapEnd` | 25, 51 |
| `E350` | `GenericAnnotationError` | None |
| `E351` | `MissingQuoteBegin` | None |
| `E352` | `MissingQuoteEnd` | None |
| `E353` | `MissingOtherCompletionContext` | None |
| `E354` | `MissingTrailingOffTerminator` | None |
| `E355` | `InterleavedScopedAnnotations` | None |
| `E356` | `UnmatchedUnderlineBegin` | 26, 27, 28, 29, 117 |
| `E357` | `UnmatchedUnderlineEnd` | 26, 27, 28, 29, 117 |
| `E358` | `UnmatchedLongFeatureBegin` | None |
| `E359` | `UnmatchedLongFeatureEnd` | None |
| `E360` | `InvalidMediaBullet` | 19, 81, 89, 90, 110, 118 |
| `E361` | `InvalidTimestamp` | 82, 89, 90 |
| `E362` | `TimestampBackwards` | 83 |
| `E363` | `InvalidPostcode` | None |
| `E364` | `MalformedWordContent` | None |
| `E365` | `MalformedTierContent` | None |
| `E366` | `LongFeatureLabelMismatch` | None |
| `E367` | `UnmatchedNonvocalBegin` | None |
| `E368` | `UnmatchedNonvocalEnd` | None |
| `E369` | `NonvocalLabelMismatch` | None |
| `E370` | `StructuralOrderError` | 52, 151, 159 |
| `E371` | `PauseInPhoGroup` | None |
| `E372` | `NestedQuotation` | None |
| `E373` | `InvalidOverlapIndex` | None |
| `E374` | `ErrorAnnotationParseError` | None |
| `E375` | `ScopedAnnotationParseError` | None |
| `E376` | `ReplacementParseError` | None |
| `E377` | `RetraceParseError` | None |
| `E378` | `OverlapAnnotationParseError` | None |
| `E380` | `UnknownSeparator` | None |
| `E381` | `PhoParseError` | None |
| `E382` | `MorParseError` | None |
| `E383` | `GraParseError` | None |
| `E384` | `SinParseError` | None |
| `E385` | `WordParseError` | None |
| `E386` | `TextTierParseError` | None |
| `E387` | `ReplacementOnFragment` | 141 |
| `E388` | `ReplacementOnNonword` | 141 |
| `E389` | `ReplacementOnFiller` | 141 |
| `E390` | `ReplacementContainsOmission` | None |
| `E391` | `ReplacementContainsUntranscribed` | None |
| `E202` | `MissingFormType` | None |
| `E203` | `InvalidFormType` | None |
| `E207` | `UnknownAnnotation` | None |
| `E208` | `EmptyReplacement` | None |
| `E209` | `EmptySpokenContent` | None |
| `E210` | `IllegalReplacementForFragment` | None |
| `E211` | `OmissionInReplacement` | None |
| `E212` | `InvalidWordFormat` | 41, 155 |
| `E213` | `UntranscribedInReplacement` | None |
| `E214` | `EmptyAnnotatedScopedAnnotations` | None |
| `E220` | `IllegalDigits` | 38, 47 |
| `E230` | `UnbalancedCADelimiter` | 26, 27, 28, 29, 117 |
| `E231` | `UnbalancedShortening` | 26, 27, 28, 29, 55, 56 |
| `E232` | `InvalidCompoundMarkerPosition` | None |
| `E233` | `EmptyCompoundPart` | None |
| `E241` | `IllegalUntranscribed` | None |
| `E242` | `UnbalancedQuotation` | 26, 27, 28, 29, 136, 137 |
| `E243` | `IllegalCharactersInWord` | 3, 4, 14, 19, 57, 92, 93, 156, 160, 161 |
| `E244` | `ConsecutiveStressMarkers` | None |
| `E245` | `StressNotBeforeSpokenMaterial` | None |
| `E246` | `LengtheningNotAfterSpokenMaterial` | None |
| `E247` | `MultiplePrimaryStress` | None |
| `E248` | `TertiaryLanguageNeedsExplicitCode` | 120 |
| `E249` | `MissingLanguageContext` | None |
| `E250` | `SecondaryStressWithoutPrimary` | None |
| `E251` | `EmptyWordContentText` | None |
| `E252` | `SyllablePauseNotBetweenSpokenMaterial` | None |
| `E253` | `EmptyWordContent` | 70 |
| `E254` | `UndeclaredLanguageCode` | 121 |
| `E401` | `DuplicateDependentTier` | 40, 140 |
| `E404` | `OrphanedDependentTier` | None |
| `E501` | `DuplicateHeader` | 6 |
| `E502` | `MissingEndHeader` | 7 |
| `E504` | `MissingRequiredHeader` | None |
| `E505` | `InvalidIDFormat` | 143 |
| `E506` | `EmptyParticipantsHeader` | None |
| `E507` | `EmptyLanguagesHeader` | 69 |
| `E508` | `EmptyDateHeader` | None |
| `E509` | `EmptyMediaHeader` | None |
| `E510` | `EmptyIDLanguage` | None |
| `E511` | `EmptyIDSpeaker` | None |
| `E512` | `EmptyParticipantCode` | None |
| `E513` | `EmptyParticipantRole` | None |
| `E514` | `MissingLanguageCode` | None |
| `E515` | `EmptyIDRole` | None |
| `E516` | `EmptyDate` | None |
| `E517` | `InvalidAgeFormat` | 153 |
| `E518` | `InvalidDateFormat` | None |
| `E519` | `InvalidLanguageCode` | 121, 122 |
| `E522` | `SpeakerNotDefined` | 9, 12, 13, 16, 18, 60, 68 |
| `E523` | `OrphanIDHeader` | 68 |
| `E524` | `BirthUnknownParticipant` | 68 |
| `E525` | `UnknownHeader` | None |
| `E526` | `UnmatchedBeginGem` | None |
| `E527` | `UnmatchedEndGem` | None |
| `E528` | `GemLabelMismatch` | None |
| `E529` | `NestedBeginGem` | None |
| `E530` | `LazyGemInsideScope` | None |
| `E531` | `MediaFilenameMismatch` | None |
| `E532` | `InvalidParticipantRole` | 9, 12, 13, 16, 142 |
| `E533` | `EmptyOptionsHeader` | None |
| `E600` | `TierValidationError` | None |
| `E601` | `InvalidDependentTier` | None |
| `E602` | `MalformedTierHeader` | None |
| `E604` | `GraWithoutMor` | None |
| `E700` | `UnexpectedTierNode` | 85 |
| `E701` | `TierBeginTimeNotMonotonic` | 83 |
| `E702` | `InvalidMorphologyFormat` | None |
| `E703` | `UnexpectedMorphologyNode` | None |
| `E704` | `SpeakerSelfOverlap` | 84 |
| `E705` | `MorCountMismatchTooFew` | 40, 94, 140 |
| `E706` | `MorCountMismatchTooMany` | 40, 94, 140 |
| `E708` | `MalformedGrammarRelation` | None |
| `E709` | `InvalidGrammarIndex` | None |
| `E710` | `UnexpectedGrammarNode` | None |
| `E712` | `GraInvalidWordIndex` | None |
| `E713` | `GraInvalidHeadIndex` | None |
| `E714` | `PhoCountMismatchTooFew` | 94 |
| `E715` | `PhoCountMismatchTooMany` | 94 |
| `E718` | `SinCountMismatchTooFew` | 94 |
| `E719` | `SinCountMismatchTooMany` | 94 |
| `E720` | `MorGraCountMismatch` | 40, 94, 140 |
| `W001` | `GenericWarning` | None |
| `W108` | `SpeakerNotFoundInParticipants` | None |
| `W210` | `MissingWhitespaceBeforeContent` | 3, 4, 14, 19, 92, 93, 160, 161 |
| `W211` | `MissingWhitespaceAfterOverlap` | 3, 4, 14, 19, 92, 93, 160, 161 |
| `W601` | `EmptyUserDefinedTier` | None |
| `W602` | `UnknownUserDefinedTier` | None |
| `W999` | `LegacyWarning` | None |
| `E999` | `UnknownError` | None |

## Priority Action Plan

### P0

- None

### P1

- CHECK `1` `Expected characters are: @ or % or *.` -> add rule (TalkBank looser; none parity)
- CHECK `2` `Missing ':' character and argument.` -> add rule (TalkBank looser; none parity)
- CHECK `5` `Colon (:) character is illegal.` -> add rule (TalkBank looser; none parity)
- CHECK `8` `Expected characters are: @ % * TAB.` -> add rule (TalkBank looser; none parity)
- CHECK `10` `Tier text is longer than [UTTLINELEN].` -> add rule (TalkBank looser; none parity)
- CHECK `11` `Symbol is not declared in the depfile.` -> add rule (TalkBank looser; none parity)
- CHECK `15` `Illegal role. Please see "depfile.cut" for list of roles.` -> add rule (TalkBank looser; none parity)
- CHECK `17` `Tier is not declared in depfile file.` -> add rule (TalkBank looser; none parity)
- CHECK `20` `Undeclared suffix in depfile.` -> add rule (TalkBank looser; none parity)
- CHECK `30` `Text is illegal.` -> add rule (TalkBank looser; none parity)
- CHECK `32` `Code is not declared in depfile.` -> add rule (TalkBank looser; none parity)
- CHECK `43` `The file must start with "@Begin" tier.` -> add rule (TalkBank looser; none parity)
- CHECK `44` `The file must end with "@End" tier.` -> add rule (TalkBank looser; none parity)
- CHECK `48` `Illegal character(s) [X] found.` -> add rule (TalkBank looser; none parity)
- CHECK `58` `Tier name is longer than 8 characters.` -> add rule (TalkBank looser; none parity)
- CHECK `102` `Italic markers are no longer legal in CHAT.` -> add rule (TalkBank looser; none parity)
- CHECK `107` `Only single commas are allowed in tier.` -> add rule (TalkBank looser; none parity)

### P2

- None

### P3

- None

## Notes and Caveats

- This mapping is comprehensive but heuristic for rules with broad/generic wording.
- CHECK rule anomalies from the reference doc are explicitly modeled as intentional behavioral divergences when TalkBank enforces stricter semantics.
- Remaining `None` mappings should be triaged manually for true coverage gaps vs non-equivalent CHECK legacy behavior.
