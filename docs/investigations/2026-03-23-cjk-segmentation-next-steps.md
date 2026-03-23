# CJK Word Segmentation: Unverified Claims and Next Steps

**Status:** Current
**Last updated:** 2026-03-23 11:50 EDT

## What We Verified

| Claim | Method | Result |
|-------|--------|--------|
| FunASR outputs per-character for Cantonese | Real `cantonese_char_tokens()` call | **Confirmed** — every CJK character becomes a separate token |
| PyCantonese groups Cantonese words | Real `pycantonese.segment()` on known compounds | **Confirmed** — `佢哋`, `鍾意`, `故事` grouped correctly |
| Stanza segments Mandarin words | Real Stanza `zh` pipeline with `pretokenized=False` | **Partially confirmed** — groups `商店` but splits `东西` |
| Tencent preserves word boundaries | Bridge code test with synthetic multi-char `Word` | **Bridge works**, but no real Tencent output tested |

## What We Have NOT Verified

### 1. Tencent ASR actually returns multi-character Cantonese words

**Claim:** Spencer said "Tencent ASR does word segmentation/tokenization."

**Gap:** All our test data for Tencent uses single-character words. We verified
the *bridge* preserves multi-char words, but we don't have evidence that Tencent
*actually produces* them for Cantonese. Spencer may be right for Mandarin but
wrong for Cantonese, or Tencent's segmentation may vary by model version.

**Next step:** Run a real Cantonese audio file through Tencent ASR on net and
examine the raw `ResultDetail` output. Look for multi-character `Word` entries.
Save a representative sample as a test fixture.

```bash
# On net, with Tencent credentials configured:
# 1. Process a Cantonese audio file through Tencent
# 2. Dump the raw ResultDetail JSON before Rust bridge processing
# 3. Check if Words array contains multi-character entries
```

### 2. Paraformer actually outputs per-character for Mandarin

**Claim:** Angel said Paraformer doesn't do word segmentation for Mandarin.
Houjun partially disagreed: "I think the mandarin models do at least attempt
some degree of tokenization."

**Gap:** We don't use Paraformer directly in batchalign3 — it's an external
model. But if Angel's team uses it and feeds output to batchalign3, we should
understand what they're getting.

**Next step:** Ask Angel's team for a sample Paraformer transcript (raw output
before any post-processing) so we can verify whether it's truly per-character.

### 3. Stanza's segmentation quality on child Cantonese/Mandarin speech

**Gap:** Our golden test uses a single adult-register Mandarin sentence. Child
speech has different vocabulary, shorter utterances, and more disfluencies.
Stanza may perform differently on child language data.

**Next step:** Create a golden test with sentences from actual TalkBank child
Cantonese/Mandarin corpora (if available) and verify segmentation quality.

### 4. PyCantonese's coverage of child Cantonese vocabulary

**Gap:** PyCantonese's dictionary is based on adult Cantonese corpora. Child
speech may use words or word forms not in the dictionary (baby talk, simplified
forms, code-mixed Cantonese-English).

**Next step:** Extract the 50 most frequent multi-character tokens from an
existing Cantonese child corpus (e.g., HKU aphasia data used for our
`yue_hku_clip.mp3` fixture) and verify PyCantonese segments them correctly.

### 5. End-to-end retokenize pipeline on real CHAT files

**Gap:** We tested the individual components (PyCantonese segmentation, Stanza
tokenization, Rust retokenize module, cache key differentiation) but have not
run the full `morphotag --retokenize` pipeline on a real Cantonese or Mandarin
CHAT file. The components may interact in unexpected ways (e.g., retokenize
module failing on CJK character patterns we haven't tested).

**Next step:** On net, run the full pipeline on a real corpus file:

```bash
# Cantonese end-to-end
batchalign3 --no-open-dashboard morphotag --retokenize \
    ../data/aphasia-data/Cantonese/Protocol/HKU/PWA/A023.cha \
    -o /tmp/yue_retok_test/ --lang yue -v

# Mandarin end-to-end (if we have Mandarin CHAT files)
batchalign3 --no-open-dashboard morphotag --retokenize \
    <mandarin_file>.cha \
    -o /tmp/cmn_retok_test/ --lang cmn -v
```

### 6. Our Cantonese per-char warning threshold

**Assumption:** We use 80% single-CJK-character words as the threshold for
emitting the per-char warning. This number was chosen without empirical basis.

**Next step:** Sample word lengths from real Cantonese corpora to determine:
- What % of words in properly segmented Cantonese text are single-character?
- What % of words in per-character ASR output are single-character? (should be ~100%)
- Is 80% the right threshold, or would 90% or 95% be more appropriate?

## Priority Order

1. **End-to-end test on real CHAT** (item 5) — highest risk, tests component integration
2. **Tencent real output** (item 1) — directly affects user documentation claims
3. **Child speech quality** (items 3+4) — affects the PolyU team's actual use case
4. **Warning threshold** (item 6) — low risk, easy to adjust later
5. **Paraformer verification** (item 2) — depends on Angel's team providing data
