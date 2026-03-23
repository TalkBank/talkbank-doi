# HKCanCor POS Tag Mapping to Universal Dependencies

**Status:** Current
**Last updated:** 2026-03-23 19:39 EDT

## Summary

HKCanCor (Hong Kong Cantonese Corpus) contains 153,656 tokens across 16,162 utterances
from 58 files of spoken Cantonese conversation. It is built into PyCantonese and
accessible via `pycantonese.hkcancor()`.

**Assessment:** HKCanCor is valuable for **POS training data** but **cannot help with
dependency parsing** (no dependency annotations). It could augment the UD_Cantonese-HK
treebank (1,004 sentences) to improve POS accuracy, but a separate dependency treebank
would still be needed.

## Corpus Statistics

| Metric | Value |
|--------|-------|
| Total tokens | 153,656 |
| Utterances | 16,162 |
| Files | 58 |
| Avg tokens/utterance | 9.5 |
| Median tokens/utterance | 7.0 |
| Max tokens/utterance | 107 |

## Annotation Coverage

| Annotation | Tokens with it | Percentage |
|------------|----------------|------------|
| POS tags | 125,394 (non-empty) | 81.6% |
| Jyutping (romanization) | 123,070 | 80.1% |
| Empty POS (punctuation) | 28,262 | 18.4% |
| GRA (dependency) | 0 | 0% |
| MOR (morphology) | 1,565 | 1.0% |

## POS Tag Inventory

HKCanCor uses 95 unique POS tags (Chinese linguistic tradition), far more granular than UD's 17 tags.

### Mapping Quality

PyCantonese provides `hkcancor_to_ud()` which maps all 134 recognized tag values
(including 34 punctuation values) to UD tags.

| Category | Tokens | Percentage |
|----------|--------|------------|
| Maps to UD (non-X) | 124,913 | 81.3% |
| Maps to X (unknown) | 481 | 0.3% |
| Empty POS → PUNCT | 28,262 | 18.4% |

**99.6% of substantive tokens map cleanly to UD.** Only 481 tokens (0.3%) map to X.

### UD Distribution After Mapping

| UD Tag | Count | % of total |
|--------|-------|------------|
| PUNCT (from empty) | 28,262 | 18.4% |
| VERB | 27,070 | 17.6% |
| PART | 23,151 | 15.1% |
| ADV | 17,080 | 11.1% |
| NOUN | 16,009 | 10.4% |
| PRON | 15,089 | 9.8% |
| CCONJ | 7,574 | 4.9% |
| ADJ | 5,236 | 3.4% |
| NUM | 3,497 | 2.3% |
| INTJ | 3,489 | 2.3% |
| AUX | 2,654 | 1.7% |
| PROPN | 2,623 | 1.7% |
| ADP | 1,439 | 0.9% |
| X | 481 | 0.3% |
| DET | 1 | 0.0% |

### Notable Many-to-One Mappings

Several HKCanCor tags collapse into one UD tag, which could introduce noise:

- **VERB** ← `v`, `v1` (neg), `vk` (directional), `Vg`, `g1`, `xv`, `xVg`, `xjv` — 8 source tags
- **PROPN** ← `nr` (person), `ns` (place), `nt` (org), `nz` (other proper), `h` (prefix 阿), plus 15 more — 20 source tags
- **ADJ** ← `a`, `b` (attributive), `g2`, `jb`, `z` (stative), `xa`, `xb`, `Ag`, `Bg`, `xja` — 10 source tags
- **NOUN** ← `n`, `q` (classifier), `nx`, `vn`, `an`, etc. — 19 source tags

The `v`→VERB and `v1`→VERB collapse is fine (both are verbs). The `q`→NOUN mapping
(classifiers as nouns) is the most debatable — UD doesn't have a CLASSIFIER tag.

### Tags That Map to X (Unknown)

| Tag | Count | Examples | Issue |
|-----|-------|----------|-------|
| `#` | 25 | ○ | Circle symbol |
| `i` | 110 | 七七八八, 眾叛親離 | Idiomatic expressions |
| `k` | 56 | 性, 仔 | Suffixes |
| `l` | 126 | 唔緊要, 割禾青 | Fixed phrases |
| `l1` | 44 | 黐線, 唔該 | Fixed expressions |
| `o` | 44 | 唥唥, 哈 | Onomatopoeia |
| `x` | 51 | Magic, 喇 | Mixed/unclear |
| `xx` | 16 | SSG, Spec | Abbreviations |
| `Ig`, `Lg`, `Mg`, `g` | 8 | Various | Rare morphemes |

Total X: 481 tokens (0.3%). Most are idioms (`i`, `l`, `l1`), onomatopoeia (`o`),
and suffixes (`k`) — edge cases that don't affect the bulk of POS training.

## Assessment for Stanza Training

### Can Use For: POS Training

- 125K tokens with clean UD POS mappings (via `hkcancor_to_ud()`)
- Would increase POS training data by ~10x over UD_Cantonese-HK (13,918 tokens)
- Spoken Cantonese domain matches TalkBank's use case exactly
- Already tokenized at word level (no segmentation needed)
- Jyutping provides potential features for POS (pronunciation → part-of-speech correlation)

### Cannot Use For: Dependency Parsing

- Zero dependency annotations (no head, no deprel)
- Would need manual annotation or projection from a parallel corpus
- UD_Cantonese-HK remains the only Cantonese dependency resource

### Steps to Convert HKCanCor to CoNLL-U for Stanza

1. Export utterances with word + POS
2. Map POS via `hkcancor_to_ud()`
3. Set `head=0` and `deprel=dep` as placeholders (or omit dep training)
4. Filter out X-tagged tokens or manually categorize them
5. Split into train/dev/test (80/10/10)
6. Train Stanza with `--train_with_pos_only` (if supported) or use combined with UD_Cantonese-HK for dep

### Risk: Domain Mismatch with UD_Cantonese-HK

- HKCanCor: spoken conversation (natural, informal)
- UD_Cantonese-HK: film subtitles + legislative proceedings (formal/semi-formal)
- Mixed training could hurt if domains clash, but spoken Cantonese is our target domain,
  so HKCanCor's domain is actually better for our use case

## Verification

Test script: `batchalign3/batchalign/tests/pipelines/morphosyntax/test_hkcancor_mapping.py`
