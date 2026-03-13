# Alignment Failure on Hand-Edited Transcripts: 2265_T4 Post-Mortem

## Summary

Running `batchalign3 align` on a heavily hand-edited CHAT file (`2265_T4.cha` from
APROCSA) produced catastrophic bullet loss: **232 of 636 utterances (36.5%) lost all
timing**. The failures occurred in large contiguous blocks, leaving entire sections
of the conversation without media bullets. This document reconstructs the problem,
identifies the root cause, and evaluates whether subsequent DP algorithm improvements
have addressed it.

**Verdict:** The current global-DP UTR design fixes the separate 407-style
token-starvation failure class, but it does not help for this class of problem.
The failure mode here is architectural — a monotonic aligner fundamentally
cannot handle transcripts whose text order diverges significantly from audio
temporal order, which is inherent in CHAT files with dense overlapping speech
(`&*` markers).

---

## Background

The file `2265_T4.cha` is a 25-minute APROCSA protocol recording of a participant
with aphasia (PAR), an investigator (INV), and two relatives (REL1, REL2). The
conversation has frequent short overlapping speech — backchannels, completions, and
interjections — especially from INV and the relatives.

The file was reported by Davida, who had reviewed the raw ASR output, corrected it
extensively, and then ran `align` on her reviewed version. The align command destroyed
timing on over a third of the utterances. The file was ultimately added to the
no-align exclusion list (`scripts/noalign_paths.txt`).

---

## Timeline

| Date | Event | Git ref |
|------|-------|---------|
| 2026-02-16 | Raw ASR output committed: 647 utterances, INV + PAR only | `bb50b0702` |
| 2026-02-19 | Initial entry/edits | `aed6665fa` |
| 2026-02-23 | Aphasia review began | `d9e93b756` |
| 2026-02-24 | Fully reviewed: 619 utterances, added REL1 + REL2, morphotag | `1245f2b31` |
| 2026-02-26 | UD conversion and further edits | `d81d0b293`, `b4255b2f6`, `1fc848a12` |
| ~2026-02-28 | Davida ran `align` → catastrophic bullet loss discovered | — |
| ~2026-02-28 | 2-speaker experiment: stripped REL1/REL2, re-ran align → same result | — |
| 2026-03-02 | File added to no-align exclusion list | `2c69cca05` |

---

## What Davida Changed in Her Review

The original ASR output (`bb50b0702`) was raw 2-speaker output from Rev.ai — run-on
text with poor utterance segmentation, no overlapping speech annotation, and all
speech attributed to only INV and PAR.

Davida's review made the following structural changes:

### 1. Added two new speakers

```
@Participants: INV Investigator, PAR Participant
```
became:
```
@Participants: INV Investigator, PAR Participant, REL1 Relative, REL2 Relative
```

REL1 had 10 utterances, REL2 had 10 utterances — short interjections and
backchannels from relatives present in the room.

### 2. Split merged ASR utterances

The ASR produced long run-on utterances merging multiple turns:

```
*INV:  rolling alright okay . 0_1765
```

Davida split these into proper individual utterances:

```
*INV:  rolling .
*INV:  alright .
*INV:  okay .
```

### 3. Added overlapping speech markers

Extremely frequent — approximately 30% of utterances contain `&*` markers:

```
*INV:  first part I'm gonna just ask you some questions &*PAR:okay .
*PAR:  but &-uh speaking , it seems to be okay &*INV:that's_good for the most part .
*PAR:  &*INV:so I [//] I'm hoping to play here &*INV:yeah in a month or
       two &*INV:definitely_possible_yeah if it's good weather .
```

This last example has **three** `&*` markers in a single utterance — three moments
where INV's words are textually interleaved into PAR's speech.

### 4. Corrected speaker attribution

Speech that the ASR attributed to the wrong speaker was reassigned. Turns that the
ASR merged across speakers were separated.

### 5. Added discourse annotations

`[/]`, `[//]`, `[+ exc]`, `@G:` section markers, proper `%mor` and `%gra` tiers
via morphotag.

---

## The Align Result: Catastrophic Bullet Loss

### Quantitative summary

| Metric | 4-speaker input | 2-speaker input |
|--------|----------------|-----------------|
| Total utterances | 636 | 636 |
| With bullets | 404 (63.5%) | 404 (63.5%) |
| **Without bullets** | **232 (36.5%)** | **232 (36.5%)** |

### Spatial distribution of untimed utterances

The untimed utterances are not distributed uniformly. They cluster in large
contiguous blocks where the aligner completely lost sync:

**Block 1 (lines 116–120):** 4 consecutive untimed utterances at the `@G: Stroke`
transition.

```
*INV:  so , sounds like you've had a great recovery . 153597_155844   ← last timed
*PAR:  I [/] &+mi I might have .                                     ← UNTIMED
*INV:  yeah .                                                         ← UNTIMED
@G:    Stroke
*INV:  <do you> [/] do you remember when you had your stroke ?        ← UNTIMED
*PAR:  kind of yeah .                                                 ← UNTIMED
*PAR:  kind of , sort of . 164460_164925                              ← timing resumes
```

Gap: 155844 → 164460 (~8.6 seconds of audio lost).

**Block 2 (lines 165–189):** 25 consecutive untimed utterances — the core of the
Recovery section where REL1 and REL2 participate most actively.

```
*INV:  mhm . 271134_274804                                           ← last timed
*REL2: &-um and brought you back for the seizure .                    ← UNTIMED
*REL1: at FirstnameT's house .                                       ← UNTIMED
*PAR:  and then I came back for a few weeks .                         ← UNTIMED
*INV:  I see .                                                        ← UNTIMED
*PAR:  and then stayed at his house for a month or two ...            ← UNTIMED
*INV:  gotcha .                                                       ← UNTIMED
*INV:  well , tell me a little bit about your recovery .              ← UNTIMED
*INV:  what kinds of things have you done to get better ?             ← UNTIMED
*PAR:  oh &-uh well , I've [/] I've got a [//] my little tablet ...  ← UNTIMED
*INV:  mhm . 322166_322326                                           ← timing resumes
  ⋮
```

Gap: 274804 → 322166 (~47 seconds of audio lost). This is the worst block — nearly
a full minute of conversation with no timing at all.

**Block 3 (lines 301–319):** 19 consecutive untimed utterances — the Important_Event
narrative where PAR tells stories and REL1 interjects.

```
*PAR:  &-uh +... 574522_575044                                       ← last timed
*INV:  it could be &*PAR:well a trip or +...                          ← UNTIMED
*PAR:  yeah I [/] I took my three [/] three kids on a cruise ...      ← UNTIMED
*PAR:  and where all did we go at ?                                   ← UNTIMED
*REL1: so , we were in Gran_Cayman .                                  ← UNTIMED
*PAR:  yeah my two boys and girl .                                    ← UNTIMED
  ⋮  (14 more untimed)
*PAR:  but &-uh &-uh we won a trip to &-uh Hawaii . 579785_586605    ← timing resumes
```

Gap: 575044 → 579785 (~5 seconds — but 19 utterances represent much more conversational
content than 5 seconds; the ASR likely compressed this section).

---

## The 2-Speaker Experiment

To test whether the new REL1/REL2 speakers were the cause, a variant was created
(`~/davida-more/2265_T4-input-2speakers.cha`) where all REL1 and REL2 utterances were
reassigned to INV:

| Version | INV | PAR | REL1 | REL2 | Total |
|---------|-----|-----|------|------|-------|
| 4-speaker | 313 | 303 | 10 | 10 | 636 |
| 2-speaker | 333 | 303 | — | — | 636 |

Result: **Identical** — 404 with bullets, 232 without, in the same locations.

**Conclusion:** The problem is not the extra speakers. It is the structural divergence
between the hand-edited transcript and the ASR output.

---

## Root Cause Analysis

### The UTR alignment problem

The `align` command runs UTR (Utterance Time Recovery) as its first step. UTR:

1. Runs fresh Whisper ASR on the audio → produces a new 2-speaker temporal transcript
2. Flattens the hand-edited transcript into a reference word sequence (text order)
3. Uses monotonic matching to transfer ASR timestamps to the reference words

The fresh ASR output looks structurally similar to the original raw ASR: long
run-on utterances, 2 speakers, no overlapping speech annotation, no `&*` markers.

The hand-edited transcript has been fundamentally restructured:
- Different utterance boundaries
- `&*` markers that interleave one speaker's words inside another speaker's utterance
- New turns that the ASR didn't segment or attributed to the wrong speaker

### Why monotonic matching fails

The `&*` overlapping speech markers are the critical factor. Consider:

```
*PAR:  &*INV:so I [//] I'm hoping to play here &*INV:yeah in a month or
       two &*INV:definitely_possible_yeah if it's good weather .
```

The words from PAR in this utterance, in text order, are:
`I I'm hoping to play here in a month or two if it's good weather`

But in the audio:
- INV says "so" first (overlapping with PAR's start)
- PAR says "I'm hoping to play here"
- INV says "yeah" (overlapping)
- PAR says "in a month or two"
- INV says "definitely possible yeah" (overlapping)
- PAR says "if it's good weather"

The ASR, not understanding the overlapping speech structure, produces something like:
`so I'm hoping to play here yeah in a month or two definitely possible yeah
if it's good weather`

The monotonic matcher tries to align these sequences, but the interleaved INV words
in the ASR (which aren't in the `&*`-stripped reference for this utterance) push the
alignment off track. The matcher may match "so" from the ASR to an earlier position,
preventing later words from matching their correct positions.

When this happens across many utterances in a dense overlapping region, the matcher
loses sync entirely. It can't find matches for a whole block of utterances, leaving
them all untimed.

### The cascading failure

Once the UTR step leaves a block of utterances untimed, the FA step tries to fill
in using proportional estimates. But:

1. Proportional estimates for a 25-utterance untimed block give each utterance a
   tiny window (~2 seconds each)
2. The FA model runs within each window and may find correct word-level timing
3. However, the monotonicity enforcement pass (`enforce_monotonicity()` in
   `add_forced_alignment_inner`) then checks whether these FA-assigned bullets are
   in order
4. If the proportional estimates placed utterances in temporal positions that don't
   match their text order (likely in overlapping-speech regions), the enforcement
   pass strips their timing
5. Result: the untimed block remains untimed

---

## Evaluation: Does the current UTR design help?

### What we changed

The current UTR design is different from the one this report originally analyzed:

1. **UTR now uses a single global Hirschberg DP alignment** across all document
   words and all ASR tokens.
2. **That global alignment fixes the separate token-starvation class** where a
   local/windowed matcher consumed tokens too early and left later utterances
   unmatched.
3. **FA remapping and retokenize still avoid broad runtime DP remaps** and stay
   deterministic where explicit structure is available.

### Why the current UTR design still does not help here

| Improvement | Relevance to 2265_T4 |
|------------|---------------------|
| `word_id` mapping | Word IDs are assigned during ASR. Davida's hand-editing created new utterances and reorganized words — the fresh ASR produces entirely new word IDs that don't correspond to the edited transcript's structure. No identity matches possible. |
| Window-constrained fallback | Requires existing bullets as anchors. The input file has no bullets (they were stripped during editing). Falls through to global fallback. |
| Global monotonic DP | Better than local matching for token-starvation, but still cannot handle word-order mismatches from `&*` interleaving. A monotonic full-document aligner still cannot represent crossing matches. |
| Monotonicity enforcement | Already active — it's part of why so many bullets are *missing*. The enforcement correctly strips bad bullets, but the effect is that the untimed blocks remain untimed. |

### The fundamental architectural limitation

**A monotonic aligner cannot handle transcripts where text order diverges from audio
temporal order.** This divergence is inherent in CHAT files with overlapping speech:

- `&*` markers textually embed one speaker's words inside another's utterance
- The ASR produces a flat temporal sequence without these interleaving markers
- Monotonic matching requires that if word A comes before word B in the reference,
  word A's ASR match must come before word B's ASR match
- When `&*`-interleaved words appear in the ASR between words that are in the same
  utterance in the reference, the monotonic constraint forces the matcher to either
  skip the interleaved words or lose sync

This is **Known DP Failure Mode #1** ("Crossing alignments / rapid overlaps") from
`dynamic-programming.md`. Restoring global DP addressed the separate
token-starvation failure mode, but it does not and cannot address this overlap
failure mode within a monotonic framework.

---

## Reproduction Steps

### Prerequisites

- Access to `~/talkbank/` workspace with `batchalign3` installed
- Access to `~/data/aphasia-data/` corpus with audio files
- Audio file `2265_T4.mp4` (or `.wav`) in the APROCSA media directory

### Test 1: Reproduce the 4-speaker failure

```bash
# Input is the hand-edited version without timestamps
cp ~/davida/2265_T4-input.cha /tmp/align-test/2265_T4.cha

# Run align (ensure audio is discoverable via media roots)
cd ~/talkbank/batchalign3
uv run batchalign3 align /tmp/align-test/ /tmp/align-output/

# Count untimed utterances
grep '^\*' /tmp/align-output/2265_T4.cha | grep -vc '[0-9]_[0-9]'
# Expected: ~232 (36.5% of 636 total)
```

### Test 2: Confirm 2-speaker variant produces identical results

```bash
cp ~/davida-more/2265_T4-input-2speakers.cha /tmp/align-test-2sp/2265_T4.cha
uv run batchalign3 align /tmp/align-test-2sp/ /tmp/align-output-2sp/

grep '^\*' /tmp/align-output-2sp/2265_T4.cha | grep -vc '[0-9]_[0-9]'
# Expected: ~232 (identical to 4-speaker)
```

### Test 3: Identify the untimed blocks

```bash
# Show all untimed main-tier lines with line numbers
grep -n '^\*' /tmp/align-output/2265_T4.cha | grep -v '[0-9]_[0-9]'

# Look for the three major blocks:
# Block 1: ~4 untimed around @G: Stroke
# Block 2: ~25 untimed in Recovery section (REL1/REL2 dense)
# Block 3: ~19 untimed in Important_Event narrative
```

---

## Test Data Locations

| File | Path | Description |
|------|------|-------------|
| 4-speaker input (no bullets) | `~/davida/2265_T4-input.cha` | Davida's hand-edited transcript, timestamps stripped |
| 4-speaker output | `~/davida/2265_T4-output.cha` | Result of running `align` on 4-speaker input |
| 2-speaker input (no bullets) | `~/davida-more/2265_T4-input-2speakers.cha` | REL1/REL2 → INV, timestamps stripped |
| 2-speaker output | `~/davida-more/2265_T4-output-2speakers.cha` | Result of running `align` on 2-speaker input |
| Original ASR | `git show bb50b0702:English/Protocol/APROCSA/2265_T4.cha` | Raw Rev.ai ASR output (647 utts, 2 speakers) |
| Fully reviewed | `git show 1245f2b31:English/Protocol/APROCSA/2265_T4.cha` | Davida's completed review (619 utts, 4 speakers) |
| No-align list | `~/talkbank/scripts/noalign_paths.txt` | Contains this file among 271 excluded paths |

---

## Potential Future Approaches

These are documented for future reference. None are currently implemented.

### 1. Per-speaker UTR

Run ASR separately per speaker channel (using diarization boundaries), then match each
speaker's ASR against only that speaker's utterances. This eliminates the crossing
alignment problem because each speaker's words are in temporal order within their own
stream. Requires reliable diarization data.

### 2. Non-monotonic local alignment

Allow crossing matches within small windows (e.g., ±5 seconds) while enforcing
monotonicity only at the utterance level. This would let `&*`-interleaved words match
out of order within a local region. Requires a custom alignment algorithm — standard
LCS/edit-distance DP enforces global monotonicity.

### 3. Backchannel-aware matching

Strip `&*` embedded speech from each utterance before matching, align the "backbone"
words, then interpolate backchannel timing from surrounding context. This is
architecturally simpler but requires the matcher to understand CHAT `&*` syntax.

### 4. Incremental alignment for pre-aligned files

If the input already has bullets (from a previous `align` run or from ASR), only
re-align utterances that were added or changed, preserving existing timing for
unmodified utterances. This would avoid the global re-alignment that destroys existing
good timing. Requires detecting which utterances changed vs. the previous version.

### 5. Accept non-monotonic bullets in the align output

In cases where the correct timestamps are non-monotonic due to text-order divergence,
the timestamps could be preserved as-is (suppressing E362 enforcement). This would
give correct per-utterance timing at the cost of violating CLAN's monotonicity
requirement. Would require downstream CLAN tools to handle non-monotonic bullets.

---

## Related Documents

- `align-monotonicity.md` — E362 enforcement mechanism and the validation gate
- `align-correctness-fixes.md` — Four independent bugs that produced wrong timestamps
- `dynamic-programming.md` (`batchalign3/book/src/reference/`) — DP inventory,
  failure modes, and redesign metrics
- `forced-alignment.md` (`batchalign3/book/src/reference/`) — FA pipeline architecture
- `noalign_paths.txt` (`scripts/`) — 271 files excluded from alignment
