# Persistence Spike: `rusqlite` vs `sqlx` (2026-02)

## Date

February 24, 2026

## Status

Decision: stay on `rusqlite` for current server generation; revisit `sqlx` when storage backend requirements expand beyond single-process SQLite.

## Scope

Compare maintainability, correctness, runtime, and migration cost for the current job-store persistence layer in `rust-next/crates/batchalign-server/src/db.rs`.

## Current Baseline (`rusqlite`)

Measured from current code:

1. DB layer size: ~1025 LOC (`db.rs`).
2. Async bridge points: 8 `tokio::task::spawn_blocking` call sites.
3. Concurrency model: single SQLite connection behind `tokio::sync::Mutex`, WAL enabled, explicit transactions for write paths.
4. Proven behavior: crash recovery, TTL pruning, and integration tests already rely on this behavior.

## Option Comparison

### Keep `rusqlite`

Pros:

1. Lowest migration risk; no query-shape rewrite needed.
2. Explicit SQLite control (PRAGMA, WAL, transaction boundaries).
3. Existing recovery semantics stay unchanged.

Cons:

1. Manual SQL string handling and row mapping remain verbose.
2. `spawn_blocking` boundary must be maintained carefully.
3. No compile-time SQL checking from migration metadata.

### Migrate to `sqlx` (SQLite)

Pros:

1. Better compile-time query validation path (with offline metadata).
2. Cleaner async-native call path for most query surfaces.
3. Easier future move to alternate backends if strategy changes.

Cons:

1. High migration cost: rewrite query layer plus test re-baselining.
2. New operational complexity: migration metadata management and CI drift checks.
3. Limited immediate runtime gain for the current single-process SQLite/WAL workload.

## Decision Rationale

1. The current bottlenecks are queueing/scheduling and worker lifecycle, not SQLite driver overhead.
2. Existing `rusqlite` behavior already matches needed durability semantics for this generation.
3. Immediate value from `sqlx` does not justify rewrite risk now.

## Follow-Up Plan

1. Keep `rusqlite` as the production path for this release line.
2. If we add multi-writer/process requirements or cross-DB support, open a dedicated `sqlx` branch spike with one representative query path and offline-check CI.
3. Re-evaluate in the next annual architecture cycle or earlier if persistence requirements materially change.

## 2026-02-26 Update: Fleet Coordination Changes the Picture

Discussion of fleet coordination needs has shifted the persistence outlook. The
current prototype — CLI-as-coordinator with per-server SQLite — is not resilient
enough to deploy confidently, while users struggle with private cache duplication,
manual load balancing, and jobs that exceed single-machine capacity.

The likely direction is **shared Postgres on Net** as the fleet coordination
database (shared job queue, fleet-wide visibility, crash recovery via row-level
state), with **sqlx** as the Rust driver. sqlx supports both SQLite
(dev/testing) and Postgres (production fleet) behind the same query API.

This means the re-evaluation trigger from item 2 above — "multi-writer/process
requirements or cross-DB support" — is now expected to be hit as fleet
coordination matures. The persistence layer should be decoupled from
SQLite-specific assumptions in preparation.

Evaluated and set aside:

- **SeaORM**: ORM abstractions not justified — the data model is flat tables
  (jobs, file statuses, cache entries), not relational entity graphs.
- **Diesel**: heavier migration story, less async-native than sqlx.
- **Redis as job queue**: current Redis config on Net (`allkeys-lru`, no AOF) is
  correct for cache but unsuitable for durable job state. Redis stays as the
  compute cache (L2 `HybridCacheManager`).
- **NATS JetStream**: clean architecture for work queues, but adds a new
  infrastructure category. Worth revisiting if Postgres `SKIP LOCKED` proves
  insufficient.
- **Temporal**: maximum resilience but heavy infrastructure (own Postgres +
  Temporal server). Overkill for ~10-machine lab fleet.
