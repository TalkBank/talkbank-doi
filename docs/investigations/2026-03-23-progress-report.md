# Progress Report: CJK Word Segmentation + Cantonese POS

**Date:** 2026-03-23
**Last updated:** 2026-03-23 19:49 EDT

## What Shipped

Two commits pushed to GitHub main:

1. **`2f9a1703`** — feat: CJK word segmentation, Cantonese POS override, OOM guard infrastructure
2. **`3c03fe3b`** — fix: retrace bugs, pre-code routing, string hacking, all-corpora validation

Email sent to Brian, Angel, Houjun, Sebastian, Spencer, Spring with status update.

## What Was Built

### CJK Word Segmentation (`--retokenize` on `morphotag`)
- **Cantonese (yue):** PyCantonese `segment()` groups per-character ASR tokens into words
- **Mandarin (cmn/zho):** Stanza neural tokenizer for word segmentation
- Wire protocol extended: `retokenize: bool` field in `MorphosyntaxRequestV2`
- Cache key includes retokenize flag (no stale cache hits)
- Pipeline version bumped to v2

### Cantonese POS Override
- PyCantonese `pos_tag()` replaces Stanza POS for ALL Cantonese morphotag (not just retokenize)
- Accuracy: ~95% vs Stanza's ~50% on Cantonese vocabulary
- Applied as post-processing: keeps Stanza depparse but replaces upos

### Trained Cantonese Stanza Model (on bilbo)
- UD_Cantonese-HK treebank (1,004 sentences, 13,918 tokens)
- Results: POS 93.5%, UAS 70.4%, LAS 65.2% (vs baseline 63%/40%/24%)
- Hybrid (trained + PyCantonese POS): POS 95%, DEP ~80%

### OOM Guard Infrastructure
- 3-layer conftest.py protection prevents golden/ML tests from OOM-crashing on <128 GB machines
- Enforced by code, not convention

## What Was Tested

### All 9 TalkBank Cantonese Corpora (737K utterances)
- Word segmentation: 84-90% multi-char preservation (consistent across corpora)
- POS coverage: 98-100% vocabulary coverage (0-2% unknown rate)
- Corpora: MOST, LeeWongLeung, CHCC, EACMC, HKU, MAIN, GlobalTales, WCT, Aphasia HKU

### Bug Discovery and Fixes
| Bug | Description | Status |
|-----|-------------|--------|
| #1 | Retrace AnnotatedGroup word_counter desync | Fixed |
| #2 | Retrace AnnotatedWord word_counter desync | Fixed |
| #3 | PyCantonese join across word boundaries | Fixed |
| #4 | Parenthesis stripping dropped bare paren words | Fixed |
| #5 | [- zho] pre-code triggered Mandarin retokenize in yue job | Fixed |
| #6 | Mandarin retokenize join lost Latin word boundaries | Fixed |
| #7 | Tree-sitter joins some adjacent CJK characters | Open (low impact) |

### Test Suite Added
- **18 Python test files** covering word segmentation, POS accuracy, retrace regression, pre-code routing, mixed-script handling, HKCanCor mapping, corpus-specific tests
- **3 Rust test functions** for retrace and stanza_raw validation

## Autonomous Session Work (after email sent)

### Completed
1. **Updated known-bugs doc** — moved bugs #5-#6 from Open to Fixed, added test coverage
2. **Updated corpus inventory** — "What We've Tested" now reflects all 9 corpora
3. **HKCanCor POS investigation** — mapped 95 Chinese-style tags to UD. Finding: 99.6% map cleanly, 153K tokens available for POS training augmentation, but zero dependency annotations. Test: `test_hkcancor_mapping.py` (8 tests, all pass)
4. **Corpus-specific tests** — new tests for LeeWongLeung (child speech), EACMC (bilingual), WCT (adult conversation). Findings: 茄子→PROPN (debatable), 希望→AUX (valid Cantonese auxiliary), 切→NOUN (ambiguous in isolation). Test: `test_cantonese_corpus_specific.py` (10 tests, all pass)
5. **Mandarin corpus tests** — validated that Mandarin CHILDES data already has word boundaries (unlike Cantonese), tested retokenize decision logic and join safety. Test: `test_mandarin_corpus_specific.py` (5 tests, all pass)
6. **Doc consolidation** — marked completed investigation docs as Historical, removed duplicate findings, updated session log with final commit list
7. **Book updates** — updated cantonese.md (validation table, trained model, HKCanCor), chinese-word-segmentation.md (bugs fixed summary, corrected Tencent claim)

## What's Still Pending

### High Priority
1. **Deploy trained Cantonese model into batchalign3** — model on bilbo at `saved_models/pos/yue_hk_nocharlm_tagger.pt`, needs packaging and integration into `_stanza_loading.py`
2. **HKCanCor as training data augmentation** — convert 153K tokens to CoNLL-U, combine with UD_Cantonese-HK, retrain
3. **Daemon warning visibility** — `tracing::warn!` for per-char input fires in daemon, not visible to CLI users

### Lower Priority
4. **Bug #7 investigation** — tree-sitter joining adjacent CJK characters (low impact)
5. **Paraformer verification** — need sample raw output to verify Mandarin word boundaries
6. **More MOST corpus testing** — run `--retokenize` on more files to find remaining edge cases

## Recommended Next Steps

1. **Package and deploy the trained Cantonese Stanza model** — biggest impact on depparse quality
2. **Convert HKCanCor to CoNLL-U and retrain** — could improve POS by 10x training data
3. **Wait for Angel/Houjun response** — they may have Paraformer samples and feedback on our results
4. **Squash+push the autonomous session work** when Franklin is back (23 new tests, doc updates)
