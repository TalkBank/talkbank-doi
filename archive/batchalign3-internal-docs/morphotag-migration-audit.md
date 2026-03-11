# Morphotag Migration Audit: batchalign2 → batchalign3

**Status:** Active migration reference

---

## Purpose

This document records the major behavior families reviewed during the
batchalign2 to batchalign3 morphotag migration and maps them to the current
batchalign3 implementation. It exists to:

1. document the preservation evidence gathered during migration
2. record confirmed divergences and their rationale
3. identify remaining regression-test gaps

The legacy batchalign2 morphotag pipeline relied heavily on string
construction, regex cleanup, and post-hoc rewriting. The current batchalign3
pipeline uses typed Rust structs (`MorWord`, `PosCategory`, `MorFeature`) and
pushes more validation into the construction path itself.

---

## 1. Per-Word Mapping (handler → MorWord)

### batchalign2: `handler()` + POS-specific handlers (ud.py:60-334)

Each POS handler returns a **string** like `"verb|run-Fin-Ind-Pres-S3"` via f-string
concatenation:

```python
# Generic handler (ud.py:60-142)
def handler(word, lang=None):
    target = word.lemma
    # ... 40 lines of string cleaning ...
    target = target.replace('$', '').replace('.', '')
    target = target.replace(',', '').replace("'", '').replace('~', '')
    # ...
    return f"{'' if not unknown else '0'}{pos}|{target}"

# Verb handler (ud.py:245-288)
def handler__VERB(word, lang=None):
    res = handler(word, lang)
    # ... extract features as strings ...
    return res + flag + stringify_feats(aspect, mood, tense, ...)

# stringify_feats (ud.py:50-54)
def stringify_feats(*feats):
    template = ("-" + "-".join(filter(lambda x: x != "", feats))).strip()
    return template.replace(",", "")
```

### batchalign3: `map_ud_word_to_mor()` + feature dispatchers

```rust
// mor_word.rs:27-84
pub fn map_ud_word_to_mor(ud: &UdWord, ctx: &MappingContext) -> Result<Mor, MappingError> {
    let (cleaned_lemma, _is_unknown) = clean_lemma(&ud.lemma, &ud.text);
    let features = compute_features(&ud.upos, &feats, &effective_pos, ud, ctx);
    let sanitized_lemma = sanitize_mor_text(&cleaned_lemma);
    let mor_word = MorWord::new(
        PosCategory::new(&effective_pos),
        MorStem::new(sanitized_lemma),
    ).with_features(features);
    Ok(Mor::new(mor_word))
}
```

### Line-by-line correspondence

| batchalign2 (ud.py) | batchalign3 (Rust) | Notes |
|-----|-----|-------|
| `handler()` lines 60-142 | `clean_lemma()` in `mor_word.rs:126-238` | Same cleanup sequence for the audited lemma-cleaning operations |
| `target.replace('$', '')` (line 91) | `target.replace('$', "")` (line 158) | Same |
| `target.replace('.', '')` (line 92) | `target.replace('.', "")` (line 159) | Same |
| `target.replace(',', '')` (line 108) | `target.replace(',', "")` (line 176) | Same |
| `target.replace("'", '')` (line 109) | `target.replace('\'', "")` (line 177) | Same |
| `target.replace('~', '')` (line 110) | `target.replace('~', "")` (line 178) | Same |
| `target.replace('_', '')` (line 121) | `target.replace('_', "")` (line 190) | Same |
| `target.replace('+', '')` (line 122) | `target.replace('+', "")` (line 191) | Same |
| `target.replace('-', '–')` (line 129) | `target.replace('-', "\u{2013}")` (line 199) | Same (ASCII hyphen → en-dash) |
| `f"{pos}\|{target}"` (line 142) | `MorWord::new(PosCategory, MorStem)` | String concat → typed construction |
| `handler__VERB` lines 245-288 | `verb_features()` in `features.rs:18-91` | Feature-for-feature match |
| `handler__PRON` lines 145-170 | `pron_features()` in `features.rs:97-152` | Feature-for-feature match |
| `handler__DET` lines 172-193 | `det_features()` in `features.rs:158-202` | Feature-for-feature match |
| `handler__ADJ` lines 195-209 | `adj_features()` in `features.rs:208-232` | Feature-for-feature match |
| `handler__NOUN` lines 211-238 | `noun_features()` in `features.rs:238-284` | Feature-for-feature match |
| `handler__PROPN` lines 240-243 | `compute_features()` dispatch (line 310) | PROPN uses NOUN handler in both |
| `handler__PUNCT` lines 301-321 | `map_actual_punct()` in `mor_word.rs:87-96` | Simplified: edge cases handled by Stanza |
| `parse_feats()` lines 44-48 | `parse_feats()` in `mor_word.rs:99-115` | Dict/HashMap from `key=val\|key=val` |
| `stringify_feats()` lines 50-54 | Individual `push_feature()` calls | Each feature pushed to typed `SmallVec` |
| `HANDLERS` dict lines 324-334 | `compute_features()` match in `mor_word.rs:296-318` | Dict dispatch → enum match |
| `person == "0" → '4'` (multiple) | `if person_raw == "0" { "4" }` (multiple) | Same Person=0 → 4 mapping |

### What changed

For the audited lemma-cleaning and per-word feature cases, the intended output
contract is preserved. The difference is structural:

- batchalign2 builds `"pron|I-Prs-Nom-S1"` as a raw string, then parses it later
- batchalign3 builds `Mor { main: MorWord { pos: "pron", lemma: "I", features: ["Prs", "Nom", "S1"] } }`

The typed representation cannot produce malformed %mor (e.g., unescaped `|` in a lemma)
because `sanitize_mor_text()` runs on every field individually.

---

## 2. Comma Handling in Features

### batchalign2: strip all commas

```python
# stringify_feats (line 54)
return template.replace(",", "")

# handler__DET (line 188)
gender_str == "-Com,Neut" → gender_str = ""   # comparison with comma
# BUT (line 181):
gender_str = "-" + feats.get("Gender", "...").replace(",", "")  # strip comma

# parse_sentence (line 552)
mor_str = (" ".join(...)).strip().replace(",", "")  # strip ALL commas from final output
```

### batchalign3: preserve commas in UD multi-value features

```rust
// mor_word.rs:106-110  (in parse_feats)
// UD multi-value features use commas (e.g. PronType=Int,Rel).
// We preserve the comma as-is to respect UD conventions.
map.insert(key.to_string(), value.to_string());
```

### Why this diverges

batchalign2 strips ALL commas from the final %mor string because commas are the CHAT
separator character (`,` = `cm|cm` in %mor notation).  This was a blunt instrument:
`PronType=Int,Rel` became `PronType=IntRel`.

batchalign3 preserves the comma because:
1. The tree-sitter grammar and %mor parser both accept commas in suffix values
2. `Int,Rel` is semantically meaningful (pronoun is both interrogative AND relative)
3. The `cm|cm` separator is a separate %mor item, not an inline character

This is a **deliberate improvement**, not a loss.  Old CLAN corpus data has `IntRel`;
new batchalign3 output has `Int,Rel`.  Both parse correctly.  The comma-preserving form
is more faithful to the UD annotation.

---

## 3. MWT/Clitic Assembly

### batchalign2: `parse_sentence()` string post-processing (ud.py:343-579)

batchalign2 builds %mor as a flat list of strings, then joins with string manipulation:

```python
# Phase 1: Build word-level strings
mor: list[str | None] = []
for word in sentence.words:
    mor.append(handle(word, lang))  # e.g. "verb|run-Inf-S"

# Phase 2: Join clitics with $ (lines 506-515)
while len(clitics) > 0:
    clitic = clitics.pop()
    mor_clone[clitic-1] = prev_item + "$" + curr_item
    mor_clone[clitic] = None

# Phase 3: Join auxiliaries with ~ (lines 517-531)
for aux in auxiliaries:
    mor_clone[aux-1] = prev_item + "~" + orig_item
    mor_clone[orig_aux] = None

# Phase 4: Join MWTs with ~ (lines 533-550)
while len(mwts) > 0:
    mwt = mwts.pop(0)
    mwt_str = "~".join([i for i in mor_clone[mwt_start-1:mwt_end] if i])
    mor_clone[mwt_start-1] = mwt_str

# Phase 5: Join and clean
mor_str = (" ".join(x for x in mor_clone if x is not None)).strip().replace(",", "")
```

### batchalign3: `assemble_mors()` typed construction (mapping.rs:305-341)

```rust
fn assemble_mors(components: &[UdWord], ctx: &MappingContext) -> Result<Mor, MappingError> {
    // Find the main word (first non-clitic)
    let main_mor = map_ud_word_to_mor(&components[main_idx], ctx)?;
    let mut mor = main_mor;

    // Pre-clitics → post-clitics (model has no pre-clitic field)
    for comp in &components[..main_idx] {
        let m = map_ud_word_to_mor(comp, ctx)?;
        mor = mor.with_post_clitic(m.main);
    }

    // Post-clitics (~)
    for comp in &components[main_idx + 1..] {
        let m = map_ud_word_to_mor(comp, ctx)?;
        mor = mor.with_post_clitic(m.main);
    }

    Ok(mor)
}
```

### Key differences

1. **Mechanism**: batchalign2 builds strings then post-hoc joins with `~`/`$`.
   batchalign3 uses UD Range tokens to know which words belong together and builds
   `Mor::with_post_clitic()` directly.

2. **Clitic identification**: batchalign2 has 50 lines of French/Italian-specific
   `elif` chains (lines 382-432) checking exact token texts.  batchalign3's
   `is_clitic()` (mapping.rs:321-328) uses pattern rules:
   - English: `n't`, `'s`, `'ve`, `'ll`
   - French: `ends_with('\'')`  or `-ce`, `-être`, `-là`
   - Italian: `ends_with('\'')`

   The batchalign3 approach is **more correct**: it doesn't need to enumerate every
   possible clitic form because UD Range tokens already tell us which words belong
   together.  `is_clitic()` only determines which component is "main" vs "clitic"
   within an already-identified MWT group.

3. **Pre-clitics**: batchalign2 distinguishes `$` (pre-clitic) from `~` (post-clitic).
   batchalign3's model has no pre-clitic field — all components become post-clitics.
   This is acceptable because Stanza's MWT model never produces true pre-clitics
   (the `$` marker was a CLAN MOR artifact, not a UD concept).

---

## 4. Language-Specific Handling (Non-English)

### 4.1 French

| Feature | batchalign2 | batchalign3 | Parity |
|---------|------------|------------|--------|
| Pronoun case (Nom/Acc) | `fr/case.py` — word lookup | `lang_fr::french_pronoun_case()` | **Ported lookup table** |
| APM nouns | `fr/apm.py` + `fr/apmn.py` | `lang_fr::is_apm_noun()` | **Ported lookup table** |
| French DET gender default | `"Masc"` when not plural | `gender_default = "Masc"` when not plural | **Same** |
| Clitic identification | 12 `elif` checks for `jusqu'`, `puisqu'`, etc. | `ends_with('\'')` + `is_clitic()` pattern | **Different mechanism, same semantics** — see Section 3 |
| Tokenizer realignment | `tokenizer_processor()` — DP align + French patches | `_realign_sentence()` — char-position map | **Different algorithm** — see Section 6 |
| `aujourd'hui` merge | Explicit `elif` in `tokenizer_processor()` | Char-position map handles automatically | **Same intended outcome** |
| `au` MWT handling | Explicit `elif` returning `(conform(i), True)` | UD Range tokens handle automatically | **Same intended outcome** |
| Triple clitics (`d'l'attraper`) | Explicit regex split on `(\w')+\w+` | UD Range tokens handle automatically | **Same intended outcome** |

### 4.2 Italian

| Feature | batchalign2 | batchalign3 | Parity |
|---------|------------|------------|--------|
| `l'` MWT un-splitting | Explicit `elif` in `tokenizer_processor()` | Char-position map merges if spurious split | **Same intended outcome** |
| `lei` = `le` + `i` | Explicit rejoin in `tokenizer_processor()` | Char-position map handles automatically | **Same intended outcome** |
| `gliel'`, `d'`, `c'`, `qual'` clitics | 5 explicit `elif` checks in `parse_sentence()` | `is_clitic()`: `ends_with('\'')` | **Pattern-based, covers all** |

### 4.3 Japanese

| Feature | batchalign2 | batchalign3 | Parity |
|---------|------------|------------|--------|
| Verb form overrides | `ja/verbforms.py` — if/elif chain | `lang_ja::japanese_verbform()` | **Ported override chain** |
| `combined` processor package | `config["processors"]["tokenize"] = "combined"` | Dedicated `alpha2 == "ja"` branch in `_load_stanza()` | **Configured, with parity test** |
| Comma → `cm` in lemma | `target.replace(',', 'cm')` (line 139) | `cleaned_lemma.replace(',', "cm")` (line 46) | **Same** |
| PUNCT POS → `cm` | Not explicit (handled by `handler__PUNCT`) | Explicit `cm` assignment for PUNCT (lines 50-57) | **Same intended output** |
| `ろ` special case | `elif word.text == "ろ": return res` (line 278-279) | `if ud.text == "\u{308D}" → return empty` (line 30-32) | **Same** |
| `たり` special case | `return res + stringify_feats("Inf", "S")` (line 282) | `push_feature("Inf"); push_feature("S")` (lines 41-43) | **Same** |

The current worker configures Japanese `combined` processor packages in the
dedicated `alpha2 == "ja"` branch, and
`test_japanese_uses_combined_processors` guards that contract.

### 4.4 Portuguese

| Feature | batchalign2 | batchalign3 | Parity |
|---------|------------|------------|--------|
| `d'água` MWT handling | Explicit `elif` returning `("d'água", True)` | UD Range tokens handle automatically | **Same intended outcome** |

### 4.5 Dutch

| Feature | batchalign2 | batchalign3 | Parity |
|---------|------------|------------|--------|
| `'s` possessive handling | Explicit `elif` returning `(conform(i), False)` | MWT postprocessor handles generically | **Same intended outcome** |

### 4.6 Hebrew

| Feature | batchalign2 | batchalign3 | Parity |
|---------|------------|------------|--------|
| `HebBinyan` feature | `feats.get("HebBinyan", "").lower()` | `feats.get("HebBinyan").to_lowercase()` | **Same** |
| `HebExistential` feature | `feats.get("HebExistential", "").lower()` | `feats.get("HebExistential").to_lowercase()` | **Same** |

### 4.7 Norwegian

| Feature | batchalign2 | batchalign3 | Parity |
|---------|------------|------------|--------|
| Trailing apostrophe strip | `if line_cut[-1] == "'": line_cut = line_cut[:-1]` (line 798-799) | **Not ported** | **DIVERGENCE** |

This was a preprocessing step in `morphoanalyze()` that stripped trailing apostrophes
from Norwegian input before sending to Stanza.  It's not present in batchalign3's
word extraction layer.

**Impact**: Norwegian words ending in `'` may be tokenized differently by Stanza.
Likely low-impact since Stanza handles apostrophes natively.

### 4.8 MWT Language Decision

#### batchalign2: Dynamic + exclusion list

```python
mwt_exclusion = ["hr", "zh", "zh-hans", "zh-hant", "ja", "ko",
                 "sl", "sr", "bg", "ru", "et", "hu",
                 "eu", "el", "he", "af", "ga", "da", "ro"]

if not any(i in mwt_exclusion or "mwt" not in get_language_resources(resources, i) for i in langs):
    config["processors"]["mwt"] = "gum" if "en" in langs else "default"
```

This is **dynamic**: it checks Stanza's resource registry to see if a language actually
HAS an MWT model, and also maintains a hardcoded exclusion list.

#### batchalign3: Static inclusion list

```python
mwt_langs = {
    "fr", "de", "it", "es", "pt", "ca", "cs", "pl", "nl", "ar",
    "tr", "fi", "lv", "lt", "sk", "uk", "sv", "nb", "nn", "is",
    "gl", "cy", "gd", "mt", "ka", "hy", "fa", "hi", "ur", "bn",
    "ta", "te", "kn", "ml", "th", "vi", "id", "ms", "tl",
}
has_mwt = alpha2 in mwt_langs
```

This is **static**: a hardcoded set of languages that use MWT.  All 13 languages
from ba2's exclusion list have been removed.

#### Comparison of MWT decisions

All languages now match ba2's MWT decision.  The 13 previously divergent languages
(bg, da, el, et, eu, ga, he, hr, hu, ja, ko, ro, sl) were removed from `mwt_langs`.
Verification: 10 of these 13 don't have Stanza MWT models at all; the 3 that do
(el, et, he) were explicitly excluded in ba2.

Enforced by `test_excluded_languages_not_in_mwt_langs` in
`tests/pipelines/morphosyntax/test_stanza_config_parity.py`.

---

## 5. Preprocessing (morphoanalyze → extract layer)

### batchalign2: `morphoanalyze()` preprocessing (ud.py:730-808)

```python
# 1. Convert utterance to string
line = str(utterance)
line_cut = utterance.strip(join_with_spaces=True)

# 2. Extract and remove ending delimiter
ending = line_cut.split(" ")[-1]

# 3. Clean the sentence
line_cut = clean_sentence(line_cut)  # removes "+,", "++", '+"'

# 4. Regex cleanup
CLEANUP_RE = re.compile(r"\+<|\+/|\(|\)|\+\^|\+//|\+\.\.\.|_|[#]")
line_cut = CLEANUP_RE.sub("", line_cut)

# 5. Special form replacement
special_forms = re.findall(r"\w+@[\w\:]+", line_cut)
for form in special_forms:
    line_cut = line_cut.replace(form, "xbxxx")

# 6. Norwegian apostrophe fix
if line_cut[-1] == "'":
    line_cut = line_cut[:-1]

# 7. Comma spacing
line_cut = line_cut.replace(",", " ,")
```

### batchalign3: Rust AST extraction

batchalign3 does NOT preprocess text strings.  Instead, it walks the parsed CHAT AST
and extracts words:

```rust
// extract.rs: extract_words()
for_each_leaf(content, Some(AlignmentDomain::Mor), |leaf| {
    match leaf {
        ContentLeaf::Word(word) => {
            if word_is_alignable(word) {
                words.push(ExtractedWord { text: word.cleaned_text(), ... });
            }
        }
        ContentLeaf::Separator(sep) => {
            words.push(ExtractedWord { text: sep_text, ... });
        }
        _ => {}
    }
});
```

### What this means

batchalign2's preprocessing was necessary because it operated on raw CHAT text strings.
It had to strip CHAT markup (`+<`, `+/`, `(`, `)`, etc.) from the text before sending
to Stanza.  batchalign3 works on the parsed AST, where these constructs are already
parsed into typed nodes — the word extraction naturally skips non-word content.

The `xbxxx` special-form replacement is preserved in both versions: batchalign3's batch
callback (`_stanza_batch_callback.py`) replaces special-form words with `"xbxxx"` before
Stanza analysis.

For the constructs audited here, the intended input to Stanza is preserved as
long as AST extraction covers the same CHAT material that the old regex cleanup
was removing. The AST-based approach is also more robust against
regex-cleanup drift.

---

## 6. Tokenizer Postprocessing

### batchalign2: `tokenizer_processor()` — DP align + language patches (ud.py:610-700)

A Stanza `tokenize_postprocessor` callback using DP character-level alignment between
Stanza's tokenization and the original sentence.  Contains 40+ lines of
language-specific patches:

- Italian: `l'` MWT un-splitting (line 663), `lei = le + i` rejoin (line 666-668)
- Portuguese: `d'água` handling (line 669-670)
- French: `aujourd'hui` (line 671-675), `au` MWT (line 676-677), triple clitics (line 680-685), `jusqu'/puisqu'/quelqu'/aujourd'` splitting (line 686-689)
- English: apostrophe MWT handling (line 690-693)
- Dutch: `'s` possessive (line 694-695)

### batchalign3: `_realign_sentence()` — char-position map + typed MWT patches

A two-stage approach:

1. **Character-position mapping** (O(n), in `tokenizer_realign::align_tokens()`):
   Builds per-character owner arrays for both original words and Stanza tokens.
   If two Stanza tokens map to the same original CHAT word, they're merged.

2. **Language-specific MWT patches** (in `tokenizer_realign::mwt_overrides`):
   A typed data model (`MwtPatch`, `MwtMatch`, `MwtAction`) with static per-language
   lookup tables replaces ba2's string-hacking patches. Covers French, Italian,
   Portuguese, and Dutch.

The Python layer (`_tokenizer_realign.py`) is now a thin wrapper that sets up the
Stanza `tokenize_postprocessor` callback and delegates to Rust for the actual
alignment and MWT patching.

### How this matches batchalign2

The character-position mapping handles generic token merging (e.g. "ice-cream"
split into ["ice", "-", "cream"]) without needing per-language rules.

For MWT decisions on merged tokens:
- **English**: `is_contraction()` detects apostrophe-containing tokens (excluding
  o'clock/o'er) → `(text, True)` lets Stanza's MWT model expand contractions
- **French**: `apply_mwt_patches()` forces MWT for "au", preserves "aujourd'hui" as
  plain text, splits multi-clitic and elision-prefix tokens
- **Italian**: Suppresses l' MWT expansion, merges spurious "le"+"i" → "lei"
- **Portuguese**: Forces MWT for "d'água"
- **Dutch**: Suppresses MWT for possessive "'s" endings

All ba2 language patches (ud.py:659-698) are now covered by the typed override tables.

### Architecture

```
tokenizer_realign/          (in batchalign-chat-ops, shared by PyO3 + server)
├── mod.rs                  align_tokens(), is_contraction()
└── mwt_overrides.rs        MwtPatch/MwtMatch/MwtAction, apply_mwt_patches()

rust/src/pyfunctions.rs     Thin PyO3 wrapper: Vec<PatchedToken> → Python list
```

---

## 7. Post-Processing Hacks

### 7.1 `~part|s` regex (ud.py:826)

```python
mor = re.sub(r"~part\|s verb\|(\w+)-Ger-S", r"~aux|is verb|\1-Part-Pres-S", mor)
```

This rewrites `~part|s verb|X-Ger-S` → `~aux|is verb|X-Part-Pres-S` on the assembled
%mor string.  It corrects English progressive constructions where Stanza tokenizes
"is running" but tags the clitic `'s` as `part|s` instead of `aux|is`.

**batchalign3**: No direct regex port.

**Why it's OK**: Golden test output shows batchalign3 produces `aux|be-Fin-Ind-Pres-S3
verb|run-Part-Pres-S` for "the dog is running" — the correct output.  Current Stanza
models no longer produce the malformed `~part|s` that triggered this regex.  The regex
was a workaround for an older Stanza version.

### 7.2 `$ZERO$` sentinel (ud.py:447, 556)

```python
# When Stanza produces a bare "0" token:
if word.text.strip() == "0":
    mor.append("$ZERO$")
    num_skipped += 1

# Later:
mor_str = mor_str.replace("$ZERO$ ", "0")
```

This uses a sentinel string to track zero-words (CHAT `0word` notation where Stanza
splits `0` and `word` into separate tokens), then post-hoc replaces the sentinel.

**batchalign3**: No sentinel needed.  The word extraction layer handles `0word` as a
single AST node.  Stanza receives the cleaned text without the leading `0`, and the
`is_unknown` flag from `clean_lemma()` tracks the zero-prefix:

```rust
if target.starts_with('0') && target.len() > 1 {
    target = text[1..].to_string();
    unknown = true;
}
```

### 7.3 `<UNK>` removal (ud.py:563-564)

```python
mor_str = mor_str.replace("<UNK>", "")
gra_str = gra_str.replace("<UNK>", "")
```

**batchalign3**: `clean_lemma()` strips `<unk>` (line 174) and `<SOS>` (line 175).
Same effect, earlier in the pipeline.

### 7.4 Empty utterance handling (ud.py:567-577)

```python
if mor_str.strip() in ["+//.", "+//?", "+//!"]:
    mor_str = ""
if mor_str.strip() == "" or gra_str.strip() == "" or mor_str.strip() == ".":
    mor_str = ""
    gra_str = ""
```

**batchalign3**: `map_ud_sentence()` returns `Err(MappingError)` for empty results.
The caller skips the utterance.  Same behavior: no %mor/%gra written for empty/trivial
utterances.

---

## 8. Retokenize Mode

### batchalign2: 90 lines of regex-based processing (ud.py:833-942)

When `retokenize=True`, batchalign2:
1. Rebuilds the utterance text from Stanza tokens (line 835)
2. Uses DP character alignment to map Stanza tokens back to original CHAT words
3. Applies 17 regex fixups to the retokenized text (lines 926-942):
   - `retokenized_ut.replace("^", "")`
   - `re.sub(r" +", " ", ...)`
   - `retokenized_ut.replace("+ \"", "+\"")`
   - `retokenized_ut.replace(" >", ">")`
   - `retokenized_ut.replace("< ", "<")`
   - ... and 12 more

### batchalign3: Typed AST retokenization (retokenize.rs, 1,414 lines)

When `retokenize=true`, batchalign3:
1. Character-level DP alignment between original and Stanza token texts
2. Maps original words → Stanza tokens
3. Walks AST, rebuilding content vectors
4. New Words created via `DirectParser::parse_word()` (full parser, not string manipulation)

### Why this is better

The regex fixups in batchalign2 were correcting CHAT formatting issues introduced by
string manipulation.  For example, `retokenized_ut.replace(" >", ">")` was fixing
spaces before angle brackets that shouldn't be there.  batchalign3 avoids this entirely
by using the CHAT parser to create new Word nodes — the parser produces correctly
formatted output by construction.

---

## 9. Multilingual Pipeline

### batchalign2: `MultilingualPipeline` (ud.py:1058-1063)

```python
if len(langs) > 1:
    return stanza.MultilingualPipeline(
        lang_configs=configs,
        lang_id_config={"langid_lang_subset": langs},
    )
```

### batchalign3: Single pipeline per language

batchalign3 does not use `MultilingualPipeline`.  Each language gets its own
`stanza.Pipeline`.  Multilingual files are handled by routing utterances to
the appropriate per-language pipeline based on `@Languages` headers.

**Impact**: For truly multilingual files with within-utterance code-switching,
batchalign2 could use Stanza language identification inside the multilingual
pipeline. `batchalign3` routes at the utterance level, so within-utterance
code-switching remains a limitation rather than a parity guarantee.

---

## 10. Current Migration Status

### Completed follow-ups

| # | Divergence | Resolution |
|---|-----------|-----------|
| 1 | 13 languages had MWT wrongly enabled in ba3 | Current `mwt_langs` now respects the ba2 exclusion cases covered by the parity test in `test_stanza_config_parity.py`. |
| 2 | Japanese `combined` processor not configured | Current `_load_stanza()` has a dedicated `alpha2 == "ja"` branch that sets `combined` packages for Japanese. |
| 3 | Language-specific tokenizer patches missing | Current `tokenizer_realign/mwt_overrides.rs` carries the typed French, Italian, Portuguese, and Dutch overrides reviewed during the migration. |
| 4 | `_is_contraction()` was English-only | `is_contraction()` remains English-specific, but non-English MWT forcing/suppression now lives in `apply_mwt_patches()`. |

### Remaining documented differences

| # | Divergence | Severity | Notes |
|---|-----------|----------|-------|
| 5 | Norwegian trailing apostrophe strip not ported | **Low** | ba2 stripped trailing `'` from Norwegian input (ud.py:798-799). Stanza handles apostrophes natively; likely a workaround for an older Stanza version. |
| 6 | Comma preserved in UD features vs stripped | **Intentional** | ba3 preserves multi-value UD features like `PronType=Int,Rel` (Croatian). ba2 stripped all commas post-hoc. This is an improvement. |
| 7 | No `MultilingualPipeline` for code-switching | **Low** | ba3 routes at utterance level, sufficient for TalkBank data where language switches happen between utterances. |

---

## 10b. Complete String Hacking Catalog

This catalog records the string-manipulation patterns reviewed during the
migration and the typed batchalign3 mechanisms that replaced them.

| Pattern | ba2 Location | Semantic | ba3 Location | Typed Approach |
|---------|-------------|----------|-------------|----------------|
| `$ZERO$` sentinel | line 447, 556 | Placeholder for zero-words | Not used | Domain-aware word filtering via `AlignmentDomain` |
| `$` clitic joining | lines 506-515 | Concatenate clitic %mor items | `nlp/mor_word.rs` `is_clitic()` | Typed check + MWT handling in inject phase |
| `~` auxiliary joining | lines 517-531 | Concatenate aux %mor items | `nlp/lang_fr.rs`, `morphosyntax/inject.rs` | Language-specific typed functions + AST injection |
| `~` MWT joining | lines 533-550 | Join Stanza MWT range items | `morphosyntax/inject.rs:111-156` | UD component parsing + %mor duplication |
| Comma stripping | line 552 | Post-hoc `replace(",", "")` | `nlp/mor_word.rs:174-176` | Lemma cleaning via `sanitize_mor_text()` |
| `<UNK>` removal | lines 563-564 | Post-hoc string replace | `nlp/mor_word.rs:152-155` | Lemma cleaning, fallback to surface form |
| Empty utterance `+//` | lines 567-568 | String pattern match | `morphosyntax/preprocess.rs` | AST extraction excludes non-linguistic content |
| `clean_sentence()` | lines 581-709 | Regex-based text cleanup | `morphosyntax/preprocess.rs` | AST-level extraction with domain awareness |
| `CLEANUP_RE` regex | line 730, 782 | Regex removal of CHAT markers | Not needed | Domain-aware AST extraction |
| `xbxxx` special form | lines 787-791 | Regex find + string replace | `retokenize/parse_helpers.rs` | Typed `FormType` enum + AST-level lookup |
| `~part\|s` gerund rewrite | line 826 | Regex substring replace | `nlp/features.rs` | UD feature mapping at word level |
| Retokenize DP alignment | lines 843-872 | Character-level DP | `dp_align.rs`, `retokenize/mapping.rs` | Same Hirschberg algorithm in Rust |
| 11 regex fixups | lines 926-942 | Post-hoc string replacements | Not needed | AST rebuild ensures well-formed CHAT |
| Language-specific patches | lines 662-700 | Conditional string logic per lang | `tokenizer_realign/mwt_overrides.rs` | Typed `MwtPatch`/`MwtAction` data model with static per-language tables |

**Architecture insight:** batchalign3 eliminates all post-hoc string manipulation by:
1. **Typed AST manipulation** — walk ChatFile AST via domain-aware filtering
2. **Typed configurations** — Rust enums (`FormType`, `LanguageCode`, `AlignmentDomain`)
3. **Early validation** — validate at creation time, not via post-hoc regex
4. **Language-aware tables** — lookup tables replace regex patterns
5. **UD as source of truth** — %mor/%gra are derived from UD via typed mapping

---

## 11. Current Automated Coverage

Current migration evidence comes from:

- Rust mapping tests covering lemma cleaning, POS/feature mapping, MWT
  assembly, and generated `%gra` validation
- language-specific Rust tests for English irregular forms, French pronoun/APM
  rules, and Japanese override behavior
- tokenizer-realignment tests covering contraction detection and per-language
  MWT patches
- Python parity tests for MWT allowlist decisions, English GUM configuration,
  non-MWT `tokenize_pretokenized`, and Japanese combined processors

Remaining high-value coverage gaps:

- Norwegian trailing-apostrophe handling
- end-to-end golden tests for French, Italian, Japanese, and Hebrew
- explicit `0word` / `$ZERO$` migration coverage
- separator-token handling (`cm|cm`, `end|end`)
- retokenize parity against legacy outputs

---

## Code References

### batchalign2 (legacy reference code)

| Component | File | Lines |
|-----------|------|-------|
| Generic handler | `pipelines/morphosyntax/ud.py` | 60-142 |
| VERB handler | same | 245-288 |
| PRON handler | same | 145-170 |
| DET handler | same | 172-193 |
| ADJ handler | same | 195-209 |
| NOUN handler | same | 211-238 |
| PUNCT handler | same | 290-334 |
| parse_sentence | same | 343-579 |
| clean_sentence | same | 581-596 |
| tokenizer_processor | same | 610-700 |
| morphoanalyze | same | 712-973 |
| StanzaEngine | same | 975-1227 |
| English irregular verbs | `pipelines/morphosyntax/en/irr.py` | all |
| French pronoun case | `pipelines/morphosyntax/fr/case.py` | all |
| French APM nouns | `pipelines/morphosyntax/fr/apm.py` + `fr/apmn.py` | all |
| Japanese verbforms | `pipelines/morphosyntax/ja/verbforms.py` | all |

### batchalign3 (current)

| Component | File | Lines |
|-----------|------|-------|
| UD→MOR mapping | `chat-ops/src/nlp/mapping.rs` | 1-460 |
| Per-word mapping | `chat-ops/src/nlp/mor_word.rs` | 27-84 |
| Lemma cleaning | `chat-ops/src/nlp/mor_word.rs` | 126-238 |
| Feature dispatch | `chat-ops/src/nlp/mor_word.rs` | 296-318 |
| VERB features | `chat-ops/src/nlp/features.rs` | 18-91 |
| PRON features | `chat-ops/src/nlp/features.rs` | 97-152 |
| DET features | `chat-ops/src/nlp/features.rs` | 158-202 |
| ADJ features | `chat-ops/src/nlp/features.rs` | 208-232 |
| NOUN features | `chat-ops/src/nlp/features.rs` | 238-284 |
| English irregular | `chat-ops/src/nlp/lang_en.rs` | all |
| French rules | `chat-ops/src/nlp/lang_fr.rs` | all |
| Japanese rules | `chat-ops/src/nlp/lang_ja.rs` | all |
| Sanitization | `chat-ops/src/nlp/validation.rs` | all |
| Tokenizer realign | `chat-ops/src/tokenizer_realign/mod.rs` | all |
| MWT overrides | `chat-ops/src/tokenizer_realign/mwt_overrides.rs` | all |
| Python realign wrapper | `inference/_tokenizer_realign.py` | all |
| PyO3 align_tokens | `rust/src/pyfunctions.rs` | thin wrapper |
| Stanza config | `worker/_main.py` | 155-248 |
| Batch callback | `inference/_stanza_batch_callback.py` | all |
