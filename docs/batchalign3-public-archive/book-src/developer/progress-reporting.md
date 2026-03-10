# Progress Reporting Assessment

Last updated: 2026-02-19

## Current Architecture

Progress flows through three layers, each with different granularity:

### Layer 1: File-Level (CLI / Server Dashboard)

**CLI local dispatch** (`dispatch_local.py`): Rich `Progress` widget with one row per
file.  Each row shows a spinner, status label, filename, and elapsed time.  Status
transitions are coarse: `Queued...` → `Processing...` → `DONE` / `FAIL` / `CRASH`.

```
⠋ Processing...  020100.cha  0:04
⠋ Processing...  020200.cha  0:03
✓ DONE            010100.cha  0:06
```

No per-utterance detail is shown.  A file that takes 45 seconds looks identical to
one that takes 2 seconds until it finishes.

**CLI server dispatch** (`dispatch_server.py`): Even coarser — a single counter line
that increments as files complete:

```
  Processing… (3 / 126)  →  http://server:8000/dashboard/jobs/...
```

The server tracks per-file per-utterance progress internally (`FileStatus.progress_current`,
`progress_total`, `progress_label`), but the CLI client only polls file completion
counts — it never displays utterance-level progress.

**Server dashboard** (HTMX at `/dashboard/`): Shows a table of files with status.
`progress_current` / `progress_total` fields are available via the API and could
render a per-file progress bar, but the current dashboard shows only status text
(`queued`, `processing`, `done`, `error`).

### Layer 2: Engine-Level (Pipeline Callback)

The pipeline loop (`pipeline.py`) passes a `callback(step, total_tasks, current_tasks)`
to each engine, where `step` is the engine index and `total_tasks` is the engine
count.  For a morphotag pipeline with one engine, this is always `(0, 1, ...)` →
`(1, 1, ...)` — useless.

For multi-engine pipelines (e.g. `asr,fa,morphosyntax`), it lets the caller know
which engine is active.  The server uses this to set `progress_label` (e.g.
"Morpho-Syntax", "Forced Alignment").

### Layer 3: Within-Engine (status_hook / progress_fn)

Engines receive `ctx.status_hook` — a `Callable[[int, int], None]` that maps to the
pipeline callback via a lambda that adds the engine step offset.  The Rust entry
points (`add_morphosyntax_batched`, `add_forced_alignment`, `add_translation`,
`add_utterance_segmentation_batched`) accept a `progress_fn` parameter and call it
as utterances complete: `progress_fn(completed, total)`.

**This is the finest-grained progress we have**, and it's the only layer that
actually tracks work within a single file.

## What Users Actually See

| Dispatch Path | Granularity | Visual |
|---------------|-------------|--------|
| CLI local (ProcessPool) | File done/fail | Rich spinner per file |
| CLI local (ThreadPool) | File done/fail | Rich spinner per file |
| CLI → server | File completion count | Single counter line |
| Server dashboard | File status text | HTML table |
| Server API | Per-utterance (available) | JSON fields, not rendered |

**In all CLI paths, users see no progress within a file.**  A 500-utterance morphotag
file shows `Processing...` for 30 seconds, then flips to `DONE`.

## The Batching Problem

### Why Single-File Progress Is Misleading for Stanza

Stanza processes utterances in a single batch per language group.  The
`make_batch_morphosyntax_callback` in `_stanza_batch_callback.py` joins all
utterances with `\n\n`, calls `nlp(combined)` once, and then splits results back.
This means:

1. Rust calls `progress_fn(0, N)` at the start.
2. Rust calls the Python batch callback **once** with all N utterances.
3. Stanza runs its neural pipeline on the entire batch (tokenize, POS, lemma,
   depparse — each over all N utterances at once).
4. The callback returns.
5. Rust calls `progress_fn(N, N)` at the end.

**There is no meaningful intermediate progress.**  The progress jumps from 0% to
100% in one step.  Showing a progress bar that sits at 0% for 25 seconds and then
jumps to 100% is worse than showing no progress bar at all — it makes the user
think the system is stuck.

### Contrast: Forced Alignment

Forced alignment (`add_forced_alignment`) processes utterances one at a time in Rust,
calling the Python FA callback per utterance group.  Here `progress_fn` genuinely
reports incremental progress (e.g. 1/50, 2/50, ..., 50/50).  A per-utterance progress
bar would be meaningful for FA.

### Contrast: Translation

Translation (`add_translation`) also processes utterances one at a time, so
`progress_fn` reports genuine incremental progress.

### Summary

| Engine | Batch Size | progress_fn Useful? |
|--------|-----------|---------------------|
| Morphosyntax (Stanza) | All utterances at once | No — jumps 0→100% |
| Forced Alignment | Per utterance group | Yes |
| Translation | Per utterance | Yes |
| Utterance Segmentation | All at once (batched) | No — same batch problem |
| UTR (Whisper) | Entire audio file | No — binary (running/done) |

### Cache Hits Make It Worse

When all utterances hit the cache (`StanzaEngine.process_handle` step 3), the engine
skips the batched callback entirely.  No `progress_fn` fires at all.  The file goes
from `Processing...` to `DONE` in milliseconds.  This is great for throughput but
means progress reporting is even more unpredictable — sometimes a file takes 0.1s,
sometimes 30s, and the progress display gives no advance warning.

## Possible Improvements

### 1. Expose Per-File Progress in CLI (Low Effort, Medium Value)

The server already tracks `progress_current`/`progress_total` per file.  The CLI
client (`dispatch_server.py`) already polls `/jobs/{id}`.  We could display:

```
  Processing… (3 / 126)  [file: 45/120 utterances, Morpho-Syntax]
```

This would only be useful for FA/translation (where progress increments).  For
morphotag (batch), it would show `0/120` then `120/120`.  Still better than nothing
— at least the user sees the utterance count and knows the system is working.

**Effort:** ~20 lines in `dispatch_server.py`.  Parse `file_statuses` for the
currently-processing file, display its `progress_current`/`progress_total`/
`progress_label`.

### 2. Sub-Batch Progress for Stanza (Medium Effort, Medium Value)

Instead of sending all N utterances to Stanza at once, chunk them into sub-batches
(e.g. 50 utterances each) and report progress between chunks.  This would give
genuine incremental progress for morphotag.

**Trade-off:** Sub-batching may reduce Stanza throughput.  Neural batching benefits
from larger batches (better GPU utilization, amortized overhead).  We'd need to
benchmark to find the sweet spot.  On CPU (our production case), the benefit of
large batches is smaller — Stanza on CPU processes linearly, not in parallel.  So
sub-batching of ~50 utterances would likely have negligible throughput impact while
giving 10+ progress updates per file.

**Where to implement:** In `_stanza_batch_callback.py`, split `items` into chunks,
call `nlp()` per chunk, and report to a progress callback between chunks.  The Rust
side would need to call `progress_fn` per sub-batch rather than just at start/end.

Alternatively, implement sub-batching purely on the Python side: the batch callback
could accept a progress function and report between chunks, without changing the
Rust calling convention.

### 3. Rich Per-File Progress Bar for Local Dispatch (Medium Effort, High Value)

Replace the current `Queued.../Processing.../DONE` with a proper Rich progress bar
per file, driven by the pipeline callback.  This would show:

```
⠋ [███████░░░░] 45/120  Morpho-Syntax  020100.cha  0:15
⠋ [█░░░░░░░░░░]  3/85   Forced Align   020200.cha  0:03
✓ DONE                                  010100.cha  0:06
```

**Prerequisite:** Needs improvement #2 (sub-batch progress) to be meaningful for
morphotag.  Without it, the bar jumps 0→100% and defeats the purpose.

**Implementation:** `dispatch_local.py` currently sets `total=1` per file in the
Rich Progress.  Change to: (a) discover utterance count before dispatch (count `*`
lines in the CHAT file), (b) set `total=utterance_count`, (c) wire the pipeline
callback to `prog.update(task_id, completed=current)`.

For ProcessPoolExecutor: not possible without IPC — workers can't update the parent
process's Rich Progress directly.  Would need a `multiprocessing.Queue` or shared
memory for progress events.

For ThreadPoolExecutor: straightforward — threads share the Progress object.

### 4. Dashboard Per-File Progress Bar (Low Effort, Medium Value)

The HTMX dashboard already has `progress_current`/`progress_total` available.
Adding a `<progress>` element or percentage text per file row would be trivial.

**Effort:** ~10 lines in the dashboard template.

### 5. ETA Estimation (High Effort, Uncertain Value)

Use historical per-utterance timings (from cache or run logs) to estimate remaining
time.  Extremely noisy in practice due to:

- Variable utterance length
- Cache hits (instant) vs misses (seconds)
- Multi-language files (different Stanza models, different speeds)
- First-file-slow effect (model loading)

**Recommendation:** Don't pursue this.  File completion rate (files/minute) is a
more reliable signal and the server already provides it implicitly.

## Recommendation

**Priority order:**

1. **Dashboard progress bar** (#4) — trivial, immediately useful for server users.
2. **CLI server-mode progress detail** (#1) — low effort, helps `--server` users.
3. **Sub-batch Stanza progress** (#2) — medium effort, prerequisite for #3, and
   worth benchmarking on CPU to confirm no throughput regression.
4. **Rich per-file progress bar** (#3) — nice polish, but only for ThreadPoolExecutor
   (local daemon and server paths), not ProcessPoolExecutor.
5. **Skip ETA** (#5) — not worth the complexity.
