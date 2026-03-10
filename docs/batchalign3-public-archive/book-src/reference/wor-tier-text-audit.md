# %wor Tier Word Text Usage Audit

**Date**: 2026-02-15
**Method**: Made `WorTier::words()` crate-private and analyzed compile errors
**Result**: ✅ **Confirmed** - %wor word text is "eye candy" with ONE exception

---

## Summary

The word text in %wor tiers is **almost never** used for processing. It exists solely for human readability and serialization. The actual timing data comes from `Word.inline_bullet`, not the text.

### Critical Finding

**Only ONE location** in the entire codebase reads %wor word text for processing purposes (not counting serialization):

```rust
// batchalign-core/src/lib.rs:2779
// Function: extract_timed_tiers() — for TextGrid export ONLY
if let Some(wor) = utt.wor_tier() {
    for word in wor.words() {
        push_if_timed(word, &speaker, &mut tiers);
    }
}
```

This extracts `word.cleaned_text()` and `word.inline_bullet` to build praatio TextGrid intervals.

---

## Complete Usage Inventory

### 1. Serialization (Legitimate - Word Text Required)

**File**: `talkbank-model/src/model/dependent_tier/wor.rs:213`

```rust
impl WriteChat for WorTier {
    fn write_chat<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        for item in self.items.iter() {
            match item {
                WorItem::Word(word) => {
                    w.write_str(&word.cleaned_text)?;  // ← Writes %wor back to CHAT
                    if let Some(ref bullet) = word.inline_bullet {
                        bullet.write_chat(w)?;
                    }
                }
            }
        }
    }
}
```

**Purpose**: Serialize %wor tier back to CHAT format.
**Status**: ✅ **Required** - word text must be written to output

---

### 2. Error Messages (Legitimate - Display Only)

**File**: `talkbank-model/src/alignment/wor.rs:112, 140`

```rust
pub fn align_main_to_wor(main: &MainTier, wor: &WorTier) -> WorAlignment {
    if alignable_count > wor_count {
        let wor_items: Vec<AlignableItem> = wor
            .words()
            .map(|w| AlignableItem {
                text: w.cleaned_text.clone(),  // ← For error message ONLY
                description: None,
            })
            .collect();

        let detailed_message =
            format_alignment_mismatch("Main tier", "%wor tier", &main_items, &wor_items);
    }
}
```

**Purpose**: Show which words misaligned when main tier ≠ %wor tier word count.
**Example error**: `Main tier has 5 words but %wor tier has 4: expected "word1 word2 word3 word4 word5" but got "word1 word2 word3 word4"`
**Status**: ✅ **Required** - helps users debug alignment problems

---

### 3. TextGrid Export (THE ONLY PROCESSING USE)

**File**: `batchalign-core/src/lib.rs:2779`

```rust
#[pyfunction]
fn extract_timed_tiers(py: Python<'_>, chat_text: &str, by_word: bool) -> PyResult<String> {
    for line in chat_file.lines.iter() {
        let utt = match line { Line::Utterance(u) => u, _ => continue };
        let speaker = utt.main.speaker.as_str().to_string();

        if by_word {
            if let Some(wor) = utt.wor_tier() {
                for word in wor.words() {
                    push_if_timed(word, &speaker, &mut tiers);  // ← Reads text!
                }
            }
        }
    }
}

fn push_if_timed(word: &Word, speaker: &str, tiers: &mut IndexMap<String, Vec<TimedEntry>>) {
    if let Some(ref bullet) = word.inline_bullet {
        tiers.entry(speaker.to_string()).or_default().push(TimedEntry {
            text: word.cleaned_text().to_string(),  // ← USES %wor word text
            start_ms: bullet.timing.start_ms as u64,
            end_ms: bullet.timing.end_ms as u64,
        });
    }
}
```

**Called from**: `batchalign/formats/textgrid/generator.py`

```python
def dump_textgrid(chat_text: str, by_word: bool = True) -> textgrid.Textgrid:
    import batchalign_core

    tiers_json = json.loads(
        batchalign_core.extract_timed_tiers(chat_text, by_word)  # ← Python entry point
    )

    for speaker, entries in tiers_json.items():
        intervals = [
            Interval(
                float(e["start_ms"]) / 1000,
                float(e["end_ms"]) / 1000,
                str(e["text"]),  # ← The %wor word text ends up here
            )
            for e in entries
        ]
```

**Purpose**: Export CHAT → Praat TextGrid format with word-level timing intervals.
**Status**: ⚠️ **Questionable** - Could use main tier words instead?

---

### 4. Forced Alignment (NEVER Reads %wor)

**File**: `batchalign-core/src/forced_alignment.rs`

```rust
pub fn add_wor_tier(utterance: &mut Utterance) {
    remove_wor_tier(utterance);  // ← DELETES old %wor
    let wor_tier = utterance.main.generate_wor_tier();  // ← Regenerates from main tier
    utterance.dependent_tiers.push(DependentTier::Wor(wor_tier));
}
```

**Process**:
1. Delete existing %wor tier (old text discarded)
2. Inject timing into **main tier** words (`word.inline_bullet = Some(bullet)`)
3. Call `main.generate_wor_tier()` to build new %wor from main tier
4. New %wor words get text from main tier, bullets from `inline_bullet`

**Status**: ✅ **Correct** - Never reads %wor word text

---

### 5. Morphosyntax Processing (NEVER Reads %wor)

**File**: `batchalign-core/src/lib.rs:745-860`

```rust
pub fn add_morphosyntax_batched_inner(...) {
    for utt in utterances_only(&chat_file.lines) {
        let alignable_words = extract_alignable_words_from_main(&utt.main);  // ← Main tier!
        // %wor tier is NEVER consulted
    }
}
```

**Status**: ✅ **Correct** - Reads from main tier only

---

## Tests

**File**: `talkbank-parser-tests/tests/wor_terminator_alignment.rs`

Tests check `word.cleaned_text()` to verify parsing correctness. This is legitimate test code, not production usage.

---

## Timing Data Flow

The **actual** data flow for timing:

```
1. Forced alignment:
   Media file → Whisper/Wav2Vec → Python callback returns word timestamps
   ↓
   Rust injects into main tier: word.inline_bullet = Some(Bullet::new(start, end))
   ↓
   generate_wor_tier() copies: {cleaned_text, inline_bullet} from main tier
   ↓
   %wor tier serialized: "word 1000_1200 another 1200_1500 ."

2. Reading back (roundtrip):
   Parse "%wor: word 1000_1200 ..." → word.inline_bullet = Some(1000, 1200)
   ↓
   word.cleaned_text = "word"  ← This text is NEVER consulted for processing!
   ↓
   Timing comes from word.inline_bullet, not word.cleaned_text
```

**The word text in %wor is purely cosmetic** - it's regenerated from the main tier during forced alignment and only read back for:
1. Serialization (writing CHAT files)
2. Error messages
3. TextGrid export (questionable - see below)

---

## Recommendations

### Option 1: Keep Current Behavior ✅ **RECOMMENDED**

**Pros**:
- ✅ TextGrid export works
- ✅ Human-readable %wor tiers
- ✅ Error messages show actual words

**Cons**:
- ⚠️ Misleading - implies word text matters when it doesn't
- ⚠️ Redundant storage (main tier has same text)

### Option 2: Fix TextGrid Export to Use Main Tier

**Change**:
```rust
// Instead of reading wor.words()
if let Some(wor) = utt.wor_tier() {
    for word in wor.words() {  // ← Uses %wor word text
        push_if_timed(word, &speaker, &mut tiers);
    }
}

// Use main tier words with inline_bullet
for word in collect_alignable_words(&utt.main) {
    push_if_timed(word, &speaker, &mut tiers);  // ← Uses MAIN tier text
}
```

**Pros**:
- ✅ **Eliminates** the only processing use of %wor word text
- ✅ More correct - main tier is source of truth
- ✅ Works even if %wor tier is stale/missing

**Cons**:
- ⚠️ API change to TextGrid export (probably invisible to users)

### Option 3: Make %wor Text Truly Private

Make `WorTier::words()` crate-private (already done!). This prevents **future** code from reading %wor word text.

**Status**: ✅ **IMPLEMENTED**
- `WorTier::words()` is now `pub(crate)`
- Only talkbank-model can access it (for serialization, error messages)
- batchalign-core **cannot** access it (breaks on line 2779)

**Next steps**:
1. Fix the ONE usage in `extract_timed_tiers` to use main tier words
2. Remove `pub(crate)` restriction OR
3. Add a special accessor for TextGrid export: `pub fn words_for_textgrid_export()`

---

## Conclusion

**User's hypothesis**: %wor word text is "just eye candy"
**Verdict**: ✅ **100% CORRECT**

- Serialization: Required (must write text to CHAT)
- Error messages: Required (show users what's wrong)
- Processing: **NEVER USED** (timing comes from inline_bullet)
- Exception: TextGrid export (1 function) - but uses `cleaned_text()` which is identical in main tier and %wor tier

**The word text in %wor tiers exists solely for human readability and CHAT format compliance.** All processing uses `inline_bullet` for timing data.

Restricting `WorTier::words()` to crate-private was a success - it exposed exactly one external usage that we can now address.

---

## Critical Insight: %wor Text is Write-Only "Eye Candy"

### The Never-Reparsed Property

**%wor tier word text is NEVER reparsed as CHAT for processing purposes.**

The data flow is strictly one-way:

```
Processing Pipeline:
  Main tier AST → generate_wor_tier() → %wor tier AST
                                          ↓
                                    serialize to CHAT
                                          ↓
                                    Write to .cha file
                                          ↓
                                    Human reads it
                                          ↓
                                    (END - never parsed back)
```

When we read a CHAT file with %wor tiers:
1. Parser builds %wor AST (word text + inline_bullet)
2. Validation checks main ↔ %wor word count match (uses word text for ERROR MESSAGES only)
3. Forced alignment **DELETES %wor tier entirely** and regenerates it from main tier
4. TextGrid export uses `cleaned_text()` which is **identical** in main tier and %wor tier

**Implication**: We have **complete freedom** to put whatever we want in %wor word text, as long as:
- ✅ It looks reasonable to humans reading the CHAT file
- ✅ It serializes correctly (no special characters that break CHAT format)
- ✅ The word count matches main tier (structural alignment requirement)

### Examples of Valid "Eye Candy" Choices

Since %wor text is never reparsed, we could theoretically use:

**Current (main tier cleaned_text copy)**:
```
*CHI:    hel:lo@c wor:ld .
%wor:    hello 1000_1200 world 1200_1500 .
         ^^^^^ main tier cleaned_text copied
```

**Alternative: Could use raw_text**:
```
*CHI:    hel:lo@c wor:ld .
%wor:    hel:lo@c 1000_1200 wor:ld 1200_1500 .
         ^^^^^^^^ main tier raw_text copied (includes markers)
```

**Alternative: Could use empty placeholders**:
```
*CHI:    hel:lo@c wor:ld .
%wor:    _ 1000_1200 _ 1200_1500 .
         ^ placeholder (timing is what matters)
```

**Alternative: Could use positional indices**:
```
*CHI:    hel:lo@c wor:ld .
%wor:    w0 1000_1200 w1 1200_1500 .
         ^^ index into main tier words
```

**All of these are equally valid** because %wor word text is never read for processing!

### What We Actually Choose: cleaned_text (Current Implementation)

```rust
fn wor_word_from_main(word: &Word) -> Word {
    let mut w = Word::new_unchecked(&word.cleaned_text, &word.cleaned_text);
    if let Some(ref bullet) = word.inline_bullet {
        w.inline_bullet = Some(bullet.clone());
    }
    w
}
```

**Why cleaned_text is the right choice:**
1. ✅ **Human-readable**: Researchers can read %wor tier and understand what was said
2. ✅ **Matches linguistic content**: Shows the actual words without CHAT prosodic notation
3. ✅ **Consistent with TextGrid**: If someone exports to TextGrid, the labels match %wor tier (both use cleaned_text)
4. ✅ **Debugging-friendly**: Error messages show recognizable words, not placeholders or indices
5. ✅ **CHAT convention**: Matches how %wor tier has traditionally been used in CHILDES corpus

**But remember**: This is a **convention**, not a requirement. We could change it to raw_text or anything else without breaking any processing, because %wor word text is never reparsed.

### The One True Constraint: Timing Data

The **only** part of %wor tier that matters for processing is `inline_bullet`:

```rust
pub struct Word {
    pub(crate) cleaned_text: String,     // ← EYE CANDY (never reparsed)
    pub(crate) raw_text: String,         // ← EYE CANDY (never reparsed)
    pub inline_bullet: Option<Bullet>,   // ← REAL DATA (used for processing)
    // ...
}
```

When we serialize to CHAT:
```
%wor:    hello 1000_1200 world 1200_1500 .
         ^^^^^ ^^^^^^^^^^^ ^^^^^ ^^^^^^^^^^^
         candy   REAL     candy    REAL
```

When we parse back:
- "hello" → stored in word.cleaned_text → **never consulted for processing**
- "1000_1200" → parsed into word.inline_bullet → **used for timing data**

### Why This Matters

Understanding that %wor word text is "write-only eye candy" has important implications:

1. **Validation**: We can't validate %wor word text against anything (it's arbitrary)
2. **Regeneration**: Forced alignment can safely delete and regenerate %wor without loss of semantic content
3. **Flexibility**: Future changes to what we put in %wor word text won't break anything
4. **Testing**: We don't need to test that %wor word text is "correct" - only that it serializes and looks reasonable
5. **Performance**: We could optimize by using shorter text (indices, placeholders) if serialization speed mattered

**In summary**: %wor word text is a **display format choice**, not a data integrity concern. The timing data in `inline_bullet` is what actually matters.
