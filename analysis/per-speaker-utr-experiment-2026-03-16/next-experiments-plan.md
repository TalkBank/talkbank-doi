# Next Experiments: Holistic Pipeline Analysis

**Status:** Draft
**Last updated:** 2026-03-17

## What We Learned

The first round of experiments revealed that:

1. **UTR-level comparison is insufficient.** UTR assigns only ~24-35
   bullets on a 1341-utterance German file. The bulk of coverage comes
   from downstream FA. Different UTR outputs lead to different FA
   groupings, which cascade into different final coverage.

2. **The pipeline is a chain:** UTR → FA grouping → FA alignment →
   postprocessing → final output. Changing any step affects all
   downstream steps.

3. **We need to measure at each stage independently** to understand
   where regressions originate and where improvements are real.

## Proposed Experiments

### Experiment A: Stage-by-stage decomposition

**Goal:** Understand where the non-English coverage regression originates.

For each test file, capture intermediate state at each pipeline stage:

| Stage | Output to capture | How |
|-------|-------------------|-----|
| 0. Input | Stripped CHAT (no timing) | `utr-experiment strip` |
| 1. UTR output | CHAT with utterance bullets only | New: `--utr-only` flag or `BATCHALIGN_DEBUG_DIR` capture |
| 2. FA groups | Group boundaries and word lists | Debug dump |
| 3. FA alignment | Word-level timing per group | Debug dump |
| 4. Final output | Complete aligned CHAT | Normal `align` output |

Compare stage 1 output between strategies to confirm UTR is behaving
correctly (we already know it is). Then compare stage 2 to see how FA
grouping differs. This identifies the exact point where the regression
enters.

**Implementation:** Use `BATCHALIGN_DEBUG_DIR` to dump intermediate
state. May need to add FA grouping dumps (currently only UTR dumps
are implemented).

### Experiment B: Full-pipeline best-of-both

**Goal:** Determine if running the full pipeline twice and keeping the
better final output eliminates the non-English regression.

For each file:
1. Run `align --utr-strategy global` → output A
2. Run `align --utr-strategy two-pass` → output B
3. Count timed utterances in A and B
4. Keep whichever has more timed utterances

**Cost:** 2x alignment per file. On small audio (13-42MB) this is
manageable. On large files it's expensive.

**Implementation options:**
- Post-hoc comparison script (run both, pick winner)
- Inline: run the full pipeline twice inside the server (major change)
- Hybrid: run UTR twice (cheap), then run FA once on the better UTR
  output (this is what we do now, but at the UTR level)

### Experiment C: FA grouping stability

**Goal:** Understand how sensitive FA grouping is to small changes in
UTR bullet assignment.

Take a file where the regression occurs (German). Capture the UTR
output from both strategies. Diff the bullets — typically only a few
utterances differ. Then manually construct variants where we take the
global UTR output and flip just 1-2 bullets to the two-pass values.
Run FA on each variant. See how much coverage changes.

This tells us whether FA grouping is fragile (small UTR changes cause
large FA changes) or stable (FA is mostly insensitive to UTR). If
fragile, we need to fix FA grouping robustness. If stable, the
regression is caused by something else.

### Experiment D: Non-English ASR quality audit

**Goal:** Determine which languages have sufficient ASR quality for
overlap-aware alignment to be meaningful.

For each language in the test set, measure:
- ASR WER (using `batchalign3 benchmark` if we have gold transcripts)
- ASR token coverage (what fraction of transcript words appear in ASR)
- ASR timing accuracy (how close are ASR timestamps to ground truth)

Languages with WER > 50% or token coverage < 60% are unlikely to
benefit from any overlap strategy — the ASR is too noisy for the DP
to produce meaningful matches regardless.

### Experiment E: Per-language strategy selection

**Goal:** Test whether per-language strategy selection improves
outcomes across the full test set.

| ASR quality | Strategy |
|-------------|----------|
| Good (WER < 30%) | Two-pass |
| Medium (WER 30-50%) | Best-of-both (full pipeline) |
| Poor (WER > 50%) | Global only |

Implementation: Detect language from `@Languages`, look up expected
ASR quality from a hardcoded table (built from Experiment D data),
select strategy accordingly.

## Priority Order

1. **Experiment A** (stage decomposition) — cheapest, most informative,
   tells us exactly where to focus.
2. **Experiment B** (full-pipeline best-of-both) — validates whether
   the approach can work at all.
3. **Experiment D** (ASR quality audit) — informs strategy selection.
4. **Experiment C** (FA stability) — explains the mechanism.
5. **Experiment E** (per-language selection) — final implementation.

## Test Files

Reuse the existing test set:
- **German (050814):** 27pp regression, clear failure case
- **Welsh (fusser12):** 9pp regression, bilingual
- **English TBI (n22, tb23):** No regression, control
- **English APROCSA (2265_T4):** Two-pass helps, English baseline
- **CORAAL (PRV_m02_1):** Dense `&*` conversion, English

## Prerequisites

- FA grouping debug dump (needs implementation in `batchalign-chat-ops`)
- `BATCHALIGN_DEBUG_DIR` capture for UTR (already implemented)
- Possibly: `--utr-only` mode that runs UTR without FA (for isolation)
