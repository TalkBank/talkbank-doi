# Async Concurrency Assessment

**Date:** 2026-02-25
**Status:** Assessment complete; one issue addressed (see update below)
**Scope:** Whether `tokio` async is the right concurrency model for the `rust-next` server and worker pool, or whether a thread-based model would be more appropriate.

> **Update (2026-03):** The server has migrated from `rusqlite` + `spawn_blocking` to `sqlx` (native async SQLite). The `spawn_blocking` cost analysis in this document no longer applies — all DB operations are now natively async.

> **Update (2026-02-25):** The one genuinely incorrect pattern identified by this assessment — `tokio::sync::Mutex<WorkerHandle>` held for 10–300 seconds during dispatch — has been fixed. The worker pool now uses a channel-based checkout model: `tokio::sync::Semaphore` tracks idle workers, `CheckedOutWorker` is an RAII guard that owns the `WorkerHandle` during dispatch, and `std::sync::Mutex<VecDeque>` holds idle workers (held ~microseconds). No `tokio::sync::Mutex` remains in the worker pool. The `Notify`-based signaling was also replaced by the semaphore (which inherently tracks exact permit counts, eliminating the missed-notification workaround). `worker_count()`, `worker_keys()`, and `worker_summary()` are now sync methods.

## Summary

The `rust-next` workspace uses `tokio` async throughout: HTTP handlers, job runner tasks, worker pool management, worker IPC, and database persistence. This document examines whether async is justified by the actual workload characteristics.

**Conclusion:** Async is the right choice for the system as a whole, justified primarily by the HTTP/SSE/WebSocket layer. The worker dispatch layer is blocking-in-disguise but doesn't cause practical problems at current scale. Switching to threads would not improve throughput — the bottleneck is the worker protocol (one request in-flight per worker), not the Rust concurrency model.

## Workload Analysis

### HTTP Layer (genuinely async)

Handlers in `routes/jobs.rs`, `routes/health.rs`, etc. follow this pattern:

1. Accept request
2. Acquire `JobStore` mutex (<5ms)
3. Spawn a background runner task via `tokio::spawn()`
4. Return immediately

This is the textbook async use case. With threads, you'd need a thread per connection or a manual thread pool. axum/tokio provides this with zero boilerplate.

The health endpoint acquires 6 brief mutex locks sequentially, each held for <1ms. Multiple concurrent health checks interleave cleanly.

### SSE / WebSocket (requires async)

`GET /jobs/{job_id}/stream` holds a connection open, filters a `BroadcastStream` by job ID, and emits events over seconds to minutes. WebSocket upgrade (`ws.rs`) is similar. These are long-lived connections with no CPU work — pure async territory. With threads, each open stream would consume an OS thread, scaling poorly under many simultaneous dashboard clients.

### Per-Job Runner Task (blocking wrapped in async)

Each job spawns a runner task (`runner.rs`) that:

1. Acquires a semaphore permit (job-level concurrency control)
2. Spawns N file tasks via `JoinSet` (N = auto-tuned worker count, 1–8)
3. Each file task calls `pool.dispatch()`, which:
   - Checks out an owned `WorkerHandle` via semaphore + idle queue pop (~microseconds)
   - Writes a JSON request to worker stdin
   - **Awaits the response from worker stdout (10–300 seconds)** — no mutex held
   - RAII guard returns the worker to the pool on drop

The `await` on worker stdout is the critical path. The tokio task is suspended but not doing useful work — it's waiting for a Python process to finish. This is fundamentally blocking I/O (pipe read) wrapped in async syntax. However, unlike the original `Mutex<WorkerHandle>` design, no `tokio::sync::Mutex` is held during this wait — the `CheckedOutWorker` RAII guard owns the handle directly.

### Worker Checkout (semaphore-based)

`checkout()` in `pool.rs` uses `tokio::sync::Semaphore` (one permit per idle worker):

```rust
// Acquire permit (async wait if all workers busy)
let permit = group.available.acquire().await?;
permit.forget(); // permits managed manually
// Pop owned handle from idle queue (~microseconds)
let handle = group.idle.lock().unwrap().pop_front().unwrap();
// Wrap in RAII guard — returns worker to pool on drop
Ok(CheckedOutWorker { handle: Some(handle), group })
```

This replaces the previous `Notify`-based pattern. `Semaphore` tracks exact permit counts (1:1 with idle workers), eliminating the missed-notification problem that required a 5-second timeout workaround with `Notify`.

### SQLite Persistence (spawn_blocking bridge)

All `rusqlite` calls go through `spawn_blocking`:

```rust
tokio::task::spawn_blocking(move || {
    let conn = conn.blocking_lock();
    // synchronous DB work
}).await
```

This adds ~0.5ms overhead per call. With threads, you'd call rusqlite directly. The current pattern is correct but is pure overhead introduced by the async runtime.

## What Async Costs

### 1. Colored functions

Most functions in the call chain are `async`. The `JobStore` still uses `tokio::sync::Mutex` (appropriate — brief holds in async handlers). The worker pool now uses `std::sync::Mutex` for all its internal state, making `worker_count()`, `worker_keys()`, and `worker_summary()` sync. This reduced async infection: health and status endpoints no longer need `.await` for pool queries.

### 2. ~~Long mutex holds on WorkerHandle~~ (RESOLVED)

~~`tokio::sync::Mutex` documentation explicitly warns against holding across long `.await` points. The `dispatch()` method acquires `worker.lock().await` and holds it for the entire worker response (10–300 seconds).~~

**Fixed:** The worker pool now uses a channel-based checkout model. Workers are owned values in a `VecDeque`, checked out via semaphore permit acquisition, and returned via an RAII `CheckedOutWorker` guard on drop. No mutex is held during dispatch. `std::sync::Mutex` is used only for the idle queue (`push_back`/`pop_front`, ~microseconds).

### 3. spawn_blocking for SQLite

Every database operation crosses the async/blocking boundary. Eight `spawn_blocking` call sites in `db.rs`. With a threaded runtime, these would be direct function calls.

### 4. ~~Notify vs Condvar~~ (RESOLVED)

~~`tokio::sync::Notify` lacks the mutex-condvar contract that prevents missed notifications. The 5-second timeout in `acquire_worker` is a workaround for this gap.~~

**Fixed:** Replaced with `tokio::sync::Semaphore`, which tracks exact permit counts (1:1 with idle workers). No timeout workaround needed — `acquire()` waits precisely until a worker is returned to the pool.

## What Async Provides

### 1. HTTP connection multiplexing (essential)

axum/hyper/tokio multiplex thousands of HTTP connections onto a small thread pool. This is non-negotiable for a server that accepts concurrent job submissions, status polls, SSE streams, and WebSocket connections simultaneously.

### 2. Long-lived streaming connections (essential)

SSE and WebSocket connections may be held open for minutes. With threads, each open connection consumes an OS thread (~8KB stack minimum). With async, each is a ~1KB task. At 50 concurrent dashboard viewers, this is the difference between 50 threads and 50 lightweight futures.

### 3. Structured concurrency (convenient)

`JoinSet` for per-file fanout, `CancellationToken` for job cancellation, `tokio::select!` for the health check loop — these patterns compose cleanly. Thread equivalents exist (`crossbeam::scope`, `Arc<AtomicBool>`) but require more manual coordination.

### 4. Ecosystem alignment (practical)

axum, reqwest, tower, tonic — the Rust HTTP ecosystem is async-first. Fighting this by running a sync server (e.g., `actix-web` in sync mode, or raw hyper with manual threading) would mean either abandoning the ecosystem or maintaining async/sync bridges at every library boundary.

## The Throughput Bottleneck

Whether async or threaded, throughput is bounded by:

> **N workers × 1 request per worker at a time**

A Python worker has one stdin and one stdout. It can process exactly one file at a time. The Rust concurrency model doesn't change this. The only ways to increase throughput are:

1. **More workers** — already implemented via `max_workers_per_key` (default 8)
2. **Faster Python processing** — not a Rust concern
3. **Multiplexed worker protocol** — multiple in-flight requests per worker process, which would require a fundamental redesign of `batchalign/worker.py` (request IDs, concurrent dispatch, response routing)

Option 3 is the only one that would change the architectural calculus. If workers could handle multiple concurrent requests, async would become genuinely beneficial for the dispatch layer (true concurrent I/O over multiplexed pipes). Until then, async vs threads is irrelevant to throughput.

## Alternative: Hybrid Model (Superseded)

The hybrid approach described below is no longer needed. The channel-based checkout refactor achieves the same goal (no long mutex holds) while staying within the async model:

- `tokio::sync::Semaphore` replaces `Condvar` for worker availability signaling
- `CheckedOutWorker` RAII guard replaces `std::sync::Mutex` long holds
- `std::sync::Mutex` is used only for the idle queue (~microsecond holds)

No `spawn_blocking` boundary is needed — the dispatch `.await` on worker stdout is async pipe I/O, and no mutex is held during the wait.

## Recommendation

**Uniform async, with the one identified issue now fixed.** The model is justified by:

1. The HTTP/SSE/WebSocket layer requires async regardless.
2. The worker dispatch layer is blocking-in-disguise (waiting on pipe I/O) but no longer holds any `tokio::sync::Mutex` during the wait — the `CheckedOutWorker` RAII guard owns the handle directly.
3. The Rust ecosystem (axum, reqwest, tower) is async-first; fighting it would create more problems than it solves.
4. `worker_count()`, `worker_keys()`, and `worker_summary()` are now sync (read `AtomicUsize` + brief `std::sync::Mutex` holds), eliminating unnecessary async infection in health/status callers.

### If revisiting in the future

The trigger for reconsidering would be:

- **Multiplexed worker protocol** — if workers gain the ability to handle multiple concurrent requests, the dispatch layer would benefit from true async I/O over the multiplexed pipe. At that point, the current architecture would become genuinely async rather than blocking-in-disguise.
- **Scale to hundreds of concurrent jobs** — if the server needs to handle 100+ concurrent jobs with 8 workers each, the 800 tokio tasks blocked on pipe reads might cause scheduler pressure. Profile before optimizing.
