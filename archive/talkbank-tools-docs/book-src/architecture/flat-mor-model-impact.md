# Impact of Flat %mor Model on CLAN Tools

**Date**: 2026-02-23
**Commit**: `06590e99` (talkbank-model %mor refactor)

## Summary

The `%mor` data model was refactored from a hierarchical CLAN-native structure
to a flat UD-style model. This document records what was dropped and how each
downstream consumer — the 17 CLAN analysis commands and the LSP — is affected.

## What Changed in the Model

### Removed Types and Fields

| Removed | Role in Old Model |
|---------|-------------------|
| `Chunk` enum (`Word` / `Compound` / `Terminator`) | Dispatch on morphological item kind |
| `MorCompound` (with `words: Vec<CompoundWord>`) | Compound word representation (`n\|bird+house`) |
| `AnnotatedChunk` (wrapper with `translation`) | Held main chunk + optional translation gloss |
| `Mor.omission: bool` | Flagged `0`-prefixed omission items |
| `Mor.pre_clitics: Vec<AnnotatedChunk>` | Pre-clitic morphemes (`$`-joined) |
| `MorWord.part_of_speech: PartOfSpeech` | POS tag (opaque newtype) |
| `MorWord.stem: String` | Lemma/stem |
| `MorWord.prefixes: Vec<Prefix>` | Morphological prefixes (`#`-joined) |
| `MorWord.suffixes: Vec<Suffix>` | Morphological suffixes (`-`/`&`-joined) |
| `AnnotatedChunk.translation: Option<Vec<String>>` | Translation gloss (`=` field) |

### New Types and Fields

| New | Role |
|-----|------|
| `MorWord { pos: PosCategory, lemma: MorStem, features: SmallVec<[MorFeature; 4]> }` | Single flat word |
| `Mor { main: MorWord, post_clitics: SmallVec<[MorWord; 2]> }` | Main word + post-clitics only |
| `MorTier.terminator: Option<String>` | Terminator moved from items to tier level |
| `PosCategory` (interned `Arc<str>`) | POS tag, now UD-style (`noun`, `verb`, `pron`) |
| `MorStem` (interned `Arc<str>`) | Lemma |
| `MorFeature` (flat or `key=value`) | Replaces prefixes + suffixes |

## Semantic Consequences

### 1. Compound Words — No Longer Distinct

Old: `Chunk::Compound(MorCompound { words: [w1, w2], part_of_speech })` for `n|bird+house`.

New: Compounds are parsed into a single `MorWord` whose lemma contains the
compound (e.g., `lemma = "bird+house"`). There is no structural way to
distinguish a compound from a simple word — the `+` is part of the lemma string.

**Impact**: Any analysis that counted compound components separately or reported
compound POS distinctly from simple POS would need to split on `+` in the lemma.
No current CLAN command does this.

### 2. Prefixes and Suffixes — Collapsed to Features

Old: `word.prefixes` and `word.suffixes` were separate ordered lists with typed
entries (each implementing `WriteChat`).

New: `word.features` is a single flat list of `MorFeature` values. The prefix
vs suffix distinction is lost. Features serialize with a `-` delimiter
(`-Plur`, `-Pres`, `-3S`), matching the old suffix format, but there is no way
to recover which were originally prefixes.

**Impact**: Any analysis that needed prefix counts separately from suffix counts
(e.g., a morphological complexity metric that weights prefixation differently)
cannot be implemented from the new model alone. No current CLAN command does this.

### 3. Pre-clitics — Dropped

Old: `Mor.pre_clitics` held morphemes joined with `$` (e.g., `$det|the`).

New: Only `post_clitics` remain. Pre-clitics are not represented.

**Impact**: Corpora that use pre-clitic notation will have those items silently
dropped during parsing. If a CLAN analysis needed to count pre-clitics (e.g.,
for Romance language determiner-noun complexes), it cannot. No current CLAN
command accesses pre-clitics.

### 4. Omission Flag — Dropped

Old: `Mor.omission: bool` marked `0`-prefixed items (word present in main tier
but absent from morphological analysis).

New: No omission flag. The parser either produces a `Mor` item or doesn't.

**Impact**: MLU previously skipped omission items. Now all items in
`mor_tier.items` are counted. If the parser still produces items for omitted
words (with some default POS/lemma), MLU counts would increase. If the parser
simply omits them, behavior is equivalent.

### 5. Translation Gloss — Dropped

Old: `AnnotatedChunk.translation: Option<Vec<String>>` held `=gloss` entries.

New: No translation field.

**Impact**: The LSP hover used to display translations. It no longer does. No
CLAN command used translations.

### 6. Terminators — Moved to Tier Level

Old: `Chunk::Terminator(String)` was an item in the `Mor.items` list.

New: `MorTier.terminator: Option<String>` is a separate field on the tier.

**Impact**: MLU previously had to filter out `Chunk::Terminator` items to avoid
counting punctuation as morphemes. Now `mor_tier.items.len()` is correct without
filtering. The graph label builder now reads `mor_tier.terminator` directly.

### 7. POS Tags — New Namespace

Old: CHAT-native tags (`n`, `v`, `pro`, `det`, `co`, `conj`, `prep`, etc.)
via `PartOfSpeech::new("n")`.

New: UD-style tags (`noun`, `verb`, `pron`, `det`, `intj`, `cconj`, `adp`, etc.)
via `PosCategory::new("noun")`. The `PosCategory` type is an interned
`Arc<str>` — any string is valid, so old-style tags from legacy corpora still
round-trip. But the canonical tags produced by the parser have changed.

**Impact**: The LSP POS description table was extended with UD-style mappings.
FREQ's `--mor` mode uses `write_chat()` serialization as the frequency key, so
it will see `noun|cat` instead of `n|cat` — frequency tables from new corpora
will use the new tags. This is a **visible output change** but not a bug.

## Per-Command Impact Assessment

### Commands That Access %mor (2 of 17)

| Command | MOR Usage | Impact | Severity |
|---------|-----------|--------|----------|
| **MLU** | `mor_tier.items.len()` | Simplified: no more omission/terminator filtering. Equivalent if parser produces the same item count. | **None** (behavior-preserving) |
| **FREQ** (`--mor`) | `mor_item.main.write_chat(&mut key)` | Frequency keys change from `n\|cat` to `noun\|cat` for new corpora. Old corpora with old tags are unaffected (tags round-trip). | **Low** (cosmetic, output format change) |

### Commands That Do Not Access %mor (15 of 17)

CHIP, COMBO, COOCCUR, DIST, FREQPOS, GEMLIST, KWAL, MAXWD, MLT, MODREP,
PHONFREQ, TIMEDUR, VOCD, WDLEN — these operate on the main tier, `%pho`,
`%gra`, or header lines. **Zero impact.**

### LSP

| Component | Impact |
|-----------|--------|
| **Hover formatter** (`mor.rs`) | Rewritten. Shows POS/Lemma/Features instead of POS/Stem/Prefixes/Suffixes. No compound breakdown. No translation. No omission indicator. |
| **Stem helper** (`helpers.rs`) | Simplified. Returns `mor.main.lemma` directly, no Chunk dispatch. Compounds show as `bird+house` (lemma string) rather than `bird + house` (joined components). |
| **Graph labels** (`labels.rs`) | No pre-clitic nodes. Terminator node sourced from `mor_tier.terminator`. |
| **POS descriptions** (`pos.rs`) | Extended with 12 UD-style tag descriptions. Old tags still recognized. |
| **Semantic tokens** | Pre-existing breakage from `mor_category` node removal in grammar (separate fix needed). |

## Future CLAN Commands at Risk

Commands not yet ported that **would** be affected by the flat model:

| Potential Command | Old Model Dependency | Mitigation |
|-------------------|---------------------|------------|
| **MORTABLE** | Prefix/suffix decomposition for morphological paradigm tables | Would need to parse features or re-derive from CHAT text |
| **TIERSEARCH** on %mor | Compound structure matching | Would match on lemma string containing `+` |
| **Custom morphological complexity** | Prefix vs suffix counts, compound component counts | Not feasible from the flat model without string parsing |

## Conclusion

The flat model is sufficient for all 17 currently implemented CLAN commands.
The only visible change is that FREQ's `--mor` mode produces UD-style POS tags
in frequency keys for new corpora. Future commands that need fine-grained
morphological decomposition (prefix/suffix/compound) would need to either parse
the `MorFeature` list or the CHAT text representation.
