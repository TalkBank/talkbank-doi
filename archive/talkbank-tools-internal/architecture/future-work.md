# Future Work Backlog (Re-audited 2026-02-18)

## Prioritization Framework
- P0: Close correctness/reliability gaps that can break trust.
- P1: Reduce operational drag and integration friction.
- P2: Strategic improvements after hardening.

## P0: Immediate — ALL DONE

1. CI parity with `make verify`: G4-G9 gates + reference corpus roundtrip job added (`ci.yml`, commit `ed54497b`).
2. Panic-based runtime control flow: audited `batchalign-core` and `talkbank-clan/combo.rs`; all flagged panics are inside `#[test]` blocks. No runtime changes needed.
3. `spec/tools` warning debt: removed unused imports and dead `write_chat_example` fn from `corpus_to_specs.rs` (commit `cc69f716`).
4. Validation-test generation: removed `generated_validation_tests_body.rs` placeholder and `mod validation_tests` include; documented that coverage lives in roundtrip corpus + error_corpus fixtures; `gen_validation_tests` binary retained for future use once spec layer classifications are audited (commit `5792a0e1`).
5. Root license files: `LICENSE-MIT` and `LICENSE-APACHE` added to repo root (commit `cc69f716`).

## P1: Near-Term — ALL DONE

1. Docs map: `docs/DOCS_MAP.md` created — canonical index and lifecycle policy for 767 markdown files (commit `8c5c1aef`).
2. CI gate summary: `ci-report` aggregator job added to `ci.yml` — writes gate table to `$GITHUB_STEP_SUMMARY`, fails on any red gate (commit `8c5c1aef`).
3. Integration contract: `docs/integrator/diagnostic-schema.md` published — stable JSON output shapes for all `chatter` surfaces with stability tiers and known issues (commit `fef020a0`).
4. Regression dashboard: `scripts/metrics-snapshot.sh` + `scripts/fetch-metrics-trend.sh` added; `metrics` CI job uploads `metrics.json` artifact (90-day retention) per run; dashboard docs at `docs/metrics/README.md` (this commit).

## P2: Medium-Term
1. Parser and validator performance profiling on large corpora with tracked budgets.
2. Incremental parsing ergonomics for LSP/editor workflows.
3. Extended conformance automation against broader external CHAT datasets.
4. Release automation hardening (tag, changelog, artifact, rollback playbook).

## Operationalization
Each backlog item should include:
1. Owner
2. Acceptance test or explicit verification command
3. Target milestone
4. Status (`planned`, `in_progress`, `done`, `deferred`)
