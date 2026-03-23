# String Hacking Audit: batchalign3 Python Code

**Status:** Historical — all findings fixed in commit 3c03fe3b
**Last updated:** 2026-03-23 19:50 EDT

## Purpose

After the `_segment_cantonese` bug (naive `"".join(words)` merging across
word boundaries), audit all string manipulation in Python inference/worker
code for similar data-losing patterns.

## Findings

### Fixed
| Location | Pattern | Risk | Status |
|----------|---------|------|--------|
| `morphosyntax.py:116` | `"".join(words)` in `_segment_cantonese` | Merged words across boundaries | **Fixed** — now checks for multi-char tokens |

### Must Fix
| Location | Pattern | Risk | Status |
|----------|---------|------|--------|
| `morphosyntax.py:268` | `.replace("(", "").replace(")", "")` | **Silently drops bare parenthesis words**, reducing word count. If `words = ["(", "word", ")"]`, result is `["word"]` (3→1). Rust `cleaned_text()` already strips CHAT parenthetical notation — the Python strip is redundant and dangerous. **HYPOTHESIS: This may be the retrace bug** — if the extractor includes `(` or `)` artifacts from retrace/group parsing, Python drops them silently, causing the MOR count mismatch (6→5). | **Remove** — blocked on Rust build (talkbank-tools API change) |
| `morphosyntax.py:309` | `"".join(w) for w in word_lists` | Joins Mandarin retokenize words without spaces | Intentional for CJK. But if non-CJK words in a mixed-language utterance, would lose boundaries. |
| `fa.py:345` | `.replace("_", " ")` | Replaces underscores with spaces in FA words | Could affect compound words. Need to check if `_` is used as CHAT compound marker. |

### Safe
| Location | Pattern | Notes |
|----------|---------|-------|
| `utseg.py:213` | `" ".join(words)` | Space-separated join — standard for Stanza pretokenized input |
| `_protocol.py:67` | `.strip()` on raw lines | IPC line parsing — stripping whitespace is correct |
| `_common.py:96,119` | `.strip()` on config values | Config parsing — correct |
| `_funaudio_asr.py:161` | `.strip()` on element values | Filtering empty ASR tokens — correct |
| `translate.py:60` | `.strip()` check | Skipping empty translation items — correct |

## Recommendations

1. The `replace("(", "")` on line 268 should be done by the Rust extractor
   (which already strips CHAT notation), not by Python string manipulation.
   This would prevent text mismatch between what Rust extracts and what
   Python processes.

2. The Mandarin `"".join(w)` on line 309 should verify all words are CJK
   before joining without spaces — mixed-language utterances with Latin
   words would lose boundaries.
