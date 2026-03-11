# Server Telemetry and Outage Runbook

## Scope

Operational guidance for the Rust server and dashboard surfaces (React web + Tauri desktop) covering:

1. OTLP trace export setup
2. Metric naming/label conventions
3. Sample dashboard panels and alert thresholds
4. Dashboard reconnect/failure triage

## OTLP Trace Export

`batchalign3` supports optional OTLP trace export.

Enable one of:

```bash
export BATCHALIGN_OTLP_ENABLE=1
export BATCHALIGN_OTLP_ENDPOINT=http://otel-collector:4318/v1/traces
```

or standard OpenTelemetry endpoint:

```bash
export OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4318
```

Recommended service identity attributes:

1. `service.name=batchalign3`
2. `service.version=<release-version>`
3. `deployment.environment=<dev|staging|prod>`

## Metric Contract (Naming + Labels)

Metric names reserved for server instrumentation and dashboards:

1. `batchalign_jobs_submitted_total` (counter)
2. `batchalign_jobs_completed_total` (counter)
3. `batchalign_jobs_failed_total` (counter)
4. `batchalign_jobs_cancelled_total` (counter)
5. `batchalign_job_queue_depth` (gauge)
6. `batchalign_job_duration_seconds` (histogram)
7. `batchalign_worker_crashes_total` (counter)
8. `batchalign_worker_restarts_total` (counter)
9. `batchalign_http_requests_total` (counter)
10. `batchalign_http_request_duration_seconds` (histogram)
11. `batchalign_http_rate_limited_total` (counter)

Required labels (keep cardinality bounded):

1. `command` (e.g., `morphotag`, `align`, plugin command id)
2. `status` (terminal states only where applicable)
3. `server` (stable host identifier)
4. `http_route` (templated route, never raw path ids)
5. `http_method`
6. `outcome` (`success`, `error`, `timeout`, `rate_limited`)

Disallowed labels:

1. Raw `job_id`
2. Raw `filename`
3. Full user input paths

## Dashboard Panels (Suggested)

1. Throughput by command:
   - `rate(batchalign_jobs_submitted_total[5m])`
2. Completion/error ratio:
   - `rate(batchalign_jobs_completed_total[15m])` vs `rate(batchalign_jobs_failed_total[15m])`
3. Queue depth by server:
   - `batchalign_job_queue_depth`
4. P95/P99 job duration by command:
   - histogram quantiles over `batchalign_job_duration_seconds`
5. HTTP p95 by route:
   - quantiles over `batchalign_http_request_duration_seconds`
6. Rate-limit pressure:
   - `rate(batchalign_http_rate_limited_total[5m])`
7. Worker crash/restart trends:
   - `rate(batchalign_worker_crashes_total[30m])`

## Alert Thresholds (Initial)

1. `JobFailureRateHigh`:
   - trigger: failed/completed ratio > 5% for 15 minutes
2. `QueueDepthBacklog`:
   - trigger: queue depth > 200 for 10 minutes
3. `ApiLatencyHigh`:
   - trigger: p95 HTTP latency > 5s for 15 minutes on `/jobs` routes
4. `RateLimitExcessive`:
   - trigger: `batchalign_http_rate_limited_total` > 100/min for 10 minutes
5. `WorkerCrashLoop`:
   - trigger: worker crashes > 5 in 15 minutes on one server

## Dashboard Reconnect/Failure Playbook

### Symptoms

1. Banner: "Realtime updates degraded. Dashboard is using polling fallback."
2. WS status indicators show `connecting`, `reconnecting`, or `offline`.

### Checks

1. Verify server health:

```bash
curl -sS http://SERVER:8000/health
```

2. Verify jobs API:

```bash
curl -sS http://SERVER:8000/jobs | head
```

3. Verify websocket endpoint is reachable from operator network.

4. In dashboard, use `Reconnect WS` button and confirm status transition.

### Expected Degraded Behavior

1. Job list/detail still refresh through polling.
2. Actions (`cancel`, `restart`, `delete`) remain usable.
3. WS error summary identifies affected server tabs.

### Escalation

1. If REST fails with WS failure: treat as server availability incident.
2. If only WS fails but polling works: treat as realtime transport incident.
3. Capture server logs + recent deployment changes before rollback.
