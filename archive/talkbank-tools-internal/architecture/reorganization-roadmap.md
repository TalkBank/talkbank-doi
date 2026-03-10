# Reorganization Roadmap (No Backward Compatibility Constraints)

## Strategy
Perform a staged but aggressive reorganization with hard quality gates between phases.
Because there are no compatibility constraints, prioritize correctness and maintainability over incrementalism.

## Phase 0: Stabilize Baseline (Blocker Phase)
### Goals
- Restore green checks for root workspace and `spec/tools`.
- Resolve known drift points.

### Required Outcomes
- Fix `rust/src/bin/test-corpus-by-corpus/tree_sitter_mode.rs` location type mismatch.
- Fix `spec/tools/src/bootstrap/analyzer.rs` node type drift (`CA_ANNOTATION` vs current generated constants).
- Establish a reproducible baseline check script.

### Exit Criteria
- `cargo check --all-targets` passes.
- `cd spec/tools && cargo check --all-targets` passes.

## Phase 1: Governance and CI Foundation
### Goals
- Install root governance and CI controls.

### Actions
- Add root OSS governance docs and templates.
- Add CI workflows for compile/test/lint/generation drift/docs checks.
- Enable branch protections and required checks.

### Exit Criteria
- Merges are policy-gated by CI.

## Phase 2: Architecture and Boundary Enforcement
### Goals
- Make subsystem ownership and dependency direction explicit.

### Actions
- Add ownership mapping and architecture docs.
- Refactor any cross-boundary leakage.
- Mark unstable/internal modules.

### Exit Criteria
- Dependency policy is testable and documented.

## Phase 3: Grammar and Symbol Registry Reform
### Goals
- Eliminate manual token drift risk.

### Actions
- Create symbol registry source.
- Generate symbol sets for grammar/parser/spec tools.
- Refactor grammar to use generated sets.

### Exit Criteria
- Registry-driven token governance in production.

## Phase 4: Spec/Generator Determinism
### Goals
- Make generation pipeline reliable and drift-proof.

### Actions
- Refactor `spec/tools` into clear stages.
- Add deterministic generation and zero-diff checks.
- Strengthen spec linting and metadata validation.

### Exit Criteria
- Full generate pass is deterministic and CI-gated.

## Phase 5: Documentation IA Migration
### Goals
- Replace sprawl with canonical audience-based docs.

### Actions
- Build new docs taxonomy and index.
- Move audits to dedicated location.
- Archive or merge stale/duplicate pages.

### Exit Criteria
- Canonical entrypoints exist and are current.

## Phase 6: Integrator Contract Hardening
### Goals
- Publish and test stable downstream surfaces.

### Actions
- Finalize parser/diagnostic/schema contracts.
- Add integration examples and tests for batch workflows.
- Document compatibility/deprecation policy.

### Exit Criteria
- Integrator guide and contract tests are complete.

## Phase 7: Pre-Release Hardening
### Goals
- Confirm release readiness.

### Actions
- Full CI pass plus corpus/regression suites.
- Security and supply chain checks.
- Release checklist dry run.

### Exit Criteria
- Project is ready for public launch with maintainable operations.

## Program Management Cadence
- Weekly architecture review.
- Bi-weekly docs and integrator review.
- Monthly quality metrics review.

## Success Metrics
- Build health: sustained green CI.
- Drift: zero uncontrolled generator drift incidents.
- Docs: reduced duplicate pages and improved discoverability.
- Integration: successful reference downstream integration tests.
