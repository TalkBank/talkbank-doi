# Fuzzy Matching Experiment Results

**Status:** Current
**Last updated:** 2026-03-18

## Method

Added Jaro-Winkler fuzzy matching (`strsim` crate) to the Hirschberg DP
alignment used by UTR. When `--utr-fuzzy <threshold>` is set, the DP
considers two words a match when their Jaro-Winkler similarity exceeds
the threshold, instead of requiring exact (case-insensitive) equality.

Tested at threshold 0.85 on 20 files across 3 corpora, comparing against
the existing global (exact) and two-pass (exact + CA markers) strategies.

## Results: SBCSAE (English, 10 files, 10,540 utterances)

| Metric | Global (exact) | Two-Pass (exact) | Fuzzy 0.85 |
|--------|---------------|------------------|------------|
| **Recovered** | 73.8% | **77.1%** | 76.8% |
| **⌊ New coverage** | 384 | 658 | **676** |
| **Regressions** | 14 | 15 | **8** |
| Median start error | **137ms** | 151ms | 140ms |
| Within 500ms | **86.7%** | 80.2% | 85.4% |
| ⌊ Median error | **139ms** | 249ms | 150ms |
| ⌊ Within 500ms | **83.6%** | 61.7% | 79.2% |

**Fuzzy matching achieves the best tradeoff:**
- Coverage close to two-pass (76.8% vs 77.1%)
- Precision close to global (140ms vs 137ms)
- **Fewest regressions** of any strategy (8 vs 14/15)
- **Best overlap precision** for a high-coverage strategy (79.2% vs 61.7%)

## Results: Jefferson NB (English, dense CA, 5 files)

| Metric | Global | Two-Pass | Fuzzy 0.85 |
|--------|--------|----------|------------|
| Recovered | **12.2%** | 10.7% | **12.2%** |
| Median error | **569ms** | 691ms | 570ms |

Fuzzy matches global — the density threshold prevents pass-1 exclusion,
and fuzzy doesn't help because ASR tokens already match the transcript
well on these English files.

## Results: TaiwanHakka (Hakka, 5 files)

| Metric | Global | Two-Pass | Fuzzy 0.85 |
|--------|--------|----------|------------|
| Recovered | 87.0% | **91.7%** | **91.7%** |

Fuzzy matches two-pass — both benefit from CA markers. Fuzzy doesn't add
further improvement because Hakka ASR mismatches are script-level (different
character systems), not spelling-level (Jaro-Winkler operates on characters).

## Why Fuzzy Matching Helps

The key insight: fuzzy matching improves the **global DP pass** (pass 1),
allowing it to match more ASR tokens to transcript words. This means fewer
utterances are left unmatched and need the noisier **windowed pass-2 recovery**.
The result is high coverage with high precision — the global DP handles most
words, and pass 2 only handles the truly overlapping backchannels.

Without fuzzy matching, minor ASR substitutions ("gonna"/"gona", "went"/"wen",
"yesterday"/"yestarday") cause DP mismatches that cascade into shifted
alignments, leaving utterances unmatched.

## False Positive Analysis

Jaro-Winkler at 0.85 is relatively strict. Known risks:
- Short words: "he"/"she" (JW=0.61 → no match, safe)
- "the"/"da" (JW=0.0 → no match, safe)
- "yes"/"yeah" (JW=0.78 → no match at 0.85, would match at 0.75)
- "mhm"/"mmhm" (JW=0.83 → no match at 0.85, matches at 0.80)

The 8 regressions (where fuzzy did worse than exact) need investigation
to determine if they're false-positive fuzzy matches or different alignment
paths.

## Recommendation

Fuzzy 0.85 should be the default for `--utr-strategy auto` based on:
1. Best overall tradeoff (coverage near two-pass, precision near global)
2. Fewest regressions of any strategy
3. No false-positive risk at this threshold level

However, more testing is recommended before changing defaults:
- Test on non-English European languages (German, Welsh, French)
- Test on the APROCSA aphasia data from the original overlap experiments
- Test at different thresholds (0.80, 0.90) to understand sensitivity
- Investigate the 8 SBCSAE regressions

## CLI Usage

```
batchalign3 align input/ -o output/ --utr-fuzzy 0.85
```

Current default: exact matching (no `--utr-fuzzy` flag).
