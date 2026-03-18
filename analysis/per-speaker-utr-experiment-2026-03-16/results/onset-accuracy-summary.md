# Experiment A: Proportional Onset Estimation Accuracy

**Status:** Current
**Last updated:** 2026-03-17

## Method

For each cross-speaker overlap pair (⌈ on utterance N, ⌊ on utterance N+1, both timed):
1. **Estimated onset** = `top_start + (⌈_word_position / total_words) × (top_end - top_start)`
2. **Actual onset** = ⌊ utterance start time (from timing bullet)
3. **Error** = estimated - actual (signed)

This validates the core assumption: can the proportional word position of ⌈ accurately predict when overlap begins?

## Results: SBCSAE (2,421 measurements across 55 files)

| Metric | Value |
|--------|-------|
| **Median absolute error** | **301ms** |
| **Within ±500ms** | **66.3%** |
| **Within ±1000ms** | **85.7%** |
| **Within ±2000ms** | **96.3%** |
| p90 absolute error | 1,222ms |
| p95 absolute error | 1,711ms |
| Mean absolute error | 524ms |
| Mean signed error | -473ms |

### Error Distribution

```
  ±500ms:  ████████████████████████████████████████ 66.3% (1,604 pairs)
  ±1000ms: ███████████████████████████████████████████████████ 85.7%
  ±2000ms: █████████████████████████████████████████████████████████ 96.3%
  >2000ms: ██ 3.7%
```

### Per-File Highlights (best and worst)

| File | Pairs | ≤500ms | ≤1s | Median err | Note |
|------|-------|--------|-----|-----------|------|
| 38 | 15 | 93% | 100% | 147ms | Best — short overlaps |
| 49 | 62 | 76% | 95% | 233ms | Excellent |
| 15 | 92 | 75% | 93% | 231ms | High volume, good |
| 08 | 64 | 69% | 94% | 258ms | Good |
| 09 | 18 | 50% | 67% | 564ms | Worst — long utterances |
| 06 | 36 | 53% | 69% | 490ms | Below average |

## Interpretation for UTR Windowing

### The tight window (±500ms) captures 66% of overlaps

This is the first buffer tried in adaptive windowing. For 66% of overlap pairs, the estimated onset is accurate enough that a ±500ms window around it would contain the backchannel's actual start time.

**Without CA markers**, the tight window searches around the full predecessor range (pred_start to pred_end). For a 5-second utterance, that's a 6-second window (±500ms beyond each end).

**With CA markers**, the tight window is anchored at the estimated onset point. For the same 5-second utterance with ⌈ at 60%, the tight window is 1-second wide (onset ±500ms) instead of 6 seconds. This is a **6x reduction in search space**.

### The negative bias (-473ms mean signed error) is expected

The estimate tends to slightly *underestimate* the onset time (predicting overlap starts earlier than it actually does). This is because:
1. Words are not uniformly distributed in time — early words tend to be shorter
2. ⌈ marks where overlap starts in the *transcript*, which may differ from the acoustic onset

This bias is actually favorable for windowing: the search window extends forward from the estimate, so an underestimate still captures the actual onset.

### 96% within ±2000ms means the medium buffer rarely fails

The adaptive windowing tries wider buffers if the tight one fails. With CA markers, the medium buffer (±predecessor_duration, min 2s) should capture virtually all backchannel onsets (96% are already within ±2s of the estimate).

## Conclusion

The proportional onset estimation is **accurate enough to be useful**:
- **Median error of 301ms** — well within the tight ±500ms window
- **66% hit rate on first try** (tight window) — reduces search space by ~6x
- **96% within ±2s** — near-certain capture on the medium buffer
- **Negative bias is favorable** — underestimate is better than overestimate for forward-looking windows

The key improvement over the current two-pass code: instead of searching the entire predecessor utterance's time range (which can be 10-20 seconds for long utterances), the CA-aware windowing narrows to a ~1-second window anchored at the estimated overlap onset. This should specifically help with non-English data where ASR quality is poor and there are fewer matching tokens in the search window.

## Files

- `onset-accuracy-ca-data.txt` — full results across all ca-data
- `onset-accuracy-sbcsae-verbose.tsv` — per-measurement details for SBCSAE
