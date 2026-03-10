# %wor Tier: Python (origin/master) vs Rust (batchalign3) Comparison

## Methodology

This comparison traces the complete word classification and %wor generation
logic in both implementations:

- **Python**: `lexer.py` classifies tokens → `parser.py` filters into
  `phonated_words` → `generator.py` iterates `utterance.content` for %wor
- **Rust**: `generate_wor_tier()` walks the AST, calling
  `word_is_alignable(word, AlignmentDomain::Wor)` on each word node

## The Key Architectural Difference

In Python, the lexer assigns a `TokenType` to each token, and the parser
builds `utterance.content` from only the **phonated_words** subset:

```python
phonated_words = [tok for tok in words
                  if tok[1][1] in [TokenType.REGULAR,
                                   TokenType.RETRACE,
                                   TokenType.PUNCT,
                                   TokenType.FP]]
```

Everything else (`ANNOT`, `FEAT`, `CORRECTION`) is **dropped from
`utterance.content` entirely** — never stored as a `Form` object.
The generator then iterates all of `utterance.content` for %wor,
so the filter above IS the %wor inclusion rule.

In Rust, all words exist in the AST. `generate_wor_tier()` walks every
node and applies `word_is_alignable(word, AlignmentDomain::Wor)`,
which only excludes: empty words, omissions, and timing tokens (`123_456`).

## Behavioral Differences

### 1. Nonwords (`&~gaga`) — DIFFERENT

| | Behavior | Why |
|---|---|---|
| **Python** | **EXCLUDED** from %wor | Lexer: `&` prefix (not `&-`) → `TokenType.ANNOT` → filtered out |
| **Rust** | **INCLUDED** in %wor as `gaga` | `WordCategory::Nonword` → `word_is_alignable(Wor)` returns true |

### 2. Phonological fragments (`&+fr`) — DIFFERENT

| | Behavior | Why |
|---|---|---|
| **Python** | **EXCLUDED** from %wor | Lexer: `&` prefix → `TokenType.ANNOT` → filtered out |
| **Rust** | **INCLUDED** in %wor as `fr` | `WordCategory::PhonologicalFragment` → `word_is_alignable(Wor)` returns true |

### 3. Untranscribed material (`xxx`, `yyy`, `www`) — DIFFERENT

| | Behavior | Why |
|---|---|---|
| **Python** | **EXCLUDED** from %wor | Lexer: `annotation_clean(form)` is in `CHAT_IGNORE` → `TokenType.ANNOT` → filtered out |
| **Rust** | **INCLUDED** in %wor as `xxx`/`yyy`/`www` | `UntranscribedStatus` set but `word_is_alignable(Wor)` still returns true |

### 4. Replacement words (`want [: wanted]`) — DIFFERENT

| | Behavior | Why |
|---|---|---|
| **Python** | %wor gets **`wanted`** (the replacement) | Lexer: `[:` pops previous word, re-lexes replacement text as `REGULAR` |
| **Rust** | %wor gets **`want`** (the original) | `generate_wor_tier` uses `replaced.word.cleaned_text` (what was spoken) |

The Python approach replaces the original word entirely — the original is
**deleted from `utterance.content`** and the replacement takes its place as
a normal `REGULAR` form. In Rust, both are preserved in the AST as a
`ReplacedWord` node, and %wor explicitly chooses the original.

### 5. Fragment with replacement (`&+fr [: friend]`) — DIFFERENT

| | Behavior | Why |
|---|---|---|
| **Python** | %wor gets **`friend`** (the replacement) | `&+fr` → ANNOT → popped by `[:`, replaced with `friend` as REGULAR |
| **Rust** | **EXCLUDED** entirely | `should_align_replaced_word_in_pho_sin`: fragment + replacement → skip |

This is a three-way divergence: Python includes the replacement, Rust
excludes the whole thing, and a naive "include everything" approach would
include the fragment.

### 6. `xxx [: word]` (untranscribed with replacement) — DIFFERENT

| | Behavior | Why |
|---|---|---|
| **Python** | %wor gets **`word`** (the replacement) | `xxx` → ANNOT → popped by `[:`, replaced with `word` as REGULAR |
| **Rust** | %wor gets **`xxx`** (the original) | `xxx` isn't fragment-like, so `should_align_replaced_word_in_pho_sin` returns true; uses original |

## Consistent Behaviors

### Fillers (`&-uh`) — SAME

Both include fillers in %wor with the prefix stripped:

| | Behavior | Text in %wor |
|---|---|---|
| **Python** | Lexer: `&-` → `TokenType.FP` → in `phonated_words` | `annotation_clean("&-uh")` → `uh` |
| **Rust** | `WordCategory::Filler` → `word_is_alignable(Wor)` true | `cleaned_text` = `uh` |

### Omissions (`0word`) — SAME

Both exclude omissions:

| | Behavior | Why |
|---|---|---|
| **Python** | Parser post-processes: `0word` → `TokenType.CORRECTION` → filtered out | Not in `phonated_words` |
| **Rust** | `WordCategory::Omission` → `word_is_alignable` returns false | Omission check is first |

### Retraces (`<I want> [/] I need`) — SAME

Both include retraced words:

| | Behavior | Why |
|---|---|---|
| **Python** | `[/]` → previous form becomes `TokenType.RETRACE` → in `phonated_words` | RETRACE is in the filter |
| **Rust** | `AnnotatedGroup` with retrace → unconditionally descended | No `should_skip_group` check for Wor |

All words from the retrace group AND the correction appear in %wor (4 words
for `<I want> [/] I need`).

### Events (`&=laughs`) — SAME

Both exclude events:

| | Behavior | Why |
|---|---|---|
| **Python** | `&` prefix → `TokenType.ANNOT` → filtered out | Not in `phonated_words` |
| **Rust** | Parsed as `Event` node, not `Word` → tree walk skips it | Events are `UtteranceContent::Event` |

### Regular words — SAME

Both include with cleaned text:

| | Python text | Rust text |
|---|---|---|
| `a::n` | `annotation_clean` strips `:` → `an` | `cleaned_text` strips lengthening → `an` |
| `hel^lo` | strips `^` → `hello` | strips syllable pause → `hello` |
| `som(e)thing` | strips `(`, `)` → `something` | expands shortening → `something` |
| `ice+cream` | strips `+` → `icecream` | strips compound marker → `icecream` |

### Punctuation / terminator — SAME

Both include the utterance terminator (`.`, `?`, `!`, etc.) in %wor.

- Python: ending punct is a `Form` with `TokenType.PUNCT` in `utterance.content`
- Rust: terminator is `WorTier.terminator`, serialized after all words

### MOR punctuation (`,`, `‡`, `„`) — SAME

Both include these as forms in `utterance.content` / as words in %wor:

- Python: `MOR_PUNCT` → `TokenType.PUNCT` → in `phonated_words`
- Rust: `Separator` nodes → the serializer includes them (as word text, not
  as timing-alignable content)

## Possible Minor Text Differences

`annotation_clean` in Python aggressively strips characters including `-`
(hyphens). If a word like `re-do` appears, Python would produce `redo`
while Rust's `cleaned_text` may preserve the hyphen. This is a text-level
difference, not an inclusion/exclusion difference.

## Summary Table

| Form | Python | Rust | Match? |
|------|--------|------|--------|
| Regular words | INCLUDED | INCLUDED | Yes |
| Fillers `&-uh` | INCLUDED (`uh`) | INCLUDED (`uh`) | Yes |
| Nonwords `&~gaga` | **EXCLUDED** | **INCLUDED** (`gaga`) | **NO** |
| Fragments `&+fr` | **EXCLUDED** | **INCLUDED** (`fr`) | **NO** |
| `xxx`, `yyy`, `www` | **EXCLUDED** | **INCLUDED** | **NO** |
| `word [: replacement]` | **REPLACEMENT** text | **ORIGINAL** text | **NO** |
| `&+fr [: friend]` | **`friend`** (replacement) | **EXCLUDED** | **NO** |
| `xxx [: word]` | **`word`** (replacement) | **`xxx`** (original) | **NO** |
| Omissions `0word` | EXCLUDED | EXCLUDED | Yes |
| Retraces `<w> [/]` | INCLUDED | INCLUDED | Yes |
| Events `&=laughs` | EXCLUDED | EXCLUDED | Yes |
| Terminator `.` `?` `!` | INCLUDED | INCLUDED | Yes |
| MOR punct `,` `‡` `„` | INCLUDED | INCLUDED | Yes |

## Assessment

There are **5 categories of behavioral difference** between the Python and
Rust implementations:

1. **Nonwords and fragments included in Rust but not Python** — The Rust Wor
   domain is maximally inclusive (everything except omissions and timing
   tokens). The Python code classified `&~` and `&+` forms as ANNOT and
   dropped them. This is arguably a Python bug — these forms represent
   phonological material that was produced and should have word-level timing.

2. **Untranscribed material (`xxx/yyy/www`) included in Rust but not Python** —
   Same reasoning: these occupy audio time and represent transcribed
   (if unintelligible) speech. Python explicitly put them in `CHAT_IGNORE`.

3. **Replacement words: original vs replacement** — The Rust approach (use
   original) matches the CHAT manual's intent: %wor tracks what was
   *actually spoken*, and the original form is what was spoken. The Python
   approach (use replacement) reflects morphological/lexical intent rather
   than phonological reality.

4. **Fragment + replacement: Rust excludes, Python uses replacement** — Rust
   treats fragments-with-replacements as non-alignable (the fragment text
   is meaningless to the FA model). Python replaces the fragment with the
   replacement word, making it alignable.

5. **Untranscribed + replacement: different text** — Follows from rules 2-3
   combined.

## Source References

**Python (origin/master):**
- `batchalign/formats/chat/lexer.py` — `__handle()` method: token classification
- `batchalign/formats/chat/parser.py` — `chat_parse_utterance()`: `phonated_words` filter
- `batchalign/formats/chat/generator.py` — `generate_chat_utterance()`: %wor serialization
- `batchalign/constants.py` — `CHAT_IGNORE`, `REPEAT_GROUP_MARKS`, `ENDING_PUNCT`

**Rust (batchalign3):**
- `talkbank-model/src/alignment/helpers/rules.rs` — `word_is_alignable()`, `should_align_replaced_word_in_pho_sin()`
- `talkbank-model/src/model/content/main_tier.rs` — `generate_wor_tier()`, `collect_wor_words_content()`
- `talkbank-model/src/model/dependent_tier/wor.rs` — `WorTier`, `WorWord`, serialization
- `batchalign-core/src/forced_alignment.rs` — `collect_fa_words()`, `collect_fa_replaced_word()`
