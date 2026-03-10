# UI Wish List: Structural Elements and Editor Features

**Status:** Running wish list (low priority, long-term)
**Date:** 2026-02-18
**Updated:** 2026-02-18

This document tracks model structures that could benefit from UI features in the LSP/VS Code extension. Part I covers cross-element relationships (pairing, spanning, balancing). Part II covers within-element structures that warrant enhanced visual treatment. These are secondary to core structural features like tier alignment but represent the richest opportunities for transcription-aware editor support.

## Table of Contents

### Part I: Cross-Element Relationships

1. [Overlap Bracket Pairing & Visualization](#1-overlap-bracket-pairing--visualization)
2. [Quotations (Two Systems)](#2-quotations-two-systems)
3. [Completion Pairing (Self & Other)](#3-completion-pairing-self--other)
4. [Long Feature & Nonvocal Spans](#4-long-feature--nonvocal-spans)
5. [TCU Continuation Chains (CA Mode)](#5-tcu-continuation-chains-ca-mode)
6. [Temporal Overlap Visualization](#6-temporal-overlap-visualization)
7. [Other Spoken Events](#7-other-spoken-events)
8. [Replacement Word Display](#8-replacement-word-display)

### Part II: Within-Element Visual Enhancements

9. [CA Delimiter Pairing](#9-ca-delimiter-pairing)
10. [Retrace & Repair Visualization](#10-retrace--repair-visualization)
11. [Pause Visualization](#11-pause-visualization)
12. [Tier Alignment & Dependency Visualization](#12-tier-alignment--dependency-visualization)
13. [Language Switching](#13-language-switching)
14. [Intonation Contour Display](#14-intonation-contour-display)
15. [Zero/Omission Words](#15-zeroomission-words)
16. [Header Table Views](#16-header-table-views)
17. [Postcode Filtering & Highlighting](#17-postcode-filtering--highlighting)

### Summary

18. [Priority Matrix](#18-priority-matrix)

---

## 1. Overlap Bracket Pairing & Visualization

### The Structure

CHAT has two overlap systems:

**CA Overlap Points** -- Unicode bracket pairs marking exact overlap scope:
```
*MOT:  I think \u2308 we should go \u2309 home .
*CHI:          \u230A can I come   \u230B too ?
```

- Top brackets `\u2308\u2309` = first speaker's overlapped region
- Bottom brackets `\u230A\u230B` = second speaker's overlapping region
- Optional indices (2-9) for multiple simultaneous overlaps: `\u23082...\u23092` / `\u230A2...\u230B2`
- Can appear standalone between words OR embedded within words: `ye\u2309s`

**Scoped Overlap Markers** -- Annotation-style: `[<]` / `[>]` with optional indices `[<1]` / `[>1]`

### Current State

- **Model:** `OverlapPoint` with `OverlapPointKind` (4 variants) and optional `OverlapIndex`. No cross-reference or pairing structure.
- **Parsing:** Solid in both parsers. Overlap points survive roundtrip.
- **Validation:** Only E704 (same-speaker self-overlap) and E373 (index range check). E347 (UnbalancedOverlap) and E348 (MissingOverlapEnd) are defined but `not_implemented`.
- **LSP:** No overlap-specific features at all.
- **VS Code:** `\u2308\u2309\u230A\u230B` are NOT registered as bracket pairs in `language-configuration.json`.

### Proposed UI Features

#### A. Bracket Registration (Quick Win)

Register overlap brackets as bracket pairs in `language-configuration.json`:
```json
{
  "brackets": [
    ["\u2308", "\u2309"],
    ["\u230A", "\u230B"]
  ]
}
```

This gives users VS Code's built-in bracket matching (highlight matching bracket on cursor, rainbow brackets if enabled) with zero LSP work. Limitation: only works within a single line/utterance -- does not cross utterances.

#### B. Within-Utterance Balance Highlighting

When cursor is on any overlap bracket, highlight the matching bracket on the same utterance in the same color. Uses LSP `textDocument/documentHighlight`.

```
*MOT:  I think [\u2308] we should go [\u2309] home .
                ^^^               ^^^
              highlighted pair (same color)
```

For indexed overlaps, highlight only index-matching pairs:
```
*MOT:  I \u23082 think \u23083 we \u23093 should \u23092 go .
           ^^                       ^^  <- cursor on \u23082, highlight \u23082 and \u23092
                   ^^    ^^             <- cursor on \u23083, highlight \u23083 and \u23093
```

#### C. Cross-Speaker Overlap Pairing

The most valuable feature: when cursor is on a top bracket `\u2308`, find the corresponding bottom bracket `\u230A` on an adjacent utterance by a different speaker (matching index if present).

```
*MOT:  I think [\u2308] we should go [\u2309] home .     <- cursor here
*CHI:          [\u230A] can I come   [\u230B] too ?       <- highlight these
```

Implementation approach:
1. Find all overlap points in the current utterance
2. Look at adjacent utterances (within a window of ~5 lines) by different speakers
3. Match top/bottom pairs by index
4. Highlight all four brackets in two color groups

#### D. Overlap Region Background Highlighting

Beyond just the brackets, highlight the *overlapping text span* with a subtle background color:

```
*MOT:  I think \u2308[we should go]\u2309 home .     <- blue background on "we should go"
*CHI:          \u230A[can I come  ]\u230B too ?       <- orange background on "can I come"
```

This makes the overlap visually obvious even in dense transcripts.

#### E. Go-to-Matching-Overlap

`textDocument/definition` from any overlap bracket jumps to the matching bracket on the other speaker's line. This makes it trivial to navigate between overlapping turns.

#### F. Gutter Decoration

A vertical line or bracket in the editor gutter showing the overlap span across lines:

```
  +-- *MOT:  I think \u2308 we should go \u2309 home .
  +-- *CHI:          \u230A can I come   \u230B too ?
```

Or for three-way overlaps:
```
  +-- *MOT:  I think \u2308 we should \u2309 go .
  +-- *CHI:          \u230A can I     \u230B come ?
  +-- *FAT:          \u230A wait      \u230B .
```

---

## 2. Quotations (Two Systems)

CHAT has two distinct quotation systems that serve different purposes.

### 2A. Within-Utterance Quotations (Unicode Marks)

#### The Structure

Quoted speech within a single utterance uses Unicode smart quotation marks:

```
*CHI:  she said \u201Cplease come here\u201D and left .
*MOT:  \u201Chello there\u201D is what I said .
```

- `\u201C` (LEFT DOUBLE QUOTATION MARK) opens the quotation
- `\u201D` (RIGHT DOUBLE QUOTATION MARK) closes the quotation
- Content inside can include words, events, pauses -- the full main tier vocabulary
- Self-contained within one utterance, no cross-utterance pairing needed

#### Current State

- **Model:** `Quotation { content: BracketedContent, span: Span }` in `UtteranceContent` enum. Well-modeled.
- **Grammar:** `quotation` rule wraps `contents` between `left_double_quote` / `right_double_quote`.
- **Parsing:** Solid in tree-sitter parser. Roundtrip works.
- **Validation:** Built into parser (balanced quotes). No semantic validation.
- **Highlighting:** `(quotation) @string.delimiter` in highlights.scm. Foldable in folds.scm.
- **LSP/VS Code:** No quotation-specific features beyond syntax highlighting.

#### Proposed UI Features

##### A. Quoted Content Background

Subtle background color on the quoted span, making the quotation boundaries obvious:

```
*CHI:  she said [\u201Cplease come here\u201D] and left .
                 ^^^^^^^^^^^^^^^^^^^^^^^
               subtle background tint on quoted content
```

##### B. Quoted Speech Speaker Attribution

When a quotation is preceded by a speech verb pattern (e.g., `said`, `told`, `asked`), show an inline hint indicating it's reported speech. This helps distinguish direct quotes from other uses of quotation marks.

##### C. Bracket Registration

Register `\u201C\u201D` as bracket pairs in `language-configuration.json` for built-in bracket matching:
```json
{
  "brackets": [
    ["\u201C", "\u201D"]
  ]
}
```

### 2B. Cross-Utterance Quotation Sequences (Linker System)

#### The Structure

Quoted speech extending across multiple utterances uses the linker/terminator system:

**Pattern A -- "Quotation Follows" (attribution first):**
```
*MOT:  and then she said +"/.
*MOT:  +" please come here .
*MOT:  +" I need your help .
```
The `+"/.` terminator signals "quotation follows". Subsequent same-speaker utterances with `+"` linker are the quoted speech.

**Pattern B -- "Quotation Precedes" (attribution last):**
```
*MOT:  +" please come here .
*MOT:  +" I need your help +".
```
The `+".` terminator closes the quotation sequence. Preceding same-speaker `+"` utterances are the quoted speech.

#### Current State

- **Model:** `Linker::QuotationFollows` (`+"`), `Terminator::QuotedNewLine` (`+"/.`) and `Terminator::QuotedPeriodSimple` (`+".`) are all modeled.
- **Validation:** E341, E344, E346 are defined but **disabled** (gated behind `context.enable_quotation_validation = false`). No validation runs by default.
- **LSP/VS Code:** No quotation sequence features.

#### Proposed UI Features

##### A. Quotation Chain Highlighting

When cursor is on any `+"` linker or quotation terminator, highlight the entire quotation chain:

```
*MOT:  and then she said [+"/.]          <- cursor on terminator
*MOT:  [+"] please come here .           <- highlighted as part of chain
*MOT:  [+"] I need your help .           <- highlighted as part of chain
```

Use a distinctive gutter decoration (e.g., a vertical quote bar `|`) and/or a subtle background color for the quotation lines.

##### B. Quotation Scope Bracket

In the gutter or as an inline decoration, show the quotation scope:

```
  +-- *MOT:  and then she said +"/.
  |   *MOT:  +" please come here .
  +-- *MOT:  +" I need your help .
```

##### C. Diagnostics (When Enabled)

When quotation validation is enabled:
- E341: `+"/.` without following `+"` from same speaker
- E344: `+".` without preceding `+"` from same speaker
- E346: `+"` with no upstream `+"/.` or downstream `+".`
- Show inline diagnostics with quick-fix suggestions

---

## 3. Completion Pairing (Self & Other)

### The Structure

**Self-completion** (`+,` linker pairs with `+/.` terminator):
```
*MOT:  I was going to +/.               <- interrupted
*CHI:  can I have cookie ?              <- interrupter
*MOT:  +, say something nice .          <- self-completion, resuming
```

**Other-completion** (`++` linker pairs with `+...` terminator):
```
*MOT:  I was going to the +...          <- trailing off
*CHI:  ++ store ?                       <- different speaker completes the thought
```

### Current State

- **Model:** All linkers and terminators modeled.
- **Validation:** E349/E350 (other-completion) and E352 (self-completion) are defined but **disabled**.
- **LSP/VS Code:** No completion pairing features.

### Proposed UI Features

#### A. Completion Link Visualization

When cursor is on `+,`, highlight the matching `+/.` utterance by the same speaker:

```
*MOT:  I was going to [+/.]              <- highlighted as source
*CHI:  can I have cookie ?
*MOT:  [+,] say something nice .         <- cursor here
```

Draw a subtle connecting line or gutter bracket between the two related utterances.

#### B. Other-Completion Link

When cursor is on `++`, highlight the `+...` utterance from the other speaker:

```
*MOT:  I was going to the [+...]         <- highlighted
*CHI:  [++] store ?                      <- cursor here
```

---

## 4. Long Feature & Nonvocal Spans

### The Structure

Cross-utterance annotation spans:

```
*CHI:  &{l=singing happy birthday to you .
*CHI:  happy birthday dear mommy &}l=singing .

*CHI:  &{n=crying I want mommy .
*CHI:  please &}n=crying .
```

### Current State

- **Model:** `LongFeatureBegin`/`LongFeatureEnd`, `NonvocalBegin`/`NonvocalEnd` with label fields.
- **Validation:** **Enabled** -- E358/E359/E366 (long features) and E367/E368/E369 (nonvocal) check balance and label matching.
- **LSP/VS Code:** No span visualization features.

### Proposed UI Features

#### A. Span Background Highlighting

When cursor is on any `&{l=` or `&}l=` marker, highlight all utterances within the span:

```
  | *CHI:  [&{l=singing] happy birthday to you .    <- subtle background
  | *CHI:  happy birthday dear mommy [&}l=singing] . <- subtle background
```

Different colors for different labels (singing = blue, crying = red, etc.).

#### B. Gutter Span Indicators

Vertical bar in gutter showing span extent:

```
  | *CHI:  &{l=singing happy birthday to you .
  | *CHI:  happy birthday dear mommy &}l=singing .
```

#### C. Go-to-Matching-Marker

From `&{l=singing`, jump to `&}l=singing` and vice versa. Standard bracket-match behavior.

---

## 5. TCU Continuation Chains (CA Mode)

### The Structure

In Conversation Analysis, Turn Constructional Units (TCUs) chain through terminator/linker pairs:

```
*CHI:  I was just \u220B                     <- technical break terminator
*CHI:  +\u220B thinking about it .           <- TCU continuation linker
```

Similarly with `\u2248` (no-break):
```
*CHI:  and then \u2248                       <- no break terminator
*CHI:  +\u2248 she left .                    <- no break continuation
```

### Current State

- **Model:** `CaTechnicalBreak`/`CaTechnicalBreakLinker` and `CaNoBreak`/`CaNoBreakLinker` all modeled.
- **Validation:** No pairing validation exists.
- **LSP/VS Code:** No TCU chain features.

### Proposed UI Features

#### A. TCU Chain Highlighting

When cursor is on a CA terminator or continuation linker, highlight the entire TCU chain with a gutter bracket and subtle background.

---

## 6. Temporal Overlap Visualization

### The Structure

Media bullet timestamps on utterances define temporal positions:

```
*MOT:  hello . 0_1500
*CHI:  hi .    800_2000
               ^^^ overlaps with MOT's utterance (800-1500ms)
```

### Current State

- **Validation:** E701 (timeline monotonicity) and E704 (self-overlap) are enabled.
- **VS Code:** Media player exists, bullet decorations exist.
- **No temporal overlap visualization.**

### Proposed UI Features

#### A. Timeline Minimap

A narrow panel showing a vertical timeline with colored bars for each speaker's utterance timing. Overlapping bars are visually obvious.

#### B. Overlap Indicator Decorations

When two speakers' bullets overlap temporally, add a decoration (icon or color) to both lines indicating they overlap. Hovering shows the overlap duration.

---

## 7. Other Spoken Events

### The Structure

Background speech by another speaker embedded inline:

```
*MOT:  and then &*CHI:mommy I said .
```

The `&*CHI:mommy` means CHI said "mommy" in the background during MOT's utterance.

### Current State

- **Model:** `OtherSpokenEvent { speaker: SpeakerCode, word: Word }` modeled.
- **Validation:** No check that the referenced speaker is a valid participant.
- **LSP/VS Code:** No special features.

### Proposed UI Features

#### A. Speaker Link

Make the speaker code in `&*CHI:word` a go-to-definition target (jump to `@Participants`), same as main tier speaker codes.

#### B. Inline Speaker Color

Color the `&*CHI` portion with CHI's speaker color (from the speaker color theme), making it visually obvious which speaker is speaking in the background.

---

## 8. Replacement Word Display

### The Structure

Replacement annotations mark what was actually spoken vs what should have been said:

```
*CHI:  goed [: went] to the park .
*CHI:  I wanna [: want to] go .
*CHI:  dat [: that] one .
```

The `[: replacement]` annotation says the child said "goed" but the target/intended form is "went". The replacement can be one or more words.

This creates a fundamental **dual-view** problem: depending on the analysis task, a researcher may want to see:
- **What was spoken** (the actual production): `goed to the park` -- for phonological analysis, error analysis, developmental studies
- **What was intended** (the target form): `went to the park` -- for syntactic analysis, MLU calculation, lexical frequency counts

### Current State

- **Model:** `ReplacedWord { word: Word, replacement: Replacement, scoped_annotations: ReplacedWordAnnotations }`. The `Replacement` contains a `ReplacementWords` (`Vec<Word>`). Well-modeled with robust validation.
- **Validation:** **Enabled** -- 9+ error codes (E208, E209, E210, E211, E387-E391) covering empty replacements, invalid sources (fragments, nonwords, fillers), and invalid targets (omissions, untranscribed).
- **Highlighting:** `(replacement) @string.special` in highlights.scm.
- **LSP/VS Code:** No replacement-specific features beyond syntax highlighting and validation diagnostics.

### Proposed UI Features

#### A. Toggle View: Spoken vs Intended

A VS Code command or status bar toggle that switches the display between three modes:

1. **Full mode** (default): Show both -- `goed [: went]`
2. **Spoken mode**: Show only what was said -- `goed` (with subtle indicator that a replacement exists)
3. **Intended mode**: Show only the target form -- `went` (with subtle indicator it's a replacement)

Implementation: Use LSP inlay hints or VS Code decorations to dim/hide one side and optionally inline the other.

```
Full mode:       *CHI:  goed [: went] to the park .
Spoken mode:     *CHI:  goed~ to the park .              (~ = has replacement)
Intended mode:   *CHI:  went* to the park .              (* = is replacement)
```

#### B. Hover: Show Both Forms

On hover over a replaced word or its replacement, show a tooltip with both forms and the relationship:

```
Spoken:   "goed"
Intended: "went"
Type:     morphological error (irregular past tense)
```

The "type" line could be inferred from error annotations (`[*]`, `[* m:+ed]`) when present.

#### C. Replacement Highlighting

Distinct visual treatment for the original word vs the replacement:
- Original word: subtle underline or different text color (indicating an error/deviation)
- Replacement: displayed in a muted/secondary color (indicating the correction)
- Consistent across the file -- all replaced words get the same visual treatment

#### D. Replacement Statistics

A panel or status bar item showing replacement statistics for the current file:
- Total replaced words
- Breakdown by error type (when `[*]` annotations are present)
- Per-speaker replacement rates

#### E. %mor Alignment Awareness

When `%mor` tier exists, replacements affect alignment: the morphological annotation typically corresponds to the **replacement** form, not the spoken form. The UI should make this relationship clear:

```
*CHI:  goed [: went] to the park .
%mor:  v|go-PAST     prep|to det|the n|park .
       ^^^^^^^^^ annotates "went", not "goed"
```

A hover or decoration could show which form the %mor line annotates.

---

---

# Part II: Within-Element Visual Enhancements

---

## 9. CA Delimiter Pairing

### The Structure

CA delimiters appear in pairs within words/utterances to mark speech quality spans:

```
*CHI:  and then \u00B0I said quietly\u00B0 to mommy .
*CHI:  she was \u2206talking really fast\u2206 .
*CHI:  \u2207slo::wly\u2207 he walked .
```

15 delimiter types exist, each marking a different speech quality:

| Symbol | Meaning | Unicode |
|--------|---------|---------|
| `\u00B0` | Softer/quieter | U+00B0 |
| `\u2206` | Faster tempo | U+2206 |
| `\u2207` | Slower tempo | U+2207 |
| `\u2581` | Lower pitch register | U+2581 |
| `\u2594` | Higher pitch register | U+2594 |
| `\u263A` | Smile voice | U+263A |
| `\u264B` | Breathy voice | U+264B |
| `\u2047` | Uncertain/guessing | U+2047 |
| `\u222C` | Whispered speech | U+222C |
| `\u03AB` | Yawning | U+03AB |
| `\u222E` | Singing voice | U+222E |
| `\u21AB` | Segment repetition | U+21AB |
| `\u204E` | Creaky voice (vocal fry) | U+204E |
| `\u25C9` | Louder speech | U+25C9 |
| `\u00A7` | Precise articulation | U+00A7 |

### Current State

- **Model:** Individual `ca_delimiter` supertype with 15 leaf node variants. Parsed individually -- no pairing enforced.
- **Validation:** No balance checking exists for CA delimiters.
- **Highlighting:** `(ca_delimiter) @punctuation.special` in highlights.scm.
- **LSP/VS Code:** No delimiter-specific features.

### Proposed UI Features

#### A. Bracket Registration (Quick Win)

Register all 15 CA delimiter pairs in `language-configuration.json`:
```json
{
  "brackets": [
    ["\u00B0", "\u00B0"], ["\u2206", "\u2206"], ["\u2207", "\u2207"],
    ["\u2581", "\u2581"], ["\u2594", "\u2594"], ["\u263A", "\u263A"],
    ...
  ]
}
```

Limitation: These are self-paired (same character opens and closes), so VS Code's built-in matching may not handle them well. May need LSP assistance.

#### B. Span Highlighting

When cursor is on any CA delimiter, highlight the matching delimiter and the content between them with a subtle background tint:

```
*CHI:  she was [\u2206talking really fast\u2206] .
                ^^^^^^^^^^^^^^^^^^^^^^
              highlighted span with background color
```

Different colors per delimiter type: soft blue for `\u00B0` (soft), warm red for `\u2206` (fast), etc.

#### C. Hover: Delimiter Meaning

On hover over any CA delimiter, show a tooltip explaining the speech quality:
```
\u2206 -- Faster tempo
  This segment was produced at a faster-than-normal rate.
```

#### D. Balance Diagnostics

Report unbalanced delimiters -- an opening `\u00B0` without a closing `\u00B0` in the same utterance. This is a new validation rule.

---

## 10. Retrace & Repair Visualization

### The Structure

Retrace markers indicate self-repair in speech. A group `<...>` followed by a retrace marker shows what the speaker corrected:

```
*CHI:  <I want> [//] I need a cookie .     <- complete retrace: "I want" -> "I need"
*CHI:  <I want> [/] I want a cookie .      <- partial retrace: "I want" repeated
*CHI:  <I want> [///] I desire a cookie .  <- multiple retrace: third+ attempt
*CHI:  <to the> [/-] at the store .        <- reformulation: restructured phrase
*CHI:  <I want> [/?] I need a cookie .     <- uncertain: unclear if retrace
```

| Marker | Meaning | Typical Research Use |
|--------|---------|---------------------|
| `[/]` | Exact repetition | Fluency analysis, word count exclusion |
| `[//]` | Revision/correction | Error repair analysis |
| `[///]` | Multiple revision | Complex repair sequences |
| `[/-]` | Reformulation | Syntactic restructuring |
| `[/?]` | Uncertain retrace | Transcriber confidence |

### Current State

- **Model:** `RetraceMarker` enum with 5 variants. Grammar has `retrace_marker` supertype.
- **Validation:** Retrace markers are parsed and modeled but no group-scope validation (e.g., "retrace must follow a group").
- **Highlighting:** `(retrace_marker) @keyword.operator` in highlights.scm.
- **LSP/VS Code:** No retrace-specific features.

### Proposed UI Features

#### A. Retrace Scope Dimming

When a retrace marker is present, dim or strikethrough the retracted group content:

```
*CHI:  <I want> [//] I need a cookie .
        ~~~~~~       <- dimmed/strikethrough (this was corrected)
                     I need a cookie .  <- normal brightness (the correction)
```

This instantly shows the reader which parts of speech were abandoned vs kept.

#### B. Retrace Type Color Coding

Different visual treatment per retrace type:
- `[/]` repetition: subtle gray (just repeated, no error)
- `[//]` revision: amber/orange (corrected)
- `[///]` multiple: red (struggled)
- `[/-]` reformulation: blue (restructured)
- `[/?]` uncertain: dotted underline

#### C. Retrace Statistics

A panel showing retrace counts per speaker per type -- valuable for fluency analysis:
```
CHI: [/] x12, [//] x8, [///] x2, [/-] x3
MOT: [/] x4, [//] x1
```

#### D. Toggle: Include/Exclude Retraced Content

A view toggle that hides retracted content, showing only the "final" version of each utterance. Essential for readability in dense transcripts and for researchers computing measures like MLU (where retraces are excluded).

---

## 11. Pause Visualization

### The Structure

Pauses represent silence in the speech stream:

```
*CHI:  I want (.) a cookie .                 <- short pause (<0.5s)
*CHI:  um (..) I think so .                  <- medium pause (0.5-1.0s)
*CHI:  well (...) okay .                     <- long pause (>1.0s)
*CHI:  and then (3.5) she left .             <- timed pause (3.5 seconds)
*CHI:  I was (1:02.3) thinking .             <- timed pause (1 min, 2.3 sec)
```

### Current State

- **Model:** `PauseToken` enum: `Short`, `Medium`, `Long`, `Timed(duration)`.
- **Grammar:** `pause_token` supertype with 4 variants including `pause_timed` with duration parsing.
- **Highlighting:** `(pause_token) @string.escape` in highlights.scm.
- **LSP/VS Code:** No pause-specific features.

### Proposed UI Features

#### A. Proportional Pause Width

Render pauses with proportional inline width -- longer pauses appear wider in the editor using inlay hints or decorations:

```
*CHI:  I want . a cookie .                   <- short: thin space
*CHI:  um .. I think so .                    <- medium: wider
*CHI:  well ... okay .                       <- long: widest
*CHI:  and then ....(3.5s).... she left .    <- timed: proportional to duration
```

#### B. Pause Duration Tooltip

On hover, show the duration category or exact time:
```
(.)  -> Short pause (< 0.5 seconds)
(..) -> Medium pause (0.5 -- 1.0 seconds)
(3.5) -> 3.5 seconds
```

#### C. Pause Highlighting Mode

A toggle that highlights all pauses in the file with intensity proportional to duration. This creates a visual "rhythm map" of the conversation, showing where speech flows smoothly and where it stalls.

---

## 12. Tier Alignment & Dependency Visualization

### The Structure

Three dependent tiers have strict word-by-word alignment with the main tier:

**%mor (morphology):** Each item corresponds to one main tier word.
```
*CHI:  I      want    a     cookie .
%mor:  pro|I  v|want  det|a n|cookie .
```

**%gra (grammatical relations):** Dependency indices reference %mor positions.
```
%gra:  1|2|SUBJ 2|0|ROOT 3|4|DET 4|2|OBJ
       ^^^^^^^^
       Word 1 ("I") is subject of word 2 ("want")
```

**%pho (phonology):** Each item is the phonological form of one main tier word.
```
*CHI:  I      want    a     cookie .
%pho:  aI     want    @     kUki
```

### Current State

- **Model:** Alignment is validated (E714/E715 for count mismatches, E600 for %gra index errors). Alignment data structures exist.
- **Validation:** Enabled and robust.
- **LSP/VS Code:** No alignment visualization. No dependency arc rendering. Tiers are just displayed as plain text lines.

### Proposed UI Features

#### A. Vertical Alignment Indicators

When cursor is on a main tier word, highlight the corresponding %mor, %gra, and %pho items:

```
*CHI:  I      [want]   a     cookie .         <- cursor on "want"
%mor:  pro|I  [v|want] det|a n|cookie .       <- highlighted
%gra:  1|2|SUBJ [2|0|ROOT] 3|4|DET 4|2|OBJ   <- highlighted
%pho:  aI     [want]   @     kUki             <- highlighted
```

Uses LSP `textDocument/documentHighlight` across lines within the same utterance.

#### B. POS Tag Color Coding

Color %mor items by part of speech: nouns in blue, verbs in green, determiners in gray, etc. This makes the morphological structure immediately scannable.

```
%mor:  pro|I  v|want  det|a  n|cookie .
       ^^^^   ^^^^^^  ^^^^^  ^^^^^^^^
       purple  green   gray    blue
```

#### C. Dependency Arc Rendering

Render %gra relations as arcs above the utterance (a la dependency tree visualizations in NLP):

```
       +--SUBJ--+  +--DET--+
       |   ROOT  |  |  OBJ  |
*CHI:  I  want   a  cookie .
```

This could be a hover panel, a side panel, or inline decorations. Extremely valuable for syntactic research.

#### D. Hover: Full Word Analysis

On hover over any main tier word, show all aligned information in one tooltip:
```
Word:     "want"
%mor:     v|want (verb, stem: want)
%gra:     2|0|ROOT (root of sentence)
%pho:     want
Position: 2 of 4
```

---

## 13. Language Switching

### The Structure

Words marked with `@s:LANG` indicate language switching in bilingual/multilingual transcripts:

```
*CHI:  I want leche@s:spa please .          <- "leche" is Spanish
*CHI:  el perro@s:spa+eng is big .          <- ambiguous Spanish+English
*CHI:  quiero@s:spa milk .                  <- Spanish word in English utterance
```

Language-scope markers `[- lang]` set the default language for an entire utterance:
```
*CHI:  [- spa] quiero leche por_favor .     <- entire utterance in Spanish
```

### Current State

- **Model:** `word_langs` parsed with `@s:LANG` syntax, `langcode` parsed with `[- lang]`. Language codes validated against registry.
- **Validation:** Language code format checked.
- **Highlighting:** `(word_langs) @string.special.symbol` in highlights.scm.
- **LSP/VS Code:** No language-switching-specific features.

### Proposed UI Features

#### A. Language Color Coding

Color-code words by language. Each language gets a distinct, consistent color across the file:

```
*CHI:  I want leche please .
              ^^^^^
          (tinted Spanish orange -- indicates code-switch)
```

The utterance-level `[- spa]` marker sets the "expected" language; only deviations (switches) get colored, keeping the display clean.

#### B. Language Gutter Indicator

A small colored dot or flag in the gutter for utterances that contain code-switches. Makes it trivial to scan a transcript and find mixed-language utterances.

#### C. Language Statistics Panel

Show language distribution per speaker:
```
CHI:  English 78%, Spanish 18%, Mixed 4%
MOT:  English 95%, Spanish 5%
```

Valuable for bilingualism research, code-switching frequency analysis.

---

## 14. Intonation Contour Display

### The Structure

Intonation markers appear as separators between words, indicating pitch contour:

```
*CHI:  I want -> a cookie -> please .
```

| Symbol | Name | Meaning |
|--------|------|---------|
| `\u21D7` | Rising to high | Strong rising pitch |
| `\u2197` | Rising to mid | Moderate rising pitch |
| `\u2192` | Level | Continuing, flat pitch |
| `\u2198` | Falling to mid | Moderate falling pitch |
| `\u21D8` | Falling to low | Strong falling pitch |
| `\u221E` | Unmarked ending | No marked intonation |
| `\u2261` | Uptake | Latching/uptake |

### Current State

- **Model:** Individual separator types modeled as enum variants.
- **Highlighting:** Captured via `(separator)` rules in highlights.scm, but not individually differentiated.
- **LSP/VS Code:** No intonation-specific features.

### Proposed UI Features

#### A. Pitch Direction Color Coding

Color intonation markers by direction:
- Rising (`\u21D7` `\u2197`): warm red/orange (going up)
- Level (`\u2192`): neutral gray
- Falling (`\u2198` `\u21D8`): cool blue (going down)

#### B. Pitch Contour Minimap

A subtle inline decoration above or below the utterance showing the pitch contour as a line:

```
                    /--\
*CHI:  I want \u2197 a cookie \u2198 please .
```

This makes the prosodic shape of the utterance immediately visible.

#### C. Hover: Marker Meaning

On hover, show what each marker means with example audio description:
```
\u2197 -- Rising to mid pitch
  Typical of continuation, non-final position in a list,
  or yes/no questions in English.
```

---

## 15. Zero/Omission Words

### The Structure

Words prefixed with `0` indicate omitted or implied words:

```
*CHI:  0does he 0have a cookie ?             <- "does" and "have" omitted in speech
*CHI:  0it goes there .                      <- "it" omitted (pro-drop)
```

The zero is the omission marker; the following word is what the speaker *should* have said but didn't.

### Current State

- **Model:** `word_prefix: zero` on the Word model. Well-supported.
- **Validation:** Omission words validated (e.g., replacement cannot contain omissions -- E390).
- **Highlighting:** `(zero) @keyword` in highlights.scm.
- **LSP/VS Code:** No omission-specific features.

### Proposed UI Features

#### A. Omission Ghost Text

Display omitted words as ghost/phantom text -- lighter color, italic, or with transparency:

```
*CHI:  does he have a cookie ?               <- "does" and "have" in ghost text
       ^^^^    ^^^^
       lighter/italic (not actually spoken)
```

This distinguishes what was spoken from what was implied, at a glance.

#### B. Toggle: Show/Hide Omissions

A toggle between:
- **Full view**: Shows omissions with ghost text (for analysis)
- **Spoken view**: Hides omissions entirely (shows only what was actually said)

Similar in concept to the replacement toggle (section 8).

---

## 16. Header Table Views

### The Structure

@ID headers are pipe-delimited structured data:

```
@ID:	eng|corpus|CHI|2;05.24|female|TD||Target_Child|||
@ID:	eng|corpus|MOT|35;06.|female|||Mother|||
```

Fields: language | corpus | speaker | age | sex | group | SES | role | education | custom

@Participants headers list the cast:
```
@Participants:	CHI Katie Child, MOT Mother, FAT Father
```

### Current State

- **Grammar:** @ID fields parsed into named nodes (`id_languages`, `id_speaker`, `id_age`, `id_sex`, etc.). @Participants parsed into `participant` entries.
- **Validation:** Required headers validated. Age format validated.
- **LSP/VS Code:** No structured header features beyond syntax highlighting.

### Proposed UI Features

#### A. @ID Hover Table

On hover over an @ID line, show a formatted table:
```
+-------------------------------------+
| Speaker:   CHI (Katie, Target_Child)|
| Age:       2;05.24 (2 years, 5 mo) |
| Sex:       Female                   |
| Language:  English                  |
| Corpus:    corpus                   |
| Group:     TD                       |
+-------------------------------------+
| OK Matching @Participants entry     |
| OK Age format valid                 |
+-------------------------------------+
```

#### B. Participant Peek

On hover over a speaker code (`*CHI:` on any main tier), show a summary card of that participant from @ID + @Participants:
```
CHI -- Katie (Target_Child)
Age: 2;05.24 | Sex: Female | Group: TD
Language: English
```

#### C. Participant Code Completions

When typing a speaker code on a new `*` line, offer completions from the @Participants list. Already partially possible with LSP completion, but could be enhanced with participant details in the completion items.

---

## 17. Postcode Filtering & Highlighting

### The Structure

Postcodes mark utterance-level properties:

```
*CHI:  cookie [+ bch] .                     <- babbling
*CHI:  I want cookie [+ trn] .              <- translation
*MOT:  yes you can have one [+ R] .         <- response
```

Common postcodes include: `bch` (babbling), `trn` (translation), `R` (response), `I` (imitation), `exc` (excluded from analysis), `cit` (citation), and many custom codes.

### Current State

- **Model:** `Postcode { code: String }` in the utterance ending.
- **Validation:** Postcodes parsed and stored. No validation of code values.
- **Highlighting:** `(postcode) @attribute` in highlights.scm.
- **LSP/VS Code:** No postcode-specific features.

### Proposed UI Features

#### A. Postcode Gutter Icons

Small colored icon in the gutter for utterances with specific postcodes:
- `[+ bch]`: orange icon (babbling -- not real speech)
- `[+ trn]`: blue icon (translation -- not original)
- `[+ exc]`: red icon (excluded from analysis)

Makes it easy to scan and skip certain utterance types.

#### B. Postcode Filter Command

A command palette action: "Filter by postcode" -- shows only utterances matching a specific postcode, or hides utterances with certain codes. Essential for analysis workflows.

#### C. Postcode Statistics

Status bar showing postcode distribution:
```
Utterances: 142 total | 8 [+ bch] | 3 [+ trn] | 2 [+ exc]
```

---

## 18. Priority Matrix

### Part I: Cross-Element Relationships

| Feature | Impact | Effort | Prerequisite | Priority |
|---------|--------|--------|-------------|----------|
| Overlap bracket registration (`language-configuration.json`) | Medium | Trivial | None | **P1** |
| Quotation mark bracket registration (`\u201C\u201D`) | Low | Trivial | None | **P1** |
| Within-utterance overlap balance highlighting | Medium | Low | None | **P2** |
| Cross-speaker overlap pair highlighting | High | Medium | Overlap pairing algorithm | **P2** |
| Replacement toggle: spoken vs intended view | High | Medium | None | **P2** |
| Replacement hover (both forms) | Medium | Low | None | **P2** |
| Long feature / nonvocal span highlighting | Medium | Low | Already validated | **P2** |
| Overlap region background highlighting | High | Medium | Cross-speaker pairing | **P3** |
| Quotation chain highlighting (cross-utterance) | Medium | Medium | Quotation validation (re-enable) | **P3** |
| Within-utterance quotation background | Low | Low | None | **P3** |
| Replacement highlighting (distinct colors) | Medium | Low | None | **P3** |
| Replacement %mor alignment indicator | Medium | Medium | Tier alignment | **P3** |
| Go-to-matching overlap bracket | High | Medium | Cross-speaker pairing | **P3** |
| Overlap gutter decoration | High | High | Cross-speaker pairing | **P3** |
| Other spoken event speaker link | Low | Low | None | **P3** |
| Self-completion (`+,` / `+/.`) link | Low | Medium | Completion validation | **P4** |
| Other-completion (`++` / `+...`) link | Low | Medium | Completion validation | **P4** |
| TCU chain highlighting | Low | Medium | TCU validation (new) | **P4** |
| Temporal overlap visualization | Medium | High | Timeline data structure | **P4** |
| Quotation gutter bracket (cross-utterance) | Medium | Medium | Quotation chain detection | **P4** |
| Replacement statistics panel | Low | Medium | None | **P4** |

### Part II: Within-Element Visual Enhancements

| Feature | Impact | Effort | Prerequisite | Priority |
|---------|--------|--------|-------------|----------|
| Tier alignment: vertical word highlighting | **Very High** | Medium | Alignment data | **P1** |
| Tier alignment: hover with full word analysis | **Very High** | Low | Alignment data | **P1** |
| Retrace scope dimming/strikethrough | High | Medium | None | **P2** |
| Retrace type color coding | Medium | Low | None | **P2** |
| POS tag color coding in %mor | High | Low | None | **P2** |
| Pause duration tooltip (hover) | Medium | Low | None | **P2** |
| CA delimiter hover (marker meaning) | Medium | Low | None | **P2** |
| Omission ghost text | Medium | Low | None | **P2** |
| @ID hover table | Medium | Low | None | **P2** |
| Participant peek (hover on speaker code) | Medium | Low | None | **P2** |
| Language switching color coding | High | Medium | Language code registry | **P3** |
| CA delimiter span highlighting | Medium | Medium | CA delimiter balance validation | **P3** |
| Intonation contour color coding | Medium | Low | None | **P3** |
| Retrace toggle: show/hide retracted content | High | Medium | None | **P3** |
| Dependency arc rendering (%gra) | High | High | Alignment + rendering engine | **P3** |
| Proportional pause width | Low | Medium | Inlay hints | **P3** |
| Postcode gutter icons | Medium | Low | None | **P3** |
| Postcode filter command | Medium | Medium | None | **P3** |
| Language statistics panel | Low | Medium | Language code registry | **P4** |
| Intonation contour minimap | Low | High | Custom rendering | **P4** |
| Retrace statistics panel | Low | Medium | None | **P4** |
| Pause rhythm map | Low | High | Custom rendering | **P4** |
| Postcode statistics | Low | Low | None | **P4** |

### Implementation Dependencies

```
=== Quick Wins (pure config, no code) ===

Overlap bracket registration (P1)
Quotation mark bracket registration (P1)

=== Tier Alignment (highest-value cluster) ===

Alignment data structure (prereq -- already exists in model)
  +-- Vertical word highlighting across tiers (P1)
  +-- Hover: full word analysis tooltip (P1)
  +-- POS tag color coding in %mor (P2)
  +-- Dependency arc rendering (P3)
  +-- Replacement %mor alignment indicator (P3)

=== Overlap Cluster ===

Cross-speaker overlap pairing algorithm (prereq)
  +-- Cross-speaker overlap highlighting (P2)
  +-- Overlap region background (P3)
  +-- Go-to-matching overlap (P3)
  +-- Overlap gutter decoration (P3)

=== Replacement Cluster ===

Replacement toggle: spoken vs intended (P2)
  +-- no dependencies (model already has both forms)

Replacement hover (P2)
  +-- no dependencies

=== Retrace/Repair Cluster ===

Retrace scope dimming (P2)
  +-- no dependencies
Retrace toggle: hide retracted content (P3)
  +-- builds on retrace scope detection

=== Quotation Cluster ===

Quotation validation re-enablement (prereq)
  +-- Quotation chain highlighting (P3)
  +-- Quotation gutter bracket (P4)

=== CA/Prosody Cluster ===

CA delimiter hover (P2)
  +-- no dependencies (static lookup table)
CA delimiter balance validation (prereq -- new)
  +-- CA delimiter span highlighting (P3)

=== Language/Metadata Cluster ===

Language code registry (prereq -- already exists)
  +-- Language switching color coding (P3)
  +-- Language statistics panel (P4)

=== Completion/TCU Cluster ===

Completion validation re-enablement (prereq)
  +-- Self-completion link (P4)
  +-- Other-completion link (P4)
```

---

## See Also

- [grammar-redesign.md](grammar-redesign.md) -- grammar simplification plan
- [grammar-stakeholders.md](grammar-stakeholders.md) -- stakeholder analysis for grammar granularity
- `grammar/grammar.js` -- tree-sitter grammar (source of all structural elements)
- `grammar/queries/highlights.scm` -- current syntax highlighting captures
- `rust/crates/talkbank-model/src/model/content/overlap.rs` -- overlap model
- `rust/crates/talkbank-model/src/model/content/linker.rs` -- linker model
- `rust/crates/talkbank-model/src/model/content/terminator.rs` -- terminator model
- `rust/crates/talkbank-model/src/model/content/group.rs` -- `Quotation` struct (within-utterance)
- `rust/crates/talkbank-model/src/model/annotation/replacement.rs` -- replacement model
- `rust/crates/talkbank-model/src/model/annotation/retrace.rs` -- retrace marker model
- `rust/crates/talkbank-model/src/model/content/pause.rs` -- pause model
- `rust/crates/talkbank-model/src/model/alignment/` -- tier alignment data structures
- `rust/crates/talkbank-model/src/validation/cross_utterance/` -- cross-utterance validation
- `spec/errors/E347_auto.md`, `E348_auto.md` -- unimplemented overlap error specs
