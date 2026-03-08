# Legacy CLAN %mor Format: Support Issues and Open Questions

**Status:** Classifier fixed (2026-03-08); default DSS/IPSYN rules remain legacy-only (documented limitation)
**Date:** 2026-03-07 (revised 2026-03-08 with empirical data audit and fixes)
**Affects:** talkbank-chat (grammar, parser, model), talkbank-clan (commands), batchalign3 (morphosyntax pipeline)

## Background

The `%mor` tier in CHAT has two distinct formats:

1. **Legacy CLAN format** — produced by the original CLAN MOR command (C/C++). Features POS subcategories (`n:prop`), fusional inflection (`&3S`), compound stems (`+`), prefix morphemes (`#`), and translation annotations (`=word`).
2. **Modern UD format** — produced by batchalign (both v2 and v3). Flat POS tags (lowercase UPOS names: `noun`, `verb`, `propn`), hyphen-delimited features (`-Fin-Ind-Pres-S3`). No subcategories, no fusional markers.

The current grammar, parser, and data model were redesigned around the modern UD format. Several legacy features were explicitly removed. This document catalogs the gaps and trade-offs.

### History: We Had Full Legacy Support and Removed It

The old codebase (`~/talkbank-utils/`, now `talkbank-chat`) had **complete** legacy MOR support before commit `06590e99` (2026-02-23, "refactor(mor): simplify model to flat UD-style MorWord"):

**Old grammar** (`grammar.js` before `ccca018`):
- `mpos` = `mor_category (COLON mor_subcategory)*` — POS subcategories
- `mor_fusional_suffix` = `AMPERSAND mor_fusional_segment` — fusional `&`
- `mor_prefixes` = `(mor_word_segment HASH)+` — prefix chains
- `mor_compound_word` = `mor_word (PLUS mor_word)+` — compounds
- `mor_translation` = `EQUALS mor_english_word` — translation annotations

**Old model** (before `06590e99`):
- `PartOfSpeech` = `{ category: PosCategory, subcategories: SmallVec<[PosSubcategory; 2]> }`
- `MorSuffix` = enum `Fusional(String) | Hyphen(String) | Colon(String)`
- `MorWord` = `{ prefixes, part_of_speech, stem, suffixes }`
- `MorCompound` = `{ prefixes, part_of_speech, words: SmallVec<[MorWord; 4]> }`
- `Chunk` = enum `Word(MorWord) | Compound(Box<MorCompound>) | Terminator(String)`

All were deliberately removed in favor of flat `POS|lemma[-Feature]*`.

### batchalign2 Also Produced Modern Format (Not Legacy)

**Critical finding:** batchalign2 (`~/batchalign2-master/`) did NOT produce legacy CLAN MOR format. It produced a modern UD-derived format identical in style to batchalign3:

- POS tags: lowercase UPOS names (`noun`, `verb`, `pron`, `det`, `aux`, `propn`)
- Features: hyphen-delimited UD features (`-Fin-Ind-Pres-S3`, `-Part-Pres-S`, `-Dem-S1`)
- Clitics: tilde-separated (`pron|I-Prs-Nom-S1~aux|be-Fin-Ind-Pres-S1`)
- **No** subcategories, fusional markers, compounds, or prefixes

The code is in `batchalign/pipelines/morphosyntax/ud.py` — `handler()` returns `{word.upos.lower()}|{lemma}`, features via `stringify_feats()` joined by `-`.

## Empirical Audit of ~/data (2026-03-08)

The original document claimed "19,000+ POS subcategory occurrences" and "11,831 %mor lines with fusional markers." These numbers were misleading because they counted ALL file types and ALL tier types.

### Actual Findings

**68,178 `.cha` files** in `~/data` contain `%mor` tiers.

**Zero `.cha` files** in `~/data` contain legacy CLAN POS subcategories (`n:prop`, `det:art`, `pro:sub`, `v:aux`) in `%mor` tiers.

**Zero `.cha` files** in `~/data` contain fusional `&` markers in `%mor` tiers.

**Where the legacy format actually lives:**

| Location | Format | Count |
|----------|--------|-------|
| `.lab` files (Spanish/Hess corpus) | Legacy CLAN in `%mor` | 120 files |
| `.cha` files in `%xmor` tier | Legacy CLAN in `%xmor` (not `%mor`) | 662 files |
| `.cha` files in `%mor` tier | Modern UD format only | 68,178 files |

The 662 files with `%xmor` (legacy) **also have** modern `%mor` (UD format). Example from YipMatthews Cantonese corpus (`020609.cha`):
```
%xmor: asp|haa5=tentative n:prop|Timmy n|ze4&DIM=older_sister ...
%mor:  propn|吓5 x|Timmy noun|姐姐 verb|俾-Inf-S ...
```

The `%xmor` tier preserves the legacy hand-curated annotation; `%mor` is the batchalign-generated UD annotation.

### Reference Corpus Issue — FIXED

`corpus/reference/tiers/mor-gra.cha` was a **synthetic file** (created 2026-02-28, commit `02ac8eff`) that contained fusional `&` markers in `%mor`. **Fixed 2026-03-08:** rewritten to proper UD format (`aux|be-Fin-Ind-Pres-S3`, `verb|get-Inf`, `noun|cookie-Plur`). All 73 reference corpus tests pass (roundtrip + parser equivalence).

## Revised Assessment: Four Legacy Features

### 1. POS Subcategories (colon-separated)

**Legacy syntax:** `n:prop|Mommy`, `v:aux|be`, `det:art|the`, `pro:sub|I`

**In `.cha` `%mor`:** Zero occurrences in `~/data`.

**In `%xmor`:** ~24,500 occurrences (YipMatthews Cantonese alone). Also in `.lab` files.

**Impact:** If we ever need to parse `%xmor` as a structural alias for `%mor`, subcategory support becomes necessary. For `%mor` alone, this is a non-issue today.

### 2. Fusional Inflection (ampersand `&`)

**Legacy syntax:** `v|be&3S`, `v|make&PROG`

**In `.cha` `%mor`:** Zero occurrences in `~/data`.

**In `%xmor` and `.lab`:** Thousands of occurrences (Spanish, Cantonese).

**Current behavior:** `&` absorbed into lemma. The reference corpus file `mor-gra.cha` contains this format incorrectly — it should be fixed.

### 3. Compound Stems (plus `+`) and 4. Prefix Morphemes (hash `#`)

**Scale:** Not audited in `%xmor`. Believed rare. No occurrences in `%mor`.

## The `%xmor` Question — RESOLVED

**Decision:** `%xmor` is an unparsed tier by definition. It is treated as opaque text (like `%com`). No legacy format support is needed for it. The 662 files that contain `%xmor` also have proper `%mor` in modern UD format.

## Ambiguity Analysis: Can Both Formats Coexist?

If we were to support both formats in a single parser, potential ambiguities:

| Construct | Legacy meaning | UD meaning | Ambiguous? |
|-----------|---------------|------------|------------|
| `n:prop\|X` | POS=`n:prop`, lemma=`X` | N/A (UD never has colons in POS) | **No** — colons never appear in UD POS |
| `v\|be&3S` | POS=`v`, lemma=`be`, fusional=`3S` | POS=`v`, lemma=`be&3S` (current) | **Yes** — `&` could be fusional marker or part of lemma |
| `v\|walk-PAST` | POS=`v`, lemma=`walk`, suffix=`PAST` | POS=`v`, lemma=`walk`, feature=`PAST` | **No** — identical structural position |
| `n\|+n\|phone+n\|man` | Compound noun | N/A (UD has no `+` compounds) | **No** — UD never starts lemma with `+` |
| `UN#v\|do` | Prefix `UN` + verb `do` | N/A (UD has no `#` prefixes) | **No** — UD never uses `#` |

**Only `&` is genuinely ambiguous.** Subcategories, compounds, and prefixes use syntax that modern UD never produces, so they can coexist without ambiguity. The `&` question requires a policy decision: either `&` always means fusional (breaking hypothetical lemmas containing `&`), or we need a tier-level format discriminator.

In practice, `&` in lemmas is extremely rare in UD output — batchalign never produces it. So treating `&` as fusional would be safe for all known data.

## BUG: talkbank-clan POS Classifier Doesn't Handle Modern UD Tags

**This is the real problem.** The CLAN command framework (`talkbank-clan/src/framework/mor.rs`) was written for legacy CLAN POS tags but all production data uses modern UD POS tags. The classifier is **silently wrong** on the most common POS categories.

### POS Classification Bugs

```rust
// Current code in classify_mor_word():
match pos_ref {
    p if p.starts_with("n:prop") || p == "n" => counts.nouns += 1,
    p if p.starts_with("v") && p != "v:aux" => counts.verbs += 1,
    "aux" | "v:aux" => counts.auxiliaries += 1,
    "mod" => counts.modals += 1,
    "prep" => counts.prepositions += 1,
    "adj" => counts.adjectives += 1,
    p if p.starts_with("adv") => counts.adverbs += 1,
    p if p.starts_with("conj") => counts.conjunctions += 1,
    p if p.starts_with("det") => counts.determiners += 1,
    p if p.starts_with("pro") => counts.pronouns += 1,
    _ => {}
}
```

| UD POS | Legacy equiv | Classifier result | Correct? |
|--------|-------------|-------------------|----------|
| `noun` | `n` | **`_ => {}` (dropped!)** | **BUG** — nouns not counted |
| `propn` | `n:prop` | **`starts_with("pro")` → pronoun** | **BUG** — proper nouns counted as pronouns |
| `verb` | `v` | `starts_with("v")` → verb | OK |
| `aux` | `v:aux`/`aux` | exact match → auxiliary | OK |
| `adj` | `adj` | exact match → adjective | OK |
| `adv` | `adv` | `starts_with("adv")` → adverb | OK |
| `det` | `det` | `starts_with("det")` → determiner | OK |
| `pron` | `pro` | `starts_with("pro")` → pronoun | OK |
| `adp` | `prep` | **`_ => {}` (dropped!)** | **BUG** — prepositions not counted |
| `cconj` | `conj` | **`_ => {}` (dropped!)** | **BUG** — conjunctions not counted |
| `sconj` | `conj` | **`_ => {}` (dropped!)** | **BUG** — conjunctions not counted |
| `intj` | `int` | **`_ => {}` (dropped!)** | **BUG** — no interjection category |
| `part` | — | `_ => {}` | OK (not counted in legacy either) |
| `num` | `num` | `_ => {}` | OK (not counted in legacy either) |

**Severity:** `noun` is the most frequent POS in any corpus. Every EVAL, KIDEVAL, MLU, FREQ run on production data produces **wrong noun counts (zero)** and **wrong pronoun counts (inflated by proper nouns)**.

### Feature Matching Bugs

The inflection counter uses substring matching on feature values:

```rust
if val.contains("pl") { counts.plurals += 1; }
if val.contains("past") { counts.past_tense += 1; }
if val.contains("presp") { counts.present_participle += 1; }
if val.contains("pastp") { counts.past_participle += 1; }
```

| UD Feature | Legacy equiv | Matching | Correct? |
|-----------|-------------|----------|----------|
| `Plur` | `PL` | `contains("pl")` on "plur" → match | OK (case-insensitive) |
| `Past` | `PAST` | `contains("past")` on "past" → match | OK |
| `Part-Pres` / `Ger` | `PRESP` | `contains("presp")` → **no match** | **BUG** — present participles not counted |
| `Part-Past` | `PASTP` | `contains("pastp")` → **no match** | **BUG** — past participles not counted |
| `Prs` | — | not checked | — |
| `S3`, `P1`, etc. | — | not checked | — |

UD uses `Part-Pres` and `Part-Past` (or `Ger` for gerund), not `PRESP`/`PASTP`. The substring match fails.

### Commands Affected

| Command | Impact |
|---------|--------|
| **EVAL** | Noun count=0, pronoun count inflated, preposition/conjunction counts=0, participle counts=0 |
| **KIDEVAL** | Same as EVAL plus cascading into DSS/IPSYN scores |
| **MLU** | Morpheme undercounting (participles not detected) |
| **DSS** | Pattern rules using `pro:sub`, `v-PAST` may partially work; `v-PRESP` broken |
| **IPSYN** | Same pattern matching issues as DSS |
| **MORTABLE** | POS extraction separate from classifier — may work if script patterns use UD tags |
| **SUGAR** | Verb detection uses `v`, `cop`, `aux`, `mod` — `verb` detected via starts_with("v") (OK) |

### Fix Applied — POS Classifier and Feature Matching (2026-03-08)

**`classify_mor_word()`** in `framework/mor.rs` now handles both UD and legacy POS tags:
- `noun`/`propn` → nouns; `verb` → verbs; `adp` → prepositions; `cconj`/`sconj` → conjunctions; `pron` → pronouns
- Legacy tags (`n`, `n:prop`, `v`, `v:aux`, `prep`, `conj`, `pro`) still work

**`classify_features()`** handles both UD and legacy feature names:
- UD: `Plur`, `Past`, `Part-Past`, `Part-Pres`, `Ger`
- Legacy: `PL`, `PAST`, `PASTP`, `PRESP`

**Other fixes applied:**
- `dss.rs` `is_complete_sentence()` — added `pron` and `propn` for UD subject detection
- `sugar.rs` `VERB_POS` — added `"verb"` alongside legacy `"v"`

**7 new UD-format tests** added to `mor.rs`, 4 to `dss.rs`, 1 to `sugar.rs`. All 463 talkbank-clan tests pass.

### Remaining Limitation: Default DSS/IPSYN Rule Sets

The **default scoring rules** in `dss.rs` and `ipsyn.rs` still use legacy-only POS patterns. These are passed to `mor_pattern_matches()` which uses prefix matching — patterns like `"pro:sub"` won't match UD `"pron"`, and `"conj:coo"` won't match `"cconj"`.

| Default pattern | Matches UD? | Why |
|----------------|-------------|-----|
| `"v"` | Yes | `"verb".starts_with("v")` |
| `"aux"` | Yes | Exact match |
| `"det:art"` | No | `"det".starts_with("det:art")` = false |
| `"pro:sub"`, `"pro:obj"` | No | `"pron".starts_with("pro:sub")` = false |
| `"cop"` | No | UD has no `cop` POS (copula = `aux`) |
| `"conj:coo"`, `"conj:sub"` | No | `"cconj".starts_with("conj:coo")` = false |
| `"pro:wh"`, `"adv:wh"` | No | UD has no subcategorized POS |

**Impact:** DSS/IPSYN scoring on UD data will undercount categories that depend on these patterns. The structural classification (`classify_mor_word`, `is_complete_sentence`, verb detection) is correct — only the scoring rule-matching is affected.

**Mitigation:** Custom `.scr` rule files override defaults. Users targeting UD data should use UD-aware rule files.

**Future work:** Update default rules to include UD alternatives. This requires a design decision about how to handle the inherent loss of subcategory information in UD format (e.g., UD `pron` = legacy `pro:sub` | `pro:obj` | `pro:per` | `pro:wh` — cannot be distinguished by POS alone, only by features).

## What batchalign Produces (Both v2 and v3)

Both batchalign2 and batchalign3 produce **only** modern UD format:
- Flat POS (lowercase UPOS: `noun`, `verb`, `pron`, `propn`, `aux`, `det`, `adp`, `cconj`, `sconj`)
- No subcategories, no fusional, no compounds, no prefixes
- Features: UD-compatible (`Plur`, `Fin`, `Ind`, `Pres`, `S3`, `Part-Past`, `Part-Pres`)

**batchalign2 was NOT creating legacy MOR.** It used `word.upos.lower()` directly as the POS tag. The UD format has been standard since batchalign2.

## Status Summary

### DONE: UD POS Classifier Fix (Critical Bug)

Fixed 2026-03-08. `classify_mor_word()`, `classify_features()`, `is_complete_sentence()`, and `VERB_POS` now handle both UD and legacy tags. 463 tests pass including 12 new UD-specific tests.

### DONE: Reference Corpus Fix

Fixed 2026-03-08. `mor-gra.cha` rewritten from incorrect fusional format to proper UD. 73/73 reference corpus tests pass.

### DOCUMENTED LIMITATION: Default DSS/IPSYN Scoring Rules

Default rule sets use legacy-only patterns. See "Remaining Limitation" section above. Custom rule files work around this. Updating defaults requires a design decision about subcategory loss in UD.

### NOT NEEDED: Legacy MOR Grammar/Parser Support

All `.cha` `%mor` tiers use modern UD format. Legacy format exists only in:

1. **`%xmor`** — Irrelevant: unparsed tier by definition. 662 files have it alongside proper `%mor`.
2. **`.lab` files** — 120 files in Spanish/Hess. If CLAN commands need to process these, their legacy `%mor` would need grammar/parser support. Old code exists in git history (`06590e99^` for model, `ccca018^` for grammar).
