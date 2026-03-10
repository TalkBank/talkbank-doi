# Dioxus Dashboard Default Pipeline and React Deprecation Track

## Date

February 24, 2026

## Status

Accepted implementation track (execution in progress).

## Decision

1. Treat Dioxus as the default dashboard artifact pipeline for batchalign3.
2. Keep React dashboard in maintenance-only mode during the deprecation window.
3. Remove React build/deploy path only after cutover gates and stability windows are met.

## Default Pipeline Definition

Primary artifact build/deploy path:

1. Build: `scripts/build_dioxus_dashboard.sh`
2. Smoke gate: `scripts/run_dioxus_dashboard_smoke.sh`
3. Serve contract: `$BATCHALIGN_DASHBOARD_DIR` or `~/.batchalign3/dashboard`

Pipeline hardening now includes deterministic release flags (`--debug-symbols false` via `BATCHALIGN_DX_DEBUG_SYMBOLS=false` default).

## React Deprecation Milestones

1. Milestone R1: Maintenance mode
   - No feature work, only critical fixes.
   - Keep rollback-ready artifact available.
2. Milestone R2: One stable release after Dioxus default cutover
   - Track incidents and rollback frequency.
   - Keep rollback path tested.
3. Milestone R3: Two stable releases without cutover-critical incidents
   - Remove React build/deploy CI paths.
   - Archive React dashboard docs to legacy section.

## Exit Criteria

1. Dioxus cutover gate fully complete, including manual UAT sign-off.
2. No P0/P1 dashboard incidents attributable to Dioxus for two stable releases.
3. Rollback drill exercised at least once during the deprecation window.
