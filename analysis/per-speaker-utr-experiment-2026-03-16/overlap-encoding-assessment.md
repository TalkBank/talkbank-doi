# Overlap Encoding Assessment

**Status:** Draft
**Last updated:** 2026-03-16

## Key Discovery: Overlapping Bullets Are Already Valid CHAT

After reading the actual validation code, CHAT already allows cross-speaker
overlapping bullets.  The temporal constraints are:

- **E701** (global timeline): Start times must be non-decreasing in text
  order.  Utterances sorted by start time satisfy this regardless of whether
  their time ranges overlap.

- **E704** (per-speaker self-overlap): The same speaker's consecutive
  utterances cannot overlap by more than 500ms.  This prevents a single
  speaker from having two utterances covering the same audio segment.

- **E362** (monotonicity): Start times must be non-decreasing — same as
  E701, just checked at a different point in the pipeline.

**None of these prohibit cross-speaker overlap.**  Two different speakers
can have overlapping time ranges as long as their utterances are ordered by
start time in the file.

### Example: Natural Encoding (Already Valid)

Audio:
```
PAR: "I went to the store"         0.0 ──── 3.5s
INV:            "mhm"              2.0 ─ 2.5s
PAR:                 "and bought"  3.5 ──── 6.0s
INV:                       "nice"  5.5 ── 6.8s
```

CHAT (ordered by start time):
```
*PAR: I went to the store . 0_3500
*INV: mhm . 2000_2500
*PAR: and bought some milk . 3500_6000
*INV: oh nice . 5500_6800
```

E701: 0 ≤ 2000 ≤ 3500 ≤ 5500 — all start times non-decreasing.  ✓
E704: PAR (0_3500, 3500_6000) — no self-overlap.  INV (2000_2500, 5500_6800) — no self-overlap.  ✓

### What `&*` Was Solving

The origin of the `&*` convention is unclear.  It was **not** a workaround
for batchalign2's alignment limitations — ba2 at the Jan 9 baseline
(commit `84ad500b`) handled overlapping bullets without complaint: no
monotonicity enforcement, no overlap check, no validation gate.  The FA
pipeline grouped overlapping cross-speaker utterances into the same window
and aligned all words against the combined audio.

The convention may have originated from CLAN's CHECK command (which warns
about timing issues), from transcription practice guidelines, or from an
earlier tool.  See `overlap-markers-assessment.md` for the full analysis.

### Where `&*` Causes Problems

The `&*` encoding is:

- **Unnatural for transcribers.**  The transcriber must decide which speaker
  "owns" an overlap and embed the other speaker's words inside that
  utterance.  This is an arbitrary choice that doesn't reflect the audio.

- **Not hostile to alignment** (contrary to earlier claims in this
  session).  The content walker skips `OtherSpokenEvent`, so `&*` words
  are invisible to the DP reference sequence.  See the detailed
  assessment in `overlap-markers-assessment.md`.

- **Lossy.**  The exact overlap boundary (when the second speaker starts
  relative to the first) is not encoded — the `&*` words are just placed
  at an approximate position in the text.  The backchannel gets no timing
  and no `%wor` entry.

- **Unreadable for dense overlap.**  Three `&*` markers in a single
  utterance (as in the 2265_T4 post-mortem) is barely comprehensible.

## What Davida's Report Should Contain

### Essential: Concrete reproducing examples

For each problem class, she should provide:

**a) The exact file** (path in the corpus or a copy of the file).

**b) The exact input she gave to `align`** — was it raw ASR output, hand-edited,
a re-run of a previously aligned file, or a file with timing stripped?

**c) What she expected** — a snippet of 5-10 utterances showing what the timing
should look like.

**d) What she got** — the same snippet from the `align` output, showing what went
wrong (untimed, wrong timing, destroyed existing timing, etc.).

**e) What she had to fix by hand** — the manual correction and roughly how long it
took.  This is the real cost metric.

### Categorized by problem type

Ask her to tag each example with one of these categories:

1. **Timing loss** — utterances that should have gotten timing but didn't.
   - Were they in overlap-heavy sections?
   - Were they her edits or original structure?
   - How many per file, and how clustered?

2. **Wrong timing** — bullets assigned but pointing to the wrong audio.
   - This is worse than untimed because it looks correct but isn't.
   - How did she detect it?  (Playback?  Visual inspection?)

3. **`&*` encoding pain** — cases where the encoding forced her to represent
   conversation unnaturally.
   - What she would have written with separate speaker lines and overlapping bullets.
   - Cases where `&*` is ambiguous or lossy.

4. **Re-alignment damage** — editing a few utterances and re-running `align`
   destroyed timing on unmodified utterances.
   - Did `align --before` help?  If not, what happened?

5. **Workarounds** — manual steps she does before/after `align` to get better
   results (reordering utterances, stripping markers, running in a specific
   sequence, adding to no-align list, etc.).

### Essential: The gold-standard four-version chain

For each problem file, we need:

1. **Raw ASR output** — the unedited `transcribe` output.
2. **Her edited input** — the transcript after her review, before alignment.
3. **The `align` output** — showing the problems.
4. **Her final corrected version** — the gold standard after manual fixes.

That chain lets us measure exactly how much human correction each file
needed and test whether algorithm changes reduce it.

### Desirable: Her ideal CHAT encoding

For 2-3 dense-overlap excerpts (10-20 utterances each), she should write
the transcript the way she'd *want* it to look if there were no technical
constraints.  Compare it to the `&*` encoding she was forced to use.  This
tells us exactly what the target representation is.

## What Ideal CHAT Looks Like for Dense Conversation

### Principle: One utterance per speaker turn, ordered by start time

```
*PAR: I went to the store and . 0_3500
*INV: mhm . 2000_2500
*PAR: and I bought some milk . 3500_6000
*INV: oh nice . 5500_6800
*REL: she always buys milk . 6200_8000
*PAR: yeah well it was on sale . 7000_9000
```

Rules:
- Each speaker turn is its own utterance on its own line.
- Each utterance has its own timing bullet from the audio.
- Utterances are ordered by start time (satisfies E701).
- Cross-speaker overlap is natural and expected.
- Same-speaker self-overlap is prohibited beyond 500ms (E704).

### When to split vs. merge within a speaker

A single speaker who pauses and resumes should get separate utterances:

```
*PAR: I went to . 0_2000
*PAR: the store . 2500_3500
```

A single speaker who runs on without a pause should be one utterance:

```
*PAR: I went to the store and bought some milk . 0_6000
```

The criterion is the audio, not the text — if there's a clear pause, split;
if the speech is continuous, don't.

### Backchannels

Short backchannels ("mhm", "yeah", "okay") are separate utterances with
their own timing, interleaved at the correct start-time position:

```
*PAR: so I was thinking about going to . 5000_8000
*INV: mhm . 6500_7000
*PAR: the beach this weekend . 8000_10000
```

NOT embedded as `&*`:

```
*PAR: so I was thinking about going to &*INV:mhm the beach this weekend . 5000_10000
```

### Dense overlap (multiple speakers talking simultaneously)

For passages where two speakers genuinely talk over each other for several
seconds, each speaker gets their own utterances in start-time order:

```
*PAR: well I think the problem is . 10000_13000
*INV: but have you considered . 11000_13500
*PAR: that nobody listens . 13000_15000
*INV: a different approach . 13500_16000
```

Cross-speaker bullets overlap naturally.  The CA overlap point markers
(⌈⌉⌊⌋) can optionally mark the exact overlap boundaries within the
text, but they don't affect timing.

### What `&*` should be reserved for

`&*` should only be used for its original CHAT purpose: marking a very
brief interjection that is genuinely embedded in another speaker's
utterance at a specific textual position — where the position matters
for linguistic analysis, not just for alignment convenience.  If the
interjection is a separate conversational turn (which most backchannels
are), it should be a separate utterance.

## How `align` Should Handle This

### The good news

If CHAT files use the natural encoding (separate speaker lines, overlapping
bullets, start-time ordered), `align` should work **better** than it does
today with `&*` encoding:

1. **Each utterance is a clean unit.**  No embedded `&*` words polluting the
   reference sequence.

2. **Each utterance has a clear audio window.**  The utterance bullet defines
   where to look in the audio.  Cross-speaker audio in that window is noise
   that the FA model handles naturally (it's trained on real audio with
   background speakers).

3. **UTR matching is simpler.**  Each speaker's word sequence, in text order,
   matches the audio temporal order — because utterances are sorted by start
   time.  The crossing-alignment problem disappears.

4. **Monotonicity enforcement rarely triggers.**  If utterances are in
   start-time order, the post-FA monotonicity pass has nothing to strip.

### What needs to change in batchalign3

1. **Nothing in the core alignment algorithm.**  The current global DP + FA
   pipeline already works correctly when the reference sequence is in
   temporal order.

2. **Transcription convention guidance.**  Tell transcribers to use separate
   speaker lines ordered by start time, not `&*` encoding.  This is a
   documentation/training change, not a code change.

3. **Post-align `&*` migration tool (optional).**  For existing corpora
   encoded with `&*`, a tool that converts `&*` segments to separate
   utterances (using the surrounding timing context to assign bullets)
   would let those corpora benefit from the natural encoding without
   manual re-transcription.

4. **Validation in `align` output.**  `align` should verify that its output
   satisfies E701 and E704.  It already does this.

### What needs to change in chatter

Nothing.  The validation rules (E701, E704, E362) already allow
cross-speaker overlapping bullets.  The format is already valid.

### CLAN playback behavior (verified from OSX-CLAN source)

CLAN handles overlapping cross-speaker bullets correctly for practical use:

- **Click-to-play**: Seeks to the clicked utterance's bullet start, plays
  to its end.  No awareness of other utterances.  Works perfectly with
  overlapping bullets.

- **Continuous playback (Shift+F7)**: Advances through utterances in
  **document order**.  When segment A ends, seeks to segment B's start.
  If B's time range overlaps A's, the overlapping audio is heard twice —
  once as part of each speaker's turn.  CLAN does not prohibit backward
  seeks; it simply plays each bullet's full range in document order.

- **Segment chaining**: Bullets are grouped into a linked-list chain
  (`struct AVInfoNextSeg`) in document order.  A new segment is created
  when a dash appears in the bullet or a different media file is referenced.
  Otherwise end times are extended.

**Key finding**: CLAN's playback is document-order driven, not media-time
driven.  It does not assume non-overlapping bullets.  Cross-speaker overlap
results in replayed audio (the overlap region is heard once per speaker),
which is reasonable for transcription review.

Source files examined:
- `OSX-CLAN/src/mac/DocumentWinController.mm` — bullet parsing, segment
  chain construction, continuous playback initiation
- `OSX-CLAN/src/mac/AVController.mm` — playback time monitoring, segment
  advancement

### VS Code extension playback behavior (verified from source)

The VS Code extension follows the same pattern as CLAN:

- **Play at Cursor**: Plays the single segment.  Unaffected by overlap.

- **Continuous Play**: Plays segments sequentially in document order.
  Overlapping audio is heard twice, matching CLAN behavior.

- **Waveform overlays**: Overlapping segments render as stacked colored
  bars.

- **No overlap-specific handling**: No merging, reordering, or simultaneous
  playback is attempted.  This is intentional.

The extension's bullet parser (`bulletParser.ts`) returns segments in
document order and assumes segments are ordered by line number.  The
`findNearestBullet()` function uses a line-based search with early exit.
All of this works correctly with overlapping bullets as long as utterances
are ordered by start time in the document (which satisfies E701).

VS Code docs updated: `GUIDE.md` now has an "Overlapping Bullets" section
explaining the playback behavior.  `DEVELOPER.md` now has a "Media Playback
Architecture" section documenting the segment sourcing, playback loop, and
overlap handling.

### What needs to change in CLAN

Nothing.  CLAN already handles overlapping cross-speaker bullets correctly.

### What needs to change in chatter

Nothing.  The validation rules (E701, E704, E362) already allow
cross-speaker overlapping bullets.

## Action Items

1. **Ask Davida for the report** structured as described above.  Emphasize:
   re-test with the current batchalign3 build before reporting problems.
   The version she was using had known bugs that have since been fixed.
   Her report should reflect the current system's behavior, not legacy
   failures.
2. **Draft updated transcription guidance** for overlap-heavy corpora:
   separate speaker lines, start-time ordered, no `&*` for backchannels.
   Both CLAN and our VS Code extension handle this correctly.
3. **Evaluate `&*` → separate-utterance migration tool** feasibility for
   existing corpora.
4. **Update the per-speaker UTR proposal** — the need for it is reduced
   both by aligner improvements and by the natural encoding making the
   crossing-alignment problem go away.
5. **Re-run alignment on Davida's problem files** with the current system
   to see if the issues persist.
