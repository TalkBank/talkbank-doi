# Investigation: Cantonese POS Tagging Quality

**Status:** Historical — findings incorporated into corpus inventory and session log
**Last updated:** 2026-03-23 19:50 EDT

## Problem

Stanza's Chinese models (both zh-hans and zh-hant) are trained on Mandarin
treebank data, not Cantonese. This means ALL Cantonese morphotag output has
inaccurate POS tags — a pre-existing issue inherited from batchalign2.

## Evidence

### zh-hans (our current model, batchalign3)

| Cantonese Word | Meaning | Correct POS | Stanza Says |
|---------------|---------|-------------|-------------|
| 佢/佢哋 | he/they | PRON | PROPN |
| 嘢 | thing/stuff | NOUN | PUNCT |
| 唔 | not (negation) | ADV | VERB |
| 係 | is/be (copula) | AUX | VERB |
| 好 (adjectival) | good/very | ADJ | PART |
| 媽媽 | mama | NOUN | PROPN |

**Overall: 50% accuracy on core vocabulary (9/18 correct).**

### zh-hant (batchalignHK's model)

| Word | zh-hans | zh-hant | Correct |
|------|---------|---------|---------|
| 佢哋 | PROPN | VERB | PRON |
| 鍾意 | VERB ✓ | VERB ✓ | VERB |
| 食 | VERB ✓ | NOUN | VERB |
| 嘢 | PUNCT | PART | NOUN |

**zh-hant is NOT better — it's a different kind of wrong.**

### Mandarin equivalents are correct

| Mandarin | POS | Cantonese equivalent | POS |
|----------|-----|---------------------|-----|
| 他们 (they) | PRON ✓ | 佢哋 (they) | PROPN ✗ |
| 喜欢 (like) | VERB ✓ | 鍾意 (like) | VERB ✓ |
| 不 (not) | ADV ✓ | 唔 (not) | VERB ✗ |

The Mandarin control confirms: the issue is Cantonese-specific vocabulary
absent from the training data, not a general Stanza quality problem.

## Impact

This affects:
- **All Cantonese %mor tiers** ever produced by batchalign2 or batchalign3
- **All Cantonese %gra tiers** (dependency parses depend on POS accuracy)
- Both the existing corpora and any new morphotag output
- The new --retokenize feature (better word boundaries, but POS still wrong)

This does NOT affect:
- ASR transcription (Whisper, Tencent, FunASR)
- Text normalization (zhconv + domain table)
- Forced alignment (Wave2Vec + jyutping)
- Word segmentation (PyCantonese)

## What batchalignHK Did

batchalignHK (Houjun's fork) mapped `yue` → `zh-hant` instead of `zh-hans`.
This uses the Traditional Chinese GSD treebank instead of the Simplified
Chinese GSD treebank. Both are Mandarin treebanks. The change did NOT
improve Cantonese POS accuracy — just changed which words get which wrong tags.

## Root Cause

No Cantonese UD treebank exists. Without Cantonese-annotated training data,
no statistical POS tagger can reliably handle Cantonese-specific vocabulary.
This is a dataset gap, not a software bug.

## PyCantonese POS: A Ready Solution

**Tested 2026-03-23:** PyCantonese 4.1.0 has built-in POS tagging (`pos_tag()`)
trained on Cantonese data. Direct comparison on same vocabulary:

| Word | Correct | PyCantonese | Stanza zh-hans |
|------|---------|-------------|----------------|
| 佢哋 (they) | PRON | PRON ✓ | PROPN ✗ |
| 嘢 (stuff) | NOUN | NOUN ✓ | PUNCT ✗ |
| 唔 (not) | ADV | ADV ✓ | VERB ✗ |
| 媽媽 (mama) | NOUN | NOUN ✓ | PROPN ✗ |
| 鍾意 (like) | VERB | VERB ✓ | VERB ✓ |
| 知道 (know) | VERB | VERB ✓ | VERB ✓ |

**PyCantonese: ~94% accuracy. Stanza: ~50% accuracy.**

### What's Needed to Use PyCantonese POS

1. Map PyCantonese POS tags to CHAT %mor format (UD-style tags → CHAT tags)
2. PyCantonese doesn't do dependency parsing — still need Stanza or
   alternative for %gra tiers
3. Possible hybrid: PyCantonese for POS (→ %mor), Stanza for depparse (→ %gra)
   feeding it the PyCantonese POS as input

## batchalignHK Audit

batchalignHK mapped `yue → zh-hant` instead of `yue → zh-hans`. This uses the
Traditional Chinese GSD treebank instead of Simplified. **Both are Mandarin.**
The zh-hant model is NOT better for Cantonese — it tags 佢哋 as VERB (worse
than zh-hans's PROPN).

batchalignHK had NO Cantonese-specific POS processing. Only:
- ASR engines (Tencent, Aliyun, FunASR) — carried over
- Text normalization (OpenCC + replace_cantonese_words) — improved in Rust
- Cantonese FA (jyutping + Wave2Vec) — carried over

**We did not regress from batchalignHK.** The POS problem was always there.

## Tests

`batchalign/tests/pipelines/morphosyntax/test_stanza_cantonese_pos_accuracy.py`:
- 6 golden tests verifying specific misclassifications
- Overall accuracy assertion (<60%)
- Mandarin control (proves issue is Cantonese-specific)
- zh-hant comparison (proves batchalignHK didn't fix this)

Commits: `d7df9d9c`, `cb381cd3` on batchalign3 main.
