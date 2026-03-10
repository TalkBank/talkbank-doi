# Comment Quality Plan and Tracker

Last updated: 2026-02-26

## Goal

Add high-quality, human-written comments across TalkBank source code, with a methodical review process and explicit progress tracking.

This effort is **not** a bulk rewrite. It is a deliberate pass over each file for:

- intent and design rationale,
- invariants and assumptions,
- edge cases and failure modes,
- links to external specs/manuals when behavior depends on them.

## Scope Baseline

Baseline scan (workspace-wide, excluding build/cache/venv directories) for primary source languages:

- Rust (`.rs`): 1,875
- Python (`.py`): 235
- TypeScript (`.ts/.tsx/.mts/.cts`): 71
- Total in scope: 2,181 files

Breakdown by repo:

| Repo | Rust | Python | TypeScript | Total |
|------|-----:|-------:|-----------:|------:|
| `batchalign3` | 823 | 180 | 31 | 1,034 |
| `talkbank-chat` | 906 | 3 | 0 | 909 |
| `talkbank-chatter` | 105 | 0 | 39 | 144 |
| `talkbank-clan` | 39 | 0 | 0 | 39 |
| `batchalign-hk-plugin` | 0 | 37 | 0 | 37 |
| `talkbank-private` | 0 | 12 | 0 | 12 |
| `tree-sitter-talkbank` | 2 | 3 | 1 | 6 |

Workload split (same baseline):

- non-test-like paths: 1,541 files
- test-like paths: 640 files

## Non-Negotiable Rules

1. No automated comment generation or bulk templated rewrites.
2. No filler comments (`Runs ...`, `Type representing ...`, `Parses ...` without real context).
3. Every file is audited; comments are added only where they improve understanding.
4. Generated/vendor code is marked `skip-generated` or `skip-vendored` in the tracker, not edited directly.
5. Every comment change lands with targeted verification (build/tests relevant to touched files).

## Comment Quality Rubric (Per Comment)

A comment is accepted only if it does at least one of the following:

- explains **why** this approach exists,
- states an **invariant** the code relies on,
- documents an **edge case** or failure mode,
- explains a **cross-module contract**,
- maps behavior to an external reference (e.g., CHAT/CLAN manuals) when required.

A comment is rejected if it:

- repeats obvious code mechanics,
- can be replaced by a better identifier name,
- uses generic boilerplate phrasing,
- is stale relative to actual code behavior.

## Per-File Workflow

1. Audit
   - read file and nearby call sites,
   - identify non-obvious behaviors worth documenting.
2. Draft plan
   - list proposed comment insertions/rewrites with line anchors.
3. Manual edit
   - apply only planned comments.
4. Self-review
   - run rubric above against each new/edited comment.
5. Verification
   - run relevant formatter/tests.
6. Tracker update
   - record status and notes (what was documented and why).

## Status Codes

- `todo`: not audited yet
- `auditing`: currently being reviewed
- `drafted`: candidate comments prepared
- `in_review`: awaiting human review
- `merged`: approved and landed
- `skip-generated`: generated file, no manual edits
- `skip-vendored`: third-party/vendored file, no manual edits
- `blocked`: needs design decision before comments can be finalized

## Phased Plan

### Phase 0: Guardrails (Day 1)

- Establish rubric and status codes (this doc).
- Create/update ignore rules for transient artifacts (`__pycache__`, `.pyc`).
- Agree maximum batch size per PR (recommended: 3-8 files).

### Phase 1: Runtime-Critical Core (Multi-day)

Priority order (approved 2026-02-25):

1. `talkbank-chat` core parser/model/error crates
2. `talkbank-chatter` LSP/CLI core behavior
3. `batchalign3` runtime pipelines and Rust bridge
4. `talkbank-clan` core analysis/transforms
5. `batchalign-hk-plugin` engine adapters
6. `tree-sitter-talkbank` grammar-facing bindings/docs glue

### Phase 2: Tests and Tooling

- Add clarifying comments where test intent is non-obvious.
- Document edge-case fixtures and regression rationale.

### Phase 3: Sweep and Consistency Pass

- Remove remaining low-value comments.
- Ensure style consistency across Rust/Python/TypeScript.
- Final spot-audit by subsystem owner.

## Tracker Dashboard (Repo Level)

| Repo | Total | Audited | Commented | Skipped (gen/vendor) | In Review | Merged |
|------|------:|--------:|----------:|---------------------:|----------:|-------:|
| `talkbank-chat` | 906 | 906 | 906 | 0 | 0 | 906 |
| `batchalign3` | 1,034 | 111 | 111 | 0 | 0 | 111 |
| `talkbank-chatter` | 144 | 91 | 91 | 0 | 0 | 91 |
| `talkbank-clan` | 39 | 32 | 32 | 0 | 0 | 32 |
| `batchalign-hk-plugin` | 37 | 5 | 5 | 0 | 0 | 5 |
| `talkbank-private` | 12 | 2 | 2 | 0 | 0 | 2 |
| `tree-sitter-talkbank` | 6 | 5 | 5 | 0 | 0 | 5 |

## Audit Backfill: 2026-02-26

Scope:

- `talkbank-chat` (`crates/**/*.rs`)
- `talkbank-chatter` (`crates/**/*.rs`)
- `batchalign3/rust/crates/**/*.rs` (including `talkbank-model` test modules)

What was audited:

- low-value mechanical doc phrasing (`Runs ...`, `Type representing ...`, `Enum variants for ...`, `Builds a value ...`)
- duplicated `# Related CHAT Manual Sections` blocks
- module docs that started with anchors only (no crates.io-facing summary line)
- manual-link integrity against regenerated manuals:
  - `/Users/chen/web/talkbank-web/site/0info/manuals/CHAT.html`
  - `/Users/chen/web/talkbank-web/site/0info/manuals/CLAN.html`

Result snapshot after fixes:

- mechanical phrase findings: `0`
- duplicate manual section blocks: `0`
- anchor-only first-line module docs: `0`
- unique manual URLs checked: `3,826`
- missing manual anchors: `0`

Notes:

- This pass included cleanup in parser-test and test-harness modules so contributor-facing test code no longer carries templated/filler comments.
- `cargo fmt` and `cargo check` were re-run in touched Rust workspaces after edits.
- Dashboard updated: `talkbank-chat` total corrected from 909 → 906 (3 files removed since baseline). All 906 files audited across Waves 1–28 (W28-01 through W28-79 sub-batches), with 0 placeholder comments, 0 missing CHAT anchors, and 0 filler doc comments remaining.

## Wave Note: 2026-02-26 (API Docs + UTF-8 Mapping Fix)

Scope:

- `talkbank-chat/crates/talkbank-errors`
- `talkbank-chat/crates/talkbank-parser-api`
- `talkbank-chat/crates/talkbank-pipeline`

Highlights:

- Expanded crates.io-facing API contracts and invariants for:
  - `line_map`
  - `chat_formatting`
  - `config` / `configurable_sink` / `offset_adjusting_sink`
  - parser API dependent-tier method inputs
  - pipeline validation option semantics
- Corrected `ValidationConfig::strict()` behavior to actually escalate unmapped warnings to errors (with explicit-override precedence documented and tested).
- Corrected UTF-8 offset accounting in `chat_formatting` span mapping (byte offsets now tracked in bytes, matching `Span`/`SourceLocation` semantics).

Verification:

- `cargo fmt -p talkbank-errors -p talkbank-parser-api -p talkbank-pipeline`
- `cargo check -p talkbank-errors -p talkbank-parser-api -p talkbank-pipeline -q`
- `cargo test -p talkbank-errors -q`
- `cargo test -p talkbank-parser-api -q`
- `cargo test -p talkbank-pipeline -q`

## Wave Note: 2026-02-26 (talkbank-derive Crates.io Pass)

Scope:

- `talkbank-chat/crates/talkbank-derive/src/*.rs`

Highlights:

- Upgraded module-level docs from terse labels to contributor-facing explanations of:
  - macro contracts
  - attribute behavior (`#[semantic_eq(skip)]`, `#[span_shift(skip)]`, `#[validation_tag(...)]`)
  - generated API semantics for `#[error_code_enum]`
- Corrected public docs example in derive crate root for `#[error_code_enum]` to match the actual `#[code("...")]` attribute syntax.
- Clarified `SemanticEq` derive note that companion `SemanticDiff` impl is emitted in tandem.

Verification:

- `cargo fmt -p talkbank-derive`
- `cargo check -p talkbank-derive -q`
- `cargo test -p talkbank-derive -q`

## Wave Note: 2026-02-26 (talkbank-lsp Alignment Docs Pass)

Scope:

- `talkbank-chatter/crates/talkbank-lsp/src/alignment/**`

Highlights:

- Rewrote module and API docs in the alignment hover subsystem to clarify:
  - separation between model-owned alignment semantics and LSP presentation code
  - resolver responsibilities by tier (`main`, `%mor`, `%gra`, `%pho`, `%sin`)
  - formatter intent (compact hover output vs full transcript rendering)
  - index-mapping helper roles and guardrails
- Replaced terse/ambiguous helper docs with contributor-facing contracts for entry points and internal utilities.

Verification:

- `cargo fmt -p talkbank-lsp`
- `cargo check -p talkbank-lsp -q`
- `cargo test -p talkbank-lsp -q`

## Wave Note: 2026-02-26 (batchalign-core NLP Docs Pass)

Scope:

- `batchalign3/rust/crates/batchalign-core/src/nlp/**`

Highlights:

- Expanded contributor-facing docs for the NLP boundary layer:
  - module responsibilities and architecture (`nlp/mod.rs`)
  - typed callback contract rationale (`nlp/types.rs`)
  - UD→CHAT mapping pipeline stages and context semantics (`nlp/mapping.rs`)
  - `%mor` sanitization purpose (`nlp/validation.rs`)
  - language-specific override intent for English/French/Japanese rule modules
- Kept this wave docs-only inside `nlp` to avoid mixing with unrelated in-progress work in other `batchalign3` files.

Verification:

- `cargo fmt -p batchalign-core`
- `cargo check -p batchalign-core -q`
- `cargo test -p batchalign-core -q`

## Wave Note: 2026-02-26 (talkbank-model Header Validation Docs Pass)

Scope:

- `batchalign3/rust/crates/talkbank-model/src/validation/header/**`

Highlights:

- Rewrote module-level docs across the header-validation subsystem to explain:
  - split between single-header dispatch (`validate`) and whole-file invariants (`structure`)
  - cross-header contracts (`@ID` speaker references vs `@Participants`)
  - gem balancing semantics for `@Bg` / `@Eg` / `@G`
- Replaced generic manual links with header-specific CHAT anchors including:
  - `#UTF8_Header`, `#Begin_Header`, `#Languages_Header`, `#Participants_Header`,
    `#ID_Header`, `#Date_Header`, `#Options_Header`, `#Media_Header`, `#End_Header`,
    `#Bg_Header`, `#Eg_Header`, `#G_Header`
- Fixed a stale suggestion URL in unknown-header diagnostics to the canonical
  manual path:
  - `https://talkbank.org/0info/manuals/CHAT.html#File_Headers`

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-transform Unified Cache Docs + Alignment Key Fix)

Scope:

- `batchalign3/rust/crates/talkbank-transform/src/unified_cache/**`

Highlights:

- Rewrote contributor-facing docs across unified-cache modules (`mod`, `cache_impl`,
  `types`, `schema_init`, `cache_utils`, `corpus_ops`, `maintenance_ops`,
  `validation_ops`, `roundtrip_ops`, `rendering`) to document real contracts:
  - cache key dimensions (path hash, version, alignment, parser kind)
  - ownership boundaries between facade and SQL operation modules
  - lifecycle behavior (idempotent schema init, freshness checks, pruning)
- Fixed a correctness bug in validation cache lookup:
  - `get_validation` now filters by `check_alignment` in SQL, instead of loading
    an arbitrary row and comparing post hoc.
- Tightened validation write semantics:
  - `set_validation` now replaces same-version validation rows across alignment
    modes for the same file key, preventing ambiguous duplicate rows.
- Added regression coverage:
  - `validation_ops::tests::validation_cache_keys_on_alignment_mode`

Verification:

- `cargo fmt -p talkbank-transform`
- `cargo check -p talkbank-transform -q`
- `cargo test -p talkbank-transform -q`

## Wave Note: 2026-02-26 (validation_runner Contracts + Cancellation/Worker Fixes)

Scope:

- `batchalign3/rust/crates/talkbank-transform/src/lib.rs`
- `batchalign3/rust/crates/talkbank-transform/src/lock_helpers.rs`
- `batchalign3/rust/crates/talkbank-transform/src/validation_runner/{cache,config,helpers,runner,types,worker,tests}.rs`

Highlights:

- Rewrote top-level docs and public-type docs in `validation_runner` and crate root
  to describe contributor-facing contracts instead of terse labels.
- Fixed cancellation semantics:
  - Dropping the cancellation sender no longer cancels a running validation job.
  - Only an explicit cancel signal (`send(())`) triggers cancellation.
- Fixed worker-count edge case:
  - `ValidationConfig.jobs = Some(0)` is now normalized to 1 worker (with warning)
    instead of silently running with zero workers.
- Added regression coverage:
  - `dropped_cancel_sender_does_not_cancel_and_jobs_zero_still_processes_files`

Verification:

- `cargo fmt -p talkbank-transform`
- `cargo check -p talkbank-transform -q`
- `cargo test -p talkbank-transform -q`

## Wave Note: 2026-02-26 (pipeline/corpus Docs Tightening Pass)

Scope:

- `batchalign3/rust/crates/talkbank-transform/src/pipeline/{mod,error,io}.rs`
- `batchalign3/rust/crates/talkbank-transform/src/corpus/{mod,discovery,manifest}.rs`

Highlights:

- Tightened module/type docs to be contributor-facing and contract-oriented
  (stage ownership, discovery semantics, manifest record meanings) instead of
  terse labels.
- Kept CHAT manual linkage, but removed broad boilerplate anchors where they
  were not adding signal in these modules.
- No behavior changes in this wave (docs-only).

Verification:

- `cargo fmt -p talkbank-transform`
- `cargo check -p talkbank-transform -q`
- `cargo test -p talkbank-transform -q`

## Wave Note: 2026-02-26 (talkbank-chat Validation Runner + Cache Regression Fixes)

Scope:

- `talkbank-chat/crates/talkbank-model/src/validation/unparsed_tier.rs`
- `talkbank-chat/crates/talkbank-transform/src/validation_runner/{runner,worker,tests}.rs`
- `talkbank-chat/crates/talkbank-transform/src/unified_cache/validation_ops.rs`

Highlights:

- Fixed `%x` tier warning severity regression in model validation:
  - `W601` (`EmptyUserDefinedTier`) and `W602` (`UnknownUserDefinedTier` for deprecated `%x` forms)
    now emit `Severity::Warning` instead of `Severity::Error`.
  - Added severity assertions to existing unit tests.
- Fixed validation-runner cancellation/worker edge cases:
  - dropping cancel sender no longer cancels active runs
  - `jobs = Some(0)` now normalizes to one worker with warning
  - added regression test covering both behaviors end-to-end.
- Fixed unified-cache alignment lookup correctness:
  - `get_validation` now filters by `check_alignment` in SQL
  - added regression test proving aligned/unaligned entries for one file do not collide.

Verification:

- `cargo fmt -p talkbank-model -p talkbank-transform`
- `cargo check -p talkbank-model -p talkbank-transform -q`
- `cargo test -p talkbank-model -q`
- `cargo test -p talkbank-transform -q`

## Wave Note: 2026-02-26 (talkbank-chat Utterance Model Docs Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/file/utterance/{mod,core,accessors,parse_health}.rs`

Highlights:

- Reworked `Utterance` module/type docs for crates.io readers around the real
  contracts:
  - one required main tier
  - ordered dependent tiers
  - runtime-only metadata fields (alignment, parse-health, language state)
- Clarified accessor semantics in `accessors.rs`:
  - accessors intentionally return the first matching tier of each kind
  - `%mod` sharing the `%pho` concrete type is now called out explicitly
- Tightened parse-health documentation in `parse_health.rs`:
  - parse-recovery taint model now documented as alignment guardrail behavior
  - normalized grammatical-relations link to `#GrammaticalRelations_Tier`
  - clarified why `taint_all_alignment_dependents` exists

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat Utterance Language Metadata Docs Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/file/utterance/utterance_language.rs`
- `talkbank-chat/crates/talkbank-model/src/model/file/utterance/language_metadata_state.rs`
- `talkbank-chat/crates/talkbank-model/src/model/file/utterance/metadata/{mod,language}.rs`

Highlights:

- Refined utterance-language docs to make the baseline/word-level split explicit
  for contributors:
  - `UtteranceLanguage` = baseline resolution state
  - `LanguageMetadata` = per-word resolved language/provenance
- Clarified state-wrapper rationale in `UtteranceLanguageMetadata`:
  - explicit `Uncomputed` vs `Computed` semantics instead of ambiguous `Option`.
- Tightened pipeline docs in `metadata/language.rs`:
  - alignable-index semantics
  - baseline resolution order (`@Languages` default vs `[- code]`)
  - source/provenance mapping behavior for word markers.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat Utterance Builder/Serialization/Validation Docs Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/file/utterance/builder.rs`
- `talkbank-chat/crates/talkbank-model/src/model/file/utterance/serialization.rs`
- `talkbank-chat/crates/talkbank-model/src/model/file/utterance/validate.rs`

Highlights:

- Improved constructor/builder docs to make ordering and intent explicit:
  - `with_*` APIs are order-preserving appenders
  - runtime metadata defaults in `Utterance::new` are now documented
  - compatibility intent of tier-specific convenience methods is clearer.
- Tightened serialization contracts:
  - docs now explicitly state why preserving dependent-tier order matters for
    roundtrip stability in non-canonical CHAT corpora.
- Clarified validation-orchestration commentary:
  - participant-header contract for speaker codes
  - `%gra`-without-`%mor` handling in the presence of parse taint
  - alignment-first behavior of `validate_with_alignment`.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat ChatFile Docs Tightening Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/file/chat_file/{mod,core,accessors,write}.rs`

Highlights:

- Tightened top-level `ChatFile` docs toward contributor-facing contracts:
  - ordering preservation and interleaving invariants
  - role of parser-derived participant map
  - type-state marker intent (`NotValidated`/`Validated`)
- Reduced mechanical phrasing in high-traffic accessors and serialization docs.
- Normalized manual links in chat-file write docs to header/main/dependent anchors
  used elsewhere in the model documentation.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat File-Layer Docs Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/file/mod.rs`
- `talkbank-chat/crates/talkbank-model/src/model/file/line.rs`
- `talkbank-chat/crates/talkbank-model/src/model/file/chat_file/validate.rs`

Highlights:

- Reworked file-layer docs for crates.io readers around ordering and invariants:
  - why `Line` exists as an interleaving-preserving unit
  - how header/utterance ordering guarantees roundtrip fidelity
  - clearer file-layer relationship (`ChatFile`/`Line`/`Utterance`)
- Replaced remaining mechanical wording in line APIs and validation entrypoints
  with contract-focused descriptions.
- Tightened manual-link targeting in validation module header to `#Main_Line`
  for consistency with utterance/chat-file docs.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat Participant Model Docs Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/participant/{mod,core,accessors}.rs`

Highlights:

- Tightened participant-model docs around the real header-merge contract:
  - `@Participants` + `@ID` + optional `@Birth` composition
  - participant-map invariants expected by parser/validation
- Replaced repetitive field-level prose in `Participant` with concise,
  contributor-oriented semantics that match code behavior.
- Normalized accessor docs to short API contracts and removed filler phrasing.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat Language Metadata Docs Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/language_metadata/{mod,metadata,source,word_info}.rs`

Highlights:

- Refined language-metadata module docs around resolution precedence and
  provenance semantics (`@Languages` default, `[- code]`, word markers).
- Tightened `LanguageMetadata` field/method docs to emphasize alignable-index
  contracts and code-switch detection semantics.
- Reworked `LanguageSource`/`WordLanguageInfo` API docs to remove repetitive
  constructor phrasing and present concise per-variant/per-constructor intent.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat Core Utility Docs Accuracy Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/intern.rs`
- `talkbank-chat/crates/talkbank-model/src/model/non_empty_string.rs`
- `talkbank-chat/crates/talkbank-model/src/model/validation_tag.rs`

Highlights:

- Corrected stale architecture docs in `intern.rs`:
  - removed outdated `phf` claims
  - documented the actual runtime design (`OnceLock` + `DashMap` with lazy prepopulation).
- Tightened `NonEmptyString` API/test docs to focus on invariant semantics
  and validation behavior rather than repetitive mechanical phrasing.
- Clarified `ValidationTagged` helper semantics in terms of explicit tag mapping.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat Header Code Docs Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/header/mod.rs`
- `talkbank-chat/crates/talkbank-model/src/model/header/codes/mod.rs`
- `talkbank-chat/crates/talkbank-model/src/model/header/codes/language.rs`
- `talkbank-chat/crates/talkbank-model/src/model/header/codes/speaker.rs`
- `talkbank-chat/crates/talkbank-model/src/model/header/header_enum/impls.rs`

Highlights:

- Tightened header-layer module docs for contributor orientation:
  - clearer responsibilities of `header_enum`, `codes`, `id`, `media`, and `types_header`
  - consistent `#Main_Line` anchor usage where relevant.
- Refined `LanguageCode` and `SpeakerCode` API docs toward contract-level behavior
  (interning semantics, validation constraints, CHAT usage contexts).
- Replaced remaining mechanical utility docs in header enum impls (`name`, `to_chat`)
  with crates.io-friendly API intent phrasing.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat MOR/GRA/PHO Tier Docs Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/dependent_tier/mor/tier.rs`
- `talkbank-chat/crates/talkbank-model/src/model/dependent_tier/gra/tier.rs`
- `talkbank-chat/crates/talkbank-model/src/model/dependent_tier/pho/tier.rs`

Highlights:

- Tightened high-traffic dependent-tier API docs from repetitive verb phrases
  to concise contract wording (constructors, predicates, serialization helpers).
- Normalized `%gra` manual anchor usage to `#GrammaticalRelations_Tier`.
- Clarified serialization method docs (`to_chat`, `to_content`, `write_chat`) to
  emphasize full-line vs content-only behavior.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat Main Tier API Docs Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/content/main_tier.rs`

Highlights:

- Tightened builder-method docs to consistent contract phrasing (`with_*`
  setters/appender semantics) instead of repetitive “builder pattern” wording.
- Cleaned `%wor` helper docs:
  - removed duplicated/awkward phrasing
  - clarified conversion intent and timing preservation language.
- Normalized main-tier manual anchors to `#Main_Line` for consistency with
  other model/file docs.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Wave Note: 2026-02-26 (talkbank-chat Tier Content Docs Pass)

Scope:

- `talkbank-chat/crates/talkbank-model/src/model/content/tier_content.rs`

Highlights:

- Reworked shared-tier payload docs for clearer model contracts:
  - main-tier/%wor shared shape
  - field semantics and ordering responsibilities
  - builder method intent and naming consistency.
- Replaced repetitive “Builder: Set ...” phrasing with concise contract wording.
- Tightened bullet-omission rendering docs for `to_content_string_no_bullets`.

Verification:

- `cargo fmt -p talkbank-model`
- `cargo check -p talkbank-model -q`
- `cargo test -p talkbank-model -q`

## Current Wave Board

Use this section to track the active batch.

| Wave | Target Files | Batch Size | Planned PRs | Status |
|------|-------------:|-----------:|------------:|--------|
| 1 | 81 | 5-8 | 12 | completed |
| 2 | 60 | 5-8 | 11 | completed |
| 3 | 54 | 2-37 | 4 | completed |
| 4 | 6 | 6 | 1 | completed |
| 5 | 6 | 6 | 1 | completed |
| 6 | 6 | 6 | 1 | completed |
| 7 | 6 | 6 | 1 | completed |
| 8 | 4 | 7 | 1 | completed |
| 9 | 22 | 22 | 1 | completed |
| 10 | 20 | 20 | 1 | completed |
| 11 | 6 | 6 | 1 | completed |
| 12 | 4 | 4 | 1 | completed |
| 13 | 8 | 8 | 1 | completed |
| 14 | 6 | 6 | 1 | completed |
| 15 | 5 | 5 | 1 | completed |
| 16 | 4 | 4 | 1 | completed |
| 17 | 4 | 4 | 1 | completed |
| 18 | 3 | 3 | 1 | completed |
| 19 | 4 | 4 | 1 | completed |
| 20 | 3 | 3 | 1 | completed |
| 21 | 3 | 3 | 1 | completed |
| 22 | 4 | 4 | 1 | completed |
| 23 | 3 | 3 | 1 | completed |
| 24 | 5 | 5 | 1 | completed |
| 25 | 3 | 3 | 1 | completed |
| 26 | 1 | 1 | 1 | completed |
| 27 | 1 | 1 | 1 | completed |
| 28 | rolling | 2-54 | rolling | in progress |

Wave 1 repo allocation:

| Repo | Files |
|------|------:|
| `talkbank-chat` | 27 |
| `batchalign3` | 24 |
| `talkbank-chatter` | 12 |
| `talkbank-clan` | 8 |
| `batchalign-hk-plugin` | 5 |
| `talkbank-private` | 2 |
| `tree-sitter-talkbank` | 3 |

### Wave 1 Batch Plan (Exact Files)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W1-01` | `talkbank-chat` | 6 | codex | merged |
| `W1-02` | `talkbank-chat` | 7 | codex | merged |
| `W1-03` | `talkbank-chat` | 7 | codex | merged |
| `W1-04` | `talkbank-chat` | 7 | codex | merged |
| `W1-05` | `batchalign3` | 6 | codex | merged |
| `W1-06` | `batchalign3` | 6 | codex | merged |
| `W1-07` | `batchalign3` | 6 | codex | merged |
| `W1-08` | `batchalign3` | 6 | codex | merged |
| `W1-09` | `talkbank-chatter` | 6 | codex | merged |
| `W1-10` | `talkbank-chatter` | 6 | codex | merged |
| `W1-11` | `talkbank-clan` | 8 | codex | merged |
| `W1-12` | mixed (`batchalign-hk-plugin`, `talkbank-private`, `tree-sitter-talkbank`) | 10 | codex | merged |

#### `W1-01` (`talkbank-chat`, parser entry points)

- [x] `crates/talkbank-tree-sitter-parser/src/lib.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/parser_api.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/file.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/main_tier.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/dependent_tier.rs`

#### `W1-02` (`talkbank-chat`, chat-file parsing flow)

- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/parse.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/streaming.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/normalize.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/utterance_parser.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_dispatch/parse.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/dependent_tier_dispatch/parse.rs`

#### `W1-03` (`talkbank-chat`, model validation and alignment)

- [x] `crates/talkbank-model/src/lib.rs`
- [x] `crates/talkbank-model/src/validation/chat_file.rs`
- [x] `crates/talkbank-model/src/validation/main_tier.rs`
- [x] `crates/talkbank-model/src/validation/utterance/mod.rs`
- [x] `crates/talkbank-model/src/validation/utterance/tiers.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/metadata/alignment.rs`
- [x] `crates/talkbank-model/src/validation/header/validate.rs`

#### `W1-04` (`talkbank-chat`, errors and transform pipeline)

- [x] `crates/talkbank-errors/src/lib.rs`
- [x] `crates/talkbank-errors/src/types.rs`
- [x] `crates/talkbank-errors/src/configurable_sink.rs`
- [x] `crates/talkbank-errors/src/enhance.rs`
- [x] `crates/talkbank-transform/src/pipeline/parse.rs`
- [x] `crates/talkbank-transform/src/pipeline/convert.rs`
- [x] `crates/talkbank-transform/src/pipeline/io.rs`

#### `W1-05` (`batchalign3`, worker/runtime control plane)

- [x] `batchalign/worker.py`
- [x] `batchalign/runtime.py`
- [x] `batchalign/rust_entrypoint.py`
- [x] `batchalign/plugins.py`
- [x] `batchalign/errors.py`
- [x] `batchalign/utils/config.py`

#### `W1-06` (`batchalign3`, pipeline orchestration/cache routing)

- [x] `batchalign/pipelines/dispatch.py`
- [x] `batchalign/pipelines/pipeline.py`
- [x] `batchalign/pipelines/task.py`
- [x] `batchalign/pipelines/context.py`
- [x] `batchalign/pipelines/cache.py`
- [x] `batchalign/pipelines/cache_redis.py`

#### `W1-07` (`batchalign3`, validation and morphosyntax internals)

- [x] `batchalign/pipelines/validation.py`
- [x] `batchalign/pipelines/morphosyntax/engine.py`
- [x] `batchalign/pipelines/morphosyntax/_stanza_callback.py`
- [x] `batchalign/pipelines/morphosyntax/_stanza_batch_callback.py`
- [x] `batchalign/pipelines/morphosyntax/_tokenizer_realign.py`
- [x] `batchalign/pipelines/utterance/_utseg_callback.py`

#### `W1-08` (`batchalign3`, translation/asr/fa data paths)

- [x] `batchalign/pipelines/utterance/_utseg_batch_callback.py`
- [x] `batchalign/pipelines/translate/_translate_callback.py`
- [x] `batchalign/pipelines/translate/seamless.py`
- [x] `batchalign/pipelines/asr/whisperx.py`
- [x] `batchalign/pipelines/fa/_fa_callback.py`
- [x] `batchalign/models/audio_io.py`

#### `W1-09` (`talkbank-chatter`, LSP backend core)

- [x] `crates/talkbank-lsp/src/backend/mod.rs`
- [x] `crates/talkbank-lsp/src/backend/state.rs`
- [x] `crates/talkbank-lsp/src/backend/incremental.rs`
- [x] `crates/talkbank-lsp/src/backend/analysis.rs`
- [x] `crates/talkbank-lsp/src/backend/documents.rs`
- [x] `crates/talkbank-lsp/src/backend/validation_cache.rs`

#### `W1-10` (`talkbank-chatter`, LSP features + alignment CLI)

- [x] `crates/talkbank-lsp/src/backend/diagnostics/validation_orchestrator.rs`
- [x] `crates/talkbank-lsp/src/backend/features/hover.rs`
- [x] `crates/talkbank-lsp/src/backend/features/completion.rs`
- [x] `crates/talkbank-lsp/src/backend/features/code_action.rs`
- [x] `crates/talkbank-cli/src/commands/alignment/helpers.rs`
- [x] `crates/talkbank-cli/src/commands/alignment/show/render/mod.rs`

#### `W1-11` (`talkbank-clan`, framework and high-traffic commands)

- [x] `src/framework/runner.rs`
- [x] `src/framework/filter.rs`
- [x] `src/framework/word_filter.rs`
- [x] `src/framework/output.rs`
- [x] `src/framework/command.rs`
- [x] `src/commands/freq.rs`
- [x] `src/commands/mlu.rs`
- [x] `src/commands/chip.rs`

#### `W1-12` (plugin/private/grammar bindings)

- [x] `batchalign-hk-plugin/src/batchalign_hk_plugin/common.py`
- [x] `batchalign-hk-plugin/src/batchalign_hk_plugin/funaudio_common.py`
- [x] `batchalign-hk-plugin/src/batchalign_hk_plugin/tencent_api.py`
- [x] `batchalign-hk-plugin/src/batchalign_hk_plugin/cantonese_fa.py`
- [x] `batchalign-hk-plugin/src/batchalign_hk_plugin/tencent_asr.py`
- [x] `talkbank-private/batchalign/scripts/extract_wor_error_files.py`
- [x] `talkbank-private/batchalign/scripts/wor_rerun.py`
- [x] `tree-sitter-talkbank/bindings/rust/lib.rs`
- [x] `tree-sitter-talkbank/bindings/rust/build.rs`
- [x] `tree-sitter-talkbank/bindings/python/tree_sitter_talkbank/__init__.py`

### Wave 2 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W2-01` | `talkbank-clan` | 6 | codex | merged |
| `W2-02` | `talkbank-clan` | 6 | codex | merged |
| `W2-03` | `talkbank-clan` | 8 | codex | merged |
| `W2-04` | `talkbank-clan` | 4 | codex | merged |
| `W2-05` | `talkbank-chat` | 6 | codex | merged |
| `W2-06` | `talkbank-chat` | 3 | codex | merged |
| `W2-07` | `talkbank-chat` | 6 | codex | merged |
| `W2-08` | `talkbank-chat` | 4 | codex | merged |
| `W2-09` | `talkbank-chat` | 6 | codex | merged |
| `W2-10` | `talkbank-chat` | 3 | codex | merged |
| `W2-11` | `batchalign3` | 8 | codex | merged |

#### `W2-01` (`talkbank-clan`, additional core analysis commands)

- [x] `src/commands/cooccur.rs`
- [x] `src/commands/combo.rs`
- [x] `src/commands/freqpos.rs`
- [x] `src/commands/maxwd.rs`
- [x] `src/commands/modrep.rs`
- [x] `src/commands/timedur.rs`

#### `W2-02` (`talkbank-clan`, remaining analytics + shared key type docs)

- [x] `src/commands/vocd.rs`
- [x] `src/commands/mlt.rs`
- [x] `src/commands/wdlen.rs`
- [x] `src/commands/gemlist.rs`
- [x] `src/commands/phonfreq.rs`
- [x] `src/framework/normalized_word.rs`

#### `W2-03` (`talkbank-clan`, CLI + transform entrypoints and remaining analysis commands)

- [x] `src/commands/dist.rs`
- [x] `src/commands/kwal.rs`
- [x] `src/cli.rs`
- [x] `src/main.rs`
- [x] `src/transforms/repeat.rs`
- [x] `src/transforms/chstring.rs`
- [x] `src/transforms/lowcase.rs`
- [x] `src/transforms/fixbullets.rs`

#### `W2-04` (`talkbank-clan`, remaining transform command docs)

- [x] `src/transforms/dates.rs`
- [x] `src/transforms/flo.rs`
- [x] `src/transforms/retrace.rs`
- [x] `src/transforms/delim.rs`

#### `W2-05` (`talkbank-chat`, bin utilities docs pass I)

- [x] `src/bin/build-corpus-manifest.rs`
- [x] `src/bin/roundtrip-analyze.rs`
- [x] `src/bin/query-manifest.rs`
- [x] `src/bin/corpus-manifest-info.rs`
- [x] `src/bin/incremental_lsp_check.rs`
- [x] `src/bin/parse-tree.rs`

#### `W2-06` (`talkbank-chat`, bin utilities docs pass II)

- [x] `src/bin/test-dashboard.rs`
- [x] `src/bin/clear-cache-prefix.rs`
- [x] `src/bin/debug-validation.rs`

#### `W2-07` (`talkbank-chat`, derive + model header/validation docs pass)

- [x] `crates/talkbank-derive/src/helpers.rs`
- [x] `crates/talkbank-model/src/validation/utterance/ca_delimiter.rs`
- [x] `crates/talkbank-model/src/model/header/types_header.rs`
- [x] `crates/talkbank-model/src/model/header/codes/participant.rs`
- [x] `crates/talkbank-model/src/model/header/codes/language.rs`
- [x] `crates/talkbank-model/src/model/header/codes/speaker.rs`

#### `W2-08` (`talkbank-chat`, model doctest + docs hygiene fixes)

- [x] `crates/talkbank-model/src/model/annotation/replacement.rs`
- [x] `crates/talkbank-model/src/model/annotation/scoped/types.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/pho/tier.rs`
- [x] `crates/talkbank-model/src/model/write_chat.rs`

#### `W2-09` (`talkbank-chat`, core model anchor-alignment docs pass)

- [x] `crates/talkbank-model/src/model/content/word/content.rs`
- [x] `crates/talkbank-model/src/model/content/word/types.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/types.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/word/word.rs`
- [x] `crates/talkbank-model/src/model/content/utterance_content/types.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/pho/word.rs`

#### `W2-10` (`talkbank-chat`, model anchor canonicalization follow-up)

- [x] `crates/talkbank-model/src/model/non_empty_string.rs`
- [x] `crates/talkbank-model/src/model/file/line.rs`
- [x] `crates/talkbank-model/src/model/annotation/scoped/mod.rs`

#### `W2-11` (`batchalign3`, Rust anchor canonicalization sweep)

- [x] `rust/grammar/grammar.js`
- [x] `rust/crates/talkbank-model/src/model/annotation/replacement.rs`
- [x] `rust/crates/talkbank-model/src/model/annotation/scoped/mod.rs`
- [x] `rust/crates/talkbank-model/src/model/annotation/scoped/types.rs`
- [x] `rust/crates/talkbank-model/src/model/dependent_tier/types.rs`
- [x] `rust/crates/talkbank-model/src/model/file/line.rs`
- [x] `book/src/developer/rust-workspace-map.md`
- [x] `book/src/developer/rust-manual-crosswalk.md`

### Wave 3 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W3-01` | `talkbank-chat` | 10 | codex | merged |
| `W3-02` | `tree-sitter-talkbank` | 2 | codex | merged |
| `W3-03` | `batchalign3` | 5 | codex | merged |
| `W3-04` | `batchalign3` | 37 | codex | merged |

#### `W3-01` (`talkbank-chat`, additional model word/CA + example docs)

- [x] `crates/talkbank-model/src/model/content/word/ca/delimiters.rs`
- [x] `crates/talkbank-model/src/model/content/word/ca/elements.rs`
- [x] `crates/talkbank-model/src/model/content/word/category.rs`
- [x] `crates/talkbank-model/src/model/content/word/language.rs`
- [x] `crates/talkbank-model/src/model/time.rs`
- [x] `examples/analyze_boxing_options.rs`
- [x] `examples/debug_alignment.rs`
- [x] `examples/debug_happy_path.rs`
- [x] `examples/test_action_annot.rs`
- [x] `examples/test_stability.rs`

#### `W3-02` (`tree-sitter-talkbank`, binding test/package comment docs)

- [x] `bindings/python/tests/test_binding.py`
- [x] `setup.py`

#### `W3-03` (`batchalign3`, rust-next worker/CLI docs + test intent)

- [x] `rust-next/crates/batchalign-cli/src/gui_cmd.rs`
- [x] `rust-next/crates/batchalign-cli/tests/common/mod.rs`
- [x] `rust-next/crates/batchalign-server/tests/integration.rs`
- [x] `rust-next/crates/batchalign-worker/src/pool.rs`
- [x] `rust-next/crates/batchalign-worker/tests/integration.rs`

#### `W3-04` (`batchalign3`, talkbank-model alignment/validation docs + readability pass)

- [x] `rust/crates/talkbank-model/src/alignment/format.rs`
- [x] `rust/crates/talkbank-model/src/alignment/helpers/count.rs`
- [x] `rust/crates/talkbank-model/src/alignment/helpers/mod.rs`
- [x] `rust/crates/talkbank-model/src/alignment/helpers/tests.rs`
- [x] `rust/crates/talkbank-model/src/alignment/mod.rs`
- [x] `rust/crates/talkbank-model/src/alignment/mor.rs`
- [x] `rust/crates/talkbank-model/src/alignment/pho.rs`
- [x] `rust/crates/talkbank-model/src/alignment/sin.rs`
- [x] `rust/crates/talkbank-model/src/alignment/wor.rs`
- [x] `rust/crates/talkbank-model/src/lib.rs`
- [x] `rust/crates/talkbank-model/src/model/annotation/annotated.rs`
- [x] `rust/crates/talkbank-model/src/model/annotation/mod.rs`
- [x] `rust/crates/talkbank-model/src/model/borrowed/word.rs`
- [x] `rust/crates/talkbank-model/src/model/content/mod.rs`
- [x] `rust/crates/talkbank-model/src/model/dependent_tier/gra/mod.rs`
- [x] `rust/crates/talkbank-model/src/model/dependent_tier/gra/tier.rs`
- [x] `rust/crates/talkbank-model/src/model/file/utterance/accessors.rs`
- [x] `rust/crates/talkbank-model/src/model/file/utterance/metadata/tests.rs`
- [x] `rust/crates/talkbank-model/src/model/header/media.rs`
- [x] `rust/crates/talkbank-model/src/model/header/write_chat.rs`
- [x] `rust/crates/talkbank-model/src/model/mod.rs`
- [x] `rust/crates/talkbank-model/src/model/non_empty_string.rs`
- [x] `rust/crates/talkbank-model/src/model/semantic_diff/context.rs`
- [x] `rust/crates/talkbank-model/src/model/semantic_diff/mod.rs`
- [x] `rust/crates/talkbank-model/src/model/semantic_diff/report.rs`
- [x] `rust/crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs`
- [x] `rust/crates/talkbank-model/src/validation/cross_utterance/tests/quotation_follows.rs`
- [x] `rust/crates/talkbank-model/src/validation/cross_utterance/tests/quotation_precedes.rs`
- [x] `rust/crates/talkbank-model/src/validation/cross_utterance/tests/self_completion.rs`
- [x] `rust/crates/talkbank-model/src/validation/header/mod.rs`
- [x] `rust/crates/talkbank-model/src/validation/header/structure.rs`
- [x] `rust/crates/talkbank-model/src/validation/mod.rs`
- [x] `rust/crates/talkbank-model/src/validation/utterance/mod.rs`
- [x] `rust/crates/talkbank-model/src/validation/utterance/tests.rs`
- [x] `rust/crates/talkbank-model/src/validation/word/language/digits.rs`
- [x] `rust/crates/talkbank-model/src/validation/word/language/mod.rs`
- [x] `rust/crates/talkbank-model/src/validation/word/language/tests.rs`

### Wave 4 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W4-01` | `talkbank-chat` | 6 | codex | merged |

#### `W4-01` (`talkbank-chat`, tier containers and dependent-tier wrapper docs)

- [x] `crates/talkbank-model/src/model/content/tier_content.rs`
- [x] `crates/talkbank-model/src/model/content/bracketed.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/sin/item.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/sin/tier.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/tier.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/core.rs`

### Wave 5 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W5-01` | `talkbank-chat` | 6 | codex | merged |

#### `W5-01` (`talkbank-chat`, core content marker and user-defined tier docs)

- [x] `crates/talkbank-model/src/model/content/action.rs`
- [x] `crates/talkbank-model/src/model/content/event.rs`
- [x] `crates/talkbank-model/src/model/content/other_spoken.rs`
- [x] `crates/talkbank-model/src/model/content/postcode.rs`
- [x] `crates/talkbank-model/src/model/content/terminator.rs`
- [x] `crates/talkbank-model/src/model/user_defined_tier.rs`

### Wave 6 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W6-01` | `talkbank-chat` | 6 | codex | merged |

#### `W6-01` (`talkbank-chat`, header model wrappers and serialization docs)

- [x] `crates/talkbank-model/src/model/header/header_enum/header.rs`
- [x] `crates/talkbank-model/src/model/header/header_enum/impls.rs`
- [x] `crates/talkbank-model/src/model/header/write_chat.rs`
- [x] `crates/talkbank-model/src/model/header/media.rs`
- [x] `crates/talkbank-model/src/model/header/enums.rs`
- [x] `crates/talkbank-model/src/model/header/codes/date.rs`

### Wave 7 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W7-01` | `talkbank-chat` | 6 | codex | merged |

#### `W7-01` (`talkbank-chat`, dependent-tier item/wrapper docs pass)

- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/content.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/segment.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/pho/item.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/tier.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/item.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/wor.rs`

Wave note:
Anchor-consistency follow-up touched previously-audited files without changing unique-file counts:
`crates/talkbank-model/src/model/content/tier_content.rs`,
`crates/talkbank-model/src/model/dependent_tier/mor/tier.rs`,
`crates/talkbank-model/src/model/dependent_tier/sin/tier.rs`,
`crates/talkbank-model/src/model/file/chat_file/core.rs`,
`crates/talkbank-model/src/model/header/header_enum/header.rs`.

### Wave 8 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W8-01` | `talkbank-chat` | 4 | codex | merged |

#### `W8-01` (`talkbank-chat`, utterance language-state + phonology anchor docs)

- [x] `crates/talkbank-model/src/model/dependent_tier/pho/tier.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/language_metadata_state.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/parse_health.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/utterance_language.rs`

Wave note:
Anchor-consistency follow-up touched previously-audited files without changing unique-file counts:
`crates/talkbank-model/src/model/annotation/replacement.rs`,
`crates/talkbank-model/src/model/content/word/content.rs`,
`crates/talkbank-model/src/model/content/word/types.rs`.

### Wave 9 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W9-01` | `talkbank-chat` | 22 | codex | merged |

#### `W9-01` (`talkbank-chat`, full data-model anchor completion + filler-cleanup pass)

- [x] `crates/talkbank-model/src/model/alignment_set.rs`
- [x] `crates/talkbank-model/src/model/annotation/bracketed.rs`
- [x] `crates/talkbank-model/src/model/annotation/scoped/types.rs`
- [x] `crates/talkbank-model/src/model/content/bracketed.rs`
- [x] `crates/talkbank-model/src/model/content/pause.rs`
- [x] `crates/talkbank-model/src/model/content/word/content.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/segment.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/analysis/newtypes.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/sin/item.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/types.rs`
- [x] `crates/talkbank-model/src/model/header/codes/header_strings.rs`
- [x] `crates/talkbank-model/src/model/intern.rs`
- [x] `crates/talkbank-model/src/model/language_metadata/metadata.rs`
- [x] `crates/talkbank-model/src/model/language_metadata/word_info.rs`
- [x] `crates/talkbank-model/src/model/non_empty_string.rs`
- [x] `crates/talkbank-model/src/model/provenance.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/context.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/mod.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/report.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/tree_renderer.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/types.rs`
- [x] `crates/talkbank-model/src/model/validation_tag.rs`

Wave note:
- Strict data-model anchor audit (`pub struct|enum` + 40-line doc window) reached `missing_anchor_types=0` for `crates/talkbank-model/src/model`.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=208`)

### Wave 10 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W10-01` | `talkbank-chat` | 20 | codex | merged |

#### `W10-01` (`talkbank-chat`, validation + alignment placeholder-doc cleanup)

- [x] `crates/talkbank-model/src/validation/context.rs`
- [x] `crates/talkbank-model/src/validation/header/checkers.rs`
- [x] `crates/talkbank-model/src/validation/header/metadata.rs`
- [x] `crates/talkbank-model/src/validation/header/participant.rs`
- [x] `crates/talkbank-model/src/validation/header/structure.rs`
- [x] `crates/talkbank-model/src/validation/header/unknown.rs`
- [x] `crates/talkbank-model/src/validation/temporal.rs`
- [x] `crates/talkbank-model/src/validation/word/language/digits.rs`
- [x] `crates/talkbank-model/src/validation/word/language/helpers.rs`
- [x] `crates/talkbank-model/src/validation/word/language/resolve.rs`
- [x] `crates/talkbank-model/src/validation/word/structure.rs`
- [x] `crates/talkbank-model/src/alignment/gra/align.rs`
- [x] `crates/talkbank-model/src/alignment/gra/types.rs`
- [x] `crates/talkbank-model/src/alignment/helpers/count.rs`
- [x] `crates/talkbank-model/src/alignment/helpers/domain.rs`
- [x] `crates/talkbank-model/src/alignment/helpers/rules.rs`
- [x] `crates/talkbank-model/src/alignment/mor.rs`
- [x] `crates/talkbank-model/src/alignment/pho.rs`
- [x] `crates/talkbank-model/src/alignment/sin.rs`
- [x] `crates/talkbank-model/src/alignment/wor.rs`

Wave note:
- Replaced residual low-value comments (`Runs ...`, generic module stubs) in validation/alignment code paths with intent-level docs and explicit CHAT manual anchors.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=213`)

### Wave 11 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W11-01` | `talkbank-chat` | 6 | codex | merged |

#### `W11-01` (`talkbank-chat`, participant/language/options/semantic-diff docs polish)

- [x] `crates/talkbank-model/src/model/file/utterance/metadata/language.rs`
- [x] `crates/talkbank-model/src/model/header/header_enum/options.rs`
- [x] `crates/talkbank-model/src/model/language_metadata/source.rs`
- [x] `crates/talkbank-model/src/model/participant/accessors.rs`
- [x] `crates/talkbank-model/src/model/participant/core.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/source_utils.rs`

Wave note:
- Replaced module-level placeholder docs and low-value helper comments with purpose/invariant wording and anchored CHAT references.
- Normalized remaining legacy manual links in participant docs to `https://talkbank.org/0info/manuals/CHAT.html#...`.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=214`)

### Wave 12 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W12-01` | `talkbank-chat` | 4 | codex | merged |

#### `W12-01` (`talkbank-chat`, annotated/chat-file validation and async/alignment docs cleanup)

- [x] `crates/talkbank-model/src/model/annotation/annotated.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/validate.rs`
- [x] `crates/talkbank-model/src/validation/async_helpers.rs`
- [x] `crates/talkbank-model/src/alignment/format.rs`

Wave note:
- Removed remaining placeholder comments (`Runs ...`, `Enum variants for ...`, `Type representing ...`) in these files and replaced them with intent/invariant wording.
- Added/kept anchored CHAT references where behavior maps to manual concepts.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=214`)

### Wave 13 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W13-01` | `talkbank-chat` | 8 | codex | merged |

#### `W13-01` (`talkbank-chat`, core content/tier type and semantic-eq placeholder cleanup)

- [x] `crates/talkbank-model/src/model/content/overlap.rs`
- [x] `crates/talkbank-model/src/model/content/separator.rs`
- [x] `crates/talkbank-model/src/model/content/word/form.rs`
- [x] `crates/talkbank-model/src/model/content/word/untranscribed.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/tier_type.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/pho/tier_type.rs`
- [x] `crates/talkbank-model/src/model/header/id.rs`
- [x] `crates/talkbank-model/src/model/semantic_eq.rs`

Wave note:
- Replaced remaining placeholder docs in these files and updated overlap anchors to stable named anchors (`CA_Overlaps`, delimiter anchors) instead of `_Toc...` links.
- Removed repetitive low-value per-impl `/// Runs semantic eq.` comments in `semantic_eq.rs` while preserving module-level rationale.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=219`)
- Residual placeholder-hit scan in `talkbank-model/src` now reports `109` (down from `148` before this wave).

### Wave 14 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W14-01` | `talkbank-chat` | 6 | codex | merged |

#### `W14-01` (`talkbank-chat`, chat-file/utterance model module-doc cleanup)

- [x] `crates/talkbank-model/src/model/file/chat_file/accessors.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/write.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/accessors.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/core.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/serialization.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/validate.rs`

Wave note:
- Replaced module-level placeholder docs in core file/utterance modules with contributor-facing intent docs and CHAT anchor references.
- Fixed stale manual anchor usage in `utterance/core.rs` (`Morphology` -> `Morphological_Tier`, `%gra` reference -> `GrammaticalRelations_Tier`).
- Removed duplicated `%mor/%gra` bullet lines in `utterance/core.rs` docs.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=219`)
- Residual placeholder-hit scan in `talkbank-model/src` now reports `100`.

### Wave 15 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W15-01` | `talkbank-chat` | 9 | codex | merged |

#### `W15-01` (`talkbank-chat`, nonvocal/annotation/dependent-tier writer docs cleanup)

- [x] `crates/talkbank-model/src/model/annotation/scoped/write.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/begin.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/end.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/label.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/simple.rs`
- [x] `crates/talkbank-model/src/model/content/utterance_content/write.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/write.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/kind.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/builder.rs`

Wave note:
- Replaced placeholder module stubs and `Runs ...` doc comments with concrete behavior docs, including explicit CHAT anchors for scoped symbols, dependent tiers, bullets, and nonverbal/nonvocal event notation.
- Clarified comparison/formatting semantics for nonvocal begin/end/simple markers (`PartialEq` ignores source span; display writes canonical CHAT text).
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=219`)
- Residual placeholder-hit scan in `talkbank-model/src` now reports `83`.

### Wave 16 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W16-01` | `talkbank-chat` | 19 | codex | merged |

#### `W16-01` (`talkbank-chat`, broad placeholder-doc elimination across model/tests/macros)

- [x] `crates/talkbank-model/src/alignment/helpers/tests.rs`
- [x] `crates/talkbank-model/src/model/content/bullet.rs`
- [x] `crates/talkbank-model/src/model/content/freecode.rs`
- [x] `crates/talkbank-model/src/model/content/linker.rs`
- [x] `crates/talkbank-model/src/model/content/long_feature.rs`
- [x] `crates/talkbank-model/src/model/content/main_tier.rs`
- [x] `crates/talkbank-model/src/model/content/pause.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/relation.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/metadata/tests.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/tests.rs`
- [x] `crates/talkbank-model/src/model/language_metadata/tests.rs`
- [x] `crates/talkbank-model/src/model/macros.rs`
- [x] `crates/talkbank-model/src/model/mod.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/impls/container.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/impls/scalar.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/report.rs`
- [x] `crates/talkbank-model/src/validation/word/language/tests.rs`
- [x] `crates/talkbank-model/src/validation/word/snapshot_tests.rs`
- [x] `crates/talkbank-model/src/validation/word/tests.rs`

Wave note:
- Replaced the remaining tracked placeholder patterns (`Runs ...`, `Type representing ...`, generic subsystem stubs) with intent-level documentation or removed redundant trait-impl doc stubs.
- Improved shared macro-generated docs (`string_newtype!`, `interned_newtype!`) so generated API docs read as deliberate contributor-facing documentation.
- Kept/normalized CHAT manual anchor links in model docs touched this wave (`missing=0`).
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=219`)
- Residual placeholder-hit scan in `talkbank-model/src` now reports `0` for tracked patterns.

### Wave 17 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W17-01` | `talkbank-chat` | 20 | codex | merged |

#### `W17-01` (`talkbank-chat`, module-level CHAT anchor coverage for model/header trees)

- [x] `crates/talkbank-model/src/model/annotation/mod.rs`
- [x] `crates/talkbank-model/src/model/content/mod.rs`
- [x] `crates/talkbank-model/src/model/content/word/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/analysis/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/word/mod.rs`
- [x] `crates/talkbank-model/src/model/file/mod.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/metadata/alignment.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/metadata/mod.rs`
- [x] `crates/talkbank-model/src/model/header/codes/date.rs`
- [x] `crates/talkbank-model/src/model/header/codes/mod.rs`
- [x] `crates/talkbank-model/src/model/header/codes/participant.rs`
- [x] `crates/talkbank-model/src/model/header/header_enum/impls.rs`
- [x] `crates/talkbank-model/src/model/header/header_enum/mod.rs`
- [x] `crates/talkbank-model/src/model/header/media.rs`
- [x] `crates/talkbank-model/src/model/header/mod.rs`
- [x] `crates/talkbank-model/src/model/header/write_chat.rs`
- [x] `crates/talkbank-model/src/model/language_metadata/mod.rs`
- [x] `crates/talkbank-model/src/model/mod.rs`
- [x] `crates/talkbank-model/src/model/participant/mod.rs`

Wave note:
- Added explicit CHAT manual anchor references to module-level docs across `model/`, `header/`, and `%mor` subtrees to improve newcomer discoverability.
- Replaced stale manual URLs (`talkbank.org/manuals/...`) and legacy `_Toc...` links in touched files with stable `https://talkbank.org/0info/manuals/CHAT.html#...` anchors.
- Fixed residual module-doc artifacts (`\\n` literals) in touched module files.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)
- Anchor-coverage audit in `talkbank-model/src/model` now leaves only 5 files without CHAT anchors, all infrastructure-focused:
  - `model/macros.rs`
  - `model/semantic_diff/impls/container.rs`
  - `model/semantic_diff/impls/mod.rs`
  - `model/semantic_diff/impls/scalar.rs`
  - `model/write_chat.rs`

### Wave 18 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W18-01` | `talkbank-chat` | 5 | codex | merged |

#### `W18-01` (`talkbank-chat`, close remaining model-level anchor coverage gaps)

- [x] `crates/talkbank-model/src/model/macros.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/impls/container.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/impls/mod.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/impls/scalar.rs`
- [x] `crates/talkbank-model/src/model/write_chat.rs`

Wave note:
- Added module-level CHAT manual anchor references to the remaining infrastructure files previously excluded by the domain-model pass.
- Result: non-test `crates/talkbank-model/src/model/**/*.rs` files now have complete CHAT-anchor coverage in module docs.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)
- Anchor-coverage audit now reports **no remaining non-test model files without CHAT anchors**.

### Wave 19 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W19-01` | `talkbank-chat` | 9 | codex | merged |

#### `W19-01` (`talkbank-chat`, remove remaining generic subsystem doc stubs in active files)

- [x] `crates/talkbank-model/src/alignment/gra/tests.rs`
- [x] `crates/talkbank-model/src/model/content/utterance_content/tests.rs`
- [x] `crates/talkbank-model/src/model/content/word/ca/tests.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/tests.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/tests.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/sin/tests.rs`
- [x] `crates/talkbank-model/src/model/participant/tests.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests.rs`
- [x] `crates/talkbank-model/src/validation/word/language/mod.rs`

Wave note:
- Replaced low-signal module comments (`Tests for this subsystem`, `Module declarations ...`) with concrete purpose docs.
- Rewrote test doc comments in touched files to describe behavior/invariants being tested rather than generic “Tests X” phrasing.
- Added CHAT anchor references in `validation/word/language/mod.rs` where the validation behavior maps directly to `@Languages` and `@s`.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)
- Generic-stub scan (`for this subsystem`) now only matches generated code:
  - `crates/talkbank-model/src/generated/symbol_sets.rs`

### Wave 20 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W20-01` | `talkbank-chat` | 2 | codex | merged |

#### `W20-01` (`talkbank-chat`, high-volume test-comment rewrite in validation hotspots)

- [x] `crates/talkbank-model/src/validation/word/tests.rs`
- [x] `crates/talkbank-model/src/validation/utterance/tests.rs`

Wave note:
- Rewrote dense clusters of mechanical `/// Tests ...` comments in the two highest-hit validation test files into behavior-oriented `/// Verifies ...` descriptions.
- Corrected misleading wording in utterance quotation/underline tests where comments previously implied balanced behavior despite intentionally malformed fixtures.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)

### Wave 21 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W21-01` | `talkbank-chat` | 3 | codex | merged |

#### `W21-01` (`talkbank-chat`, next high-density test-comment cleanup cluster)

- [x] `crates/talkbank-model/src/model/dependent_tier/mor/analysis/newtypes.rs`
- [x] `crates/talkbank-model/src/validation/header/structure.rs`
- [x] `crates/talkbank-model/src/validation/word/language/tests.rs`

Wave note:
- Replaced remaining `/// Tests ...` doc comments in these files with behavior-oriented `/// Verifies ...` wording.
- Normalized error-code phrasing in header-structure tests to explicit `` `E###` `` references.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)

### Wave 22 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W22-01` | `talkbank-chat` | 2 | codex | merged |

#### `W22-01` (`talkbank-chat`, test-comment cleanup in semantic-diff and primitive model utilities)

- [x] `crates/talkbank-model/src/model/non_empty_string.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/tree_renderer.rs`

Wave note:
- Replaced residual `/// Tests ...` phrasing with behavior-oriented `/// Verifies ...` comments in both files’ inline test modules.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)

### Wave 23 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W23-01` | `talkbank-chat` | 3 | codex | merged |

#### `W23-01` (`talkbank-chat`, comment cleanup in scoped-marker and dependent-tier modules)

- [x] `crates/talkbank-model/src/model/dependent_tier/gra/tier.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/text/mod.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs`

Wave note:
- Replaced residual `/// Tests ...` comments with behavior-oriented `/// Verifies ...` phrasing in these files.
- Normalized scoped-marker test comments to explicit `` `E###` `` references for error-code expectations.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)

### Wave 24 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W24-01` | `talkbank-chat` | 26 | codex | merged |

#### `W24-01` (`talkbank-chat`, full talkbank-model residual test-comment sweep)

- [x] `crates/talkbank-model/src/alignment/format.rs`
- [x] `crates/talkbank-model/src/alignment/location_tests.rs`
- [x] `crates/talkbank-model/src/alignment/sin.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/act.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/cod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/relation.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/relation_type.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/tier.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/pho/tests.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/metadata/tests.rs`
- [x] `crates/talkbank-model/src/model/header/codes/participant.rs`
- [x] `crates/talkbank-model/src/model/header/codes/speaker.rs`
- [x] `crates/talkbank-model/src/model/intern.rs`
- [x] `crates/talkbank-model/src/model/mod.rs`
- [x] `crates/talkbank-model/src/model/semantic_eq.rs`
- [x] `crates/talkbank-model/src/validation/async_helpers.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/edge_cases.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/other_completion.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/quotation_follows.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/quotation_precedes.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/self_completion.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/terminator_linker_pairing.rs`
- [x] `crates/talkbank-model/src/validation/header/mod.rs`
- [x] `crates/talkbank-model/src/validation/mod.rs`
- [x] `crates/talkbank-model/src/validation/temporal.rs`
- [x] `crates/talkbank-model/src/validation/unparsed_tier.rs`

Wave note:
- Ran a full non-generated `talkbank-model` sweep to replace residual `/// Tests ...`/`/// Test ...` phrasing with `/// Verifies ...` behavior comments.
- Normalized warning-code comments in `validation/unparsed_tier.rs` to correct `W601/W602` labels.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)
- Completion gate (non-generated `talkbank-model/src`) now reports:
  - placeholder/mechanical scan (`Tests ...`, subsystem stubs, `Runs ...`, `Type representing ...`): `0`
  - model-level CHAT anchor coverage gaps (`src/model` non-test files): `0`

### Wave 25 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W25-01` | `talkbank-chat` | 40 | codex | merged |

#### `W25-01` (`talkbank-chat`, crate-wide CHAT anchor coverage completion outside `model/`)

- [x] `crates/talkbank-model/src/alignment/gra/mod.rs`
- [x] `crates/talkbank-model/src/alignment/helpers/mod.rs`
- [x] `crates/talkbank-model/src/alignment/mod.rs`
- [x] `crates/talkbank-model/src/alignment/types.rs`
- [x] `crates/talkbank-model/src/lib.rs`
- [x] `crates/talkbank-model/src/validation/bullet.rs`
- [x] `crates/talkbank-model/src/validation/chat_file.rs`
- [x] `crates/talkbank-model/src/validation/config.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/completion.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/helpers.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/mod.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/quotation_follows.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/quotation_precedes.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/quoted_linker.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs`
- [x] `crates/talkbank-model/src/validation/header/mod.rs`
- [x] `crates/talkbank-model/src/validation/header/validate.rs`
- [x] `crates/talkbank-model/src/validation/main_tier.rs`
- [x] `crates/talkbank-model/src/validation/mod.rs`
- [x] `crates/talkbank-model/src/validation/retrace/collection/bracketed.rs`
- [x] `crates/talkbank-model/src/validation/retrace/collection/mod.rs`
- [x] `crates/talkbank-model/src/validation/retrace/collection/utterance.rs`
- [x] `crates/talkbank-model/src/validation/retrace/detection.rs`
- [x] `crates/talkbank-model/src/validation/retrace/mod.rs`
- [x] `crates/talkbank-model/src/validation/retrace/rendering/bracketed.rs`
- [x] `crates/talkbank-model/src/validation/retrace/rendering/mod.rs`
- [x] `crates/talkbank-model/src/validation/retrace/rendering/utterance.rs`
- [x] `crates/talkbank-model/src/validation/retrace/types.rs`
- [x] `crates/talkbank-model/src/validation/speaker.rs`
- [x] `crates/talkbank-model/src/validation/state.rs`
- [x] `crates/talkbank-model/src/validation/trait.rs`
- [x] `crates/talkbank-model/src/validation/unparsed_tier.rs`
- [x] `crates/talkbank-model/src/validation/utterance/ca_delimiter.rs`
- [x] `crates/talkbank-model/src/validation/utterance/comma.rs`
- [x] `crates/talkbank-model/src/validation/utterance/mod.rs`
- [x] `crates/talkbank-model/src/validation/utterance/overlap.rs`
- [x] `crates/talkbank-model/src/validation/utterance/quotation.rs`
- [x] `crates/talkbank-model/src/validation/utterance/tiers.rs`
- [x] `crates/talkbank-model/src/validation/utterance/underline.rs`
- [x] `crates/talkbank-model/src/validation/word/mod.rs`

Wave note:
- Added module-level CHAT manual anchor references to all remaining non-test, non-generated `talkbank-model` files that previously lacked explicit direct anchors.
- Fixed stale doc artifacts in touched files (`\\n` literal remnants in module docs).
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)
- Coverage gate now reports:
  - non-test/non-generated `talkbank-model` files audited: `198`
  - files missing direct CHAT anchor links: `0`
  - mechanical placeholder scan in non-generated tree: `0`

### Wave 26 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W26-01` | `talkbank-chat` | 10 | codex | merged |

#### `W26-01` (`talkbank-chat`, type-level CHAT anchor completion for remaining model types)

- [x] `crates/talkbank-model/src/model/time.rs` (`MediaTiming`)
- [x] `crates/talkbank-model/src/model/annotation/replacement.rs` (`ReplacementWords`)
- [x] `crates/talkbank-model/src/model/content/overlap.rs` (`OverlapIndex`, `OverlapPoint`)
- [x] `crates/talkbank-model/src/model/content/word/types.rs` (`WordContents`)
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/content.rs` (`BulletContentSegments`)
- [x] `crates/talkbank-model/src/model/dependent_tier/sin/item.rs` (`SinToken`)
- [x] `crates/talkbank-model/src/model/dependent_tier/types.rs` (`TextTier`)
- [x] `crates/talkbank-model/src/model/file/utterance/language_metadata_state.rs` (`UtteranceLanguageMetadata`)
- [x] `crates/talkbank-model/src/model/file/utterance/utterance_language.rs` (`UtteranceLanguage`)

Wave note:
- Added explicit, type-level CHAT manual anchors for the last public model types that still relied only on module-level references.
- Completion gate for `crates/talkbank-model/src/model` now reports:
  - public `struct`/`enum` types missing type-level docs: `0`
  - public `struct`/`enum` types missing a direct CHAT anchor in their own docs: `0`
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo test -p talkbank-model --doc -q`
  - `RUSTFLAGS='-Wmissing-docs' cargo check -p talkbank-model -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)

### Wave 27 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W27-01` | `talkbank-chat` | 48 | codex | merged |
| `W27-02` | `talkbank-chat` | 22 | codex | merged |
| `W27-03` | `talkbank-chat` | 36 | codex | merged |
| `W27-04` | `talkbank-chat` | 28 | codex | merged |
| `W27-05` | `talkbank-chat` | 37 | codex | merged |

#### `W27-01` (`talkbank-chat`, talkbank-tree-sitter-parser documentation and anchor cleanup)

- [x] `crates/talkbank-tree-sitter-parser/src/lib.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/node_types.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/parser_api.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/file.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/main_tier.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/dependent_tier.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/header.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/parser_struct.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/parse.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/streaming.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/normalize.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_dispatch/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_dispatch/parse.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_dispatch/finder.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/pre_begin.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/dispatch/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/dispatch/core.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/dispatch/simple.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/dispatch/structured.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/dispatch/special.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/dispatch/gem.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/dependent_tier_dispatch/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/dependent_tier_dispatch/parse.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/dependent_tier_dispatch/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/dependent_tier_dispatch/parsed.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/dependent_tier_dispatch/raw.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/dependent_tier_dispatch/unparsed.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/dependent_tier_dispatch/user_defined.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/single_item/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/single_item/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/single_item/parse_word.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/single_item/parse_main_tier.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/single_item/parse_utterance.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/single_item/parse_tiers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/contents.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/convert/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/convert/prefix.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/convert/body.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/convert/linkers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/convert/end.rs`
- [x] `scripts/generate-node-types.js`

Wave note:
- Added module-level CHAT manual anchor references across parser API and file-level parser internals in `talkbank-tree-sitter-parser`.
- Replaced low-value doc stubs (`Runs ...`, `Parses ...`, `Type representing ...`) in touched files with behavior-oriented comments and explicit parser responsibilities.
- Fixed a pre-existing failing doctest in generated `node_types` docs by updating `scripts/generate-node-types.js` and regenerating `src/node_types.rs`.
- Validation checks passed:
  - `cargo fmt -p talkbank-tree-sitter-parser`
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)
- Coverage gate (non-test/non-generated `talkbank-tree-sitter-parser/src`) now reports:
  - files missing direct CHAT anchor links: `123` (down from `169`)
  - remaining low-value stub hits (`Runs/Parses/Type representing` scan): `31`

#### `W27-02` (`talkbank-chat`, tree-sitter parser header/content stub cleanup and anchor expansion)

- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/pho/cst.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/text/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/separator.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/metadata/languages.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/id/parse.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/metadata/media.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/id/fields.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/metadata/types.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/metadata/situation.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/participants.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/metadata/t_header.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/metadata/pid.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/pho_group.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/sin_group.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/group/parser.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/replacement/parse.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/replacement/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/base/long_feature.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/group/nested.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/base/nonvocal.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/base/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/base/other_spoken.rs`

Wave note:
- Replaced remaining non-test low-value doc stubs in these parser hotspots with behavior-oriented comments.
- Added additional module-level CHAT manual anchors in header and main-tier content parsing modules.
- Fixed doctest regressions in grammar-doc code fences (` ```text ` where snippets are grammar, not Rust).
- Validation checks passed:
  - `cargo fmt -p talkbank-tree-sitter-parser`
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=224`)
- Coverage gate (non-test/non-generated `talkbank-tree-sitter-parser/src`) now reports:
  - files missing direct CHAT anchor links: `101` (down from `123`)
  - remaining low-value stub hits (`Runs/Parses/Type representing` scan): `0`

#### `W27-03` (`talkbank-chat`, tier parser/API documentation pass with CHAT anchors)

- [x] `crates/talkbank-tree-sitter-parser/src/api/parser_impl/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/parser_impl/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/tiers/action.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/tiers/grammar.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/tiers/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/tiers/morphology.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/tiers/phonology.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/api/tiers/text.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_dispatch/finder.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/utterance_parser.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/dependent_tier.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/act.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/cod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/wor.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/pho/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/pho/groups.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/pho/unparsed.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/gra/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/gra/tier.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/gra/relation.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/mor/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/mor/tier.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/mor/item.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/mor/word.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/sin/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/sin/groups.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/sin/parse.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/text/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/text/add.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/text/com.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/text/exp.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/text/gpx.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/text/int.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/text/sit.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/text/spa.rs`

Wave note:
- Added module-level CHAT manual links across tier API surface and core typed tier parser modules.
- Rewrote subsystem placeholder docs into behavior-oriented descriptions (tier semantics, CST shape expectations, and parser responsibilities).
- Replaced newly-touched mechanical phrasing (`Runs ...`, subsystem boilerplate) with explicit conversion intent and error-handling context.
- Validation checks passed:
  - `cargo fmt -p talkbank-tree-sitter-parser`
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=228`)
- Coverage gate (non-test/non-generated `talkbank-tree-sitter-parser/src`) now reports:
  - files missing direct CHAT anchor links: `64` (down from `101`)
  - remaining low-value stub hits (`Runs/Parses/Type representing` scan): `40`

#### `W27-04` (`talkbank-chat`, main-tier tree parsing documentation and anchor pass)

- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/word/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/errors.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/finder.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/terminator.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/structure/utterance_end.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/word.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/quotation.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/nonword.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/errors.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/group/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/group/contents.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/base/internal_bullet.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/content/base/overlap_point.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/retrace.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/overlap.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/error_annotation.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/replacement/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/scoped/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/scoped/list.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/scoped/single.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/main_tier/annotations/scoped/symbols.rs`

Wave note:
- Added direct CHAT manual anchors throughout `main_tier` tree-parsing modules, including structure/content/annotation layers and shared `tree_parsing` entrypoints.
- Replaced remaining subsystem-placeholder docs in these files with explicit parser-contract comments (what each layer converts, and where delegation occurs).
- Normalized malformed grammar doc fences in touched files (` ```text ` blocks).
- Validation checks passed:
  - `cargo fmt -p talkbank-tree-sitter-parser`
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=230`)
- Coverage gate (non-test/non-generated `talkbank-tree-sitter-parser/src`) now reports:
  - files missing direct CHAT anchor links: `36` (down from `64`)
  - remaining low-value stub hits (`Runs/Parses/Type representing` scan): `35`

#### `W27-05` (`talkbank-chat`, parser helpers/headers/bullets anchor completion)

- [x] `crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_dispatch/tests.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/participants/birth.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/participants/builder.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/participants/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/bullet_content/inline_bullet.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/bullet_content/inline_pic.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/bullet_content/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/bullet_content/parse.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/dependent_tier/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/freecode/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/id/helpers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/id/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/metadata/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/header/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/media_bullet.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/error_analysis/dependent_tier.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/error_analysis/header.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/error_analysis/line.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/error_analysis/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/error_analysis/utterance.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/error_checking.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/ca.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/overlap.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/pause.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/supertypes/annotations.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/supertypes/ca.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/supertypes/headers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/supertypes/linkers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/supertypes/mod.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/supertypes/overlap.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/supertypes/terminators.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/parser_helpers/supertypes/tiers.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/postcode/mod.rs`

Wave note:
- Completed direct CHAT manual anchor coverage for all non-test, non-generated files in `talkbank-tree-sitter-parser/src`.
- Rewrote remaining subsystem placeholder docs in participants/helpers/supertypes/bullet/header modules with concrete behavior and manual linkage.
- Cleaned lingering low-value test doc stubs in `header_dispatch/tests.rs`.
- Validation checks passed:
  - `cargo fmt -p talkbank-tree-sitter-parser`
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=230`)
- Coverage gate (non-test/non-generated `talkbank-tree-sitter-parser/src`) now reports:
  - files missing direct CHAT anchor links: `0` (down from `36`)
  - remaining low-value stub hits (`Runs/Parses/Type representing` scan): `0` (excluding `tests.rs`)

### Wave 28 Batch Plan (Rolling)

| Batch | Repo | Files | Owner | Status |
|------|------|------:|-------|--------|
| `W28-01` | `talkbank-chat` | 14 | codex | merged |
| `W28-02` | `talkbank-chat` | 16 | codex | merged |
| `W28-03` | `talkbank-chat` | 7 | codex | merged |
| `W28-04` | `talkbank-chat` | 5 | codex | merged |
| `W28-05` | `talkbank-chat` | 11 | codex | merged |
| `W28-06` | `talkbank-chat` | 9 | codex | merged |
| `W28-07` | `talkbank-chat` | 10 | codex | merged |
| `W28-08` | `talkbank-chat` | 28 | codex | merged |

#### `W28-01` (`talkbank-chat`, talkbank-errors crate anchor + quality-doc pass)

- [x] `crates/talkbank-errors/src/lib.rs`
- [x] `crates/talkbank-errors/src/types.rs`
- [x] `crates/talkbank-errors/src/context.rs`
- [x] `crates/talkbank-errors/src/sink.rs`
- [x] `crates/talkbank-errors/src/config.rs`
- [x] `crates/talkbank-errors/src/configurable_sink.rs`
- [x] `crates/talkbank-errors/src/offset_adjusting_sink.rs`
- [x] `crates/talkbank-errors/src/chat_formatting.rs`
- [x] `crates/talkbank-errors/src/line_map.rs`
- [x] `crates/talkbank-errors/src/span_shift.rs`
- [x] `crates/talkbank-errors/src/enhance.rs`
- [x] `crates/talkbank-errors/src/codes/mod.rs`
- [x] `crates/talkbank-errors/src/codes/error_code.rs`
- [x] `crates/talkbank-errors/src/codes/temporal.rs`

Wave note:
- Added direct CHAT manual anchors across all non-test `talkbank-errors` modules.
- Replaced low-value subsystem stub comments in sink/span-shift code with behavior-oriented docs.
- Removed stale literal `\\n` artifacts in touched module docs.
- Validation checks passed:
  - `cargo fmt -p talkbank-errors`
  - `cargo check -p talkbank-errors -q`
  - `cargo test -p talkbank-errors --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=230`)
- Coverage gate (non-test/non-generated Rust files under `talkbank-chat/crates`) now reports:
  - files missing direct CHAT anchor links: `81` (down from `95`)

#### `W28-02` (`talkbank-chat`, talkbank-direct-parser crate anchor + quality-doc pass)

- [x] `crates/talkbank-direct-parser/src/lib.rs`
- [x] `crates/talkbank-direct-parser/src/file.rs`
- [x] `crates/talkbank-direct-parser/src/header.rs`
- [x] `crates/talkbank-direct-parser/src/main_tier.rs`
- [x] `crates/talkbank-direct-parser/src/word.rs`
- [x] `crates/talkbank-direct-parser/src/dependent_tier.rs`
- [x] `crates/talkbank-direct-parser/src/mor_tier.rs`
- [x] `crates/talkbank-direct-parser/src/gra_tier.rs`
- [x] `crates/talkbank-direct-parser/src/pho_tier.rs`
- [x] `crates/talkbank-direct-parser/src/sin_tier.rs`
- [x] `crates/talkbank-direct-parser/src/text_tier.rs`
- [x] `crates/talkbank-direct-parser/src/wor_tier.rs`
- [x] `crates/talkbank-direct-parser/src/tokens.rs`
- [x] `crates/talkbank-direct-parser/src/whitespace.rs`
- [x] `crates/talkbank-direct-parser/src/recovery.rs`
- [x] `crates/talkbank-direct-parser/src/primitives.rs`

Wave note:
- Added direct CHAT manual anchor sections to every non-test module in `talkbank-direct-parser`.
- Replaced remaining low-value non-test `/// Runs ...` comments with behavior-oriented API docs in parser entry points and helper utilities.
- Cleaned an escaped newline artifact in `recovery.rs` module docs.
- Validation checks passed:
  - `cargo fmt -p talkbank-direct-parser`
  - `cargo check -p talkbank-direct-parser -q`
  - `cargo test -p talkbank-direct-parser --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=230`)
- Coverage gate (non-test/non-generated Rust files under `talkbank-chat/crates`) now reports:
  - files missing direct CHAT anchor links: `64` (down from `81`)

#### `W28-03` (`talkbank-chat`, talkbank-derive crate anchor + quality-doc pass)

- [x] `crates/talkbank-derive/src/lib.rs`
- [x] `crates/talkbank-derive/src/helpers.rs`
- [x] `crates/talkbank-derive/src/error_code_enum.rs`
- [x] `crates/talkbank-derive/src/semantic_eq.rs`
- [x] `crates/talkbank-derive/src/semantic_diff.rs`
- [x] `crates/talkbank-derive/src/span_shift.rs`
- [x] `crates/talkbank-derive/src/validation_tagged.rs`

Wave note:
- Added direct CHAT manual anchors to every non-test module in `talkbank-derive`.
- Replaced non-informative derive-generated API comments (`Runs ...`, vague method docs) with behavior-oriented descriptions.
- Validation checks passed:
  - `cargo fmt -p talkbank-derive`
  - `cargo check -p talkbank-derive -q`
  - `cargo test -p talkbank-derive --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=230`)
- Coverage gate (non-test/non-generated Rust files under `talkbank-chat/crates`) now reports:
  - files missing direct CHAT anchor links: `57` (down from `64`)

#### `W28-04` (`talkbank-chat`, utility/API singleton files + node-types generator)

- [x] `crates/talkbank-json/src/lib.rs`
- [x] `crates/talkbank-parser-api/src/lib.rs`
- [x] `crates/talkbank-pipeline/src/lib.rs`
- [x] `crates/talkbank-tree-sitter-parser/src/node_types.rs`
- [x] `scripts/generate-node-types.js`

Wave note:
- Added direct CHAT manual anchors to previously uncovered singleton files in `talkbank-json`, `talkbank-parser-api`, and `talkbank-pipeline`.
- Replaced remaining low-value conversion docs in `ParseOutcome` impls with behavior-oriented wording.
- Updated `scripts/generate-node-types.js` so regenerated `node_types.rs` always includes the CHAT manual anchor block.
- Validation checks passed:
  - `cargo fmt -p talkbank-json -p talkbank-parser-api -p talkbank-pipeline -p talkbank-tree-sitter-parser`
  - `cargo check -p talkbank-json -p talkbank-parser-api -p talkbank-pipeline -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-json -p talkbank-parser-api -p talkbank-pipeline -p talkbank-tree-sitter-parser --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=230`)
- Coverage gate (non-test/non-generated Rust files under `talkbank-chat/crates`) now reports:
  - files missing direct CHAT anchor links: `53` (down from `57`)

#### `W28-05` (`talkbank-chat`, talkbank-transform corpus/pipeline documentation batch)

- [x] `crates/talkbank-transform/src/lib.rs`
- [x] `crates/talkbank-transform/src/corpus/mod.rs`
- [x] `crates/talkbank-transform/src/corpus/discovery.rs`
- [x] `crates/talkbank-transform/src/corpus/manifest.rs`
- [x] `crates/talkbank-transform/src/pipeline/mod.rs`
- [x] `crates/talkbank-transform/src/pipeline/convert.rs`
- [x] `crates/talkbank-transform/src/pipeline/io.rs`
- [x] `crates/talkbank-transform/src/pipeline/parse.rs`
- [x] `crates/talkbank-transform/src/pipeline/error.rs`
- [x] `crates/talkbank-transform/src/lock_helpers.rs`
- [x] `crates/talkbank-transform/src/rendering.rs`

Wave note:
- Added direct CHAT manual anchors across transform crate entry points, corpus discovery/manifest modules, and core parse/convert pipeline modules.
- Replaced low-value utility docs (`Runs fmt`, `Runs from`, escaped doc artifacts) with behavior-oriented comments.
- Validation checks passed:
  - `cargo fmt -p talkbank-transform`
  - `cargo check -p talkbank-transform -q`
  - `cargo test -p talkbank-transform --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=230`)
- Coverage gate (non-test/non-generated Rust files under `talkbank-chat/crates`) now reports:
  - files missing direct CHAT anchor links: `42` (down from `53`)

#### `W28-06` (`talkbank-chat`, talkbank-transform unified-cache documentation batch)

- [x] `crates/talkbank-transform/src/unified_cache/mod.rs`
- [x] `crates/talkbank-transform/src/unified_cache/types.rs`
- [x] `crates/talkbank-transform/src/unified_cache/error.rs`
- [x] `crates/talkbank-transform/src/unified_cache/cache_impl.rs`
- [x] `crates/talkbank-transform/src/unified_cache/cache_utils.rs`
- [x] `crates/talkbank-transform/src/unified_cache/schema_init.rs`
- [x] `crates/talkbank-transform/src/unified_cache/maintenance_ops.rs`
- [x] `crates/talkbank-transform/src/unified_cache/roundtrip_ops.rs`
- [x] `crates/talkbank-transform/src/unified_cache/validation_ops.rs`

Wave note:
- Added direct CHAT manual anchor blocks across the entire `unified_cache` module family.
- Replaced low-value trait/method docs in `cache_impl.rs` with explicit behavior semantics.
- Validation checks passed:
  - `cargo fmt -p talkbank-transform`
  - `cargo check -p talkbank-transform -q`
  - `cargo test -p talkbank-transform --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=230`)
- Coverage gate (non-test/non-generated Rust files under `talkbank-chat/crates`) now reports:
  - files missing direct CHAT anchor links: `33` (down from `42`)

#### `W28-07` (`talkbank-chat`, talkbank-transform validation-runner documentation batch)

- [x] `crates/talkbank-transform/src/validation_runner/mod.rs`
- [x] `crates/talkbank-transform/src/validation_runner/types.rs`
- [x] `crates/talkbank-transform/src/validation_runner/config.rs`
- [x] `crates/talkbank-transform/src/validation_runner/cache.rs`
- [x] `crates/talkbank-transform/src/validation_runner/helpers.rs`
- [x] `crates/talkbank-transform/src/validation_runner/worker.rs`
- [x] `crates/talkbank-transform/src/validation_runner/wor_filter.rs`
- [x] `crates/talkbank-transform/src/validation_runner/runner.rs`
- [x] `crates/talkbank-transform/src/validation_runner/roundtrip.rs`
- [x] `crates/talkbank-transform/src/validation_runner/tests.rs`

Wave note:
- Added CHAT manual anchor sections across all `validation_runner` modules.
- Removed stale escaped-doc artifacts and low-value default-doc stubs in configuration docs.
- Validation checks passed:
  - `cargo fmt -p talkbank-transform`
  - `cargo check -p talkbank-transform -q`
  - `cargo test -p talkbank-transform --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=230`)
- Coverage gate (non-test/non-generated Rust files under `talkbank-chat/crates`) now reports:
  - files missing direct CHAT anchor links: `24` (down from `33`)

#### `W28-08` (`talkbank-chat`, talkbank-parser-tests documentation completion batch)

- [x] `crates/talkbank-parser-tests/src/lib.rs`
- [x] `crates/talkbank-parser-tests/src/bug_annotations.rs`
- [x] `crates/talkbank-parser-tests/src/feature_signature.rs`
- [x] `crates/talkbank-parser-tests/src/snapshot.rs`
- [x] `crates/talkbank-parser-tests/src/template.rs`
- [x] `crates/talkbank-parser-tests/src/test_error.rs`
- [x] `crates/talkbank-parser-tests/src/bin/*.rs` (18 files)
- [x] `crates/talkbank-parser-tests/examples/compare_file.rs`
- [x] `crates/talkbank-parser-tests/tests/error_words_validation.rs`
- [x] `crates/talkbank-parser-tests/tests/parse_error_corpus.rs`
- [x] `crates/talkbank-parser-tests/tests/warning_corpus.rs`

Wave note:
- Added direct CHAT manual anchor blocks across all previously uncovered `talkbank-parser-tests` source files.
- Replaced or upgraded low-value helper docs in key parser-test utility modules and binaries.
- Resolved a pre-existing doctest break in `template.rs` (` ```\\n ` fence artifact).
- Validation checks passed:
  - `cargo fmt -p talkbank-parser-tests`
  - `cargo check -p talkbank-parser-tests -q`
  - `cargo test -p talkbank-parser-tests --doc -q`
  - `make -s chat-anchors-check` (`missing=0`, `referenced_anchors=230`)
- Coverage gate (non-test/non-generated Rust files under `talkbank-chat/crates`) now reports:
  - files missing direct CHAT anchor links: `0` (down from `24`)

#### `W28-09` (`talkbank-chatter`, talkbank-cli documentation completion batch)

- [x] `crates/talkbank-cli/src/**/*.rs` (38 files)
- [x] `crates/talkbank-cli/tests/cache_tests.rs` (targeted low-value doc cleanup)

Wave note:
- Added direct CHAT manual anchor sections to all `talkbank-cli` Rust source files that were missing anchors.
- Replaced low-value boilerplate docs (`Runs ...`, `Type representing ...`, generic subsystem stubs) with behavior-oriented comments in CLI dispatch, validation, TUI state/rendering, and alignment display modules.
- Fixed stale escaped-doc artifact in `commands/schema.rs` (`\n` literal in module docs).
- Validation checks passed:
  - `cargo fmt -p talkbank-cli`
  - `cargo check -p talkbank-cli -q`
  - `cargo test -p talkbank-cli -q`

#### `W28-10` (`talkbank-chatter`, talkbank-lsp + send2clan documentation completion batch)

- [x] `crates/talkbank-lsp/src/**/*.rs` (51 files)
- [x] `crates/send2clan-sys/src/tests.rs`
- [x] `crates/talkbank-revai/src/lib.rs` (targeted low-value doc cleanup)

Wave note:
- Added direct CHAT manual anchor sections to all previously uncovered `talkbank-lsp` modules (alignment, backend, graph, semantic tokens, entrypoints).
- Replaced low-value LSP docs (`Runs ...`, generic subsystem labels) with concrete contract/invariant wording in hover, highlight range finding, request routing, capability declaration, and graph rendering helpers.
- Resolved escaped-doc artifacts in `talkbank-lsp` formatter/type modules and removed leftover mechanical test docs in `send2clan-sys`.
- Validation checks passed:
  - `cargo fmt -p talkbank-lsp -p send2clan-sys -p talkbank-revai`
  - `cargo check -p talkbank-lsp -p send2clan-sys -p talkbank-revai -q`
  - `cargo test -p talkbank-lsp -q`
  - `cargo test -p send2clan-sys -q`
  - `cargo test -p talkbank-revai -q`
- Coverage gate for `talkbank-chatter` Rust sources now reports:
  - files missing CHAT manual anchor links (non-generated, excluding `tests/` and `examples/` directories): `0`

#### `W28-11` (`batchalign3/rust`, shared parser-support crate documentation batch)

- [x] `rust/crates/talkbank-json/src/lib.rs`
- [x] `rust/crates/talkbank-parser-api/src/lib.rs`
- [x] `rust/crates/talkbank-pipeline/src/lib.rs`
- [x] `rust/crates/talkbank-revai/src/{lib.rs,types.rs}`
- [x] `rust/crates/talkbank-derive/src/*.rs` (7 files)
- [x] `rust/crates/talkbank-errors/src/**/*.rs` (15 files)
- [x] `rust/crates/talkbank-direct-parser/src/**/*.rs` (15 files)

Wave note:
- Added direct CHAT manual anchor sections to all previously uncovered non-test source files in seven shared Rust support crates.
- Filled module-level docs for `talkbank-errors` files that previously had no crate/module docs (`context`, `span_shift`, `codes/mod`, `sink`, `types`, etc.).
- Carried through formatting updates in touched crates (including one existing direct-parser test file touched by workspace formatting).
- Validation checks passed:
  - `cargo fmt -p talkbank-json -p talkbank-parser-api -p talkbank-pipeline -p talkbank-revai -p talkbank-derive -p talkbank-errors -p talkbank-direct-parser`
  - `cargo check -p talkbank-json -p talkbank-parser-api -p talkbank-pipeline -p talkbank-revai -p talkbank-derive -p talkbank-errors -p talkbank-direct-parser -q`
  - `cargo test -p talkbank-json -p talkbank-parser-api -p talkbank-pipeline -p talkbank-revai -p talkbank-derive -p talkbank-errors -p talkbank-direct-parser --doc -q`
- Remaining uncovered Rust crates in `batchalign3/rust/crates` are now concentrated in:
  - `talkbank-tree-sitter-parser` (`178`)
  - `talkbank-model` (`136`)
  - `talkbank-transform` (`31`)
  - `talkbank-parser-tests` (`24`)
  - `batchalign-core` (`16`)

#### `W28-12` (`batchalign3/rust`, talkbank-model documentation completion batch)

- [x] `rust/crates/talkbank-model/src/lib.rs`
- [x] `rust/crates/talkbank-model/src/model/**/*.rs` (67 files)
- [x] `rust/crates/talkbank-model/src/alignment/**/*.rs` (17 files)
- [x] `rust/crates/talkbank-model/src/validation/**/*.rs` (51 files)
- [x] `rust/crates/talkbank-model` non-generated source coverage complete

Wave note:
- Added direct CHAT manual anchor sections across all remaining non-generated `talkbank-model` source files (including validation and alignment modules).
- Preserved existing substantive module/type docs and appended anchor sections where missing.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model --doc -q`
  - `cargo test -p talkbank-model -q`
- Remaining uncovered Rust crates in `batchalign3/rust/crates` are now:
  - `talkbank-tree-sitter-parser` (`178`)
  - `talkbank-transform` (`31`)
  - `talkbank-parser-tests` (`24`)
  - `batchalign-core` (`16`)

#### `W28-13` (`batchalign3/rust`, talkbank-transform documentation completion batch)

- [x] `rust/crates/talkbank-transform/src/lib.rs`
- [x] `rust/crates/talkbank-transform/src/corpus/**/*.rs` (3 files)
- [x] `rust/crates/talkbank-transform/src/pipeline/**/*.rs` (5 files)
- [x] `rust/crates/talkbank-transform/src/unified_cache/**/*.rs` (11 files)
- [x] `rust/crates/talkbank-transform/src/validation_runner/**/*.rs` (10 files)
- [x] `rust/crates/talkbank-transform/src/lock_helpers.rs`

Wave note:
- Added missing CHAT anchor sections across all remaining non-generated `talkbank-transform` source files.
- Backfilled crates.io-facing summaries in modules that would otherwise have had anchor-only headers (`pipeline/mod.rs`, `pipeline/error.rs`, `unified_cache/error.rs`).
- Validation checks passed:
  - `cargo fmt -p talkbank-transform`
  - `cargo check -p talkbank-transform -q`
  - `cargo test -p talkbank-transform --doc -q`

#### `W28-14` (`audit`, anchor-only header quality backfill)

- [x] `batchalign3/rust/crates/talkbank-model` anchor-only module docs audit
- [x] 40 non-test `talkbank-model` files rewritten with purpose/contract summary lines before CHAT anchors
- [x] cross-repo non-test anchor-only check (`talkbank-chat`, `talkbank-chatter`, `batchalign3`) now clean

Wave note:
- Per user feedback, shifted from anchor-presence checks to crates.io quality checks:
  - no anchor-only headers in non-test/non-generated Rust source
  - no low-value boilerplate stubs (`Runs ...`, `Type representing ...`, etc.) in newly touched paths
- Post-backfill audit result:
  - `talkbank-chat` non-test anchor-only: `0`
  - `talkbank-chatter` non-test anchor-only: `0`
  - `batchalign3` non-test anchor-only: `0`
- Remaining uncovered Rust crates in `batchalign3/rust/crates` are now:
  - `talkbank-tree-sitter-parser` (`178`)
  - `talkbank-parser-tests` (`24`)
  - `batchalign-core` (`16`)

#### `W28-15` (`batchalign3/rust`, crates.io doc-quality pass for parser-tests + batchalign-core)

- [x] `rust/crates/batchalign-core/src/**/*.rs` (16 files)
- [x] `rust/crates/talkbank-parser-tests/src/{lib.rs,bug_annotations.rs,feature_signature.rs,snapshot.rs,template.rs,test_error.rs}`
- [x] rustfmt carry-through in `talkbank-parser-tests` test/bin files touched by package formatting

Wave note:
- Added CHAT anchor sections to the remaining non-bin `batchalign-core` and `talkbank-parser-tests` source files.
- Added substantive module summaries in files that previously had no module docs (`batchalign-core/src/nlp/{mod,types,mapping,validation}.rs`, `talkbank-parser-tests/src/test_error.rs`).
- Removed one remaining low-value phrase in `batchalign-core` API docs (`Runs the complete ...` -> `Performs the complete ...`).
- Validation checks passed:
  - `cargo fmt -p batchalign-core -p talkbank-parser-tests`
  - `cargo check -p batchalign-core -p talkbank-parser-tests -q`
  - `cargo test -p batchalign-core --doc -q`
  - `cargo test -p talkbank-parser-tests --doc -q`
  - `cargo test -p batchalign-core -q`
- Known environment gap during full test run:
  - `cargo test -p talkbank-parser-tests -q` fails in this workspace because `tests/parser_equivalence_files.rs` expects `../../corpus/reference/**/*.cha`, which is not present locally.
- Remaining uncovered Rust crate in `batchalign3/rust/crates` (non-test/non-bin/non-generated) is now:
  - `talkbank-tree-sitter-parser` (`170`)

#### `W28-16` (`batchalign3/rust`, talkbank-tree-sitter-parser public API batch)

- [x] `rust/crates/talkbank-tree-sitter-parser/src/lib.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/api/**/*.rs` (14 files)
- [x] `rust/crates/talkbank-tree-sitter-parser/src/node_types.rs` (doctest fix)

Wave note:
- Added CHAT anchor sections across the public API surface (`lib` + `api`) while preserving existing architecture-focused module docs.
- Fixed a pre-existing doctest break in `node_types.rs` by making the example import path crate-extern and marking the partial snippet as `ignore`.
- Validation checks passed:
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`
- Remaining uncovered Rust crate scope in `batchalign3/rust/crates` (non-test/non-bin/non-generated) is now:
  - `talkbank-tree-sitter-parser` (`155`)

#### `W28-17` (`batchalign3/rust`, talkbank-tree-sitter-parser parser-orchestration batch)

- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/mod.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/mod.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/mod.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/dependent_tier_dispatch/mod.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_dispatch/mod.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/{mod.rs,pre_begin.rs,helpers.rs,dispatch/mod.rs}`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/parser_struct.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/single_item/{mod.rs,parse_tiers.rs}`

Wave note:
- Added CHAT anchor sections to high-level parser orchestration modules that already contained substantive module docs.
- Kept this batch scoped to orchestration entrypoints to avoid introducing anchor-only docs in low-level helper modules lacking narrative headers.
- Validation checks passed:
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`
- Remaining uncovered Rust crate scope in `batchalign3/rust/crates` (non-test/non-bin/non-generated) is now:
  - `talkbank-tree-sitter-parser` (`143`)

#### `W28-18` (`batchalign3/rust`, talkbank-tree-sitter-parser chat-file internals batch)

- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/chat_file/{parse,streaming,helpers,normalize}.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/dependent_tier_dispatch/{parse,parsed,raw,unparsed,user_defined,helpers}.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_dispatch/{parse,finder}.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/header_parser/dispatch/{core,gem,simple,special,structured}.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/single_item/{helpers,parse_word,parse_main_tier,parse_utterance}.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/chat_file_parser/utterance_parser.rs`

Wave note:
- Added crates.io-facing module summaries plus CHAT anchors in 22 internal parser files that previously had no module docs.
- Focused on parser-orchestration internals first so docs explain contracts at key dispatch/conversion boundaries before leaf-level helpers.
- Validation checks passed:
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`
- Remaining uncovered Rust crate scope in `batchalign3/rust/crates` (non-test/non-bin/non-generated) is now:
  - `talkbank-tree-sitter-parser` (`121`)

#### `W28-19` (`batchalign3/rust`, talkbank-tree-sitter-parser tier/participants batch)

- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/participants/**/*.rs` (3 files)
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/tier_parsers/**/*.rs` (30 files)

Wave note:
- Added CHAT anchors to 11 modules that already had narrative docs in `tier_parsers` and `participants`.
- Added new crates.io-facing module summaries plus anchors for 22 files that previously lacked module docs (`pho/*`, `gra/*`, `text/*`, `mor/*`, `sin/*`, and participant helpers).
- Validation checks passed:
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`
- Remaining uncovered Rust crate scope in `batchalign3/rust/crates` (non-test/non-bin/non-generated) is now:
  - `talkbank-tree-sitter-parser` (`88`)

#### `W28-20` (`batchalign3/rust`, talkbank-tree-sitter-parser tree_parsing documented-modules batch)

- [x] `rust/crates/talkbank-tree-sitter-parser/src/node_types.rs`
- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/**/*.rs` (34 files with existing narrative module docs)

Wave note:
- Added CHAT anchors to all remaining `tree_parsing` modules that already had substantive top-level docs.
- Kept this batch scoped to already-documented modules to maintain crates.io readability while preparing the next pass for no-doc files.
- Validation checks passed:
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`
- Remaining uncovered Rust crate scope in `batchalign3/rust/crates` (non-test/non-bin/non-generated) is now:
  - `talkbank-tree-sitter-parser` (`54`)

#### `W28-21` (`batchalign3/rust`, talkbank-tree-sitter-parser final no-doc tree_parsing batch)

- [x] `rust/crates/talkbank-tree-sitter-parser/src/parser/tree_parsing/**/*.rs` remaining no-doc files (54 files)

Wave note:
- Added crates.io-facing module summaries plus CHAT anchors to all remaining `tree_parsing` files that previously had no module docs.
- Coverage result after this batch:
  - `batchalign3/rust/crates` non-test/non-bin/non-generated Rust files missing CHAT manual anchors: `0`
- Validation checks passed:
  - `cargo check -p talkbank-tree-sitter-parser -q`
  - `cargo test -p talkbank-tree-sitter-parser --doc -q`

#### `W28-22` (`batchalign3/rust`, talkbank-transform wor-filter + parser docs batch)

- [x] `rust/crates/talkbank-transform/src/validation_runner/wor_filter.rs`
- [x] `rust/crates/talkbank-transform/src/pipeline/parse.rs`
- [x] `rust/crates/talkbank-transform/src/pipeline/convert.rs`

Wave note:
- Added focused `%wor` filter regression tests covering:
  - always-%wor error-code classification
  - `%wor` context detection for `E316` (`UnparsableContent`)
  - filtered/non-filtered behavior in `has_real_errors` and `filter_wor_errors`
- Tightened top-level crates.io docs in parser/conversion modules and kept CHAT-anchor links.
- Validation checks passed:
  - `cargo fmt -p talkbank-transform`
  - `cargo check -p talkbank-transform -q`
  - `cargo test -p talkbank-transform -q`

#### `W28-23` (`batchalign3/rust`, talkbank-model user-tier severity fix + docs)

- [x] `rust/crates/talkbank-model/src/validation/unparsed_tier.rs`
- [x] `rust/crates/talkbank-model/src/validation/word/mod.rs`

Wave note:
- Corrected a real validation bug:
  - `%x` tier diagnostics use warning codes (`W601`/`W602`) and now correctly
    emit `Severity::Warning` instead of `Severity::Error`.
- Added regression assertions for warning severities in `unparsed_tier` tests.
- Rewrote module docs for user-defined tiers and word-validation module entrypoint
  to better describe contracts for contributors.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-24` (`talkbank-chat`, header model docs quality pass)

- [x] `crates/talkbank-model/src/model/header/header_enum/options.rs`
- [x] `crates/talkbank-model/src/model/header/header_enum/header.rs`
- [x] `crates/talkbank-model/src/model/header/enums.rs`
- [x] `crates/talkbank-model/src/model/header/id.rs`
- [x] `crates/talkbank-model/src/model/header/media.rs`
- [x] `crates/talkbank-model/src/model/header/codes/date.rs`

Wave note:
- Replaced remaining mechanical/stale header docs with crates.io-facing API contracts:
  - `@Options` behavior and unknown-token handling expectations
  - header-wrapper ordering semantics (`@Languages`, `@Participants`, `@Options`)
  - `@ID` fixed-slot serialization guarantees (including trailing delimiter behavior)
  - `@Media` field semantics and availability/status vocabulary
- Corrected stale manual links:
  - `Header` doc URL now points to `https://talkbank.org/0info/manuals/CHAT.html#File_Headers`.
- Clarified `@Date` validator scope so contributors know it enforces lexical shape, not full calendar validity.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-25` (`talkbank-chat`, content primitive docs quality pass)

- [x] `crates/talkbank-model/src/model/content/group.rs`
- [x] `crates/talkbank-model/src/model/content/freecode.rs`
- [x] `crates/talkbank-model/src/model/content/action.rs`
- [x] `crates/talkbank-model/src/model/content/event.rs`

Wave note:
- Rewrote docs for core main-tier primitives from descriptive/mechanical style to
  contributor-facing API contracts:
  - clearer role split across `<...>`, `‹...›`, `〔...〕`, and `“...”` group types
  - explicit serialization guarantees for `[^ ...]`, `0`, and `&=...` forms
  - removal of stale/noisy phrasing (`Updates chat`, duplicated line-level prose)
- Kept CHAT-manual references intact while tightening model intent for crates.io readers.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-26` (`talkbank-chat`, timing/overlap/nonvocal docs quality pass)

- [x] `crates/talkbank-model/src/model/content/pause.rs`
- [x] `crates/talkbank-model/src/model/content/long_feature.rs`
- [x] `crates/talkbank-model/src/model/content/overlap.rs`
- [x] `crates/talkbank-model/src/model/content/bullet.rs`
- [x] `crates/talkbank-model/src/model/content/postcode.rs`
- [x] `crates/talkbank-model/src/model/content/other_spoken.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/mod.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/label.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/begin.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/end.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/simple.rs`

Wave note:
- Reworked docs across timing, overlap, and nonvocal primitives to emphasize
  model contracts and serialization guarantees instead of repetitive phrasing.
- Clarified pairing semantics and label contracts for scoped long/nonvocal events.
- Tightened overlap docs around marker kinds vs disambiguation index validation.
- Preserved/retained CHAT manual anchor coverage while reducing mechanical text.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-27` (`talkbank-chat`, separator/linker/terminator/bracketed docs pass)

- [x] `crates/talkbank-model/src/model/content/separator.rs`
- [x] `crates/talkbank-model/src/model/content/linker.rs`
- [x] `crates/talkbank-model/src/model/content/terminator.rs`
- [x] `crates/talkbank-model/src/model/content/bracketed.rs`

Wave note:
- Tightened docs for core turn-structure tokens so crates.io readers get clear
  distinctions between:
  - separators vs linkers vs terminators
  - bracketed container semantics vs item-level token semantics
- Replaced remaining mechanical API wording in these modules with explicit
  contract language around serialization and source-span metadata behavior.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-28` (`talkbank-chat`, word marker docs refinement pass)

- [x] `crates/talkbank-model/src/model/content/word/form.rs`
- [x] `crates/talkbank-model/src/model/content/word/language.rs`
- [x] `crates/talkbank-model/src/model/content/word/category.rs`

Wave note:
- Tightened crates.io-facing docs for word-level marker types:
  - special-form suffixes (`@...`)
  - language overrides (`@s...`)
  - category prefixes (`0`, `&~`, `&-`, `&+`)
- Removed remaining mechanical phrasing in high-traffic helper methods.
- Corrected stale API docs in `WordLanguageMarker`:
  - `as_multiple` docs now match implementation (returns only `Multiple`, not `Ambiguous`).
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-29` (`talkbank-chat`, core word-model docs pass)

- [x] `crates/talkbank-model/src/model/content/word/content.rs`
- [x] `crates/talkbank-model/src/model/content/word/types.rs`

Wave note:
- Refined crates.io-facing docs for the core word model to clarify:
  - internal-word content token contracts vs outer word markers
  - builder/helper method intent on `Word`
  - span metadata semantics for stress/lengthening/compound/underline markers
- Removed repetitive constructor/setter phrasing and replaced with concise API contracts.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-30` (`talkbank-chat`, CA word-marker docs correctness pass)

- [x] `crates/talkbank-model/src/model/content/word/ca/elements.rs`
- [x] `crates/talkbank-model/src/model/content/word/ca/delimiters.rs`
- [x] `crates/talkbank-model/src/model/content/word/untranscribed.rs`

Wave note:
- Tightened crates.io-facing docs for CA word marker types and corrected stale
  symbol descriptions to match actual serialization code paths:
  - `CAElementType::{Constriction, Hardening, HurriedStart, Inhalation, LaughInWord, PitchReset, SuddenStop}`
  - `CADelimiterType::{LowPitch, HighPitch}` and duplicate/ambiguous louder notes
- Kept behavior unchanged; this was documentation-accuracy work plus wording cleanup.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-31` (`talkbank-chat`, word/utterance module docs consistency pass)

- [x] `crates/talkbank-model/src/model/content/word/mod.rs`
- [x] `crates/talkbank-model/src/model/content/word/ca/mod.rs`
- [x] `crates/talkbank-model/src/model/content/utterance_content/types.rs`
- [x] `crates/talkbank-model/src/model/content/utterance_content/write.rs`

Wave note:
- Tightened module-level docs for contributor orientation and corrected stale CA
  symbol examples in `word/ca/mod.rs` to match current code mappings.
- Normalized helper doc wording in utterance-content overlap predicates and
  serialization entrypoint docs.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-32` (`talkbank-chat`, main-tier and shared payload docs pass)

- [x] `crates/talkbank-model/src/model/content/main_tier.rs`
- [x] `crates/talkbank-model/src/model/content/tier_content.rs`

Wave note:
- Tightened contributor-facing docs for two central model entrypoints:
  - `MainTier` speaker/payload/span contracts
  - `TierContent` constructor/setter and serialization helper semantics
- Replaced repetitive “builder-style setter” phrasing with concise API-contract wording
  while keeping behavior unchanged.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-33` (`talkbank-chat`, chat-file core/accessor docs pass)

- [x] `crates/talkbank-model/src/model/file/chat_file/core.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/accessors.rs`

Wave note:
- Tightened crates.io-facing docs around file-layer APIs:
  - explicit type-state transition contract in `ChatFile::change_state` / `validate_into`
  - clearer constructor semantics for `new`, `with_participants`, `with_line_map`
  - concise accessor contracts for header/utterance/participant helpers
- No behavior changes; documentation-only quality pass.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-34` (`talkbank-chat`, file-line serialization docs pass)

- [x] `crates/talkbank-model/src/model/file/line.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/write.rs`

Wave note:
- Tightened file-line API docs for constructor/serialization helpers.
- Corrected anchor wording in `Line` docs to `#Main_Line` for consistency with
  other file-layer docs.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-35` (`talkbank-chat`, annotation + %act/%cod docs contracts pass)

- [x] `crates/talkbank-model/src/model/annotation/annotated.rs`
- [x] `crates/talkbank-model/src/model/annotation/replacement.rs`
- [x] `crates/talkbank-model/src/model/annotation/bracketed.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/act.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/cod.rs`

Wave note:
- Rewrote remaining mechanical constructor/setter docs in high-use annotation and
  dependent-tier APIs into contract-focused crates.io language:
  - lenient construction vs validation-phase invariants for replacement/annotated wrappers
  - explicit serialization boundaries for bracket payload writers
  - clearer `%act` / `%cod` constructor and roundtrip helper expectations
- Improved test-doc phrasing in `%act` and `%cod` modules to describe behavioral intent.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-36` (`talkbank-chat`, %pho/%sin/%gra tier docs consistency pass)

- [x] `crates/talkbank-model/src/model/dependent_tier/pho/tier.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/sin/tier.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/tier.rs`

Wave note:
- Replaced remaining boilerplate method docs with contract-oriented wording in
  phonology, sign/gesture, and grammatical-relation tier models.
- Corrected stale `%gra` manual anchor usage in `GraRelations` docs and improved
  test-doc phrasing so assertions describe behavior instead of generic "verifies" text.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-37` (`talkbank-chat`, dependent-tier text wrappers docs deepening pass)

- [x] `crates/talkbank-model/src/model/dependent_tier/types.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/text/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/content.rs`

Wave note:
- Expanded short constructor/accessor docs into 2-3 sentence contributor-facing
  comments that explain intent, invariants, and when to use convenience helpers.
- Clarified serialization contracts for `DependentTier::write_chat`, macro-generated
  `%com/%exp/%add/%spa/%sit/%gpx/%int` wrappers, and bullet-content emptiness semantics.
- Normalized top-level dependent-tier manual anchor usage to `#Dependent_Tiers`.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-38` (`talkbank-chat`, `%mor` tier/item contributor docs pass)

- [x] `crates/talkbank-model/src/model/dependent_tier/mor/tier.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/item.rs`

Wave note:
- Expanded `%mor` constructor/accessor/serializer docs into richer 2-3 sentence
  explanations focused on alignment contracts (`count_chunks`, terminator handling)
  and validation-phase responsibilities.
- Replaced terse test-helper comments in `%mor` validation tests with intent-driven
  wording that explains why each fixture/assertion exists.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-39` (`talkbank-chat`, `%gra` relation docs and tests pass)

- [x] `crates/talkbank-model/src/model/dependent_tier/gra/relation.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/relation_type.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/tier_type.rs`

Wave note:
- Expanded `%gra` constructor/serializer and test docs into richer 2-3 sentence
  comments that explain root-detection conventions and interning expectations.
- Normalized manual references in `%gra` files to the canonical tier anchor
  `#GrammaticalRelations_Tier`.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-40` (`talkbank-chat`, `%pho`/`%sin` item docs + `%pho` test docs pass)

- [x] `crates/talkbank-model/src/model/dependent_tier/pho/item.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/sin/item.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/pho/tests.rs`

Wave note:
- Expanded short constructor/wrapper docs in `%pho` and `%sin` item models into
  2-3 sentence comments with explicit notes on ordering, invariants, and validation phase.
- Rewrote `%pho` test doc comments to describe behavioral intent rather than
  placeholder "verifies" phrasing.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-41` (`talkbank-chat`, `%wor` + `%mor` feature docs deepening pass)

- [x] `crates/talkbank-model/src/model/dependent_tier/wor.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/analysis/newtypes.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/word/word.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/segment.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/write.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/text/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/mod.rs`

Wave note:
- Expanded `%wor`/`%mor` feature constructor and builder docs into richer 2-3 sentence
  comments focused on invariants (ordering, interning, deferred validation, and alignment use).
- Rewrote remaining short test comments in `%mor` feature and text-tier tests to
  describe behavioral intent rather than generic "verifies" phrasing.
- Normalized remaining dependent-tier manual links to canonical anchors, including
  `#GrammaticalRelations_Tier`.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-42` (`talkbank-chat`, core model + semantic diff test docs pass)

- [x] `crates/talkbank-model/src/model/mod.rs`
- [x] `crates/talkbank-model/src/model/semantic_eq.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/tree_renderer.rs`

Wave note:
- Replaced remaining terse "verifies" test comments in central model and semantic
  utility modules with 2-3 sentence intent-focused docs suitable for crates.io readers.
- Clarified what each regression test is guarding (path parsing, tree insertion,
  compact rendering behavior, and symbol mapping) rather than restating function names.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-43` (`talkbank-chat`, header/intern/metadata docs deepening pass)

- [x] `crates/talkbank-model/src/model/intern.rs`
- [x] `crates/talkbank-model/src/model/non_empty_string.rs`
- [x] `crates/talkbank-model/src/model/header/types_header.rs`
- [x] `crates/talkbank-model/src/model/header/codes/participant.rs`
- [x] `crates/talkbank-model/src/model/header/codes/speaker.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/mod.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/tree_renderer.rs`
- [x] `crates/talkbank-model/src/model/language_metadata/metadata.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/metadata/tests.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/tests.rs`

Wave note:
- Reworked terse constructor/test docs across interning, speaker/participant headers,
  and semantic-diff utilities into 2-3 sentence, contract-focused explanations.
- Clarified utterance language-metadata regression tests with intent-driven comments
  so contributors can see exactly which resolution paths and validation tags are protected.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-44` (`talkbank-chat`, macro docs + CA/test helper polish)

- [x] `crates/talkbank-model/src/model/macros.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/builder.rs`
- [x] `crates/talkbank-model/src/model/content/word/ca/tests.rs`

Wave note:
- Improved macro-generated API doc templates so produced `new`/`as_str` docs
  better explain lexical preservation, interning behavior, and encapsulation.
- Expanded remaining terse builder and CA symbol-test comments into intent-oriented
  2-3 sentence docs suitable for contributor-facing crates.io reading.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-45` (`talkbank-chat`, utterance metadata test-doc cleanup)

- [x] `crates/talkbank-model/src/model/file/utterance/metadata/tests.rs`

Wave note:
- Replaced remaining terse/awkward alignment-test comments with 2-3 sentence
  descriptions focused on what invariants are being protected (`%wor` bullet retention
  and parse-health taint scoping).
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-46` (`talkbank-chat`, semantic-diff + parse-state docs deepening pass)

- [x] `crates/talkbank-model/src/model/semantic_diff/report.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/context.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/types.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/parse_health.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/utterance_language.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/language_metadata_state.rs`

Wave note:
- Expanded terse utility-method docs in semantic diff report/context/types into
  contributor-facing explanations that clarify truncation behavior, traversal context,
  and stable string-key contracts.
- Improved utterance parse/language-state docs to better explain state transitions,
  conservative gating behavior, and intended downstream usage patterns.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-47` (`talkbank-chat`, validation/alignment/retrace contributor-docs pass)

- [x] `crates/talkbank-model/src/alignment/helpers/count.rs`
- [x] `crates/talkbank-model/src/alignment/location_tests.rs`
- [x] `crates/talkbank-model/src/alignment/mor.rs`
- [x] `crates/talkbank-model/src/validation/async_helpers.rs`
- [x] `crates/talkbank-model/src/validation/bullet.rs`
- [x] `crates/talkbank-model/src/validation/context.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/completion.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/mod.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/quotation_precedes.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/quoted_linker.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/edge_cases.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/helpers.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/other_completion.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/quotation_follows.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/quotation_precedes.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/self_completion.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/terminator_linker_pairing.rs`
- [x] `crates/talkbank-model/src/validation/header/mod.rs`
- [x] `crates/talkbank-model/src/validation/header/participant.rs`
- [x] `crates/talkbank-model/src/validation/header/structure.rs`
- [x] `crates/talkbank-model/src/validation/retrace/detection.rs`
- [x] `crates/talkbank-model/src/validation/retrace/rendering/bracketed.rs`
- [x] `crates/talkbank-model/src/validation/retrace/rendering/mod.rs`
- [x] `crates/talkbank-model/src/validation/retrace/rendering/utterance.rs`
- [x] `crates/talkbank-model/src/validation/speaker.rs`
- [x] `crates/talkbank-model/src/validation/temporal.rs`
- [x] `crates/talkbank-model/src/validation/unparsed_tier.rs`
- [x] `crates/talkbank-model/src/validation/utterance/overlap.rs`
- [x] `crates/talkbank-model/src/validation/utterance/tests.rs`
- [x] `crates/talkbank-model/src/validation/word/language/tests.rs`
- [x] `crates/talkbank-model/src/validation/word/structure.rs`
- [x] `crates/talkbank-model/src/validation/word/tests.rs`

Wave note:
- Replaced remaining terse/mechanical doc comments in validation and alignment
  hot paths with contract-oriented, crates.io-facing documentation (invariants,
  traversal semantics, domain-specific alignment behavior, and error intent).
- Completed a full `/// Verifies ...` cleanup in `talkbank-model` tests, rewriting
  those comments into behavior-focused regression intent docs.
- Added/kept relevant CHAT manual anchors at module level in touched validation/
  alignment modules for contributor cross-reference.
- Validation checks passed (re-run after each sub-batch):
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-48` (`talkbank-chat`, `%pho/%wor/%sin` alignment docs polish)

- [x] `crates/talkbank-model/src/alignment/pho.rs`
- [x] `crates/talkbank-model/src/alignment/wor.rs`
- [x] `crates/talkbank-model/src/alignment/sin.rs`

Wave note:
- Tightened alignment API docs for `%pho/%wor/%sin` to emphasize contributor-facing
  contracts (1:1 pairing semantics, mismatch diagnostics, and error-free checks).
- Replaced remaining terse `%sin` test comments with behavior-oriented assertions
  and completed another targeted cleanup of lingering `/// Verifies ...` phrasing.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-49` (`talkbank-chat`, utterance/retrace validation docs contracts pass)

- [x] `crates/talkbank-model/src/validation/cross_utterance/quotation_follows.rs`
- [x] `crates/talkbank-model/src/validation/utterance/ca_delimiter.rs`
- [x] `crates/talkbank-model/src/validation/utterance/underline.rs`
- [x] `crates/talkbank-model/src/validation/utterance/quotation.rs`
- [x] `crates/talkbank-model/src/validation/utterance/comma.rs`
- [x] `crates/talkbank-model/src/validation/retrace/collection/mod.rs`
- [x] `crates/talkbank-model/src/validation/retrace/collection/utterance.rs`
- [x] `crates/talkbank-model/src/validation/retrace/collection/bracketed.rs`

Wave note:
- Reworked terse/templated docs in utterance-level validators and retrace
  collectors into contributor-facing contracts that describe traversal scope,
  stack/pairing behavior, and how emitted checkpoints feed downstream checks.
- Tightened module-level manual references to relevant CHAT sections instead of
  broad unrelated anchor sets.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-50` (`talkbank-chat`, retrace + word-language docs accuracy pass)

- [x] `crates/talkbank-model/src/validation/retrace/mod.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/scoped_markers.rs`
- [x] `crates/talkbank-model/src/validation/word/language/digits.rs`
- [x] `crates/talkbank-model/src/validation/word/language/helpers.rs`
- [x] `crates/talkbank-model/src/validation/word/language/resolve.rs`

Wave note:
- Tightened retrace and scoped-marker validation docs to clarify traversal flow,
  stack/label matching behavior, and diagnostic semantics for contributors.
- Corrected stale documentation around mixed-language digit policy so comments now
  match current permissive implementation (any candidate language may allow digits).
- Expanded helper/resolution docs to better separate resolution mechanics from
  downstream rule quantification policy.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-51` (`talkbank-chat`, alignment-rule + `%wor` extraction docs pass)

- [x] `crates/talkbank-model/src/alignment/helpers/rules.rs`
- [x] `crates/talkbank-model/src/model/content/main_tier.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/validate.rs`

Wave note:
- Expanded terse alignment-rule comments to clarify domain-agnostic exclusion
  predicates and `%mor`/`%wor` lexical inclusion semantics for contributors.
- Improved `%wor` extraction helper docs in `MainTier` so traversal order and
  replacement-word fallback behavior are explicit.
- Tightened file-level option helper docs (`CA`/`bullets`) in `ChatFile` validation
  to better document downstream validation-context effects.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-52` (`talkbank-chat`, semantic-diff + alignment-metadata docs deepening)

- [x] `crates/talkbank-model/src/model/semantic_diff/source_utils.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/tree_renderer.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/metadata/alignment.rs`

Wave note:
- Expanded terse semantic-diff helper docs into contributor-facing contracts,
  especially around byte/line conversion semantics and tree-rendering behavior.
- Tightened utterance alignment-metadata helper docs to clarify why count-based
  fallback alignments exist and how parse-health gating diagnostics are constructed.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-53` (`talkbank-chat`, interner singleton docs contracts pass)

- [x] `crates/talkbank-model/src/model/intern.rs`

Wave note:
- Refined global interner API docs to clarify singleton lifecycle, seeding
  behavior, and process-lifetime reuse expectations for contributors.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-54` (`talkbank-chat`, alignment-count + header-dispatch docs polish)

- [x] `crates/talkbank-model/src/alignment/helpers/count.rs`
- [x] `crates/talkbank-model/src/validation/retrace/detection.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/validate.rs`
- [x] `crates/talkbank-model/src/validation/header/validate.rs`

Wave note:
- Improved contributor-facing docs around fast-path counting and diagnostic
  extraction semantics in alignment helpers.
- Tightened retrace detection docs for nested traversal intent and iterator usage.
- Clarified file-level validation-context construction and header-dispatch roles
  so orchestration responsibilities are explicit to new contributors.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-55` (`talkbank-chat`, alignment/temporal/word-structure docs deepening)

- [x] `crates/talkbank-model/src/alignment/helpers/rules.rs`
- [x] `crates/talkbank-model/src/alignment/mor.rs`
- [x] `crates/talkbank-model/src/validation/word/structure.rs`
- [x] `crates/talkbank-model/src/validation/temporal.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/metadata/alignment.rs`

Wave note:
- Expanded core alignment and temporal contracts with contributor-facing details
  about domain-specific inclusion rules, placeholder-pair semantics, and
  ordering guarantees used by diagnostics.
- Deepened word-structure helper docs to clarify prosodic/helper invariants and
  make small predicate helpers less opaque to new contributors.
- Strengthened utterance alignment-metadata docs around diagnostics cache behavior
  and index-only unit purpose.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-56` (`talkbank-chat`, validation utility docs clarity pass)

- [x] `crates/talkbank-model/src/validation/context.rs`
- [x] `crates/talkbank-model/src/validation/unparsed_tier.rs`
- [x] `crates/talkbank-model/src/validation/header/participant.rs`
- [x] `crates/talkbank-model/src/validation/speaker.rs`

Wave note:
- Clarified core validation utility contracts around digit-policy helpers,
  `%x*` migration warnings, canonical participant-role matching, and speaker-ID
  invalid-character reporting behavior.
- Focused on small but high-frequency APIs used across many validation paths.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-57` (`talkbank-chat`, semantic/language-metadata/alignment-domain docs pass)

- [x] `crates/talkbank-model/src/model/semantic_eq.rs`
- [x] `crates/talkbank-model/src/model/language_metadata/metadata.rs`
- [x] `crates/talkbank-model/src/alignment/helpers/mod.rs`
- [x] `crates/talkbank-model/src/alignment/helpers/domain.rs`

Wave note:
- Upgraded terse docs in `SemanticEq` to clearly define semantic-vs-structural
  equality expectations for roundtrip and migration tests.
- Deepened `LanguageMetadata` container docs around ordering invariants,
  code-switching interpretation, and wrapper semantics used by downstream
  alignment/language consumers.
- Clarified alignment helper module/domain responsibilities so `%mor/%pho/%sin/%wor`
  policy branches are easier for new contributors to reason about.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-58` (`talkbank-chat`, language-metadata + main-tier module docs refinement)

- [x] `crates/talkbank-model/src/model/language_metadata/mod.rs`
- [x] `crates/talkbank-model/src/model/language_metadata/source.rs`
- [x] `crates/talkbank-model/src/model/language_metadata/word_info.rs`
- [x] `crates/talkbank-model/src/model/content/mod.rs`

Wave note:
- Expanded crates.io-facing docs for language metadata provenance and helper
  constructors, with clearer contracts about source semantics and unresolved state.
- Polished main-tier module documentation to better explain how content types
  connect parser output to validation/alignment consumers.
- Added extra intent-level detail to short utility docs to reduce "what is this
  for?" ambiguity for new contributors.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-59` (`talkbank-chat`, word-model API docs refinement)

- [x] `crates/talkbank-model/src/model/content/word/mod.rs`
- [x] `crates/talkbank-model/src/model/content/word/category.rs`
- [x] `crates/talkbank-model/src/model/content/word/form.rs`
- [x] `crates/talkbank-model/src/model/content/word/language.rs`

Wave note:
- Expanded terse docs in core word-model APIs to clarify intent-level contracts
  for omission categories, marker parsing/serialization helpers, and language
  marker interpretation.
- Added more explicit guidance where helper methods are convenience shortcuts
  versus full semantic handling (especially for multi/ambiguous language markers).
- Kept manual anchor coverage intact while shifting wording toward contributor-
  facing crates.io clarity.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-60` (`talkbank-chat`, CA word-marker docs consistency pass)

- [x] `crates/talkbank-model/src/model/content/word/ca/mod.rs`
- [x] `crates/talkbank-model/src/model/content/word/ca/delimiters.rs`
- [x] `crates/talkbank-model/src/model/content/word/ca/elements.rs`
- [x] `crates/talkbank-model/src/model/content/word/untranscribed.rs`
- [x] `crates/talkbank-model/src/model/content/word/mod.rs`

Wave note:
- Tightened CA marker docs around parser/serializer symbol synchronization and
  clarified where pairing constraints are enforced (token-level vs utterance-level).
- Improved constructor/span docs to better communicate diagnostic-only metadata
  semantics and builder usage patterns for contributors.
- Added missing intent detail on untranscribed marker serialization behavior.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-61` (`talkbank-chat`, dependent-tier + nonvocal contract docs pass)

- [x] `crates/talkbank-model/src/model/dependent_tier/pho/word.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/kind.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/relation.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/tier_type.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/label.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/begin.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/end.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/simple.rs`

Wave note:
- Expanded thin module/API docs in dependent-tier utilities and `%pho/%gra`
  model types to make data contracts clearer for crates.io readers.
- Tightened nonvocal marker docs around span semantics, pairing behavior, and
  builder usage so parser vs test construction intent is explicit.
- Kept CHAT-manual anchor coverage while improving rationale-level wording.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-62` (`talkbank-chat`, semantic-diff + header-code docs refinement)

- [x] `crates/talkbank-model/src/model/semantic_diff/context.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/report.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/mod.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/types.rs`
- [x] `crates/talkbank-model/src/model/header/codes/language.rs`
- [x] `crates/talkbank-model/src/model/header/codes/speaker.rs`

Wave note:
- Expanded contributor-facing docs in semantic-diff infrastructure to clarify
  traversal context behavior, truncation policy, and stable diff taxonomy goals.
- Improved language/speaker code module docs with stronger API-contract wording
  and explicit compatibility rationale (interning behavior and legacy limits).
- Removed one redundant derive-adjacent doc line in `LanguageCode` while keeping
  manual anchors and model semantics unchanged.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-63` (`talkbank-chat`, `%mor` + header/user-tier docs polish)

- [x] `crates/talkbank-model/src/model/dependent_tier/mor/analysis/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/item.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/tier.rs`
- [x] `crates/talkbank-model/src/model/user_defined_tier.rs`
- [x] `crates/talkbank-model/src/model/header/types_header.rs`

Wave note:
- Expanded thin module-level docs around `%mor` analysis atoms and clarified
  `%mor` item/chunk semantics used by `%gra` alignment.
- Tightened content-validation docs to distinguish lexical checks from
  structural alignment checks in `%mor` pipelines.
- Improved contributor-facing rationale for user-defined tier storage and
  `@Types` constructor ordering constraints.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-64` (`talkbank-chat`, `%mor/%gra` and scoped-annotation module docs pass)

- [x] `crates/talkbank-model/src/model/dependent_tier/mor/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/word/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/mod.rs`
- [x] `crates/talkbank-model/src/model/annotation/scoped/mod.rs`
- [x] `crates/talkbank-model/src/model/annotation/scoped/types.rs`

Wave note:
- Expanded thin module docs in `%mor`/`%gra` entry points to better describe
  how atom/word/tier layers compose for parser and serializer pipelines.
- Improved scoped-annotation docs with clearer typed-model intent and overlap
  index construction/validation contract.
- Removed one redundant derive-adjacent enum doc line while preserving behavior.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-65` (`talkbank-chat`, header/bullet/`%mor` entry docs refinement)

- [x] `crates/talkbank-model/src/model/header/codes/header_strings.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/segment.rs`
- [x] `crates/talkbank-model/src/validation/header/metadata.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/word/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/gra/relation_type.rs`

Wave note:
- Strengthened module-level intent docs for header string newtypes, bullet
  content segment primitives, and metadata header validators.
- Clarified `%mor` entry-module roles and added more explicit contracts around
  lossless segment preservation and diagnostic granularity.
- Added rationale-level notes for `%gra` relation-label interning behavior.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-66` (`talkbank-chat`, nonvocal + source/validation helper docs pass)

- [x] `crates/talkbank-model/src/model/content/nonvocal/begin.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/end.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/label.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/simple.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/source_utils.rs`
- [x] `crates/talkbank-model/src/validation/speaker.rs`
- [x] `crates/talkbank-model/src/validation/unparsed_tier.rs`

Wave note:
- Expanded thin module-level docs in nonvocal marker files to clarify scoped
  versus point-event semantics and pairing expectations.
- Improved semantic-diff source-helper docs to better explain why byte-span
  conversion and caret rendering utilities are centralized.
- Tightened contributor-facing rationale in speaker and `%x*` tier validation
  helper modules (lenient policy and migration intent).
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-67` (`talkbank-chat`, thin `%gra/%mor` and test-helper docs pass)

- [x] `crates/talkbank-model/src/model/dependent_tier/gra/tier_type.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/tier.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/types.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/pho/tests.rs`
- [x] `crates/talkbank-model/src/validation/word/snapshot_tests.rs`
- [x] `crates/talkbank-model/src/model/content/utterance_content/tests.rs`

Wave note:
- Expanded remaining short module docs in `%gra/%mor` and semantic-diff type
  modules to clarify architectural intent for contributors.
- Replaced terse helper/test comments in snapshot and layout-inspection tests
  with behavior-focused explanations of what regressions each helper guards.
- Kept this wave documentation-only and compatibility-neutral.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-68` (`talkbank-chat`, test-module intent docs expansion)

- [x] `crates/talkbank-model/src/alignment/gra/tests.rs`
- [x] `crates/talkbank-model/src/alignment/helpers/tests.rs`
- [x] `crates/talkbank-model/src/model/content/word/ca/tests.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/tests.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/tests.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/sin/tests.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/tests.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/metadata/tests.rs`

Wave note:
- Expanded terse one-line module docs across key test suites so contributors can
  quickly see what regression surface each module protects.
- Focused this wave on intent-level test documentation only; no behavioral code
  changes were introduced.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-69` (`talkbank-chat`, cross-utterance and metadata test docs pass)

- [x] `crates/talkbank-model/src/model/language_metadata/tests.rs`
- [x] `crates/talkbank-model/src/model/participant/tests.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests.rs`
- [x] `crates/talkbank-model/src/validation/word/language/tests.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/edge_cases.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/helpers.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/other_completion.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/quotation_follows.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/quotation_precedes.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/self_completion.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/terminator_linker_pairing.rs`

Wave note:
- Replaced generic one-line test-module docs with contributor-facing summaries
  that explain each suite’s regression surface and current rule-status context.
- Clarified helper-module intent so test fixture utilities are easier to reason
  about when re-enabling deferred cross-utterance validations.
- Kept changes documentation-only; no runtime behavior updates.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-70` (`talkbank-chat`, core content-token constructor docs deepening)

- [x] `crates/talkbank-model/src/model/content/action.rs`
- [x] `crates/talkbank-model/src/model/content/event.rs`
- [x] `crates/talkbank-model/src/model/content/freecode.rs`
- [x] `crates/talkbank-model/src/model/content/postcode.rs`
- [x] `crates/talkbank-model/src/model/content/pause.rs`
- [x] `crates/talkbank-model/src/model/content/bullet.rs`
- [x] `crates/talkbank-model/src/model/content/other_spoken.rs`
- [x] `crates/talkbank-model/src/model/content/overlap.rs`

Wave note:
- Upgraded terse constructor docs across core content-token types to explain
  parser/test usage patterns, span semantics, and source-fidelity intent.
- Clarified where constructor permissiveness is deliberate so validation can
  report richer diagnostics later in the pipeline.
- Kept all changes documentation-only.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-71` (`talkbank-chat`, group/long-feature/file-line constructor docs pass)

- [x] `crates/talkbank-model/src/model/content/long_feature.rs`
- [x] `crates/talkbank-model/src/model/content/group.rs`
- [x] `crates/talkbank-model/src/model/file/line.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/core.rs`

Wave note:
- Expanded constructor and helper docs around type-state boundaries so
  contributors can see where unchecked parse artifacts are intentionally
  accepted and where validation should enforce stricter invariants.
- Clarified span and source-line semantics in file-line and chat-file core
  helpers to make source-fidelity contracts explicit for downstream tools.
- Kept this wave documentation-only.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-72` (`talkbank-chat`, alignment/context/word API docs deepening pass)

- [x] `crates/talkbank-model/src/alignment/types.rs`
- [x] `crates/talkbank-model/src/alignment/pho.rs`
- [x] `crates/talkbank-model/src/alignment/wor.rs`
- [x] `crates/talkbank-model/src/validation/context.rs`
- [x] `crates/talkbank-model/src/model/content/word/types.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/mod.rs`

Wave note:
- Reworked terse alignment API comments into contract-focused docs that clarify
  placeholder-pair semantics, builder accumulation patterns, and mismatch flow.
- Expanded `ValidationContext` builder docs to explain copy-on-write behavior
  (`Arc::make_mut`) and how file-level state differs from per-field overlays.
- Deepened `Word`/`WordContents` API docs around metadata-vs-surface-text
  invariants so contributors do not accidentally desynchronize `raw_text` and
  structured content.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-73` (`talkbank-chat`, residual low-value wording cleanup in model tests/docs)

- [x] `crates/talkbank-model/src/validation/word/tests.rs`
- [x] `crates/talkbank-model/src/model/participant/tests.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/metadata/tests.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/mod.rs`
- [x] `crates/talkbank-model/src/validation/cross_utterance/tests/helpers.rs`

Wave note:
- Replaced the last remaining low-information "Runs/Constructs/Parses" phrasing
  in `talkbank-model` with behavior-focused descriptions tied to test intent.
- Clarified one key parse-health regression test contract (`main` taint vs
  `%mor↔%gra` continuity) to make expected alignment behavior explicit.
- Verified the phrase-based low-value-comment scan now returns zero matches for
  `talkbank-model`.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-74` (`talkbank-chat`, serializer/accessor + semantic-diff docs refinement)

- [x] `crates/talkbank-model/src/model/annotation/scoped/write.rs`
- [x] `crates/talkbank-model/src/model/content/utterance_content/write.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/write.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/accessors.rs`
- [x] `crates/talkbank-model/src/model/participant/accessors.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/impls/container.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/impls/scalar.rs`
- [x] `crates/talkbank-model/src/validation/word/language/mod.rs`

Wave note:
- Deepened serialization/accessor docs to emphasize order-preservation and
  roundtrip guarantees (especially where CHAT control markers are emitted verbatim).
- Clarified semantic-diff container/scalar contracts (prefix-first divergence
  reporting, `Span` metadata exclusion, and delegation semantics).
- Expanded word-language module docs so contributors can navigate resolution vs
  digit-policy responsibilities quickly.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-75` (`talkbank-chat`, header/file/alignment glue docs deepening)

- [x] `crates/talkbank-model/src/model/header/header_enum/impls.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/write.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/validate.rs`
- [x] `crates/talkbank-model/src/validation/word/mod.rs`
- [x] `crates/talkbank-model/src/alignment/gra/mod.rs`
- [x] `crates/talkbank-model/src/alignment/gra/types.rs`
- [x] `crates/talkbank-model/src/validation/header/unknown.rs`
- [x] `crates/talkbank-model/src/validation/config.rs`

Wave note:
- Expanded terse glue-module docs into explicit contracts around canonical label
  mapping, roundtrip serialization ordering, and utterance validation scope.
- Clarified `%mor↔%gra` alignment result semantics (complete rows vs placeholders)
  so contributors can reason about mismatch diagnostics without reading aligner internals.
- Added migration rationale in `validation::config` and legacy-header severity
  rationale in unknown-header validation.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-76` (`talkbank-chat`, module-boundary docs deepening across header/validation/%mor`)

- [x] `crates/talkbank-model/src/model/header/header_enum/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/kind.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/mod.rs`
- [x] `crates/talkbank-model/src/validation/header/mod.rs`
- [x] `crates/talkbank-model/src/validation/utterance/mod.rs`
- [x] `crates/talkbank-model/src/model/semantic_diff/impls/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/analysis/mod.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/mor/word/mod.rs`
- [x] `crates/talkbank-model/src/model/header/codes/date.rs`

Wave note:
- Expanded thin module docs to clarify subsystem boundaries (utterance-local vs
  cross-utterance validation, header dispatch entrypoints, semantic-diff split
  between container/scalar policies).
- Tightened API docs for dependent-tier kind/span helpers and `%mor` atom/word
  submodules to make intent and invariants clearer to new contributors.
- Added rationale notes in date-validation docs (legacy-compatible month policy
  and multi-component error reporting).
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-77` (`talkbank-chat`, accessor/builder/relation-label docs refinement)

- [x] `crates/talkbank-model/src/model/dependent_tier/gra/relation_type.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/builder.rs`
- [x] `crates/talkbank-model/src/model/header/codes/header_strings.rs`
- [x] `crates/talkbank-model/src/model/participant/accessors.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/accessors.rs`

Wave note:
- Added clearer contracts for `%gra` relation-label modeling (open-text token
  preservation plus deferred vocabulary policy).
- Deepened utterance-builder docs around parse-taint semantics, tier ordering,
  and user-defined tier roundtrip behavior.
- Clarified header-string newtype design as intentionally normalization-free and
  tightened accessor docs in participant/chat-file helpers.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-78` (`talkbank-chat`, serializer/validator contract-docs tightening)

- [x] `crates/talkbank-model/src/model/annotation/scoped/write.rs`
- [x] `crates/talkbank-model/src/model/content/utterance_content/write.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/write.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/write.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/validate.rs`
- [x] `crates/talkbank-model/src/model/header/header_enum/impls.rs`

Wave note:
- Added contract-level rationale in core writer APIs to emphasize no-normalization
  roundtrip behavior and ownership of newline/continuation formatting boundaries.
- Clarified utterance validation ordering/statefulness (`validate_with_alignment`)
  so contributors understand cache mutation and diagnostic sequencing.
- Tightened header-name mapping docs to highlight centralized string-table intent.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

#### `W28-79` (`talkbank-chat`, filler doc expansion for low-count files)

- [x] `crates/talkbank-model/src/model/annotation/scoped/write.rs`
- [x] `crates/talkbank-model/src/model/content/utterance_content/write.rs`
- [x] `crates/talkbank-model/src/model/content/nonvocal/label.rs`
- [x] `crates/talkbank-model/src/model/dependent_tier/bullet_content/write.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/accessors.rs`
- [x] `crates/talkbank-model/src/model/file/chat_file/write.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/builder.rs`
- [x] `crates/talkbank-model/src/model/file/utterance/validate.rs`
- [x] `crates/talkbank-model/src/model/header/header_enum/impls.rs`
- [x] `crates/talkbank-model/src/model/participant/accessors.rs`

Wave note:
- Expanded module-level/method-level narratives to clear the remaining
  under-documented files flagged by the `rg` scan (now zero matches).
  Additional context covers writer contracts, parse-taint heuristics,
  and scoped-label invariants.
- Validation checks passed:
  - `cargo fmt -p talkbank-model`
  - `cargo check -p talkbank-model -q`
  - `cargo test -p talkbank-model -q`

## Per-PR Checklist

- [ ] Comments are manually authored, not template-generated.
- [ ] Each new comment passes rubric.
- [ ] No new filler doc comments introduced.
- [ ] Relevant tests/format checks passed.
- [ ] Tracker dashboard and wave board updated.
