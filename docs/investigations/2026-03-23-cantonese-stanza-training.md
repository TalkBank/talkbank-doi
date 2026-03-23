# Investigation: Training a Cantonese-Specific Stanza Model

**Status:** Research complete, implementation not started
**Last updated:** 2026-03-23 13:15 EDT

## Motivation

Stanza's `zh` model (Mandarin Chinese Treebank) scores ~50% POS accuracy on
Cantonese vocabulary. PyCantonese POS override improves this to ~94% but
doesn't fix dependency parsing (%gra tiers), which also uses the Mandarin model.

A Cantonese-specific Stanza model trained on the UD_Cantonese-HK treebank
would fix both POS and dependency parsing.

## Resources Available

### UD_Cantonese-HK Treebank
- **Size:** 1,004 sentences, 13,918 tokens
- **Source:** Film subtitles + legislative proceedings (Hong Kong)
- **Script:** Traditional Chinese
- **Annotation:** Full UD annotation (POS, deps, morphology)
- **Contributors:** Tak-sum Wong, Herman H.M. Leung (City University of HK)
- **GitHub:** https://github.com/UniversalDependencies/UD_Cantonese-HK
- **UD version:** Available since v2.1

### Stanza Training Infrastructure
- Official training tutorial: https://github.com/stanfordnlp/stanza-train
- Training docs: https://stanfordnlp.github.io/stanza/training.html
- Supports training POS, lemma, depparse from UD CoNLL-U data
- Requires pretrained word vectors (Chinese vectors available)

## Training Plan

```bash
# 1. Clone treebank
git clone https://github.com/UniversalDependencies/UD_Cantonese-HK

# 2. Prepare data
python -m stanza.utils.datasets.prepare_pos_treebank UD_Cantonese-HK
python -m stanza.utils.datasets.prepare_depparse_treebank UD_Cantonese-HK

# 3. Train POS (needs GPU, ~30 min for small treebank)
python -m stanza.utils.training.run_pos UD_Cantonese-HK \
    --wordvec_pretrain_file zh_pretrain.pt

# 4. Train dependency parser
python -m stanza.utils.training.run_depparse UD_Cantonese-HK \
    --wordvec_pretrain_file zh_pretrain.pt

# 5. Package as Stanza model
# (follow stanza-train packaging instructions)
```

## Considerations

### Size Concerns
1,004 sentences is small for NLP model training. For comparison:
- Chinese Treebank (Mandarin): ~57,000 sentences
- English GSD: ~12,000 sentences

However, small UD treebanks (1,000-2,000 sentences) have produced usable
models for other languages (e.g., Welsh, Buryat, Old English). Transfer
learning from the Chinese pretrained vectors helps compensate.

### Domain Mismatch
The treebank contains film subtitles and legislative proceedings — formal
Cantonese. Our use case is child speech and aphasia transcripts — informal
Cantonese. Domain mismatch may reduce accuracy.

Angel Chan's PolyU team has Cantonese child speech expertise and may have
annotated data that could supplement the UD treebank.

### Integration Path
If the model works, integration is straightforward:
1. Package model files into batchalign3's model distribution
2. Map `yue` → custom Cantonese model instead of `zh`
3. Remove PyCantonese POS override (no longer needed)
4. PyCantonese would still be used for word segmentation and jyutping

## Recommendation

**Short term (current):** Keep PyCantonese POS override. It's fast, tested,
and gives ~94% accuracy on POS.

**Medium term:** Train a Cantonese Stanza model on UD_Cantonese-HK. Evaluate
on our test sentences. If it beats PyCantonese POS AND gives good dependency
parses, deploy it.

**Potential collaboration:** Discuss with Angel's PolyU team about:
1. Supplementing UD_Cantonese-HK with child speech annotations
2. Evaluating our pipeline on their Cantonese data
3. Contributing improvements back to the UD treebank

Sources:
- [UD_Cantonese-HK](https://universaldependencies.org/treebanks/yue_hk/index.html)
- [Stanza Training Tutorial](https://github.com/stanfordnlp/stanza-train)
- [Stanza Training Docs](https://stanfordnlp.github.io/stanza/training.html)
- [Cantonese NLP Survey](https://link.springer.com/article/10.1007/s10579-024-09744-w)
