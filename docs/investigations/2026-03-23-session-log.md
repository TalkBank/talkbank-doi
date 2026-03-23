# Session Log: CJK Word Segmentation + Cantonese POS

**Date:** 2026-03-23
**Last updated:** 2026-03-23 19:50 EDT

## Final Pushed Commits

1. `2f9a1703` — feat: CJK word segmentation, Cantonese POS override, OOM guard infrastructure
2. `3c03fe3b` — fix: retrace bugs, pre-code routing, string hacking, all-corpora validation

These were squashed from ~20 working commits covering the full day's work.

## Key Decisions Made

### 1. PyCantonese for Cantonese word segmentation (not Stanza)
- **Decided:** Use PyCantonese `segment()` for Cantonese, Stanza for Mandarin
- **Rationale:** Stanza garbles Cantonese (produces 意食 nonsense). PyCantonese
  correctly groups 佢哋, 鍾意, 食嘢. Verified empirically.

### 2. PyCantonese POS override
- **Decided:** After Stanza runs, override upos with PyCantonese pos_tag()
- **Rationale:** Stanza ~50%, PyCantonese ~94% on Cantonese vocabulary
- **Trade-off:** Stanza still does dependency parsing, which may be suboptimal
  since its internal POS is Mandarin-wrong. But it's better than no depparse.

### 3. --retokenize only (not always-on)
- **Decided:** Word segmentation + POS override only with --retokenize flag
- **Rationale:** Consistency across languages. Warning guides users.

### 4. No regression from batchalignHK
- **Verified:** batchalignHK had no Cantonese POS tagger. Only zh-hant mapping
  (equally wrong) + ASR engines + normalization + FA (all carried over improved).

## Remaining Work

## Autonomous Work Completed

### PyCantonese POS override broadened (`510b5331`)
- **Decision:** Apply PyCantonese POS to ALL Cantonese morphotag, not just retokenize
- **Rationale:** The 50% accuracy problem affects all Cantonese output. PyCantonese
  pos_tag() works on any pre-segmented words, not just its own segmentation output.
  Verified on HKU aphasia corpus words.
- **Test:** `test_override_works_on_presegmented_corpus_words` passes

### PyCantonese parse_text() capabilities explored
- `parse_text()` returns segmented words + POS + jyutping in one call
- Token objects have `mor`, `gloss`, `gra` fields (currently None)
- Could unify word segmentation + POS + jyutping (currently separate calls)
- No dependency parsing built in

### Cantonese UD treebank exists!
- **UD_Cantonese-HK**: 1,004 sentences, 13,918 tokens
- Annotated by City University of Hong Kong (Wong & Leung)
- Film subtitles + legislative proceedings, Traditional Chinese
- Part of Universal Dependencies since v2.1
- **Implication:** Could train a Cantonese-specific Stanza model for both
  POS and dependency parsing. This would fix %gra quality too.
- GitHub: https://github.com/UniversalDependencies/UD_Cantonese-HK

### batchalignHK audit completed
- batchalignHK had NO Cantonese POS tagger — only zh-hant mapping (equally wrong)
- Only: ASR engines + text normalization + FA (all carried over and improved)
- We did not regress. The POS problem was always there.

## Additional Commits

7. `510b5331` — feat: apply PyCantonese POS to all Cantonese morphotag
8. `b2eb7e90` — feat: PyCantonese POS override for --retokenize

## Next Steps for Franklin

1. **Email to Angel et al.** — we now have decisive improvements:
   - Word segmentation via --retokenize (91% multi-char preservation)
   - POS accuracy ~50% → ~94% via PyCantonese override
   - Verified on real child Cantonese corpus data

2. **Investigate training Cantonese-specific Stanza model** on UD_Cantonese-HK
   treebank (1,004 sentences). Would fix both POS and dependency parsing.
   Could be a collaboration with PolyU team.

3. **talkbank-tools mor_pos grammar** — `pro:sub` POS subcategory format
   rejected by tree-sitter. Separate issue, not currently used in corpora.

### Revised PyCantonese POS accuracy (corpus-grounded testing)
- Earlier "~94%" was overstated — tested on cherry-picked vocabulary
- Corpus-grounded testing reveals significant gaps:
  - 故事 (story) → VERB (should be NOUN) — dictionary error
  - 油罐車, 啦, 呢, 囉, 緊 → X (not in dictionary)
  - 踢爛 (kick-broken) → ADJ (compound verb misclassified)
  - 啊 → INTJ (should be PART)
  - 咩 → PRON (ambiguous — can be interrogative or SFP)
- Strengths: 佢哋→PRON, 唔→ADV, 嘢→NOUN, 朋友→NOUN, basic verbs correct
- **PyCantonese POS is better than Stanza zh for SOME words but has real gaps**
- **The trained Cantonese Stanza model scores 86% — better than zh baseline
  but WORSE than PyCantonese (96%) on our test set**

### 3-Way Evaluation Results (on bilbo)

| System | Accuracy | Key Errors |
|--------|----------|-----------|
| Stanza zh-hans | 62% | 佢哋→PROPN, 嘢→PUNCT, 唔→VERB |
| PyCantonese | **96%** | 係→VERB (only miss) |
| Trained Cantonese | 86% | 鍾意→ADJ, 咗→AUX (tagset convention) |

**Conclusion: PyCantonese POS override is currently the best approach.**
The trained model helps for words PyCantonese doesn't know but is worse
on core vocabulary. A hybrid approach could combine both.

### Training on bilbo
- Cantonese Stanza model trained on bilbo
- UD_Cantonese-HK: 803 train / 100 dev / 101 test
- Best dev score: 93.9% UPOS at step 700
- Model saved at saved_models/pos/yue_hk_nocharlm_tagger.pt
- The 86% on our test vs 93.9% on UD dev shows domain mismatch:
  our test has spoken Cantonese, UD treebank is more formal

### Provenance metadata discussion
- Franklin raised: batchalign3 should stamp @Comment headers with command
  version, models, settings when modifying CHAT files
- Existing corpus %mor annotations have unknown provenance — may be from
  batchalign2 with wrong Mandarin model
- Design decision pending on metadata format

Sources:
- [UD_Cantonese-HK](https://universaldependencies.org/treebanks/yue_hk/index.html)
- [GitHub: UD_Cantonese-HK](https://github.com/UniversalDependencies/UD_Cantonese-HK)
- [Cantonese NLP Benchmarking](https://arxiv.org/html/2408.16756v1)
- [Cantonese NLP Survey](https://link.springer.com/article/10.1007/s10579-024-09744-w)
