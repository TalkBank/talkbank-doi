# Net Auto-Tune Regression — 2026-02-24

## Summary

On February 24, 2026, Net hit another auto-tune memory regression and the
server was killed by jetsam (`OS_REASON_JETSAM`). Later the same day, Net also
hit a full kernel watchdog panic under extreme VM pressure during an `align`
job with UTR fanout (`num_workers: 8`). This is another incident in the long
series of concurrency auto-tune mistakes.

## Python Server Root Cause

File:
`~/.local/share/uv/tools/batchalign-next/lib/python3.14t/site-packages/batchalign/serve/job_store.py`

Function: `_auto_max_concurrent()`

Problematic logic before hotfix:

```python
budget_mb = 4_000 if FREE_THREADED else 12_000
return max(2, min(by_cpu, by_mem))
```

On a large-memory host, this made free-threaded builds dramatically more
aggressive than non-free-threaded builds. On Net (256 GB class machine), this
scaled to roughly 51 slots by memory, which is far above safe load behavior.

## Emergency Hotfix Applied on Net

Date: February 24, 2026

Applied changes:

1. Set per-slot budget to 12 GB for all Python builds.
2. Added hard cap of 8 concurrent slots in `_auto_max_concurrent()`.
3. Added `max_concurrent_jobs: 8` to `~/.batchalign-next/server.yaml` as an
   explicit production override.
4. Restarted `org.talkbank.batchalign-server` and verified `/health`.
   At incident time this service was running as a user LaunchAgent; fleet
   operations have since moved to LaunchDaemon deployment in `/Library/LaunchDaemons`.

Verification after restart included:

- Server healthy on `http://127.0.0.1:8000/health`
- `workers_available: 8`

## Second Failure (Same Day): Watchdog Panic, Not Just Jetsam

After the first hotfix, Net crashed again. `~/crash.log` showed:

- `panic ... watchdog timeout: no checkins from watchdogd`
- `Compressor Info ... 100% of segments limit (BAD) with 72 swapfiles`

This is a full system lockup/reset path, not just a single user-space process
termination.

Correlated job evidence from `/jobs/{id}`:

- command: `align`
- `num_workers: 8`
- file progress included `Utterance Timing Recovery` activity

Interpretation:

1. Job-slot overcommit was one bug class (fixed via `max_concurrent_jobs` cap).
2. Per-job worker auto-tune remained too aggressive for `align` + UTR behavior.
3. 8-way fanout can still push Net into swap-thrash and watchdog panic even
   when only one job is active.

## Rust Rewrite Audit (Ongoing `rust-next`)

Audited file:
`batchalign3/rust-next/crates/batchalign-server/src/store.rs`

Findings:

1. The exact Python 3.14t branch bug (`4_000 if FREE_THREADED`) did not exist.
2. But the same *class* of risk still existed: no hard slot cap.
3. Formula was:

```rust
by_cpu.min(by_mem).max(2)
```

With 12 GB budget on high-memory hosts, this could still auto-select >8
concurrent slots (for example, ~16-17 on a 256 GB class host depending on
`available_memory`).

## Rust Fix Added

Changes made in `store.rs`:

1. Added explicit constants:
   - `AUTO_CONCURRENT_BUDGET_MB = 12_000`
   - `AUTO_CONCURRENT_MAX_SLOTS = 8`
   - `AUTO_CONCURRENT_FALLBACK_SLOTS = 4`
2. Added helper `auto_max_concurrent_from(available_mb, by_cpu)`.
3. Updated selection logic to:

```rust
by_cpu.min(by_mem).min(AUTO_CONCURRENT_MAX_SLOTS).max(2)
```

4. Added tests:
   - `auto_max_concurrent_caps_large_hosts`
   - `auto_max_concurrent_fallback_and_floor`

## Required Follow-Up

1. Keep explicit `max_concurrent_jobs` values in production configs.
2. Set explicit `max_workers_per_job` caps in production (temporary safe value:
   `2` on Net until UTR-aware tuning is merged).
3. Make per-job auto-tune UTR-aware (align with UTR enabled must budget higher
   than plain align).
4. Require table-driven auto-tune tests for 32/64/128/256 GB profiles before
   releases.
5. Treat any "free-threaded memory budget reduction" changes as high-risk and
   block merge until validated on Net-like hardware.
