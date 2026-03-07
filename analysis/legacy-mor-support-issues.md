# Legacy CLAN %mor Format: Support Issues and Open Questions

**Status:** Unresolved — for future discussion
**Date:** 2026-03-07
**Affects:** talkbank-chat (grammar, parser, model), talkbank-clan (commands), batchalign3 (morphosyntax pipeline)

## Background

The `%mor` tier in CHAT has two distinct formats:

1. **Legacy CLAN format** — used by virtually all existing TalkBank corpora (95,000+ files). Produced by the original CLAN MOR command written in C/C++.
2. **Modern UD format** — introduced in 2026 by batchalign3's Stanza-based pipeline. Flat structure, UD-compatible feature names.

The current grammar, parser, and data model were redesigned around the modern UD format. Several legacy features were explicitly removed. This document catalogs the gaps and the trade-offs involved in each decision.

## Four Legacy Features Not Currently Supported

### 1. POS Subcategories (colon-separated)

**Legacy syntax:** `n:prop|Mommy`, `v:aux|be`, `det:art|the`, `pro:sub|I`, `pro:poss:det|my`

**Scale:** 19,000+ occurrences in `~/data` corpus. `det:art` alone appears 5,204 times.

**Current behavior:** The grammar's `mor_pos` rule excludes colons. `n:prop|Mommy` parses as POS=`n`, lemma=`Mommy`, with `:prop` silently lost.

**Model status:** `PosCategory` is an interned `Arc<str>` — it *could* hold `n:prop` if the grammar allowed it. No model change needed; only a grammar rule change.

**CLAN command impact:** `classify_mor_word()` in `talkbank-clan/src/framework/mor.rs` already handles subcategories via `starts_with()` matching (`p.starts_with("n:prop")`, `p.starts_with("v")`, etc.). If the parser preserved subcategories, commands would work unchanged.

**Open question:** Should the grammar accept colons in POS, or should a migration tool strip subcategories to flat UD tags?

### 2. Fusional Inflection (ampersand `&`)

**Legacy syntax:** `v|be&3S`, `v|make&PROG`, `v|eat&PAST`

**Scale:** 11,831 `%mor` lines in `~/data` contain fusional `&` markers.

**Current behavior:** `&` is accepted inside lemmas by the permissive regex, so `v|be&3S` parses as POS=`v`, lemma=`be&3S`. The `3S` is stuck in the lemma rather than being a separate feature.

**Model status:** Would require either:
- (a) Splitting `&`-delimited parts in the parser (lemma before `&`, feature after), or
- (b) A model-layer normalization pass after parsing.

**CLAN command impact:** The old `classify_mor_token()` text function (now removed) handled this via substring matching (`token.contains("&past")`). The typed `classify_mor_word()` checks `word.features`, which would be empty for fusional forms — so inflection counts would be wrong.

**Open question:** Is fusional `&` a parser concern (split during parse) or a normalization concern (post-parse transform)?

### 3. Compound Stems (plus `+`)

**Legacy syntax:** `n|+n|phone+n|man` (compound noun), `v|+v|up+v|set` (phrasal verb)

**Scale:** Unknown but believed to be rare in English corpora. More common in German, Dutch.

**Current behavior:** No grammar rule exists. Parsing fails or error-recovers.

**Model status:** The `MorCompound` type was explicitly removed in the 2026 UD redesign. Re-adding it would mean restoring a type hierarchy (`Mor` containing either a simple word or a compound of sub-words).

**Open question:** Is compound stem support needed for any active use case, or can legacy compounds be flattened to simple lemmas (e.g., `n|phoneman`)?

### 4. Prefix Morphemes (hash `#`)

**Legacy syntax:** `v|#un#do` (prefix un- + verb do), `v|#re#make`

**Scale:** Unknown but believed to be rare.

**Current behavior:** No grammar rule exists. Parsing fails or error-recovers.

**Model status:** The `MorPrefix` type was explicitly removed. Similar to compounds, re-adding requires restoring type complexity.

**Open question:** Same as compounds — are there active corpora that need prefix morpheme structure, or can they be flattened?

## Test Evidence

Explicit failing tests exist at:
```
talkbank-chat/crates/talkbank-parser-tests/tests/parser_suite/legacy_mor.rs
```

Seven test functions document the parsing failures for subcategories, fusional inflection, and roundtrip fidelity. All are marked as "currently FAIL".

## The Decision Space

### Option A: Full Legacy Support

Extend the grammar and model to accept all four legacy features:
- Grammar: add colon-in-POS, `&`-splitting, `+`-compounds, `#`-prefixes
- Model: restore `MorCompound`, `MorPrefix`, or map to flat equivalents
- Parser: handle both legacy and UD formats
- Risk: increased complexity, two code paths, regression risk

### Option B: Parse-and-Normalize

Accept legacy syntax in the grammar but normalize to UD equivalents during parsing:
- `n:prop` → POS=`n:prop` (preserve as-is in `PosCategory` string)
- `v|be&3S` → POS=`v`, lemma=`be`, features=[`3S`]
- `n|+n|phone+n|man` → POS=`n`, lemma=`phone+man` (flatten)
- `v|#un#do` → POS=`v`, lemma=`undo` (flatten)
- Pro: single model, legacy files parse correctly, no data loss for common cases
- Con: compound/prefix structure is lost (but rarely needed)

### Option C: Migration Tool Only

Do not change the parser. Provide a migration tool that rewrites legacy `%mor` to UD format:
- Fastest to implement
- Requires corpora to be explicitly migrated
- Risk: silent data loss if someone parses a legacy file without migrating first

### Option D: Status Quo with Validation Warning

Current approach. Legacy `%mor` partially parses with silent data loss. Add a validation warning (e.g., W-level) when legacy features are detected:
- Least work
- Most dangerous for correctness

## Impact on Existing Commands

| Command | Uses %mor? | Impact of Missing Subcategories | Impact of Missing Fusional |
|---------|-----------|-------------------------------|---------------------------|
| EVAL | Yes | `v:aux` counted as verb (wrong) | Past tense undercounted |
| KIDEVAL | Yes | Same as EVAL | Same as EVAL |
| DSS | Yes (text) | Pattern rules may fail to match | Rules with `&` patterns break |
| IPSYN | Yes (text) | Pattern rules may fail to match | Rules with `&` patterns break |
| MLU | Yes | Morpheme count wrong for fusional | Morpheme count wrong |
| FREQ +t%mor | Yes | Subcategory collapsed | Fusional stuck in lemma |
| MORTABLE | Yes | Category breakdown wrong | — |

## Relationship to %trn / %grt

The CLAUDE.md for talkbank-clan states:

> If `%trn` / `%grt` support is introduced, model them as AST aliases of `%mor` / `%gra`. They are structural aliases, not separate text formats.

`%trn` is a legacy alternative name for `%mor`. If we support legacy `%mor` syntax, `%trn` support comes for free (it's just a tier name alias). If we don't support legacy syntax, `%trn` has the same problems.

## What batchalign3 Produces

The batchalign3 morphosyntax pipeline (`mor_word.rs` → `map_ud_word_to_mor()`) produces **only** the modern UD format:
- Flat POS (lowercased UPOS: `noun`, `verb`, `pron`)
- No subcategories, no fusional, no compounds, no prefixes
- Features are UD-compatible (`Plur`, `Fin`, `Ind`, `Pres`, `S3`)

New annotations will always be in modern format. Legacy support is only needed for reading/processing existing corpora.

## Recommendation (To Be Discussed)

Option B (parse-and-normalize) appears to offer the best trade-off:
- POS subcategories: trivial to support (just allow `:` in `mor_pos` regex)
- Fusional `&`: moderate effort (split in parser, emit as feature)
- Compounds/prefixes: flatten to simple lemmas (low value in preserving structure)
- Preserves model simplicity while handling the 30,000+ real-world occurrences

The key question for the user: **Is there any active analysis workflow that requires compound stem structure or prefix morpheme structure?** If not, Option B covers >99% of legacy files.
