# Acceptance Test Matrix and Quality Metrics

## Objective
Define measurable, testable acceptance criteria for the full reorganization program.

## Acceptance Matrix

| Area | Requirement | Verification | Frequency | Blocking |
|---|---|---|---|---|
| Build health | `cargo check --all-targets` passes in `rust/` | CI + local script | every PR | yes |
| Spec tooling | `cargo check --all-targets` passes in `spec/tools` | CI | every PR | yes |
| Grammar sync | generated grammar artifacts are in sync | drift check job | every PR touching grammar | yes |
| Parser equivalence | reference corpus equivalence tests pass | `talkbank-parser-tests` | every PR affecting parser/model | yes |
| Snapshot stability | changed snapshots reviewed and intentional | snapshot CI gate | every PR with snapshots | yes |
| Diagnostics schema | errors conform to schema contract | contract tests | every PR affecting diagnostics | yes |
| Docs entrypoints | audience index pages are valid and linked | docs lint | every PR touching docs | yes |
| Integrator contract | contract fixtures pass for API/JSON | integrator tests | every PR affecting outputs | yes |
| Security baseline | dependency/license/security checks pass | security workflow | nightly + release | yes |

## Parser Acceptance Scenarios
1. Header parsing with spacing, continuation, and unknown header fallback.
2. Word-level suffix combinations: form markers, language markers, POS, category prefixes.
3. CA delimiter/element interactions and cleaned-text behavior.
4. Scoped annotations including durations, overlaps, and retracing symbols.
5. Dependent tier dispatch correctness by prefix and known labels.
6. Serialization roundtrip and semantic equivalence to reference parser output.

## Documentation Acceptance Scenarios
1. A new contributor can find setup, architecture, and contribution guidance in <= 3 clicks.
2. An end user can run validation and interpret errors from one page.
3. An integrator can find API contract and migration notes from one page.
4. Every key page includes owner, status, and last-reviewed metadata.

## Metrics Dashboard (Required)
Track the following over time:
- CI pass rate by workflow.
- Mean time to fix broken main branch.
- Number of drift incidents (generation mismatch).
- Parser equivalence failure count per week.
- Snapshot churn (files changed per PR).
- Documentation staleness count (pages past review window).
- Integrator contract break incidents.

## Quality Thresholds
- CI pass rate: >= 98% rolling 30 days.
- Main-branch broken duration: < 4 hours median.
- Drift incidents: 0 unresolved at any time.
- Parser equivalence: 0 unexpected deltas on protected corpus.
- Docs staleness: <= 5% of canonical pages beyond review SLA.

## Gate Escalation Policy
- First failure: PR blocked until fix.
- Repeated failure class (>=3 in 14 days): create focused remediation sprint.
- Persistent instability (>=4 weeks): freeze feature work in affected subsystem.

## Evidence Artifacts
Store acceptance evidence in:
- CI logs and badges,
- periodic quality reports in `docs/audits/quality/`,
- release checklists with timestamped sign-off.

## Final Readiness Criteria
Reorganization is considered complete only when:
- all blocking gates are green for 2 consecutive weeks,
- all priority workstreams closed,
- no open release-blocking risks,
- documentation portals fully migrated and linked.
