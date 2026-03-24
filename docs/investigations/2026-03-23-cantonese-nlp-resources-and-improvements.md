# Cantonese NLP Resources and Improvement Opportunities

**Status:** Current
**Last updated:** 2026-03-23 23:40 EDT

Research into available Cantonese NLP resources and actionable improvements
for batchalign3's Cantonese pipeline.

## Immediate Improvements (can do now)

### 1. Train a Cantonese Character Language Model (charlm)

**Impact: HIGH** — charlm improves all Stanza components (tokenizer, POS, depparse).
Stanza's zh-hans models with charlm score significantly better than nocharlm variants.

**Data source: Cantonese Wikipedia (yue.wikipedia.org)**
- ~50,000+ articles in Cantonese
- Download from dumps.wikimedia.org (yue wiki dump)
- Clean, well-written Cantonese text
- CC BY-SA licensed

**How:** Stanza has a documented charlm training pipeline:
1. Download yue Wikipedia dump
2. Extract text with `wiki2text` or `gensim`
3. Run `stanza/models/charlm.py` to train forward + backward models
4. Use trained charlm when training tokenizer/POS/depparse

**Effort:** ~1 day. Data download + extraction + training.
**Where:** bilbo (needs ~8 GB RAM for training).

Reference: [Stanza charlm docs](https://stanfordnlp.github.io/stanza/new_language_charlm.html)

### 2. Add UD_Chinese-HK as parallel training data for depparse

**Impact: MEDIUM** — more dependency-annotated data for the parser.

UD_Chinese-HK is a **parallel treebank** with UD_Cantonese-HK — same film
subtitles and legislative proceedings, annotated in Mandarin. It has ~1,000
sentences with full dependency annotations.

While it's Mandarin (not Cantonese), the parallel structure means the
dependency patterns are closely aligned. Cross-lingual transfer from this
data could help the dependency parser learn syntactic structures that are
shared between Cantonese and Mandarin.

**How:** Clone UD_Chinese-HK, combine with UD_Cantonese-HK for depparse training.
**Effort:** 2 hours.

Source: [UD_Chinese-HK](https://github.com/UniversalDependencies/UD_Chinese-HK)

### 3. Use indiejoseph/bert-base-cantonese as Stanza BERT embeddings

**Impact: HIGH** — BERT embeddings dramatically improve Stanza accuracy.

A Cantonese BERT model exists on HuggingFace: `indiejoseph/bert-base-cantonese`.
It was continually pre-trained from `bert-base-chinese` on the Cantonese Common
Crawl dataset (198M tokens). Stanza supports using BERT as additional features
for POS and depparse via `--bert_model`.

**How:** Pass `--bert_model indiejoseph/bert-base-cantonese` during Stanza training.
**Effort:** 4 hours (retrain POS + depparse with BERT features).
**Caveat:** Increases model size and inference time. Need to benchmark tradeoff.

Source: [indiejoseph/bert-base-cantonese](https://huggingface.co/indiejoseph/bert-base-cantonese)

## Medium-Term Improvements (this week)

### 4. Augment training data with LIHKG corpus

**Impact: HIGH for charlm and tokenizer.**

LIHKG is Hong Kong's most popular online forum. A scraper collected 172M+
unique sentences of Cantonese text. This is informal spoken-style Cantonese
— excellent domain match for TalkBank's child speech and conversation data.

The BART-base-cantonese model was trained on LIHKG data. The ayaka14732/lihkg-scraper
tool can collect the data.

**How:** Scrape or download LIHKG text, use for charlm training.
**Caveat:** License unclear for scraped forum data. May need to check terms.

Source: [ayaka14732/bart-base-cantonese](https://github.com/ayaka14732/bart-base-cantonese)

### 5. CantoNLU benchmark evaluation

**Impact: MEDIUM** — validates our model against community standards.

CantoNLU (October 2025) provides 7 evaluation tasks for Cantonese NLP, including
POS tagging and dependency parsing. Running our trained model on their benchmark
would give us community-comparable numbers.

**How:** Download CantoNLU test sets, run our models, report scores.
**Effort:** 4 hours.

Source: [CantoNLU](https://arxiv.org/html/2510.20670v1)

### 6. Yue-Benchmark evaluation for broader coverage

The Yue-Benchmark (NAACL 2025) evaluates LLMs on Cantonese across factual
generation, math, reasoning, and general knowledge. While this targets LLMs,
the datasets could inform vocabulary gaps in our tokenizer/POS models.

Source: [Yue-Benchmark](https://github.com/jiangjyjy/Yue-Benchmark)

## Longer-Term Improvements

### 7. Train from scratch with YueTung-scale data

Recent work (March 2025) constructed a 2-billion-token Cantonese corpus through
rigorous filtering of web data. While this targets LLM training, the filtered
text could produce an excellent charlm and potentially enable training a much
larger tokenizer.

Source: [YueTung paper](https://arxiv.org/abs/2503.03702)

### 8. Contribute yue models to Stanza upstream

Submit a PR to `stanfordnlp/stanza-resources` adding `yue` as an officially
supported language. This benefits the entire NLP community and gives us
long-term maintenance support from the Stanza team.

**Prerequisite:** Solid evaluation numbers, clean model packaging, documentation.

Source: [Stanza new language guide](https://stanfordnlp.github.io/stanza/new_language.html)

### 9. Collaboration with PolyU team on child Cantonese annotation

Angel Chan's team at PolyU works on child Cantonese acquisition. They may be
willing to annotate child speech data with dependency relations, which would
directly address our depparse bottleneck (only 803 training sentences).

### 10. rime-cantonese lexicon for tokenizer dictionary

The rime-cantonese project has ~170,000 word-romanization pairs for Cantonese.
This could supplement the tokenizer's vocabulary, especially for proper nouns
and domain-specific terms that the UD+HKCanCor training data doesn't cover.

Source: [rime-cantonese](https://github.com/CanCLID/awesome-cantonese-nlp)

## Priority Order

| # | Improvement | Impact | Effort | Dependencies |
|---|------------|--------|--------|-------------|
| 1 | Cantonese charlm (Wikipedia) | HIGH | 1 day | None — can start now |
| 3 | BERT embeddings | HIGH | 4 hours | Download model |
| 2 | UD_Chinese-HK for depparse | MEDIUM | 2 hours | Clone treebank |
| 5 | CantoNLU benchmark | MEDIUM | 4 hours | Download benchmark |
| 4 | LIHKG augmentation | HIGH | 1 day | License check |
| 8 | Stanza upstream PR | HIGH (community) | 1 week | Eval numbers ready |
| 9 | PolyU collaboration | HIGH (depparse) | Months | Relationship building |

## Resources Index

| Resource | Type | Size | License | URL |
|----------|------|------|---------|-----|
| UD_Cantonese-HK | Treebank | 1,004 sents | CC BY-SA 4.0 | [GitHub](https://github.com/UniversalDependencies/UD_Cantonese-HK) |
| UD_Chinese-HK | Treebank (parallel) | ~1,000 sents | CC BY-SA 4.0 | [GitHub](https://github.com/UniversalDependencies/UD_Chinese-HK) |
| HKCanCor | POS corpus | 16,162 utts | Research | [PyCantonese](https://pycantonese.org/) |
| yue Wikipedia | Raw text | ~50K articles | CC BY-SA | [dumps.wikimedia.org](https://dumps.wikimedia.org) |
| bert-base-cantonese | BERT model | ~400 MB | MIT | [HuggingFace](https://huggingface.co/indiejoseph/bert-base-cantonese) |
| CantoNLU | Benchmark | 7 tasks | Research | [arXiv](https://arxiv.org/html/2510.20670v1) |
| awesome-cantonese-nlp | Resource list | — | — | [GitHub](https://github.com/CanCLID/awesome-cantonese-nlp) |
| LIHKG scraper | Forum text | 172M sents | Unclear | [GitHub](https://github.com/ayaka14732/lihkg-scraper) |
| Cantonese NLP survey | Paper | — | — | [Springer](https://link.springer.com/article/10.1007/s10579-024-09744-w) |
