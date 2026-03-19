# TRN → CHAT Translation Guide

**Status:** Current
**Last updated:** 2026-03-19

This document describes how the SBCSAE converter translates DuBois TRN
transcription format into CHAT format. It is intended for human reviewers
who need to understand and verify the converter's output.

For TRN format details, see `trn-format-analysis.md`.
For our extensions to the TRN format, see `trn/SYNTAX-EXTENSIONS.md`.
For data corrections, see `trn/FIXES.md`.

---

## Pipeline Overview

```
TRN file
  ↓  read + decode (ISO-8859-1, NUL/DEL repair)
  ↓  format detection (A/B/C)
  ↓  line parsing (timestamps, speakers, content)
  ↓  bracket tokenization
  ↓  content parsing (winnow)
  ↓  speaker ID mapping
  ↓  utterance grouping + terminator resolution
TrnDocument (intermediate, serializable to JSON)
  ↓  global constraint-based inference (petgraph union-find)
OverlapAssignment (bracket ID → top/bottom role)
  ↓  CHAT emission
.cha file
```

Each stage is independent and serializable. The intermediate `TrnDocument`
contains all parsed content with brackets as symmetric Open/Close markers
(no top/bottom role). The `OverlapAssignment` is where top/bottom decisions
live — it can be produced by inference, by diffing against hand-edited CHAT,
or by manual annotation.

---

## Content Transformations

### Pauses

| TRN | CHAT | Notes |
|-----|------|-------|
| `..` | `(..)` | Short pause |
| `...` | `(...)` | Medium pause |
| `...(1.2)` | `(1.2)` | Timed pause — not observed in SBCSAE |

**Edge case:** `...(H)` is medium pause + inhalation, NOT a timed pause
attempt. The parser distinguishes these by checking if the character after
`...(` is a digit (timed pause) or a letter (medium pause + vocalism).

### Lengthening

| TRN | CHAT | Notes |
|-----|------|-------|
| `ta=p` | `ta:p` | `=` after a letter → `:` |
| `ye=ah` | `ye:ah` | Applied inside words during post-processing |

### Glottal Stop

| TRN | CHAT | Notes |
|-----|------|-------|
| `%` (standalone) | `ʔuh` | Between words or at utterance boundaries |
| `a%b` (in-word) | `aʔb` | `%` inside a word → `ʔ` |
| `(%Hx)` | `ʔuh &=ex` | `%` prefix on vocalism → glottal + vocalism |

**Caution:** `%` also appears in long feature labels (`<%...%>`). The parser
handles this by checking context — `%` before `>` is a label character, not a
glottal stop.

### Vocalisms and Body Sounds

| TRN | CHAT | Notes |
|-----|------|-------|
| `(H)` | `&=in` | Inhalation |
| `(H)=` | `&=in &=lengthened` | Lengthened inhalation |
| `(Hx)` | `&=ex` | Exhalation |
| `(TSK)` | `&=tsk` | Click / tongue |
| `(SNIFF)` | `&=sniff` | Lowercased |
| `(THROAT)` | `&=throat` | Lowercased |
| `(COUGH)` | `&=cough` | Lowercased |
| `(NAME)` | `&=name` | General pattern: lowercase |

### Environmental Comments

| TRN | CHAT | Notes |
|-----|------|-------|
| `((DOOR_SLAM))` | `&=DOOR_SLAM` | Preserved in uppercase |
| `((J,_M,_P_LAUGHING_7.8_SEC))` | `&=J_AND_M_AND_P_LAUGHING_7_POINT_8_SEC` | Commas → `_AND`, periods → `_POINT_` |

### Laughter

| TRN | CHAT | Notes |
|-----|------|-------|
| `@` | `&=laugh` | Single laugh syllable |
| `@@@` | `&=laugh &=laugh &=laugh` | Multiple laughs — one `&=laugh` per `@` |
| `@Word` | `Word [% laugh]` | Laughing word: prefix stripped, `[% laugh]` added |

### Long Features

| TRN | CHAT | Notes |
|-----|------|-------|
| `<X word X>` | `&{l=X word &}l=X` | Uncertain hearing |
| `<VOX word VOX>` | `&{l=VOX word &}l=VOX` | Special voice quality |
| `<@ word @>` | `&{l=@ word &}l=@` | Laughing quality |
| `<P word P>` | `&{l=P word &}l=P` | Piano (soft) |
| `<F word F>` | `&{l=F word &}l=F` | Forte (loud) |
| `<WH word WH>` | `&{l=WH word &}l=WH` | Whisper |
| `<L2 word L2>` | `&{l=L2 word &}l=L2` | Second language |
| `<% word %>` | `&{l=% word &}l=%` | Creaky voice |

**Nested features:** `<F<VOX word VOX>F>` → `&{l=F &{l=VOX word &}l=VOX &}l=F`.

**Intra-word close:** `wordX>` (no space before label) is split into
`word` + `&}l=X` using a known-label table.

### Nonvocal Sounds

| TRN | CHAT | Notes |
|-----|------|-------|
| `<<THUMP>>` | `&={n=THUMP}` | Simple nonvocal |
| `<<CLAP +++ CLAP>>` | `&{n=CLAP &=nonvocal &=nonvocal &=nonvocal &}n=CLAP` | Beats within nonvocal span |
| `+` (in `<<...>>`) | `&=nonvocal` | Beat/impact |

### Referential Markers (Pseudographs)

| TRN | CHAT | Notes |
|-----|------|-------|
| `~Mae` | `Mae` | Known person — prefix stripped |
| `!Kevin` | `Kevin` | Proper noun — prefix stripped |
| `#Paul` | `Paul` | Anonymized — prefix stripped |

### Truncation and Terminators

| TRN | CHAT | Rule |
|-----|------|------|
| `.` | `.` | Period — sentence-final |
| `?` | `?` | Question |
| `--` | `+/.` | Truncation — always emitted as interruption |
| `,` (at turn end) | `.` | Comma at end of final continuation → period |
| `&` (at turn end) | `+,` | Continuation linker → self-completion |
| (no terminator) | `.` | Implicit period inserted |

**Automated `--` resolution:** The converter uses temporal analysis:
- If the next speaker's start time is **before** this speaker's `--` timestamp
  (negative gap), the speaker was interrupted → `+/.`
- If the next speaker's start time is **at or after** the `--` timestamp
  (zero or positive gap), the speaker trailed off → `+...`

This gives ~28% `+/.` and ~72% `+...` on SBC002. Brian's hand-edited CHAT has
~52% `+/.` and ~49% `+...` — his decisions were based on listening to audio,
not purely on timing. The temporal approximation is principled but imperfect:
many TRN timestamps are exactly aligned (gap=0), which all become `+...` under
our rule even though some were perceptually interruptions.

### Phonological Fragments

| TRN | CHAT | Notes |
|-----|------|-------|
| `/word/` | `/word/` | Preserved as-is |

---

## Overlap Markers

### Bracket → Marker Translation

| TRN | CHAT | Condition |
|-----|------|-----------|
| `[` | `⌈` | First speaker in an overlap group (top begin) |
| `]` | `⌉` | First speaker's close (top end) |
| `[` | `⌊` | Responding speaker (bottom begin) |
| `]` | `⌋` | Responding speaker's close (bottom end) |
| `[2` | `⌈2` or `⌊2` | Numbered: second concurrent overlap group |
| `[3` | `⌈3` or `⌊3` | Third concurrent group, etc. |

### Top vs Bottom Assignment

The TRN format uses symmetric brackets — `[` and `]` for both speakers. The
converter must infer who is "top" (first speaker) and who is "bottom" (respondent).

**Rule:** In each overlap set, the first speaker in document order is the top.
All other speakers are bottoms.

### Overlap Grouping (Inference)

Bracket pairs are grouped into overlap sets using a constraint graph with
petgraph union-find:

1. **Spatial alignment edges** (strongest signal): The TRN transcribers used
   column indentation to visually align corresponding brackets. If speaker B's
   `[` is indented to the same column as speaker A's `[`, they're in the same
   overlap set. 5,269 such edges found across the corpus.

2. **Temporal overlap**: If two bracket pairs from different speakers have
   overlapping time ranges (within 200ms tolerance) and the same lexical index
   (unnumbered with unnumbered, `[2` with `[2`), they're grouped together.

3. **Line adjacency**: Brackets from different speakers within 2 lines of each
   other, with the same index, are grouped.

4. **Transitive closure**: Union-find gives this for free. If A groups with B
   and B groups with C, all three are in the same set.

5. **Same-speaker split**: After grouping, any set with multiple pairs from the
   same speaker is split into sub-sets (one pair per speaker per set).

### Index Assignment

| TRN Index | CHAT Index | Notes |
|-----------|-----------|-------|
| Unnumbered `[...]` | No index (or `⌈1` if explicit) | First overlap group |
| `[2...2]` | `⌈2`/`⌊2` | Second concurrent group |
| `[3...3]` | `⌈3`/`⌊3` | Third, etc. |
| `[9...9]` | `⌈9`/`⌊9` | Maximum |

Indices encode **sequential overlap group layering** — they increment as new
overlap groups open while previous ones are still active.

---

## Utterance Grouping

A TRN **turn** (speaker line + continuation lines) maps to one or more CHAT
utterances:

1. Each new speaker label starts a new CHAT utterance.
2. Within a same-speaker turn, terminators (`.`, `?`, `--`) split the turn
   into multiple utterances.
3. Commas at the end of the final continuation line become periods.
4. Utterances are sorted by start time for monotonic timestamps.

### Timing Bullets

TRN timestamps (floating-point seconds) → CHAT bullets (integer milliseconds):

```
TRN:  0.00 6.52
CHAT: \x150_6520\x15
```

The `\x15` (NAK) character delimits the bullet. Start and end times are
separated by `_`.

---

## Speaker ID Mapping

TRN uses full names (`JAMIE`, `HAROLD`). CHAT uses truncated IDs (`JAMI`, `HARO`).

**Rule:** Truncate to 4 characters (`MAX_WHO = 4`).

**Special cases** (from the original Java `Speakers.java`):
- `SHANE` → `SHAN`, `SHARON` → `SHA` (conflict avoidance)
- `KEN` → `KEN`, `KENDRA` → `KEND`
- `TOM_1` → `TOM1`, `TOM_2` → `TOM2`, `TOM_3` → `TOM3`
- `AUD_1` → `AUD1`, `AUD_2` → `AUD2`, `AUD_3` → `AUD3`

**Prefixes stripped:** `>` (environment), `#` (anonymized), `*` (uncertain).

**Conflicts:** If two names truncate to the same ID, the second gets
`JUNK0`, `JUNK1`, etc.

---

## CHAT File Structure

```
@UTF8
@Begin
@Languages:	eng
@Participants:	JAMI Speaker, HARO Speaker, ...
@Options:	CA
@ID:	eng|SBCSAE|JAMI|||||Speaker|||
@ID:	eng|SBCSAE|HARO|||||Speaker|||
@Media:	02, audio
*JAMI:	How ⌈ can you teach a three-year-old to ⌉ ta:p ⌈2 dance ⌉2 . \x150_6520\x15
*HARO:	⌊ I can't imagine teaching a ⌋ +/.
...
@End
```

---

## CHAT Sanitization Rules

Some TRN content is valid in TRN but illegal in CHAT. The converter
sanitizes these during emission:

| TRN Content | CHAT Problem | Sanitization |
|-------------|-------------|--------------|
| `((J,_M,_P_LAUGHING_7.8_SEC))` | Commas and periods illegal in happening names | `,` → `_AND`, `.` → `_POINT_` |
| `((OVERHEAD_LIGHT_GOES_ON_BY_ITSELF))` | Very long happening names | Preserved (CHAT allows long names) |
| Vocalism names with digits: `(YAWN0)` | Digits in vocalism names | Fixed in TRN source (typo) |
| `<LABEL` crossing `((COMMENT))` | Brackets inside comment names | Fixed in TRN source (bracket repositioned) |
| `SM@>` (compound label close) | Ambiguous: is `@` part of label or content? | Fixed in TRN: `SM> @>` (spaces added) |
| `<@Word` (word jammed into label) | Parser can't separate label from content | Fixed in TRN: `<@ Word` (space added) |

The sanitization is applied in `emit_chat.rs` during the CHAT emission stage.
The TRN source fixes are tracked in `trn/FIXES.md`.

## Heuristics and Approximations

This section documents every decision in the converter that involves
heuristic reasoning rather than deterministic translation.

### H1: Overlap grouping (which brackets belong together)

**Algorithm:** petgraph union-find over a constraint graph.
**Edges:** Alignment edges (indentation), temporal overlap (200ms tolerance),
line adjacency (≤2 lines).
**Accuracy:** ~99% of brackets grouped correctly (2,140 E348 at parity with
hand-edited baseline of 2,152). 225 E347 cross-utterance mismatches remain.
**Limitation:** Purely temporal — does not consider content similarity,
intonation, or discourse structure.

### H2: Top vs bottom assignment (who is "first")

**Rule:** First speaker in document order within each overlap set is the top.
**Accuracy:** Correct by definition for the TRN format (the speaker whose
bracket appears first in the file was speaking first).
**Limitation:** Document order ≠ temporal order in rare cases where TRN lines
are not sorted by start time.

### H3: Truncation terminator (`--` → `+/.` vs `+...`)

**Algorithm:** Temporal gap analysis.
- gap < 0 (next speaker started before `--`): `+/.` (interruption)
- gap ≥ 0 (this speaker stopped before next speaker): `+...` (trail off)

**Accuracy:** Principled but imperfect. Produces ~28% `+/.` and ~72% `+...`
on SBC002, vs Brian's ~52%/~49%. The discrepancy is because Brian's decisions
incorporated audio perception, not just timing.
**Manual review needed:** Cases where gap ≈ 0 are genuinely ambiguous.

### H4: Alignment edge computation (indentation correspondence)

**Algorithm:** For each Open bracket with `char_offset > 0` (indented), search
backward up to 5 lines for a different-speaker Open bracket within 2 columns.
**Coverage:** 5,269 edges found across 24,514 open brackets (21.5%).
**Limitation:** Only detects indentation-based alignment. Many legitimate
overlap correspondences have no indentation cue.

### H5: Same-speaker set splitting

**Algorithm:** After union-find grouping, any connected component with multiple
pairs from the same speaker is split greedily by document order.
**Accuracy:** Prevents self-overlap (E704) but may create artificial set
boundaries within genuine multi-utterance overlaps.

### H6: Utterance boundary placement

**Rule:** Split at TRN terminators (`.`, `?`, `--`) and at speaker changes.
**Note:** Bracket pairs that span terminators have their open and close on
different CHAT utterances. This is valid — the validator's E348 within-utterance
check is suppressed for cross-utterance spans, and E347 handles cross-utterance
pairing correctly.

### H7: Compatible index matching

**Rule:** Unnumbered brackets only group with unnumbered; `[2` only with `[2`.
**Exception:** Alignment edges override this — if indentation shows an
unnumbered bracket aligns with a numbered one, they're grouped.
**Limitation:** May under-group in cases where a top is numbered but the
bottom is unnumbered (or vice versa) without an alignment edge.

## Known Limitations

1. **`--` resolved by temporal gap**: See H3 above. Not 100% accurate.

2. **Cross-utterance bracket spans are valid**: Overlap markers that span
   across utterance boundaries (open on one `*SPK:` line, close on a later
   one from the same speaker) are legitimate. The validator's within-utterance
   check (E348) is suppressed for these; the cross-utterance check (E347)
   handles pairing correctly. This was validated against the hand-edited SBCSAE
   (2,152 E348 false positives eliminated) and confirmed across all TalkBank
   data repos (~49,400 total eliminated).

4. **Multi-line long features**: Some long features (`<LABEL ... LABEL>`)
   span multiple TRN lines. If the label is split across lines, the parser
   may not detect the close. These produce E370 errors (~23 cases).

5. **Encoding artifacts**: SBC037 (Spanish) and SBC060 (Windows-1252) have
   non-ASCII characters that may not round-trip perfectly through ISO-8859-1
   decoding.
