# Postmortem: Net Server Crash — Feb 25, 2026

**Date:** 2026-02-25
**Duration of impact:** ~50 minutes (10:50 → 11:40)
**Severity:** Service outage (server crash + reboot)
**Affected users:** davida (align job lost)

## Summary

The batchalign server on Net crashed during Whisper alignment inference due to an Apple Metal Performance Shaders (MPS) assertion failure. The crash was severe enough to trigger a macOS reboot. After rebooting, launchd restarted the server 12 times before it stabilized. One align job (11 files) was left stuck in `queued` status because auto-resume is intentionally disabled.

## Timeline (all times PST)

| Time | Event |
|------|-------|
| 10:49:55 | davida submitted align job `6794b04b9b9a` — 11 ACWT files from `/Users/macw/0data/aphasia-data/English/Protocol/ACWT/PWA` |
| ~10:50 | Server (PID 2607) began processing: loaded Whisper model, started MPS-accelerated inference |
| ~10:55 | **CRASH** — Metal assertion failure in `MPSNDArrayMatrixMultiplication` (dtype mismatch) |
| 10:56 | macOS rebooted (triggered by Metal crash or automatic watchdog) |
| 10:56–11:40 | Restart storm — launchd restarted the server 12 times (PIDs: 2658, 685, 703, 1008, 719, 1011, 686, 1827, 2234, 2779, 3205, 906). Each restart loaded Whisper models (~1.5 GB), competing for memory during boot. Some instances shut down cleanly (port conflicts or memory pressure). |
| 11:40 | Server stabilized on PID 906. Health check passes. |
| 13:28 | Investigation began. Server confirmed healthy, stuck job identified. |

## Root Cause

Apple's Metal Performance Shaders framework hit a dtype mismatch assertion during Whisper's matrix multiplication on MPS (Apple Silicon GPU):

```
/AppleInternal/.../MPSNDArrayMatrixMultiplication.mm:5028:
failed assertion 'Destination NDArray and Accumulator NDArray cannot have
different datatype in MPSNDArrayMatrixMultiplication'
```

This is a known class of intermittent bugs with PyTorch's MPS backend on Apple Silicon. The assertion fires when the MPS kernel receives tensors with mismatched floating-point precision (e.g., float16 destination vs float32 accumulator) in a code path that doesn't handle mixed precision.

**Why intermittent:** Previous align jobs completed successfully on the same server (e.g., `f48329193cfb` and `f34bae78f7cd` on Feb 23, both with 8 workers). The bug depends on runtime tensor allocation patterns and MPS kernel dispatch timing — it doesn't reproduce deterministically.

**Why a reboot:** MPS assertion failures can corrupt GPU state. macOS may have rebooted via the GPU watchdog or kernel panic handler.

## Impact

- **davida's align job** (`6794b04b9b9a`, 11 ACWT files): Lost. All files stuck in `queued`. Job cancelled during investigation.
- **~50 minutes of downtime**: Server unavailable from crash (~10:55) until stabilization (11:40).
- **Rev.AI uploads**: 56 retry events logged. Orphaned connections from the crash caused upload failures for in-flight transcription work. Some retries succeeded after restart; a few continued on the current server instance (intermittent network, not related to the crash).
- **No data loss**: Source files on disk are unmodified. Only the job's processing state was lost.

## Detection

User reported the crash. Investigation via SSH to Net, reading `~/.batchalign/logs/launchd-server-stderr.log` (74,757 lines, 11 MB) and SQLite job database at `~/.batchalign-next/jobs.db`.

## Contributing Factors

1. **No MPS error recovery**: PyTorch's MPS backend does not gracefully handle assertion failures — the process aborts immediately. There's no try/catch path for MPS kernel crashes.
2. **Auto-resume disabled by default**: `BATCHALIGN_AUTO_RESUME` defaults to `"0"` to prevent crash loops. This is correct safety behavior but means queued jobs are silently abandoned after restarts.
3. **Restart storm during boot**: launchd's `KeepAlive: Crashed: true` + `ThrottleInterval: 10` caused rapid restart cycling. Each restart loaded the full Whisper model (~1.5 GB MPS allocation), competing with other boot-time processes for memory.

## Server State at Investigation Time

```json
{
  "status": "ok",
  "version": "0.8.1-post.12",
  "free_threaded": true,
  "capabilities": ["align", "coref", "morphotag", "translate", "utseg"],
  "loaded_pipelines": ["morphotag:eng:2", "align:eng:2"],
  "active_jobs": 0,
  "worker_crashes": 0,
  "memory_gate_aborts": 0
}
```

## Remediation

### Immediate (done)
- [x] Cancelled stuck job `6794b04b9b9a`
- [x] Verified server is healthy on PID 906

### Fix applied (done)
- [x] **Root cause fixed**: Changed `torch_dtype=torch.bfloat16` to `torch.float32` on MPS in `batchalign/models/whisper/infer_asr.py`. Non-MPS devices retain bfloat16/float16.
- [x] **Comprehensive Rollout**: Applied explicit `torch.float32` configuration for MPS devices across all other Whisper engine pipelines to preemptively avoid identical crashes (`infer_fa.py`, `whisperx.py`, `oai_whisper.py`) across both legacy `batchalign-next` and the `batchalign3` rewrite.
- [x] **Deployed to Net**: Patched files scp'd and server restarted (PID verified healthy).
- [x] **Local source patched**: `batchalign3/batchalign/models/whisper/infer_asr.py`, `infer_fa.py`, `whisperx.py`, and `oai_whisper.py` updated.
- [x] **Documented**: Added as issue #2 in `book/src/architecture/server-known-issues.md`, user-facing entry in `book/src/user-guide/troubleshooting.md`.
- [ ] **Notify davida** to re-submit the align job.

### Remaining
- [ ] **Increase ThrottleInterval** in the launchd plist from 10s to 30s to reduce restart storm severity.
- [ ] **Monitor**: Watch for recurrence. Now that all Whisper pipelines explicitly use MPS in `float32`, if a float32 matmul still triggers the assertion in the future, a complete CPU fallback for all Whisper inference on MPS will be necessary.

## Historical Context

The server was redeployed on Feb 24 (Python 3.14t, version 0.8.1-post.12). There were also 4 reboots on Feb 24 (15:21, 15:31, 16:17, 16:24) — possibly related to the same MPS issue during initial testing, or manual reboots during deployment.

The earlier `ImportError: cannot import name 'MAX_PROCESS_WORKERS'` (log lines 1272–1299) was from a stale Python 3.12 installation during the initial deployment on Feb 24 and is unrelated to today's crash.

## Lessons Learned

1. MPS on Apple Silicon is not production-stable for all workloads. Whisper inference with mixed precision is a known risk area.
2. The `BATCHALIGN_AUTO_RESUME=0` default is correct — without it, the server would have crash-looped trying to re-process the same job that triggered the MPS bug.
3. The launchd restart storm (12 restarts in 44 minutes) wastes significant resources loading models repeatedly. A longer throttle interval or exponential backoff would help.
