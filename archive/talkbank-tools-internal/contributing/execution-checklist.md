# Execution Checklist with File Targets and CI Gates

## Objective
Provide an implementation-ready checklist for executing the full pre-release reorganization.
Every task includes exact file targets, required commands, and blocking CI gates.

## Usage
- Execute tasks in listed order unless marked parallel-safe.
- Do not mark a task complete without attaching command output evidence.
- If a blocking gate fails, stop downstream tasks in that phase.

## Gate Legend
- `G1`: `cd rust && cargo check --all-targets`
- `G2`: `cd spec/tools && cargo check --all-targets`
- `G3`: `cd grammar && tree-sitter generate && tree-sitter test`
- `G4`: `cargo nextest run -p talkbank-parser-tests --test generated`
- `G5`: `cargo nextest run -p talkbank-parser-tests --test parser_equivalence_words`
- `G6`: `make generated-check`
- `G7`: `.github/workflows/ci.yml` green on PR

## Phase 0: Baseline and Guardrails

### T0.1 Establish Verification Entrypoint
- File targets:
  - `Makefile`
  - `docs/contributor/dev-checks.md`
- Actions:
  - add canonical `make verify` target chaining `G1`, `G2`, `G3`, `G4`, `G5`.
- Local commands:
  - `make verify`
- Blocking gates:
  - `G1`, `G2`, `G3`, `G4`, `G5`

### T0.2 Freeze Drift via CI Required Checks
- File targets:
  - `.github/workflows/ci.yml`
  - `docs/contributor/branch-protection.md`
- Actions:
  - ensure all critical jobs are required and documented.
- Local commands:
  - `act` (optional local), then PR CI.
- Blocking gates:
  - `G7`

## Phase 1: Governance and Ownership

### T1.1 Add Ownership Matrix
- File targets:
  - `.github/CODEOWNERS`
  - `docs/architecture/ownership-map.md`
- Actions:
  - map repo sections to owning maintainers.
- Local commands:
  - `rg -n "CODEOWNERS|ownership" .github docs`
- Blocking gates:
  - `G7`

### T1.2 Standardize Contribution and Review Rules
- File targets:
  - `CONTRIBUTING.md`
  - `.github/pull_request_template.md`
  - `.github/ISSUE_TEMPLATE/` (add templates)
- Actions:
  - require doc-impact, test-impact, and generated-artifact checks in every PR.
- Local commands:
  - none mandatory beyond lint/check.
- Blocking gates:
  - `G7`

## Phase 2: Grammar and Symbol Registry Reform

### T2.1 Create Canonical Symbol Registry
- File targets:
  - `spec/symbols/symbol_registry.yaml`
  - `spec/symbols/README.md`
- Actions:
  - define symbol category, allowed contexts, and reserved contexts.
- Local commands:
  - schema/validation command for registry (add if missing).
- Blocking gates:
  - `G6`, `G7`

### T2.2 Generate Symbol Sets for All Consumers
- File targets:
  - `grammar/scripts/generate-symbol-sets.js`
  - `grammar/src/generated_symbol_sets.js`
  - `spec/tools/src/generated/symbol_sets.rs`
  - `rust/crates/talkbank-model/src/generated/symbol_sets.rs`
- Actions:
  - wire one-way generation from registry to all consumers.
- Local commands:
  - `node grammar/scripts/generate-symbol-sets.js`
  - `make generated-check`
- Blocking gates:
  - `G6`, `G7`

### T2.3 Refactor Grammar to Use Generated Sets
- File targets:
  - `grammar/grammar.js`
  - `grammar/src/grammar.json`
- Actions:
  - replace manual forbidden/reserved literals with generated constants.
- Local commands:
  - `cd grammar && tree-sitter generate && tree-sitter test`
- Blocking gates:
  - `G3`, `G6`, `G7`

## Phase 3: Parser and Model Contracts

### T3.1 Publish Parser Contract
- File targets:
  - `docs/architecture/parser-contract.md`
  - `rust/crates/talkbank-parser-api/src/lib.rs`
- Actions:
  - specify parse envelope, span guarantees, and parser role policy.
- Local commands:
  - `cd rust && cargo check --all-targets`
- Blocking gates:
  - `G1`, `G7`

### T3.2 Add Contract Tests for Edge Semantics
- File targets:
  - `rust/crates/talkbank-parser-tests/tests/parser_equivalence_words.rs`
  - `rust/crates/talkbank-parser-tests/tests/generated.rs`
  - `rust/crates/talkbank-parser-tests/src/snapshot.rs`
- Actions:
  - lock behavior for language markers, user forms, durations, CA delimiters.
- Local commands:
  - `cargo nextest run -p talkbank-parser-tests --test parser_equivalence_words`
  - `cargo nextest run -p talkbank-parser-tests --test generated`
- Blocking gates:
  - `G4`, `G5`, `G7`

### T3.3 ~~Dual Owned/Borrowed Model~~ (Completed — removed)

The borrowed model layer was removed in favour of direct owned type construction
with `SmolStr` inline storage and `Arc<str>` interning.

## Phase 4: Diagnostics and Error Taxonomy

### T4.1 Normalize Error Code Catalog
- File targets:
  - `rust/crates/talkbank-errors/src/types.rs`
  - `docs/errors/error-catalog.md`
  - `docs/errors/diagnostic-schema.md`
- Actions:
  - unify code naming and severity semantics.
- Local commands:
  - `cd rust && cargo test -p talkbank-errors`
- Blocking gates:
  - `G1`, `G7`

### T4.2 Span Quality Regression Tests
- File targets:
  - `rust/tests/full_line_context_test.rs`
  - `rust/crates/talkbank-parser-tests/tests/property_tests_modules/error_messages.rs`
- Actions:
  - add assertions on location precision and context rendering.
- Local commands:
  - targeted test invocations for span-sensitive suites.
- Blocking gates:
  - `G7`

## Phase 5: Spec Tools Determinism

### T5.1 Refactor Pipeline Stages
- File targets:
  - `spec/tools/src/bootstrap/`
  - `spec/tools/src/output/`
  - `spec/tools/src/spec/`
  - `docs/architecture/spec-tool-pipeline.md`
- Actions:
  - separate ingest/analyze/emit/verify clearly.
- Local commands:
  - `cd spec/tools && cargo check --all-targets && cargo test`
- Blocking gates:
  - `G2`, `G6`, `G7`

### T5.2 Deterministic Generation Proof
- File targets:
  - `Makefile`
  - `scripts/verify-determinism.sh`
- Actions:
  - run two consecutive generation passes and diff outputs.
- Local commands:
  - `make generated-check`
- Blocking gates:
  - `G6`, `G7`

## Phase 6: Documentation IA Migration

### T6.1 Finalize Audience Portals
- File targets:
  - `docs/index.md`
  - `docs/user/`
  - `docs/integrator/`
  - `docs/contributor/`
  - `docs/architecture/`
  - `docs/audits/`
- Actions:
  - consolidate and redirect duplicate pages into canonical portal structure.
- Local commands:
  - link checker + docs lint (add scripts if absent).
- Blocking gates:
  - `G7`

### T6.2 Deprecate or Archive Stale Docs
- File targets:
  - legacy docs under `docs/` (example: standalone audit files)
  - archive pointers in `docs/audits/README.md`
- Actions:
  - prevent orphaned content and conflicting guidance.
- Local commands:
  - `rg -n "deprecated|superseded|owner|last reviewed" docs`
- Blocking gates:
  - `G7`

## Phase 7: Integrator Hardening (batchalign-class)

### T7.1 Publish Integrator Contract
- File targets:
  - `docs/integrator/api-contract.md`
  - `docs/integrator/migration-policy.md`
  - `rust/schema/` (if schema updates required)
- Actions:
  - define output stability, error schema guarantees, and migration rules.
- Local commands:
  - `cd rust && cargo test -p talkbank-json`
- Blocking gates:
  - `G1`, `G7`

### T7.2 Add Integration Fixtures and Smoke Tests
- File targets:
  - `rust/crates/talkbank-parser-tests/tests/` (new integration fixtures)
  - `docs/integrator/examples/`
- Actions:
  - ensure parse-read-modify-write workflows are covered.
- Local commands:
  - targeted integration test command(s).
- Blocking gates:
  - `G4`, `G5`, `G7`

## Phase 8: Pre-Release Hardening

### T8.1 Full Verification Run
- File targets:
  - none (execution step)
- Actions:
  - run complete verification before release candidate cut.
- Local commands:
  - `make verify`
  - `make generated-check`
- Blocking gates:
  - `G1`, `G2`, `G3`, `G4`, `G5`, `G6`, `G7`

### T8.2 Release Dry Run
- File targets:
  - `docs/contributor/release-checklist.md`
  - `docs/contributor/incident-playbook.md`
- Actions:
  - execute dry run and document rollback path.
- Local commands:
  - release script dry-run command.
- Blocking gates:
  - `G7`

## Tracking Template
Use this table in PR descriptions for in-flight phase execution.

| Task ID | Owner | Branch | Status | Evidence Link | Gate Result |
|---|---|---|---|---|---|
| T0.1 |  |  |  |  |  |
| T0.2 |  |  |  |  |  |
| ... |  |  |  |  |  |

## Non-Negotiable Rules During Execution
- No manual edits to generated artifacts without updating generators.
- No parser behavior changes without contract tests.
- No documentation migration without portal index updates.
- No release candidate when any blocking gate is red.
