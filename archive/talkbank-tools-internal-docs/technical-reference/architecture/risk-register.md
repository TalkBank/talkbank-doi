# Risk Register and Mitigation Strategy (Re-audited 2026-02-18)

## Risk Scoring Model
Scale 1-5:
- Likelihood
- Impact
- Detectability (higher = harder to detect early)

RPN = `Likelihood * Impact * Detectability`.
Release-blocking threshold: `RPN >= 40`.

## Current High-Priority Risks

## R1: CI and Local Gate Mismatch
- Description: PRs can pass CI while missing some behavioral gates required by local `make verify`.
- Evidence: CI does not currently run G4-G9 or reference corpus roundtrip.
- Likelihood: 4
- Impact: 5
- Detectability: 3
- RPN: 60 (release-blocking)
- Mitigation:
  - add `make verify` parity in CI,
  - add required corpus roundtrip status.

## R2: Runtime Panic Footguns
- Description: non-test runtime paths still contain `panic!` control flow in production crates.
- Evidence: `batchalign-core` and `talkbank-clan` source files contain panic paths.
- Likelihood: 3
- Impact: 5
- Detectability: 3
- RPN: 45 (release-blocking)
- Mitigation:
  - replace panic paths with typed errors and call-site handling,
  - add lint/check policy to prevent new runtime panics.

## R3: Documentation Drift/Contradiction
- Description: large markdown surface raises probability of stale contradictory guidance.
- Evidence: 767 markdown files across repo; existing analysis set had stale assumptions.
- Likelihood: 4
- Impact: 4
- Detectability: 3
- RPN: 48 (release-blocking)
- Mitigation:
  - canonical docs index per audience,
  - stale-page metadata and periodic pruning.

## R4: Partial Spec-Test Generation Contract
- Description: validation test generation path is unclear/incomplete.
- Evidence: `generated_validation_tests_body.rs` remains placeholder.
- Likelihood: 3
- Impact: 4
- Detectability: 3
- RPN: 36
- Mitigation:
  - implement generation end-to-end or remove placeholder with explicit rationale.

## R5: Spec Tools Maintenance Drift
- Description: warnings and TODOs in `spec/tools` increase entropy of the generation pipeline.
- Evidence: `cargo check --all-targets` warnings in `tools/src/bin/corpus_to_specs.rs`.
- Likelihood: 3
- Impact: 3
- Detectability: 2
- RPN: 18
- Mitigation:
  - warning cleanup and owner assignment for `spec/tools` binaries.

## Lowered Risks (Compared to Prior Baseline)
1. Missing CI at root: mitigated (CI exists and runs core jobs).
2. Baseline compile instability: mitigated (local `make check` and `make verify` currently pass).
3. Symbol drift without automation: reduced (shared symbol generation and generated-check gate are in place).

## Review Cadence
1. Weekly: update risk scores and owner/status.
2. Per release: verify all `RPN >= 40` items are actively mitigated.
