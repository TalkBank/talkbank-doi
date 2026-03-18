# Corrected Experiment Results (Intra-Word Fix)

**Status:** Current
**Last updated:** 2026-03-17

Results rerun after fixing intra-word overlap marker extraction (`butt⌈er⌉`,
`a⌈nd`, etc.) and adding index-aware region pairing.

## Overlap Audit (Experiment B)

| Metric | Before (buggy) | After (fixed) | Change |
|--------|---------------|---------------|--------|
| Files with markers | 366 | **456** | **+90** |
| Fully paired | 135 (37%) | **297 (65%)** | **+162, +28pp** |
| Mixed | 214 (58%) | 116 (25%) | -98 |
| Open only | 17 (5%) | 43 (9%) | +26 |
| Cross-speaker pairs | 12,081 | **28,947** | **+16,866** |
| Timed pairs | 2,451 | **3,890** | **+1,439** |
| Temporal consistency | 99% | **99%** | Same |

The intra-word fix nearly doubled detected cross-speaker pairs and moved many
files from "mixed" to "fully paired" — the markers were there all along, just
inside words where the old code couldn't see them.

## Onset Accuracy (Experiment A validation, SBCSAE)

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Measurements | 2,421 | **3,160** | **+739** |
| Median error | 301ms | **282ms** | **-19ms** |
| Within ±500ms | 66.3% | **68.6%** | **+2.3pp** |
| Within ±1s | 85.7% | **87.2%** | **+1.5pp** |
| Within ±2s | 96.3% | **96.5%** | +0.2pp |

More measurements found, slightly better accuracy — intra-word markers give
more precise word positions than space-separated ones.

## End-to-End Timing (SBCSAE, 10 files)

Essentially unchanged — SBCSAE files mostly use space-separated markers.

| Metric | Global | Two-Pass |
|--------|--------|----------|
| Recovered timed | 7,776 (73.8%) | **8,131 (77.1%)** |
| ⌊ Recovered | 2,136 | **2,407** |
| ⌊ New coverage | 384 | **656** |
| Median start error | **137ms** | 150ms |
| ⌊ Median start error | **139ms** | 247ms |
| ⌊ Within 500ms | **83.6%** | 61.9% |

The coverage vs precision tradeoff is the same as before. Two-pass recovers
+272 more overlap timings at the cost of slightly noisier timing.

## Conclusion

The intra-word fix was important for **corpus analysis** (almost doubled
detected pairs) but had minimal impact on **alignment quality** for the
SBCSAE test set. Files with heavy intra-word markers (Jefferson NB,
TaiwanHakka, Bergmann) would see larger alignment improvements — these
should be tested next.
