# Dioxus Dashboard UAT Execution Sheet

## Purpose

Runbook-grade checklist for final operator validation before declaring Dioxus
dashboard cutover complete.

## Run Metadata

Record the following before execution:

1. UAT run ID
2. Date and time window
3. Environment (`staging` or `pre-prod`)
4. Server version (`batchalign3 --version`)
5. Dashboard artifact source and commit SHA
6. Pre-UAT evidence bundle run ID and path (`scripts/collect_dioxus_cutover_evidence.sh`)
7. Evidence status file result (`status.env` must be `RESULT=PASS`)
8. Test operator names

## Preconditions

All preconditions must be true before UAT starts:

1. Rust workspace compiles cleanly.
2. Pre-UAT evidence bundle run for this commit is green.
3. Rollback artifact is prepared and verified loadable.
4. Staging dataset and long-running workload fixture are available.

## Execution Cases

Mark each case `PASS`, `FAIL`, or `N/A`, and attach evidence path.

| ID | Area | Scenario | Expected Result | Status | Evidence |
|---|---|---|---|---|---|
| UAT-01 | List | Load dashboard landing list | Jobs render without console/runtime errors |  |  |
| UAT-02 | Detail | Open detail from list selection | Correct job detail appears with server label |  |  |
| UAT-03 | Actions | Cancel running job | Backend state and UI status become `cancelled` |  |  |
| UAT-04 | Actions | Restart terminal job | Backend state and UI status become `queued` |  |  |
| UAT-05 | Actions | Delete terminal job | Job disappears and backend returns deleted |  |  |
| UAT-06 | Files | Observe per-file updates on active job | File rows update in detail view without reload |  |  |
| UAT-07 | WS | Force temporary websocket interruption | Reconnect occurs and status indicators recover |  |  |
| UAT-08 | Polling | Keep websocket unavailable for full session | Polling keeps list/detail/actions usable |  |  |
| UAT-09 | Fleet | Validate `GET /fleet` discovery with 2+ servers | Server tabs appear and route correctly |  |  |
| UAT-10 | Fleet | Validate `?servers=` override mode | Query-defined servers are honored |  |  |
| UAT-11 | Accessibility | Keyboard-only navigation through list/detail/actions | Core workflows succeed without pointer input |  |  |
| UAT-12 | Accessibility | Screen-reader sanity pass on banners/status | Critical status text is announced/readable |  |  |
| UAT-13 | Recovery | Restart server during active dashboard session | UI recovers with clear degraded-state messaging |  |  |
| UAT-14 | Errors | Trigger representative REST failure | Remediation banner explains operator next step |  |  |
| UAT-15 | Rollback | Execute rollback artifact drill once | Rollback procedure succeeds and is reversible |  |  |

## Defect Handling

Classify each defect:

1. `P0`: data loss, unsafe action routing, unusable operator path
2. `P1`: major workflow regression without workaround
3. `P2`: degraded UX with workaround
4. `P3`: cosmetic/non-critical

Gate policy:

1. Any open `P0` or `P1` blocks cutover.
2. `P2` and `P3` require owner, tracking ID, and target fix milestone.

## Evidence Checklist

Collect and store:

1. Terminal transcript for smoke + OpenAPI checks
2. `summary.txt` + `status.env` from the evidence bundle directory
3. Browser screenshots for each failed case
4. Network trace for reconnect/outage scenarios
5. Final defect list with severity and owner
6. Completed sign-off template document

## Exit Condition

UAT is complete only when:

1. All mandatory cases are `PASS`.
2. No open `P0/P1` defects remain.
3. Rollback drill was executed successfully.
4. Sign-off template is completed by required approvers.
