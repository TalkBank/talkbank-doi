# %wor Tier Terminator Bullet Behavior

## Summary

The terminator (`.`, `?`, `!`) in a %wor tier must be the **final token** with **no timing bullet after it**. This document clarifies the correct behavior, the source of E342 errors in the corpus, and how both Python and Rust implementations handle %wor generation.

## Correct Format

```
*CHI: hello world . 100_200
%wor: hello •110_120• world •130_140• .
```

The terminator `.` appears at the end with **no bullet**. Word-level timing bullets precede each word. The utterance-level timing (`100_200`) appears only on the main tier, not in %wor.

## Incorrect Format (E342 Error)

```
*CHI: hello world . 100_200
%wor: hello •110_120• world •130_140• . •100_200•
                                        ^^^^^^^^^^ WRONG!
```

The utterance-level timing bullet appears **after** the terminator. This violates the %wor tier grammar and triggers E342 "Missing required element" errors.

## E342 Errors in Corpus (902 Files)

Validation of 99,000+ files found **902 files** with trailing bullets after terminators:

| Collection | Files | Notes |
|------------|------:|-------|
| dementia-data | 297 | Pitt-orig/Dementia, Pitt-orig/Control |
| aphasia-data | 209 | Kurland, NEURAL-2, SCALE |
| childes-data | 179 | Various corpora |
| tbi-data | 75 | — |
| slabank-data | 58 | — |
| rhd-data | 44 | — |
| ca-data | 27 | — |
| biling-data | 12 | — |
| fluency-data | 1 | — |
| **Total** | **902** | |

### Source of E342 Errors

These errors are **pre-existing legacy data** from:
- Very old CLAN alignment tools (pre-2020)
- Ancient batchalign versions (if any existed before Nov 2023)

**NOT from**:
- ❌ Current Python batchalign (master branch)
- ❌ Current Rust batchalign3 (batchalign3)

Both current implementations generate the correct format.

## Python Implementation (master branch)

### Code: `batchalign/formats/chat/generator.py`

```python
def generate_chat_utterance(utterance, ...):
    wor_elems = []

    for i in utterance.content:
        if i.time:
            # Each word gets: word •start_end•
            wor_elems.append(f"{i.text} •{i.time[0]}_{i.time[1]}•")
        else:
            # Words without timing (including terminator)
            wor_elems.append(i.text)

    result.append("%wor:\t" + " ".join(wor_elems))
```

**Behavior**: The terminator is added as plain text (no timing) because it's not a word that should be aligned. The utterance-level timing stays on the main tier only.

**Result**: ✅ Generates correct format

## Rust Implementation (batchalign3)

### Original Bug (Fixed 2026-02-14)

The Rust function `generate_wor_tier()` in `talkbank-model` originally **copied** the utterance-level bullet from the main tier:

```rust
// WRONG - before fix:
WorTier {
    items: wor_items,
    bullet: self.content.bullet.clone(),  // ← copied utterance bullet!
    span: Span::DUMMY,
}
```

When serialized, this produced:
```
%wor: word1 •110_120• word2 •130_140• . •100_200•
                                        ^^^^^^^^^^ from bullet field
```

### The Fix

Changed to:
```rust
// CORRECT - after fix:
WorTier {
    items: wor_items,
    bullet: None,  // ← no utterance bullet in %wor
    span: Span::DUMMY,
}
```

**Result**: ✅ Now generates correct format

### Impact of the Bug

The Rust bug affected **chained workflows**:
- `align → morphotag` failed because strict parser rejected %wor with trailing bullet
- Round-trip tests passed (lenient parser recovered)
- Only discovered when trying to run morphotag on freshly aligned files

**Fixed in commit**: c5e653d3 (batchalign3)
**Documented in**: `docs/wor-tier-bullet-bug.md`

## Validation

The validator now correctly identifies E342 errors:

```rust
// In wor_tier_body grammar:
text_with_bullets terminator
                  ^^^^^^^^^^ must be final token, no bullet after
```

Files with trailing bullets fail strict parsing and must be re-aligned.

## Remediation

The 902 affected files need to be re-aligned:

```bash
# Using server mode (recommended):
batchalign3 align --server http://server:8000 \
  --file-list results/wor_errors/all_wor_files.txt \
  input/ output/
```

Both Python (master) and Rust (align) implementations will generate correct %wor without trailing bullets.

## Key Takeaways

1. **Correct format**: Terminator is final token, no bullet after it
2. **E342 errors**: Legacy data from old tools, not current batchalign
3. **Python (master)**: Always generated correct format
4. **Rust (align)**: Had a bug (now fixed) that copied utterance bullet
5. **Fix**: Re-align affected files with either master or batchalign3

## References

- E342 error spec: `~/talkbank-utils/spec/errors/E342_auto.md`
- Rust fix: `docs/wor-tier-bullet-bug.md`
- Error audit: `results/wor_errors/summary.md`
- CHAT manual: https://talkbank.org/0info/manuals/CHAT.pdf
