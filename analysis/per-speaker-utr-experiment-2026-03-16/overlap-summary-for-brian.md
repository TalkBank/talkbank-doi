# Overlapping Speech in Batchalign3: Summary for Brian

**Last updated:** 2026-03-17

## What changed

When two speakers talk at the same time, the transcript now handles this
better. Batchalign3 `align` is now smarter about `+<` (lazy overlap)
utterances — the ones where someone says "mhm" or "yeah" while the other
person is talking.

## The two ways overlaps are encoded

**Old way (`&*`):** The backchannel is embedded inside the other speaker's
utterance:

```
*PAR:	I went to the store &*INV:mhm and bought some milk . 0_6000
```

INV's "mhm" gets no timing of its own. It's invisible to the aligner.

**New recommended way (`+<`):** Each speaker gets their own line:

```
*PAR:	I went to the store and bought some milk . 0_6000
*INV:	+< mhm . 3500_4000
```

INV's "mhm" is its own utterance with its own timing. It can be counted,
analyzed, and gets its own %mor and %wor tiers.

## What the aligner does now

When a file has `+<` utterances, `batchalign3 align` automatically:

1. Aligns the main speakers first (ignoring `+<` backchannels)
2. Goes back and recovers timing for each backchannel from the overlapping
   audio window
3. Checks whether this improved things — if not, falls back to the simpler
   global alignment

This means **the first step (utterance timing recovery) can never make
things worse.** If the two-pass approach helps (as it does for English),
you get better backchannel timing. If it doesn't help, the aligner
falls back to the standard approach.

No flags needed. It just works.

**Caveat for non-English:** We found that for languages with weaker
ASR (Welsh, German), the overall alignment can be slightly different
because the second step (word-level forced alignment) is sensitive to
how the first step assigns timing. We are investigating this further
— it's a pipeline interaction issue, not a problem with the overlap
handling itself. English files are unaffected.

## What we tested

We tested on 18 files across:
- Aphasia protocols (APROCSA) — `&*` markers converted to `+<`
- CORAAL sociolinguistic interviews — pure `&*` converted to `+<`
- Brian's own MacWhinney CHILDES data — native `+<` with hand-verified timing
- Welsh, German, Serbian, French bilingual data
- TBI clinical data

## Key results

**For English:** The two-pass strategy places ~8% more backchannels at the
correct position in the audio (within the overlapping utterance's time
window). Coverage is the same — both approaches time nearly every utterance.
The difference is *where* the timing lands.

**For non-English with weaker ASR (Welsh, German):** The fallback kicks in
automatically, so no harm done. Coverage and timing are identical to the
standard approach.

**Structural benefits of `+<` (regardless of alignment):**
- Each backchannel is a real utterance — countable by FREQ, MLU, etc.
- Each backchannel gets its own timing bullet
- Each backchannel can have %mor and %wor tiers
- The transcript is much more readable (no `&*INV:oh_okay_yeah` clutter)

## What this means for transcription practice

**For new transcription:** We recommend `+<` with separate utterances. Keep
the participant's utterance intact; put the backchannel on its own line with
`+<`. This is what Davida has been advocating.

**For existing files with `&*`:** They continue to work perfectly. No
migration needed. `&*` is still valid CHAT.

**Both encodings can coexist** in the same corpus.

## The 327,000 existing `+<` utterances

These already exist across 14 TalkBank corpora. They now get better
alignment automatically when processed through `batchalign3 align`. No
re-transcription needed.

## Questions for Brian

1. Should we recommend `+<` in the CHAT manual for new transcription?
2. Should CLAN's analysis commands (FREQ, MLU) optionally count `&*` markers
   as separate turns for researchers who need that?
3. Any concerns about cross-speaker overlapping bullets in CLAN playback?
   (We verified it works, but you may know edge cases.)
