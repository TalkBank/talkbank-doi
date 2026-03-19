# SBCSAE TRN Overlap Extractor — Design

**Status:** Draft
**Last updated:** 2026-03-19

Standalone Rust tool that parses 60 SBCSAE `.trn` files and produces:

1. **JSON** — structured overlap correspondence data for programmatic merge
2. **CHAT** — complete `.cha` files, diffable against existing hand-edited CHAT

The CHAT output is a direct string transformation of TRN content — not an AST-based
pipeline. The Java converter's content rules are the reference, but implemented as
simple text rewriting rather than porting the ANTLR/StringTemplate architecture.

See `trn-format-analysis.md` for the complete format reference.
See `docs/sbcsae-overlap-investigation.md` for the project overview.

---

## What we extract

For each TRN file, a JSON structure containing:

1. **Overlap runs** — sequences of temporally connected overlap events, each containing:
   - Sequential indices used (unnumbered=1, then 2, 3, ...)
   - For each index: top speaker + bracket text, bottom speaker(s) + bracket text(s)
   - Timestamp ranges for each bracketed region
   - **Column offsets** of each bracket in the original line (indentation hints for
     alignment with CHAT — decorative but potentially useful for matching)
2. **Diagnostics** — every data quality issue found

We do NOT parse the full TRN content into CHAT structure. We only need to locate
bracket tokens and run the overlap inference state machine.

---

## Directory structure

```
scripts/sbcsae-converter/
├── trn-format-analysis.md      # Format reference (exists)
├── design.md                   # This file
├── Cargo.toml
├── src/
│   ├── main.rs                 # CLI: arg parsing, file iteration, JSON output
│   ├── types.rs                # All data structures
│   ├── encoding.rs             # ISO-8859-1 reading, NUL repair, Windows-1252 mapping
│   ├── format.rs               # Format variant detection (A/B/C) and line parsing
│   ├── bracket.rs              # Bracket tokenizer within content
│   ├── overlap.rs              # Overlap state machine (port of Java OverlapInfo)
│   └── diagnostics.rs          # Diagnostic types and formatting
└── tests/
    ├── known_examples.rs       # Hand-verified examples from SBC002, SBC013, etc.
    └── full_corpus.rs          # Parse all 60 files, no panics, snapshot diagnostics
```

### Dependencies (minimal)

- `serde`, `serde_json` — output serialization
- `encoding_rs` — ISO-8859-1 / Windows-1252 decoding
- `clap` — CLI argument parsing

No parser combinator library. The line format is simple enough for hand-written
parsing; the bracket tokenizer is a single-pass character scan.

---

## Parsing pipeline

### Stage 1: Read and decode (`encoding.rs`)

1. Read file as raw bytes
2. Decode as ISO-8859-1 (via `encoding_rs`)
3. Replace NUL bytes (0x00) with `c`, emit `NulByte` diagnostic for each
4. Map Windows-1252 0x92 to `'`, emit `Windows1252Char` diagnostic
5. Strip `\r` from CRLF files

### Stage 2: Format detection and line parsing (`format.rs`)

Auto-detect variant from the first non-empty line:

| Signal | Variant |
|--------|---------|
| First tab-field contains space between two numbers | A or C |
| — and second tab-field looks like padded speaker | A |
| — and second tab-field is content (no speaker separator) | C |
| First two tab-fields are each a single number | B |

For each line, extract:
- `start_time: f64`, `end_time: f64`
- `speaker: Option<String>` — None for continuation lines
- `raw_content: String` — untouched content field
- `content_column: usize` — absolute column offset where content begins in the
  original line (critical for indentation hints)

Track `current_speaker` across continuation lines.

Special cases (all emit diagnostics):
- `000000000` timestamps → `ZeroTimestamp`
- Trailing ` :` on timestamps (SBC027) → strip, emit `TimestampAnomaly`
- Extra-digit timestamps (SBC034) → `TimestampAnomaly`
- Missing colon (SBC012 `MONTOYA `) → `MissingSpeakerColon`
- Broken tab structure (SBC013 ~22 lines) → `BrokenTabFormatting`, fall back to
  space-splitting

### Stage 3: Bracket tokenization (`bracket.rs`)

Single-pass scan of each line's `raw_content`. For each bracket token, record:
- `char_offset` — position within `raw_content`
- `column` — absolute position in original line (`content_column + char_offset`)
- `kind` — Open / Close / CloseForced
- `lexical_index` — None (unnumbered) or Some(2..=9)

**Skip zones** (not overlap brackets):
- `((` through `))` — environmental comments
- `<<` through `>>` — nonvocal sounds

**Tokenization rules:**
- `[` → Open. Check if next char is digit 2–9; if so, consume as `lexical_index`.
- digit 2–9 immediately followed by `]` → Close with that index.
- digit 2–9 immediately followed by `$]` → CloseForced with that index.
- `$]` → CloseForced, unnumbered.
- bare `]` → Close, unnumbered.

The "immediately adjacent" rule distinguishes overlap digits from content digits
(e.g., "2 dogs" is not an overlap close).

**Brackets inside words** (e.g., `does[n't]`, `[3@doing @okay3]`): handled
naturally — we're scanning for `[` and `]` characters regardless of word context.

**Index mismatches** (e.g., `[3mhm4]`): tokenizer correctly reports Open(3) then
Close(4) as separate tokens. The state machine handles the mismatch.

### Stage 4: Overlap inference state machine (`overlap.rs`)

Direct port of `OverlapInfo` → `OverlapRun` → `OverlapSet` from Java.

#### Core data structures

```rust
struct OverlapPair {
    who: String,
    begin: BracketToken,
    end: Option<BracketToken>,
}

struct OverlapSet {
    real_index: usize,
    top: OverlapPair,
    bottoms: Vec<OverlapPair>,
    frozen_end: Option<BracketToken>,
}

struct OverlapRun {
    sets: Vec<OverlapSet>,
    continue_first_overlap: bool,
    last_saw_overlap: bool,
    saw_overlap: bool,
}

struct OverlapState {
    previous_run: Option<OverlapRun>,
    current_run: Option<OverlapRun>,
    completed_runs: Vec<OverlapRunOutput>,
    run_counter: usize,
    diagnostics: Vec<Diagnostic>,
}
```

#### Index mapping

Lexical index: unnumbered `[]` = 0, `[2]` = 1, ..., `[9]` = 8.

Real index: computed by `actual_index()` which handles wraparound at 9:
```rust
fn actual_index(&self, lexical: u8) -> usize {
    let size = self.sets.len();
    let multiple = size / 9;
    let anchor = multiple * 9;
    let up = anchor + lexical as usize;
    if up <= size + 2 { up }
    else {
        let down = up.saturating_sub(9);
        if down > 0 { down } else { up }
    }
}
```

#### The unnumbered bracket heuristic

When a new unnumbered `[` appears and an unnumbered set already exists in the
current run, is it a new top (new run) or a bottom (same run)?

The Java code's decision tree:
1. If `mayWrap()` is true (sets.len() is a multiple of 9): always try bottom.
2. If `continueFirstOverlap` is true (`{+}` directive): always try bottom.
3. Otherwise, if `!lastSawOverlap` (no overlap in previous turn): start new run.
4. Otherwise, if the next set is already complete: start new run.
5. Otherwise: try bottom.

In Rust, model this as a return enum rather than Java's exception-based control flow:
```rust
enum AddBeginResult {
    Top,
    Bottom,
    TryNewRun,  // Caller should start a new run and retry
}
```

#### Turn structure as a disambiguation signal

Beyond the Java heuristic, the **sequence of speakers across turns** provides a
powerful signal for overlap group boundaries that the original code only partially
exploited.

**The core insight:** In conversation, overlaps happen between speakers who are
talking at roughly the same time. When speakers cycle — A talks, then B, then C,
then A comes back — the return of A after non-overlapping intervening turns is a
strong signal that any new `[` from A starts a **fresh overlap group**, not a
continuation of a group from before the interruption.

**Example — speaker cycling implies new group:**
```
JAMIE:   ... [overlap] ...          ← overlap group 1
HAROLD:      [overlap] ...          ← bottom of group 1
PETE:    ... some unrelated turn ...   ← no overlaps, different speaker
JAMIE:   ... [new bracket] ...      ← new group 2, not a bottom of group 1
MILES:       [new bracket] ...      ← bottom of group 2
```

PETE's intervening non-overlapping turn creates a natural boundary. When JAMIE
returns, any `[` is almost certainly a new overlap run.

**Example — adjacency implies same group:**
```
JAMIE:   ... [overlap text] ...     ← top of group
HAROLD:      [response] ...         ← bottom — same group, immediately adjacent
```

When two speakers' turns are temporally adjacent and both contain brackets, they
are likely in the same overlap group.

**Signals that suggest a new group (beyond the Java heuristic):**

1. **Intervening non-overlapping turns.** If one or more turns from other speakers
   occur between the last overlap bracket and the current `[`, with no brackets
   of their own, this is a strong boundary signal. The conversation moved on and
   came back.

2. **Temporal gap.** If the current turn's timestamp is well past the end of the
   previous overlap group's timestamps, the overlap has ended and this is new.

3. **Speaker returning after absence.** If speaker A had the top of the last
   overlap group, then B and C spoke without overlaps, and now A opens a new `[`,
   this is very likely a new group — A is re-entering the conversation.

4. **Same speaker, new turn, no continuation.** If the same speaker who had the
   top of the previous group starts a completely new turn (not a continuation line)
   with a new `[`, and the previous group was already complete (top and bottom both
   closed), this is a new group.

**Signals that suggest same group (bottom, not new top):**

1. **Different speaker, temporally overlapping timestamps.** The most direct signal
   — this speaker's turn time range overlaps with the top speaker's time range.

2. **Immediately adjacent turns with brackets.** Two speakers' bracketed turns are
   adjacent with no gap.

3. **Same overlap run not yet complete.** The current run has an open top without
   any bottom yet — the next speaker's `[` is probably the bottom.

**Implementation approach:**

Enhance the `OverlapState` to track:
```rust
struct OverlapState {
    // ... existing fields ...
    /// Speakers of the last N turns, most recent first
    recent_speakers: VecDeque<String>,
    /// Whether the last N turns had any overlap brackets
    recent_had_overlaps: VecDeque<bool>,
    /// Timestamp range of the most recent completed overlap run
    last_run_time_range: Option<(f64, f64)>,
}
```

When the unnumbered bracket heuristic reaches its ambiguous case (step 5 in the
Java decision tree), consult these additional signals as tiebreakers before
defaulting to "try bottom."

This is an **enhancement over the Java heuristic**, not a replacement. The Java
code's `lastSawOverlap` flag is a crude version of signal #1 (it only looks one
turn back). The enhanced version looks at the full recent turn history and
timestamps.

**Caveat:** These signals are heuristic. They will not be 100% correct. The
numbered indices (when present) are always authoritative. These signals only
matter for unnumbered brackets, which in SBCSAE are the simple two-party case.

#### Turn boundaries

A **turn** starts with a line that has a speaker label and continues through all
continuation lines. `reset_seen()` is called between turns (shifts `saw_overlap`
to `last_saw_overlap`, clears `saw_overlap`). The enhanced turn tracking also
pushes to `recent_speakers` and `recent_had_overlaps` at each boundary.

#### Error recovery

Every Java exception type maps to a `DiagnosticCode`. Push diagnostic, continue.
Never panic. Map:

| Java exception | Diagnostic | Recovery |
|----------------|------------|----------|
| `InvalidIndexException` | `InvalidIndex` | Skip bracket |
| `SameSpeakerException` | `SameSpeakerOverlap` | Skip bracket |
| `NoTopException` | `UnmatchedBracket` | Try previous run, then skip |
| `NoBottomException` | `NoBottom` | Emit on run close |
| `IncompleteTopException` | `IncompleteTop` | Emit on run close |
| `IncompleteBottomException` | `IncompleteBottom` | Emit on run close |
| `FrozenEndException` | `UnmatchedBracket` | Skip bracket |
| `HighOverlapIndexException` | `HighOverlapIndex` | Skip bracket |
| `MissingBeginException` | `UnmatchedBracket` | Skip bracket |
| `CloseException` | `UnmatchedBracket` | Skip bracket |
| `TryNewException` | (not an error) | Start new run |

### Stage 5: Speaker ID mapping

Generate the TRN-name → CHAT-ID mapping, replicating the Java `Speakers` logic:

1. **Special cases** (hardcoded in the original Java, `Speakers.java:14–25`):
   `TOM_1`→`TOM1`, `TOM_2`→`TOM2`, `TOM_3`→`TOM3`, `AUD_1`→`AUD1`,
   `AUD_2`→`AUD2`, `AUD_3`→`AUD3`, `SHANE`→`SHAN`, `SHARON`→`SHA`,
   `KEN`→`KEN`, `KENDRA`→`KEND`
2. **Default**: truncate to first 4 characters (`MAX_WHO = 4`)
3. **Conflict detection**: if two names truncate to the same ID (e.g., `SHARON`
   and `SHANE` without the special list), the second gets a `JUNK0`, `JUNK1`, etc.
   fallback. Emit a diagnostic.
4. **Environment speakers**: `>ENV` → `ENV`, `>DOG` → `DOG`, etc. (strip `>` prefix)

The mapping is emitted in the JSON output as `speaker_map` (TRN name → CHAT ID).
This is critical for the merge step: overlap participants in the JSON use TRN names,
but the existing CHAT files use the truncated IDs. The merge tool needs to translate.

**Validation against actual CHAT**: as a sanity check, the tool can optionally read
the `@Participants` line from the corresponding CHAT file and verify the mapping
matches. Flag mismatches as diagnostics.

### Stage 6: Text extraction (post-processing)

After brackets are classified, extract the text between each begin/end pair. For
multi-line spans, concatenate content across lines. Record the bracketed text in
each `OverlapParticipant`.

### Stage 7: CHAT emission (`emit_chat.rs`)

Direct string transformation of TRN content into CHAT format. Not an AST — just
text rewriting using the rules below, derived from studying the Java converter's
DT.flex, DTParser.g, ChatWalker.g, and Chat.stg.

#### File structure

```
@UTF8
@Begin
@Languages:	eng
@Participants:	JAMI Speaker, HARO Speaker, ...
@Options:	CA, caps, bullets
@ID:	eng|SBCSAE|JAMI|||||Speaker|||
...
@Media:	NN, audio
*JAMI:	content . ⌜start_end⌝
...
@End
```

#### Content transformation rules

| TRN | CHAT | Notes |
|-----|------|-------|
| `[...] / [N...N]` | `⌈ ⌉ / ⌊ ⌋` + index | From overlap state machine |
| `..` | `(..)` | Short pause |
| `...` | `(...)` | Medium pause (not followed by `(`) |
| `...(N.N)` | `(N.N)` | Timed pause (strip `...`) |
| `=` after letter | `:` | Lengthening (`ta=p` → `ta:p`) |
| `%` | `ʔ` or `ʔuh` | Glottal: `%` alone → `ʔuh`, `%` in word → `ʔ` |
| `--` (end of turn) | `+/.` or `+...` | Truncation → `+/.` (new speaker next) or `+...` (same speaker continues) |
| `--` (mid-content) | `-` or `+//.` | Internal truncation |
| `(H)` | `&=in` | Inhalation |
| `(Hx)` | `&=ex` | Exhalation |
| `(TSK)` | `&=tsk` | Click |
| `((NAME))` | `&=NAME` | Environmental comment → happening |
| `<X...X>` | `&{l=X ...&}l=X` | Uncertain hearing (long feature) |
| `<VOX...VOX>` | `&{l=VOX ...&}l=VOX` | Voice quality (etc. for all long features) |
| `<<NAME...NAME>>` | `&{n=NAME ...&}n=NAME` | Nonvocal sound |
| `<<NAME>>` | `&{n=NAME}` | Simple nonvocal |
| `+` (in nonvocal) | `&=nonvocal` | Beat within `<<...>>` |
| `@` | `[% laugh]` or `&=laugh` | Context-dependent |
| `@@@` | `&=laughs` | Multiple laughs |
| `<@...@>` | `&{l=@ ...&}l=@` | Laughing quality |
| `~Name` | `Name` | Known person (tilde stripped) |
| `!Name` | `Name` | Proper noun (bang stripped) |
| `#Name` | `Name` | Anonymized (hash stripped) |
| `_/word/` | `/word/` | Phonological fragment |
| `&` (end of line) | `+,` | Continuation linker |
| `&` (start of line) | `+,` | Continuation (same speaker) |
| `.` | `.` | Period terminator |
| `?` | `?` | Question terminator |
| `,` (at end) | `.` or `+,` | Context-dependent |
| `(VOCALISM)` | `&=VOCALISM` | General vocalism |
| timestamps | `⌜start_end⌝` | Milliseconds: `round(float * 1000)` |
| SPEAKER: | `*SPK:\t` | Via speaker map |
| >ENV: | Environment lines | May become `%com:` or `&=` |

#### Turn grouping

A TRN **turn** (speaker + continuation lines) maps to one or more CHAT utterances
(`*SPK:` lines). Each TRN line with a timestamp becomes a timing bullet on the
CHAT utterance. Multi-line turns produce a single `*SPK:` line with embedded
timing, or multiple `*SPK:` lines if the turn contains multiple terminators.

#### Terminator inference

The DTParser.g has complex terminator logic (lines 1247–1288). Summary:
- `.` → `.` (period)
- `?` → `?` (question)
- `--` at end, next turn is different speaker → `+/.` (interruption)
- `--` at end, next turn is same speaker → `+...` (trailing off)
- `,` at end of final line → `.` (substitute period)
- `&` at end of final line → `+/.` (continuation truncation)
- No terminator → `.` (insert period)

### Stage 8: Output assembly (`main.rs`)

Build `FileOutput` for JSON. Write `.cha` files for CHAT output. Both modes
available via CLI flags.

---

## Indentation sensitivity hints

Although spatial alignment is not authoritative for overlap pairing (§7 of format
analysis), the column offsets are potentially valuable for the **merge step** —
aligning TRN overlap regions to CHAT utterances.

Each `BracketLocation` in the output includes:
- `char_offset` — position within the line's content field
- `column` — absolute position in the original line

The `column` value encodes the visual indentation that the original transcribers
used to show overlap correspondence. When merging with CHAT, if multiple candidate
CHAT utterances could match a TRN overlap region, the column offset provides a
secondary alignment signal: the overlap bracket's visual position hints at which
word in the preceding speaker's utterance it was meant to align with.

This is explicitly a **hint**, not a constraint. The format analysis found most
alignments are accurate (within 1–2 columns) but some are wrong (off by 5+). The
merge step should use timing and content matching as primary signals, with column
position as a tiebreaker.

---

## Output JSON schema

```json
{
  "filename": "SBC002.trn",
  "format_variant": "A",
  "total_lines": 1250,
  "speaker_map": {
    "JAMIE": "JAMI",
    "HAROLD": "HARO",
    "MILES": "MILE",
    "PETE": "PETE"
  },
  "overlap_runs": [
    {
      "run_id": 0,
      "first_line": 1,
      "last_line": 4,
      "sets": [
        {
          "real_index": 0,
          "display_index": "unnumbered",
          "top": {
            "speaker": "JAMIE",
            "begin": {
              "line_number": 1,
              "char_offset": 4,
              "column": 20,
              "time_range": [0.0, 6.52]
            },
            "end": {
              "line_number": 1,
              "char_offset": 42,
              "column": 58,
              "time_range": [0.0, 6.52]
            },
            "bracketed_text": "can you teach a three-year-old to"
          },
          "bottoms": [
            {
              "speaker": "HAROLD",
              "begin": {
                "line_number": 2,
                "char_offset": 4,
                "column": 20,
                "time_range": [4.43, 5.78]
              },
              "end": {
                "line_number": 2,
                "char_offset": 33,
                "column": 49,
                "time_range": [4.43, 5.78]
              },
              "bracketed_text": "I can't imagine teaching a"
            }
          ],
          "complete": true
        }
      ]
    }
  ],
  "diagnostics": [
    {
      "severity": "warning",
      "line_number": 42,
      "column": 15,
      "code": "BracketIndexMismatch",
      "message": "Open index 3 does not match close index 2"
    }
  ]
}
```

---

## CLI interface

```
trn-overlap-extract [OPTIONS] <INPUT>...

Arguments:
  <INPUT>...  TRN files or directories to process

Options:
  -o, --output-dir <DIR>    Output directory for JSON files [default: stdout]
  --include-lines           Include full parsed line data in output
  --summary                 Emit corpus-level summary JSON
  --diagnostics-only        Only emit diagnostics, no overlap data
  -v, --verbose             Print progress to stderr
```

---

## Implementation sequence

1. `types.rs` — all data structures, derive Serialize
2. `encoding.rs` — file reading and decoding
3. `format.rs` — line parsing and format detection, with unit tests per variant
4. `bracket.rs` — bracket tokenizer with unit tests
5. `overlap.rs` — state machine port, with unit tests on hand-crafted sequences
6. `diagnostics.rs` — diagnostic types
7. `main.rs` — wire together, JSON output
8. Integration tests against real TRN files
9. Golden tests for regression protection

---

## Test strategy

### Unit tests

- **Bracket tokenizer**: `"[foo]"`, `"[3bar3]"`, `"((comment [not]]))"`,
  `"<<thump>> [real]"`, `"[3mhm4]"` (mismatch), `"[4M4][5hm=5]"` (adjacent),
  `"$]"` (forced close)
- **Overlap state machine**: SBC002 lines 1–4 (JAMIE/HAROLD), SBC002 lines 58–61
  (3-way with indices 3–5), unnumbered heuristic, same-speaker error, all error
  recovery paths

### Integration tests

- SBC002.trn — 4 speakers, moderate overlap complexity, hand-verified output
- SBC013.trn — 9 speakers, broken formatting, highest overlap density
- SBC004.trn — bracket index mismatch (`[3they2]`)
- SBC014.trn — Format C (unique variant)

### Corpus tests (`#[ignore]` by default)

- Parse all 60 files, assert zero panics
- Snapshot total bracket + diagnostic counts
- Verify ~22,000 overlap brackets total

### Golden tests

- 3–5 small file excerpts with committed expected JSON
- `--update-golden` pattern for regeneration
