# Cantonese --retokenize Known Bugs

**Status:** Active tracking
**Last updated:** 2026-03-23 19:30 EDT

## Fixed

### 1. Retrace AnnotatedGroup word_counter desync
- **Symptom:** "MOR item count (5) does not match alignable word count (6)"
- **Source:** MOST 10002/40415b.cha, utterance with `<下 次> [/]`
- **Root cause:** `rebuild_content` recursed into retrace AnnotatedGroup, incrementing word_counter for words excluded from MOR extraction
- **Fix:** Skip retrace AnnotatedGroup during rebuild (check `is_retrace_annotated_group`)
- **Commit:** `c50cbd6b`

### 2. Retrace AnnotatedWord word_counter desync
- **Symptom:** "MOR item count (1) does not match alignable word count (2)"
- **Source:** MOST 10002/40415b.cha, utterance with `拜拜 [/] 拜 [/] 拜 拜`
- **Root cause:** Same as above but for `AnnotatedWord` with retrace annotation
- **Fix:** Skip retrace AnnotatedWord during rebuild (check `is_retrace_annotated_word`)
- **Commit:** `284f522e`

### 3. _segment_cantonese joined across word boundaries
- **Symptom:** `啦飯啦` (nonsense) appeared in segmented output
- **Root cause:** Naive `"".join(words)` concatenated all words, PyCantonese merged across boundaries
- **Fix:** Only re-segment when ALL CJK tokens are single characters
- **Commit:** `c167f83d`

### 4. Parenthesis stripping dropped bare paren words
- **Symptom:** Potential word count reduction (not confirmed as retrace root cause)
- **Root cause:** `.replace("(", "").replace(")", "")` on line 268 silently drops bare paren words
- **Fix:** Removed — Rust cleaned_text() handles CHAT notation
- **Commit:** `db8407ea`

## Open

### 5. `[- zho]` language pre-code causes word count mismatch
- **Symptom:** "MOR item count (7) does not match alignable word count (8)"
- **Source:** MOST 10011/40412d.cha, utterance with `[- zho]`
- **Root cause:** Unknown — `[- zho]` is a language pre-code, not a retrace. The extractor or rebuild may be counting it differently from how Stanza sees it.
- **Status:** Not investigated. Files without `[- zho]` work correctly.
- **Workaround:** Use morphotag without --retokenize on files with language pre-codes.

### 6. Tree-sitter parser joins some adjacent CJK characters
- **Symptom:** Error messages show `食飯`, `下次`, `最危` as joined words when source has spaces
- **Root cause:** Unknown — tree-sitter parse produces 9 separate standalone_word nodes (confirmed by talkbank-tools investigation), but something in batchalign3's extraction or injection path joins them
- **Status:** Partially investigated. The joining doesn't cause failures by itself (the retokenize mapping handles N:1 merges). But it means the AST doesn't perfectly preserve the original tokenization.
- **Impact:** Low — merged words get correct POS from Stanza/PyCantonese.

## Test Coverage

| Bug | Rust Test | Python Test | E2E Test |
|-----|-----------|-------------|----------|
| #1 (AnnotatedGroup) | `test_cjk_retokenize_retrace_with_n1_merges` | — | `test_morphotag_retokenize_with_retrace_succeeds` |
| #2 (AnnotatedWord) | (covered by MOST e2e) | — | (MOST corpus succeeds) |
| #3 (segment join) | — | `test_segment_cantonese_preserves_existing_multichar` | — |
| #4 (paren strip) | — | `test_paren_strip_reduces_word_count` | — |
| #5 (pre-code) | — | — | — (needs test) |
| #6 (CJK joining) | — | — | — (needs investigation) |
