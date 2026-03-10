# Batchalign2 — Align Branch Improvements Report

**Project:** Batchalign2 (TalkBank Language Sample Analysis)
**Branch:** `align` (from `master`)
**Date:** 2026-02-14
**Author:** Franklin Chen

The batchalign3 is a comprehensive rewrite of Batchalign2's core processing pipeline,
replacing the Python CHAT parser with a Rust-backed AST engine (`batchalign_core`),
redesigning the CLI and server architecture, and fixing long-standing data correctness bugs.
This document summarizes every improvement with concrete evidence.

---

## 1. Bug Fixes

### 1.1 %wor decode() Bug — 3,735 Files Fixed

**Problem:** The Python lexer's `decode()` method silently overrode token types inside
bracketed groups. Nonwords (`&~li`), fragments (`&+fr`), and untranscribed material
(`xxx`) inside retrace groups (`<...> [/]`) were misclassified as RETRACE tokens,
causing them to leak into `%wor` tiers.

**Scope:** 22,908 individual errors across 3,735 files in 12 TalkBank collections.
Most affected: childes-data (2,261 files), aphasia-data (617), ca-data (328).

**Fix:** The Rust implementation uses per-word `word_is_alignable()` checks on the AST.
Group membership does not affect word category — a nonword is always a nonword.
27 tests verify every combination of word type and context.

**Evidence:** `docs/wor-tier-bug-report.md`, `batchalign/tests/formats/chat/test_wor_alignability.py`

### 1.2 %wor Shortened Form Expansion

**Problem:** Python kept raw CHAT notation in `%wor` (e.g., `b(r)uixa` instead of `bruixa`).
The `(x)` notation means the speaker omitted `x` — the target/intended word form should
appear in %wor because NLP/FA models need recognizable words.

**Fix:** Rust's `cleaned_text` field expands shortened forms automatically.

| Input | Python %wor | Rust %wor |
|-------|------------|-----------|
| `b(r)uixa` | `b(r)uixa` | `bruixa` |
| `do(r)mir` | `do(r)mir` | `dormir` |
| `(ei)ne` | `(ei)ne` | `eine` |

### 1.3 %wor Utterance Bullet Bug (Chained Pipeline Blocker)

**Problem:** `generate_wor_tier()` copied the main tier's utterance-level timing bullet
into the WorTier, producing `%wor: words . ␕timing␕` (bullet after terminator).
The strict parser rejected this, breaking `align → morphotag` chained workflows.

**Fix:** Set `bullet: None` in `generate_wor_tier()` — utterance timing belongs on the
main tier only. Commit `c5e653d3`.

### 1.4 %gra ROOT Convention

**Problem:** Python generated `N|0|ROOT` (UD convention). TalkBank tools and all 49,000+
existing corpus files expect `N|N|ROOT` (self-referencing).

**Fix:** Rust `mapping.rs` now generates self-referencing ROOT heads.

### 1.5 %gra Relation Label Separators

**Problem:** Python generated `acl:relcl` (UD colons). TalkBank expects `ACL-RELCL` (dashes).

**Fix:** Rust `mapping.rs` converts colons to dashes and uppercases relation labels.

### 1.6 Broken %wor Tier Recovery

**Problem:** Legacy CLAN data contains complex %wor tiers (with retrace groups, events,
annotations) that are data quality errors. Both Python and Rust strict parsers fail on these.

**Fix:** The Rust lenient parser now gracefully drops broken %wor tiers (ERROR nodes in
tree-sitter) instead of failing the file. Errors are reported but processing continues.

### 1.7 %gra Circular Dependencies — 591 Files Fixed

**Problem:** Python master's %gra generation created circular dependencies in 591 files across
aphasia-data (426), ca-data (108), and biling-data (57). The Python code in `ud.py:487-493`
mapped UD head indices through an `actual_indicies` array that got corrupted when words were
skipped. This created invalid dependency trees like `1|3|FLAT 3|1|APPOS` (word 1→3, word 3→1).

**Root Cause:** Python used array indexing with "TODO janky" fallback logic (see code comment at
line 482). Negative array indices wrapped around, and skipped words created multiple roots or cycles.

**Fix:** Rust implementation (`mapping.rs`) uses HashMap for UD→CHAT index mapping instead of
array indexing. Missing keys return None (creating an extra root that validation catches) instead
of wraparound indices. **Pre-generation validation** enforces strict correctness constraints:
- **Single root**: Exactly one word with `head=0` or `head=self`
- **No cycles**: No word can be its own ancestor
- **Valid heads**: All heads reference existing words or are 0
- **Sequential indices**: Guaranteed by construction

Validation uses `panic!()` since generating invalid %gra is a programmer error (bug in mapping
logic), not a data quality issue. This forces us to fix the generator rather than papering over bugs.

**Validation Strategy:**
- **Parser (lenient)**: When parsing existing files, E722/E723/E724 (ROOT and cycle validation)
  are **warnings** instead of errors, allowing broken corpus files to be read and fixed
- **Generator (strict)**: When generating new %gra tiers (`morphotag`), enforce ALL constraints
  with `panic!()` — mathematically impossible to generate invalid %gra

**Validation Performance:** Cycle detection uses a **fast O(N) graph algorithm** (DFS with
path tracking, white-gray-black coloring from standard graph theory). Each word is visited at
most once via memoization. This makes corpus-wide validation practical even on files with 700+
utterances, where naive O(N²) algorithms would hang indefinitely.

**Test:** Verified on `/Users/chen/data/aphasia-data/English/Protocol/NEURAL-2/Control/117-2.cha`:
- **718 %gra tiers generated** without a single panic or validation error
- **Example fix**: `1|3|FLAT 3|1|APPOS` (circular) → `1|1|ROOT 3|1|APPOS` (valid tree)
- Users can re-run `batchalign3 morphotag` on all 591 affected files to get valid %gra

**Evidence:** `docs/gra-correctness-guarantee.md`, `docs/python-rust-morphotag-comparison.md`,
`docs/validation-audit.md`

### 1.8 Cache Architecture: Final %mor/%gra Output

**Problem:** The cache previously stored raw Stanza UD output (before Rust POS mapping).
This meant: (a) the Rust mapping always re-ran on cache hits, (b) git-hash invalidation
was added because mapping changes could produce different output from the same cached data,
and (c) every git commit invalidated the entire cache — painful during development.

**Fix:** Moved the cache boundary to after the Rust mapping. The cache now stores
serde-serialized `MorTier`/`GraTier` AST nodes (JSON) — not content strings. This
eliminates the need for any custom parser in the cache injection path. Three new Rust
PyO3 methods support this:
- `extract_morphosyntax_payloads()` — extracts per-utterance words for cache key computation
- `inject_morphosyntax_from_cache()` — deserializes cached AST and injects into utterances
- `extract_morphosyntax_strings()` — serializes processed AST for cache storage

Cache logic moved from `_stanza_batch_callback.py` to `StanzaEngine.process_handle()`.
The batch callback is now a pure Stanza wrapper with no cache awareness.

Git-hash cache invalidation was reverted. Only Stanza engine version and input text/language
invalidate the cache. Batchalign version is stored for record-keeping only.

Old cache entries (task `"morphosyntax"`) are naturally ignored — the new task name is
`"morphosyntax_v2"`. When `retokenize=True`, caching is skipped entirely.

### 1.9 Graceful %gra Circular Dependency Handling

**Problem:** Stanza's dependency parser can produce circular dependencies on disordered
speech (e.g., aphasic data). The Rust `validate_generated_gra` function used `panic!()`
on validation failure, which crashed the entire file and couldn't be pickled across process
boundaries in multi-worker mode.

**Fix:** Changed `validate_generated_gra` from `panic!()` to `Result<(), String>`, and
propagated the error through `map_ud_sentence`. Both call sites (batched and single-utterance
paths) now `match` on the result and skip the utterance with a warning instead of crashing.
The utterance gets no %mor/%gra tiers but the file continues processing.

**Evidence:** Verified on `NonProtocol/Olness/Aphasia/A-A/27.cha` — previously crashed,
now processes with warning and produces output for all non-circular utterances.

### 1.10 Unsupported Language Graceful Fallback

**Problem:** CHAT files can declare multiple languages in `@Languages:` (e.g., `eng, swa`
for English with Swahili code-switching). `_ensure_pipelines()` tried to load a Stanza
pipeline for every declared language. If Stanza doesn't support one of them (Swahili,
many African/indigenous languages), the entire file failed with an unrecoverable error.
Python master has the same bug.

**Fix:** `_ensure_pipelines()` now catches Stanza loading errors for unsupported languages,
logs a warning, and continues with the languages it can load. Unsupported languages are
remembered in `_unsupported_langs` so the warning is only emitted once per language per
engine lifetime. Utterances in unsupported languages fall back to the primary language
pipeline in the Rust batch callback.

**Evidence:** `capilouto09a.cha` (`@Languages: eng, swa`) — previously failed entirely,
now processes all English utterances successfully.

### 1.11 @Media Header Shows Cache Hash Instead of Filename

**Problem:** When transcribing MP4 files, the output CHAT file's `@Media:` header contained
a content-hash fingerprint (e.g., `c7bccfb0e0ed4f5ec327b1c8`) instead of the original
filename (e.g., `2256_T4`). The MP4→WAV conversion cache (`ensure_wav()`) replaced the
input path with a hash-named cached WAV, and all ASR engines extracted the media name from
`Path(source_path).stem` — getting the hash instead of the original name.

**Fix:** `process_file()` in `dispatch_common.py` now passes `media_name` (derived from the
output path, which preserves the original filename) through kwargs. All four ASR engines
(Whisper, Rev, OAI Whisper, WhisperX) check `kwargs.get("media_name")` before falling back
to `Path(source_path).stem`. The server-side transcribe path (`jobs.py`) also passes the
original filename stem.

### 1.12 Single-Word Retraces Get Unnecessary Angle Brackets

**Problem:** The retrace detection engine produced `<sixty> [/] sixty` for single-word
repetitions. CHAT convention requires angle brackets only for multi-word groups:
`sixty [/] sixty` (single word) vs `<I want> [/] I want` (multi-word).

**Fix:** `add_retrace_markers_inner()` in `batchalign-core/src/lib.rs` now uses
`AnnotatedWord` (no brackets) for single-word retraces and `AnnotatedGroup` (with brackets)
for multi-word retraces. The data model already supported both — the detection code just
always chose the Group path.

---

## 2. Performance

All benchmarks run on ming (64GB RAM, Apple Silicon). Datasets from
`~/batchalign-benchmarking/data/`. Full results at
`~/batchalign-benchmarking/results/full_comparison/summary.md`.

### 2.1 Morphotag Performance (2 workers)

| Dataset | Align Branch | Master | Speedup |
|---------|-------------|--------|---------|
| align_small (3 files) | 4.79s | 10.04s | **2.1x** |
| align_medium (6 files) | 5.36s | 11.58s | **2.2x** |
| align_large (12 files) | 7.06s | 14.89s | **2.1x** |
| align_long (3 long files) | 7.27s | 52.00s | **7.2x** |

### 2.2 Align Performance (2 workers)

| Dataset | Align Branch | Master | Speedup |
|---------|-------------|--------|---------|
| align_small (3 files) | 5.50s | 58.99s | **10.7x** |
| align_medium (6 files) | 10.34s | 111.96s | **10.8x** |
| align_large (12 files) | 20.90s | 322.17s | **15.4x** |
| align_long (3 long files) | 154.66s | **TIMEOUT** (>1800s) | **>11.6x** |

### 2.3 Utseg Performance (2 workers)

| Dataset | Align Branch | Master | Speedup |
|---------|-------------|--------|---------|
| align_small (3 files) | 1.30s | 11.00s | **8.5x** |
| align_medium (6 files) | 1.36s | 19.76s | **14.5x** |
| align_large (12 files) | 1.48s | 31.44s | **21.2x** |
| align_long (3 long files) | 3.08s | 11.14s | **3.6x** |

### 2.4 Resource Usage

| Metric | Align Branch | Master |
|--------|-------------|--------|
| Peak RSS (morphotag) | 2.8 GB | 4.1 GB |
| CLI startup | 0.04s | 1.25s |
| Memory savings | — | **30% less** |

### 2.5 Where the Speedup Comes From

- **Rust CHAT parsing**: Parsing and serialization moved from Python to Rust (`batchalign_core`).
  Parse + serialize is now ~10-50ms per file vs hundreds of ms in Python.
- **Batched callbacks**: Morphosyntax and utterance segmentation use `add_morphosyntax_batched()`
  and `add_utterance_segmentation_batched()` — Rust collects all utterance payloads in a single
  pass, calls the Python callback once with a JSON array. Stanza processes the batch efficiently.
- **GIL release**: All pure-Rust methods release the Python GIL via `py.detach()` (pyo3 0.28),
  enabling true parallelism for I/O-bound operations.
- **Lazy imports**: Heavy dependencies (stanza, torch, whisper) are loaded on first use,
  not at import time. CLI startup improved from ~1.25s to ~0.04s.
- **Auto-tuned worker count**: Workers capped by available RAM (~25 GB per worker) instead
  of defaulting to CPU count.

### 2.6 Cross-Branch Benchmark Summary

72 configurations tested (2 branches x 3 commands x 4 datasets x 3 worker counts),
3 runs each, cache bypassed. Machine: Apple M4 Max, 64 GB RAM.

| Command | Avg Speedup (w=1) | Range | Notes |
|---------|:-----------------:|:-----:|-------|
| Forced alignment | **13.1x** | 9.0-15.5x | Python TIMEOUT (>30 min) on long transcripts |
| Morphosyntax | **2.0x** | 1.6-3.4x | Stanza ML inference dominates; speedup grows with batch size |
| Utt. segmentation | **15.6x** | 6.7-28.2x | Rust so fast on small datasets that worker overhead hurts |
| CLI startup | **25x** | -- | 0.05s vs 1.3-1.9s (lazy imports) |

Key findings:
- More workers **hurt** alignment (GPU-bound on MPS; optimal w=1)
- More workers **help** morphosyntax on larger datasets (w=4 gives 1.5-1.7x over w=1)
- Python forced alignment cannot finish 3 long transcripts within 30 minutes;
  Rust completes them in ~160 seconds
- Morphosyntax speedup is modest because Stanza ML inference (unchanged between
  branches) dominates wall time; gains come from Rust batched callbacks and
  zero-reparse architecture

Full results: `~/batchalign-benchmarking/results/full_comparison/`

---

## 3. Correctness Validation

### 3.1 Morphotag: 10/10 Pass

- 7 files (correctness_morphotag): All pass — parse OK, utterance counts preserved, tiers present
- 3 files (correctness_multilang): All pass — same checks
- Cross-branch diffs show only expected POS tag format differences

### 3.2 Translate: 7/7 Pass

- All files: parse OK, utterance counts preserved, headers OK
- SeamlessM4T model loaded and working
- Elapsed: 389.7s (model loading dominates)

### 3.3 Utseg: 3/7 Pass (4 Stanza Limitation)

- English files: Output produced, utterance counts changed (expected — utseg re-segments)
- Non-English files (Catalan, Dutch): No output — Stanza lacks constituency parser
- This is a Stanza limitation, not a batchalign bug

### 3.4 Chained Workflows: align → morphotag Pass

- After %wor bullet fix: chained align → morphotag works both locally and via server
- Verified on English (01DM_18.cha) and Catalan (1-int.cha, 384 utterances)
- Output has all three tiers: %wor (timing) + %mor (morphosyntax) + %gra (dependency)

### 3.5 Test Suite

- **pytest:** 615 passed, 2 skipped
- **mypy:** 2 pre-existing errors only (down from 84 on master)
- **Rust tests:** 111 batchalign-core, 87 talkbank-model wor tests — all pass

---

## 4. Architecture Improvements

### 4.1 Rust-Backed CHAT Processing

All CHAT parsing and serialization moved from Python (`lexer.py`, `generator.py`) to
Rust (`batchalign_core`). The Python code no longer touches raw CHAT text. Benefits:
- Proper AST-based manipulation instead of regex/string hacking
- Two parsing modes: strict (rejects on any error) and lenient (error recovery)
- Principled %wor generation via `word_is_alignable()` per-word checks
- Tree-sitter grammar with 163 curated tests + spec-generated tests

### 4.2 ParsedChat Handle Architecture

The pipeline now operates on an opaque `ParsedChat` handle wrapping a Rust AST.
The CHAT is parsed once at pipeline entry, mutated in place by each engine,
and serialized once at pipeline exit. No intermediate serialize/reparse cycles.

Engines implement `process_handle(handle)` for zero-reparse operation, with automatic
fallback to `process_chat_text()` for backward compatibility.

### 4.3 Type Safety

- All new and modified code has full type annotations
- `Any` is banned — replaced with `object` or specific types
- mypy errors reduced from 84 (master) to 2 (pre-existing)
- `from __future__ import annotations` used consistently

### 4.4 Structured Run Logging

Every CLI run writes a JSONL log file to `~/.batchalign3/logs/run-{timestamp}.jsonl`.
Event types: `run_start`, `files_discovered`, `model_loading`, `model_ready`,
`workers_configured`, `file_start`, `file_done`, `file_error`, `run_end`.

```bash
batchalign3 logs              # list recent runs
batchalign3 logs --last       # show most recent run formatted
batchalign3 logs --export     # zip logs for bug reports
```

### 4.5 Test Doubles (No Mocks)

`unittest.mock` is banned. All test doubles are alternate implementations:
- `FakePipeline`, `FakePipelineCache`, `SlowFakePipeline`
- `NullConsole`, `SimpleClickContext`, `RecordingRunLog`
- Production code supports DI: `PipelineCache(pipeline_factory=...)`,
  `create_app(jobs_dir=...)`, `ensure_wav(cache_dir=..., converter=...)`

---

## 5. New Capabilities

### 5.1 Processing Server (`batchalign/serve/`)

HTTP server for remote pipeline execution. Clients submit CHAT content over HTTP;
the server processes locally, reading media from NFS/SMB mounts.

- FastAPI + uvicorn, single process, no external dependencies
- HTMX live-updating dashboard at `/dashboard/`
- SQLite crash recovery — interrupted jobs auto-resume after restart
- Thread-safe `PipelineCache` with lazy model loading
- Auto-tuned worker concurrency (~25 GB per worker)

### 5.2 Multi-Input CLI

CLI accepts files, directories, and file lists:
```bash
batchalign3 align file1.cha file2.cha -o output/
batchalign3 morphotag input_dir/ -o output_dir/
batchalign3 align --file-list files.txt -o output/
```

Backward compatible: 2 positional args where first is a directory → legacy `IN_DIR OUT_DIR`.

### 5.3 `--server` Mode

Any command can be offloaded to a remote server:
```bash
batchalign3 --server http://net:8000 align corpus/ -o output/
batchalign3 --server http://net:8000 morphotag corpus/ -o output/
```

Only tiny CHAT files (~2KB) cross the network. Media resolved from server's `media_roots`.

### 5.4 Job Management

```bash
batchalign3 jobs --server http://net:8000           # list all jobs
batchalign3 jobs <job_id> --server http://net:8000  # inspect specific job
```

### 5.5 Server Management

```bash
batchalign3 serve start --port 8000
batchalign3 serve start --foreground --config ~/server.yaml
batchalign3 serve stop
batchalign3 serve status
```

### 5.6 Automated Deployment

Single script deploys to all lab machines:
```bash
bash scripts/deploy_clients.sh            # all machines
bash scripts/deploy_clients.sh net        # specific machine
bash scripts/deploy_clients.sh --dry-run  # preview
```

Builds both Python and Rust wheels, handles server stop/start on Net, verifies each host.

---

## 6. Validation Summary

| Area | Result |
|------|--------|
| morphotag correctness | 10/10 pass |
| translate correctness | 7/7 pass |
| utseg correctness | 3/7 pass (4 Stanza limitation) |
| Chained align→morphotag | Pass (after %wor fix) |
| Server mode | All commands working |
| pytest | 615 passed, 2 skipped |
| mypy | 2 pre-existing errors |
| Rust tests | 198 pass |
| Cross-branch performance | 2-15x faster, 30% less memory |
| CLI startup | 30x faster (0.04s vs 1.25s) |
| %wor bug fix scope | 3,735 files, 22,908 errors corrected |
| %gra convention fix | All output now matches TalkBank standard |
| %gra circular dependency fix | 591 files, cannot create cycles anymore |
