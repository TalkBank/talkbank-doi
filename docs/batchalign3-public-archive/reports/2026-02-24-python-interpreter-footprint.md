# Python Interpreter Footprint in batchalign3

Date: 2026-02-24

## Executive Summary

- `batchalign3` (Rust CLI/server) does not run ML itself; Python interpreters are worker runtimes.
- Interpreters are pooled and reused, keyed by `(command, lang)`, not spawned per job/file.
- Startup can create temporary and warmup workers; steady-state usually converges to one worker per active `(command, lang)` key per daemon.
- Compared to batchalign2, cold start and idle memory can be higher (especially with warmup), while repeated-job latency is typically better due to persistent workers.

## Direct Answer: How Many Python Interpreters?

## Local Mode

1. Rust binary + remote server (`--server` for non-transcribe commands):
- Local Python worker interpreters: typically `0`.

2. Local daemon path:
- At startup: `+1` temporary probe worker (capability detection), then exits.
- If warmup is enabled (default): prestarts workers for warmup commands intersected with capabilities (default candidate set: `morphotag`, `align`, `transcribe`).
- After startup: one live Python interpreter per worker key `(command, lang)` that has been spawned and not idled out.

3. Local daemon + sidecar fallback (`transcribe`, `transcribe_s`, `avqi` flows):
- Same rules as above, but potentially for two daemons (main + sidecar), each with its own pool.

## Server Mode (`serve start`)

- Same worker-pool model as local daemon mode.
- Interpreters are not per job; they are persistent per `(command, lang)` key in that daemon.

## Confirmed Behavior from Live Check

Using the local release binary and `--test-echo`:

1. Warmup enabled:
- Observed 3 persistent Python workers at idle (`morphotag`, `align`, `transcribe`).

2. Warmup disabled (`server.yaml` with `warmup: false`):
- Observed 0 persistent Python workers at idle.
- Submitting two morphotag jobs produced a single persistent morphotag worker reused across jobs.

## Is This Worse Than Python-First for Memory/Startup?

Short answer: mixed.

1. Startup time:
- Often worse on cold start if warmup is enabled, because workers and models are loaded up front.

2. Idle memory:
- Often worse with warmup on, because warmed workers remain resident.
- Better with warmup off (near-zero worker memory at idle).

3. Repeated job latency:
- Usually better, because workers are persistent and model load is amortized.

4. Peak memory risk:
- Driven mainly by number of concurrent Python workers and model footprints, not by Rust control-plane overhead.
- Rust adds negligible memory compared with ML workers.

## Clarification vs batchalign2

It is not accurate that batchalign2 had only one interpreter in practical multi-file operation.

- batchalign2 control plane was Python.
- Multi-file execution used worker threads or worker processes depending on mode.
- So total runtime often included more than one execution worker even in Python-first architecture.

## Current Architecture Notes (Important)

1. Worker lifecycle:
- Workers are created lazily per `(command, lang)`.
- Dead workers are restarted.
- Idle workers are removed after timeout.

2. Capability detection:
- Uses a temporary probe worker at startup.

3. Warmup:
- Default enabled in config; can be disabled.
- Warmup command list defaults to `morphotag`, `align`, `transcribe` when not explicitly set.

4. `workers_available` in health endpoint:
- This is a job-slot metric (based on concurrency semaphore), not a count of live Python worker processes.

## Practical Guidance

If the priority is minimizing startup/idle memory:

1. Set `warmup: false` in `~/.batchalign3/server.yaml`.
2. Keep explicit conservative `max_concurrent_jobs`.
3. Use sidecar only where command/dependency split requires it.

If the priority is minimizing first-request latency:

1. Keep `warmup: true`.
2. Restrict warmup commands to the specific commands used in that deployment.

## Evidence Pointers (Code)

- Worker pool keying/spawn/reuse: `rust-next/crates/batchalign-worker/src/pool.rs`
- Python worker process spawn command: `rust-next/crates/batchalign-worker/src/handle.rs`
- Server startup capability probe and warmup: `rust-next/crates/batchalign-server/src/lib.rs`
- Default config (`warmup: true`): `rust-next/crates/batchalign-types/src/config.rs`
- Transcribe/avqi local-daemon routing and sidecar fallback: `rust-next/crates/batchalign-cli/src/dispatch.rs`
- Health endpoint and `workers_available` source: `rust-next/crates/batchalign-server/src/routes/health.rs`, `rust-next/crates/batchalign-server/src/store.rs`
- Python worker entry/data plane: `batchalign/worker.py`

