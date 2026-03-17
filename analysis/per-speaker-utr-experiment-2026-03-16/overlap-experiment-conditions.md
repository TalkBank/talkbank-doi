# Overlap Alignment Experiment: Conditions and Test Data

**Status:** Draft
**Last updated:** 2026-03-17

## Research Question

Does the two-pass `+<`-aware UTR strategy improve alignment coverage and
timing quality for overlapping speech, compared to the current global DP
approach?

## Experimental Conditions

### APROCSA test set (4 files with `&*` markers)

These files originally use `&*` to embed backchannels. We convert them to
separate utterances in different ways to test each condition.

| Condition | CHAT input | UTR strategy | What it tests |
|-----------|-----------|-------------|---------------|
| **A** — Original `&*` | `&*` markers intact | Auto (GlobalUtr) | **Current production.** Backchannels invisible to DP, get no timing. Baseline. |
| **B** — No linker, global | `&*` → separate utterances, **no `+<`** | Auto (GlobalUtr) | **The original failure.** Backchannels as plain utterances in the global DP at wrong temporal positions. No signal to the aligner that they overlap. This is what people tried before giving up and using `&*`. |
| **C** — With `+<`, global | `&*` → separate utterances **with `+<`** | Global (forced) | **`+<` present but ignored.** Shows the DP penalty when backchannel words are in the global reference. The `+<` linker exists but GlobalUtr doesn't use it. |
| **D** — With `+<`, two-pass | `&*` → separate utterances **with `+<`** | TwoPass | **The improvement.** Pass 1 excludes `+<` utterances, pass 2 recovers their timing from the predecessor's audio window. |

**Expected results:**
- A: High main-speaker coverage, zero backchannel timing
- B: Possibly degraded main-speaker coverage (repeated "mhm"/"yeah" tokens
  in DP cause ambiguity), backchannels may get wrong timing or none
- C: Similar to B (GlobalUtr doesn't use `+<` signal)
- D: High main-speaker coverage (same as A) PLUS backchannel timing

**Key comparison:** B vs D shows the full improvement. C vs D isolates the
strategy effect (same CHAT input, different algorithm). A vs D shows what
`+<` encoding gains over `&*`.

### CHILDES ground-truth test set (existing `+<` files)

These files natively use `+<` and already have timing. We strip timing,
re-align, and compare against the original bullets as ground truth.

| Condition | Strategy | What it tests |
|-----------|----------|---------------|
| **Ground truth** | (original timing) | Reference — what the transcriber intended |
| **Global** | GlobalUtr (forced) | How well the global DP recovers `+<` backchannel timing |
| **Two-pass** | TwoPassOverlapUtr | Whether two-pass gives better backchannel timing |

## Test Files

### APROCSA (`&*` → `+<` conversion)

| File | Utts | `&*` markers | Speakers | Domain |
|------|------|-------------|----------|--------|
| 2265_T4 | 636 | 47 | 4 (INV, PAR, REL1, REL2) | Aphasia protocol |
| 2420_T3 | 912 | 74 | 3 (INV, PAR, REL) | Aphasia protocol |
| 2432_T2 | 952 | 78 | 2 (INV, PAR) | Aphasia protocol |
| 2463_T2 | 1070 | 86 | 2 (INV, PAR) | Aphasia protocol |

Audio: `/Volumes/Other/aphasia/English/Protocol/APROCSA/` on net.

### CHILDES — MacWhinney (ground-truth)

| File | Utts | `+<` count | Speakers | Domain |
|------|------|-----------|----------|--------|
| 060211a2 | 938 | 159 | 4 (FAT, MAR, MOT, CHI) | Family conversation |
| 060211b1 | 865 | 105 | 3 (CHI, FAT, MAR) | Family conversation |
| 060406b2 | 714 | 91 | 4 (CHI, FAT, MAR, MOT) | Family conversation |

Audio: `/Volumes/CHILDES/CHILDES/Eng-NA/MacWhinney/` on net.

### CHILDES — Snow (ground-truth)

| File | Utts | `+<` count | Timed | Speakers | Domain |
|------|------|-----------|-------|----------|--------|
| 020518b | 3347 | 131 | 69% | 5 (CHI, FAT, LIA, MOT, UNI) | Family conversation |

Audio: `/Volumes/CHILDES/CHILDES/Eng-NA/Snow/` on net.

### Additional corpora (pending)

| Corpus | File | `+<` | Bullets | Language | Domain |
|--------|------|------|---------|----------|--------|
| ca-data | Croatian/2015_44 | 191 | 480 | hrv, eng | Conversation analysis |
| dementia-data | English/Lanzi/Treatment/11-14-17 | 75 | 985 | eng | Dementia protocol |
| tbi-data | English/Coelho/N/n22 | 156 | 520 | eng | TBI protocol |
| tbi-data | English/Coelho/TB/tb23 | 151 | 619 | eng | TBI protocol |
| phon-data | (TBD) | (TBD) | (TBD) | (TBD) | Phonology |

## Metrics

### Coverage (primary)

Per speaker and overall:
- **Timed utterances:** count with bullet after alignment
- **Untimed utterances:** count without bullet
- **Coverage %:** timed / total

### Timing quality (secondary, ground-truth set only)

For `+<` utterances that received timing in both strategies:
- **Within-window rate:** Does the recovered bullet fall within or near the
  ground-truth bullet?
- **Endpoint tolerance:** Absolute difference in start/end times (ms)

### Main-speaker impact (critical safety check)

- Does condition B/C/D degrade main-speaker timing vs condition A?
- If yes: the backchannel words in the DP are harming the primary alignment.

## Running the Experiments

```bash
cd analysis/per-speaker-utr-experiment-2026-03-16/

# APROCSA experiment (4 conditions × 4 files)
bash scripts/run-overlap-experiment.sh

# CHILDES ground-truth experiment (2 conditions × 4 files)
bash scripts/run-groundtruth-experiment.sh

# Dry run (shows commands without executing)
bash scripts/run-overlap-experiment.sh --dry-run
bash scripts/run-groundtruth-experiment.sh --dry-run
```
