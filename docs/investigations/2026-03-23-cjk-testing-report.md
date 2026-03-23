# CJK Word Segmentation Testing Report

**Status:** Current
**Last updated:** 2026-03-23 12:30 EDT

## What Was Tested

### 1. End-to-End `morphotag --retokenize` on Cantonese

**Input:** Synthetic per-character CHAT file (3 utterances):
```
*PAR:	佢 哋 好 鍾 意 食 嘢 .
*PAR:	我 想 去 買 故 事 書 .
*PAR:	你 知 唔 知 道 .
```

**Output (successful):**
```
*PAR:	佢哋 好 鍾意 食嘢 .
%mor:	propn|佢哋 adv|好 verb|鍾意-Inf-S verb|食嘢-Inf-S .
%gra:	1|4|NSUBJ 2|4|ADVMOD 3|4|ADVCL 4|0|ROOT 5|4|PUNCT
*PAR:	我 想 去 買 故事 書 .
%mor:	pron|我-Int-S1 aux|想-Inf-S verb|去-Inf-S verb|買-Inf-S noun|故事 part|書 .
%gra:	1|3|NSUBJ 2|3|AUX 3|0|ROOT 4|3|XCOMP 5|6|COMPOUND 6|4|OBJ 7|3|PUNCT
*PAR:	你 知 唔 知道 .
%mor:	pron|你-Int-S2 verb|知-Inf-S verb|唔-Inf-S verb|知道-Inf-S .
%gra:	1|2|NSUBJ 2|0|ROOT 3|4|ADVCL 4|2|XCOMP 5|2|PUNCT
```

**Verified groupings:** 佢哋 (they), 鍾意 (like), 食嘢 (eat stuff), 故事 (story),
知道 (know), 好睇 (good to watch)

### 2. PyCantonese Corpus Quality

**Corpus:** CHILDES CHCC Winston Cantonese (child bilingual speech)
- 2,280 unique pure-CJK words extracted
- 917 single-character, 1,363 multi-character

**Results:**
- **91% of multi-character words preserved** as single tokens by PyCantonese
- The 9% that split are mostly multi-word phrases transcribed as single tokens
  (book titles like `貓小姐的露水茶`, idiomatic expressions like `大驚小怪`)
- These splits are linguistically correct — PyCantonese is segmenting phrases
  into constituent words

**Key words all correct:** 佢哋, 鍾意, 故事, 媽媽, 爸爸, 知道, 多謝, 聖誕, 飛機, 蛋糕

### 3. Stanza Mandarin Tokenizer Quality

**Input:** `我去商店买东西` (I go to the store to buy things)
**Output:** `['我', '去', '商店', '买', '东', '西']`

- **商店** (store) correctly grouped ✓
- **东西** (things) split into 东+西 — known Stanza limitation for ambiguous compounds
- 7 chars → 6 words: still an improvement over per-character

### 4. Claim Verification Summary

| Claim | Verified | Method |
|-------|----------|--------|
| FunASR outputs per-character | ✓ | `cantonese_char_tokens()` real call |
| Tencent preserves word boundaries | Partial | Bridge tested, no real Tencent output |
| PyCantonese segments correctly | ✓ | Real `segment()` + corpus validation |
| Stanza segments Mandarin | ✓ | Real Stanza zh model (golden test) |

### 5. Per-Character Warning

**Issue found:** The warning fires in `mod.rs` (single-file incremental path)
but the CLI actually dispatches through `batch.rs` (batch path). Fixed by
adding the warning to `batch.rs`.

**Remaining issue:** The warning uses `tracing::warn!` which fires in the
daemon process, not the CLI process. Users won't see it in their terminal.
Surfacing it through SSE events or job results requires a larger change.

## What Was NOT Tested

1. **Real Tencent ASR output** — need credentials and real audio on net
2. **Real Paraformer output** — need model download or sample from Angel's team
3. **End-to-end on complex CHAT** — the A023.cha corpus file has CHAT syntax
   (parenthesized groups, code markers) that a naive per-char conversion breaks.
   Real FunASR output wouldn't have these issues since it starts from audio.
4. **The warning reaching the user's terminal** — daemon architecture issue

## Commits

1. `35a95473` — feat: CJK word segmentation (116 files)
2. `f0a72774` — docs: language documentation consolidation (9 files)
3. `e16a62eb` — test: corpus quality + warning fix (2 files)
