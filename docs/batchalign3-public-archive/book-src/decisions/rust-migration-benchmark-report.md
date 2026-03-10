# Cross-Branch Benchmark Report: Rust Migration Performance

**Author:** Franklin Chen
**Date:** 2026-02-13
**Branches:** `align` (Rust migration) vs `bench-baseline` (master + instrumentation)
**Machine:** Apple M4 Max, 64 GB RAM, macOS Darwin 25.3.0
**Methodology:** 72 configs (2 branches x 3 commands x 4 datasets x 3 worker counts), 3 runs each, cache bypassed (`--override-cache`)

## Executive Summary

The Rust migration delivers **9-15x speedup for forced alignment**, **7-28x for utterance segmentation**, and **1.6-3.4x for morphosyntactic analysis**. The old Python forced alignment engine cannot finish long transcripts within 30 minutes; the Rust engine completes them in ~160 seconds. CLI startup improved from 1.3-1.9s to 0.05s via lazy imports.

## Forced Alignment (`align`)

| Dataset | Files | Workers | Rust (s) | Python (s) | Speedup |
|---------|------:|--------:|---------:|-----------:|--------:|
| align_small | 3 | 1 | 6.1 | 54.3 | **9.0x** |
|  |  | 2 | 5.5 | 59.0 | **10.7x** |
|  |  | 4 | 6.4 | 53.3 | **8.3x** |
| align_medium | 6 | 1 | 10.2 | 150.6 | **14.7x** |
|  |  | 2 | 10.3 | 112.0 | **10.8x** |
|  |  | 4 | 12.0 | 145.8 | **12.1x** |
| align_large | 18 | 1 | 20.1 | 311.2 | **15.5x** |
|  |  | 2 | 20.9 | 322.2 | **15.4x** |
|  |  | 4 | 24.3 | 302.3 | **12.5x** |
| align_long | 3 | 1 | 160.5 | TIMEOUT | **>11x** |
|  |  | 2 | 154.7 | TIMEOUT | **>11x** |
|  |  | 4 | 157.5 | TIMEOUT | **>11x** |

The baseline timed out (>1800s) on all `align_long` configurations. Extrapolating from the `align_large` ratio (15.5x), the baseline would need ~2500s per run for `align_long`.

**Worker scaling:** More workers **hurt** alignment performance (GPU-bound inference serializes on MPS). Optimal: **w=1**.

**Engine note:** Forced alignment uses a local Whisper model (`openai/whisper-large-v2`) for alignment only -- it matches existing transcript text to audio via cross-attention, not transcription. This is distinct from ASR, which uses Rev.AI (cloud API) for transcription. The Rust speedup comes from Rust-native utterance grouping, DP alignment, and timing correction; the Whisper model call itself is unchanged between branches.

## Morphosyntactic Analysis (`morphotag`)

| Dataset | Files | Workers | Rust (s) | Python (s) | Speedup |
|---------|------:|--------:|---------:|-----------:|--------:|
| align_small | 3 | 1 | 5.3 | 10.3 | **1.9x** |
|  |  | 2 | 5.0 | 10.0 | **2.0x** |
|  |  | 4 | 5.0 | 9.7 | **1.9x** |
| align_medium | 6 | 1 | 6.9 | 12.0 | **1.7x** |
|  |  | 2 | 5.7 | 11.3 | **2.0x** |
|  |  | 4 | 5.7 | 11.2 | **2.0x** |
| align_large | 18 | 1 | 10.2 | 16.7 | **1.6x** |
|  |  | 2 | 7.8 | 15.0 | **1.9x** |
|  |  | 4 | 7.0 | 13.7 | **2.0x** |
| align_long | 3 | 1 | 23.3 | 65.2 | **2.8x** |
|  |  | 2 | 15.8 | 52.2 | **3.3x** |
|  |  | 4 | 14.1 | 47.6 | **3.4x** |

Morphotag speedup is more modest (1.6-3.4x) because most time is spent in Stanza ML inference, not CHAT manipulation. The Rust batched callback reduces Python-to-Rust overhead. Speedup grows with dataset size as batch efficiency improves.

**Worker scaling:** w=4 helps on larger datasets (1.5-1.7x over w=1 on align branch).

## Utterance Segmentation (`utseg`)

| Dataset | Files | Workers | Rust (s) | Python (s) | Speedup |
|---------|------:|--------:|---------:|-----------:|--------:|
| align_small | 3 | 1 | 1.2 | 10.7 | **8.8x** |
|  |  | 2 | 1.3 | 11.0 | **8.5x** |
|  |  | 4 | 1.4 | 8.4 | **5.9x** |
| align_medium | 6 | 1 | 1.3 | 25.1 | **18.8x** |
|  |  | 2 | 1.4 | 19.8 | **14.6x** |
|  |  | 4 | 1.5 | 11.8 | **7.7x** |
| align_large | 18 | 1 | 1.6 | 45.1 | **28.2x** |
|  |  | 2 | 1.5 | 31.4 | **21.2x** |
|  |  | 4 | 1.6 | 20.9 | **13.1x** |
| align_long | 3 | 1 | 4.5 | 30.2 | **6.7x** |
|  |  | 2 | 3.1 | 11.1 | **3.6x** |
|  |  | 4 | 3.0 | 8.7 | **2.9x** |

Utterance segmentation shows the largest speedup range (2.9-28.2x). The Rust branch is so fast on small/medium datasets (1.2-1.6s) that worker overhead actually hurts. On the baseline, worker scaling helps significantly (up to 3.5x from w=1 to w=4).

## CLI Startup Time

| Branch | Startup (s) |
|--------|------------:|
| align (Rust) | 0.05 |
| bench-baseline | 1.3-1.9 |

The ~25x startup improvement comes from lazy imports -- heavy dependencies (torch, stanza, whisper) are only loaded when a pipeline is actually constructed.

## Correctness Comparison

### Forced Alignment -- Expected differences
The Rust branch produces proper word-level timing bullets (`%wor` tiers with millisecond timestamps) and inline utterance timing. The baseline produces bare utterances with no `%wor` tier and no timing. This is correct -- the Rust FA engine is working as designed.

### Morphosyntax -- Rust retokenization bug
Two categories of differences:

**Expected (Stanza version):** Different morphological analysis of colloquial forms (`gonna`, `gotta`), and `%tim` tier ordering. These are Stanza version differences and not regressions.

**Bug -- `add_morphosyntax_batched` corrupts special CHAT tokens during retokenization.** Non-NLP words (`xxx`, `&~uh`, `[//]` retrace groups) are dropped or shifted, causing adjacent words to duplicate. Every instance follows the same pattern:

| Original Input | Rust morphotag Output | Bug |
|---|---|---|
| `&~uh popcorn !` | `popcorn popcorn !` | `&~uh` dropped, next word duplicated |
| `okay , xxx I'll just put it here .` | `okay , I'll just put it here here .` | `xxx` dropped, last word duplicated |
| `roll it [//] roll it [//] roll it [//] roll it ?` | `roll roll [//] roll roll [//] it it [//] roll it ?` | retrace groups shifted |
| `xxx put her in the highchair ?` | `put her in the highchair highchair ?` | `xxx` dropped, last word duplicated |
| `oh look you got a fun bus xxx take my shoes off .` | `oh look you got a fun bus take my shoes off off .` | `xxx` dropped, last word duplicated |
| `xxx byebye .` | `byebye byebye .` | `xxx` dropped, word duplicated |

7 affected utterances found across 18 files (align_large). Root cause: the Rust batched callback's word-mapping alignment loses sync when non-NLP tokens are removed during retokenization.

### Utterance Segmentation -- Baseline is buggy, Rust is correct
The **baseline** Python utseg has severe correctness issues:
- Drops entire utterances silently (~25 missing across test files)
- Removes participants from `@Participants` and `@ID` headers (e.g., CHI removed entirely)
- Corrupts main-tier text (`what's in there ?` to `there in there .`, `gotta start it` to `one start it`)
- Changes question marks to periods on many utterances

The Rust branch preserves all utterances, participants, and punctuation correctly. This is a correctness **improvement** over master.

## Bugs Found During Benchmarking

### 1. Rust u64 subtraction overflow in FA grouping (FIXED)
- **Location:** `batchalign_core/src/forced_alignment.rs:90`
- **Trigger:** Out-of-order timing bullets where `utt_end < seg_start`
- **Symptom:** `PanicException: attempt to subtract with overflow` when running `bench align` on align_large (18 files) or align_long (3 files). Only manifests without UTR (which the bench command skips).
- **Fix:** Added bounds check `utt_end <= seg_start` before the subtraction. Regression test added (`test_group_utterances_backwards_bullets`).
- **Status:** Fixed, not yet committed to talkbank-utils.

### 2. Morphotag retokenization drops non-NLP words (OPEN)
- **Location:** `batchalign_core` `add_morphosyntax_batched` or Python batch callback `_stanza_batch_callback.py`
- **Trigger:** CHAT utterances containing `xxx` (unintelligible), `&~uh` (filled pause), or `[//]` retrace groups
- **Symptom:** Non-NLP tokens silently dropped, adjacent words duplicated to fill the gap
- **Impact:** 7 affected utterances across 18-file test set. Production morphotag output has corrupted main-tier text.
- **Status:** Open -- needs investigation.

## Summary

| Command | Avg Speedup (w=1) | Range | Notes |
|---------|:-----------------:|:-----:|-------|
| Forced Alignment | **13.1x** | 9.0-15.5x | 3 datasets compared; baseline TIMEOUT on align_long |
| Morphosyntax | **2.0x** | 1.6-2.8x | 4 datasets compared |
| Utt. Segmentation | **15.6x** | 6.7-28.2x | 4 datasets compared |

## Known Issues

1. **Baseline align_long timeout:** The old Python FA engine cannot complete 3 long transcript files within 30 minutes. The benchmark used `--timeout 1800s`. These 3 configs (w=1,2,4) are recorded as failures.
2. **Subtraction overflow (fixed):** A `u64` subtraction overflow in `forced_alignment.rs:90` caused panics on files with out-of-order timing bullets. Fixed by adding a bounds check (`utt_end <= seg_start`). Regression test added.
3. **No RSS data:** The `MemoryWatchdog` didn't capture peak RSS (processes exited before measurement). Per-engine RSS deltas are available in the run logs.

## Raw Data

```
Results directory: ~/batchalign-benchmarking/results/full_comparison/
Combined JSONL:   ~/batchalign-benchmarking/results/full_comparison/combined.jsonl
Correctness diffs: ~/batchalign-benchmarking/results/full_comparison/comparison/
Run logs:         ~/.batchalign-next/logs/
```

To regenerate analysis:
```bash
python scripts/compare_results.py results/full_comparison/combined.jsonl
python scripts/throughput_report.py results/full_comparison/combined.jsonl --data-dir data
```
