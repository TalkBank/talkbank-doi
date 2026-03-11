# Performance: Future Work

Assessment of further optimization opportunities beyond the completed items
(OnceLock word cache, ValidationContext Arc split, Arc\<ChatFile\> in LSP,
thread-local parser pool, SQLite PRAGMAs, release profile, LineIndex,
did\_save skip, per-thread SQLite connections via `CachePool`, pass/fail-only cache, audit
mode parallelization, roundtrip parse-only). See
`performance-optimizations.md` for completed work and
`docs/audits/performance-audit-2026-02-24.md` for the original audit.

---

## Radical Optimizations (Need Careful Analysis)

These are architecture-level changes that could yield significant gains but
carry non-trivial risk and require prototyping before committing.

### Lazy/Incremental AST in LSP

**Problem.** The LSP does full CST→AST conversion even when tree-sitter's
incremental parse only touched one utterance. The partial-update infrastructure
exists (`parse_utterance_cst()` on changed ranges) but unchanged utterances
are still re-walked from CST.

**Radical approach.** Content-hash each utterance's source text span. Maintain
a `HashMap<u64, Arc<Utterance>>` cache keyed by hash. On incremental parse,
only convert changed utterances; reuse cached `Arc<Utterance>` for unchanged
ones.

**Analysis needed:**
- Tree-sitter reports `changed_ranges()` at the byte level, not the utterance
  level. Need to map byte ranges to utterance boundaries (line-start based).
- Cached utterances have absolute spans that must be adjusted after edits above
  them. The `SpanShift` derive exists but hasn't been tested for live shifting
  of cached AST nodes.
- Validation errors reference absolute spans — must shift them too, or
  revalidate affected utterances (defeating the cache benefit).

---

## Not Worth Pursuing

**`ErrorContext::source_text: String → Arc<str>`** (Audit #8): Would share one
allocation across multiple errors from the same source. But most files produce
0–3 errors, `enhance_errors_with_source()` replaces the full source with a
small context snippet before errors are used, the cache stores only pass/fail
(no long-lived error accumulation), and CHAT files average ~100–500 bytes. The
459-site refactor across 80+ files would yield unmeasurable savings.

**Chumsky parser caching** (Audit #9): Symbol sets are already `OnceLock`-cached.
Offset-dependent parsers (`word_parser(offset)`, `word_body_parser(offset)`)
capture offset in closures for span calculation — cannot be stored in statics.
Offset-independent parsers (`category_parser()`, `form_marker_parser()`) have
complex generic types unsuitable for `OnceLock<impl Parser>`. The direct parser
is not on any hot path for batch validation.

**Zero-copy parsing** (`Word<'src>` borrowing from input): Lifetime parameters
would propagate through every model type, trait impl, and downstream consumer.
The input `&str` can't outlive the parse call in the LSP (replaced on every
edit). The SmolStr strategy already avoids most allocations for short words.

**Arena allocation per file** (`bumpalo`): Model types escape the parse scope
— cached in `DashMap<Url, Arc<ChatFile>>` (LSP), serialized to JSON (schema
tests), stored in `Vec<Utterance>` on `ChatFile`. Arena allocation requires
lifetime-parameterized types or `Arc<Arena>` co-ownership that defeats the
single-deallocation benefit.

**mmap for file I/O**: Files are small (50KB–2MB). OS page cache already
handles this. mmap setup + page fault cost is likely higher than
`read_to_string()` for typical CHAT files.

**Parallel validation within a file**: Most files are small (13 lines average
in reference corpus). The crossbeam worker pool already parallelizes across
files. Intra-file parallelism adds contention for the few large files and is
wasted for the 99% that are tiny.

**Async I/O for batch validation**: Workers are CPU-bound (parse + validate),
not I/O-bound. Async adds complexity with no throughput gain — OS readahead
on sequential file access already saturates I/O.

**String interning for repeated values** (`lasso::ThreadedRodeo` for speaker
codes, language codes, POS tags): The candidate strings (`"CHI"`, `"MOT"`,
`"eng"`, `"n"`, `"v"`) are all <=23 bytes, so `SmolStr` already stores them
inline with no heap allocation. Interning would save pointer-comparison
equality checks, but these are not on any hot path. The costs are high:
global interner state (awkward for LSP where files open/close), serde/derive
integration, and interned strings that leak in long-running processes.
The original motivation included cache deserialization of repeated error
strings, but error storage was removed entirely (cache is now pass/fail only).

**Binary cache serialization** (`bincode`/`rkyv` for cached errors): Moot —
the cache no longer stores errors. It stores only pass/fail booleans per file,
so there is nothing to deserialize.

**Per-thread SQLite read connections**: Done — `CachePool` uses
`thread_local::ThreadLocal<CacheConnection>` to give each worker its own
connection. WAL mode handles concurrency. No Mutex anywhere.

---

## Completed Incremental Items

All straightforward improvements from the audit are now done.

### LineMap Cache (Audit #5) — Done

`SourceLocation::calculate_line_column()` now uses a thread-local cache keyed
by `(source_ptr, source_len)`. Validation processes one file at a time per
thread, so the LineMap is built once and reused for all errors in a file
(near-100% hit rate, zero API changes).

### ParseError Allocation Reduction (Audit #8) — Done

- `ErrorVec` uses `SmallVec<[ParseError; 2]>`.
- `ErrorContext::expected` uses `SmallVec<[String; 2]>`.

### LSP Incremental Sync (Audit #11) — Done

Changed `TextDocumentSyncKind::FULL` to `INCREMENTAL`. The `did_change`
handler now applies range-based text patches from the client, falling back to
full replacement when no range is provided.

### Streaming JSON Output (Audit #16) — Done

`--format json` now streams results as JSONL (one JSON object per line,
compact). Each file result is emitted immediately as it arrives. Summary stats
are emitted as the final line with `"type": "summary"`. Audit mode continues
to stream via `StreamingAuditSink` as before.
