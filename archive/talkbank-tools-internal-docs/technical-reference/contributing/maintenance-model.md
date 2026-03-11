# Maintenance Operating Model

## Objective
Define a sustainable post-reorganization operating model so project quality does not decay
after initial cleanup.

## Operating Principles
- Prevent drift, do not just react to drift.
- Keep source-of-truth ownership explicit.
- Turn repeated incidents into policy and automation.

## Roles and Responsibilities
- Maintainers:
  - architecture decisions,
  - release governance,
  - gate policy ownership.
- Module owners:
  - subsystem health,
  - review quality,
  - docs freshness.
- Integrator liaison:
  - downstream contract stability,
  - migration notes and compatibility communication.

## Maintenance Cadence
- Daily:
  - monitor CI health,
  - triage blocking failures.
- Weekly:
  - review open regressions,
  - verify snapshot drift and generation drift incidents.
- Bi-weekly:
  - docs staleness sweep,
  - integrator feedback review.
- Monthly:
  - quality metrics report,
  - risk register review and reprioritization.
- Quarterly:
  - architecture review and backlog reset.

## Incident Classes
1. Build/CI incident.
2. Parser correctness incident.
3. Diagnostic quality incident.
4. Documentation mismatch incident.
5. Integrator contract incident.

Each incident should produce:
- root cause,
- immediate containment,
- permanent fix,
- test or automation to prevent recurrence.

## Technical Debt Policy
- Debt must be tracked as explicit backlog items, not comments in code only.
- Each debt item requires:
  - impact statement,
  - owner,
  - target milestone,
  - risk if deferred.

## Versioning and Release Rhythm (Pre-1.0)
- Use frequent pre-release tags for visibility.
- Document behavioral changes even before 1.0.
- Maintain an upgrade guide for integrators between minor pre-release versions.

## Documentation Maintenance Rules
- Canonical pages must include owner and last-reviewed date.
- Any code behavior change requires doc-impact check in PR template.
- Stale docs beyond SLA become merge blockers for affected subsystem changes.

## Tooling Maintenance Rules
- Generation tools require determinism tests.
- New scripts must be discoverable from top-level docs.
- Deprecated tools should be archived with replacement guidance.

## Health Signals
Healthy project state is indicated by:
- green required CI checks,
- low regression rate,
- low stale-doc count,
- no unresolved release-blocking risks,
- successful downstream smoke tests.

## End-State
The project operates with predictable quality, low cognitive overhead for contributors,
and reliable contracts for CHAT editors, researchers, and downstream software.
