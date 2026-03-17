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

## `+<` Lazy Overlap Linker: Audit and Status

### Usage is massive (not deprecated)

Contrary to earlier assumption, `+<` is heavily used across all corpora:

| Corpus | Files | Utterances with `+<` |
|--------|-------|---------------------|
| childes-data | 10,596 | 194,720 |
| phon-data | 614 | 50,892 |
| biling-data | 248 | 37,727 |
| aphasia-data | 1,241 | 15,720 |
| tbi-data | 251 | 7,469 |
| ca-data | 242 | 6,606 |
| dementia-data | 1,536 | 4,745 |
| asd-data | 234 | 3,950 |
| rhd-data | 172 | 1,693 |
| class-data | 65 | 1,197 |
| slabank-data | 153 | 947 |
| homebank-data | 67 | 576 |
| fluency-data | 194 | 339 |
| psychosis-data | 13 | 14 |
| **Total** | | **~327,000** |

Of these, roughly 131,000 (40%) already have timing bullets.

### Are the `+<` bullets actually correct?

Of the 327,000 `+<` utterances, about 107,000 (33%) have both their own
bullet and a bullet on the previous utterance, allowing us to check
whether the timing reflects actual temporal overlap.

**Results across all timed `+<` utterances:**

| Corpus | Timed `+<` | True overlap | Abutting (gap=0) | Small gap (≤500ms) | Large gap (>1s) |
|--------|-----------|-------------|-----------------|-------------------|----------------|
| biling-data | 36,530 | 25,537 (69%) | 9,084 (25%) | 870 (2%) | 627 (2%) |
| tbi-data | 3,797 | 2,722 (71%) | — | — | — |
| aphasia-data | 13,025 | 7,321 (56%) | 3,444 (26%) | 1,312 (10%) | 526 (4%) |
| dementia-data | 3,733 | 2,144 (57%) | — | — | — |
| rhd-data | 1,342 | 749 (55%) | — | — | — |
| childes-data | 45,545 | 17,880 (39%) | 13,182 (29%) | 6,549 (14%) | 5,105 (11%) |
| ca-data | 4,350 | 483 (11%) | 3,704 (85%) | 98 (2%) | 28 (<1%) |
| phon-data | 3,517 | 41 (1%) | 3,440 (97%) | 30 (<1%) | 5 (<1%) |

Three distinct patterns:

**1. True temporal overlap (biling-data, tbi-data pattern — 69-71%).**
The `+<` utterance's start time falls within the previous utterance's
time range.  These are genuine overlaps with plausible, independently
assigned timing.  Example from biling-data (166ms overlap):

```
*VER: cusì che (a)speterò me mare: +... 201222_202185
*LUC: +< fin a quando la lavora ? 202019_203576
```

**2. Abutting / clamped (ca-data, phon-data pattern — 85-97% at gap=0).**
The `+<` utterance starts exactly where the previous one ends.  This
strongly suggests the bullets were **clamped to prevent overlap** — the
original timing was overlapping, but something (CLAN's fixbullets, a
post-processing script, or the original alignment tool) forced the start
time to equal the previous end time.  Example from ca-data:

```
*INV: <you were asleep> [//] you slept through ... +/. ...  _920625
*PAR: +< <I was> [/] I was sleep . 920625_921425
```

PAR almost certainly started speaking before 920625ms (that's the `+<`
signal), but the bullet was clamped to the previous utterance's end.
The `+<` preserves the overlap information that the clamped timing lost.

**3. Mixed (aphasia-data, childes-data — 39-56% true overlap).**  A mix
of genuinely overlapping timing, clamped timing, and near-misses.  In
aphasia-data, 83% of the non-overlapping cases have gaps ≤500ms,
suggesting most are near-overlaps or minor clamping.  In childes-data,
18% have gaps >1 second, which may represent loose use of `+<` for
"quick response" rather than strict temporal overlap.

**Conclusion on bullet quality:** The timing on `+<` utterances is a
mix of genuine, clamped, and approximate.  We cannot assume the bullets
are authoritative measures of overlap timing.  However, the `+<` linker
itself is a reliable signal that the transcriber heard overlapping
speech, regardless of whether the bullet faithfully captures the exact
overlap boundary.  For the two-pass UTR approach, the `+<` signal is
what matters — the aligner will assign fresh timing from the audio,
not rely on existing bullets.

### Batchalign2 ignored `+<` entirely

At the Jan 9, 2026 baseline (commit `84ad500b`):

- `annotation_clean()` in `formats/chat/utils.py` strips all `+`
  characters (`.replace("+","")`), destroying `+<` along with all other
  linkers.
- `morphosyntax/ud.py` explicitly strips `+<` before Stanza:
  `line_cut.replace("+<", "")`.
- No code anywhere in ba2 reads `+<` as a signal for alignment, timing,
  or any other purpose.

### `+<` as an alignment hint

`+<` means "this utterance started before the previous one finished."
This is exactly the information the aligner needs to handle overlapping
utterances correctly.  If a `+<` utterance is present, the aligner knows:

1. This utterance's timing overlaps with the previous utterance.
2. Its words should not be included in the global UTR reference (they'd
   be out of temporal order relative to the previous utterance's words).
3. Its timing can be constrained to fall within or near the previous
   utterance's time range.

The simplest algorithmic improvement would be: **skip `+<` utterances
during UTR**, then assign them timing in a second pass using the
surrounding context.  This is ~5 lines of Rust and eliminates the
repeated-token ambiguity concern for the separate-utterance encoding.

## Davida's Feedback (2026-03-16)

### Why `&*` was used instead of separate utterances

> "We're not doing it now because batchalign couldn't handle it."

This confirms the convention was a workaround for alignment tool
limitations, though as documented above, ba2 itself had no technical
prohibition — it simply ignored `&*` content.  The perceived limitation
may have been from batchalign 1 or from practical experience with poor
alignment results on overlapping files.

### Multi-backchannel example

```
*PAR: but I grew up in Princeton New_Jersey &*INV:oh_okay_yeah and
      &-uh came to graduate school &*INV:mhm at UNC Chapel_Hill
      &*INV:oh and [//] &-uh in ninety one &*INV:mhm or maybe
      ninety two . 104745_118254
```

Under the natural encoding (Option B, PAR unsplit):

```
*PAR: but I grew up in Princeton New_Jersey and &-uh came to graduate
      school at UNC Chapel_Hill and [//] &-uh in ninety one or maybe
      ninety two . 104745_118254
*INV: +< oh okay yeah .
*INV: +< mhm .
*INV: +< oh .
*INV: +< mhm .
```

Each backchannel is a separate utterance on INV's tier.  PAR's utterance
is intact.  The INV utterances have no timing yet — `align` would need
to assign it.

### Partial overlap example

```
*PAR: without looking at the pictures ? 98040_99243
*INV: +< tell me a story that goes with the pictures . 99070_100786
```

INV starts during PAR (99070 < 99243) but continues after PAR finishes
(100786 > 99243).  This is valid CHAT — start times are non-decreasing
(98040 ≤ 99070).  The `+<` marks the overlap for human readers.

## Our Recommendations

### Transcript format: Davida's Option B + `+<`

```
*PAR: but I grew up in Princeton New_Jersey and &-uh came to graduate
      school at UNC Chapel_Hill and [//] &-uh in ninety one or maybe
      ninety two . 104745_118254
*INV: +< oh okay yeah .
*INV: +< mhm .
*INV: +< oh .
*INV: +< mhm .
```

Rationale:

- **PAR stays intact.**  The participant's narrative is one utterance.
  This preserves MLU, utterance counts, and turn structure for analysis.

- **Each backchannel is a separate utterance on its own speaker tier.**
  Countable, analyzable, can receive its own timing.

- **`+<` marks the overlap.**  This is already massively established
  (327,000 existing uses).  It costs the transcriber nothing, is
  immediately readable by humans, and gives the aligner a machine-readable
  signal that the utterance overlaps with the previous one.

- **Utterances ordered by start time.**  Satisfies E701 naturally.  For
  the multi-backchannel case, the `+<` utterances go after PAR's
  utterance in text order (PAR's start time is earliest).

- **`+<` for partial overlaps too.**  When INV starts during PAR but
  continues past:

  ```
  *PAR: without looking at the pictures ? 98040_99243
  *INV: +< tell me a story that goes with the pictures . 99070_100786
  ```

  Valid CHAT (98040 ≤ 99070).  `+<` documents the overlap.

### `+<` should be recommended, not required

For new transcription, recommend `+<` on all overlapping utterances.
It helps both humans and the aligner.  But do not reject files without
it — overlapping timestamps alone are sufficient for the aligner to
work correctly via the existing global DP path.

For existing files that already have `+<` (327,000 utterances), no
changes needed.

### Alignment improvement: two-pass UTR using `+<`

The concrete algorithm change we recommend:

**Pass 1 — Main speakers:** Build the global UTR reference from only
non-`+<` utterances.  These are the main speaker turns, in temporal
order, with no crossing alignment.  The global Hirschberg DP runs as
today and assigns utterance-level timing.

**Pass 2 — Overlap utterances:** For each `+<` utterance, its timing
window is known from context: the previous utterance's bullet range
(possibly expanded by a small buffer).  FA can align the `+<`
utterance's words directly against that audio window.  No DP is
needed — just FA on a constrained segment.

This is better than all of the proposals from earlier in this session:

- **Better than backbone extraction:** That was solving a problem that
  doesn't exist (`&*` is already invisible to the DP).

- **Better than per-speaker reference partitioning:** That would give
  each speaker their own DP pass, but INV's backchannel reference
  ("mhm yeah mhm oh mhm") matched against the full-file ASR still
  has repeated-token ambiguity.  The `+<` approach avoids DP entirely
  for backchannels.

- **Better than the full per-speaker UTR proposal:** That required
  diarization, per-speaker audio extraction, and complex merge logic.
  The `+<` approach needs none of that — the speaker code and `+<`
  linker are already in the transcript.

**Scope of the change:** ~20-30 lines in `utr.rs` (filter `+<`
utterances from `collect_utr_utterance_info`, then a post-pass that
assigns timing from the previous utterance's bullet).  No new crates,
no new worker protocol, no diarization.

**Backward compatible:** Files without `+<` use the existing global
DP path unchanged.  Files with `+<` get better alignment for
backchannel utterances.

### Both `&*` and separate utterances remain valid

Do not force migration.  Both encodings work:

- `&*` is invisible to the DP (walker skips `OtherSpokenEvent`).
  Backchannels get no timing.  Fine for existing corpora.

- Separate utterances with `+<` give backchannels their own timing
  and analysis visibility.  Recommended for new transcription.

Transcribers can adopt the new convention organically.  A migration
tool is a future option, not a prerequisite.

## Questions for Brian

These need Brian's input.  We include our suggested answers for his
review.

1. **Should `+<` be required on overlapping utterances?**

   Our suggestion: **Recommended, not required.**  327,000 existing
   uses show it's established practice.  It helps both human readability
   and alignment quality.  But files with overlapping timestamps and no
   `+<` should not be rejected.

2. **For multi-backchannel cases, should each be a separate `+<`
   utterance?**

   Our suggestion: **Yes.**  Each backchannel is a separate
   conversational act.  One `+<` utterance per backchannel, each on
   its own line.  This is the most natural representation and the
   easiest for analysis tools.

3. **How should ordering work when the overlap speaker continues past
   the main speaker?**

   Our suggestion: **Order by start time** (satisfies E701).  The
   overlapping speaker's utterance follows the main speaker's in text
   order because its start time is later.  `+<` marks the overlap.

4. **Does CLAN need changes?**

   Our assessment: **No.**  CLAN's playback handles overlapping bullets
   correctly (verified from OSX-CLAN source).  But Brian may know of
   other CLAN features (analysis commands, the editor) that assume
   non-overlap.

5. **Should we pursue the two-pass UTR improvement?**

   Our suggestion: **Yes.**  It's a small change (~20-30 lines of Rust)
   that makes `align` aware of `+<` for the first time.  327,000
   existing `+<` utterances benefit immediately.  No risk to files
   without `+<`.
