# Option and Result Audit (Rust Workspace)

## Scope
- Audited `crates/**.rs` for `Option<` and `-> Option<` usage.
- Focused on parser/model semantics where `None` can hide failure state.

## Progress Update (2026-02-10)
- `Utterance.language_metadata` no longer uses `Option<LanguageMetadata>`.
- Replaced with explicit enum state:
  - `UtteranceLanguageMetadata::Uncomputed`
  - `UtteranceLanguageMetadata::Computed { metadata }`
- Rationale:
  - removes ambiguous `None` semantics from a core model field,
  - makes pipeline state explicit for downstream tools,
  - aligns with no-sentinel/no-ambiguous-state policy.

## Current Footprint
- Total `Option<` usages: **788**
- Total `-> Option<` return signatures: **437**
- Total function signatures returning `Option`: **169**
- Parser API surfaces with `errors: &impl ErrorSink` and `-> Option<`: **17**

### `Option<` counts by crate
- `talkbank-model`: 187
- `talkbank-parser`: 186
- `talkbank-lsp`: 112
- `talkbank-direct-parser`: 85
- `talkbank-parser-tests`: 58
- `talkbank-cli`: 50
- `talkbank-transform`: 42
- `talkbank-model` (errors module): 29
- `talkbank-model` (parser_api module): 25

### `-> Option<` counts by crate
- `talkbank-parser`: 138
- `talkbank-lsp`: 77
- `talkbank-direct-parser`: 65
- `talkbank-model`: 63
- `talkbank-parser-tests`: 34
- `talkbank-model` (parser_api): 25

## Classification

### A) High-risk Option usage (replace)
These are fallible parse/conversion paths where `None` loses semantic state and competes with diagnostics:
- `talkbank-model/src/parser_api.rs`: trait methods returning `Option<T>` while also accepting `ErrorSink`
- `talkbank-direct-parser/src/{word,header,mor_tier,gra_tier,pho_tier,file}.rs`
- `talkbank-parser/src/api/parser_api.rs`
- `talkbank-parser/src/parser/tree_parsing/{main_tier/word/mod.rs,postcode/mod.rs,parser_helpers/node_dispatch/pause.rs}`

Recommendation:
- Introduce a shared parse outcome enum for piece parsers, e.g.:
  - `Parsed(T)`
  - `Recovered(T)`
  - `Rejected { reason }`
- For API entrypoints, prefer `Result<T, ParseErrors>` for non-streaming calls.
- Keep streaming sinks, but return explicit outcome instead of `None`.

### B) Medium-risk Option usage (evaluate/convert where semantics exceed binary absence)
Model fields where `None` means multiple things (uncomputed vs unresolved vs not-applicable):
- `talkbank-model/src/model/file/utterance/core.rs`
- `talkbank-model/src/model/language_metadata/metadata.rs` (`tier_language`)
- `talkbank-model/src/model/alignment_set.rs` optional tier alignment fields

Recommendation:
- Replace with explicit state enums when there are 3+ states.
- Keep `Option` only when a field is truly optional and semantically unambiguous.

### C) Low-risk Option usage (keep)
Natural optionality and lookup/accessor patterns:
- Accessors like `utterance.mor() -> Option<&MorTier>`
- Optional syntax slots such as optional `bullet`, optional `terminator`, optional participant metadata fields
- Small utility finders (`find_child_by_kind`, etc.) where `None` unambiguously means "not found"

## Policy Proposal

### Rule 1: No `Option` for fallible parser outcomes
If a function can fail due to invalid input, use `Result` or explicit outcome enum. `Option` is only acceptable for pure "not-present" queries.

### Rule 2: No ambiguous `None` in domain model state
When a field can represent more than two meaningful states, use an enum.

### Rule 3: Keep `Option` for true optional data
If semantics are exactly present/absent and absence is not an error, `Option` remains acceptable.

### Rule 4: Annotate error variants explicitly
When model enums represent validation-relevant states, use `ValidationTagged` derive with:
- `#[validation_tag(error|warning|clean)]` for explicit semantics
- Naming convention fallback:
  - `*Error` => error
  - `*Warning` => warning
- Default (no suffix/annotation) => clean

## Immediate Worklist (ordered)
1. Parser contract redesign in `talkbank-model` (`parser_api` module):
   - Introduce explicit outcome type for tier/word/header parse methods.
   - Adapt tree-sitter and direct parser implementations.
2. Piece parser cleanup:
   - Convert `-> Option<T>` helper chains in direct parser and tree-sitter parser to explicit outcomes.
3. Model state cleanup:
   - Continue replacing ambiguous runtime metadata `Option` fields with enums.
4. Guardrails:
   - Add a CI lint script that flags new `errors: &impl ErrorSink` + `-> Option<` signatures.
   - Implemented as:
     - `scripts/check-errorsink-option-signatures.sh`
     - `scripts/errorsink_option_allowlist.txt`

## Notes
- This audit is not a blanket "remove all Option" change. It targets `Option` where state/error semantics are currently being hidden.
- Existing changes already moved utterance-level language state to an explicit enum and removed fabricated language fallbacks.
