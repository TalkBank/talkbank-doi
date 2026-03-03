# 15. Observability and Operations

## Findings

- Logging/tracing exists in several components, but there is no single telemetry schema across repos.
- `batchalign3` has run logs and job state persistence, which is a strong base.
- Ops visibility across fleet + local daemon + server can be made significantly more operator-friendly.

## Recommendations

1. Define a common telemetry contract:
   - correlation IDs
   - job IDs
   - command/task tags
   - duration and resource fields
2. Emit structured logs everywhere (JSON for machines, pretty mode for humans).
3. Add metrics endpoints or exporters for key runtime counters.
4. Create operational SLOs:
   - job success rate
   - median completion time per command
   - failure category trends
5. Build dashboards and alert policies around those SLOs.

## Tools to leverage

- `tracing` ecosystem in Rust (with JSON formatter)
- OpenTelemetry exporters where practical
- Lightweight metrics via Prometheus-compatible endpoints for servers

## Observability checklist

- [ ] Define shared log field schema and correlation strategy
- [ ] Add per-command latency and error counters
- [ ] Add operator dashboard for fleet health and queue pressure
- [ ] Add alerting thresholds for repeated failure modes
- [ ] Add postmortem template and incident review cadence
