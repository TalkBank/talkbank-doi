# Implementation Workstreams and Deliverables

## Objective
Turn the reorganization strategy into execution-ready workstreams with concrete outputs,
owners, dependencies, and completion evidence.

## Program Structure
Use ten parallel workstreams with explicit interfaces between them. Each workstream has:
- a technical lead,
- a QA owner,
- weekly review artifacts,
- objective completion criteria.

## Workstream A: Repository Topology and Ownership
### Scope
- Finalize top-level directory purpose and boundaries.
- Create CODEOWNERS mapping by subsystem.
- Enforce package boundary direction.

### Deliverables
- `docs/architecture/repo-topology.md`
- `docs/architecture/ownership-map.md`
- CI check for dependency direction violations.

### Exit Evidence
- No unresolved ownership gaps.
- No cross-boundary imports violating policy.

## Workstream B: Grammar and Symbol Governance
### Scope
- Move symbol authority into generated registry.
- Remove manual forbidden/allowed duplication from `grammar.js`.

### Deliverables
- `spec/symbols/symbol_registry.yaml`
- `grammar/src/generated_symbol_sets.js`
- `rust/crates/talkbank-model/src/generated/symbol_sets.rs`
- drift-check script and CI gate.

### Exit Evidence
- Symbol changes only via registry update + generation pass.
- Grammar and parser compile after regeneration with zero manual edits.

## Workstream C: Parser/Model Contract Hardening
### Scope
- Declare canonical parser contract and AST invariants.
- Add contract tests around span stability, annotations, and serialization.

### Deliverables
- `docs/architecture/parser-contract.md`
- contract integration tests in `rust/crates/talkbank-parser-tests/`.

### Exit Evidence
- Contract tests block merges on regression.
- All parser surfaces document failure behavior.

## Workstream D: Diagnostics and Error Taxonomy
### Scope
- Unify diagnostic schema across CLI, LSP, and API outputs.
- Remove ambiguous error text and weak source spans.

### Deliverables
- `docs/errors/error-catalog.md`
- `docs/errors/diagnostic-schema.md`
- diagnostic quality tests (message shape, spans, severity).

### Exit Evidence
- Error code catalog has no undocumented codes.
- Severity policy is enforced by tests.

## Workstream E: Testing and Quality Gates
### Scope
- Implement layered test gates (unit/contract/generated/corpus/property).
- Make generation and snapshot drift explicit and reviewable.

### Deliverables
- CI workflow matrix with required checks.
- `make verify` top-level verification entrypoint.
- flaky test quarantine and remediation policy.

### Exit Evidence
- Green required checks for every PR.
- Zero silent generated artifact drift.

## Workstream F: Spec Tools and Generation Determinism
### Scope
- Refactor `spec/tools` into clear pipeline stages.
- Enforce deterministic ordering and stable output.

### Deliverables
- `docs/architecture/spec-tool-pipeline.md`
- normalized stage interfaces (`ingest`, `analyze`, `emit`, `verify`).

### Exit Evidence
- Two consecutive generation runs produce byte-identical outputs.
- determinism check is CI-blocking.

## Workstream G: Documentation IA and Audience Portals
### Scope
- Reorganize docs into audience portals.
- Migrate scattered audits into discoverable structure.

### Deliverables
- `docs/index.md`
- `docs/user/`, `docs/integrator/`, `docs/contributor/`, `docs/architecture/`, `docs/audits/`.
- staleness metadata in key docs.

### Exit Evidence
- Every audience has one canonical start page.
- No major duplicate pages with conflicting guidance.

## Workstream H: Integrator Experience (batchalign-class)
### Scope
- Define stable integrator contract and examples.
- Provide robust failure-handling and upgrade guides.

### Deliverables
- `docs/integrator/api-contract.md`
- executable integration examples.
- parser-output fixture suite for downstream tests.

### Exit Evidence
- Integrator smoke tests pass in CI.
- documented compatibility policy for pre-1.0 releases.

## Workstream I: Release Governance and OSS Readiness
### Scope
- Prepare contribution and governance workflows for public hosting.
- Add release checklist and incident playbooks.

### Deliverables
- release runbook,
- contributor quickstart,
- security policy hardening.

### Exit Evidence
- Release dry-run complete.
- Maintainer operations docs are complete and reviewed.

## ~~Workstream J: Dual Owned/Borrowed Model Architecture~~ (Completed — removed)

The borrowed model layer was removed. String storage now uses `SmolStr` (inline ≤23 bytes)
and `Arc<str>` interning (high-frequency repeated values) directly in the owned model.

## Dependency Graph (Critical)
1. Workstream B must complete before final grammar/parser stabilization.
2. Workstream F depends on Workstream B outputs.
3. Workstream C and D can run in parallel but must converge before release.
4. Workstream G should start early; final migration waits for stable contracts.
5. Workstream H depends on C, D, and F outputs.
6. Workstream J depends on Workstream C contract baselines and Workstream E test gates.
7. Workstream H should consume Workstream J outputs for high-volume integrator guidance.
8. Workstream I depends on all prior workstreams.

## Cadence and Reporting
- Weekly technical status by workstream.
- Bi-weekly cross-workstream integration review.
- Monthly risk and metric review against acceptance matrix.

## Done Definition for the Program
Program completion requires:
- all workstream exit evidence archived,
- all required CI gates enabled,
- all audience docs linked from `docs/index.md`,
- release dry-run completed with no unresolved blockers.
