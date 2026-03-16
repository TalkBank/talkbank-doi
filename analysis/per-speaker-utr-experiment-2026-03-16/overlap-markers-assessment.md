# Assessment: `&*` Overlapping Speech Markers in TalkBank Corpora

**Status:** Current
**Last updated:** 2026-03-16

## What `&*` Is

The `&*` marker embeds one speaker's words inside another speaker's
utterance.  The syntax is `&*SPK:word` or `&*SPK:word_word` (underscores
join compound expressions).  It represents speech that overlaps temporally
with the main speaker's utterance.

```
*PAR: I went to the store &*INV:mhm and bought some milk . 0_6000
```

Here, INV said "mhm" while PAR was talking.  The `&*INV:mhm` is placed at
the approximate position in PAR's text where the overlap occurred.

## Why `&*` Exists

`&*` became a transcription convention at some point in TalkBank's history,
used in at least 8 corpora with ~35,000 markers.

### Was `&*` a workaround for alignment limitations?

**No — at least not for batchalign2.**  Examination of batchalign2-master
at the Jan 9, 2026 baseline (commit `84ad500b`) shows:

- **UTR** (`utr/utils.py:bulletize_doc`): Flattens all speakers' words
  into one reference sequence for global DP alignment.  No speaker
  awareness, no overlap checking.  If the input already has utterance
  timing, UTR skips entirely.

- **FA** (`fa/wave2vec_fa.py`): Groups utterances into ~15-second windows.
  Overlapping utterances from different speakers land in the same group
  naturally.  The FA model aligns all words from all speakers against the
  combined audio window.

- **No monotonicity enforcement.**  No E362, E704, or overlap check in
  the pipeline.  The output is whatever the DP and FA produce, including
  overlapping bullets.

- **No post-alignment validation gate.**

Ba2 would have processed overlapping cross-speaker bullets without
complaint.  The `&*` convention must have originated earlier — possibly
from batchalign 1, from CLAN's CHECK command (which enforces E701/E362
start-time ordering), or simply from transcription practice guidelines
that predate the automated alignment tools.

**Regardless of origin, `&*` is not required by any current tool.**  The
CHAT format allows cross-speaker overlapping bullets: E701 only requires
non-decreasing start times, and E704 only prohibits same-speaker
self-overlap beyond 500ms.  Two different speakers can overlap freely.

## What the Corpora Actually Contain

### Scale

| Corpus | Files with `&*` | Total `&*` markers |
|--------|----------------|-------------------|
| ca-data | 256 / 6,606 | 12,016 |
| aphasia-data | 644 / 6,298 | 10,161 |
| rhd-data | 190 / 300 | 5,160 |
| psychosis-data | 236 / 979 | 2,799 |
| tbi-data | 135 / 1,116 | 2,105 |
| dementia-data | 390 / 6,131 | 1,680 |
| slabank-data | 191 / 9,637 | 774 |
| childes-data | 146 / 49,718 | 411 |
| **Total** | | **~35,000** |

### Content: overwhelmingly single-word backchannels

| Corpus | Single-word | Multi-word | % single |
|--------|------------|-----------|----------|
| ca-data | 11,573 | 443 | 96% |
| aphasia-data | 8,966 | 1,195 | 88% |
| psychosis-data | 2,741 | 58 | 98% |
| tbi-data | 1,887 | 218 | 90% |
| dementia-data | 1,502 | 178 | 89% |
| rhd-data | 4,307 | 853 | 83% |
| **Total** | **30,976** | **2,945** | **91%** |

The top words across all corpora:

| Word | Count |
|------|-------|
| mhm | ~12,500 |
| yeah | ~5,500 |
| okay | ~3,300 |
| mm | ~1,400 |
| uhhuh | ~560 |
| oh | ~500 |
| right | ~200 |
| yes | ~200 |

These are textbook backchannels — the listener acknowledging the speaker.

### Multi-word content: still backchannels

The "multi-word" `&*` markers are compound backchannels joined by
underscores:

| Content | Count |
|---------|-------|
| oh_okay | 173 |
| yeah_yeah | 93 |
| oh_yeah | 78 |
| oh_wow | 87 |
| oh_no | 48 |
| oh_my_gosh | 41 |
| mhm_mhm | 72 |
| I_know | 31 |

These are the same phenomenon — listener feedback — just slightly longer.

### Who gets embedded

The `&*` speaker (the one whose words are embedded) is almost always the
investigator:

| Corpus | INV embedded | PAR embedded | Other |
|--------|-------------|-------------|-------|
| aphasia-data | 7,335 (72%) | 2,650 (26%) | 176 (2%) |
| ca-data | 10,294 (86%) | 1,690 (14%) | 32 (<1%) |

The pattern is clear: PAR talks, INV says "mhm", and the transcriber
embeds INV's backchannel into PAR's utterance.

### Density: usually sparse

Lines with 2+ `&*` markers (multiple backchannels in one utterance):
- aphasia-data: 539 lines (5.3% of all `&*` lines)
- ca-data: 591 lines (4.9%)

Lines with 3+ `&*` markers (dense backchannel during a long narrative):
- aphasia-data: 40 lines
- ca-data: 54 lines

The 3+ cases look like this:

```
*PAR: okay , there was this man on his deathbed &*INV:mhm talking
      with his sons about a vineyard &*INV:mhm that was going to
      grow &*INV:mhm good produce .
```

PAR is telling a long story.  INV says "mhm" three times during it.
These are three separate conversational acts that should be three separate
utterances.

### Position within the utterance

In aphasia-data, 2,824 of ~10,000 `&*` markers (28%) are at the very
beginning of the utterance:

```
*PAR: &*INV:mhm and then I went to the store .
```

This means the backchannel happened *before* or *at the start of* PAR's
turn.  Embedding it in PAR's utterance is especially unnatural — it's not
even overlapping with PAR's speech, it's a separate prior turn.

## Problems with `&*`

### For transcribers

1. **Unnatural representation.**  Forcing the transcriber to decide which
   speaker "owns" an overlap and embedding the other speaker's words.

2. **Ambiguous overlap boundaries.**  The `&*` marker's position in the
   text is approximate — it doesn't encode exactly when the overlap starts
   and ends relative to the main speaker's words.

3. **Unreadable at density.**  Three `&*INV:mhm` markers in a single
   utterance obscures the main speaker's content.

4. **Arbitrary start-of-utterance embedding.**  `*PAR: &*INV:mhm and
   then...` — the backchannel isn't overlapping PAR's speech, it's a
   separate turn that the convention forces into the wrong place.

### For alignment

1. **~~Reference sequence pollution~~ — NOT a problem.**  The current Rust
   code already excludes `&*` (`OtherSpokenEvent`) from the DP reference
   sequence.  `collect_fa_words()` uses the content walker, which skips
   `OtherSpokenEvent` in its match arms.  Both UTR and FA word extraction
   use this path.  The backbone extraction proposal drafted earlier in
   this session was solving a problem that doesn't exist.

2. **No separate timing.**  The `&*` content has no bullet of its own —
   its timing is subsumed by the host utterance's bullet.  Word-level
   timing for the backchannel is unavailable.

3. **No `%wor` participation.**  The content walker skips `OtherSpokenEvent`
   for all domains, so `&*` words are excluded from `%wor` tier generation.
   This is correct — the `&*` content belongs to a different speaker and
   should not appear in the host utterance's dependent tiers.

### For analysis

1. **Backchannels not independently accessible.**  CLAN analysis commands
   that count utterances, compute turn-taking metrics, or analyze speaker
   participation cannot easily count embedded `&*` markers as separate
   conversational turns.

2. **Timing precision lost.**  Research on overlap timing, backchannel
   placement, or turn-transition timing cannot be done when the backchannel
   has no independent timestamp.

## The Natural Alternative

Each speaker turn gets its own utterance, ordered by start time:

```
*PAR: I went to the store . 0_3500
*INV: mhm . 2000_2500
*PAR: and bought some milk . 3500_6000
```

Instead of:

```
*PAR: I went to the store &*INV:mhm and bought some milk . 0_6000
```

### Why this is already valid CHAT

- **E701** (global timeline): Start times are non-decreasing
  (0 ≤ 2000 ≤ 3500).  Passes.
- **E704** (same-speaker self-overlap): PAR 0–3500 and 3500–6000 don't
  overlap.  INV has only one utterance.  Passes.
- **E362** (monotonicity): Same as E701.  Passes.

Cross-speaker overlap (PAR's 0–3500 overlaps INV's 2000–2500) is allowed
by all validation rules.

### Playback behavior (verified from source code)

- **CLAN**: Plays each utterance in full, in document order.  The overlap
  region is heard twice (once per speaker).  Handles this correctly.
- **VS Code extension**: Same behavior as CLAN.  Documented in GUIDE.md.

### Alignment tradeoffs

The separate-utterance encoding changes what the aligner sees, and the
tradeoffs are different from what we initially assumed.

**With `&*` (current corpora):** The content walker skips
`OtherSpokenEvent`, so `&*` words are invisible to the DP.  PAR's
utterance reference contains only PAR's words.  The ASR produces all
speakers' words in temporal order, so the extra backchannel words in the
ASR are unmatched noise that the DP skips as insertions.  For isolated
backchannels this works well.  For dense backchannels (3+ `&*INV:mhm`
in one utterance), the unmatched ASR words are minor noise that the
global DP handles.  **`&*` is effectively invisible and harmless to
alignment.**

**Without `&*` (separate utterances, Option B):** PAR's utterance is
still one unit with all of PAR's words.  But INV's "mhm" is now a
separate utterance that participates in the UTR reference sequence.
The UTR flattens all utterances' words in text order:

```
Reference: [PAR's 14 words] [INV's "yeah"]
ASR:       [...PAR-words... "yeah" ...more-PAR-words...]
```

"yeah" appears at the end of the reference (because INV's utterance
follows PAR's in text order) but in the middle of the ASR (because INV
spoke during PAR's utterance).  The monotonic DP cannot match "yeah" to
its correct ASR position (which is before some of PAR's later matches).
It either:

1. Leaves INV's "yeah" unmatched (harmless — INV gets no UTR timing,
   FA fills in with proportional estimation).
2. Matches "yeah" to a later occurrence in the ASR if one exists,
   potentially consuming a token that should match a different word.

For a single backchannel, outcome 1 is overwhelmingly likely and benign.
For dense backchannels (5+ "mhm" utterances interleaved during a long
PAR narrative), the reference now contains 5 repeated "mhm" tokens at
positions that don't match their ASR positions.  This is **Known DP
Failure Mode #2: repeated-token ambiguity.**  The DP has many equal-cost
paths and may pick semantically wrong matches.

**Practical risk assessment:** Low to moderate.  The backchannel words
("mhm", "yeah") are common in ASR output and appear frequently.  The DP
may match them to wrong positions, but this affects only the backchannel
utterances' timing, not PAR's.  PAR's words are unique content that
matches unambiguously.  The worst case is that some backchannel utterances
get no timing or wrong timing — which is exactly what happens with `&*`
today (no timing at all), so the separate-utterance encoding is no worse
for backchannel timing and potentially better (FA can attempt alignment).

**Net assessment:** The separate-utterance encoding does not harm the
main speaker's alignment.  It may slightly degrade backchannel utterance
timing in dense-overlap cases compared to a hypothetical per-speaker
aligner, but it cannot be worse than `&*` (which gives backchannels zero
timing).  For practical purposes, both encodings work with the current
aligner.

## The PAR-continues question

When the main speaker is interrupted by a backchannel but continues the
same thought, the transcriber must decide whether the main speaker's
utterance is one unit or two.

### Option A: Split at the backchannel

```
*PAR: I went to the store . 0_3500
*INV: mhm . 2000_2500
*PAR: and bought some milk . 3500_6000
```

PAR's thought is two utterances.  Each has its own timing.

### Option B: PAR is one utterance, INV interleaves

```
*PAR: I went to the store and bought some milk . 0_6000
*INV: mhm . 2000_2500
```

PAR's utterance is one continuous thought.  INV's backchannel is a
separate utterance interleaved by start time.  PAR's bullet (0–6000)
covers the full range.  INV's bullet (2000–2500) is inside PAR's range.

This is valid CHAT as long as utterances are ordered by start time:
start 0 ≤ start 2000.  ✓

**But:** INV's utterance now comes *after* PAR's in text order, even
though it occurred *during* PAR's speech.  This is honest about the
structure — INV's turn started after PAR's turn started.

### Recommendation: Option B (do not split the main speaker)

Davida's preference (confirmed 2026-03-16) is Option B: **keep the main
speaker's utterance intact and put the backchannel on its own line.**

```
*PAR: when I had my first stroke I had no idea what happened to me . 1000_2000
*INV: +< yeah . 1400_1500
```

Rationale:

- **The participant's thought is one unit.**  Splitting it just because
  the investigator said "mhm" in the middle would fragment the
  participant's utterance for analysis purposes (MLU, utterance counts,
  turn-taking metrics).

- **The investigator's backchannel is a separate conversational act.**
  It should be on its own speaker tier so it can be tracked, counted,
  and analyzed independently.

- **The `+<` lazy overlap linker** (already in our grammar as
  `Linker::LazyOverlapPrecedes`) marks that INV's utterance started
  before PAR's finished.  With bullets, the overlap is already evident
  from timing (1400 is inside 1000–2000), so `+<` is optional but
  provides a human-readable signal.  Whether to require `+<` on
  overlapping utterances is a convention choice — it's valid CHAT
  either way.

- **This applies equally to longer overlaps** — not just "yeah" but
  family members saying full sentences during the participant's speech.
  Each speaker's words belong on their own tier.

Either way, `&*` is not used.

## Migration Path for Existing Corpora

### Scale of the problem

~35,000 `&*` markers across ~2,200 files in 8 corpora.

### Feasibility of automated migration

High.  91% of markers are single-word backchannels with a known word.
The migration tool would:

1. Parse the CHAT file using `talkbank-parser`.
2. For each utterance containing `&*` markers, walk the AST to find
   `OverlappingSpeech` nodes (or the equivalent content type).
3. Extract the `&*` content (speaker code + word).
4. Create a new `Utterance` for the extracted content.
5. Assign timing: interpolate from the surrounding words' inline bullets
   (if the file has word-level timing from `%wor`), or from the
   utterance-level bullet proportionally.
6. Insert the new utterance at the correct position (sorted by start
   time) and remove the `&*` marker from the host utterance.
7. If the host utterance should be split at the backchannel position
   (Option A), split it into two utterances with appropriate timing.
8. Serialize back to CHAT.

### What the tool should NOT do automatically

- **Split utterances without timing data.**  If the file has no word-level
  timing, the tool cannot determine where to split PAR's utterance.  It
  should extract the `&*` content as a separate utterance (with
  interpolated timing) but leave PAR's utterance intact.

- **Migrate files that are under active review.**  Let Davida finish her
  current review workflow before migrating files she's working on.

### Implementation

The tool should use the Rust model directly (like the `utr-experiment`
tool), not text hacking.  It would be a `chatter` subcommand or a
standalone binary.

### Validation after migration

After migration, every output file must:
1. Parse without errors.
2. Pass `chatter validate` (including E701, E704, E362).
3. Roundtrip cleanly (`to-json` → `from-json` → compare).
4. Have the same number of speaker-attributed words as the input (no
   content lost).

## Recommendations

### Support both encodings; do not force migration

Both `&*` and separate-utterance encoding are valid CHAT, both work with
the current aligner, and both play back correctly in CLAN and VS Code.
Transcribers should use whichever they prefer.  The tradeoffs are:

| | `&*` encoding | Separate utterances |
|-|---------------|---------------------|
| Transcription | Unnatural; transcriber must pick "owner" | Natural; each speaker on own line |
| Alignment (main speaker) | Identical — `&*` is invisible to DP | Identical — PAR's words match the same |
| Alignment (backchannel) | No timing at all | May get timing from FA; may fail on dense repeats |
| `%wor` for backchannel | No entry | Gets own `%wor` tier on own utterance |
| Analysis | Backchannels invisible to utterance-counting commands | Backchannels are full utterances, countable |
| Readability | Poor at density (3+ `&*` per line) | Clean |
| DP risk | None — `&*` filtered by walker | Low — repeated backchannel tokens add minor ambiguity |
| Legacy compatibility | 35,000 markers across 8 corpora | Would require migration |

### For new transcription

Recommend separate utterances (Option B).  Do not break up the main
speaker's utterance; put the backchannel on its own line:

```
*PAR: when I had my first stroke I had no idea what happened to me . 1000_2000
*INV: +< yeah . 1400_1500
```

The `+<` lazy overlap linker is optional but provides a human-readable
signal.  Document this in transcription guidance.

`&*` remains valid CHAT and should not be prohibited — some transcribers
may prefer it for very short overlaps, and it causes no alignment
problems.

### For existing corpora

Do not force migration.  The `&*` encoding works correctly with the
current aligner (invisible to DP, no timing impact on the main speaker).
Migration would change ~35,000 markers across ~2,200 files with no
guarantee of better alignment results and some risk of introducing
repeated-token ambiguity in dense-backchannel regions.

If a migration tool is built in the future, it should be opt-in, run on
a test corpus first, and produce output that Davida reviews before
scaling.

### For `align`

No algorithm changes needed for either encoding.  The current code:

- Skips `&*` content via the content walker (`OtherSpokenEvent => {}`)
- Handles separate-utterance encoding correctly via the global
  Hirschberg DP (backchannel words are minor noise at the end of the
  reference sequence)

The backbone-extraction and per-speaker UTR proposals drafted earlier
in this session were based on incorrect assumptions about `&*` polluting
the reference sequence.  They are not needed.

If dense-backchannel files produce alignment problems in practice
(Davida's report will tell us), the right fix would be per-speaker DP
alignment, not reference-sequence manipulation.

### For chatter validation

No changes needed.  E701, E704, and E362 already allow cross-speaker
overlapping bullets.

### For analysis commands

Consider whether utterance-counting commands (freq, mlu, mlt, etc.)
should optionally count `&*` markers as separate turns.  This would
let researchers analyze backchannel frequency without migrating files.

### Preserve `&*` in the grammar and parser

`&*` remains a valid CHAT construct.  Even if new transcription guidance
recommends separate utterances, existing corpora use `&*` and will
continue to.  The grammar, parser, model (`OtherSpokenEvent`), and
content walker behavior should not change.
