# Bug Report: Tree-Sitter Parser Joins CJK Characters Differently With Bullet Timestamps

**Status:** Needs talkbank-tools investigation
**Last updated:** 2026-03-23 18:45 EDT
**Filed by:** Franklin (from batchalign3 retokenize investigation)

## Summary

The tree-sitter parser produces different word boundaries for the same Cantonese utterance depending on whether bullet timestamps are present. Adjacent single CJK characters are joined into multi-character tokens when bullets exist but remain separate without bullets. This causes a word count mismatch in batchalign3's morphotag --retokenize pipeline.

## Reproduction

Same utterance, two fixtures:

**Without bullet:**
```
*PAR0:	呢 度 <下 次> [/] 下 次 食 飯 啦 .
```
Parser produces: `["呢", "度", "下", "次", "食", "飯", "啦"]` — 7 words (retrace skipped for MOR)

**With bullet:**
```
*PAR0:	呢 度 <下 次> [/] 下 次 食 飯 啦 . ␕102450_112560␕
```
(where ␕ is U+0015 NAK)

Parser appears to produce: `["呢", "度", "食飯", "啦", "飯", "啦"]` — 6 words (based on error message showing `食飯` joined and `下次` joined in retrace group)

## Evidence

Error from batchalign3 end-to-end test:
```
MOR item count (5) does not match alignable word count (6) in utterance:
*PAR0: 呢 度 <下次> [/] 食飯 啦 飯 啦 . 102450_112560
```

Note: the error message shows `<下次>` (joined) and `食飯` (joined) — the parser joined adjacent CJK characters. The source file has spaces between all characters.

## Source Data

MOST corpus file: `data/childes-other-data/Chinese/Cantonese/MOST/10002/40415b.cha` line 46.

This is ASR output from batchalign 0.7.22 FunASR — per-character Cantonese with bullet timestamps on every utterance.

## Impact on batchalign3

The word count discrepancy breaks the morphotag --retokenize pipeline:

1. Rust extracts N words from the parsed AST (e.g., 6 after joining)
2. Rust sends N words to Python worker as a batch payload
3. Python sends N words to Stanza, gets N MOR items back
4. Rust tries to inject N MOR items into the AST
5. But the AST has a different alignable word count because `retokenize_utterance` rewrites the AST and `inject_morphosyntax` re-counts — if the counts drift, injection fails

The `_segment_cantonese` function in Python also behaves differently based on whether words are single-char or multi-char (it only re-segments all-single-char input). So the joining changes the segmentation path.

## Questions for talkbank-tools

1. **Is joining adjacent CJK characters with bullets intentional?** The grammar may have a rule that coalesces adjacent characters into a single word token when followed by a bullet timestamp.

2. **Should the parser preserve space-separated characters as separate words?** For batchalign3, each space-separated token on the main tier should be a separate word in the AST, regardless of bullets. Joining them changes the word count.

3. **Does the bullet timestamp change how the main tier content is parsed?** The main tier content before the bullet should parse identically whether or not a bullet follows.

## Suggested Fix

If the joining is unintentional: fix the grammar/parser to preserve space-separated CJK characters as separate words regardless of bullet presence.

If the joining is intentional: batchalign3 needs to know the "original" word count (before joining) to align Stanza output correctly. This may require a different extraction strategy.

## Test Fixtures

Both fixtures are in `batchalign3/test-fixtures/retok_yue_retrace.cha`. The current fixture has bullets. To reproduce the non-bullet behavior, remove the `␕102450_112560␕` from the utterance line.

## Related Investigation

Full investigation log: `docs/investigations/2026-03-23-session-log.md`
String hacking audit: `docs/investigations/2026-03-23-string-hacking-audit.md`
Debugging proposal: `docs/investigations/2026-03-23-debugging-capability-proposal.md`
