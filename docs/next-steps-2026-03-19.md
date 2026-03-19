# Next Steps: Post Language-Aware UTR and Language Hardening

**Status:** Draft
**Last updated:** 2026-03-19

## Context

On 2026-03-19 we shipped: language-aware UTR strategy (non-English → global),
Rev.AI language code regression fix (~75 explicit mappings), Whisper silent
fallback → hard error, job-submission language validation, and comprehensive
language code documentation. Per-utterance morphosyntax routing confirmed
working (improvement over BA2).

## Immediate (high priority)

### 1. Test per-utterance routing with real bilingual corpora

Validate output quality on MLE, Patagonia, Siarad from biling-data. The code
path is confirmed but we haven't validated morphosyntax output against ground
truth for bilingual files. Brian cares about this — it's user-visible.

**Files:** `data/biling-data/` (MLE-MPF, Bangor/Patagonia, Bangor/Siarad)
**What to check:** Do French/Welsh/Spanish utterances get correct %mor/%gra?

### 2. Expand validate_language_support() to all engines

We implemented Rev.AI validation at job submission. Still needed:
- Stanza: check if a model exists for the language before loading
- Whisper: pre-check that pycountry resolves the code (now hard-errors, but
  the error surfaces at inference time, not submission time)
- Custom engines (Tencent/Aliyun): validate at submission, not worker load

**Where:** `crates/batchalign-app/src/types/request.rs` — extend
`validate_language_support()`.

### 3. Deploy to fleet and test

Build release binary, deploy to net. Brian's team benefits immediately from
language-aware UTR (non-English files stop losing timing coverage).

**Commands:** `cd batchalign3 && cargo build --release -p batchalign-cli`
then `bash deploy/scripts/deploy_batchalign3.sh --server`

### 4. Reassess prior non-English experiment results

German and other non-English files showed poor timing in earlier experiments.
Some of this may have been caused by the Rev.AI language code truncation bug
(e.g., German `deu` → `de` worked by luck, but other codes didn't). Need to
check whether any prior experiment results were affected by bugs we've now
fixed.

**Files:** experiment results in `analysis/per-speaker-utr-experiment-2026-03-16/results/`
**What to check:** Were any experiments run with Rev.AI where the language code
was silently wrong? Were any Whisper runs silently in English?

## Medium-term

### 5. Per-utterance ASR engine selection (Gap 3)

Currently all utterances use one ASR engine per job. A bilingual interview
where one speaker speaks a Rev.AI-supported language and another doesn't
would benefit from per-utterance engine switching. Architecturally complex —
requires multiple ASR models loaded simultaneously and routing per utterance.

### 6. Centralized language support registry

Replace ad-hoc per-engine mapping tables with a single source of truth.
Makes "does batchalign3 support language X?" a one-lookup answer. Currently
each engine (Rev.AI, Whisper, Stanza, Tencent, Aliyun) has its own table
in different files (Rust and Python).

### 7. Deep-dive Hakka file 02 regression mechanism

Experiment 4 has debug artifacts for all 5 TaiwanHakka files × 2 strategies.
File 02 is the -200 swing that drove the aggregate Hakka regression.
Understanding the mechanism (FA group widening? UTR token misalignment?)
could inform a smarter auto strategy that works for non-English overlap files.

**Debug dirs:** `analysis/per-speaker-utr-experiment-2026-03-16/data/experiment4/debug-{global,two-pass}/02/`

## Done (2026-03-19)

- [x] Language-aware UTR strategy (non-English → global)
- [x] Rev.AI language code fix (~75 entries, no truncation)
- [x] Whisper silent fallback → hard ValueError
- [x] Job-submission language validation (Rev.AI)
- [x] Benchmark docs (pipeline, gold file, output format)
- [x] Language code resolution docs (per-engine table, no-silent-fallback policy)
- [x] Per-utterance routing docs (user-facing + migration book)
- [x] Experiments 1, 3, 4 run and results committed
- [x] batchalign-next vs BA3 audit (no real regressions found)
