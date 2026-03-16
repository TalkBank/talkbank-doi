# Domain 3 Audit: Architecture & Maintainability

**Status:** Current
**Last updated:** 2026-03-16

## 1. Executive Summary

This document serves as the Domain 3 (Architecture & Maintainability) audit for the `batchalign3` and `talkbank-tools` ecosystems within the broader TalkBank software portfolio. It critically evaluates the structural integrity, abstraction cleanliness, and module coupling of the foundational Rust crates and the Python-Rust interop boundaries (PyO3). 

Our investigation reveals a fundamentally solid but highly coupled architecture. The separation of parsing (`talkbank-parser`), modeling/validation (`talkbank-model`), and coordination (`talkbank-transform`) within `talkbank-tools` demonstrates excellent abstraction logic and concurrency awareness (e.g., thread-local parser pools). However, as we cross the boundary into `batchalign3`, particularly across the `PyO3` interface and the worker subprocess dispatch system, we observe significant coupling, brittle state management, and potential single points of failure in the serialization protocols. 

The core of this report nitpicks these boundaries—specifically the JSON-over-stdio IPC and the opaque `ParsedChat` handle—providing actionable recommendations to decouple the machine-learning pipeline from the core AST, mitigate state drift, and harden the system for long-term maintainability.

---

## 1.1 Reconciliation Update (2026-03-16)

The body below preserves the original audit text. This section records the
current disposition after the final sweep.

- **Fixed in this sweep:**
  - same-key worker bootstrap is now serialized, so burst loads no longer fan
    out into simultaneous heavy Python startup for the same
    `(target, lang, engine_overrides)` bucket
  - the stdio worker boundary is harder to poison: bounded stray stdout noise is
    tolerated, protocol-shaped malformed JSON still hard-fails, and Worker
    Protocol V2 result DTOs now reject non-finite / reversed timing data
  - `ParsedChat` callback mutations and morphosyntax cache injection now use
    clone-on-write staging, so failures do not partially mutate the long-lived
    AST handle
- **Partially addressed / intentionally deferred:**
  - the full replacement of newline-delimited stdio JSON is still deferred to
    the Worker Protocol V2 transport migration rather than being landed as a
    late-sweep rewrite
  - broad PyO3 thinning is still desirable, but the remaining work should
    continue as targeted logic extraction into normal Rust crates instead of a
    wide bridge rewrite under release pressure
  - the deeper `talkbank-tools` validation/cache architecture suggestions below
    were not part of this final release-hardening pass

---

## 2. TalkBank Tools Core Crates Architecture

The `talkbank-tools` repository forms the bedrock of the Rust migration. It divides the problem space into distinct layers, primarily `talkbank-parser`, `talkbank-model`, and `talkbank-transform`.

### 2.1 `talkbank-parser`: Abstraction and Concurrency
*Citations:* `/talkbank-tools/crates/talkbank-parser/src/lib.rs`

The `talkbank-parser` crate serves as a thin but highly critical abstraction layer over the `tree-sitter-talkbank` grammar. The architectural decision to abstract the tree-sitter Concrete Syntax Tree (CST) into the strongly-typed Abstract Syntax Tree (AST) found in `talkbank-model` is correct and robust. 

**Nitpick - Concurrency Model:** 
The implementation uses a thread-local pool for `TreeSitterParser`:
```rust
thread_local! {
    static THREAD_PARSER: RefCell<Option<TreeSitterParser>> = const { RefCell::new(None) };
}
```
While this avoids shared mutable cross-thread state and prevents mutex contention during highly parallel batch parsing, it introduces hidden statefulness. If the tree-sitter parser internal state becomes corrupted or if a specific file triggers a catastrophic backtracking failure, the thread-local instance could theoretically be left in a dirty state, poisoning subsequent parses on the same thread pool worker.
*Recommendation:* Implement a generation counter or explicit `reset()` verification protocol when checking out the parser from the `THREAD_PARSER` pool to guarantee pristine state.

### 2.2 `talkbank-model`: Structural Integrity and Validation
*Citations:* `/talkbank-tools/crates/talkbank-model/src/lib.rs`

The `talkbank-model` crate houses the core business logic, including cross-tier validation and alignment primitives. 

**Nitpick - Validation Coupling:** 
Validation logic is currently intertwined with the AST structures (`validation` module within `talkbank-model`). While this ensures that types are strictly validated, it bloats the core model with rule-specific evaluation criteria that might evolve independently of the format schema itself. For example, specific `Validate` implementations heavily depend on external context (`ValidationContext`). If a new corpus requires a slightly modified validation matrix, extending the core model crate is required, breaking the Open-Closed Principle.
*Recommendation:* Abstract the validation engine into an independent ruleset executor outside of `talkbank-model`, or use a strategy pattern that allows `talkbank-transform` to inject validation strategies dynamically.

### 2.3 `talkbank-transform`: Pipeline Coordination
*Citations:* `/talkbank-tools/crates/talkbank-transform/src/lib.rs`

The `talkbank-transform` crate aggregates `parser` and `model` into streaming entry points (`parse_and_validate_streaming`). It heavily relies on unified caching (`SQLite-based`) to accelerate repeated validations.

**Nitpick - SQLite Caching Coupling:**
The integration of `unified_cache` directly inside the transform pipeline crate is slightly misaligned. Transformation (AST to JSON, AST normalization) and caching (I/O, DB locking) are distinct concerns. The pipeline functions (`pipeline.rs`) are forced to negotiate with `lock_helpers.rs` to handle poison recovery, making the core transformation logic tightly bound to disk state management.

---

## 3. BatchAlign3 Server/Worker Boundaries

The `batchalign3` pipeline orchestrates Python-based ML engines using a Rust control plane. This relies heavily on a custom worker pool implementation and a PyO3 bridge.

### 3.1 The Worker Pool Architecture
*Citations:* `/batchalign3/crates/batchalign-app/src/worker/pool/mod.rs`

The `WorkerPool` logic defines how the Rust server spawns and manages Python worker subprocesses. 
The system recently migrated from a `Arc<tokio::sync::Mutex<WorkerHandle>>` pattern to an owned-value model:
```rust
struct WorkerGroup {
    idle: std::sync::Mutex<VecDeque<WorkerHandle>>,
    available: Semaphore,
    total: AtomicUsize,
}
```

**Architectural Wins:**
This split-concurrency model is excellent. By using a standard `std::sync::Mutex` for instantaneous queue access and a `tokio::sync::Semaphore` for async permit waiting, it prevents the Rust control plane from holding async locks over long durations (e.g., 10-300s dispatch windows), dramatically improving server responsiveness.

**Nitpick - Laziness and Spawn Spikes:**
Workers are spawned lazily on demand. Under massive concurrent burst loads (e.g., a massive corpus directory submitted via CLI), the `total` AtomicUsize will immediately fill up to `max_workers_per_key` (default 8). This causes an immediate concurrent launch of 8 heavy Python ML processes, which can OOM (Out of Memory) a constrained machine or cause extreme CPU thrashing during Stanza/Whisper model loading.
*Recommendation:* Implement a staggered startup delay or a global spawn lock that serializes the *bootstrapping* of heavy Python workers, even if their ultimate execution is parallel.

### 3.2 IPC via JSON-over-Stdio
*Citations:* `/batchalign3/crates/batchalign-app/src/worker/handle.rs`

Communication between the Rust server and Python ML engine relies on newline-delimited JSON over `stdio`.
```rust
enum WorkerRequest<'a> {
    Infer { request: &'a InferRequest },
    ExecuteV2 { request: &'a ExecuteRequestV2 },
    // ...
}
```

**Nitpick - Brittle Protocol Constraints:**
The protocol explicitly treats any `op` mismatch as a protocol violation and crashes the request. While fail-fast is good, JSON over `stdio` in Python is notoriously brittle. If a poorly-behaved ML dependency (e.g., a C-extension within Stanza or PyTorch) writes a warning directly to `sys.stdout` (file descriptor 1) at the C-level, bypassing Python's `logging` module, the Rust `BufReader` will attempt to parse it as a JSON envelope, fail, and poison the worker connection.
*Recommendation:* Use an out-of-band communication channel (e.g., named pipes, Unix domain sockets, or a dedicated loopback TCP socket) for the control protocol, leaving `stdout`/`stderr` strictly for logging and debugging.

---

## 4. The Python-Rust PyO3 Boundary

Perhaps the most critical and complex architectural decision in `batchalign3` is the `PyO3` bridge. 

*Citations:* `/batchalign3/pyo3/src/lib.rs`

### 4.1 State Management: The `ParsedChat` Handle
The architecture utilizes an opaque handle pattern:
> "The Python pipeline passes an opaque `ParsedChat` handle between engines: parse once, mutate in place, serialize once."

The domains are heavily fragmented into `#[pymethods]`:
- `morphosyntax`
- `fa`
- `text`
- `speakers`
- `cleanup`

**Nitpick - In-Place Mutation Safety:**
This design implies that the Python side is orchestrating the mutation of a Rust-owned memory structure across multiple sequential ML passes. 
If an ML engine in Python (e.g., Forced Alignment) throws an exception halfway through injecting temporal data via `ParsedChat.fa_inject(...)`, the Rust AST is left in a partially mutated, invalid state. Because the mutation is in-place rather than functional (immutable transformations), a failed pipeline step destroys the original parsed state, requiring a full re-parse from disk or string to recover.

### 4.2 Over-Coupling at the PyO3 Interface
The `pyo3/src/lib.rs` boundary is exceptionally fat. It handles logic spanning from DP Alignment (`dp_align.rs`) to metadata extraction (`metadata.rs`), to Rev.AI bridging (`revai.rs`). 

**Nitpick - Domain Leakage:**
The PyO3 bridge is no longer just a bridging abstraction; it has become a god-module containing core ML algorithms (Hirschberg DP) and business logic (Rev.AI client wrapping). The `batchalign3/pyo3` crate is effectively the control center for the ML pipeline, rather than a thin FFI layer. 
*Recommendation:* Extract the business logic (DP alignment, specific CHAT metadata extraction rules) back into `talkbank-transform` or a new `talkbank-ml-core` crate. The PyO3 boundary should contain *only* the `#[pyclass]` definitions and FFI conversion traits.

---

## 5. Summary of Actionable Recommendations

### Immediate Priority (High Risk)
1. **Migrate IPC away from `stdio`:** Refactor `WorkerHandle` to establish a Unix Domain Socket or localhost TCP connection for JSON DTO exchange. Continue capturing `stdio` for worker logging, but isolate the command protocol. This prevents native C-library prints from crashing the worker pool.
2. **Transactional AST Mutation:** Update the `ParsedChat` PyO3 methods to use an undo-log or clone-on-write pattern. If a Python script fails during a sequence of `.inject_xxx()` calls, the Rust state should rollback to its pre-mutation snapshot.

### Medium Priority (Maintainability)
3. **Thin the PyO3 Bridge:** Move `dp_align.rs` and `revai.rs` out of the `pyo3` crate and into standard Rust crates. Expose them through standard Rust APIs, which are then thinly wrapped by the PyO3 interface.
4. **Stagger Worker Bootstrapping:** Add a global `tokio::sync::Mutex` to the `try_claim_spawn_slot()` logic that prevents more than one worker *from the same key* from actively executing the python initialization phase simultaneously, smoothing out CPU and Memory load spikes.

### Long Term (Architecture)
5. **Decouple Validation:** Separate the `Validate` trait and validation rules from `talkbank-model`. Inject validation logic via `talkbank-transform` to ensure the core AST definition remains purely structural.
6. **Abstract Cache Drivers:** Remove SQLite-specific lock and poison recovery logic from the main path of `talkbank-transform`, abstracting it behind a `CacheStorage` trait to allow memory-only caches for server environments.

---
*End of Report.*
