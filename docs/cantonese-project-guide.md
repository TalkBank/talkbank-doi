# Cantonese NLP Project Guide

**Status:** Current
**Last updated:** 2026-03-24 08:29 EDT

Complete reference for all Cantonese/CJK NLP work in TalkBank: what we built,
where everything lives, how to reproduce it, and who's involved.

## People

| Person | Role | Affiliation |
|--------|------|-------------|
| **Brian MacWhinney** | PI, TalkBank | CMU |
| **Franklin Chen** | Developer, batchalign3 | CMU |
| **Houjun Liu** | Developer, batchalign3 | Stanford |
| **Angel Chan** | Cantonese language acquisition researcher | PolyU (Hong Kong) |
| **Spencer** | Research assistant, testing batchalign3 | PolyU |
| **Spring** | Research assistant | PolyU |
| **Wanlin** | Research assistant, provided Paraformer data | PolyU |
| **Sebastian** | Suggested PyCantonese for word segmentation | — |

## Timeline

| Date | Event |
|------|-------|
| 2026-03-23 AM | Angel's team reports per-character ASR output problem for Cantonese |
| 2026-03-23 AM | CJK `--retokenize` implemented (PyCantonese for yue, Stanza for cmn) |
| 2026-03-23 PM | PyCantonese POS override deployed (50% → 95% accuracy) |
| 2026-03-23 PM | 6 bugs found and fixed across all 9 TalkBank Cantonese corpora |
| 2026-03-23 PM | Email sent to Angel et al. with initial status |
| 2026-03-23 EVE | Houjun suggests unified Stanza model instead of 3-tool pipeline |
| 2026-03-23 EVE | HKCanCor → CoNLL-U conversion (16,162 utterances, 153K tokens) |
| 2026-03-23 EVE | Unified model training on bilbo: tokenizer F1=96.4%, POS 93.4% |
| 2026-03-23 NIGHT | Parallel treebank added, depparse LAS 67.7% (+3 over v1) |
| 2026-03-23 NIGHT | Cantonese Wikipedia charlm training started (76.6M chars) |
| 2026-03-24 AM | Paraformer samples received from Wanlin, analyzed (100% per-char, CER 5.5%) |
| 2026-03-24 AM | Parser threading refactor, batchalign3 pushed to GitHub |
| 2026-03-24 AM | Reply email sent to full thread |

## What We Built

### 1. CJK Word Segmentation (`--retokenize`)

Per-character ASR output → word-level tokens for morphosyntax analysis.

| Language | Engine | Method |
|----------|--------|--------|
| Cantonese (yue) | PyCantonese `segment()` | Dictionary-based |
| Mandarin (cmn/zho) | Stanza neural tokenizer | Neural (pretokenized=False) |

**Shipped in:** batchalign3 commits `2f9a1703` + `3c03fe3b` + `047865d4`

**Usage:** `batchalign3 morphotag --retokenize corpus/ -o output/ --lang yue`

### 2. Cantonese POS Override

PyCantonese `pos_tag()` replaces Stanza POS for all Cantonese morphotag output.
Stanza's Mandarin model scored ~50% on Cantonese; PyCantonese scores ~95%.

Applied as post-processing in `batchalign/inference/morphosyntax.py`.

### 3. Unified Cantonese Stanza Model (trained, not yet deployed)

Tokenizer + POS + depparse trained on HKCanCor + UD_Cantonese-HK.
Will replace the 3-tool pipeline when integrated.

| Component | Score | Baseline |
|-----------|-------|----------|
| Tokenizer F1 (UD held-out) | 90.3% | PyCantonese 77.3% |
| POS (UD held-out) | 93.4% | PyCantonese 73.1% |
| Depparse LAS (held-out) | 67.7% | Mandarin model 24% |

### 4. Paraformer Verification

All 5 ASR engines confirmed 100% per-character for CJK:
Whisper, Tencent, Aliyun, FunASR, Paraformer.

Paraformer CER on Mandarin: 5.5% overall. No speaker diarization.

## Repositories and Projects

### batchalign3 (`talkbank/batchalign3`)

The main product. CJK retokenize + POS override are shipped.

| File | What |
|------|------|
| `batchalign/inference/morphosyntax.py` | `_segment_cantonese()`, `_override_pos_with_pycantonese()`, Mandarin retokenize |
| `batchalign/worker/_stanza_loading.py` | `load_stanza_retokenize_model()` for Mandarin |
| `crates/batchalign-chat-ops/src/retokenize/` | Rust AST retokenization (word splits/merges) |
| `crates/batchalign-chat-ops/src/morphosyntax/` | Cache key with retokenize flag, pipeline version |
| `book/src/reference/languages/cantonese.md` | User-facing Cantonese reference |
| `book/src/reference/chinese-word-segmentation.md` | User-facing word segmentation reference |
| `book/src/architecture/cjk-word-segmentation.md` | Developer architecture rationale |

**Tests (26 files, ~175 tests):**

| Test file | What it tests |
|-----------|--------------|
| `test_cantonese_all_corpora.py` | Word seg + POS across all 9 TalkBank corpora |
| `test_cantonese_corpus_gold.py` | POS on Aphasia HKU utterances |
| `test_cantonese_corpus_specific.py` | LeeWongLeung, EACMC, WCT corpus tests |
| `test_cantonese_mor_comparison.py` | Existing %mor vs PyCantonese |
| `test_cantonese_pos_judgments.py` | Manual linguistic judgments |
| `test_cantonese_tagset_analysis.py` | Tagset normalization |
| `test_hkcancor_mapping.py` | HKCanCor → UD POS mapping (99.6% clean) |
| `test_mandarin_corpus_specific.py` | Mandarin data characteristics + retokenize logic |
| `test_mandarin_retok_mixed_script.py` | Latin+CJK join safety |
| `test_pycantonese_corpus_quality.py` | PyCantonese on CHCC corpus |
| `test_pycantonese_pos_vs_stanza.py` | PyCantonese vs Stanza accuracy |
| `test_retokenize_retrace_e2e.py` | End-to-end retrace regression |
| `test_retokenize_retrace_regression.py` | Python-side retrace check |
| `test_precode_retokenize.py` | [- zho] pre-code routing |
| `test_paren_strip_bug.py` | Parenthesis stripping regression |
| `test_word_segmentation.py` | Wire types + PyCantonese segmentation |
| `test_cjk_word_segmentation_claims.py` | Claim verification with real models |
| `test_retokenize_vs_engines.py` | Stanza vs PyCantonese for Cantonese |
| `test_stanza_cantonese_pos_accuracy.py` | Documents ~50% Stanza accuracy |
| `test_stanza_cantonese_depparse.py` | Depparse quality |
| `test_tencent_word_segmentation.py` | Tencent per-character verification |

### cantonese-unified-training (`talkbank/cantonese-unified-training`)

Standalone project for training the unified Stanza model. Not on GitHub — local + bilbo.

| File | What |
|------|------|
| `METHODOLOGY.md` | Full provenance, reasoning, pitfalls, results |
| `CLAUDE.md` | Coding standards for this project |
| `scripts/setup_data.sh` | Reproducible data download from all sources |
| `scripts/convert_hkcancor.py` | HKCanCor → CoNLL-U (16,162 utterances) |
| `scripts/combine_datasets.py` | Merge HKCanCor + UD splits |
| `scripts/add_parallel_treebank.py` | Add UD_Chinese-HK for depparse |
| `scripts/benchmark_pycantonese.py` | Tokenization baseline (F1=0.79) |
| `scripts/extract_wikipedia.py` | Cantonese Wikipedia → text (76.6M chars) |
| `scripts/train_all.py` | Train tokenizer + POS + depparse |
| `scripts/train_charlm.py` | Train character language model |
| `scripts/train_with_charlm_and_bert.py` | Retrain with charlm + BERT features |
| `scripts/run_eval.py` | Held-out evaluation |
| `scripts/eval_depparse_heldout.py` | Depparse on 101-sentence test |
| `scripts/eval_talkbank_corpora.py` | Cross-domain eval on 9 corpora |
| `scripts/error_analysis.py` | Tokenization error patterns |
| `scripts/compare_all_models.py` | Side-by-side model comparison |
| `tests/test_conllu_quality.py` | CoNLL-U format validation (10 tests) |

**Models on bilbo** (`~/cantonese-unified-training/models/`):

| Model | Size | What |
|-------|------|------|
| `yue_combined_tokenizer.pt` | 971 KB | Cantonese tokenizer (F1=96.4%) |
| `yue_combined_tagger.pt` | 17 MB | Cantonese POS (93.4%) |
| `yue_hk_depparse.pt` | 101 MB | Cantonese depparse v1 (LAS 64.7%) |
| `yue_combined_depparse_v2.pt` | 102 MB | Depparse v2 with parallel data (LAS 67.7%) |
| `yue_forward_charlm.pt` | 28 MB | Forward character LM from Wikipedia |

### paraformer-analysis (`talkbank/data-incoming/paraformer-samples`)

Analysis of Paraformer raw output from Wanlin.

| File | What |
|------|------|
| `scripts/analyze_paraformer.py` | CER, word seg, diarization analysis |
| `NLM-Mandarin examples/Raw output/` | 5 raw Paraformer transcripts |
| `NLM-Mandarin examples/Gold transcripts/` | 4 gold transcripts |
| `NLM-Mandarin examples/Audios/` | 5 audio files |

## Investigation Documents

All in `talkbank/docs/investigations/`:

| Document | Status | What |
|----------|--------|------|
| `cantonese-corpus-inventory.md` | Current | All 9 TalkBank Cantonese corpora, testing status |
| `cantonese-retokenize-known-bugs.md` | Current | 7 bugs tracked, 6 fixed, 1 open |
| `2026-03-24-paraformer-analysis.md` | Current | Paraformer CER 5.5%, 100% per-char |
| `2026-03-23-hkcancor-pos-mapping.md` | Current | HKCanCor 95 tags → UD, 99.6% clean |
| `2026-03-23-cantonese-nlp-resources-and-improvements.md` | Current | External resources + improvement roadmap |
| `2026-03-23-model-packaging-proposal.md` | Draft | 5 distribution options, HuggingFace recommended |
| `2026-03-23-cantonese-pos-quality.md` | Historical | Early POS findings (superseded by corpus inventory) |
| `2026-03-23-cantonese-stanza-training.md` | Historical | Initial training research |
| `2026-03-23-cantonese-stanza-training-plan.md` | Historical | Original training plan for bilbo |
| `2026-03-23-cjk-segmentation-next-steps.md` | Historical | Early next-steps (all shipped or tracked) |
| `2026-03-23-cjk-testing-report.md` | Historical | Initial testing report |
| `2026-03-23-houjun-retokenize-claim.md` | Historical | Houjun's claim verification |
| `2026-03-23-progress-report.md` | Historical | End-of-day-1 report |

## Email Drafts

| File | Status | What |
|------|--------|------|
| `docs/emails/2026-03-23-word-segmentation-reply.md` | Sent 2026-03-23 | Initial status to Angel et al. |
| `docs/emails/2026-03-24-houjun-unified-model-reply.md` | Sent 2026-03-24 | Unified model + Paraformer findings |

## Training Data Provenance

| Source | License | Tokens | Domain | Used for |
|--------|---------|--------|--------|----------|
| UD_Cantonese-HK | CC BY-SA 4.0 | 13,918 | Film + legislative | Tokenizer, POS, depparse |
| UD_Chinese-HK | CC BY-SA 4.0 | 9,874 | Film + legislative (Mandarin parallel) | Depparse |
| HKCanCor | CC BY 4.0 | 153,656 | Spoken conversation | Tokenizer, POS |
| Cantonese Wikipedia | CC BY-SA 3.0 | 76.6M chars | Encyclopedia | Character language model |
| bert-base-cantonese | MIT | 198M tokens | Common Crawl | BERT features (queued) |

All sources verified safe for public model distribution. Trained models would be CC BY-SA 4.0.

## Bilbo (`ssh macw@bilbo`)

Two training projects live on bilbo:

### `~/cantonese-unified-training/` (current)

The main training project, synced from `talkbank/cantonese-unified-training/`.

```
models/
  yue_combined_tokenizer.pt       971 KB   Cantonese tokenizer (F1=96.4%)
  yue_combined_tagger.pt           17 MB   Cantonese POS (93.4%)
  yue_hk_depparse.pt              101 MB   Depparse v1 (LAS 64.7%)
  yue_combined_depparse_v2.pt     102 MB   Depparse v2 with parallel data (LAS 67.7%)
  yue_forward_charlm.pt            28 MB   Forward charlm (trained on Wikipedia)
  yue_wiki_vocab.pt                48 KB   Charlm vocabulary

data/
  ud-cantonese-hk/                         Cloned UD treebank (1,004 sents)
  ud-chinese-hk/                           Parallel Mandarin treebank
  hkcancor-conllu/                         Converted HKCanCor (16,162 sents)
  combined/                                Merged training data (13,732 sents)
  depparse-combined/                       UD + Chinese-HK for depparse (1,606 sents)
  wikipedia/                               Cantonese Wikipedia dump (129 MB bz2)
    extracted/                             Plain text (76.6M chars, 27 batch files)
  charlm_raw/yue/                          Charlm training data (146 MB train, 6.8 MB dev)
  bert-base-cantonese/                     indiejoseph BERT model (392 MB)
```

**Environment:** Python 3.12 venv at `.venv/`. Dependencies: stanza, torch, pycantonese, transformers.

### `~/cantonese-stanza-training/` (earlier, superseded)

The afternoon session's initial training run (before HKCanCor was added).
Contains the first UD-only trained models in `saved_models/`. Superseded
by `cantonese-unified-training/` which uses combined data.

Keep for reference but don't use these models — the unified project has
better results.

## What's Next

### Immediate (blocked on model integration decision)
1. **Deploy trained Cantonese model into batchalign3** — needs model packaging decision (see proposal doc)
2. **Discuss model distribution with Brian** — HuggingFace recommended

### Ready to run (bilbo)
3. **Forward charlm trained** (28 MB) — backward charlm still needed
4. **Retrain POS + depparse with charlm** — run `scripts/train_with_charlm_and_bert.py` after backward charlm

### Future
5. **BERT-enhanced training** — `indiejoseph/bert-base-cantonese` for deeper features
6. **CantoNLU benchmark** — evaluate against community standard
7. **Stanza upstream PR** — contribute `yue` as official Stanza language
8. **PolyU collaboration** — child Cantonese annotation for depparse improvement

## How to Reproduce

### Retokenize pipeline (shipped)
```bash
# Install batchalign3
pip install batchalign3  # or clone and build

# Run on Cantonese
batchalign3 morphotag --retokenize corpus/ -o output/ --lang yue

# Run on Mandarin
batchalign3 morphotag --retokenize corpus/ -o output/ --lang cmn
```

### Model training
```bash
# On bilbo (or any machine with Python 3.12 + GPU)
cd cantonese-unified-training
bash scripts/setup_data.sh        # Download all data
uv run python scripts/train_all.py --device mps  # Train
uv run python scripts/run_eval.py                 # Evaluate
```

### Paraformer analysis
```bash
cd data-incoming/paraformer-samples
uv run python scripts/analyze_paraformer.py
```
