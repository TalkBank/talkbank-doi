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

### 4. Reassess prior non-English experiment results — DONE

All experiments used Rev.AI as the default UTR ASR engine (`--utr-engine rev`).
The Rev.AI language code truncation bug affected Hakka but not other languages:

| Language | Code sent to Rev.AI | Correct? | Experiment impact |
|---|---|---|---|
| Hakka (hak) | `ha` (Hausa) | **WRONG** | UTR ASR may have transcribed as Hausa or failed silently. Hakka timing results are suspect. |
| German (deu) | `de` | Correct | German regression was FA grouping sensitivity, not language bug |
| Welsh (cym) | `cy` | Correct | Results valid |
| Serbian (srp) | `sr` | Correct | Results valid |
| French (fra) | `fr` | Correct | Results valid |

**Hakka:** The -200 swing on file 02 and general Hakka variance may have been
amplified by the UTR pre-pass receiving garbage ASR tokens (Rev.AI transcribing
Hakka audio as Hausa). However, the language-aware UTR fix routes all non-English
to GlobalUtr, which doesn't depend on ASR token quality for overlap recovery.
Even with correct language codes, Hakka would use GlobalUtr. No need to re-run.

**German:** The 23s median timing error and two-pass regression (64.7% vs 91.6%)
were caused by FA grouping sensitivity (152 vs 162 groups with different UTR
bullet distributions), not by the language code bug. Fixed by grouping-aware
fallback (2026-03-17). Confirmed: `deu` → `de` is a correct mapping.

**No prior conclusions about German/Welsh/Serbian/French need to change.** Only
Hakka results were potentially compromised, and the mitigation (language-aware
GlobalUtr) is already in place.

### 5. Fix morphotag V2 payload validation on corpus files — DONE

Fixed: custom serde serializer for `special_forms` flattens Rust enums to
`(Option<String>, Option<String>)` for the Python wire format. Validated on
Patagonia/30 (Welsh/English/Spanish trilingual, 1760 utterances) with
correct per-language Stanza routing.

### 6. Add --lang override to morphotag/utseg/translate — DONE

Added `--lang` flag to morphotag, translate, and coref. When set, overrides
the `@Languages` header from the CHAT file. UtsegArgs already had it.

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
