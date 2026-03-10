# Dioxus Dashboard Cutover Gate Execution (2026-02-24)

## Status

Partially complete: automated gates are green; manual operator UAT remains open.

## Automated Evidence Run

Command:

`scripts/collect_dioxus_cutover_evidence.sh`

Run ID:

`20260224T181855Z`

Artifact bundle:

`artifacts/dioxus-cutover/20260224T181855Z/`

Key files:

1. `artifacts/dioxus-cutover/20260224T181855Z/summary.txt`
2. `artifacts/dioxus-cutover/20260224T181855Z/status.env` (`RESULT=PASS`)
3. `artifacts/dioxus-cutover/20260224T181855Z/metadata.txt`

Fail-fast verification run (intentional failure):

1. Run ID: `verify-fail-20260224T1819Z`
2. Trigger: invalid `DIOXUS_CLI_BIN`
3. Result: `RESULT=FAIL`, `FAILED_STEP=smoke`, `FAILED_EXIT_CODE=127`

Note: `mdbook build` passes but currently emits known non-blocking parsing
warnings in `reference/per-utterance-language-routing.md`,
`developer/launchd-template.plist`, and `developer/setup-launchd.sh`.

## Automated Evidence Details

1. Dashboard smoke suite validates:
   - Snapshot websocket hydration behavior (including REST-empty list scenario)
   - Fleet discovery from `GET /fleet`
   - Fleet override from `?servers=`
   - Server-scoped actions (`cancel`, `restart`, `delete`)
   - Websocket reconnect and degraded-state banner behavior
   - Polling fallback behavior under sustained websocket outage
   - Keyboard selection flow for job detail access
   - Axe accessibility baseline for serious/critical semantic violations
2. Rust workspace compile gate is green.
3. OpenAPI drift gate is green.
4. mdBook compile gate is green.
5. No dashboard contract changes were required for:
   - `GET /jobs`, `GET /jobs/{id}`
   - job action endpoints
   - websocket `/ws` event envelope
   - static dashboard serving contract (`$BATCHALIGN_DASHBOARD_DIR`, `~/.batchalign3/dashboard`)

## Gate Items Still Open

1. Manual operator UAT sign-off.
2. UAT execution sheet completion record.

Required templates for closure:

1. `decisions/dioxus-dashboard-uat-execution-sheet.md`
2. `decisions/dioxus-dashboard-uat-signoff-template.md`

## Manual UAT Checklist (pending)

1. Confirm list/detail/actions against staging server with live worker load.
2. Confirm per-file table behavior during real long-running jobs.
3. Confirm recovery behavior after server restart during active sessions.
4. Confirm keyboard-only navigation and screen-reader announcements.
5. Record sign-off decision and rollback owner.

## Cutover Recommendation

Do not declare final cutover yet. Continue with Dioxus as default artifact
pipeline candidate, close the two open gate items above, then issue explicit
go/no-go decision.
