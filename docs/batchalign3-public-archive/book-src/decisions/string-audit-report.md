# String Type Audit Report: batchalign-core

## Executive Summary

**talkbank-model already has excellent type-safe newtypes with interning!** The problem is that **batchalign-core bypasses them and uses raw `String`** in several places.

## Existing Type-Safe Newtypes (talkbank-model)

| Type | Purpose | Interning | Status |
|------|---------|-----------|--------|
| `PosCategory` | POS tags (v, n, pro) | Yes | Well-designed |
| `PosSubcategory` | Subcategories (prop, art) | Yes | Well-designed |
| `MorStem` | Lemmas/stems | Yes | Well-designed |
| `MorPrefix` | Prefixes (RE#, UN#) | Yes | Well-designed |
| `FormType` | Form markers (@c, @b) | No | Enum (recently fixed) |

All use the `interned_newtype!` macro which:
- Wraps `Arc<str>` for O(1) cloning
- Global interning for memory efficiency
- Serde transparent serialization
- Type-safe - can't mix up different string types

## Problem Areas in batchalign-core

### 1. ExtractedWord (extract.rs)

**Current:**
```rust
pub struct ExtractedWord {
    pub text: String,           // Raw string
    pub raw_text: String,       // Raw string
    pub form_type: Option<FormType>,  // Good!
    pub lang_marker: bool,
}
```

**Should be:**
```rust
pub struct ExtractedWord {
    pub text: WordText,          // Newtype wrapper
    pub raw_text: RawWordText,   // Newtype wrapper
    pub form_type: Option<FormType>,
    pub lang_marker: bool,
}
```

**Impact:** These strings are created/cloned thousands of times during NLP processing.

---

### 2. MorphosyntaxBatchItem (lib.rs)

**Current:**
```rust
struct MorphosyntaxBatchItem {
    words: Vec<String>,         // Raw strings
    terminator: String,          // Raw string
    special_forms: Vec<(Option<FormType>, bool)>,
}
```

**Should be:**
```rust
struct MorphosyntaxBatchItem {
    words: Vec<WordText>,
    terminator: Terminator,      // Could be enum: Period | Question | Exclamation
    special_forms: Vec<(Option<FormType>, bool)>,
}
```

---

### 3. Forced Alignment (forced_alignment.rs)

**Current:**
```rust
pub struct FaWord {
    pub word: String,           // Raw string
    pub start_ms: u64,
    pub end_ms: u64,
}

pub struct FaGroupPayload {
    pub text: String,           // Raw string
    pub words: Vec<FaWord>,
}
```

**Should use:** `WordText` newtype

---

### 4. Speaker Codes (extract.rs)

**Current:**
```rust
pub struct ExtractedUtterance {
    pub speaker: String,        // Raw string (e.g., "CHI", "MOT")
    pub utterance_index: usize,
    pub words: Vec<ExtractedWord>,
}
```

**Should be:**
```rust
pub speaker: SpeakerCode,
```

---

### 5. Language Codes (mapping.rs in talkbank-model)

**Current:**
```rust
pub struct MappingContext {
    pub lang: String,           // Raw string (e.g., "eng", "spa")
}
```

**Should be:**
```rust
pub lang: LanguageCode,     // ISO 639-3 code
```

---

### 6. POS Tag Construction in lib.rs

**CRITICAL ISSUE:**

```rust
// lib.rs lines 900-930
let pos_tag = match ft {
    FormType::C => "c",   // String literal
    FormType::B => "b",   // String literal
    // ... 20+ more cases
};

word.part_of_speech = PartOfSpeech::new(PosCategory::new(pos_tag));
```

**This is the WORST offender!** We match on a type-safe enum, then convert back to strings, then wrap in newtypes. This is primitive obsession at its worst.

**Should be:**

Option A: Extend FormType with a method
```rust
impl FormType {
    pub fn to_pos_category(&self) -> PosCategory {
        match self {
            FormType::C => PosCategory::new("c"),
            FormType::B => PosCategory::new("b"),
            // ...
        }
    }
}
```

Option B: Make PosCategory an enum
```rust
pub enum PosCategory {
    Verb, Noun, Pronoun, Determiner,
    Communicator, Babbling, Approximate,
    // ... etc.
}
```

---

## Proposed Newtype System for batchalign-core

Since talkbank-model already has great newtypes for morphological data, we need newtypes for **word content** and **identifiers**:

```rust
// batchalign-core/src/text_types.rs

use std::marker::PhantomData;
use std::fmt;

/// Generic text wrapper with phantom type marker
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Text<M> {
    inner: String,
    #[serde(skip)]
    _marker: PhantomData<M>,
}

impl<M> Text<M> {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            inner: s.into(),
            _marker: PhantomData,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn into_inner(self) -> String {
        self.inner
    }
}

// Marker types
pub struct WordTextMarker;
pub struct RawWordTextMarker;
pub struct SpeakerCodeMarker;
pub struct LanguageCodeMarker;

// Type aliases
pub type WordText = Text<WordTextMarker>;
pub type RawWordText = Text<RawWordTextMarker>;
pub type SpeakerCode = Text<SpeakerCodeMarker>;
pub type LanguageCode = Text<LanguageCodeMarker>;
```

---

## Migration Plan

### Phase 1: Add text_types.rs (batchalign-core)
- Create phantom type system
- Add conversions and Display impls
- Add serde support

### Phase 2: Update ExtractedWord
- Change `text: String` to `text: WordText`
- Change `raw_text: String` to `raw_text: RawWordText`
- Fix all call sites

### Phase 3: Update MorphosyntaxBatchItem
- Change `words: Vec<String>` to `words: Vec<WordText>`
- Add `Terminator` enum or newtype

### Phase 4: Fix POS tag construction
- Remove string literal match arms
- Use FormType to PosCategory conversion

### Phase 5: Update forced alignment
- Use WordText for FaWord

### Phase 6: Add speaker/language newtypes
- SpeakerCode newtype
- LanguageCode newtype

---

## Benefits

1. **Compile-time safety**: Cannot pass WordText where SpeakerCode expected
2. **Zero runtime cost**: PhantomData has zero size
3. **Self-documenting**: `fn process(word: WordText)` vs `fn process(s: String)`
4. **Leverages existing work**: talkbank-model already has most of what we need
5. **Prevents bugs**: Type system catches incorrect string usage

---

## Question: Should PosCategory be an enum?

Currently `PosCategory` is an interned string newtype. Should it be an enum instead?

**Arguments FOR enum:**
- Compile-time exhaustiveness checking
- Can't create invalid POS tags
- Better IDE autocomplete
- Matches FormType pattern

**Arguments AGAINST enum:**
- Breaks if CHAT adds new POS tags
- Harder to extend
- String interning already provides good memory characteristics
- Current design is flexible

**Recommendation:** Keep PosCategory as interned newtype BUT add a `FormType::to_pos_category()` method to avoid string literals.

---

## Estimated Impact

- **Lines to change:** ~200-300 in batchalign-core
- **Files affected:** ~8-10
- **Compilation breaks:** ~50-80 (good - catches all misuses!)
- **Runtime cost:** Zero (phantom types)
- **Memory impact:** Minimal (Arc already used in talkbank-model)
- **Benefit:** Massive increase in type safety
