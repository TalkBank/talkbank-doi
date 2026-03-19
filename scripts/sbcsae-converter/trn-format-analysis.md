# SBCSAE TRN Format: Empirical Analysis

**Status:** Current
**Last updated:** 2026-03-19

This document records every observed convention and violation in the 60 SBCSAE `.trn`
transcript files downloaded from UCSB (https://linguistics.ucsb.edu/research/santa-barbara-corpus-spoken-american-english). The source files are archived at `~/sbcsae-trn/`.

The purpose is to guide the design of a new TRN parser that:
1. Faithfully parses the overlap correspondence encoded in numbered brackets
2. Tolerates and reports (rather than crashes on) the many data quality issues
3. Produces output that can be merged back into the existing hand-edited CHAT files

The existing hand-edited CHAT files in `ca-data/SBCSAE/` are the gold standard for
**content** (years of manual corrections). The TRN files are the gold standard for
**overlap correspondence** (numbered bracket indices lost in the original conversion).
The goal is to merge the two — not to replace the CHAT.

---

## 1. Line Format

There are **three distinct formats**, reflecting different transcription eras:

### Format A — Early files (SBC001–SBC013)

```
0.00 9.21⟨TAB⟩LENORE: ⟨TAB⟩... So you don't need to go...
9.21 9.52⟨TAB⟩        ⟨TAB⟩to --
```

- **Timestamps:** 2 decimal places, space-separated
- **Field separator:** TAB between timestamps and speaker, TAB between speaker and content
- **Speaker field:** Name + colon + trailing spaces, padded to 8 characters total.
  Continuation lines: 8 spaces (matching the padding width)
- **Line endings:** LF (Unix)

### Format B — Late files (SBC015–SBC060)

```
2.660⟨TAB⟩2.805⟨TAB⟩JOANNE:⟨TAB⟩But,
2.805⟨TAB⟩4.685⟨TAB⟩⟨TAB⟩so these slides...
```

- **Timestamps:** 3 decimal places, tab-separated from each other
- **Field separator:** TABs throughout — 4 fields: start, end, speaker, content
- **Speaker field:** No padding. Continuation lines: empty field (just `⟨TAB⟩⟨TAB⟩`)
- **Line endings:** LF for SBC015–SBC030; CRLF for SBC031–SBC060

### Format C — SBC014 only (transitional)

```
0.00 2.53 FRED:   ⟨TAB⟩... Okay.
2.53 4.73         ⟨TAB⟩One= large loan (Hx),
```

- **Timestamps:** 2 decimal places, space-separated
- **Speaker field:** Space-separated from timestamps (not tab-separated), but padded
  to 8 characters. TAB only before content.
- **Line endings:** LF

### Parser implication

The parser must detect which format variant a file uses (ideally from the first few
lines) and adapt. A unified strategy: split on TABs, then determine whether the first
tab-field is "start end" (Format A/C) or just "start" (Format B).

---

## 2. Character Encoding

The format is nominally ISO-8859-1 (confirmed by the Java `DT.flex` lexer, line 103:
`ISO88591IO.createReader`).

| Encoding | Files | Notes |
|----------|-------|-------|
| Pure ASCII | 53 files | No characters above 0x7F |
| ISO-8859-1 | SBC037 | Spanish accented characters in `<L2...L2>` sections |
| Windows-1252 | SBC060 | 0x92 (right single quote) used as apostrophe, 147 occurrences |
| NUL bytes (0x00) | SBC015, SBC016, SBC018, SBC020, SBC028 | Corrupt bytes replacing 'c'/'C' |

### NUL byte corruption

Five files have NUL bytes (0x00) that replace the letter 'c' or 'C':

| File | Context | Should be |
|------|---------|-----------|
| SBC015 | `\x00hurch` | `church` |
| SBC016 | `\x00ouple` | `Couple` |
| SBC018 | `\x00oming` | `Coming` |
| SBC020 | `\x00ontrols`, `\x00annot` | `Controls`, `cannot` |
| SBC028 | `\x00orrect` | `correct` |

### Parser implication

Read as ISO-8859-1. Replace NUL bytes with 'c' (warn). Map 0x92→`'` for SBC060.

---

## 3. Speaker Conventions

### Standard format

`UPPERCASE_NAME:` — all-caps ASCII letters, optional underscore + digits or letters,
followed by colon. Examples: `JAMIE:`, `TOM_1:`, `JILL_S:`, `AUD1:` (no underscore
variant also exists).

### Special speakers

| Pattern | Meaning | Examples |
|---------|---------|---------|
| `>NAME:` | Non-human / environment | `>ENV:`, `>DOG:`, `>CAT:`, `>BABY:`, `>MAC:`, `>HORSE:`, `>RADIO:` |
| `X:` | Unidentified speaker | 23 files |
| `X_2:`, `X_3:` | Multiple unknowns | SBC053 |
| `ALL:` | Everyone speaking | SBC006, SBC008 |
| `MANY:` | Multiple unidentified | 12 files |
| `BOTH:` | Both participants | SBC028 |
| `CONGR:` | Congregation | SBC021 |
| `KEN/KEV:` | Ambiguous identity | SBC013 |
| `#NAME:` | Anonymized | `#FOSTER` (SBC025), `#READ` (SBC021) |
| `AUD1:`–`AUD8:` | Numbered audience | SBC021 |

### Files with most speakers

SBC023 (19), SBC012 (12), SBC049 (12), SBC053 (11), SBC004 (10).

---

## 4. Overlap Bracket System

This is the most important section for the converter. The TRN format encodes overlapping
speech using bracket pairs with optional numeric indices.

### Basic mechanism

The **first** speaker in an overlap group opens with `[` and closes with `]`. Other
speakers who overlap during that same stretch use matching indices. Unnumbered `[...]`
is index 1 (the first overlap layer). Then `[2...2]`, `[3...3]`, etc. for additional
simultaneous overlap layers.

Example (SBC002, lines 58–61):
```
61.36 63.58  HAROLD:  I guess that means his broken leg is [3@doing @okay3].
62.97 63.86  PETE:                                         [3I was wonder3]ing about that,
63.86 65.25           I was imagining [4he had broke an arm4] or something.
64.24 64.99  JAMIE:                   [4<HI Oh yeah= HI>4].
```

`[3...3]` pairs HAROLD (top) with PETE (bottom). `[4...4]` pairs PETE (top) with JAMIE (bottom).

### Index distribution

| Index | Approx. occurrences | Notes |
|-------|-------------------|-------|
| Unnumbered `[...]` | ~12,300 | Most common; index 1 |
| `[2...2]` | ~4,600 | |
| `[3...3]` | ~2,300 | |
| `[4...4]` | ~1,300 | |
| `[5...5]` | ~800 | |
| `[6...6]` | ~480 | |
| `[7...7]` | ~320 | |
| `[8...8]` | ~230 | |
| `[9...9]` | ~160 | Maximum observed |

Index 9 is the highest. The Java code wraps at 9 (`MAX_OVERLAPS = 9`) — after `[9...9]`,
the next overlap run starts fresh with unnumbered `[...]`.

### Overlap runs

Indices reset between non-overlapping turns. A group of overlapping utterances forms a
**run** starting with unnumbered brackets and incrementing as needed. A new non-overlapping
stretch resets to unnumbered.

Within a run, the Java code (`OverlapRun.java`) handles wrapping: index 0 after 9
becomes real index 10, and so on. In practice, wrapping beyond 9 is rare.

### 1:N pairing (one top, multiple bottoms)

`OverlapSet.java` explicitly supports multiple bottoms for one top. Example (SBC002,
lines 68–71):
```
68.96 73.50  HAROLD:  ... [3He healed very quickly3].
72.10 73.20  JAMIE:       [3<X Guess X> kids' bo=nes,
73.20 75.07              just like3] .. [4grow4] [5back5] really fast (Hx).
73.72 74.37  PETE:                      [4M4][5hm=5].
73.81 74.22  HAROLD:                             [5Yeah5].
```

Index 5 has JAMIE as top and both PETE and HAROLD as bottoms.

### Top/bottom inference

In the TRN format, `[` and `]` don't inherently mark "top" vs "bottom" — that's
inferred. The Java `OverlapInfo` class uses this algorithm:

1. **First `[` of a given index:** this is the **top** (start of overlap)
2. **Subsequent `[` of the same index:** these are **bottoms** (respondents)
3. **`]` of a given index:** closes the bracket for whoever owns it (matched by speaker)

The converter must replicate this: first occurrence of each index = top (`⌈`/`⌉`),
subsequent = bottom (`⌊`/`⌋`).

---

## 5. Self-Violations and Data Quality Issues

### 5.1 Missing colon after speaker name

- **SBC012 line 34:** `MONTOYA ` (tab follows, no colon)
  ```
  49.24 50.27⟨TAB⟩MONTOYA ⟨TAB⟩... Okay,
  ```

### 5.2 Case-inconsistent speaker names

| File | Line | Seen | Should be |
|------|------|------|-----------|
| SBC001 | ~1234 | `Doris:` | `DORIS:` |
| SBC007 | ~555 | `Mary:` | `MARY:` |
| SBC058 | ~463 | `Steven:` | `STEVEN:` |

### 5.3 Uncertain and typographic speaker names

| File | Line | Speaker | Issue |
|------|------|---------|-------|
| SBC023 | ~275 | `JANICD:` | Likely typo for `JANICE` |
| SBC023 | ~624 | `SUE?:` | `?` indicates transcriber uncertainty |
| SBC023 | ~720 | `NORA?:` | `?` indicates transcriber uncertainty |
| SBC023 | ~227 | `*X:` | Starred unknown, unique occurrence |

### 5.4 Bracket mismatches (opening index ≠ closing index)

These are genuine data errors where a bracket opens with one index and closes with
another, typically because the bracketed text spans two overlapping regions:

| File | Line | Text | Issue |
|------|------|------|-------|
| SBC004 | ~199 | `[3they2]` | Index 3 open, index 2 close |
| SBC013 | ~1098 | `[8@9][9=9]` | Adjacent overlaps sharing boundary |
| SBC016 | ~837 | `[3mhm4]` | Mismatch |
| SBC033 | ~9 | `[3She4]` | Crosses overlap boundary |
| SBC034 | ~402 | `[3n= yea=h2]` | Word spans two overlap regions |

### 5.5 Index gaps (skipped numbers)

- **SBC051 lines ~1071–1072:** Jump from `[6...6]` to `[8...8]`, skipping `[7]`
- **SBC015 line ~346:** `[3]` appears without preceding `[2]` in local context

### 5.6 Unmatched brackets

Approximately 75 total index imbalances across the corpus (more opening than closing
brackets, or vice versa). Most are off by 1–3, suggesting minor transcription errors.
Worst offenders: SBC013 (7 imbalances), SBC023 (4), SBC015 (4), SBC019 (3).

### 5.7 Timestamp anomalies

| File | Line | Issue |
|------|------|-------|
| SBC034 | 679 | `13548.02` — extra digit, should be ~1354.802 |
| SBC027 | 65, 127, 387 | Trailing ` :` on end timestamp (e.g., `77.540 :`) |
| SBC002, SBC015, SBC019, SBC023 | various | Start time < previous end (minor, fractions of a second) |

### 5.8 Zero-timestamp blocks

Five files have lines with `000000000 000000000` timestamps:

| File | Count | Content |
|------|-------|---------|
| SBC003 | 6 | Annotator comments: `$ COMMA OR PERIOD?`, etc. |
| SBC005 | 2 | Annotator comments: `$ COMMA OR PEROID?` [sic] |
| SBC007 | 1 | Corrupted inline content |
| SBC011 | 40 | `$ TEXT ENDS` marker + 39 untimed dialogue lines |
| SBC014 | 1 | Corrupted inline content |

### 5.9 Empty content lines

58 lines across the corpus have empty or whitespace-only content. Most common in
SBC017 (5), SBC019 (4), SBC042 (4), SBC033 (2).

### 5.10 Broken tab formatting

SBC013 lines ~490–552 (~22 lines) have malformed field separation — timestamps and
speaker are space-separated inline without proper tab structure:
```
385.62 386.44        simple pleasures.
```

### 5.11 Missing `>` prefix on environment speaker

SBC042 line ~394: `ENV:` without the required `>` prefix (should be `>ENV:`).

---

## 6. Content Conventions

### 6.1 Pauses

| Notation | Meaning | Occurrences |
|----------|---------|-------------|
| `..` | Short pause | ~14,000 |
| `...` | Medium/long pause | ~16,300 |
| `...(N.N)` | Timed pause (seconds) | Not observed in SBCSAE |

### 6.2 Truncation and false starts

| Notation | Meaning | Occurrences |
|----------|---------|-------------|
| `--` | Utterance truncation | ~4,750 |
| `-` (word-final) | Word truncation | ~4,700 (e.g., `y-`, `n-`, `s-`) |
| `%` | Glottal stop / false start | ~1,700 |

### 6.3 Laughter

| Notation | Meaning | Occurrences |
|----------|---------|-------------|
| `@` | Single laugh syllable | ~11,200 total |
| `@@@` | Multiple laughs | (counted in above) |
| `<@...@>` | Laughing quality on speech | ~420 |

### 6.4 Vocalisms and body sounds

| Notation | Meaning | Occurrences |
|----------|---------|-------------|
| `(H)` | Inhalation | ~10,000 |
| `(Hx)` | Exhalation | ~1,100 |
| `(TSK)` | Click/tongue | ~930 |
| `(SNIFF)` | Sniff | ~350 |
| `(THROAT)` | Throat clear | ~270 |
| `(COUGH)` | Cough | ~250 |
| `(SWALLOW)` | Swallow | ~60 |
| Others | `(GASP)`, `(SIGH)`, `(YAWN)`, `(DRINK)`, `(KISS)`, `(GRUNT)`, `(WHISTLE)`, `(SNORT)`, `(HUMMING)`, `(MURMUR)`, `(BLOW)` | <50 each |

### 6.5 Long features (paired delimiters `<X...X>`)

Most common:

| Tag | Meaning | Count |
|-----|---------|-------|
| `<X...X>` | Uncertain hearing | ~1,270 |
| `<VOX...VOX>` | Special voice quality | ~580 |
| `<@...@>` | Laughing quality | ~420 |
| `<P...P>` | Piano (soft) | ~230 |
| `<Q...Q>` | Quotation voice | ~150 |
| `<HI...HI>` | High pitch | ~145 |
| `<WH...WH>` | Whisper | ~140 |
| `<L2...L2>` | Second language | ~120 |
| `<MRC...MRC>` | Marcato (emphatic) | ~100 |
| `<F...F>` | Forte (loud) | ~90 |

Rare: `<FOOD>`, `<SING>`, `<SM>` (smile), `<READ>`, `<PAR>` (parenthetical), `<YWN>`
(yawn), `<BR>`, `<YELL>`, `<SHOUT>`, `<SOB>`, `<CRY>`, `<ACCENT>`, `<FF>` (fortissimo),
`<PP>` (pianissimo), and others.

### 6.6 Nonvocal/environmental sounds

| Notation | Meaning | Examples |
|----------|---------|---------|
| `<<NAME...NAME>>` | Environmental sound span | `<<CLAP...CLAP>>`, `<<THUMP>>`, `<<POUND>>` |
| `+` (within `<<...>>`) | Beat/impact marker | Used for clapping beats |
| `((NAME))` | Environmental comment | `((TELEVISION))`, `((APPLAUSE))`, `((DOOR_CLOSING))` |

### 6.7 Referential markers

| Prefix | Meaning | Count | Examples |
|--------|---------|-------|---------|
| `~` | Known person | ~1,150 | `~Mae`, `~Gerald` |
| `!` | Proper noun | ~380 | `!Kevin`, `!Mary` |
| `#` | Anonymized name | ~160 | `#Paul`, `#Greg` |

### 6.8 Lengthening

`=` after a letter indicates vowel or consonant lengthening (~7,200 occurrences):
`s=o`, `ye=ah`, `n=o`, `I=`, `ri=ght`.

### 6.9 Unintelligible speech

`XX` (single word), `XXX` (multiple words), `XXXX` (longer stretches). Also within
`<X...X>` for uncertain hearing: `<X I think X>`.

### 6.10 Ampersand linker `&`

~126 occurrences. Marks continuation across line breaks within one intonation unit:
```
656.95 657.60  SPEAKER:  if [one] [2out of2] &
658.14 659.08            & [2d=ifferent2],
```

`&` at end of line = "continued on next line". `&` at start of next continuation line =
"continuing from previous".

---

## 7. Spatial (Column) Alignment

On continuation lines (those without a speaker label), content is sometimes indented
with leading spaces so that overlap brackets visually align with the corresponding
bracket in the preceding speaker's line:

```
61.36 63.58  HAROLD:  I guess that means his broken leg is [3@doing @okay3].
62.97 63.86  PETE:                                         [3I was wonder3]ing about that,
```

The spaces before `[3` on PETE's line align it with the `[3` on HAROLD's line. This
is a **visual aid** — the numbered indices already encode the correspondence fully.

### Reliability

Spatial alignment is **decorative, not authoritative**:
- It's present on ~151 continuation lines across the corpus
- Most are accurately aligned (within 1–2 columns)
- A few are visibly wrong (off by 5+ characters)
- Many overlap bottoms have no spatial alignment at all

**The parser should ignore spatial alignment entirely and rely on bracket indices.**

The Java `OverlapInfo` class comment confirms this (line 15):
> "Do not use column checking heuristic because it is inaccurate."

---

## 8. Heuristic Directives

The Java lexer recognizes three inline directives: `{--}` (OVERLAP_KEEP),
`{+}` (OVERLAP_CONTINUE), `{-}` (OVERLAP_DISCONTINUE). These were manually inserted
into TRN files to override the overlap inference heuristic.

**None of these appear in the 60 SBCSAE .trn source files.** They were apparently
used for other corpora or were added during conversion. The parser should recognize
them (for completeness) but they will not appear in SBCSAE.

---

## 9. Timing Characteristics

- **70,083** total timed lines across 60 files
- **20.6%** of consecutive line pairs have temporally overlapping timestamps (expected —
  this is overlapping speech)
- **9.1%** have gaps (silence between turns)
- **~70%** are contiguous (end of one ≈ start of next)
- **Zero-duration lines** occur for `>ENV` events (e.g., `786.42 786.42`)

### Monotonicity violations (within same speaker's continuation)

7 cases where a continuation line's start time is earlier than the previous line's
start time. All are minor (fractions of a second) except the SBC034 typo.

---

## 10. Cross-File Consistency

### Era differences

| Property | SBC001–013 | SBC014 | SBC015–060 |
|----------|-----------|--------|------------|
| Timestamps | 2 decimal | 2 decimal | 3 decimal |
| Timestamp separator | space | space | tab |
| Speaker separator | tab | space | tab |
| Speaker padding | 8 chars | 8 chars | none |
| Continuation marker | 8 spaces | 8 spaces | empty field |
| Line endings | LF | LF | LF (015–030), CRLF (031–060) |

### Content conventions

The actual transcription conventions (overlap brackets, pauses, long features, etc.)
are **consistent across all 60 files.** The differences are purely structural/formatting.

### Notably unusual files

| File | Why |
|------|-----|
| SBC013 | Longest (2,259 lines), 9 speakers, highest overlap complexity, 22 lines with broken tab formatting |
| SBC014 | Unique format variant (Format C) |
| SBC021 | Most unusual speaker inventory (AUD1–8, CONGR, #READ, MANY, WALT) |
| SBC023 | Most speakers (19), uncertain speakers (`SUE?`, `NORA?`), typo (`JANICD`) |
| SBC037 | Only file with ISO-8859-1 accented characters (Spanish) |
| SBC060 | Only file with Windows-1252 smart quotes |
| SBC011 | 40 untimed lines after `$ TEXT ENDS` marker |

---

## 11. Remaining Ambiguity

Even with numbered indices, some ambiguity persists:

### 11.1 Unnumbered brackets when there's only one overlap layer

When there's a simple two-party overlap with no additional layers, only unnumbered
`[...]` is used. This is unambiguous — there's only one possible pairing. But the
converter must still correctly identify top vs. bottom.

### 11.2 Brackets that span overlap boundaries

When a single bracketed span crosses from one overlap region into another (§5.4), the
opening and closing indices disagree. The converter must decide which index "wins" or
whether to split the span.

### 11.3 The `[]` heuristic problem

When a new unnumbered `[` appears after an existing overlap run, is it:
- A new overlap group (top of a new run)?
- The bottom of the existing unnumbered overlap?

The Java code uses the `TryNewException` / `continueFirstOverlap` / `lastSawOverlap`
heuristic (see `OverlapRun.addBegin`, lines 216–278). This heuristic is the source of
most conversion errors and was the reason manual `{+}` / `{-}` directives existed.

For SBCSAE specifically, this is less of a problem because multi-party overlaps almost
always use numbered indices. But the converter should still implement the heuristic for
the simple two-party cases.

### 11.3a Turn structure as a disambiguation signal

Beyond the bracket-level heuristic, the **sequence of speakers across turns** provides
a strong signal for overlap group boundaries that the original Java code only partially
exploited (via the single-turn-lookback `lastSawOverlap` flag).

**Speaker cycling implies new group.** In conversation, speakers take turns. When
the pattern is A→B→C→A, A's return after intervening non-overlapping turns from B
and C is a strong signal that any new `[` from A starts a fresh overlap group:

```
JAMIE:   ... [overlap] ...              ← group 1
HAROLD:      [overlap] ...              ← bottom of group 1
PETE:    ... some unrelated turn ...    ← no overlaps
JAMIE:   ... [new bracket] ...          ← almost certainly a new group 2
MILES:       [new bracket] ...          ← bottom of group 2
```

**Temporal gaps reinforce boundaries.** If the current turn's timestamps are well
past the end of the previous overlap group, the overlap has ended regardless of
speaker identity.

**Adjacency implies same group.** When two speakers' turns are temporally adjacent
and both contain brackets, they are likely in the same overlap group — this is the
normal case of overlapping speech.

These observations inform the enhanced heuristic in the converter design
(`design.md` §4, "Turn structure as a disambiguation signal").

### 11.4 Unmatched brackets

~75 index imbalances mean some brackets simply have no partner. The converter must
emit a diagnostic and produce the best possible output anyway.

---

## 12. Overlap Group Sequencing — Higher-Level Semantics

The overlap indices are not just disambiguation tags — they encode a **sequential
counter of concurrent overlap layers** that carries structural information about
how overlap groups nest and chain in time.

### The model

- Unnumbered `[...]` (index 1): the first overlap group
- `[2...2]`: a second overlap group that opens while the first is still active, or
  immediately adjacent to it
- `[3...3]`: a third group, and so on up to `[9...9]`
- Indices reset when the conversation returns to non-overlapping speech

This means the indices encode **temporal nesting and chaining**. Example from
SBC002, lines 68–71:
```
JAMIE:   just like3] .. [4grow4] [5back5] really fast (Hx).
PETE:                   [4M4][5hm=5].
HAROLD:                          [5Yeah5].
```
Index 3 is closing while index 4 opens, which chains into index 5. This is a
**sequence of overlap groups**, not independent events.

### Nesting (overlap groups within overlap groups)

The samtale-data corpus (Danish CA) demonstrates deep nesting. From
`samtale-data/Sam4/moedregruppen1.cha`, lines 1152–1155:
```
*DO:   mh::⌈2hhhh  ⌈3 huhh            ⌉3 hhuhhhh  ⌉2
*MIA:      ⌊2°huhh ⌊3 h hu°           ⌋3
*SUS:              ⌊3 jom det bare sån⌋3 et stykke⌋2 ⌈4 jenka⌉↘
*DO:                                                ⌊4 ∙hshh⌋hh
```
Index 3 is **nested inside** index 2. Index 4 starts as index 2 closes. The
indices form a tree-like structure, not a flat list.

### Why this matters

Without indices, we cannot determine:
1. Which overlap groups are concurrent vs. sequential
2. Whether a bottom marker belongs to the current group or an enclosing one
3. The temporal layering of multi-party overlaps

This is a **higher level of overlap structure** than simple pairwise matching.
The indices enable analysis of overlap group boundaries, nesting depth, and
chaining patterns — all of which are lost in the current unindexed SBCSAE files.

### Existing indexed CHAT data in TalkBank

A survey of all `data/*-data/` repos (2026-03-19) found only **13 files** with
indexed overlap markers outside SBCSAE:

**ca-data/CLAPI/** (French CA, 4 files, light usage — indices 2–3):
- `biscuit.cha` (2 indexed markers)
- `tuerie.cha` (2)
- `chambre.cha` (3)
- `logeurs.cha` (14, including index 3)

**samtale-data/** (Danish CA, 9 files, heavy usage — indices up to 5):
- `Sam4/moedregruppen1.cha` (100 indexed markers, indices 2–5, deep nesting)
- `Sam3/gamledage.cha` (44, indices 2–3)
- `Sam4/studiegruppe.cha` (26, indices 2–3)
- `Sam3/omfodbold.cha` (29, indices 2–3)
- `Sam4/moedregruppen0.cha` (6, index 2)
- `Sam3/225_deller.cha` (3, index 2)
- `Sam3/kartofler_og_broccoli.cha` (2, index 2)
- `Adgangskode/Steensig/Telefon/undskyld.cha` (4, indices 2–3)
- `Adgangskode/Radio/alkohol.cha` (2, index 2)

**All other data repos:** zero indexed markers.

SBCSAE (60 files, ~22,000 overlap brackets in the TRN source) should be the
**largest corpus with indexed overlaps** once the indices are restored. The
samtale data proves the CHAT format and our tooling support them end-to-end.

### Implications for the converter

The converter should not just assign arbitrary disambiguation indices — it must
preserve the **sequential group semantics**: indices incrementing as new overlap
groups open, resetting between non-overlapping stretches, and correctly
representing nesting when groups are contained within other groups.

---

## 13. Relationship to the Java `DT` Parser

The legacy Java parser in `java-chatter-stable/src/main/java/org/talkbank/dt/` is the
only existing reference implementation. Key architectural insights:

| Java class | Role | Key insight |
|------------|------|-------------|
| `DT.flex` | JFlex lexer | Tokenizes `[`, `]`, `[2`, `2]`, etc. Reads ISO-8859-1. |
| `DTParser.g` | ANTLR 3 parser | Builds CHAT AST. `overlap` rule delegates to `OverlapInfo` for top/bottom inference. |
| `OverlapInfo` | Overlap state machine | Manages current and previous overlap runs. Handles run transitions. |
| `OverlapRun` | Single run of overlaps | Tracks `OverlapSet` array (index 0–8, wrapping at 9). |
| `OverlapSet` | One overlap index | One `OverlapPair` top + `ArrayList<OverlapPair>` bottoms (1:N). |
| `OverlapPair` | Begin/end/who | Tracks a single participant's bracket pair within an overlap. |
| `Speakers` | Speaker registry | Auto-assigns 3-letter CHAT IDs from TRN speaker names. |

The critical insight: the Java parser **correctly parsed** the numbered indices **and
did emit them** as indexed CHAT markers (`⌈2`, `⌊3`, etc.). The full chain is:
DT.flex tokenizes `[2` → `BEGIN_OVERLAP` with text `"2"` → DTParser.g rewrites to
`OVERLAP_START_TOP`/`OVERLAP_START_BOTTOM` preserving the text → ChatWalker.g passes
`index={text}` to the template → Chat.stg emits `$overlapPointMap.(all)$$index$`.

The indices were **stripped at some unknown point between ~2011 and 2022**, before
the ca-data git repo was initialized. The 2022 "Init for GitLab" commit already
contains only unindexed markers. A new converter can regenerate them.

---

## 14. Design Considerations for the New Converter

### Goal

Produce the most faithful possible representation of the original TRN files'
overlap structure, so that the sequential group indices can be merged back into the
existing hand-edited CHAT files. The hand-edited CHAT is gold for content; the TRN
is gold for overlap correspondence. We are not replacing the CHAT.

### Output format (decision pending)

Three options are under consideration — see `docs/sbcsae-overlap-investigation.md`
for the full tradeoff analysis:

- **Option A (CHAT text):** Full TRN→CHAT conversion. Diffable but substantial work
  to reimplement all content transformations, and the diff will mix content
  divergences with overlap information.
- **Option B (structured overlap data):** JSON/similar per-file overlap correspondence
  table — just the indices, speakers, timing, bracketed text. Focused on exactly
  what we need. Requires a separate alignment step for merging.
- **Option C (CHAT AST):** Parse into talkbank-model Rust types, then serialize.
  Leverages existing infrastructure but couples to model internals and requires
  translating TRN content conventions.

If the merge strategy only needs to patch indices onto existing markers, Option B
is simplest. If we also want a reference "faithful original conversion" CHAT,
Option A or C is needed.

### Error tolerance

The parser must **never crash** on bad input. Every violation in §5 should produce a
diagnostic and a best-effort parse. The Java parser's 14 exception types suggest this
is non-trivial.

### Overlap group semantics

The parser must preserve sequential group semantics (§12), not just pairwise
disambiguation:
- Track which indices belong to the same overlap run
- Preserve nesting relationships (inner groups within outer groups)
- Report violations: non-sequential indices, gaps, mismatched open/close

### Format detection

Auto-detect Format A/B/C from the first non-empty line. This avoids hardcoding file
number ranges.

### Testing strategy

1. Parse all 60 files successfully (no crashes)
2. Compare extracted overlap pairs against the Java parser's output (if we can run it)
3. Spot-check overlap correspondence against manual reading of specific examples
4. Verify all known data quality issues from §5 are reported as diagnostics
5. Validate against samtale-data files that already have indexed CHAT markers — the
   converter's overlap semantics should be consistent with how those files work
