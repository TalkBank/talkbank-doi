# Text Type System with Provenance

> **Update (2026-03):** Only 3 of the 8 proposed types were implemented:
> `ChatRawText`, `ChatCleanedText`, and `SpeakerCode`. These now live in
> `batchalign-chat-ops/src/text_types.rs` (moved from `batchalign-core`
> during CHAT divorce).

## Problem Statement

Current newtypes don't encode provenance:
- `WordText` - but where did it come from? CHAT? Stanza output? Constructed?
- `RawWordText` - raw from where? Which tier?

## Text Flow Through System

```
CHAT File (on disk)
    |
Word::raw_text() -> String with @c markers, timing, etc.
    |
Word::cleaned_text() -> String without markers (for Stanza)
    |
Stanza NLP
    |
UdWord.text -> Token text from Stanza (may differ from input!)
UdWord.lemma -> Lemma from Stanza
    |
map_ud_to_mor()
    |
MorStem, PosCategory (already newtypes in talkbank-model)
    |
serialize back to CHAT
```

## Proposed Type System by Provenance

### From CHAT AST (source: talkbank-model Word)
```rust
/// Raw text as it appears in CHAT transcript
/// Source: Word::raw_text()
/// Contains: @c markers, timing bullets, annotations
pub struct ChatRawText(String);

/// Cleaned text extracted from CHAT for NLP
/// Source: Word::cleaned_text()
/// Contains: Just the lexical content, no markers
pub struct ChatCleanedText(String);
```

### From Stanza/NLP Output
```rust
/// Token text from Stanza UD output
/// Source: UdWord.text
/// Note: May differ from input due to tokenization!
pub struct StanzaTokenText(String);

/// Lemma from Stanza UD output
/// Source: UdWord.lemma
/// Note: This becomes MorStem (already a newtype in talkbank-model)
pub struct StanzaLemma(String);
```

### From Forced Alignment Output
```rust
/// Word text from FA model output
/// Source: FaWord.word field from model JSON
pub struct AlignedWordText(String);
```

### Identifiers (source: CHAT headers)
```rust
/// Speaker code from @Participants
/// Source: Utterance.speaker
pub struct SpeakerCode(String);

/// Language code from @Languages
/// Source: File headers, ISO 639-3
pub struct LanguageCode(String);
```

### Constructed/Synthetic
```rust
/// Utterance terminator
/// Source: Utterance.terminator
/// Values: ".", "?", "!"
pub struct Terminator(String);
```

## Usage in ExtractedWord

```rust
pub struct ExtractedWord {
    /// Cleaned text for NLP (no CHAT markers)
    pub text: ChatCleanedText,

    /// Raw text as in transcript (with markers)
    pub raw_text: ChatRawText,

    /// Form type from CHAT (@c, @b, etc.)
    pub form_type: Option<FormType>,

    /// Has language marker (@s:eng)
    pub lang_marker: bool,
}
```

## Benefits

1. **Clear provenance**: Type name tells you where text came from
2. **Prevent mixing**: Can't pass ChatRawText where ChatCleanedText expected
3. **Self-documenting**: `fn process(word: ChatCleanedText)` vs `fn process(s: String)`
4. **Catch bugs**: Type system prevents using raw text where cleaned expected

## Data Flow Type Safety

```rust
// Extract from CHAT
let raw: ChatRawText = word.raw_text().into();
let cleaned: ChatCleanedText = word.cleaned_text().into();

// Send cleaned to Stanza (ChatCleanedText -> String -> Stanza)
let stanza_result = nlp.process(cleaned.as_str());

// Get back Stanza token
let token: StanzaTokenText = stanza_result.text.into();

// Map to CHAT morphology (StanzaLemma -> MorStem)
let lemma: MorStem = MorStem::new(stanza_result.lemma);
```

## Alternative: Single Type with Phantom Marker?

```rust
/// Generic text with provenance marker
pub struct Text<Source> {
    inner: String,
    _marker: PhantomData<Source>,
}

// Provenance markers
pub struct ChatRaw;
pub struct ChatCleaned;
pub struct StanzaToken;
pub struct StanzaLemma;

// Type aliases
pub type ChatRawText = Text<ChatRaw>;
pub type ChatCleanedText = Text<ChatCleaned>;
pub type StanzaTokenText = Text<StanzaToken>;
```

**Verdict: NO.** You already rejected phantom types as too clever. Stick with simple newtypes.

## Implementation Priority

1. Keep current simple newtypes
2. Rename to encode provenance:
   - `WordText` to `ChatCleanedText`
   - `RawWordText` to `ChatRawText`
3. Add Stanza output types if needed
4. Add conversions at boundaries

## Question for You

Do you want ALL text in the system to have provenance-encoding newtypes? Or just critical ones?

Options:
- **Minimal**: Just ChatCleanedText, ChatRawText, SpeakerCode (what we have now, renamed)
- **Moderate**: Add StanzaTokenText, AlignedWordText for model outputs
- **Maximal**: Every string in the system is wrapped with provenance

What level of strictness do you want?
