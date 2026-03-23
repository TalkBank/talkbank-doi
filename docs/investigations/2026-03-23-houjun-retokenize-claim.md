# Investigation: Houjun's Retokenize Claim

**Status:** Resolved
**Last updated:** 2026-03-23 12:45 EDT

## The Claim

Houjun Liu (2026-03-23 email to all): "I'm pretty certain that Retokenize as
a secondary pass would do a better job than either Tencent or in particular
FunAudio alone since Stanza has a special model trained just for this task."

## Three Sub-Claims Tested

### Sub-claim 1: Retokenize > FunASR alone

**Verdict: CONFIRMED** (trivially true)

FunASR produces per-character tokens (zero word boundaries). Any word segmenter
is better than nothing. PyCantonese retokenize consistently reduces token count
on every test sentence.

Test: `TestRetokenizeVsFunASR::test_retokenize_strictly_improves_word_count`

### Sub-claim 2: Retokenize > Tencent alone

**Verdict: UNVERIFIABLE** (no real Tencent data)

Spencer claims Tencent returns pre-segmented words, but our test data only
shows single-character words. If Tencent already segments correctly, retokenize
is unnecessary — and potentially harmful if it re-segments correct boundaries
differently.

We verified retokenize is at least **non-harmful** on already-segmented input:
`['佢哋', '好', '鍾意', '食嘢']` → `['佢哋', '好', '鍾意', '食嘢']` (identity).

Test: `TestRetokenizeVsTencent::test_retokenize_on_already_segmented_input_is_identity`

### Sub-claim 3: Stanza has a "special model trained just for this task"

**Verdict: DISPROVED for Cantonese**

Stanza's `zh` model is trained on the **Chinese Treebank (Mandarin formal text)**,
not on Cantonese. It produces actively wrong segmentations on Cantonese text:

```
Input:       佢哋好鍾意食嘢 (they really like eating stuff)

Stanza (zh): ['佢哋', '好', '鍾', '意食', '嘢']
                                    ^^^^
                              WRONG: '意食' is nonsense
                              (merged 意 from 鍾意 with 食 from 食嘢)

PyCantonese: ['佢哋', '好', '鍾意', '食嘢']
                           CORRECT   CORRECT
```

Stanza got 佢哋 right (likely because it appears in Mandarin too) but garbled
鍾意 (Cantonese "to like") and 食嘢 (Cantonese "eat stuff") — these are
Cantonese-specific words not in the Mandarin training data.

Test: `TestStanzaVsPyCantonese::test_stanza_misses_cantonese_specific_words`

## Conclusion

Houjun's claim is:
- **Correct for FunASR** (any segmenter > no segmenter)
- **Unverifiable for Tencent** (need real data)
- **Wrong for Cantonese word segmentation** (Stanza is not the right tool;
  PyCantonese is, as Sebastian originally suggested)
- **Correct for Mandarin** (Stanza's zh tokenizer does segment Mandarin,
  though imperfectly)

The implementation uses PyCantonese for Cantonese and Stanza for Mandarin —
the right tool for each language.

## Tests

All in `batchalign/tests/pipelines/morphosyntax/test_retokenize_vs_engines.py`:
- `TestRetokenizeVsFunASR` (3 tests) — all pass
- `TestRetokenizeVsTencent` (2 tests) — all pass
- `TestStanzaVsPyCantonese` (1 golden test) — passes, confirms Stanza garbles Cantonese

Commit: `98678105` on batchalign3 main.
