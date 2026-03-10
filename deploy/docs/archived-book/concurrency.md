# Concurrency/Cluster/Cache Assessment (2026-02-19)

> **Staleness note:** This assessment predates the Rust server (axum), which
> replaced the Python FastAPI server as the primary production server. The
> Python code paths reviewed here are still used for local fallback dispatch
> but are no longer the primary execution path. See
> [Rust Server Migration](../architecture/rust-server-migration.md).

This is a frank architecture review of the current codebase state, focused on:

- server/job orchestration,
- pool/worker behavior,
- fleet (multi-server) dispatch,
- cache layers (SQLite/Redis),
- feasibility of framework replacement and Rust migration.

## Scope and Evidence

Reviewed directly from current code paths:

- `batchalign/cli/dispatch.py`
- `batchalign/cli/dispatch_local.py`
- `batchalign/cli/dispatch_server.py`
- `batchalign/cli/daemon.py`
- `batchalign/serve/app.py`
- `batchalign/serve/job_store.py`
- `batchalign/serve/job_db.py`
- `batchalign/serve/jobs.py`
- `batchalign/runtime.py`
- `batchalign/pipelines/cache.py`
- `batchalign/pipelines/cache_redis.py`
- `batchalign/pipelines/morphosyntax/engine.py`
- `batchalign/pipelines/morphosyntax/_stanza_batch_callback.py`

## What the System Is Today

The project is no longer "simple local sequential commands." It is now a layered execution system:

1. CLI router:
   - Local direct execution (`dispatch_local`).
   - Local daemon over loopback HTTP (`dispatch_server` paths mode).
   - Remote server dispatch over HTTP.
   - Fleet fan-out across multiple servers.
2. Server runtime:
   - FastAPI + WebSocket.
   - In-memory `JobStore` + SQLite persistence (`jobs.db`) for recovery.
   - Per-job concurrency plus per-file concurrency.
3. Pipeline/cache runtime:
   - Task-specific pipeline engines.
   - Per-utterance SQLite cache.
   - Optional Redis-backed hybrid cache, activated by server config.

So: yes, this is now a custom orchestration platform, not just a CLI wrapper.

## Current Concurrency Model

### Local CLI (`dispatch_local`)

- Command-class routing:
  - CPU-heavy commands -> `ProcessPoolExecutor`.
  - GPU/network-heavy commands -> `ThreadPoolExecutor` with shared pipeline.
- Worker auto-tuning from CPU/RAM heuristics.
- No automatic process-pool retries; unfinished files are marked failed after a pool crash.

### Local daemon path (default when no `--server`)

- `daemon.py` keeps a persistent localhost server process.
- CLI sends source/output paths (`paths_mode`) so model memory is reused across runs.
- Startup is serialized with `filelock` to avoid daemon race conditions.

### Remote server / fleet path

- `dispatch_server.py` submits jobs to one or more HTTP servers.
- Multi-server fan-out:
  - health-check each server,
  - weight by `workers_available`,
  - split files and poll jobs in parallel,
  - write results incrementally as files complete.

### Server internals (`JobStore`)

- One daemon thread per submitted job.
- Global `threading.Semaphore` caps concurrent jobs.
- Inside each job:
  - CPU-bound commands can route to `ProcessPoolExecutor`.
  - Otherwise thread-level file parallelism or sequential fallback.
- Explicit memory gate blocks/aborts work when available RAM falls below threshold. The gate is skipped when the pool has idle workers for the job's `(command, lang)` — those workers are already loaded and reusable without new allocation.
- Write-through status persistence to SQLite (`JobDB`) on every transition.
- Startup recovery:
  - mark in-flight jobs interrupted,
  - load from DB,
  - reset resumable file states,
  - auto-resume queued interrupted jobs.

## Cache Model

### Active

- `CacheManager` (SQLite WAL):
  - shared process singleton,
  - global lock around DB calls,
  - per-utterance key/value entries with engine-version invalidation.
- `HybridCacheManager` + `RedisCacheManager`:
  - wired through `CacheManager.default()` and `configure_default_cache()`,
  - activated when `server.yaml` sets `redis_url`,
  - process workers initialize the same backend in `_server_worker_init`,
  - automatic fallback to local SQLite if Redis is unavailable.

## Good Engineering Decisions Already Present

- Clear command-type routing (CPU vs GPU/network) in both CLI and server.
- Practical recovery model (`jobs.db` + resume).
- Memory-pressure gate to prevent obvious OOM spirals.
- Pipeline warmup on server startup to reduce first-call model load risk.
- File-conflict detection preventing duplicate in-flight processing by same client.
- Better than average test coverage for job store, dispatch, cache, and recovery flows.

## Key Risks and Complexity Debt

### 1. Job finalization invariant was a real bug (now fixed)

Earlier on 2026-02-19, server processing loops could `break` early (memory-gate
trip, broken process pool) and still allow job-level `completed`.

Fix now in code:
- `JobStore._run_job_inner` enforces terminal file states before final status.
- Any unfinished file is converted to a system `error`.
- Non-cancelled jobs with forced terminal errors finish as `failed`.
- Regression tests were added for both thread-path and process-path partial-completion cases.

### 2. Crash-policy divergence was present (now fixed)

- Local and server process-pool paths now both use no-auto-retry crash handling.
- Unfinished files are forced into terminal failure states.

### 3. Distributed cache is now wired, but deployment discipline still matters

- Redis cache wiring is now live behind `redis_url`.
- Fallback behavior is intentional: if Redis is unreachable, work continues on local SQLite.
- `/health` now exposes cache backend/connectivity state.

Remaining risk is operational drift (assuming Redis is active when servers are actually in fallback).

### 4. Pipeline-cache race can duplicate expensive model loads

`PipelineCache.get()` releases lock before building pipelines, then double-checks insert. This avoids lock hold during heavy I/O, but can load duplicate heavy pipelines concurrently on first access to same key.

Impact:
- avoidable RAM spikes and startup latency under bursty first-use traffic.

### 5. Security boundary is "trusted network only"

The server has no auth and paths mode allows absolute source/output paths. This is acceptable for localhost/trusted lab LAN, but unsafe for internet exposure.

### 6. Documentation drift is real

User-facing docs still had stale guidance from abandoned orchestration experiments even though runtime is now daemon/server/pool based. This amplifies the feeling that architecture is out of control because docs and code disagree.

## Should You Replace This With a Framework?

Short answer: not immediately, unless your goal is reliability/operability guarantees rather than raw throughput.

Reason:
- Main throughput limits are still ML model/runtime behavior (Stanza/PyTorch/Whisper/etc.), not queue framework choice.
- A workflow framework would add infra and still require custom model lifecycle handling, media mapping, and file-level progress logic.

What to do instead right now:
- keep custom orchestration,
- tighten correctness invariants,
- reduce duplicated dispatch logic,
- standardize Redis rollout validation using `/health` cache fields.

## Rust Migration Reality Check

Your instinct is directionally right, but full Rust replacement is constrained by Python ML dependencies (especially Stanza and model stacks).

Pragmatic near-term Rust shape:

1. Rust control plane (CLI/server/job state/progress/protocols).
2. Python ML worker sidecar process for Stanza/Whisper/other Python-only engines.
3. Stable IPC contract (JSON/msgpack/gRPC) between Rust and Python worker.

This gives:
- stronger concurrency safety in orchestration,
- simpler server/runtime model,
- incremental migration without blocking on full model-porting.

## Recommended Next Steps (Priority Order)

1. Deploy Redis path explicitly:
   - done in code; now deploy by setting `redis_url` and verify `redis_cache_connected=true` on each server.
2. Reduce duplicated orchestration logic:
   - shared executor policy module for local/server routes.
3. Define migration target architecture:
   - Rust orchestrator + Python ML sidecar, not full immediate Rust rewrite.

## Free-Threaded Python (3.14t): New Parallelism Opportunities

Updated 2026-02-19 after deploying 3.14t to Net.

### What Changed

Python 3.14t with `PYTHON_GIL=0` allows true multi-threaded execution in a single
process.  Before this, CPU-bound Python code (Stanza inference) could only
parallelize via `ProcessPoolExecutor`, duplicating the entire model stack per worker
(8 GB each).  Now threads share one model copy (~1 GB total) with genuine parallel
execution.

The codebase already implements conditional dispatch:

- `runtime.py`: `FREE_THREADED` flag detected at import time.
- `PROCESS_COMMANDS`: On 3.14t, only `opensmile` (native C code) uses processes.
  `morphotag`, `utseg`, `coref` all move to `ThreadPoolExecutor`.
- `COMMAND_BASE_MB`: Thread workers budget 2 GB (inference working set) instead of
  8 GB (full model load).

### Opportunities Already Realized

**1. Morphotag throughput: 2.2x at 6 threads.**

Stanza's neural pipeline is empirically thread-safe.  `_stanza_batch_callback.py`
skips `nlp_lock` on free-threaded Python, allowing concurrent `nlp()` calls across
threads.  `TokenizerContext` uses `threading.local()` for thread-safe per-thread
state.

**2. Memory savings: 64 GB → 3 GB on Net (8 morphotag workers).**

ProcessPoolExecutor: 8 × 8 GB = 64 GB.  ThreadPoolExecutor: 1 GB models + 8 × 0.25 GB
inference = 3 GB.  This frees 61 GB for OS caches, concurrent jobs, and align/transcribe.

**3. Simplified server architecture.**

With threads, there's no ProcessPoolExecutor for morphotag at all — no worker init,
no pickle-over-pipe, no fork-time copy-on-write faults.  The server's
`_process_files_parallel` path (ThreadPoolExecutor with shared pipeline) handles
everything except opensmile.

### Opportunities Not Yet Realized

**4. Intra-file Stanza parallelism (sub-batch threading).**

Currently, the Stanza batch callback processes all utterances in one call to `nlp()`.
With free-threaded Python, we could split a single file's utterances into sub-batches
and process them in parallel threads — not just different files, but different
*parts* of the same file.

Example: a 500-utterance file could be split into 4 chunks of 125, each processed
by a separate thread calling `nlp()`.  This would parallelize within-file work
(currently sequential) and also enable genuine progress reporting (see
`docs/progress-reporting.md`).

**Trade-off:** Stanza batching benefits from seeing all sentences together (for
cross-sentence context in some processors).  Need to benchmark whether sub-batching
degrades quality or throughput.  On CPU, the benefit of large batches is small —
Stanza processes tokens sequentially on CPU, so sub-batching likely has no quality
or throughput penalty.

**5. Parallel multi-language Stanza inference.**

Multi-language files (e.g. Mandarin + English) currently process each language group
sequentially.  With threads, we could process language groups in parallel — the
English Stanza pipeline and Chinese Stanza pipeline have no shared state.

This is a Rust-side change: `add_morphosyntax_batched` currently iterates language
groups sequentially.  It could spawn threads (via Rayon or `std::thread`) for each
language group, each calling its own Python callback.  The Python callbacks would
run in parallel on free-threaded Python.

**Impact:** Only matters for multi-language files, which are a minority.  Low
priority.

**6. Parallel engine execution within a pipeline.**

Currently, engines run sequentially: ASR → FA → morphosyntax.  Some engines have
no data dependency: for example, translation and morphosyntax could run in parallel
on the same `ParsedChat` handle (they modify different tiers).

This would require:

- Rust-side: parallel `process_handle` calls with shared `ParsedChat` (needs
  interior mutability or separate tier access).
- Python-side: just threads (free-threaded handles the GIL).

**Impact:** Only matters for multi-engine pipelines.  Low priority, high complexity.

**7. Threaded forced alignment.**

FA engines (Wave2Vec, Whisper FA) call PyTorch inference per utterance group.  On
free-threaded Python, multiple utterance groups could be processed in parallel.
Currently serialized because `add_forced_alignment` iterates groups sequentially
in Rust.

**Trade-off:** PyTorch on MPS/CUDA already serializes GPU kernels, so threading
FA on GPU gives no speedup.  On CPU (e.g. Wave2Vec on Net), threading would help.
Low priority because FA is rarely the bottleneck.

### Cache Thread Safety (Production Incident — Fixed)

The singleton `CacheManager.default()` was created in the main thread but accessed
from `ThreadPoolExecutor` worker threads.  Two bugs surfaced in production:

1. **FD leak** (2026-02-19 AM): Each engine's `process_handle()` created a new
   `CacheManager()` with a new SQLite connection, never closed.  macOS ulimit=256;
   server hit 304 FDs → `Errno 24: Too many open files`.  Fixed: singleton pattern
   (`CacheManager.default()`).

2. **Cross-thread SQLite** (2026-02-19 PM): SQLite default `check_same_thread=True`
   rejected cross-thread connection sharing.  All 126 files in a job failed with
   `SQLite objects created in a thread can only be used in that same thread`.
   Fixed: `check_same_thread=False` + `threading.Lock` on all DB operations.

Both were caught by TDD tests added to `test_cache.py::TestSingletonThreadSafety`
(6 tests).  These tests exercise the exact pattern: create singleton in main thread,
read/write from ThreadPoolExecutor workers.

**Lesson:** Free-threaded Python exposes thread-safety assumptions that were
invisible under the GIL.  Any module-level singleton that touches I/O needs
explicit thread-safety review.  The SQLite `check_same_thread` default is a
particularly subtle trap — it's a Python-level check, not a SQLite limitation.

### GIL Release in Rust (batchalign_core)

All pure-Rust `batchalign_core` methods release the GIL via `py.detach()` (PyO3
0.28).  This means:

- Parsing, serialization, validation, and AST manipulation run without holding the
  GIL — true parallelism even on regular Python (for Rust work).
- Callback-based methods (e.g. `add_morphosyntax_batched`) acquire the GIL only
  during the Python callback invocation, then release it for post-callback Rust work.

On free-threaded Python, this distinction doesn't matter (GIL is always off), but
it means Rust-heavy operations like parse/serialize/validate are already parallel-safe
regardless of Python version.

### Recommendations

1. **Benchmark sub-batch Stanza on CPU** (#4 above).  If no quality/throughput
   regression, implement for both progress reporting and within-file parallelism.
2. **Don't pursue parallel engines** (#6) — high complexity, low return.
3. **Review all singletons for thread safety** after any change to module-level
   state.  The CacheManager incident shows the pattern.
4. **Consider `OMP_NUM_THREADS=2`** in the server startup to prevent PyTorch
   OpenMP oversubscription when running many Python threads.

## Throughput Benchmarks

Empirical worker-sweep data from February 2026, measured with `--override-cache` (no
utterance cache hits).  Server scenarios use a prewarmed server; master CLI includes
cold-start model loading.  Peak RSS measured via psutil process-tree sampling.

### Morphotag Worker Sweep

**Ming (16 cores, 64 GB) -- 200 files**

| Workers | Master f/h | Master RSS | GIL=0 f/h | GIL=0 RSS |
| ------: | ---------: | ---------: | ---------: | --------: |
|       1 |        865 |     2.5 GB |      1,692 |    5.7 GB |
|       2 |      1,081 |     4.1 GB |      3,081 |    6.2 GB |
|       4 |      1,547 |     7.3 GB |      4,898 |    6.5 GB |
|       8 |  **1,689** |    13.6 GB |  **6,107** |    6.4 GB |
|      16 |      1,685 |    23.2 GB |      6,107 |    6.4 GB |

GIL=0 w=8 is **3.6x faster than master with 53% less memory**.  Plateau at w=8.

**Bilbo (28 cores, 256 GB) -- 200 files**

| Workers | GIL=1 f/h | GIL=1 RSS | GIL=0 f/h | GIL=0 RSS |
| ------: | --------: | --------: | --------: | --------: |
|       1 |     1,830 |    5.9 GB |     1,798 |    5.8 GB |
|       4 |     3,568 |   13.7 GB |     2,984 |    5.8 GB |
|       8 | **3,727** |   22.3 GB |     2,394 |    6.0 GB |
|      12 |     3,721 |   22.1 GB |     2,050 |    6.3 GB |
|      28 |       --- |       --- |     1,030 |    7.9 GB |

GIL=0 **degrades catastrophically** on high-core machines.  Apple Accelerate
spawns ~8 internal threads per matrix op; at w=28 that is 224 threads fighting
over 28 cores.  GIL=1 (ProcessPool) wins by 1.25x at optimal workers.

**Threshold rule:** >=24 cores use GIL=1 (ProcessPool), <24 cores use GIL=0
(ThreadPool).  Worker cap at 8 -- morphotag plateaus at w=8, zero gain at w=12+.

### Align (Forced Alignment) Concurrency Model

Align is classified as GPU-heavy, so it uses **ThreadPoolExecutor** with a shared
pipeline -- not ProcessPoolExecutor (which would duplicate ~4 GB of models per
worker).  Threads overlap I/O (audio decode, cache, CHAT parse/write) while MPS
serializes GPU kernels.  Thread count capped at `MAX_GPU_WORKERS = 8`.

Additional server protections:

- **macOS memory haircut:** `available_memory_mb()` applies a 30% discount
  because macOS over-reports compressed pages as available.
- **Memory pressure gate:** blocks up to 2 minutes if RAM drops below 2 GB,
  then fails gracefully instead of crashing.
- **Largest-first file ordering:** prevents straggler problems.
- **ProcessPool crash resilience:** retries remaining files with halved worker
  count + `gc.collect()` between retries.

### Summary vs Master

| Command   | Master best              | Align branch best                | Speedup  | Memory saved |
| --------- | ------------------------ | -------------------------------- | -------- | ------------ |
| Morphotag | 1,689 f/h (w=8, 13.6 GB) | 6,107 f/h (GIL=0 w=8, 6.4 GB) | **3.6x** | **53%**      |
| Align     | 627 f/h (w=4, 5.4 GB)    | 1,187 f/h (GIL=0 w=8, 3.0 GB) | **1.9x** | **44%**      |

Master's memory scales linearly with workers (separate model copies per process).
The batchalign3 server shares models across workers, keeping RSS flat regardless
of worker count.

## Bottom Line

The system is not "out of control," but it has crossed the threshold where custom orchestration now needs product-level rigor. Throughput improvements were real, but complexity has accumulated across multiple layers (daemon, server, fleet, cache, recovery). The right move is to stabilize invariants and simplify architecture boundaries before adding new concurrency features.

Free-threaded Python eliminated the fundamental GIL bottleneck for Stanza-based
commands.  The remaining parallelism opportunities (sub-batch threading, multi-language
parallelism, parallel engines) are incremental optimizations with diminishing returns.
The biggest win — shared model memory — is already deployed.
