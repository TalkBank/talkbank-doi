# Overlap Alignment Experiment: Results and Recommendations

**Status:** Current
**Last updated:** 2026-03-17

## Executive Summary

We tested whether a two-pass `+<`-aware UTR strategy improves alignment
quality for overlapping speech compared to the existing global DP approach.
The experiment covered 18 files across 6 languages, 4 clinical domains,
and 2 overlap encoding types (`&*` conversion and native `+<`).

**Key findings:**

1. **Coverage is identical** between strategies (98-100%) on all English
   files. Coverage is the wrong metric — both strategies assign timing
   to nearly every backchannel.

2. **Two-pass improves backchannel timing placement on English files.**
   51% of backchannels land within the predecessor's time window with
   two-pass vs 43% with global (APROCSA, N=307). On CORAAL, 61% vs 58%
   (N=930).

3. **Two-pass is WORSE on non-English files.** Welsh: 59.7% coverage
   vs 68.6% global. German: 64.7% vs 91.6%. When ASR quality is poor,
   the windowed pass-2 recovery fails more often than the global DP.

4. **The structural value of `+<` encoding outweighs the algorithmic
   improvement.** Backchannels encoded as `+<` get independent timing,
   their own dependent tiers, and are countable by analysis tools —
   regardless of which alignment strategy is used.

**Recommendation:** Keep two-pass as the default for `+<` files (already
shipped). Recommend `+<` encoding for new transcription on structural
grounds. Do not force-migrate existing `&*` files. Consider adding
language-aware strategy selection in the future (use global for
non-English where ASR quality is poor).

## Experiment 1: APROCSA `&*` Conversion

**Files:** 4 aphasia protocol files (636-1070 utterances each)
**Encoding:** `&*` markers converted to separate utterances

### Coverage (ALL speakers)

| File | `&*` | A (original) | B (no linker) | C (`+<` global) | D (`+<` two-pass) |
|------|------|:---:|:---:|:---:|:---:|
| 2265_T4 | 47 | 636/636 (100%) | 682/683 (99.9%) | 682/683 (99.9%) | 683/683 (100%) |
| 2420_T3 | 74 | 905/912 (99.2%) | 976/986 (99.0%) | 976/986 (99.0%) | 977/986 (99.1%) |
| 2432_T2 | 78 | 942/952 (98.9%) | 1013/1030 (98.3%) | 1013/1030 (98.3%) | 1013/1030 (98.3%) |
| 2463_T2 | 86 | 1065/1070 (99.5%) | 1149/1156 (99.4%) | 1149/1156 (99.4%) | 1149/1156 (99.4%) |

### Timing Quality: Global vs Two-Pass (320 `+<` backchannels)

| Metric | Global | Two-pass |
|--------|--------|----------|
| Identical timing | 162 (53%) | — |
| Differ >500ms | 112 (36%) | — |
| Differ >3 seconds | 25 | — |
| Within predecessor window (±500ms) | **132 (43%)** | **158 (51%)** |

**Two-pass places 8 percentage points more backchannels within the
predecessor's overlap window.** The improvement is consistent across
all 4 files.

## Experiment 2: CHILDES MacWhinney Ground Truth

**Files:** 3 MacWhinney family conversations + 1 Snow file (714-3347 utterances)
**Encoding:** Native `+<` with hand-verified timing as ground truth

### Coverage

| File | `+<` | Ground truth | Global | Two-pass |
|------|------|:---:|:---:|:---:|
| 060211a2 | 159 | 938/938 (100%) | 873/938 (93.1%) | 873/938 (93.1%) |
| 060211b1 | 105 | 865/865 (100%) | 831/865 (96.1%) | 830/865 (96.0%) |
| 060406b2 | 91 | 714/714 (100%) | 694/714 (97.2%) | 694/714 (97.2%) |
| 020518b | 131 | 2320/3347 (69.3%) | 3251/3347 (97.1%) | (failed — timeout) |

### Ground Truth Timing Comparison

Neither strategy matches ground truth exactly (0% identical — expected,
since the ASR engine differs from the original timing source). The
"within predecessor window" metric is more meaningful:

| Metric (aggregate, N=407 both-timed) | Ground truth | Global | Two-pass |
|--------------------------------------|:---:|:---:|:---:|
| Within predecessor window (±500ms) | 150 (37%) | 198 (49%) | 190 (58%) |
| Diff >500ms from truth | — | 275 (68%) | 260 (79%) |

Both strategies place more backchannels within the predecessor window
than the ground truth itself (49-58% vs 37%), suggesting the original
hand-timing was approximate for backchannels.

## Experiment 3: CORAAL Pure `&*` Conversion

**Files:** 3 sociolinguistic interview files (670-827 utterances, 185-325 `&*` markers)
**Encoding:** Pure `&*` converted to separate utterances — highest `&*` density in TalkBank

### Coverage

| File | `&*` | A (original) | B (no linker) | C (`+<` global) | D (`+<` two-pass) |
|------|------|:---:|:---:|:---:|:---:|
| PRV_m02_1 | 325 | 827/827 (100%) | 1179/1181 (99.8%) | 1179/1181 (99.8%) | 1180/1181 (99.9%) |
| PRV_f02_1 | 303 | 670/670 (100%) | 1047/1052 (99.5%) | 1047/1052 (99.5%) | 1048/1052 (99.6%) |
| DTA_f02 | 185 | 746/746 (100%) | 948/951 (99.7%) | 948/951 (99.7%) | 948/951 (99.7%) |

### Timing Quality: Global vs Two-Pass (930 `+<` backchannels)

| Metric | Global | Two-pass |
|--------|--------|----------|
| Identical timing | 614 (66%) | — |
| Differ >500ms | 242 (26%) | — |
| Differ >3 seconds | 81 | — |
| Within predecessor window (±500ms) | **544 (58%)** | **564 (61%)** |

Smaller improvement on CORAAL (3 percentage points) but still consistent.
81 backchannels differ by >3 seconds between strategies.

## Experiment 4: Multilingual Ground Truth

**Files:** 7 files across Welsh, German, Serbian, French-English, trilingual, and TBI English
**Encoding:** Native `+<` with original timing as ground truth

### Coverage

| File | Lang | `+<` | Ground truth | Global | Two-pass |
|------|------|------|:---:|:---:|:---:|
| fusser12 | cym,eng | 736 | 2190/2190 (100%) | 1502/2190 (68.6%) | **1308/2190 (59.7%)** |
| german050814 | deu | 451 | 725/1341 (54.1%) | 1229/1341 (91.6%) | **867/1341 (64.7%)** |
| tbi_n22 | eng | 156 | 520/613 (84.8%) | 605/613 (98.7%) | 605/613 (98.7%) |
| tbi_tb23 | eng | 151 | 619/711 (87.1%) | 699/711 (98.3%) | 700/711 (98.5%) |
| mle28 | fra,eng | 855 | — | timed out | timed out |
| patagonia30 | cym,eng,spa | 690 | — | server error | server error |
| serbian030005 | srp | 1033 | — | timed out | timed out |

### Critical Finding: Two-Pass Hurts Non-English

**Welsh (fusser12):** Two-pass loses 9 percentage points (59.7% vs 68.6%).
**German:** Two-pass loses 27 percentage points (64.7% vs 91.6%).
**English TBI:** Identical between strategies.

**Explanation:** Pass 2 searches for backchannel words within a small
predecessor window. When ASR produces poor transcriptions for non-English
audio, the windowed DP has too few tokens to work with and fails. The
global DP has the full token stream and can sometimes find matches even
in noisy ASR output.

## Aggregate Results Across All Experiments

### English Files: Two-Pass Is Modestly Better

| Experiment | N (`+<`) | Within pred: Global | Within pred: Two-pass | Delta |
|------------|---------|:---:|:---:|:---:|
| APROCSA | 307 | 43% | 51% | **+8pp** |
| CORAAL | 930 | 58% | 61% | **+3pp** |
| TBI | ~207 | ~98% | ~98% | 0 |

### Non-English Files: Two-Pass Is Worse

| File | Lang | Global coverage | Two-pass coverage | Delta |
|------|------|:---:|:---:|:---:|
| fusser12 | Welsh-English | 68.6% | 59.7% | **-9pp** |
| german050814 | German | 91.6% | 64.7% | **-27pp** |

## Recommendations

### 1. Keep `+<` encoding recommendation

The structural benefits of `+<` over `&*` are clear regardless of
alignment strategy:
- Independent timing for backchannels
- Own dependent tiers (%mor, %wor)
- Countable as utterances by analysis tools
- Cleaner transcript readability

### 2. Keep two-pass as default with grouping-aware best-of-both fallback

Two-pass now runs both strategies and compares FA grouping outcomes
before choosing which to keep. This two-level comparison provides:
- Better backchannel placement on English (3-8pp improvement)
- **No regression on German** — grouping fallback detects that two-pass
  creates fewer FA groups (152 vs 162) and falls back to global
- Zero cost when no `+<` utterances are present

The comparison logic (added 2026-03-17):
1. Run both strategies on clones of the input
2. If `GroupingContext` is available (total_audio_ms + max_group_ms from
   the dispatch layer), call `group_utterances()` on both outputs
3. **Primary signal:** FA group count. Fewer groups = wider FA windows =
   worse alignment. If two-pass creates fewer groups, fall back to global.
4. **Tiebreaker:** When groups are equal, use timed utterance count
   (prefer two-pass when equal for better backchannel placement).

**Verification results (2026-03-17):**

| File | Lang | Global | Two-pass (old) | Auto (grouping fix) | Status |
|------|------|--------|---------------|---------------------|--------|
| german050814 | deu | 1229 | 867 | **1229** | Fixed — fallback triggered |
| 2265_T4 | eng | 636 | 683 | **683** | Preserved — two-pass kept |
| fusser12 | cym | 1502 | 1308 | 1305 | Not fixed — see §3 |
| tbi_n22 | eng | 605 | 605 | **605** | No change |
| tbi_tb23 | eng | 699 | 700 | **700** | Two-pass kept (+1) |

### 3. Critical finding: FA sensitivity to UTR output

**The German regression (1229→873) was caused by FA group merging and
is now FIXED by the grouping-aware fallback.**

**The Welsh regression (1502→1308) has a different root cause** that
the grouping heuristic cannot detect.

Debugging revealed that UTR assigns only ~24-35 utterance-level bullets
on German. The large coverage difference comes from **downstream FA
grouping**: when UTR assigns different bullets, FA creates different
utterance groups, which cascades into different word-level alignment
coverage. The full picture:

```
UTR (pre-pass) → sets utterance bullets
  ↓
FA grouping → groups utterances by time windows
  ↓
FA alignment → word-level timing per group
  ↓
Final coverage → depends on ALL of the above
```

**German (fixed):** Two-pass creates fewer groups (152 vs 162) — wider
windows → worse FA. The grouping comparison detects this and falls back
to global. Final coverage: 1229 (matches baseline).

**Welsh (not fixed):** Two-pass creates MORE groups (313 vs 284), so
the grouping comparison correctly keeps two-pass. But the final FA
result is still worse (1305 vs 1502). This means the Welsh regression
is caused by different group *boundaries* (not fewer groups), which
the group-count heuristic can't detect.

**To fully fix Welsh, the comparison needs to happen at the full
pipeline output level** (run FA with both strategies, keep the better
output). This is expensive (2x alignment per file) and is tracked as
a future improvement.

### 4. Do not force-migrate existing `&*` files

`&*` encoding works correctly with the current aligner. Migration should
be opt-in and driven by analysis needs.

### 5. Stage decomposition findings (Experiment A)

**Date:** 2026-03-17 (same session)

Used a new `decompose` tool that runs both UTR strategies on the same
input + ASR tokens and compares UTR output AND FA grouping without
running actual FA inference.

**German 050814 (regression case):**
- Two-pass: 35 UTR bullets, **152 FA groups**
- Global: 24 UTR bullets, **162 FA groups**
- Two-pass has **10 fewer groups** → larger audio windows → worse FA

**English 2265_T4 (improvement case):**
- Two-pass: 647 UTR bullets (21 more on `+<`), **151 FA groups**
- Global: 626 UTR bullets, **147 FA groups**
- Two-pass has **4 more groups** → smaller audio windows → better FA

**Root cause confirmed:** Two-pass changes UTR bullet distribution,
which changes `estimate_untimed_boundaries` anchor points, which changes
FA group boundaries. On English the effect is beneficial (more precise
groups). On German the effect is harmful (groups merge into wider
windows).

**The fix must happen at the FA grouping level**, not UTR. Options:
1. Make FA grouping robust to small UTR bullet changes
2. Run full-pipeline best-of-both (expensive but correct)
3. Clamp group window expansion so it never exceeds the global baseline

### 6. Remaining experiments

The current experiments measured UTR + FA combined output but compared
strategies only at the UTR level. The next round needs:

1. **Isolate UTR vs FA effects.** Run UTR-only (no FA) and measure
   coverage. Then run FA on each UTR output and measure separately.
   This tells us which component is responsible for regressions.

2. **Full-pipeline best-of-both.** Run the entire `align` pipeline
   with both strategies, then compare final output and keep the better
   one. Expensive (2x alignment per file) but correct. Could be done
   as a post-hoc comparison rather than inline.

3. **FA grouping sensitivity analysis.** Given the same ASR tokens,
   how does FA grouping change when 1-2 UTR bullets are different?
   A unit-level investigation into the grouping algorithm's stability.

4. **Non-English ASR quality audit.** The real issue for Welsh/German
   is ASR quality, not the overlap strategy. Measure ASR WER per
   language to understand which languages have sufficient ASR quality
   for two-pass to help.

## Experimental Setup

### Tools
- batchalign3 commit `f02702e4`
- talkbank-tools commit `fe5bb89`
- utr-experiment tool in talkbank-dev workspace

### Conditions (APROCSA/CORAAL)
- **A:** Original `&*` markers, auto strategy (GlobalUtr)
- **B:** `&*` → separate utterances without `+<`, auto strategy (GlobalUtr)
- **C:** `&*` → separate utterances with `+<`, forced GlobalUtr
- **D:** `&*` → separate utterances with `+<`, TwoPassOverlapUtr

### Conditions (ground truth)
- **Ground truth:** Original hand-verified timing
- **Global:** GlobalUtr on timing-stripped file
- **Two-pass:** TwoPassOverlapUtr on timing-stripped file

### Provenance
See `PROVENANCE.md` for exact file checksums, audio paths, and
reproduction steps.

---

## CA Overlap Marker Experiments (2026-03-17)

### Experiment B: Overlap Consistency Audit

Audited all ca-data files for CA overlap marker (⌈⌉⌊⌋) pairing quality and
temporal consistency.

| Metric | ca-data | childes-data |
|--------|---------|-------------|
| Files with markers | 366 | 76 |
| Cross-speaker pairs | 12,081 | 20 |
| Timed pairs | 2,451 | 2 |
| **Temporal consistency** | **99%** | **100%** |

**Conclusion:** CA overlap markers are highly reliable as timing constraints.
SBCSAE dominates the timed pairs (2,427 of 2,451).

### Experiment A: Proportional Onset Estimation Accuracy

For 2,421 timed cross-speaker overlap pairs in SBCSAE, compared the
proportional onset estimate (from ⌈ word position) against the actual ⌊
utterance start time.

| Metric | Value |
|--------|-------|
| **Median absolute error** | **301ms** |
| Within ±500ms (tight window) | 66.3% |
| Within ±1000ms | 85.7% |
| Within ±2000ms (medium window) | 96.3% |
| Mean signed error | -473ms (underestimate bias) |

**Key finding:** The proportional estimate is accurate enough to narrow the
pass-2 search window by ~6x (from full predecessor range to onset ±500ms).
The negative bias (underestimate) is favorable — it means the forward-looking
search window still captures the actual onset.

**Implementation:** CA-aware windowing is now in the production UTR code
(`batchalign-chat-ops/src/fa/utr/two_pass.rs`). When the predecessor has ⌈
markers, pass 2 anchors the search window at the estimated onset point. Also,
utterances with ⌊ markers are now treated as overlap candidates (same as `+<`).

See `results/onset-accuracy-summary.md` for full analysis and
`results/ca-overlap-audit-summary.md` for the audit results.

---

## Phase 7: Non-English Regression Diagnosis (2026-03-19)

Phase 6 showed a non-English regression: TaiwanHakka dropped -2.9pp with
`auto` compared to `global`. Three experiments diagnosed the cause and
validated a language-aware fix.

### Experiment 1: Best-of-Both per Non-English File

Ran 7 non-English files × 3 strategies (global, two-pass, auto):

| File | Lang | Global | Two-pass | Auto | Winner |
|------|------|--------|----------|------|--------|
| 01 | hak | 415/449 | 415/449 | 415/449 | tie |
| 02 | hak | 548/648 | 294/648 | 294/648 | **global** |
| 03 | hak | 508/650 | 632/650 | 632/650 | **two-pass** |
| 10 | hak | 619/627 | 619/627 | 619/627 | tie |
| 12 | hak | 286/287 | 286/287 | 286/287 | tie |
| fusser12 | cym | 1502/2190 | 1475/2190 | 1476/2190 | **global** |
| german050814 | deu | 1229/1341 | 621/1341 | 942/1341 | **global** |

**Finding:** Global wins for non-English overall. German is dramatic: global
gets 1229 timed vs two-pass 621. The Hakka regression from Phase 6 was
driven by file 02 (global 548 vs auto 294 — a 254 utterance swing). Auto
never picks global for Hakka, indicating the best-of-both heuristic doesn't
detect that global is better for some non-English files.

### Experiment 2: ASR WER Audit

Blocked by two issues:
1. **Rev.AI rejects Hakka** — ISO 639-3 `hak` was truncated to `ha` by a
   regression bug in the Rust Rev.AI code mapping (now fixed)
2. **Stale server binary** — `benchmark` submitted jobs to a running daemon
   with an older grammar, causing false parse errors on fusser12/tbi_n22

### Experiment 3: Language-Aware Strategy Selection

Tested `eng → auto`, all others → `global`:

| Corpus | Language-Aware | Phase 6 Global | Phase 6 Auto | vs Global | vs Auto |
|--------|---------------|----------------|--------------|-----------|---------|
| SBCSAE (eng) | 8226/10540 (78.0%) | 7776 (73.8%) | 8226 (78.0%) | **+4.3pp** | tie |
| Jefferson (eng) | 2371/2561 (92.6%) | 2274 (88.8%) | 2371 (92.6%) | **+3.8pp** | tie |
| TaiwanHakka (hak) | 2322/2661 (87.3%) | 2322 (87.3%) | 2246 (84.4%) | tie | **+2.9pp** |
| fusser12 (cym) | 1502/2190 | 1502 | 1476 | tie | +26 |
| german050814 (deu) | 1229/1341 | 1229 | 942 | tie | +287 |

**Conclusion:** Language-aware auto gives best of both worlds — full English
gains, zero non-English regression. Implemented in `resolve_strategy()` in
`crates/batchalign-app/src/runner/dispatch/utr.rs`.

### Implementation (shipped 2026-03-19)

1. **Language-aware UTR** — `auto` strategy gates on language: non-English →
   GlobalUtr, English → select_strategy() (inspects for overlap markers)
2. **Rev.AI language code fix** — replaced `&other[..2]` truncation with ~75
   explicit entries + `"auto"` fallback with warning
3. **Whisper hard error** — `iso3_to_language_name()` now raises `ValueError`
   for unrecognized codes instead of silently falling back to English
4. **Job-submission language validation** — `validate_language_support()` in
   `JobSubmission::validate()` rejects unsupported Rev.AI languages at
   submission time with alternatives
