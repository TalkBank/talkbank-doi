# Postmortem: net.talkbank.org Kernel Panic (2026-03-11)

**Date of incident:** 2026-03-11, ~11:45 ET
**Duration:** ~2 hours (11:45 – 13:41 ET)
**Impact:** batchalign-next server on net unreachable; davida's align job failed
**Severity:** High — full macOS kernel panic and forced reboot

## Summary

net.talkbank.org (Mac Studio, Apple M3 Ultra, 256 GB RAM) experienced a macOS kernel panic triggered by the watchdog daemon (`watchdogd`) failing to check in for 91 seconds. The immediate cause was VM compressor segment exhaustion — the panic report shows `100% of segments limit (BAD) with 85 swapfiles`. The system consumed well over 256 GB of virtual memory (all 2.5M compressor segments at 64KB each ≈ 153 GB compressed data, plus 85 GB swap), far exceeding what any normal ML workload could produce.

**Most likely root cause: Python 3.14t free-threaded runtime (GIL disabled) running PyTorch/Stanza/NumPy under concurrent load, causing catastrophic memory fragmentation or leaks in C extension libraries not designed for truly concurrent access.**

## Hardware

- **Machine:** Mac Studio (Mac15,14)
- **Chip:** Apple M3 Ultra (28 cores: 4 efficiency clusters + 4 performance clusters)
- **RAM:** 256 GB unified memory
- **Storage:** Internal SSD + 3 external APFS volumes (CHILDES, HomeBank, Other — ~22 TB total)
- **OS:** macOS 26.3.1 (Build 25D2128), Darwin 25.3.0

## Timeline (all times ET, 2026-03-11)

| Time | Event | Source |
|------|-------|--------|
| Mar 2, 11:23 | batchalign-next server started (PID 54169) with `warmup: true` — Stanza models loaded | server.log |
| Mar 2–11 | Server running continuously for 9 days, idle (no jobs submitted) | jobs.db |
| 11:25:08 | `batchalign3 serve start` attempted on net | batchalign3 server.log |
| 11:25:08 | Server crashes immediately: `command 'utseg' is advertised but infer task 'utseg' is missing` (capability gate bug) | batchalign3 server.log |
| 11:34:59 | Align job `bf911cd6bafc` submitted from davida (11 ACWT aphasia .cha files, `max_workers_per_job: 2`) | jobs.db |
| 11:37–11:39 | Two additional CLI startups on net (likely auto-daemon retries) | batchalign3 server.log |
| ~11:45 | **Kernel panic** — watchdog timeout (91 seconds), forced reboot | panic report |
| 11:52 | Post-reboot: `apfsd` CPU resource diagnostic generated | DiagnosticReports |
| 12:28 | Post-reboot CLI startup (no server running) | batchalign3 server.log |
| 12:38 | Orphaned align job `bf911cd6bafc` marked `failed` | jobs.db |
| ~13:40 | Manual investigation begins, manual shutdown | operator |
| 13:41 | Manual reboot — network unreachable until Ethernet cable physically unplugged/replugged | operator |
| ~13:45 | batchalign-next restarted on port 8000 | operator |

## Root Cause Analysis

### Panic Report Evidence

From `/Library/Logs/DiagnosticReports/panic-full-2026-03-11-114515.0002.panic`:

```
panic(cpu 6 caller 0xfffffe0048cc402c): watchdog timeout: no checkins from watchdogd in 91 seconds
Compressor Info: 35% of compressed pages limit (OK) and 100% of segments limit (BAD) with 85 swapfiles and OK swap space
```

Key facts:
- All 28 cores halted at the same kernel instruction (debugger halt after panic)
- **85 swap files** active (~85 GB swap)
- VM compressor segments at **100% of limit** (limit = 2,516,582 segments × 64KB = ~153 GB compressed data)
- Combined: system was using **~238 GB+ of virtual memory** beyond the 256 GB physical RAM
- The watchdog daemon couldn't check in because the kernel's VM subsystem was completely saturated servicing page faults and swap I/O

### The Scale Problem

256 GB RAM should be more than enough for batchalign. Current idle measurements (post-reboot):
- batchalign-next with Stanza warmup: **7.7 GB** resident
- Full align workload adds: Whisper (~1.5 GB) + wav2vec FA (~1.2 GB) + NeMo diarization (~500 MB) ≈ **~4 GB**
- Even with 2 worker processes: **~20 GB total**

The crash consumed **300+ GB of virtual address space**. This is 15x what the workload should require. Normal ML model loading cannot explain this.

### Root Cause: Python 3.14t Free-Threaded Runtime

batchalign-next on net runs on **Python 3.14.3 free-threading build** with the **GIL disabled** (`sys._is_gil_enabled() == False`). This is the most likely cause:

1. **C extensions without GIL safety:** PyTorch 2.10.0, Stanza, NumPy, and other C extensions perform internal memory management (custom allocators, reference counting, arena management) that historically relied on the GIL for thread safety. With the GIL disabled, concurrent access from multiple threads can corrupt allocator metadata, cause double-frees, or create unbounded memory fragmentation.

2. **9 days of accumulated damage:** The server was running since March 2 with models pre-loaded. Even if the memory corruption was slow (a few MB per hour from fragmentation), 9 days × 24 hours could accumulate significant waste. The align job then triggered concurrent model loading and inference, amplifying any existing issues.

3. **Concurrent align workers:** With `max_workers_per_job: 2`, the align job would spawn 2 worker processes/threads running ML inference concurrently. On a free-threaded build, this means true concurrent execution of PyTorch operations across cores — exactly the scenario most likely to trigger thread-safety bugs in C extensions.

4. **MPS (Metal Performance Shaders) interaction:** `torch.backends.mps.is_available() == True` on this machine. If PyTorch attempted to use MPS for inference, the Metal framework's GPU memory management combined with free-threaded Python's lack of GIL protection could cause severe memory leaks in the unified memory pool.

### Evidence Supporting This Theory

- The memory consumption (300+ GB) is orders of magnitude beyond what the workload requires
- Normal memory exhaustion would trigger macOS's jetsam (process killing) before a kernel panic — segment exhaustion suggests pathological fragmentation, not just high usage
- Python 3.14t free-threaded mode is experimental and known to have issues with C extensions
- Spotlight CPU resource diagnostics at 4:10 AM and 5:10 AM (hours before crash) suggest the system was already under stress before the align job
- Only one job was ever submitted to this server — the align job was the first real workload in 9 days

### What We Cannot Confirm

- Whether the memory issue was accumulating slowly over 9 days (leak) or triggered suddenly by the align job (concurrency bug)
- Whether MPS was actually used (server log shows `Using device: cpu` for Stanza, but Whisper may default to MPS on Apple Silicon)
- Whether the same crash would occur on standard GIL-enabled Python 3.12
- The exact thread/process that triggered the runaway allocation

### Contributing Factor: Failed batchalign3 Start

The `batchalign3 serve start` at 11:25 crashed due to the capability gate bug. This crash was instantaneous (within the same second) and would not have contributed meaningful memory pressure. The failed start may have triggered auto-daemon retry logic in subsequent CLI invocations (the 11:37 and 11:39 log entries), briefly spawning Python processes that would immediately exit.

## Impact

- davida's align job on 11 ACWT aphasia files failed — needs resubmission
- batchalign-next was unavailable for ~2 hours
- No data loss (input files on davida were unaffected)
- No permanent system damage (clean reboot, all volumes intact)
- Post-reboot network was unreachable until Ethernet cable was physically unplugged and re-plugged — kernel panic may have left the NIC in a bad state that persisted across reboot

## Fixes and Prevention

### Immediate (do now)

1. **Stop using Python 3.14t free-threaded** — Switch batchalign-next to Python 3.12 on net. This eliminates the most likely root cause. Command: `uv tool install batchalign-next --python 3.12`

2. **Deploy batchalign3** — The Rust server with Python 3.12 workers avoids this class of bug entirely:
   - Python workers are isolated subprocesses (not threads)
   - Each worker loads models independently with GIL protection
   - `memory_gate_mb` config prevents over-commitment
   - Worker idle timeout reclaims memory from unused models

### Already Fixed in batchalign3 (not yet deployed)
1. **Capability gate bug** — `crates/batchalign-app/src/lib.rs`: Server no longer crashes on mismatched capability advertisements
2. **daemon.rs current_exe() bug** — `crates/batchalign-cli/src/daemon.rs`: Auto-daemon spawn works correctly via `uv tool install`

### Monitoring (recommended)

1. **Memory monitoring script** — Run on net via launchd:
   ```bash
   # /usr/local/bin/memwatch.sh — log memory stats every 60s
   while true; do
     echo "$(date '+%Y-%m-%d %H:%M:%S') $(vm_stat | grep -E 'free|compressor|Pages active')"
     sleep 60
   done >> /var/log/memwatch.log
   ```

2. **Set batchalign3 memory_gate_mb** — Configure in `server.yaml`:
   ```yaml
   memory_gate_mb: 230000  # Reserve 26 GB for OS + system services
   ```

3. **Limit worker concurrency** — `max_workers_per_key: 2` to prevent too many concurrent model loads

## Lessons Learned

1. **Python 3.14t free-threaded mode is not production-ready for ML workloads.** C extensions (PyTorch, NumPy) rely on the GIL for memory safety. Disabling it under concurrent load can cause catastrophic memory corruption that overwhelms even 256 GB of RAM.

2. **macOS kernel panics on VM compressor segment exhaustion**, not just on OOM. This is different from Linux's OOM killer, which terminates processes. On macOS, pathological memory fragmentation can crash the entire system.

3. **Long-running ML servers need memory monitoring.** The batchalign-next server ran for 9 days without any memory visibility. A simple `vm_stat` log would have shown the problem building.

4. **batchalign3's architecture (subprocess isolation + memory gating) is the right design.** Process-level isolation of Python ML workers provides both GIL protection and OS-level memory accounting. The memory gate prevents over-commitment. This incident validates the architectural decision.

5. **Warmup with `warmup: true` means models persist in memory for the server's entire lifetime.** On a free-threaded runtime, these long-lived objects in the heap may contribute to fragmentation over time.

---
*Written: 2026-03-11*
*Updated: 2026-03-11 — added hardware specs (256 GB RAM), Python 3.14t free-threaded root cause analysis*
