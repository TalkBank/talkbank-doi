# TalkBank CHAT / CLAN Roadmap

**Date:** 2026-03-06 (updated 2026-03-08)
**Scope:** `talkbank-chat`, `talkbank-clan`, `talkbank-chatter`, book/docs, parity testing, converters
**Purpose:** Consolidate the next work into one plan that can be studied and executed deliberately, with strong parity and testing discipline.

## Execution Status (2026-03-07)

### Phase A: Parser Semantic Context Audit — COMPLETE
- A1: Context classification table written at `talkbank-parser-api/CONTEXT-CLASSIFICATION.md`
  - 2/24 methods are context-sensitive (main_tier, utterance — CA mode only)
  - 22/24 are context-free (forwarding to base methods)
- A2: No external callers use fragment parsing — all use whole-file parse (context derived from headers)
- A3: No expansion of `FragmentSemanticContext` needed
- A4: Fragment parity tests already exist for CA mode

### Phase B: Expand Golden Test Coverage — COMPLETE
- B1: Added 9 new golden tests (GEM x2, INDENT, ROLES, TRIM x2, CHAT2TEXT x2, CHAT2ELAN)
- B3: Added 11 option variant golden tests:
  - FREQ: `+t%mor` (morpheme tokens), `+t*CHI` (speaker filter), `+scookie` (word include), `+z1-1` (utterance range), `+t%mor` on eng-conversation
  - MLU: `+t*CHI` (speaker filter), `-t%mor` (word-based MLU)
  - KWAL: multiple `+s` keywords, `+sgoing` on eng-conversation
  - COMBO: `+s` AND search on mor-gra.cha, `+s` AND search on eng-conversation
- Infrastructure: Added `run_rust_filtered()` helper for tests with `FilterConfig` (speaker/word/range filtering)
- Total golden tests: 75 → 98
- All transforms now have golden tests (23/23)
- All converters now have golden tests (14/14)

### Phase C: Command Semantics Cleanup — COMPLETE
- C1: Text-hacking classification audit at `talkbank-clan/TEXT-HACKING-AUDIT.md`
  - Found 4 category (c) files: dss.rs, eval.rs, kideval.rs, postmortem.rs
- C2: Converted eval.rs and kideval.rs to typed `MorTier.items` access
  - Added framework helpers: `extract_mor_tier()`, `classify_mor_item()`, `count_morphemes_typed()`
- C3: Converted dss.rs and ipsyn.rs to typed `MorTier` access (2026-03-08)
  - Added `mor_pattern_matches()`, `any_item_has_pos()` to `framework/mor.rs`
  - Fixed pre-existing bug: DSS "v-PAST" compound patterns now correctly match
  - Updated kideval.rs to use `Vec<Mor>` instead of `Vec<String>` for DSS/IPSYN state
- C4: Full re-audit of remaining 10 files (vocd, combo, kwal, chip, uniq, chains, trnfix, rely, keymap, mortable)
  - All classified as (a) legitimate boundary or (b) generic text-token fallback by policy
  - Zero remaining category (c) violations across all command code
- Remaining: postmortem.rs %mor AST rewriting (transform, not command — Phase D dependency)

### Phase D: Command Parity Work — MOSTLY COMPLETE
- D1: Full parity audit of all 53 golden test snapshot pairs completed
  - Audit document: `talkbank-clan/COMMAND-PARITY-AUDIT.md`
  - **Fixed:** Word matching changed from substring to exact + wildcard across KWAL, COMBO, and WordFilter
    - Added shared `word_pattern_matches()` to `framework/word_filter.rs`
    - KWAL `+scookie` no longer matches "cookies" (CLAN parity)
    - COMBO search terms use exact match (CLAN parity)
    - `+s`/`-s` FilterConfig uses exact match (CLAN parity)
  - **Fixed:** 6 stale @rust snapshots refreshed (MLU, SUGAR, UNIQ, VOCD, WDLEN, CHECK)
  - **Fixed (2026-03-08):** FREQ `+t%mor` clitic splitting — post-clitics now counted as separate items
  - **Fixed (2026-03-08):** PHONFREQ IPA character counting — now includes all Unicode alphabetic + compound markers
  - **Resolved (2026-03-08):** FREQ `+z` range — not a real gap, CLAN requires `+zu`/`+zw`/`+zt` prefix
  - Classified all divergences: 10 format-only, 10 empty-CLAN (test corpus too small)
  - Remaining: MODREP stress markers (cosmetic), converter parity gaps (separate audit)
- postmortem.rs AST-based %mor rewriting still pending

### Phase E: Documentation Modernization — PARTIALLY COMPLETE
- E2: Created tracking artifacts (CONTEXT-CLASSIFICATION.md, TEXT-HACKING-AUDIT.md)
- E3: Created dependent-tier semantics matrix at `book/src/appendices/dependent-tier-semantics.md`
- E4: Created transform taxonomy at `book/src/appendices/transform-taxonomy.md`
- E1: Per-command book audit — deferred to Phase D
- E5: VS Code docs update — deferred

### Updated Baseline (2026-03-08)
- **75 golden tests** with 21 new variant tests (speaker filtering, cross-corpus) for DSS, IPSYN, EVAL, KIDEVAL, VOCD, MLT, MLU, WDLEN, DIST, FLUCALC, CHAINS, COOCCUR, SUGAR, FREQPOS, TIMEDUR
- **453 total tests** in talkbank-clan (448 pass, 5 pre-existing transform/converter failures)
- **0 text-hacking violations** across all command code (full re-audit complete)

## 1. Current Inventory

Concrete numbers grounding the rest of this document:

- **77 commands total**: 34 analysis + 23 transforms + 14 converters + 6 deliberately not implemented
- **75 golden tests**: 42 analysis + 21 transform + 12 converter, producing 122 snapshot files
- **73 reference corpus files** at 100% parser equivalence
- **Status matrix**: `talkbank-clan/book/src/appendices/status-matrix.md` covers all 77 commands
- **VS Code feature assessment**: `talkbank-chatter/vscode/CLAN-FEATURES.md` (572 lines)
- **Fragment context system**: `FragmentSemanticContext` with 24 `*_with_context()` methods in the `ChatParser` trait
- **Source of truth**: `~/CLAN.html` (53,690 lines, local copy of the CLAN manual)

## 2. Working Principles

These are the rules the implementation should now follow.

1. Treat the legacy `CLAN.html` manual as the primary statement of command intent.
2. Treat the current Rust code as the current behavior baseline, not the truth source.
3. Treat the reference corpus, golden tests, and old CLAN binaries as executable evidence.
4. Prefer typed AST operations over raw text manipulation whenever the operation is semantic rather than layout-oriented.
5. If a command or parser fragment needs file semantics, pass explicit context.
6. If we cannot do the correct thing, error explicitly. Do not silently default.
7. Every bug found and fixed must be recorded in docs.
8. Every rewritten path must get tests.

## 3. Current State

### 3.1 What is materially better now

- Much of the semantic CLAN logic in `talkbank-clan` no longer reparses serialized CHAT text.
- `%mor` / `%gra` handling has been moved toward typed AST access where our model supports it.
- `%cod` now has a conservative `talkbank-clan` semantic layer instead of four commands each inventing their own tokenization.
- `TRIM` has been brought back toward legacy CLAN meaning.
- `FIXBULLETS` is materially broader than before and now works on more structured bullet-bearing content.
- Fragment parser parity testing now exists for:
  - headers
  - dependent tiers
  - main tiers
  - utterances
- Fragment parsing now has an explicit semantic context API.
- Public tree-sitter convenience wrappers now obey the same context rules as the trait-backed APIs.

### 3.2 What is still incomplete

- Fragment semantic context is not yet rolled through all callers and not yet meaningfully used by all fragment parsers.
- A number of CLAN commands still need parity review against `CLAN.html`, CLAN binaries, and corpus behavior.
- Some transforms are correctly text-level, but their boundaries and rationale should be tightened.
- The modern `talkbank-clan` book still does not yet absorb all relevant non-GUI content from `CLAN.html`.
- The golden harness is operational (75 tests, 122 snapshots). Coverage needs expansion to more commands and option combinations.
- ~20 command/transform files still use `to_chat_string()` or `split_whitespace()` for semantic inspection (see Section 5.3 audit).
- `CLAN.html` is at `~/CLAN.html`, not inside the workspace tree. Consider symlinking or documenting the path convention.

## 4. Source-of-Truth Method

Every parity task should follow this order.

1. Read the relevant `CLAN.html` section.
2. Read the current Rust implementation.
3. Read existing unit tests and golden tests.
4. If behavior is still unclear, run the legacy CLAN command on targeted examples.
5. Add or extend corpus/golden fixtures before changing code.
6. Make the code change.
7. Add docs for the bug and fix.
8. Run the relevant test matrix.

This should be treated as the standard workflow, not an optional extra.

## 5. Main Workstreams

## 5.1 Fragment Parser Semantic Context

### Goal

Make every fragment parser capable of correct semantics when the caller knows file context, and make context-free use conservative and explicit.

### Why this matters

We already proved that fragment parsing can silently drift from whole-file semantics. `@Comment` bullets and CA omission handling were both examples of this. A fragment API without semantic context is only safe when the fragment language is actually context-free.

### Work items

1. Audit every `*_with_context(...)` method in [`talkbank-parser-api`](../talkbank-chat/crates/talkbank-parser-api/src/lib.rs).
2. Classify each fragment parser into one of three buckets:
   - truly context-free
   - context-aware today
   - context-sensitive in principle but not implemented yet
3. For each context-sensitive parser, decide what semantics belong in `FragmentSemanticContext`.
4. Expand `FragmentSemanticContext` only for semantics we can justify from real behavior.
5. Update real callers to pass context when they already know it.
6. Reject context-dependent constructs explicitly when context is absent.

### Likely next targets

- `parse_word()` if any option/header state changes word interpretation
- selected dependent tiers if file options alter interpretation
- editor/LSP and VS Code fragment parsing surfaces
- any CLI or utility surfaces that parse fragments detached from files

### Testing

- Keep corpus-driven whole-file vs fragment parity tests.
- Add public wrapper tests whenever a convenience API is added or fixed.
- Add targeted regression tests when a new context-sensitive construct is found.

### Deliverables

- Context classification table in docs
- Wider `*_with_context(...)` adoption
- No remaining convenience wrappers that bypass trait semantics

## 5.2 Fragment Parser Entry-Point Parity

### Goal

Ensure isolated parser entrypoints behave like the same parser’s whole-file parse, modulo clearly documented context limitations.

### Work items

1. Continue reference-corpus-driven parity audits for any remaining public fragment surfaces.
2. Keep adding focused regressions when parity tests expose drift.
3. For every intentional mismatch, document it as either:
   - requires context
   - intentionally narrower API
   - bug pending fix

### Known principle

The reference corpus is not just for full files. It should drive fragment parser expectations too.

## 5.3 AST-First CLAN Command Semantics

### Goal

Finish removing hidden semantic text hacking from command reimplementations.

### Current status

This is much better than it was, but still not fully settled.

### Remaining tasks

1. Audit all remaining command paths that serialize AST back to CHAT and then inspect the text.
2. Separate them into:
   - legitimate output/display boundary serialization
   - generic text-token fallback by explicit command policy
   - still-wrong semantic reparsing
3. Eliminate the still-wrong category.

### Specific files identified

**Commands using `to_chat_string()` for semantic inspection** (not just display):
- `vocd.rs`, `combo.rs`, `kwal.rs`, `chip.rs`, `uniq.rs`

**Commands using `split_whitespace()` on serialized tier content:**
- `chains.rs`, `kideval.rs`, `trnfix.rs`, `rely.rs`, `keymap.rs`, `eval.rs`, `dss.rs`, `mortable.rs`

**Transform files with text reparsing:**
- `fixbullets.rs`, `postmortem.rs`, `lines.rs`, `dataclean.rs`

**Framework files:**
- `cod.rs` (intentional: semantic layer), `chat_ast.rs`, `transform.rs`

Each needs classification: (a) legitimate display boundary, (b) generic text-token fallback by policy, (c) still-wrong semantic reparsing.

### Policy question

For commands that currently accept arbitrary tiers and tokenize them generically, decide whether parity actually requires that or whether the command should be restricted to semantically modeled tiers.

## 5.4 `%cod` Semantic Model

### Goal

Stabilize `%cod` handling without overcommitting the shared CHAT grammar too early.

### Current state

- `talkbank-clan` has a local conservative `%cod` semantic layer.
- The shared `talkbank-chat` parser/model still treats `%cod` as bullet-capable text content.
- Real data shows selectors like `<w4>`, `<w4-5>`, `<wl>`, `<W2>`.

### Next steps

1. Validate the clan-local `%cod` model against more real data and CLAN command behavior.
2. Use old CLAN command runs to test `CODES`, `CHAINS`, `KEYMAP`, and `RELY`.
3. Decide whether the clan-local model is stable enough to promote into `talkbank-chat`.
4. If promoted:
   - update parser
   - update model
   - extend corpus/spec fixtures
   - migrate command logic from clan-local layer to shared AST

### Promotion criteria

Only move `%cod` into the shared grammar/model if:
- the item boundaries are stable,
- the selector model is conservative enough,
- the main command semantics are supported,
- roundtripping remains reliable.

## 5.5 `%mor` / `%gra` Structural Improvement

### Goal

Use typed structure where it exists, but do not overclaim parity where our data model is still weaker than CLAN’s.

### Current reality

- `%mor` and `%gra` clearly have structure.
- Our handling is better than before.
- Our `%mor` data model is still impoverished relative to original CLAN.

### Next steps

1. Audit remaining `%mor` / `%gra` command behavior against CLAN semantics rather than legacy string accidents.
2. Identify where our current model is too weak to express intended behavior.
3. Distinguish:
   - command bugs
   - shared model limitations
   - acceptable conservative simplifications
4. Expand the shared model only where justified by command needs and corpus evidence.

### Known Model Gaps

1. **Compound stems opaque**: `MorStem` is an interned `Arc<str>`. Compounds like `fire+truck` stored as raw string — no `CompoundStem { parts }`.
2. **No pre-clitics**: `Mor` has `main: MorWord` + `post_clitics: SmallVec<[MorWord; 2]>` but no `pre_clitics` (French `$pro|je verb|manger`).
3. **No ambiguity chains**: Legacy CLAN uses `^`-separated alternatives (`noun|bank^verb|bank`). No `MorAmbiguity` type. Status matrix notes: “POST: Requires ^-separated ambiguity format not in grammar.”
4. **PosCategory is opaque**: Interned `Arc<str>`, no enum of valid UD POS tags. Content/function word distinction requires string comparison.
5. **MorFeature is flat**: `{ key: Option<Arc<str>>, value: Arc<str> }` — no deeper decomposition for agglutinative/fusional languages.

### Caution

Do not let “typed” become “fictionally precise”. If the current model cannot represent a CLAN distinction, document the limitation instead of implying we support it.

## 5.6 Other “Free Text” Dependent Tiers

### Goal

Systematically determine whether any other dependent tiers need a minimal semantic layer the way `%cod` did.

### Work items

1. Enumerate current free-text dependent tier types in `talkbank-chat`.
2. For each tier:
   - inspect `depfile.cut`
   - inspect `CLAN.html`
   - inspect actual corpus examples
   - inspect command usage
3. Classify each as:
   - genuinely free text
   - bullet/text only
   - minimally structured
   - richly structured and under-modeled
4. Add a doc table summarizing the result.

### Existing Classification

The `DependentTier` enum already classifies all tiers (28 variants):

1. **Fully structured** (7): Mor, Gra, Pho, Mod, Sin, Act, Cod
2. **Bullet-bearing text** (7): Add, Com, Exp, Gpx, Int, Sit, Spa
3. **Simple text** (10): Alt, Coh, Def, Eng, Err, Fac, Flo, Gls, Ort, Par
4. **Special** (4): Tim, Wor, UserDefined, Unsupported

Classification exists in code; the doc table should formalize it and assess which text tiers might need structure.

### Expected output

A stable “dependent tier semantics matrix” saying which tiers should remain raw text and which should gain structure.

## 5.7 Transform Classification and Cleanup

### Goal

Keep text-level transforms where appropriate, but make sure semantic transforms are not implemented as raw text hacks.

### Full 23-Transform Inventory

**Text/layout** (by design): LONGTIER, LINES, INDENT, LOWCASE, CHSTRING

**Structured/AST** (operate on typed model): QUOTES, ORT, RETRACE, COMBTIER, POSTMORTEM, FLO, FIXBULLETS, TIERORDER, TRIM, MAKEMOD, COMPOUND, REPEAT, ROLES, DATES, DELIM

**Mixed/needs audit**: DATACLEAN, FIXIT, GEM

4 transforms use text reparsing patterns: `fixbullets.rs`, `postmortem.rs`, `lines.rs`, `dataclean.rs`.

### Remaining tasks

1. Write down the transform taxonomy clearly in docs.
2. Re-check each transform against the taxonomy.
3. Add tests that lock any intentional text-level behavior to its stated scope.
4. Continue removing silent structural degradation.

## 5.8 CLAN Command Parity Audit

### Goal

Bring `talkbank-clan` command behavior as close as reasonably possible to legacy CLAN while keeping principled AST and error behavior.

### Priority order

1. Commands whose current meaning diverges from the manual
2. Commands with evidence of semantic underimplementation
3. Commands whose docs are now better than their code
4. Commands whose code is good but docs still lag

### Current high-value targets

FIXBULLETS, TIERORDER, KWAL, and RELY all already have golden tests with "Verified" parity in the status matrix. The priority is now **edge cases and option variants**, not initial parity work. Focus on:

- option combinations not yet covered by golden tests
- commands where `CLAN.html` (at `~/CLAN.html`) clearly describes richer behavior than we currently implement
- commands with no golden tests yet

Reference: `talkbank-clan/book/src/appendices/status-matrix.md`

### Per-command parity checklist

For each command:

1. Read `CLAN.html`.
2. Check whether the local book chapter reflects it.
3. Compare current Rust behavior.
4. Find or generate minimal corpus fixtures.
5. Run legacy CLAN when possible.
6. Add or extend golden tests.
7. Fix code.
8. Document bug, fix, and any remaining divergence.

## 5.9 Golden Test Generation from Legacy CLAN

### Goal

Exploit the fact that we can run old CLAN binaries to generate stronger executable parity baselines.

### Why this matters

The manual explains intent, but binaries settle edge behavior. For tricky commands, we should not guess.

### Current state

The harness is operational:
- `tests/clan_golden.rs` with `run_clan()` function, `strip_clan_header()` normalization, insta snapshots
- `tests/converter_golden.rs` and `tests/transform_golden.rs` for those categories
- `scripts/parity-check.sh` for batch comparison (10 commands across corpus)
- 75 golden tests producing 122 snapshots

Infrastructure is complete; the work is about **expansion**, not creation.

### Work items

1. Expand coverage to more commands and option variants.
2. Add converter `@clan` parity for converters that currently have `@rust`-only snapshots (CHAT2SRT, LAB2CHAT, LENA2CHAT, RTF2CHAT).
3. Add golden tests for CHAT2TEXT and CHAT2ELAN (no snapshots yet).
4. Document normalization rules as they arise (header stripping already done).

## 5.10 Reference Corpus and Spec Growth

### Goal

Use the sacred reference corpus and spec machinery to drive parser correctness, not just parser unit tests.

### Work items

1. Continue adding real constructs when corpus evidence proves they exist.
2. Prefer real corpus-backed examples over invented test strings when possible.
3. Add fragment-focused assertions derived from the reference corpus.
4. Expand coverage for underrepresented headers, dependent tiers, and option-sensitive constructs.

### Immediate candidates

- more option-sensitive examples
- more bullet-bearing headers
- more `%cod` selector forms
- more edge-case dependent tiers that appear in real corpora

## 5.11 Documentation Modernization

### Goal

Make the `talkbank-clan` book the modern command manual, incorporating and improving the non-GUI content from `CLAN.html`.

### Work items

1. Continue the command-by-command manual audit.
2. For each command chapter:
   - incorporate legacy command intent
   - explain modern Rust behavior
   - document divergences and bugs
   - separate GUI content from CLI/semantic content
3. Move GUI/editor workflow material toward the VS Code extension docs rather than the command book.
4. Keep the status matrix and audit appendix current.

### Important rule

Docs should never quietly describe desired behavior as if it were already implemented.

## 5.12 VS Code Extension Documentation Split

### Goal

Carry forward the GUI/editor functionality from CLAN documentation into the VS Code extension docs, not into the CLI command book.

### Work items

`CLAN-FEATURES.md` (572 lines) already exists in `talkbank-chatter/vscode/` with a detailed feature parity assessment. The work is refinement and updates, not creation from scratch.

1. Extract GUI-related material from `CLAN.html` during command audits.
2. Classify it as:
   - command semantics
   - editor workflow
   - media workflow
   - legacy GUI-only behavior
3. Update `CLAN-FEATURES.md` with findings from ongoing command audits.

## 5.13 Testing Policy and CI Discipline

### Goal

Make “all new code written or rewritten must be tested” an enforced working norm.

### Work items

1. Maintain a test checklist for each change type:
   - parser/model change
   - command parity change
   - transform change
   - public API change
   - doc-only parity clarification
2. Prefer:
   - unit tests for local invariants
   - corpus/spec tests for syntax/model behavior
   - golden tests for CLAN parity
   - public API tests for wrapper surfaces
3. Keep bug-fix docs paired with tests in the same change.
4. Where feasible, add CI gates or scripts that make the expected test set obvious.

## 5.14 Converter Parity

### Goal

Bring the 14 converters to the same parity discipline as analysis commands and transforms.

### Full converter inventory

| Converter | Golden Test | `@clan` Parity |
|-----------|:-----------:|:--------------:|
| CHAT2TEXT | — | — |
| CHAT2ELAN | — | — |
| CHAT2PRAAT | Yes | Yes |
| CHAT2SRT | Yes | — |
| ELAN2CHAT | Yes | Yes |
| LAB2CHAT | Yes | — |
| LENA2CHAT | Yes | — |
| LIPP2CHAT | Yes | Yes |
| PLAY2CHAT | Yes | Yes |
| PRAAT2CHAT | Yes | Yes |
| RTF2CHAT | Yes | — |
| SALT2CHAT | Yes | Yes |
| SRT2CHAT | Yes | Yes |
| TEXT2CHAT | Yes | Yes |

### Work items

1. Add golden tests for CHAT2TEXT and CHAT2ELAN.
2. Add `@clan` parity snapshots for CHAT2SRT, LAB2CHAT, LENA2CHAT, RTF2CHAT where legacy binaries are available.
3. Review converter files for text-hacking patterns (see `chat2elan.rs` using `find(":\t")` to extract speaker-prefixed text).

## 6. Concrete Near-Term Execution Order

This is the recommended sequence for the next serious pass.

### Phase A: Finish parser semantics infrastructure

1. Audit remaining fragment parsers for real semantic-context needs. Note: `FragmentSemanticContext` currently carries only `@Options` flags with `ca_mode()` and `bullets_mode()`. Audit whether additional context is needed.
2. Expand context-aware behavior only where justified.
3. Push `*_with_context(...)` into real callers that already know file semantics.
4. Keep parity tests growing from the reference corpus.

### Phase B: Expand parity coverage

1. Expand golden test coverage (harness operational, 75 tests). Priority: option variants, converter parity, edge-case fixtures.
2. Add `@clan` parity for converters with available legacy binaries.
3. Add normalization rules and document them.

**Note:** Phases A and B are independent and can run in parallel.

### Phase C: Finish command semantics cleanup

1. Revisit remaining generic text-token fallbacks.
2. Tighten `%cod` command behavior under real CLAN outputs.
3. Continue `%mor` / `%gra` cleanup carefully, respecting model limitations.
4. Triage the ~20 files identified in Section 5.3 audit.

### Phase D: Finish command parity work

1. `FIXBULLETS`
2. `TIERORDER`
3. `KWAL`
4. `RELY`
5. next commands identified by the manual audit

### Phase E: Finish documentation modernization

1. Continue through the remaining command chapters.
2. Keep the audit appendix and status matrix current.
3. Split GUI material into extension-facing docs.

## 7. Open Design Questions

These need deliberate answers rather than incremental drift.

1. Which fragment semantics truly belong in `FragmentSemanticContext`?
2. When should a command accept generic tier-token text, and when should it reject unsupported tiers?
3. When is a clan-local semantic layer appropriate versus promotion into `talkbank-chat`?
4. How much of CLAN’s `%mor` richness should be modeled in shared AST versus handled conservatively in commands?
5. Which GUI workflows from legacy CLAN are actually targets for the VS Code extension?

## 8. Done Means

This work is in good shape when all of the following are true.

1. Fragment parser entrypoints either match whole-file semantics or explicitly require context.
2. Public parser wrappers do not bypass semantic checks.
3. Semantic CLAN command logic does not secretly operate by reparsing serialized CHAT.
4. Remaining text-level transforms are text-level by design and documented as such.
5. Major command behavior is checked against `CLAN.html`, tests, and legacy CLAN output.
6. The `talkbank-clan` book is the best current command manual for non-GUI usage.
7. GUI/editor material is documented in the extension domain instead of leaking into CLI docs.
8. Every material rewrite ships with tests and bug documentation.

## 9. Suggested Tracking Artifacts

To keep this manageable, maintain:

- one command parity tracker — **partially exists**: `status-matrix.md` covers all 77 commands
- one fragment parser context tracker — **does not exist yet**
- one dependent-tier semantics matrix — **partially exists**: `DependentTier` enum classifies all 28 variants; needs doc formalization
- one legacy CLAN golden fixture index — **partially exists**: snapshot naming convention (`@clan`/`@rust`) serves as implicit index
- one docs modernization checklist — **does not exist yet**

This roadmap should be updated as those trackers become more concrete.
