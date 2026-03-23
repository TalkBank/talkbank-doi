# Language-Specific Documentation Audit

**Status:** Current
**Last updated:** 2026-03-23 11:55 EDT

## Problem

Language-specific documentation in batchalign3 is fragmented across multiple
files (user guide, reference, architecture) and focused on implementation
details rather than giving a comprehensive picture of: what works, what
doesn't, what the limitations are of various models/libraries, and what's
planned.

Each language with special treatment needs a single authoritative page covering:
1. **What we support** — which pipeline stages have language-specific behavior
2. **How it works** — which models/libraries, which code paths
3. **What's verified** — tested with real models, empirical results
4. **Known limitations** — model quality issues, edge cases, failure modes
5. **Future work** — planned improvements, open questions

## Current State

### Cantonese (`yue`) — most extensive, most fragmented
- `reference/cantonese-processing.md` (267 lines) — text normalization, char tokenization
- `architecture/hk-cantonese-engines.md` (400 lines) — ASR/FA engine architecture
- `user-guide/hk-cantonese-engines.md` (130 lines) — user-facing engine guide
- `reference/chinese-word-segmentation.md` (128 lines) — word segmentation (new)
- **Missing:** unified Cantonese language support page tying all of this together
- **Missing:** PyCantonese dictionary coverage analysis
- **Missing:** known limitations of each ASR engine for Cantonese child speech
- **Missing:** comparison of engines on real child Cantonese data

### Mandarin (`cmn`/`zho`) — minimal
- Covered only in `chinese-word-segmentation.md` (new) and `language-specific-processing.md`
- **Missing:** dedicated page covering Mandarin-specific ASR, number expansion (Chinese numbers use `num2chinese.rs`), word segmentation quality
- **Missing:** Stanza `zh` tokenizer accuracy analysis (we now know it's imperfect)

### Japanese (`jpn`) — single page
- `reference/japanese-morphosyntax.md` — retokenize, Stanza `combined` package, POS overrides
- **Missing:** known limitations, whitespace artifact handling quality, compound verb handling

### Hebrew (`heb`) — single page
- `reference/hebrew-morphosyntax.md` — RTL handling, Stanza config
- **Missing:** known limitations, RTL punctuation edge cases

### Other languages with special treatment
- **Korean (`kor`)** — MWT exclusion, similar CJK issues to Chinese/Japanese
- **Arabic (`ara`)** — RTL, MWT handling
- **Thai (`tha`)** — no word boundaries in script (similar to CJK issue)
- **Vietnamese (`vie`)** — tone marks, MWT exclusion
- **None of these have dedicated documentation**

## Proposed Structure

For each language with significant special treatment, create:

```
book/src/reference/languages/
├── cantonese.md      # Comprehensive: ASR engines, normalization, word seg, FA, limitations
├── mandarin.md       # Word seg, number expansion, Stanza quality, limitations
├── japanese.md       # Retokenize, combined package, POS overrides, limitations
├── hebrew.md         # RTL, morphosyntax config, limitations
└── overview.md       # Links to all, comparison table of what's special per language
```

Each page follows the same template:
1. Language code(s) and Stanza mapping
2. Pipeline stages with special behavior (table)
3. Models and libraries used (with versions)
4. Verified behavior (links to tests)
5. Known limitations (empirically discovered)
6. Future work

## Priority

1. **Cantonese** — most complex, active user group (PolyU team), most fragmented docs
2. **Mandarin** — active need (same user group), known Stanza limitations
3. **Japanese** — existing retokenize users, could use limitations section
4. **Hebrew** — less active but needs RTL documentation
5. **Others** — document as issues arise

## "HK" Branding Must Go

The "HK" prefix on engine names and documentation is a vestige of the abandoned
plugin concept. Cantonese ASR/FA engines are not "HK plugins" — they're
engine alternatives for Cantonese, the same way Whisper and Rev.AI are engine
alternatives for English.

The consolidation should:
- Rename `hk-cantonese-engines.md` → `cantonese.md` (comprehensive)
- Remove "HK" prefix from docs, headings, and user-facing descriptions
- Keep "hk" in code identifiers (`inference/hk/`, `AsrBackendV2::HkTencent`)
  as a separate cleanup — code renaming is higher risk and can be done later

## Relationship to Existing Pages

The existing fragmented pages should be **consolidated** into comprehensive
language pages. The "HK engines" architecture page content merges into the
Cantonese page's architecture section. There should be no standalone "HK" pages.
