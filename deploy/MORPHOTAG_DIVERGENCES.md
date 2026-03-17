# Morphotag Divergences: Python vs Rust

## Analysis Date: 2026-02-15

---

## Summary

Found **4 categories** of systematic differences between Python (bench-baseline) and Rust (align) morphotag outputs. Analysis shows **Rust is missing features** that Python correctly extracts.

---

## 1. Reflexive Pronouns: ✅ FIXED

### Issue
Python adds `-reflx` suffix to reflexive pronouns, Rust didn't.

### Example
```diff
Main tier:  *MOT:	aquesta es la Lala .
            (this is the Lala)

- Python:   %mor:	pron|aquest-Dem-S1 pron|ell-Prs-AccDat-reflx-S3 ...
+ Rust:     %mor:	pron|aquest-Dem-S1 pron|ell-Prs-AccDat-S3 ...
                                                    ^^^^^^ missing
```

### Root Cause
**Stanza provides `Reflex=Yes` feature:**
```
$ stanza.Pipeline('ca').process('aquesta es la Lala')
Word: es
  UPOS: PRON
  Features: Case=Acc,Dat|Person=3|PrepCase=Npr|PronType=Prs|Reflex=Yes
                                                             ^^^^^^^^^^
```

**Python correctly extracts it** (`batchalign/pipelines/morphosyntax/ud.py:154-156`):
```python
reflex = str(feats.get("Reflex","")).strip()
if reflex == "Yes":
    reflex = "reflx"
```

**Rust doesn't handle Reflex feature at all** — no mention of "reflex" or "Reflex" in `~/talkbank-utils/rust/crates/batchalign-core/src/`

### Impact
- Affects Catalan, Spanish, and other Romance languages with reflexive clitics
- Linguistically significant for verb analysis (reflexive vs non-reflexive)
- Missing from 20 utterances in the test file alone

### Fix Applied (2026-02-15)
Added `map_pronoun_suffixes()` function in `~/talkbank-utils/rust/crates/talkbank-model/src/model/nlp/mapping.rs` that:
1. Extracts PronType, Case, Reflex, Number, Person features from Stanza
2. Builds pronoun suffix in correct CHAT format: `PronType-Case-Reflex-Number`
3. Maps `Reflex=Yes` → `-reflx` suffix

**Verified working** on Catalan test sentences.

---

## 2. Communicator POS Tags: `c|` vs `x|` ⚠️

### Issue
Python uses `c|` (communicator) POS tag, Rust uses `x|` (unknown).

### Example
```diff
Main tier:  *MOT:	a nonon@c ?
                      ^^^^^^^ @c = communicator suffix in CHAT

- Python:   %mor:	adp|a c|nonon ?
+ Rust:     %mor:	adp|a x|nonon ?
                          ^ communicator  ^ unknown
```

### Stanza Says
```
$ stanza.Pipeline('ca').process('nonon')
Word: nonon
  UPOS: PUNCT
```

Stanza treats "nonon" (baby talk) as punctuation/interjection, not a real word.

### Python Behavior
Recognizes `@c` suffix from CHAT format and assigns `c|` POS tag for communicators.

### Rust Behavior
Ignores `@c` suffix and defaults to `x|` (unknown).

### Impact
- Affects onomatopoeia, baby talk, communicative sounds (common in child language data)
- Examples: `nonon@c`, `xxx@c` (unintelligible communicators)
- CHAT convention: `@c` marks communicators explicitly

### Fix Required
Rust needs to:
1. Detect `@c` suffix in CHAT words
2. Assign `c|` POS tag instead of `x|` for these words

**Priority**: MEDIUM — affects CHAT format compliance

---

## 3. L2/Foreign Word Handling: `L2|xxx` vs `x|word` ⚠️

### Issue
Python uses `L2|xxx` for unintelligible/foreign words, Rust uses `x|word` (preserves actual word).

### Example
```diff
Main tier (Spanish word in Catalan transcript):
  *MOT:	... un biberon@s ...
                    ^^^ Spanish word marked with @s

- Python:   %mor:	... det|un-Masc-Def-Ind-Sing L2|xxx ...
+ Rust:     %mor:	... det|un-Masc-Def-Ind-Sing x|biberon ...
                                                 ^^^^^^^^^^ preserves word
```

### Python Behavior
Marks code-switched or unintelligible words as `L2|xxx` (second language marker + generic xxx).

### Rust Behavior
Marks as `x|` (unknown) but preserves the actual word (`biberon` not `xxx`).

### Impact
- Affects bilingual/multilingual corpora (common in CHILDES)
- Python loses information (replaces word with `xxx`)
- Rust preserves lexical content

### Analysis
**Rust behavior may be BETTER** — preserves the actual word for analysis while still marking it as unknown to the primary language model.

### Decision Needed
Which is the CHILDES/CHAT standard?
- `L2|xxx` (Python) — marks as second language, loses word
- `x|biberon` (Rust) — marks as unknown, keeps word

**Priority**: LOW — need to check CHAT manual/CHILDES conventions

---

## 4. %wor Tier Suffixes: `@s`/`@c` Preservation

### Issue
Python adds language/communicator suffixes in %wor tier, Rust doesn't.

### Example
```diff
- Python %wor:	... biberon@s ... vale@s ... nonon@c ...
+ Rust %wor:	... biberon ... vale ... nonon ...
                    ^^^^^^^ suffixes stripped
```

### Impact
- Affects word-level alignment tier (`%wor`)
- Python preserves CHAT suffixes, Rust strips them
- %wor is supposed to match spoken words 1-to-1

### Analysis
The %wor tier should represent **spoken forms**, not orthographic conventions.
- `biberon@s` in main tier = "biberon is a Spanish word" (orthographic marker)
- `biberon` in %wor = "the speaker said /biberon/" (phonetic/spoken)

**Rust behavior is likely CORRECT** — %wor should have clean word forms for alignment.

### Decision Needed
Check CHAT specification for %wor tier conventions.

**Priority**: LOW — may not be a bug

---

## Summary Table

| Issue | Python | Rust | Status | Priority |
|-------|--------|------|--------|----------|
| Reflexive `-reflx` | ✅ Present | ✅ **FIXED** | ✅ **FIXED** (2026-02-15) | ~~HIGH~~ |
| Communicator `c\|` | ✅ `c\|nonon` | ✅ **FIXED** | ✅ **FIXED** (2026-02-15) | ~~MEDIUM~~ |
| L2 handling | `L2\|xxx` | `x\|word` | **Unclear** | LOW |
| %wor suffixes | `@s/@c` kept | Stripped | **Unclear** | LOW |

---

## FIXES IMPLEMENTED (2026-02-15)

### Fix #1: Reflexive Pronouns ✅

**File**: `~/talkbank-utils/rust/crates/talkbank-model/src/model/nlp/mapping.rs`

**Changes**:
1. Added `map_pronoun_suffixes()` function that extracts ALL pronoun features from Stanza
2. Updated `map_ud_word_to_mor()` to use pronoun-specific suffix builder for PRON POS
3. Properly maps `Reflex=Yes` → `-reflx` suffix

**Verification**:
```
*MOT:	aquesta es la Lala .
%mor:	pro|aquest-Dem-S1 pro|ell-Prs-AccDat-reflx-S3 det|el n:prop|Lala .
                                            ^^^^^^ NOW PRESENT!
```

### Fix #2: Communicator Detection ✅

**File**: `~/talkbank-utils/rust/crates/batchalign-core/src/lib.rs`

**Changes**:
1. Added FormType → POS category override logic in `add_morphosyntax_batched_inner`
2. Exhaustive match on ALL FormType variants (C, B, A, D, F, FP, G, I, K, L, LS, N, O, P, Q, SAS, SI, SL, T, U, WP, X)
3. Overrides POS tag for words with form markers AFTER Stanza processing

**Verification**:
```
*MOT:	a nonon@c ?
%mor:	prep|a c|nonon ?
               ^^^^^^^ NOW CORRECT!
```

### Bonus: Maximal Type Safety ✅

**File**: `~/talkbank-utils/rust/crates/batchalign-core/src/text_types.rs`

**Changes**: Implemented provenance-encoding newtypes for ALL strings:
- `ChatRawText` - raw text from CHAT AST
- `ChatCleanedText` - cleaned text for NLP
- `StanzaTokenText` - token from Stanza output
- `StanzaLemma` - lemma from Stanza output
- `AlignedWordText` - word from FA model
- `SpeakerCode` - speaker identifier
- `LanguageCode` - ISO 639-3 code
- `Terminator` - utterance terminator

**Benefit**: Compiler prevents mixing text from different sources. Type names encode data flow and provenance.

---

## Next Steps

### 1. Fix Reflexive Feature (HIGH PRIORITY)
**Location**: `~/talkbank-utils/rust/crates/batchalign-core/src/`

**Task**: Add Reflex feature extraction from Stanza
```rust
// Pseudocode
if token.feats.contains("Reflex=Yes") {
    suffix.push("reflx");
}
```

**Files to check**:
- Where Stanza token features are processed
- Where PRON POS handler builds the morphology string

**Verification**:
```bash
# After fix, should match Python:
%mor:	pron|ell-Prs-AccDat-reflx-S3
```

### 2. Fix Communicator Detection (MEDIUM PRIORITY)
**Task**: Detect `@c` suffix and assign `c|` POS

**Files to check**:
- Where CHAT word text is extracted (before Stanza)
- Where POS tags are assigned based on CHAT suffixes

**Verification**:
```bash
# Main tier: nonon@c
# Should produce: c|nonon (not x|nonon)
```

### 3. Clarify L2 and %wor Conventions (LOW PRIORITY)
**Task**: Check CHILDES/CHAT manual for:
- Standard handling of code-switched words
- %wor tier suffix conventions

**Resources**:
- CHAT manual: https://talkbank.org/0info/manuals/CHAT.html
- Ask boss/CHILDES team for conventions

---

## Test Command to Verify Fixes

After implementing fixes in Rust:
```bash
# Rebuild Rust extension
cd ~/batchalign2
VIRTUAL_ENV=.venv maturin develop -m ~/talkbank-utils/rust/crates/batchalign-core/Cargo.toml

# Re-run morphotag
uv run batchalign-next morphotag ~/batchalign-benchmarking/data/correctness_morphotag /tmp/rust_fixed

# Compare
diff ~/correctness_test/python/morphotag/010911.cha /tmp/rust_fixed/010911.cha
```

**Expected**: Reflexive and communicator differences should disappear.

---

## Files for Reference

- Python morphotag: `~/batchalign2-bench-baseline/batchalign/pipelines/morphosyntax/ud.py`
- Python PRON handler: lines 145-170 (reflexive logic at 154-156)
- Rust codebase: `~/talkbank-utils/rust/crates/batchalign-core/src/`
- Test data: `~/correctness_test/{python,rust}/morphotag/010911.cha`
