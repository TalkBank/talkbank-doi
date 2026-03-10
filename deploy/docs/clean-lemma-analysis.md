# What is `clean_lemma` and should we keep it?

## Background

When batchalign runs morphosyntactic analysis, it sends utterance text to Stanza (Stanford's NLP toolkit), which returns lemmas — dictionary forms of words (e.g., "running" → "run", "went" → "go"). These lemmas become the stems in our `%mor` tier output (e.g., `v|run`, `v|go`).

The problem: Stanza's lemmas often contain characters that are **reserved syntax** in CHAT's `%mor` format. For example, `|` separates POS from stem, `-` marks morpheme boundaries, `~` joins multi-word tokens. If a Stanza lemma like `ice-cream` passes through uncleaned, the `-` gets misinterpreted as a morpheme boundary, producing invalid or wrong output.

`clean_lemma` is the function that sanitizes Stanza lemmas before they become `%mor` stems. It was inherited from the original Python batchalign2 codebase and ported to Rust.

**Location:** `batchalign3/rust-next/crates/batchalign-chat-ops/src/nlp/mapping.rs`, function `clean_lemma()` (lines 465–578). This code was moved from the PyO3 `rust/` crate to the shared `batchalign-chat-ops` crate during the CHAT Divorce migration (2026-03). The PyO3 crate now re-exports from `batchalign_chat_ops::nlp`.

**Second layer:** After `clean_lemma()`, there is a second sanitization pass via `sanitize_mor_text()` in `batchalign-chat-ops/src/nlp/validation.rs`. This replaces CHAT structural separators (`|`, `#`, `-`, `&`, `$`, `~`) with underscores and strips whitespace. Both layers run in sequence: `clean_lemma()` first, then `sanitize_mor_text()` before the lemma is stored as a `MorStem`.

## What it does (simplified)

1. **Strips characters that are CHAT `%mor` syntax** — `|`, `-`, `~`, `+`, `_`, `$`, `()`, `,`, `@`-suffixes
2. **Converts ASCII hyphens to en-dashes** — because CLAN MOR uses en-dash (`–`) for real hyphens inside stems, while ASCII hyphen (`-`) means "morpheme boundary"
3. **Handles Stanza garbage** — empty lemmas, `<SOS>`/`<unk>` sentinel tokens, smart quotes, Japanese quote characters
4. **Falls back to the surface word** when all cleaning produces an empty string (prevents invalid output)
5. **One hard-coded special case** — a specific Dutch Stanza bug (`"door zogen"`)

## Full rule inventory

| Rule | What it does | Why (CLAN MOR compatibility) |
|------|-------------|------|
| Japanese quotes | `「` `」` `"` → fall back to surface text | Stanza lemmatizes these as quote chars; CLAN MOR uses the surface form |
| Empty lemma | → fall back to surface text | Stanza sometimes produces empty lemmas |
| `0`-prefix (unknown) | `0word` → `word`, sets `is_unknown=true` | CHAT convention: `0` prefix = omitted word |
| `<SOS>` / `<unk>` | Strip or fall back to surface text | Stanza sentinel tokens that leak through |
| Strip `$` | Remove `$` characters | `$` is a CHAT pre-clitic marker in %mor; can't appear inside stems |
| Strip `.` | Remove periods | Periods are CHAT terminators; can't appear in stems |
| Strip leading/trailing `-` | Remove boundary hyphens | `-` is the CHAT suffix/prefix morpheme marker |
| Collapse `--` | `--` → `-` (twice) | Double dashes aren't valid in CHAT stems |
| Strip `,` `'` `~` `()` `/100` `/r` | Remove these characters | `,` is CHAT separator; `'` clashes with clitics; `~` is MWT joiner; `()` are CHAT shortenings; `/100` `/r` are unknown Stanza artifacts |
| Pipe handling | `foo\|bar` → `foo` | `\|` is the CHAT POS-stem delimiter; can't appear in stem text |
| Strip `_` `+` | Remove underscores and plus | `_` = CHAT compound marker in %mor; `+` = compound marker on main tier |
| `door zogen` special case | Hard-coded fall back to surface text | Specific Dutch Stanza bug |
| Hyphen → en-dash | ASCII `-` → `–` (U+2013) | CLAN MOR uses en-dash for stem-internal hyphens; ASCII hyphen = morpheme boundary |
| Smart quotes | `"` (U+201C) → fall back to surface text | Stanza lemma artifact |
| Strip `@\w` suffix | `word@s` → `word` | CHAT `@` marks word-form codes (e.g., `@s` = second language); not part of the stem |
| Empty-string guard | If all stripping produces `""`, fall back to surface text, then `"x"` | Prevents `punct\|` (empty stem → E342 parse error) |

## Information loss analysis

A key concern: `clean_lemma` is a one-way transformation. Once the cleaned lemma is written to `%mor`, the original Stanza lemma is gone. Some of these transformations are **unrecoverable** — you cannot reconstruct what Stanza originally said by looking at the `%mor` output.

### Lossless rules (recoverable — no information destroyed)

These rules are safe because the original information is either still present in the surface word on the main tier, or the transformation is reversible:

| Rule | Why it's recoverable |
|------|---------------------|
| Empty/garbage fallback (`<SOS>`, `<unk>`, empty, smart quotes) | The original was junk — falling back to the surface word is strictly better |
| `0`-prefix removal | The `0` is already on the main tier; `is_unknown` flag is preserved |
| Japanese quote fallback | Surface text is preserved on the main tier |
| Empty-string guard | Last resort — losing nothing because input was already empty |

### **Lossy rules (unrecoverable — information destroyed)**

These are the concerning ones. The original Stanza lemma carried meaningful linguistic information that gets silently discarded:

| Rule | What's lost | Example | Impact |
|------|------------|---------|--------|
| **Strip apostrophes** | Stanza's lemma for contractions | `don't` → lemma `do` has `'` stripped → `do` (happens to be fine here, but `o'clock` → `oclock`) | Moderate — changes the lemma itself for words with legitimate apostrophes |
| **Strip periods** | Abbreviation markers | `U.S.` → lemma `U.S.` → `US` | Low — but loses the distinction between abbreviation and word |
| **Strip `_` `+`** | Stanza's multi-word lemmas | Stanza lemma `ice_cream` → `icecream`; `pick_up` → `pickup` | **High** — Stanza uses `_` to represent multi-word expressions in lemmas. Stripping it merges the component words into a nonsense string. `pickup` is fine, but `New_York` → `NewYork` is not a real lemma |
| **Strip `()`** | Stanza's optional morpheme markers | Some languages use `()` in lemmas for optional parts | Low for English, potentially higher for other languages |
| **Pipe truncation** | Everything after `\|` in the lemma | If Stanza produces `lemma\|variant`, only `lemma` survives | Unknown — rare, but when it happens you lose the variant info entirely |
| **Hyphen → en-dash** | The fact that Stanza used ASCII hyphen | `self-esteem` → `self–esteem` (en-dash) | Low in isolation, but means downstream tools can't distinguish Stanza's original hyphen from a real en-dash. Reversible only if you assume all en-dashes came from hyphens (which isn't true) |
| **Strip `@\w` suffix** | Word-form annotation | `word@s` → `word` | Moderate — the `@s` (second language marker) was linguistically meaningful |
| **`door zogen` hard-code** | The actual lemma | Falls back to surface text unconditionally | Low — very narrow |
| **Strip `,` `~` `/100` `/r`** | Various Stanza artifacts | These characters in the lemma | Low — mostly Stanza noise, but we can't verify after the fact |

### The core problem

The **highest-impact loss** is the underscore stripping. Stanza uses underscores as a standard convention for multi-word expression lemmas across many languages:
- English: `pick_up`, `ice_cream`, `New_York`
- French: `parce_que`, `peut-être`
- German: compound lemmas

When we strip `_`, `New_York` becomes `NewYork` — a string that isn't a word in any language. A researcher looking at the `%mor` output has no way to know what the original lemma was. This is not just a formatting issue; it's a **data quality** issue.

The **second-highest loss** is the apostrophe stripping. English contractions, possessives, and words like `o'clock` all have legitimate apostrophes in their lemmas. Stripping them silently changes what the lemma means.

### What we could do instead

If we move away from CLAN MOR, we could:

1. **Preserve the original Stanza lemma** in a structured field (e.g., JSON `%mor` with separate `pos`, `lemma`, `features` fields) instead of encoding everything into a single flat string with reserved characters
2. **Only escape** reserved characters rather than stripping them — e.g., `New\_York` or a quoting mechanism
3. **Store both** the raw Stanza lemma and the CLAN-compatible cleaned version, so downstream tools can choose

Any of these approaches would eliminate the information loss while still supporting CLAN-compatible output when needed.

## Impact on our ported CLAN tools

We have ported 16 CLAN analysis commands to Rust in `talkbank-clan/`. The question is: how many of them actually care about `clean_lemma`?

### Only 2 commands read %mor at all

| Command | How it uses %mor |
|---------|-----------------|
| **MLU** | Counts the number of `%mor` items per utterance to compute mean length of utterance |
| **FREQ** (with `--mor` flag) | Serializes each `%mor` item as a frequency key (e.g., `verb\|run-PAST`) and counts occurrences |

The other 14 commands (MLT, WDLEN, MAXWD, FREQPOS, TIMEDUR, KWAL, GEMLIST, COMBO, COOCCUR, DIST, CHIP, VOCD, PHONFREQ, MODREP) either work on the main tier only or on non-%mor dependent tiers (%pho, %mod). They are completely unaffected by `clean_lemma`.

### MLU: no impact

MLU calls `mor_tier.items.len()` — it counts items, period. It never examines the stem text, POS category, or features. Whether the stem says `NewYork`, `New_York`, or anything else is irrelevant. `clean_lemma` has **zero effect** on MLU output.

### FREQ with `--mor`: direct impact

FREQ serializes each morphological word via `write_chat()` to produce frequency keys like `verb|run-PAST`, then counts how often each key appears. The stem text **is** part of the frequency bucket.

Concretely:
- With current `clean_lemma`: Stanza lemma `New_York` → cleaned to `NewYork` → FREQ counts `noun|newyork`
- Without underscore stripping: Stanza lemma preserved as `New_York` → FREQ would count `noun|new_york`

These are different frequency buckets — FREQ output changes. But FREQ is just faithfully displaying whatever is already stored in `%mor`. It doesn't do its own cleaning. The real decision point is what goes into `%mor` at batchalign time, not what CLAN tools do with it afterward.

### Summary

| Tool | Reads stem text? | `clean_lemma` impact |
|------|-----------------|---------------------|
| **MLU** | No — counts items only | **None** |
| **FREQ** (without `--mor`) | No — counts main tier words | **None** |
| **FREQ** (with `--mor`) | Yes — stem is part of frequency key | **Direct** — different stems = different frequency buckets |
| All other 14 commands | Don't read `%mor` at all | **None** |

**In short**: Of 16 ported CLAN commands, only FREQ with `--mor` is affected by `clean_lemma`, and even then it's just displaying what batchalign already wrote. The CLAN tools themselves are not the stakeholder — the `%mor` data produced by batchalign is.

## Recommendation

**If we're keeping CLAN MOR's `%mor` format**: Keep all of it. Every rule exists because a real Stanza output hit a real CHAT parse error in production. Removing any rule will eventually produce invalid `%mor` output that fails validation.

**If we're moving away from CLAN MOR's format**: Many rules are only necessary because of CLAN MOR's character-level syntax conventions. Specifically:

- The **hyphen → en-dash conversion** (rule exists solely because CLAN MOR reserves ASCII `-`)
- The **`_` `+` `~` `$` stripping** (these are all CLAN MOR delimiters)
- The **`@` suffix stripping** (CHAT word-form annotation convention)
- The **pipe handling** (CLAN MOR's POS|stem delimiter)

These could all be simplified or removed if we define our own morphology representation where these characters aren't reserved.

**Regardless of direction, keep**:
- The Stanza garbage handlers (`<SOS>`, `<unk>`, empty lemmas, smart quotes) — these are Stanza bugs, not CHAT conventions
- The empty-string fallback guard — defensive coding, always needed

## Compound Lemma Boundaries Across UD Languages

Several UD treebanks use special characters *inside* lemmas to mark morphological boundaries. These characters are meaningful linguistic annotations — not noise — but some collide with CHAT's reserved syntax. Here is the complete inventory and how our pipeline handles each:

| Language | Marker | Meaning | Example | `clean_lemma()` | `sanitize_mor_text()` | Net Result |
|----------|--------|---------|---------|-----------------|----------------------|------------|
| **Estonian** | `=` | Compound boundary | `maja=uks` (house-door) | Passes through | Passes through | **Preserved** in %mor lemma |
| **Basque** | `!` | Derivational boundary | `partxi!se` (to share + derivation) | Passes through | Passes through | **Preserved** in %mor lemma |
| **Finnish** | `#` | Compound boundary | `jää#kaappi` (ice-cabinet) | Passes through | **Replaced with `_`** | **Mangled** — `#` is a reserved CHAT character (legacy prefix marker) |
| **Many** | `_` | Stanza MWE convention | `New_York`, `parce_que`, `pick_up` | **Stripped entirely** | N/A (already gone) | **Destroyed** — `NewYork`, `parceque`, `pickup` |

### Analysis

**`=` and `!`** are safe: they aren't reserved by CHAT `%mor` syntax, so both cleaning layers pass them through. Estonian compound boundaries and Basque derivational boundaries appear correctly in the final %mor output.

**`#`** is problematic: `clean_lemma()` doesn't touch it, but `sanitize_mor_text()` replaces it with `_` because `#` is reserved in traditional CHAT MOR for prefix markers (e.g., `v|#un#do`). Finnish compound lemmas like `jää#kaappi` become `jää_kaappi`. The underscore is better than stripping entirely, but the boundary marker loses its distinct identity.

**`_`** is the worst case: Stanza uses underscores as a *de facto standard* for multi-word expression lemmas across many languages. `clean_lemma()` strips them entirely (line 529), turning `New_York` into `NewYork` — a string that isn't a word in any language. This is a data quality issue, not just a formatting one.

### The "comma after the lemma" in Dutch and French

This is a **separate issue** from compound lemma boundaries. What appears as "commas in the features" in Dutch/French CONLLU output is actually UD's **multi-value feature notation** — e.g., `PronType=Int,Rel` meaning a pronoun is *both* interrogative and relative.

These commas are **not** in the lemma. They appear in the feature column of CONLLU, and in CHAT %mor they appear in the feature suffix: `pron|wat-Int,Rel`. Our pipeline preserves them correctly — see `parse_feats()` in `mapping.rs` (lines 437–453) and the test `test_multivalue_ud_features_preserve_commas`. The grammar and both parsers accept commas in feature values.

The commas do **not** separate "inherent" from "inflectional" features. In UD, commas within a single feature always mean "multiple values of the same feature" (e.g., both interrogative AND relative). Feature ordering in our output is determined by the POS-specific suffix builder, not by any inherent/inflectional distinction.

### Unified Unicode Separator (Not Yet Implemented)

As of 2026-03-02, no unified Unicode compound lemma separator has been implemented. The current situation is inconsistent: `=` and `!` pass through, `#` gets mangled, `_` gets destroyed.

A possible future improvement: pick a single rare Unicode character (such as `⁀` U+2040 CHARACTER TIE, or `⸱` U+2E31 WORD SEPARATOR MIDDLE DOT) to replace all compound-boundary markers uniformly. This would require:

1. A mapping table in `clean_lemma()`: `!` → unified marker, `#` → unified marker, `=` → unified marker, `_` → unified marker (for known MWE lemmas)
2. Updating `sanitize_mor_text()` to allow the chosen character through
3. Updating the tree-sitter grammar's `mor_lemma` rule if the character isn't already accepted
4. Documenting the convention in the %mor tier specification

This is a design decision that needs sign-off before implementation.

## Bottom line

`clean_lemma` is a compatibility shim between Stanza's messy output and CLAN MOR's strict syntax. The decision of what to keep depends entirely on whether our `%mor` output needs to remain CLAN-compatible or whether we're free to define our own format.
