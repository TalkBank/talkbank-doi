# Morphotag Output Divergences: Rust (align) vs Python (master)

**Date:** 2026-02-16
**Test:** `uv run batchalign-next --override-cache morphotag --in-place` on 10 files from `~/data/aphasia-data`
**Command:** `head -10 ./morphotag-file-list.txt`

---

## Summary

10 files were re-processed with the Rust align branch morphotag. Three categories of **intentional improvements** and one **bug** (now fixed) were found.

| Category | Count | Status |
|----------|-------|--------|
| `@u` words: POS corrected to `u\|` | ~48 lines | Correct improvement |
| `@l` words: POS corrected to `l\|` | ~6 lines | Correct improvement |
| %gra ROOT head index | ~190 lines | Bug (fixed) |
| MWT expansion (e.g. `coulda`) | ~2 lines | Correct improvement |

---

## Category 1: `@u` Words (Unintelligible/Neologistic)

Words marked with the CHAT `@u` suffix are unintelligible or neologistic. Their POS should be `u|`, not a real lexical category.

**Python master (WRONG):**
```
*PAR: keke@u keke@u keke@u keke@u . 219624_221927
%mor: propn|keke propn|keke propn|keke propn|keke .
```

**Rust align (CORRECT):**
```
*PAR: keke@u keke@u keke@u keke@u . 219624_221927
%mor: u|keke u|keke u|keke u|keke .
```

Python master treats neologisms like `keke@u` as proper nouns (`propn|`). Stanza sees an unknown word and guesses PROPN. Our Rust mapping correctly detects the `@u` marker and overrides with `u|`.

**Impact:** ~48 changed lines across the 10 files. All aphasia data with jargon/neologism is affected.

---

## Category 2: `@l` Words (Letters)

Words marked with `@l` represent individual letters being spelled out. Their POS should be `l|`.

**Python master (WRONG):**
```
*INV: you wrote j@l . 242969_245784
%mor: pron|you-Prs-Nom-S2 verb|write-Fin-Ind-Past-S2-irr propn|j-Acc .
```

**Rust align (CORRECT):**
```
*INV: you wrote j@l . 242969_245784
%mor: pron|you-Prs-Nom-S2 verb|write-Fin-Ind-Past-S2-irr l|j-Acc .
```

Another example:
```
*INV: it starts with an m@l , okay . 281566_283856
- %mor: ... noun|m cm|cm intj|okay .        ← Python: noun
+ %mor: ... l|m cm|cm intj|okay .            ← Rust: l (letter)
```

**Impact:** ~6 changed lines. Affects any file that discusses spelling or individual letters.

---

## Category 3: %gra ROOT Head Index (BUG — NOW FIXED)

### The bug

In Universal Dependencies and CHAT %gra, the ROOT relation should have `head=0` (pointing to the virtual root node):

```
%gra: 1|3|AUX 2|3|NSUBJ 3|0|ROOT 4|5|DET 5|3|OBJ 6|3|PUNCT
                          ^^^^^
                          head=0 (correct)
```

**Python master output:**
```
%gra: 1|3|AUX 2|3|NSUBJ 3|5|ROOT 4|5|DET 5|3|OBJ 6|3|PUNCT
                          ^^^^^
                          head=5 (WRONG — Python negative indexing bug)
```

Python master's `actual_indicies[elem[1]-1]` evaluates to `actual_indicies[-1]` when `elem[1]` is 0 (ROOT). Python's negative indexing wraps around to the last element, producing an arbitrary non-zero head. This is the root cause of the circular dependency bugs documented in `python-gra-generation-analysis.md`.

**Rust align output (BEFORE fix):**
```
%gra: 1|3|AUX 2|3|NSUBJ 3|3|ROOT 4|5|DET 5|3|OBJ 6|3|PUNCT
                          ^^^^^
                          head=3 (WRONG — self-referencing)
```

Our Rust code had `chat_idx` instead of `0` for ROOT relations, producing self-referencing heads (`X|X|ROOT`). While not as bad as Python's circular dependencies (self-referencing is at least a valid tree), it's still wrong per UD conventions.

**Rust align output (AFTER fix):**
```
%gra: 1|3|AUX 2|3|NSUBJ 3|0|ROOT 4|5|DET 5|3|OBJ 6|3|PUNCT
                          ^^^^^
                          head=0 (CORRECT)
```

### Fix applied

**File:** `~/talkbank-utils/rust/crates/batchalign-core/src/nlp/mapping.rs:178-179`

```rust
// BEFORE (wrong):
let head_chat_idx = if ud.head == 0 {
    chat_idx // ROOT points to self in TalkBank convention
// AFTER (correct):
let head_chat_idx = if ud.head == 0 {
    0 // ROOT head=0 in both UD and CHAT %gra convention
```

### Validation was already correct

The existing validation code (`tier.rs:205`) already handled both conventions:
```rust
if rel.head == 0 || rel.head == rel.index {
    roots.push(rel.index);
}
```
No changes needed to validation — it accepts `head=0` (UD standard) and `head=self` (legacy), so the fix is backwards-compatible.

### Impact

Every single ROOT relation in every file processed by `batchalign-next morphotag` was affected. In the 10-file test: 190 %gra lines changed. All ROOT relations were `X|X|ROOT` (self-referencing) and should be `X|0|ROOT`.

---

## Category 4: MWT Expansion

Multi-word tokens are now properly expanded with correct POS tags.

**Python master:**
```
%mor: pron|you-Prs-Nom-S2 aux|coulda-Fin-Ind-Pres-S2 verb|do-Part-Past-S-irr ...
```

**Rust align:**
```
%mor: pron|you-Prs-Nom-S2 aux|coul-Fin-S~aux|da-Inf-S verb|do-Part-Past-S-irr ...
```

This is correct — "coulda" is a contraction of "could" + "have" (spoken as "da"), properly represented as a multi-word token with clitics.

**Impact:** Occasional, only affects contracted/informal speech forms.

---

## Comparison Summary

| Aspect | Python master | Rust align (after fix) |
|--------|--------------|----------------------|
| `@u` POS | `propn\|` (wrong) | `u\|` (correct) |
| `@l` POS | `propn\|` or `noun\|` (wrong) | `l\|` (correct) |
| ROOT head | Arbitrary word index (Python -1 indexing bug) | `0` (correct UD convention) |
| MWT handling | Single opaque token | Properly expanded clitics |
| Circular dependencies | 87.5% of files (documented) | **Impossible** (validated) |

---

## Verification

All changes verified by:
1. Rust tests: 196 passed, 0 failed (`cargo test -p batchalign-core`)
2. Python tests: 6 passed (`pytest test_handle_pipeline.py`)
3. Pre-serialization validation gate: no warnings on clean output
4. Real corpus data: 10 aphasia files processed successfully
