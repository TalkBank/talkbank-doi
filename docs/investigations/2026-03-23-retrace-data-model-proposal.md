# Proposal: First-Class Retrace Representation in talkbank-model

**Status:** Proposal for talkbank-tools
**Last updated:** 2026-03-23 19:15 EDT
**Filed by:** Franklin (from batchalign3 retokenize bug investigation)

## Problem

Retrace content (`<...> [/]`, `word [/]`, `[//]`, `[///]`, `[/-]`) is currently represented as `Annotated<Group>` or `Annotated<Word>` — the same types used for non-retrace annotations like `[*]` (error), `[= text]` (explanation), `[>]`/`[<]` (overlap).

The ONLY way to identify retrace content is to inspect `scoped_annotations` for retrace variants at runtime. This has caused:

1. **A real bug in batchalign3** — `rebuild_content` in the retokenize module recursed into `AnnotatedGroup` content for retraces, incrementing `word_counter` for words that were excluded from MOR extraction. This desynchronized the word-token mapping and caused "MOR item count does not match alignable word count" failures on real corpus data (MOST corpus, 166K utterances).

2. **Duplicated retrace detection logic** — batchalign3 had to copy `is_retrace_annotation` from `validation::retrace::detection` because it's `pub(crate)`. Every consumer that walks content must independently check for retraces.

3. **Easy-to-miss match arms** — When adding new `match` handlers for `AnnotatedGroup` or `AnnotatedWord`, there's nothing in the type system that reminds you to handle the retrace case differently. The current fix in batchalign3 adds `is_retrace_annotated_group` and `is_retrace_annotated_word` checks before every recursion, which is fragile.

## What batchalign3 Needs

### 1. Retraces should be impossible to miss in `match` arms

Currently:
```rust
match item {
    UtteranceContent::AnnotatedGroup(annotated) => {
        // Must remember to check is_retrace_annotated_group() here!
        // Forgetting causes word_counter desync bugs.
        rebuild_bracketed_content(...);
    }
}
```

Proposed:
```rust
match item {
    UtteranceContent::Retrace(retrace) => {
        // Compiler forces handling this variant.
        // Pass through unchanged — retrace content is frozen.
        new_content.push(UtteranceContent::Retrace(retrace));
    }
    UtteranceContent::AnnotatedGroup(annotated) => {
        // Only non-retrace annotated groups reach here.
        rebuild_bracketed_content(...);
    }
}
```

### 2. `is_retrace_annotation` should be public

```rust
// Currently: pub(crate) in validation::retrace::detection
// Needed: pub in talkbank_model (or a method on ScopedAnnotation)

impl ScopedAnnotation {
    pub fn is_retrace(&self) -> bool { ... }
}
```

### 3. `walk_words` should support explicit retrace skipping

Currently `TierDomain::Mor` conflates "skip retraces" with "MOR-domain extraction behavior". A caller who wants to walk all words but skip retraces has to use `TierDomain::Mor` even if they're not doing MOR extraction.

```rust
// Current: domain controls both alignment behavior AND retrace skipping
walk_words(&content, Some(TierDomain::Mor), |word_item| { ... });

// Proposed: separate concerns
walk_words(&content, WalkConfig { skip_retraces: true, domain: None }, |word_item| { ... });
```

## Proposed Data Model Change

### Option A: Dedicated `Retrace` Variant (Recommended)

Add `Retrace` as a first-class variant in both `UtteranceContent` and `BracketedItem`:

```rust
pub enum UtteranceContent {
    Word(Box<Word>),
    AnnotatedWord(Annotated<Word>),
    AnnotatedGroup(Annotated<Group>),   // only non-retrace annotations
    Retrace(RetraceContent),            // NEW: retrace-specific
    Group(Group),
    // ... other variants
}

pub struct RetraceContent {
    /// The retraced content (what was said and then corrected).
    pub content: BracketedContent,
    /// The retrace type ([/], [//], [///], [/-]).
    pub retrace_type: RetraceType,
    /// Source span for diagnostics.
    pub span: Span,
}

pub enum RetraceType {
    Partial,        // [/]
    Full,           // [//]
    Multiple,       // [///]
    Reformulation,  // [/-]
    Uncertain,      // [/?]
}
```

**Advantages:**
- Type system enforces handling: `match` must have a `Retrace` arm
- No runtime annotation inspection needed
- Clear ownership: retrace content is frozen/immutable for NLP purposes
- `walk_words` can skip `Retrace` without checking annotations

**Disadvantages:**
- Breaking change: all existing `match` arms on `UtteranceContent` need updating
- Parser needs to produce `Retrace` instead of `AnnotatedGroup` for retrace cases
- Serialization format change (JSON schema)

### Option B: Method on Annotated (Minimal Change)

Keep the current representation but add public methods:

```rust
impl<T> Annotated<T> {
    /// Check if this annotation carries retrace semantics.
    pub fn is_retrace(&self) -> bool {
        self.scoped_annotations.iter().any(ScopedAnnotation::is_retrace)
    }
}

impl ScopedAnnotation {
    pub fn is_retrace(&self) -> bool {
        matches!(self,
            Self::PartialRetracing | Self::Retracing |
            Self::MultipleRetracing | Self::Reformulation |
            Self::UncertainRetracing
        )
    }
}
```

**Advantages:**
- No breaking changes
- Easy to implement

**Disadvantages:**
- Still easy to forget the retrace check
- Type system doesn't enforce handling
- Every consumer must still call `is_retrace()` independently

### Option C: Separate `AnnotatedWord` into `AnnotatedWord` + `RetracedWord`

Similar to Option A but at the Word level only:

```rust
pub enum UtteranceContent {
    Word(Box<Word>),
    AnnotatedWord(Annotated<Word>),     // only non-retrace
    RetracedWord(RetracedWord),          // NEW: word [/]
    AnnotatedGroup(Annotated<Group>),    // only non-retrace
    RetracedGroup(RetracedGroup),        // NEW: <...> [/]
    // ...
}
```

This is a hybrid of A and the current model.

## Recommendation

**Option A** (dedicated `Retrace` variant) is the cleanest for consumers. The breaking change is large but happens once. All downstream code becomes more correct by construction.

If the breaking change is too large right now, **Option B** (public methods) is a quick win that unblocks batchalign3 immediately. It can be followed by Option A later.

## Impact on batchalign3

With Option A, our retokenize `rebuild_content` would simply:
```rust
UtteranceContent::Retrace(retrace) => {
    new_content.push(UtteranceContent::Retrace(retrace));
}
```

No `is_retrace_annotated_group`, no `is_retrace_annotated_word`, no risk of forgetting the check.

With Option B, we'd replace our copied `is_retrace_annotated_*` functions with `annotated.is_retrace()` from the public API.

## Current batchalign3 Workaround

Until talkbank-tools changes, batchalign3 has local `is_retrace_annotated_group` and `is_retrace_annotated_word` functions in `retokenize/rebuild.rs` that duplicate the retrace detection logic. These should be removed once talkbank-tools provides a public API.
