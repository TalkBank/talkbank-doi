# Performance Audit: talkbank-chat & talkbank-chatter

**Date:** 2026-02-24
**Scope:** talkbank-chat (core crates), talkbank-chatter (CLI, LSP, VS Code)
**Focus:** 95K-file corpus processing (CLI) and interactive editing (LSP)

## Implementation Status

| # | Finding | Status |
|---|---------|--------|
| 1 | `Word::cleaned_text()` allocations | **Done** — `OnceLock<SmolStr>` cache on `Word`, returns `&str` |
| 2 | Audit mode single-threaded | **Done** — parallel via `std::thread::scope` + crossbeam channel, atomic `report_file_results()` |
| 3 | Missing `[profile.release]` | **Done** — added to talkbank-chatter/Cargo.toml |
| 4 | `ValidationContext::clone()` overhead | **Done** — split into `Arc<SharedValidationData>` + mutable overlay |
| 5 | LineMap rebuild per error | **Deferred** — `calculate_line_column` rarely called in main validation path |
| 6 | SQLite single Mutex | Not started |
| 7 | Missing SQLite PRAGMAs | **Done** — added synchronous/cache_size/busy_timeout/mmap_size |
| 8-9 | ParseError cloning / chumsky rebuild | Not started |
| 10 | `parse_and_validate()` creates parser per call | **Done** — uses thread-local pool via `with_parser()` |
| 11 | Full document sync (LSP) | **Deferred** — CHAT files small enough; incremental sync requires UTF-16 handling |
| 12 | `did_save` redundant validation | **Done** — skips if validation_cache exists |
| 13 | `ChatFile` deep-cloned (LSP) | **Done** — `DashMap<Url, Arc<ChatFile>>`, handlers get `Arc::clone()` |
| 14 | `offset_to_position` O(n) (LSP) | **Done** — LineIndex with O(log n) binary search for batch callers |
| 15 | Roundtrip re-parse runs validation | **Done** — uses `ParseValidateOptions::default()` (parse-only) |
| 16 | JSON accumulate-then-serialize | **Deferred** — ~10 MB for 95K files; changing to JSONL breaks output contract |
| 17 | schemars | Not started |
| 18-28 | LOW items | Not started |

**Bonus fix:** Updated `highlights.scm` — 5 removed MOR node types replaced with current grammar equivalents (pre-existing failure).

## HIGH Impact

### 1. `Word::cleaned_text()` allocates a new String every call
**Repo:** talkbank-chat
**Files:** `crates/talkbank-model/src/model/content/word/types.rs`

`cleaned_text()` calls `compute_cleaned_text()` which allocates a new `String` on every invocation by iterating over `content` and concatenating `Text` and `Shortening` variants. Called 2-4x per word during validation and alignment. For 95K files with millions of words, this is the single largest allocation hotspot.

**Fix:** Cache as `OnceLock<SmolStr>` on `Word`, or compute once at the top of `validate()` and pass to helpers.

### 2. Audit mode is single-threaded
**Repo:** talkbank-chatter
**File:** `crates/talkbank-cli/src/commands/validate_parallel.rs:600-704`

`run_audit_mode()` processes files in a sequential `for` loop while normal validation uses crossbeam worker threads. For 95K files, audit mode is orders of magnitude slower than normal validation.

**Fix:** Wire audit mode into the existing crossbeam worker pool. The `StreamingAuditSink` already uses `Mutex` and is designed for concurrent access.

### 3. No `[profile.release]` in talkbank-chatter
**Repo:** talkbank-chatter
**File:** `Cargo.toml`

Missing LTO, codegen-units=1, strip. Release builds use Rust defaults (lto=false, codegen-units=16, strip=none). A few-line addition gives 10-20% throughput improvement.

**Fix:** Add `[profile.release]` with `lto = "thin"`, `codegen-units = 1`, `strip = "symbols"`.

## MEDIUM Impact

### 4. `ValidationContext::clone()` deep-clones HashSet+Vec per word
**Repo:** talkbank-chat
**File:** `crates/talkbank-model/src/model/content/main_tier.rs:506-547`

`ValidationContext` is a 200+ byte struct containing `HashSet<SpeakerCode>`, two `Vec<LanguageCode>`, and several other fields. Cloned 3+ times per utterance and again for every word. File-level data (participants, languages) is needlessly deep-copied.

**Fix:** Share immutable file-level data via `Arc`, only clone the mutable per-field overlay.

### 5. `LineMap::new()` rebuilt from scratch per error
**Repo:** talkbank-chat
**File:** `crates/talkbank-errors/src/types.rs:226-229`

`SourceLocation::calculate_line_column()` calls `LineMap::new(source)` which does an O(n) pass over the entire source text. If multiple errors are reported for the same file, the LineMap is rebuilt each time.

**Fix:** Build once per file and pass through the validation pipeline, or cache alongside source text.

### 6. SQLite single Mutex serializes all worker threads
**Repo:** talkbank-chat
**File:** `crates/talkbank-transform/src/unified_cache/cache_impl.rs:21-24`

`UnifiedCache` wraps `Connection` in `Arc<Mutex<Connection>>`. Every cache get/set locks the global mutex, serializing 14 worker threads through a single point.

**Fix:** Use per-thread connections or a connection pool. SQLite with WAL supports concurrent readers.

### 7. Missing SQLite performance PRAGMAs
**Repo:** talkbank-chat
**File:** `crates/talkbank-transform/src/unified_cache/cache_impl.rs:50-59`

Missing: `synchronous=NORMAL` (WAL is already crash-safe), `cache_size=-8000` (8MB vs 2MB default), `busy_timeout=5000`, `mmap_io=268435456` (256MB). Combined 2-3x write speedup.

**Fix:** Add PRAGMAs after WAL mode line.

### 8. `ParseError` is expensive to clone
**Repo:** talkbank-chat
**File:** `crates/talkbank-errors/src/types.rs:284-327`

Contains ~5 String fields. Cloned when sending errors to both cache and event channel in the validation runner.

**Fix:** Wrap in `Arc<ParseError>` for sharing, or use `Arc<str>` for message fields.

### 9. Chumsky word parser rebuilt per word
**Repo:** talkbank-chat
**File:** `crates/talkbank-direct-parser/src/word.rs:37-76`

The chumsky combinator chain is heap-allocated closures, reconstructed on every `parse_word_impl()` call. The offset parameter prevents caching.

**Fix:** Build parser once, apply offset as post-parse span adjustment.

### 10. `parse_and_validate()` creates a new TreeSitterParser each call
**Repo:** talkbank-chat
**File:** `crates/talkbank-transform/src/pipeline/parse.rs:191-192, 247-248`

Ignores the existing thread-local pool in `lib.rs`.

**Fix:** Use `with_parser()` to access the thread-local pool.

### 11. Full document sync instead of incremental (LSP)
**Repo:** talkbank-chatter
**File:** `crates/talkbank-lsp/src/backend/capabilities.rs:9`

Advertises `TextDocumentSyncKind::FULL`, sending entire doc on every keystroke. Incremental infrastructure (`IncrementalChatDocument`) already exists.

**Fix:** Change to `TextDocumentSyncKind::INCREMENTAL`.

### 12. `did_save` triggers redundant full validation (LSP)
**Repo:** talkbank-chatter
**File:** `crates/talkbank-lsp/src/backend/documents.rs:88-105`

Re-validates with `old_text: None` even though debounced `did_change` just validated. Forces full parse/validate on every save.

**Fix:** Skip validation on save if document hasn't changed since last validation.

### 13. `ChatFile` deep-cloned on every LSP feature request
**Repo:** talkbank-chatter
**File:** `crates/talkbank-lsp/src/backend/requests.rs:23-27`

Hover, completion, symbols, etc. all clone the full AST from the DashMap.

**Fix:** Use `Arc<ChatFile>` in the DashMap, return `Arc::clone()`.

### 14. `offset_to_position` is O(n) per call, O(n²) for semantic tokens (LSP)
**Repo:** talkbank-chatter
**File:** `crates/talkbank-lsp/src/backend/utils.rs:17-49`

Iterates from start of file to the offset. For semantic tokens with ~5K tokens on large files, this is O(n²) total.

**Fix:** Pre-compute line offset index, use binary search for O(log n) per conversion.

### 15. Roundtrip does double parse+validate
**Repo:** talkbank-chatter
**File:** `crates/talkbank-transform/src/validation_runner/roundtrip.rs:30-103`

Re-validates the re-serialized output. Only parse (not validate) is needed for roundtrip comparison.

**Fix:** Parse-only the re-serialized output, skip validation.

### 16. JSON results accumulate all 95K entries in memory
**Repo:** talkbank-chatter
**File:** `crates/talkbank-cli/src/commands/validate_parallel.rs:244`

`Vec<serde_json::Value>` for `--format json` grows unboundedly.

**Fix:** Stream as JSONL or write incrementally.

### 17. `schemars` compiles for all downstream crates
**Repo:** talkbank-chat
**File:** `Cargo.toml`

Feature-gate `schemars` derive behind a `schema` feature on `talkbank-model` and `talkbank-errors` so parsing-only consumers skip the compile cost.

**Fix:** Add `schema` feature flag, gate `JsonSchema` derives.

## LOW Impact

| # | Finding | Repo |
|---|---------|------|
| 18 | Alignment preview builds strings for all words — truncate early | talkbank-chat |
| 19 | Roundtrip `strip_wor_lines()` allocates full copy — compare line-by-line | talkbank-chat |
| 20 | Error vector cloned for cache + event channel — share via Arc | talkbank-chat |
| 21 | `syn` `extra-traits` feature in proc macros — minor compile time savings | talkbank-chat |
| 22 | Worker reads file into String then copies into Arc — use `read()` + direct Arc | talkbank-chat |
| 23 | Inconsistent file discovery (manual read_dir vs WalkDir) | talkbank-chatter |
| 24 | Redundant error DELETE in `set_validation` transaction | talkbank-chatter |
| 25 | Work queue pre-allocates `bounded(total_files)` channel | talkbank-chatter |
| 26 | LSP semantic tokens lack range support | talkbank-chatter |
| 27 | LSP `all_errors()` clones all errors on every publish | talkbank-chatter |
| 28 | `tokio` `full` features — only needs rt-multi-thread, macros, io-std, time | talkbank-chatter |
