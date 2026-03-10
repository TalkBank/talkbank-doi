# Job Retry / Crash Recovery Proposal

## Problem

When Celery workers crash (SIGSEGV, OOM, semaphore exhaustion), in-flight
tasks are lost. The job stays in Redis as "queued" or "running" forever,
but no worker will pick up the tasks again because they've already been
consumed from the Celery queue. The only option today is to delete the
job and resubmit from the client.

## What Survives a Crash

Already persisted in Redis:
- `ba:job:{id}` -- job metadata (command, lang, num_speakers, options,
  config_dict, engine_overrides, media_mapping, media_subdir)
- `ba:job:{id}:files` -- per-file status hash (queued/processing/done/error)
- `ba:job:{id}:results` -- completed file results (CHAT text or CSV)

**What is lost:** The input data for each file. CHAT content and media
filenames are passed as Celery task arguments, which vanish when the
task is consumed from the Redis queue and the worker dies.

## Proposed Changes

### 1. Persist task inputs at submission time

In `app.py` `submit_job()`, after creating the job in Redis, store each
file's input data:

```
ba:job:{id}:inputs:{filename}  ->  {
    "chat_content": "...",      # or null for media-input commands
    "media_mapping": "...",
    "media_subdir": "...",
}
```

Cost: ~2KB per CHAT file, negligible for media-only (just filenames).
TTL matches the job TTL (7 days default).

### 2. Add `POST /jobs/{id}/retry` endpoint

Logic:
1. Read job metadata from `ba:job:{id}`
2. Read per-file statuses from `ba:job:{id}:files`
3. Collect files where status is NOT "done"
4. Read their inputs from `ba:job:{id}:inputs:{filename}`
5. Dispatch new `process_file` tasks for those files only
6. Create a new Celery chord (group + finalize callback)
7. Reset job status to "running", reset `completed_files` counter
   to only count previously-done files

Returns: updated job info with count of retried files.

### 3. Optional: auto-recovery on server startup

In `serve start`, after health check, scan for stale jobs:
- Jobs with status "running" or "queued" that have incomplete files
- Print a warning: "Found N incomplete jobs. Resume with:
  `curl -X POST http://host:port/jobs/{id}/retry`"

Or auto-resume them (configurable via `auto_resume_jobs: true` in
server.yaml).

### 4. Client-side support

Add retry to the `batchalign jobs` CLI:
```
batchalign jobs <job_id> --retry --server http://server:8000
```

Or automatically retry when polling detects a stalled job (no progress
for N minutes + server was restarted).

## Files to Change

| File | Change |
|------|--------|
| `serve/app.py` | Store inputs in Redis at submission; add `POST /jobs/{id}/retry` endpoint |
| `serve/redis_store.py` | `store_inputs()`, `get_inputs()`, `get_incomplete_files()` methods |
| `serve/models.py` | `RetryResponse` schema |
| `cli/serve_cmd.py` | Optional: stale job scan on startup |
| `cli/dispatch_server.py` | Optional: `--retry` flag support |
| `tests/serve/test_app.py` | Tests for retry endpoint |

## Edge Cases

- **Job was cancelled** -- don't allow retry (status check).
- **All files already done** -- return immediately, trigger finalize.
- **Inputs expired from Redis** -- return error "inputs no longer
  available, please resubmit". This happens if retry is attempted
  after `job_ttl_days`.
- **Different server config after restart** -- media_roots may have
  changed. Media resolution happens at processing time, so this is
  fine as long as the media still exists.
- **Partial results from crashed "processing" files** -- reset their
  status to "queued" before re-dispatching.

## Not In Scope

- Resuming jobs across different server versions (schema migration).
- Deduplication if the same file was partially processed (pipeline is
  idempotent, so re-processing is safe).
- Client-side retry for network failures (already handled by
  `_request_with_retry` in dispatch_server.py).
