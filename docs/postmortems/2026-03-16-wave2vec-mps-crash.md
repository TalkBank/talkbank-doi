# Postmortem: Wave2Vec FA Worker Crash on MPS (2026-03-16)

**Date of incident:** 2026-03-16, ~15:31 ET
**Duration:** ~3 minutes (15:31 – 15:33 ET), but 6 of 11 files permanently failed
**Impact:** Davida's `align` job on aphasia-data ACWT corpus — 6/11 files failed
**Severity:** Medium — partial job failure, no data loss, retryable after fix

## Summary

Davida submitted an `align` job from `macw@davida` to the batchalign3 server on
net (port 8001), processing 11 PWA/ACWT files from the aphasia-data corpus.
5 files completed (FA results cached from a prior run, plus ACWT08a which ran
fresh FA successfully). The Wave2Vec FA worker (PID 28032) crashed during
concurrent processing of 3 large video files, killing all in-flight work. All
6 remaining files failed after 2-3 retry attempts each.

## Root Cause

`load_wave2vec_fa()` in `batchalign/inference/fa.py` loaded the MMS_FA model
onto MPS without forcing float32:

```python
model = bundle.get_model().to(device)  # No dtype override on MPS
```

The Whisper FA loader already had the correct pattern:

```python
if device.type == "mps":
    torch_dtype = torch.float32
```

The MMS_FA bundle's default dtype can include bfloat16 operations that MPS does
not reliably support. Under concurrent load with large audio files (200-460 MB
mp4 videos → WAV conversion → Wave2Vec inference), this triggered a worker
process crash surfacing as `Broken pipe (os error 32)`.

## Timeline

| Time (ET) | Event |
|-----------|-------|
| 15:30:47 | Job `c6ad3d68` submitted: `align`, 11 PWA/ACWT files, `eng`, wave2vec FA |
| 15:31:21 | 11 files dispatched in parallel. SQLite contention on 4 `insert_attempt_start` |
| 15:31:21–44 | 5 files complete via FA cache hits (01a, 03a, 04a, 07a: 5-11s each) |
| 15:31:32–33:17 | ACWT08a runs fresh FA successfully (1m45s) |
| 15:33:15–17 | Wave2Vec worker (PID 28032) crashes. ACWT05a, 09a, 12a fail (in-flight ~2 min) |
| 15:33:17–24 | Retry cascade: 02a, 05a, 09a, 10a, 11a, 12a retry 2-3× each, all crash in 1-3s |
| 15:33:24 | Job marked `completed` with 6 file errors |
| 15:33:39 | Server log: `Worker process died, scheduling restart target=infer:fa engine_overrides={"fa":"wave2vec"} pid=28032` |

## What Went Right

- Server data mappings worked correctly (`aphasia-data` → `/Volumes/Other/aphasia/`)
- FA cache served 4 files instantly from a prior run
- Retry system correctly classified crashes as `worker_crash` and retried
- Job marked completed (not stuck) despite partial failures
- Attempt history preserved for diagnosis

## What Went Wrong

1. **Missing MPS dtype safety on Wave2Vec** — The fix was already known for
   Whisper (documented in MEMORY.md: "MPS bfloat16 crash: force torch.float32
   on MPS in infer_asr.py") but never applied to the Wave2Vec FA path.

2. **Error messages surfaced system internals** — Davida saw `"FA processing
   failed: worker error: I/O error: Broken pipe (os error 32)"` in the
   dashboard. This is useless to end users.

3. **SQLite write contention** — 4 `insert_attempt_start` calls failed with
   `database is locked` at job start (11 concurrent files). Non-fatal but
   caused missing attempt records for ACWT03a.

## Fix

**Wave2Vec MPS dtype** (`batchalign/inference/fa.py`):

```python
model = bundle.get_model()
if device.type == "mps":
    model = model.float()  # Force float32 — MPS bfloat16 unreliable
model = model.to(device)
```

Test: `test_load_wave2vec_fa_forces_float32_on_mps` — parametrized across
cpu/cuda/mps/fallback, asserts `.float()` called only on MPS.

**User-facing error messages** (`error_classification.rs`):

Added `user_facing_error()` translator that converts `FailureCategory` into
actionable messages. Applied to FA, transcribe, benchmark, and media analysis
pipelines. Raw errors logged via `tracing::warn` for developer debugging.

**Frontend category mapping** (`useFileFilters.ts`):

Added `CATEGORY_NORMALIZE` map translating all 11 backend `FailureCategory`
values into 5 display groups. Previously `worker_crash` showed as raw text.

## Action Items

- [x] Fix Wave2Vec MPS dtype (this postmortem)
- [x] Add TDD test for MPS float32 enforcement
- [x] Add user-facing error message translation layer
- [x] Fix frontend error category mapping
- [ ] Redeploy batchalign3 to net with the fix
- [ ] Restart Davida's job to process the 6 failed files
- [ ] Investigate SQLite WAL contention under high parallelism (11 concurrent writes)
- [ ] Audit all remaining model loaders for MPS dtype safety
