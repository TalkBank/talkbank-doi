# TalkBank CHAT / CLAN Roadmap

**Date:** 2026-03-06  
**Scope:** `talkbank-chat`, `talkbank-clan`, `talkbank-chatter`, book/docs, parity testing  
**Purpose:** Consolidate the next work into one plan that can be studied and executed deliberately, with strong parity and testing discipline.

## 1. Working Principles

These are the rules the implementation should now follow.

1. Treat the legacy `CLAN.html` manual as the primary statement of command intent.
2. Treat the current Rust code as the current behavior baseline, not the truth source.
3. Treat the reference corpus, golden tests, and old CLAN binaries as executable evidence.
4. Prefer typed AST operations over raw text manipulation whenever the operation is semantic rather than layout-oriented.
5. If a command or parser fragment needs file semantics, pass explicit context.
6. If we cannot do the correct thing, error explicitly. Do not silently default.
7. Every bug found and fixed must be recorded in docs.
8. Every rewritten path must get tests.

## 2. Current State

### 2.1 What is materially better now

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

### 2.2 What is still incomplete

- Fragment semantic context is not yet rolled through all callers and not yet meaningfully used by all fragment parsers.
- A number of CLAN commands still need parity review against `CLAN.html`, CLAN binaries, and corpus behavior.
- Some transforms are correctly text-level, but their boundaries and rationale should be tightened.
- The modern `talkbank-clan` book still does not yet absorb all relevant non-GUI content from `CLAN.html`.
- We still need stronger golden generation from actual legacy CLAN command runs.

## 3. Source-of-Truth Method

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

## 4. Main Workstreams

## 4.1 Fragment Parser Semantic Context

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

## 4.2 Fragment Parser Entry-Point Parity

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

## 4.3 AST-First CLAN Command Semantics

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

### Specific areas to study

- generic `--tier` fallbacks in `CHAINS`, `KEYMAP`, `RELY`
- any commands still using serialized main-tier text for semantic analysis
- commands with mixed semantic and presentation responsibilities

### Policy question

For commands that currently accept arbitrary tiers and tokenize them generically, decide whether parity actually requires that or whether the command should be restricted to semantically modeled tiers.

## 4.4 `%cod` Semantic Model

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

## 4.5 `%mor` / `%gra` Structural Improvement

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

### Caution

Do not let “typed” become “fictionally precise”. If the current model cannot represent a CLAN distinction, document the limitation instead of implying we support it.

## 4.6 Other “Free Text” Dependent Tiers

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

### Expected output

A stable “dependent tier semantics matrix” saying which tiers should remain raw text and which should gain structure.

## 4.7 Transform Classification and Cleanup

### Goal

Keep text-level transforms where appropriate, but make sure semantic transforms are not implemented as raw text hacks.

### Current understanding

Probably acceptable as text/layout transforms:
- `LONGTIER`
- `LINES`
- `INDENT`
- much of `DATACLEAN`

Structured or explicitly guarded already:
- `QUOTES`
- `ORT`
- `RETRACE`
- `COMBTIER`
- `POSTMORTEM` now errors rather than silently degrading `%mor`

### Remaining tasks

1. Write down the transform taxonomy clearly in docs.
2. Re-check each transform against the taxonomy.
3. Add tests that lock any intentional text-level behavior to its stated scope.
4. Continue removing silent structural degradation.

## 4.8 CLAN Command Parity Audit

### Goal

Bring `talkbank-clan` command behavior as close as reasonably possible to legacy CLAN while keeping principled AST and error behavior.

### Priority order

1. Commands whose current meaning diverges from the manual
2. Commands with evidence of semantic underimplementation
3. Commands whose docs are now better than their code
4. Commands whose code is good but docs still lag

### Current high-value targets

- `FIXBULLETS`
- `TIERORDER`
- `KWAL`
- `RELY`
- any command where `CLAN.html` clearly describes richer behavior than we currently implement

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

## 4.9 Golden Test Generation from Legacy CLAN

### Goal

Exploit the fact that we can run old CLAN binaries to generate stronger executable parity baselines.

### Why this matters

The manual explains intent, but binaries settle edge behavior. For tricky commands, we should not guess.

### Work items

1. Build a repeatable harness for running legacy CLAN commands on fixture corpora.
2. Store inputs and normalized outputs suitable for golden comparison.
3. Document normalization rules where CLAN output contains unstable formatting.
4. Tag tests by command and option set.

### First commands to prioritize

- `TRIM`
- `FIXBULLETS`
- `TIERORDER`
- `KWAL`
- `RELY`
- `%cod` commands

### Design requirement

The harness should make it easy to add a new command fixture once and keep reusing it.

## 4.10 Reference Corpus and Spec Growth

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

## 4.11 Documentation Modernization

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

## 4.12 VS Code Extension Documentation Split

### Goal

Carry forward the GUI/editor functionality from CLAN documentation into the VS Code extension docs, not into the CLI command book.

### Work items

1. Extract GUI-related material from `CLAN.html` during command audits.
2. Classify it as:
   - command semantics
   - editor workflow
   - media workflow
   - legacy GUI-only behavior
3. Write or update a VS Code extension plan/doc set for the editor workflows we actually support or intend to support.

## 4.13 Testing Policy and CI Discipline

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

## 5. Concrete Near-Term Execution Order

This is the recommended sequence for the next serious pass.

### Phase A: Finish parser semantics infrastructure

1. Audit remaining fragment parsers for real semantic-context needs.
2. Expand context-aware behavior only where justified.
3. Push `*_with_context(...)` into real callers that already know file semantics.
4. Keep parity tests growing from the reference corpus.

### Phase B: Strengthen parity harnesses

1. Build the legacy CLAN golden harness.
2. Start with the highest-risk commands.
3. Add normalization rules and document them.

### Phase C: Finish command semantics cleanup

1. Revisit remaining generic text-token fallbacks.
2. Tighten `%cod` command behavior under real CLAN outputs.
3. Continue `%mor` / `%gra` cleanup carefully, respecting model limitations.
4. Review any remaining semantic reparsing of serialized CHAT.

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

## 6. Open Design Questions

These need deliberate answers rather than incremental drift.

1. Which fragment semantics truly belong in `FragmentSemanticContext`?
2. When should a command accept generic tier-token text, and when should it reject unsupported tiers?
3. When is a clan-local semantic layer appropriate versus promotion into `talkbank-chat`?
4. How much of CLAN’s `%mor` richness should be modeled in shared AST versus handled conservatively in commands?
5. Which GUI workflows from legacy CLAN are actually targets for the VS Code extension?

## 7. Done Means

This work is in good shape when all of the following are true.

1. Fragment parser entrypoints either match whole-file semantics or explicitly require context.
2. Public parser wrappers do not bypass semantic checks.
3. Semantic CLAN command logic does not secretly operate by reparsing serialized CHAT.
4. Remaining text-level transforms are text-level by design and documented as such.
5. Major command behavior is checked against `CLAN.html`, tests, and legacy CLAN output.
6. The `talkbank-clan` book is the best current command manual for non-GUI usage.
7. GUI/editor material is documented in the extension domain instead of leaking into CLI docs.
8. Every material rewrite ships with tests and bug documentation.

## 8. Suggested Tracking Artifacts

To keep this manageable, maintain:

- one command parity tracker
- one fragment parser context tracker
- one dependent-tier semantics matrix
- one legacy CLAN golden fixture index
- one docs modernization checklist

This roadmap should be updated as those trackers become more concrete.
