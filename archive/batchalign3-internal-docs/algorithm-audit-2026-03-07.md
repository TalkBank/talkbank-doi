# Comprehensive Algorithm & Data Structure Audit

**Date:** 2026-03-07
**Scope:** All batchalign3 algorithms and data structures (excluding morphotag),
including the built-in HK/Cantonese engines. Covers Python inference, Rust
server/worker/cache, Rust chat-ops, and the current provider/extension split.

> This page is a dated design-review snapshot from March 7, 2026. It records
> the conclusions of that review pass; current source and tests are
> authoritative for current behavior.

## Executive Summary

This review pass found a generally strong architecture across the areas it
examined: Python handles stateless ML inference, Rust owns CHAT lifecycle and
server orchestration, and the built-in provider layer feeds typed worker
contracts without owning CHAT mutation.

Within the scope reviewed here, no correctness blockers were identified. The
findings below were low-priority optimization or code-health follow-ups, and
most were expected to have negligible runtime impact relative to ML inference.

### Verdict by Area

| Area | Quality | Issues Found |
|------|---------|-------------|
| Python inference modules | Excellent | 0 critical, 2 minor |
| Worker IPC protocol | Excellent | 0 critical, 0 minor |
| Rust worker pool | Excellent | 0 critical, 0 minor |
| Rust server/queue/retry | Excellent | 0 critical, 0 minor |
| Rust cache (SQLite) | Excellent | 0 critical, 1 minor |
| Rust chat-ops (NLP alignment) | Excellent | 0 critical, 2 minor |
| Rust chat-ops (translate/FA) | Excellent | 0 critical, 2 minor |
| HK/Cantonese engines | Very Good | 0 critical, 2 minor |
| DP / sequence alignment | Excellent | 0 critical, 0 minor (recently optimized) |

---

## 1. Python Inference Modules

### 1.1 Architecture Assessment

The reviewed inference modules (`fa.py`, `asr.py`, `morphosyntax.py`,
`utseg.py`, `translate.py`, `coref.py`, `speaker.py`, `opensmile.py`,
`avqi.py`, `benchmark.py`) follow a consistent pattern:

1. Pydantic model for input validation (`*BatchItem`)
2. Pydantic model for output (`*Response`)
3. `batch_infer_*()` function: validate → infer → wrap result
4. Per-item error isolation (one failure doesn't abort the batch)
5. Elapsed time stamped on first result

This consistency is a significant strength — adding a new inference task
requires only copying the pattern and filling in the model-specific logic.

### 1.2 Data Structures

| Structure | Location | Assessment |
|-----------|----------|------------|
| `ASRAudioFile` | `audio.py` | Well-designed lazy/eager container with LRU chunk cache. `dict` + `list` for LRU is fine at cache_limit=16. |
| `FaInferItem` parallel arrays | `fa.py` | `words`, `word_ids`, `word_utterance_indices`, `word_utterance_word_indices` — 4 parallel lists with Pydantic validator ensuring length sync. Struct-of-arrays is correct here (matches Rust wire format). |
| `UdWord` | `morphosyntax.py` | Pydantic model with validators for bogus lemma detection and pad deprel sanitization. Clean. |
| `ChainRef` / `CorefRawAnnotation` | `coref.py` | Sparse representation (only sentences with coref data included). Efficient. |
| `_WorkerState` | `_types.py` | Module-level singleton with typed fields. Appropriate for single-worker-per-process model. |

### 1.3 Algorithms

**Constituency tree walking** (`utseg.py:_parse_tree_indices`): Recursive
traversal looking for S-level phrases under coordination. The `unique_ranges`
deduplication loop (lines 216-222) has an O(n²) set-difference pattern, but
`n` is the number of phrase ranges per sentence — typically < 10. Not a concern.

**AVQI voiced segment extraction** (`avqi.py:_extract_voiced_segments`): Sliding
window (30ms) over audio with power threshold. Calls Praat via parselmouth for
each window, which dominates runtime. The Python loop is fine.

**Audio chunk caching** (`audio.py:ASRAudioFile`): Manual LRU with `dict` +
`list`. At `cache_limit=16`, O(n) `pop(0)` on eviction is negligible. An
`OrderedDict` would be marginally cleaner but not measurably faster.

### 1.4 Minor Findings

**M1. Morphosyntax batch grouping creates intermediate lists.**
`batch_infer_morphosyntax()` (line 203-205) unpacks `by_lang` items into
three separate lists (`indices`, `texts`, `word_lists`). This is idiomatic
and clear. The duplication is negligible — these are references, not copies
of the word data.

**M2. Translation Google rate limiting uses `time.sleep(1.5)`.** This is
intentional (Google Translate rate limit avoidance) and documented. Not an
algorithmic concern.

---

## 2. Worker IPC Protocol

### 2.1 Architecture Assessment

The stdio JSON-lines protocol (`_protocol.py`) is a clean, minimal
request-response loop. Each operation is dispatched by `op` string to a handler.
The protocol is:

- **Synchronous per-worker**: One request at a time per worker process.
  Concurrency comes from multiple workers in the pool, not multiplexing.
- **Self-framing**: One JSON object per line, no length prefix needed.
- **Stateless dispatch**: `_infer()` delegates to `_batch_infer()` (wrapping
  single items), eliminating code duplication.

### 2.2 Data Flow Efficiency

```
Rust server
  → JSON serialize BatchInferRequest
  → write to worker stdin
  → worker reads line, json.loads()
  → Pydantic model_validate(raw_item)  [per item]
  → inference (ML model call)
  → Pydantic model_dump()  [per item]
  → json.dumps() + write to stdout
  → Rust reads line, serde_json::from_str()
```

The double serialization (Pydantic model_validate → model_dump) on the Python
side adds ~1ms per item. Given that inference takes 50-5000ms per item, this
is negligible. The Pydantic validation catches malformed payloads at the
boundary, which is the right tradeoff.

### 2.3 Command Capability Integration

The current release does not use `batchalign.plugins` discovery. Instead,
server-side command validation accepts the built-in command set plus any
additional worker-advertised capabilities that appear at runtime. That keeps
command routing flexible without making entry-point discovery part of the
public release contract.

---

## 3. Rust Worker Pool (`batchalign-worker`)

### 3.1 Concurrency Model — Excellent

The worker pool uses a split concurrency model that eliminates the most common
async footgun:

| Component | Type | Purpose |
|-----------|------|---------|
| `idle` | `std::sync::Mutex<VecDeque>` | Microsecond-held, never across `.await` |
| `available` | `tokio::sync::Semaphore` | Async-aware availability tracking |
| `total` | `AtomicUsize` | Lock-free capacity checking |
| `CheckedOutWorker` | RAII guard | Auto-return on drop (success or panic) |

This is textbook correct. The previous `Arc<tokio::sync::Mutex<WorkerHandle>>`
pattern (holding a tokio mutex for 10-300s during dispatch) has been eliminated.

### 3.2 Pre-scaling

`pre_scale()` uses `compare_exchange` on `AtomicUsize` for concurrent-safe
spawn slot claiming. This prevents over-provisioning when multiple concurrent
callers try to scale the same group. Clean lock-free design.

### 3.3 Assessment

No changes are suggested from this review pass. The pool is well-designed for
its use case
(managing a handful of heavyweight ML worker processes). The data structures
are appropriately simple — `HashMap<(String, String), Arc<WorkerGroup>>` with
`VecDeque<WorkerHandle>` inside. No need for more complex structures at this
scale.

---

## 4. Rust Server (`batchalign-app`)

### 4.1 Queue/Dispatch Architecture

The `QueueBackend` trait + `LocalQueueBackend` + `QueueDispatcher` pattern
cleanly separates queue-state decisions from execution launch:

```
QueueBackend (trait)
  └── LocalQueueBackend (in-process, Notify-based)
        ↓
QueueDispatcher
  └── spawns runner tasks via crate::runner::spawn_job()
```

This is designed for future Postgres-backed fleet coordination (`claim_ready_jobs()`
would become a SQL `SELECT ... FOR UPDATE SKIP LOCKED`). The trait is minimal
and correct.

### 4.2 Retry Policy

`dispatch_batch_infer_with_retry()` implements a clean retry loop with:
- Typed failure taxonomy (`classify_worker_error()` + `is_retryable_worker_failure()`)
- Exponential backoff via `RetryPolicy`
- Terminal failure passthrough (no infinite retry)

This is exactly what you want for transient worker crashes. No issues.

### 4.3 Memory Gate

The idle-worker bypass logic (check pool for reusable workers before checking
system memory) prevents a subtle deadlock: idle workers from a previous job
consume all RAM, but they're the exact workers the new job would reuse. This
is a well-thought-out edge case handler.

---

## 5. Rust Cache (`batchalign-cache`)

### 5.1 Architecture Assessment

SQLite WAL-mode cache with `CacheBackend` trait for future backend swaps.
Three-dimensional key: `content_hash × task × engine_version`. Clean design.

### 5.2 Data Structures

| Structure | Assessment |
|-----------|------------|
| `SqlitePool` (5 connections) | Appropriate for local cache workload |
| `CHUNK_SIZE = 900` | Correct (under SQLite's 999 parameter limit) |
| `HashMap<String, Value>` for batch results | Fine — no ordering requirement |

### 5.3 Minor Finding

**M3. Cross-task key collision.** The SQLite schema uses `key` as the sole
PRIMARY KEY (matching Python's `CacheManager`). Same key with different task
overwrites the previous entry. This is documented in the test
`test_cross_task_isolation()` and matches legacy behavior. If task isolation
becomes important, the PK should be `(key, task)` — but this is a deliberate
compatibility decision, not a bug.

---

## 6. Rust Chat-Ops (NLP Alignment)

*Findings from previous session's completed audit.*

### 6.1 Retokenize / Tokenizer Realign — Excellent

**`WordTokenMapping`** (`retokenize/mapping.rs`): Dense `Vec<SmallVec<[usize; 4]>>`
indexed by word position. O(1) lookup, inline storage for the common 1-2 token
case. Two-stage algorithm: deterministic span-join first, length-aware monotonic
fallback. No DP.

**Character-position realignment** (`tokenizer_realign/mod.rs`): O(n)
parallel character-walk algorithm. Builds per-char owner arrays for both
original and Stanza tokens, walks both in parallel. Clean and efficient.

### 6.2 Minor Findings (from previous session)

**M4. Lemma cleaning has 15+ chained `.replace()` allocations.**
`tokenizer_realign/lemma.rs` applies multiple string replacements sequentially.
Each `.replace()` allocates a new `String`. A single-pass `Replacer` would
eliminate intermediate allocations. Low priority — lemma cleaning runs once
per word, and words are short.

**M5. MWT rule matching allocates `to_lowercase()` per token.**
`tokenizer_realign/mwt_overrides.rs` calls `.to_lowercase()` on each token
during rule matching. Pre-computing lowercase versions or using
case-insensitive comparison would avoid redundant allocations. Low priority
— at 5+ languages, a data-driven approach (table of rules) would be better
than code changes.

---

## 7. Rust Chat-Ops (Translate, FA, Compare)

*Findings from previous session's completed audit.*

### 7.1 Minor Findings

**M6. `translate.rs` applies 13 sequential `.replace()` calls.** Chinese
text normalization chains 13 string replacements. A single-pass approach
(e.g., `aho-corasick`) would reduce allocations from O(13n) to O(n). Low
priority — translation batches are small and the API call dominates.

**M7. `fa/postprocess.rs` monotonicity enforcement is O(w²).** The current
forward-scan with lookahead checks O(w) predecessors for each of O(w) words.
A backward pass would be O(w). Low priority — utterances rarely exceed 50
words, making the quadratic term negligible.

---

## 8. DP / Sequence Alignment

*Recently optimized (this session).*

### 8.1 Current State

All DP is restricted to intrinsically necessary uses:

| Algorithm | Location | Purpose | Avoidable? |
|-----------|----------|---------|------------|
| Hirschberg | `dp_align.rs` | WER evaluation, transcript comparison | No — arbitrary sequence alignment |
| DTW | Python Whisper FA | Cross-attention matrix alignment | No — model-intrinsic |
| CTC forced alignment | `torchaudio.functional.forced_align` | Wave2Vec FA | No — model-intrinsic |
| LCS (Patience) | `similar` crate | %wor mismatch diagnostics | No — diff formatting |

### 8.2 Recent Optimizations

1. **Prefix/suffix stripping** — O(n) preprocessing reduces DP problem size
   by 10-100x for 80-95% accuracy transcripts.
2. **`Alignable` trait** — generic unification of `String` and `char` variants
   eliminated ~200 lines of duplicated code.
3. **DP allowlist test** — CI enforces no new DP callsites outside 3 approved
   locations.

No further low-risk improvements were identified in this review pass without
changing the problem definition.

---

## 9. HK/Cantonese Engines

### 9.1 Architecture Assessment

The current release ships 4 built-in HK-specific providers under
`batchalign/inference/hk/`:

| Provider | Task | Backend | Credentials |
|----------|------|---------|-------------|
| `tencent` | ASR | Tencent Cloud COS + ASR API | `~/.batchalign.ini` |
| `aliyun` | ASR | Aliyun NLS WebSocket | `~/.batchalign.ini` |
| `funaudio` | ASR | FunASR local model | None (auto-download) |
| `wav2vec_canto` | FA | Wave2Vec + pycantonese jyutping | None |

All providers follow the same `load_*()` + `infer_*()` pattern as the rest of
the built-in inference layer, which is a strength — the abstraction is
consistent.

### 9.2 Lazy Import Pattern

Heavy SDK imports are deferred until load/infer time inside the HK modules.
This avoids paying Tencent/Aliyun/FunASR import cost when those engines are not
selected.

### 9.3 Cantonese Text Pipeline

```
Input text
  → `zhconv` zh-HK conversion (simplified → HK traditional)
  → Manual replacement table (31 entries, longest-first)
  → Punctuation stripping (for char-level tokens)
```

The replacement table in `cantonese.rs` is ordered correctly: multi-character
replacements before single-character ones to prevent partial matches.

### 9.4 Data Structures

| Structure | Location | Assessment |
|-----------|----------|------------|
| `REPLACEMENTS` | `cantonese.rs` | 31-entry table compiled into an Aho-Corasick automaton with leftmost-longest matching. |
| `_AliyunRunner._sentences` | `aliyun_asr.py` | List of sentence word-lists. Clean accumulation from WebSocket callbacks. |
| `TencentRecognizer` | `tencent_api.py` | Stateful client with COS + ASR clients. Poll loop with 10s interval and 10min timeout. |
| `FunAudioRecognizer` | `funaudio_common.py` | Lazy model loading (`_get_model()`). Dual-path for paraformer vs SenseVoice. |

### 9.5 Minor Findings

**M8. Historical HK normalization note.**
The pre-fold-in Python implementation used sequential replacement over the old
plugin package. The current release moved Cantonese normalization to
`cantonese.rs`, where a 31-entry Aho-Corasick table now handles the same work.

**M9. Aliyun credential validation is duplicated.** `load_aliyun_asr()` (lines
199-234) manually validates config keys, while `common.py:read_asr_config()`
provides the same validation as a shared helper. The Tencent provider uses
`read_asr_config()` correctly; Aliyun should too. This is a minor code
duplication, not a correctness issue.

---

## 10. Cross-Cutting Patterns

### 10.1 Strengths

1. **Consistent batch_infer pattern.** Every inference module (including the
   HK/Cantonese engines) follows the same structure: validate items → per-item try/except →
   stamp elapsed on first result → return `BatchInferResponse`. This makes
   the system highly extensible.

2. **Clean separation of concerns.** Python never sees CHAT text. Rust never
   loads ML models. The IPC boundary is a well-defined set of Pydantic models.

3. **No unnecessary abstractions.** The codebase avoids over-engineering.
   Worker state is a simple singleton. Audio caching is a manual LRU with 16
   slots. The queue backend trait has exactly 3 methods. Nothing is more
   complex than it needs to be.

4. **Type safety at boundaries.** Pydantic models on the Python side,
   `serde`-derived structs on the Rust side, with the JSON-lines protocol
   as the shared contract. This catches mismatches early.

5. **RAII everywhere.** `CheckedOutWorker` returns workers to the pool on
   drop. `CancellationToken` for job cancellation. Temporary files cleaned
   up in `finally` blocks. No resource leaks.

### 10.2 Extensibility Assessment

| Extension point | How to add | Effort |
|----------------|-----------|--------|
| New inference task | Add `InferTask` variant + `batch_infer_*()` function + dispatch entry | Low |
| New ASR provider | Add a built-in module with `load` + `infer` and wire it into dispatch | Low |
| New FA provider | Same as ASR | Low |
| New cache backend (e.g. Postgres) | Implement `CacheBackend` trait | Medium |
| New queue backend (e.g. Redis) | Implement `QueueBackend` trait | Medium |
| New worker transport (e.g. gRPC) | Requires `WorkerHandle` changes | High |

### 10.3 No Improvements Needed

The following areas were audited and found to need no changes:

- **Worker pool sizing** — `max_workers_per_key = 8` is appropriate for
  heavyweight ML processes. Dynamic sizing would add complexity without benefit.
- **Cache key formula** — SHA-256 of content + lang + engine is collision-safe
  and invalidation-correct.
- **IPC framing** — JSON-lines is simple and adequate. MessagePack or protobuf
  would save ~10% serialization overhead but add dependency complexity.
- **Audio lazy loading** — The eager/lazy fallback with LRU cache handles the
  common case (sequential utterance processing from one file) well.

---

## Summary of All Findings

| ID | Area | Finding | Priority | Impact |
|----|------|---------|----------|--------|
| M1 | morphosyntax.py | Intermediate list creation in batch grouping | None | Idiomatic, references only |
| M2 | translate.py | Google rate limiting sleep | None | Intentional |
| M3 | batchalign-cache | Cross-task key collision by design | None | Legacy compat |
| M4 | tokenizer_realign | 15+ chained `.replace()` for lemma cleaning | Low | ~1% of inference time |
| M5 | mwt_overrides | Per-token `to_lowercase()` allocation | Low | Negligible |
| M6 | translate.rs | 13 sequential `.replace()` for Chinese normalization | Low | API call dominates |
| M7 | fa/postprocess.rs | O(w²) monotonicity enforcement | Low | w < 50 typical |
| M8 | HK normalization | Historical pre-fold-in sequential replacement path | Low | Resolved by Rust fold-in |
| M9 | HK aliyun_asr.py | Credential validation not using shared helper | Low | Code duplication |

This dated review snapshot recorded no correctness blockers in the reviewed
areas and a set of minor observations for follow-up. Treat those observations
as review notes, not as a standing release guarantee for the entire repo.
