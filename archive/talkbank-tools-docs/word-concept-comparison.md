# The Concept of "Word" Across CLAN, talkbank-model, and batchalign-core

This document compares how three systems represent and classify the units that
appear on the main tier of a CHAT transcript. The systems have different goals,
which drives real differences in granularity, composability, and correctness
guarantees.

---

## 1. Original CLAN (C, ~1978-present)

### Representation

Words in CLAN are raw null-terminated C strings. The utterance line is split on
whitespace; each token is an entry in a `char *[]`. There is no AST, no type
system, and no central classification function.

### Classification strategy: string-prefix hacking

Every CLAN command that needs to decide "is this a real word?" reimplements the
same set of `if (word[0] == '&' && word[1] == '-')` tests. Key prefix rules:

| Prefix | Meaning | Example |
|--------|---------|---------|
| `&-` | Filled pause | `&-uh`, `&-um` |
| `&~` | Nonword / babbling | `&~gaga`, `&~gugu` |
| `&+` | Phonological fragment | `&+fr`, `&+w` |
| `0` | Omitted word | `0is`, `0det` |
| `xxx` / `yyy` / `www` | Untranscribed | (verbatim strings) |
| `#` | Pause | `#`, `##` |
| `&=` | Event / action | `&=laughs`, `&=coughs` |

Prosodic markup is embedded in the string itself -- lengthening markers (`::`),
syllable pauses (`^`), stress marks, CA pitch markers, etc. Commands strip
these inline, each with its own ad hoc stripping function.

### The `%mor` tier

The `%mor` tier is a parallel space-separated string. Each token has the form
`pos|stem-suffix` (e.g., `n|dog-POSS`). Parsing is done with `strtok` and
pointer arithmetic. No typed representation exists.

### Problems

- Every command duplicates the classification logic.
- No single source of truth for "what counts as a word."
- Prosodic markup stripping is per-command and inconsistent across the codebase.
- The ~215K-line codebase that resulted is extremely hard to maintain.

---

## 2. talkbank-model (Rust, this codebase)

### Representation

Words are typed `Word` structs in the AST produced by the tree-sitter parser.
Each `Word` carries structured fields:

```rust
pub struct Word {
    pub span: Span,
    pub word_id: Option<String>,
    pub(crate) raw_text: String,      // original, including prosodic markup
    pub(crate) cleaned_text: String,  // linguistic content, markup stripped
    pub content: WordContents,        // structured decomposition
    pub category: Option<WordCategory>,
    pub form_type: Option<FormType>,
    pub untranscribed: Option<UntranscribedStatus>,
    pub lang: Option<WordLanguageMarker>,
    pub part_of_speech: Option<String>,
    pub inline_bullet: Option<Bullet>,
}
```

Pauses, events, and actions are **separate AST node types** -- they never appear
as `Word` instances, so commands that walk `Word` nodes never see them.

### The two text fields

| Field | Content | Example input `a::n` |
|-------|---------|----------------------|
| `raw_text` | Original CHAT representation with all markup | `"a::n"` |
| `cleaned_text` | Linguistic content only | `"an"` |

`cleaned_text` strips: lengthening marks (`:`), syllable pauses (`^`), stress
markers, CA pitch and voice-quality markers, overlap points, and underline
markers. It preserves shortened material by restoring the elided text from
parentheses.

### Classification via `WordCategory` and `UntranscribedStatus`

Instead of string-prefix hacks, classification is captured in typed fields set
by the parser:

```rust
pub enum WordCategory {
    Omission,             // 0is, 0det -- absent word
    CAOmission,           // (word) -- uncertain but present, CA style
    Nonword,              // &~gaga -- babbling
    Filler,               // &-uh, &-um -- filled pause
    PhonologicalFragment, // &+fr -- incomplete attempt
}

// word.untranscribed.is_some() covers: xxx, yyy, www
```

### `is_countable_word()` -- the single classification point

```rust
pub fn is_countable_word(word: &Word) -> bool {
    if word.untranscribed.is_some() { return false; }
    if let Some(ref cat) = word.category {
        if !is_countable_category(cat) { return false; }
    }
    if word.cleaned_text().is_empty() { return false; }
    true
}
```

`CAOmission` is the one category that *is* countable, because the transcriber
heard something and attempted to record it -- unlike `Omission` which represents
speech the speaker failed to produce at all.

### `NormalizedWord` -- the canonical map key

For analysis commands that need to count or compare words, `NormalizedWord`
provides the canonical form:

```rust
pub struct NormalizedWord(String);

impl NormalizedWord {
    pub fn from_word(word: &Word) -> Self {
        NormalizedWord(word.cleaned_text().to_lowercase())
    }
}
```

This is the single place where `cleaned_text` + lowercasing is applied.
All 9 analysis commands (FREQ, DIST, MAXWD, KWAL, COMBO, COOCCUR, WDLEN, MLU,
MLT) use it as their map key rather than raw `String`.

### The `%mor` tier

Morphological annotations are structured as `MorTier` containing `MorWord`
entries. Each `MorWord` has a `part_of_speech`, a stem, and a list of suffixes.
Clitics are stored separately as `MorItem.clitics`.

---

## 3. batchalign-core (Python, batchalign2)

### Representation

Words are `Form` objects (also described as tokens) in a flat list within an
`Utterance`:

```python
class Form(BaseModel):
    text: str
    time: Optional[Tuple[int, int]] = None   # word-level timing (ms)
    morphology: Optional[List[Morphology]] = None
    dependency: Optional[List[Dependency]] = None
    coreference: Optional[List[Coref]] = None
    type: TokenType = TokenType.REGULAR
```

### The `TokenType` enum

```python
class TokenType(IntEnum):
    REGULAR    = 0  # hello -- ordinary word
    RETRACE    = 1  # <I am I am> [/] -- retraced/repeated material
    FEAT       = 2  # (.) -- feature annotation
    FP         = 3  # &-uh -- filled pause
    ANNOT      = 4  # &~ject -- special annotation / nonword marker
    PUNCT      = 5  # . ? ! -- punctuation (including %mor tier punct)
    CORRECTION = 6  # test [= test] -- correction
```

### Mapping between CLAN strings and batchalign `TokenType`

| CLAN string | batchalign type | Notes |
|-------------|-----------------|-------|
| `hello` | `REGULAR` | Ordinary lexical word |
| `&-uh` | `FP` | Stored as `"uh"` (prefix stripped), typed as filled pause |
| `&~gaga` | `ANNOT` | Annotation/nonword |
| `<I am> [/]` | `RETRACE` | Retraced material |
| `[= correction]` | `CORRECTION` | Inline correction |
| `(.)` | `FEAT` | Micro-pause feature |
| `xxx`, `yyy`, `www` | matched against `CHAT_IGNORE` constant | Filtered at string level, not a distinct TokenType |
| `0word` | `CORRECTION` | Strips leading `0`, marks as CORRECTION |
| `.`, `?`, `!` | `PUNCT` | Utterance-ending punctuation |

`CHAT_IGNORE = ["xxx", "yyy", "www"]` is a module-level string constant checked
wherever unintelligible speech needs to be filtered.

### Morphology stored inline

Unlike CLAN's separate `%mor` string tier and talkbank-model's separate
`MorTier`, batchalign stores morphological analysis directly on each `Form`:

```python
class Morphology(BaseModel):
    lemma: str
    pos: str    # e.g. "pron"
    feats: str  # e.g. "Dem-Acc-S1"

class Dependency(BaseModel):
    id: int
    dep_id: int
    dep_type: str  # e.g. "NSUBJ"
```

### Utterance-level convenience

`utterance.strip()` returns a simplified utterance with all RETRACE and FP
tokens removed, suitable for NLP pipelines that don't want disfluency noise.

---

## Comparison Summary

### "Is this a real word?" across the three systems

| System | How to test | Where logic lives |
|--------|-------------|-------------------|
| CLAN | `if (word[0]=='&' && word[1]=='-')` ... | Duplicated per-command in C |
| talkbank-model | `is_countable_word(word)` | Single function in `word_filter.rs` |
| batchalign | `token.type == TokenType.REGULAR` or check against set | Caller convention (no single helper) |

### Granularity of classification

| Token type | CLAN | talkbank-model | batchalign |
|------------|------|----------------|------------|
| Regular word | (default) | `Word` with no category/untranscribed | `REGULAR` |
| Filler (uh/um) | `&-` prefix | `WordCategory::Filler` | `FP` |
| Nonword/babbling | `&~` prefix | `WordCategory::Nonword` | `ANNOT` |
| Fragment | `&+` prefix | `WordCategory::PhonologicalFragment` | (no distinct type) |
| Omission | `0` prefix | `WordCategory::Omission` | `CORRECTION` (strips `0`, loses semantics) |
| CA omission | `(word)` syntax | `WordCategory::CAOmission` | (no handling at all) |
| Unintelligible | `xxx`/`yyy`/`www` | `UntranscribedStatus` field | `CHAT_IGNORE` constant |
| Retracing | `[/]` `[//]` markup | AST structure (no explicit type) | `RETRACE` |
| Pause | `#` prefix | Separate `Pause` AST node | `FEAT` |
| Event | `&=` prefix | Separate `Event` AST node | (part of main tier) |
| Punctuation | Not a "word" in CLAN | Separate `Terminator` AST node | `PUNCT` |

### Prosodic / conversational analysis markup

| System | Strategy |
|--------|----------|
| CLAN | Inline in raw string; each command strips ad hoc |
| talkbank-model | Dual fields: `raw_text` (preserved) vs `cleaned_text` (stripped once at parse time) |
| batchalign | `text` field stores word content as-is; no systematic markup stripping |

### Morphology representation

| System | Format |
|--------|--------|
| CLAN | Parallel `%mor` string line; `pos|stem-suffix` text |
| talkbank-model | Typed `MorTier` -> `MorWord` -> stem + `Vec<MorSuffix>` + POS; separate from main word |
| batchalign | Inline `morphology: List[Morphology]` on each `Form`; stores lemma, pos, feats |

---

## Why the divergences exist

The three systems aren't modeling the same thing. They're modeling different
*views* of the same phenomenon (speech transcribed in CHAT format), optimized
for different primary consumers:

| System | Primary consumer | Implicit question |
|--------|-----------------|-------------------|
| CLAN | Linguistic analysis (FREQ, MLU) | "Should I count this?" |
| talkbank-model | Roundtrip fidelity + analysis | "What *is* this?" (identity) |
| batchalign | NLP pipeline (Stanza, forced alignment) | "Should I send this to the model?" |

This explains the specific divergences:

**Why batchalign collapses fragment and nonword into `ANNOT`:**
Neither produces useful input for Stanza. The distinction between "incomplete
attempt at a real word" vs "babbling" matters for acquisition research but the
NLP model treats both as noise to strip via `utterance.strip()`.

**Why batchalign has `RETRACE` but talkbank-model doesn't:**
Batchalign strips retracings before sending to NLP. talkbank-model keeps them
in the AST for roundtrip fidelity -- they're structurally present but not
specially categorized because the model's job is to faithfully represent the
transcript.

**Why talkbank-model distinguishes `CAOmission` from `Omission`:**
For MLU counting, `(word)` should be counted (the speaker vocalized something)
but `0is` should not (the speaker said nothing). batchalign doesn't represent
omissions at all because there's no audio to align.

**Why batchalign treats `0word` as `CORRECTION`:**
batchalign's parser strips the leading `0` and marks the word as `CORRECTION`,
which means on re-serialization the `0` prefix is lost. This is a lossy
transformation that destroys the semantic distinction between "word the speaker
should have said" (omission) and "corrected word" (correction).

**Why batchalign doesn't handle CA omissions `(word)`:**
No code in batchalign recognizes or generates CA omission syntax. This means
transcripts with CA omissions lose semantic information when processed through
batchalign.

---

## Architectural concerns in talkbank-model

### `cleaned_text` should not be a stored field

`cleaned_text` is a **derived value**: it is always computable from `content`
by concatenating only `WordContent::Text` and `WordContent::Shortening`
variants. The parser computes it during word construction by accumulating into
a `String` as it walks the CST:

```rust
// In process_word_content_node_borrowed():
kind::INITIAL_WORD_SEGMENT | kind::WORD_SEGMENT => {
    // ... push to content AND append to cleaned_text
    cleaned_text.push_str(text_part);
}
kind::SHORTENING => {
    cleaned_text.push_str(shortening_text.as_ref());
    // ... push to content
}
kind::COLON => {
    // ... push Lengthening to content
    // Lengthening is prosodic notation, not lexical content.
    // (nothing appended to cleaned_text)
}
```

Storing a derived value as a field creates several hazards:

**Desynchronization risk.** `content` is `pub` -- any code that mutates it
(adds/removes/reorders content items) leaves `cleaned_text` stale. There is
no mechanism to detect or prevent this. `set_text(raw, cleaned)` allows setting
`cleaned_text` to an arbitrary string without updating `content` at all.

**Constructor inconsistency.** `Word::new_unchecked(raw, cleaned)` sets
`content` to `[Text(cleaned)]` -- a flat text node using the cleaned value, not
the structured decomposition the parser would produce. This means for
parser-constructed words, `content` has full structure
(`Text + Lengthening + Shortening + ...`) but for API-constructed words,
`content` is just `[Text(cleaned)]`. Two different internal representations
for the "same" word.

**JSON roundtrip divergence.** On deserialization, `cleaned_text` comes from
the JSON field value. If the JSON was hand-edited or produced by a different
tool and `cleaned_text` doesn't match what `content` would produce, the
inconsistency is silently accepted.

**The `untranscribed` field has the same issue.** It is computed from
`cleaned_text` after construction (`match cleaned_text.as_str() { "xxx" => ... }`).
It is therefore doubly-derived: `content -> cleaned_text -> untranscribed`.
Though `untranscribed` at least represents a semantic classification rather
than a cached concatenation.

**What `cleaned_text` should be instead:** A computed method on `Word` that
walks `content` and concatenates `Text` + `Shortening` items. This is trivially
cheap for the 1-3 content items in a typical word, and eliminates the
synchronization obligation entirely.

### `raw_text` is more defensible but still suspect

`raw_text` preserves the exact original CHAT string. `WriteChat` for `Word`
reconstructs from `content + category + form_type + lang`, so `raw_text` is
*mostly* derivable. However, reconstruction might not be bit-identical in edge
cases (e.g., multiple consecutive `:` colons for lengthening). `raw_text` serves
as a source-of-truth fallback for roundtrip fidelity and diagnostic messages.

Unlike `cleaned_text`, `raw_text` represents genuinely independent information
(the original input) rather than a projection of `content`.

### `Word` field audit

| Field | Independent? | Risk |
|-------|-------------|------|
| `span` | Yes -- source location from parser | Low |
| `word_id` | Yes -- assigned for alignment | Low |
| `raw_text` | Mostly yes -- original input, not perfectly derivable from content | Low |
| **`cleaned_text`** | **No -- always derivable from content** | **High: desync with content** |
| `content` | Yes -- structured decomposition, source of truth | Low |
| `category` | Yes -- from CST prefix node | Low |
| `form_type` | Yes -- from CST @-marker node | Low |
| **`untranscribed`** | **No -- derived from cleaned_text** | **Medium: desync if cleaned_text is wrong** |
| `lang` | Yes -- from CST @s node | Low |
| `part_of_speech` | Yes -- from CST $-marker | Low |
| `inline_bullet` | Yes -- from %wor tier | Low |

---

## Concrete data corruption risks

These are specific scenarios where current behavior could produce wrong results.

### 1. Stale `cleaned_text` corrupts frequency counts

If any code path mutates `word.content` without updating `cleaned_text`,
`NormalizedWord::from_word(word)` produces wrong frequency keys.
Two words that should merge won't (inflated type count), or two different words
map to the same key (deflated type count, wrong token counts).

**Affected commands:** FREQ, COOCCUR, DIST, MAXWD (all use NormalizedWord as
map key).

**Current mitigation:** `content` is `pub` but nothing in the codebase
currently mutates it after construction. The risk is latent.

### 2. Wrong `cleaned_text` breaks untranscribed detection

The parser checks `match cleaned_text.as_str() { "xxx" | "yyy" | "www" => ... }`.
If `cleaned_text` included prosodic markup (e.g., `xxx:` instead of `xxx`),
the word would not be recognized as untranscribed. This would corrupt:

- **MLU**: `xxx`-only utterances should be excluded from the denominator.
  Counting them inflates the denominator and deflates MLU.
- **FREQ**: Untranscribed words should not be counted. Including them adds
  phantom entries ("xxx" as a word).
- **All commands via `is_countable_word()`**: The function checks
  `word.untranscribed.is_some()` as its first test. Wrong untranscribed status
  means wrong countability.

**Current mitigation:** The parser's `cleaned_text` accumulation logic is
well-tested and correctly excludes prosodic markers. The risk is that future
grammar changes introduce new node types that are not correctly handled in
`process_word_content_node_borrowed()`.

### 3. Word length measurements include markup characters

WDLEN computes word length from `cleaned_text().chars().count()`. If
`cleaned_text` incorrectly included prosodic markup characters:
- `a::n` would be measured as 4 characters instead of 2
- `°softer°` would be measured as 8 characters instead of 6

**Current mitigation:** Same as above -- parser logic is correct today.

### 4. Compound marker in `cleaned_text` would split frequency counts

`ice+cream` should have `cleaned_text` = `"icecream"`. If the `+` leaked into
`cleaned_text`, it would be `"ice+cream"` -- a different string, producing
a different `NormalizedWord`, counted separately from any occurrence of
`"icecream"`.

**Current mitigation:** The parser explicitly does not append to `cleaned_text`
for `CompoundMarker` nodes. The grammar correctly classifies `+` as a
`CompoundMarker` within words.

### 5. Lowercasing is not always correct

`NormalizedWord::from_word()` applies `.to_lowercase()` unconditionally.
This is correct for English but potentially wrong for:
- **German:** Noun capitalization carries meaning (`Weg` "path" vs `weg` "away")
- **Languages with case-sensitive morphology:** Lowercasing can merge distinct
  lemmas

**Current mitigation:** None. All TalkBank data processed so far is primarily
English. This will need revisiting for multilingual support.

### 6. batchalign `0word` -> `CORRECTION` is a lossy transformation

batchalign's parser strips the leading `0` from omission words and marks them
as `CORRECTION`. On re-serialization, the word appears without the `0` prefix:
- Input: `*CHI: 0is happy .`
- batchalign parse: `Form(text="is", type=CORRECTION)`
- batchalign output: `*CHI: is happy .`

The omission information is silently destroyed. If this output is then parsed
by talkbank-model, the `is` will be treated as a regular word and counted in
frequency/MLU/etc., changing analysis results.

### 7. batchalign does not strip prosodic markup

batchalign stores `Form.text` as-is from the CHAT string. A word like `a::n`
becomes `Form(text="a::n", type=REGULAR)`. When batchalign sends this to
Stanza for morphological analysis, Stanza receives `"a::n"` -- not `"an"`.
This will likely cause incorrect POS tagging and lemmatization.

When the same word is round-tripped through talkbank-model, it gets
`cleaned_text = "an"` which is the correct form for NLP. The two systems
will produce different analysis results for the same input.

### 8. The "what is a word" question for `is_countable_word()` is use-case dependent

The current `is_countable_word()` function is designed for CLAN-compatible
linguistic analysis (FREQ, MLU, etc.). But different consumers need different
subsets:

| Consumer | Regular | Filler | Fragment | Nonword | Omission | CA omission | Retracing |
|----------|---------|--------|----------|---------|----------|-------------|-----------|
| FREQ/MLU (linguistic) | yes | no | no | no | no | yes | ? |
| Forced alignment | yes | yes | yes | yes | no | yes | yes |
| NLP (Stanza) | yes | no | no | no | no | yes | no |
| CA research | yes | yes | yes | yes | depends | yes | yes |
| Phonological analysis | yes | no | yes | yes | no | yes | no |

`is_countable_word()` currently implements the FREQ/MLU column. There is no
mechanism for other use cases to request a different subset. If a forced
alignment pipeline uses `is_countable_word()`, it will incorrectly exclude
fillers and fragments that do have audio.

---

## Key design tensions

1. **Inline vs separate morphology**: batchalign's inline model is ergonomic for
   per-word queries but complicates pipelines that produce morphology
   independently (e.g., Stanza) and need to align output back to tokens.
   talkbank-model's separate tier model mirrors CHAT's own structure but
   requires cross-tier alignment logic.

2. **Typed AST vs tagged flat list**: talkbank-model uses separate AST node
   types for pauses/events (so they're never reached by word-walking code), while
   batchalign uses `TokenType` variants in a flat list (so callers must filter).
   The AST approach gives stronger static guarantees; the flat list is simpler to
   iterate and serialize.

3. **CA omission**: Only talkbank-model distinguishes `CAOmission` (something
   the transcriber heard, in CA notation) from `Omission` (something the speaker
   failed to produce). CLAN and batchalign treat both the same way (or don't
   represent omissions at all). This distinction matters for MLU and FREQ -- a
   `(word)` should be counted, a `0word` should not.

4. **Fragment vs nonword**: talkbank-model distinguishes `PhonologicalFragment`
   (`&+fr` -- incomplete word attempt) from `Nonword` (`&~gaga` -- babbling). Both
   collapse to `ANNOT` in batchalign. For most NLP purposes this doesn't matter,
   but for acquisition research (measuring phonological development) it does.

5. **`cleaned_text` as concept vs implementation**: The *concept* of "word text
   with prosodic markup stripped" is correct and necessary. But implementing it
   as a cached field rather than a computed method creates synchronization
   obligations that the model does not enforce. The same concern applies to
   `untranscribed`, which is derived from `cleaned_text`.

---

*Last updated: 2026-02-18*
