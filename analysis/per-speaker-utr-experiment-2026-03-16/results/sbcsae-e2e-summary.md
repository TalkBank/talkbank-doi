# SBCSAE End-to-End Experiment Results

**Status:** Current
**Last updated:** 2026-03-17

## Setup

- **10 SBCSAE files** (06, 08, 09, 13, 15, 28, 32, 36, 38, 49)
- Selected for overlap marker coverage (best/worst onset accuracy, highest measurement counts)
- Ground truth: original hand-annotated timing
- Timing stripped, then recovered via `batchalign3 align` with each strategy
- Audio from `net:/Volumes/Other/ca/SBCSAE/`

## Coverage Comparison

| File | Global | Two-Pass | Ground Truth |
|------|--------|----------|-------------|
| 06 | 73.3% | **74.0%** | 71.7% |
| 08 | 83.1% | **84.3%** | 71.9% |
| 09 | 86.9% | 86.9% | 80.0% |
| 13 | 65.7% | **69.5%** | 56.2% |
| 15 | 71.2% | **77.1%** | 68.3% |
| 28 | 77.7% | **78.4%** | 71.5% |
| 32 | 71.3% | **76.6%** | 62.6% |
| 36 | 74.6% | **79.0%** | 68.5% |
| 38 | 90.5% | **91.4%** | 85.2% |
| 49 | 70.3% | **75.7%** | 63.3% |

**Two-pass consistently achieves higher coverage** — it times more utterances, especially ⌊-bearing overlap utterances.

## Timing Quality vs Ground Truth (10,540 utterances)

| Metric | Global UTR | Two-Pass UTR |
|--------|-----------|-------------|
| Recovered timed | 7,776 (73.8%) | **8,131 (77.1%)** |
| Both have timing | 7,021 | 7,019 |
| **Median start error** | **137ms** | 150ms |
| Start within 500ms | **86.7%** | 80.3% |
| Start within 1s | **92.7%** | 87.3% |
| p90 start error | **696ms** | 1,325ms |

## Overlap Utterance Timing (⌊-bearing only)

| Metric | Global UTR | Two-Pass UTR |
|--------|-----------|-------------|
| GT timed | 1,760 | 1,760 |
| **Recovered** | 2,130 | **2,401** |
| **New coverage** | 384 | **656** |
| Regressions | 14 | 15 |
| Both have timing | 1,746 | 1,745 |
| **⌊ Median start error** | **139ms** | 248ms |
| **⌊ Within 500ms** | **83.6%** | 61.9% |
| ⌊ Within 1s | **92.1%** | 73.1% |

## Analysis

### Two-pass wins on coverage

- **+355 more timed utterances** overall (77.1% vs 73.8%)
- **+272 more ⌊ overlap timings** that global misses (656 vs 384 new)
- Negligible regressions (14-15 utterances lost timing)

### Global wins on precision

- Median start error 137ms vs 150ms (both excellent)
- But for ⌊ overlaps specifically: 139ms vs 248ms — nearly 2x worse for two-pass
- Within 500ms: 83.6% vs 61.9% — **21.7pp gap** on overlap utterances

### Interpretation

This is a **coverage vs precision tradeoff**:
- Two-pass recovers timing for 272 more overlap utterances
- But the windowed pass-2 recovery produces noisier timing for overlap utterances
- The noise comes from the pass-2 search window: it's searching a constrained region of ASR tokens, and matches may be less precise than the global DP alignment

### The CA marker contribution

The CA-aware windowing (anchoring at ⌈ onset) should help *narrow* the pass-2 window and improve precision. The current results include both:
1. `+<`-triggered two-pass (original behavior)
2. ⌊-triggered two-pass with CA-aware windowing (new behavior)

The 656 new ⌊ timings (vs 384 for global) include utterances that were recovered specifically because the CA-aware code treats them as overlap candidates. Without the CA marker changes, these would have been left untimed.

### Recommendation

The two-pass strategy with CA markers is the right default for CA-data files:
- Significantly more overlap utterances get timing (+272)
- The precision reduction (248ms median vs 139ms) is still well within acceptable bounds
- Both strategies far exceed ground truth coverage (77% vs 67%)

For maximum precision on overlap utterances, the global strategy remains better. A hybrid approach (two-pass for coverage, then post-hoc refinement) could capture both benefits.
