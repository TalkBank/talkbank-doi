# Throughput Benchmarks — February 2026

**Status:** Historical
**Last updated:** 2026-03-15

This report captures the February 2026 free-threaded Python 3.14t experiment
wave. It is preserved for benchmark history, not current deployment guidance.
Current `batchalign3` runtime guidance uses Python 3.12; see:

- `batchalign3/book/src/developer/python-versioning.md`
- `batchalign3/book/src/user-guide/installation.md`
- `docs/net-talkbank-server.md`

Cache-free measurements with `--override-cache`. These scenarios used Python
3.14t (free-threaded build) with the `batchalign3` server at the time of the
experiment. The master CLI used Python 3.12.

## Executive Summary

- **Morphotag on 16-core test machine**: GIL=0 w=8 → **6,107 files/hr** — 3.6x vs master
- **Morphotag on 28-core lab machine**: GIL=1 w=8 → **3,727 files/hr** — GIL=0 degrades catastrophically
- **Align on 16-core test machine**: GIL=0 w=8 → **1,187 files/hr** — 1.9x vs master
- **Align on 28-core lab machine**: GIL=1 w=12 → **1,230 files/hr** — GIL=0 tied at same workers

## Recommended Production Configuration

Production and lab machines share identical hardware (28 cores, 256 GB). Both should use:

```yaml
# server.yaml
max_workers_per_job: 8
max_concurrent_jobs: 3
```

With `PYTHON_GIL=1` (ProcessPool mode).

**Why GIL=1**: Morphotag (the dominant workload) is 1.25x faster with GIL=1 on
28-core machines. Align is tied either way. GIL=0 causes catastrophic thread
contention from Apple Accelerate on high-core-count machines.

**Why w=8**: Morphotag plateaus at w=8 (3,727 f/h), zero gain at w=12 (3,721).
Align at w=8 is within 5% of w=12. Using w=8 instead of w=12 allows 3 concurrent
jobs within memory budget (3 x 22GB = 66GB active, well within 256GB).

**Auto-tuner rules**:
- `>= 24 cores` → GIL=1 (ProcessPool), max_workers = min(8, cores/3)
- `< 24 cores` → GIL=0 (ThreadPool), max_workers = min(8, cores/2)

## Worker Sweep: Morphotag

### Test Machine (16 cores, 64 GB) — 200 files

| Workers | Master | | | GIL=1 | | | GIL=0 | | |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| | Time | f/h | RSS | Time | f/h | RSS | Time | f/h | RSS |
| 1 | 13.9m | 865 | 2.5 GB | **6.6m** | **1,831** | 5.8 GB | 7.1m | 1,692 | 5.7 GB |
| 2 | 11.1m | 1,081 | 4.1 GB | 6.6m | 1,825 | 5.8 GB | 3.9m | 3,081 | 6.2 GB |
| 4 | 7.8m | 1,547 | 7.3 GB | 6.8m | 1,757 | 5.5 GB | 2.5m | 4,898 | 6.5 GB |
| 8 | **7.1m** | **1,689** | 13.6 GB | — | — | — | **2.0m** | **6,107** | 6.4 GB |
| 16 | 7.1m | 1,685 | 23.2 GB | — | — | — | 2.0m | 6,107 | 6.4 GB |

- **Master best**: w=8, 1,689 f/h (13.6 GB RSS — each worker loads separate models)
- **GIL=1**: No scaling at all (w=1 to w=4 all ~1,830 f/h). ProcessPool overhead on 64GB machine negates parallelism. Skipped w=8/16 (would exceed RAM).
- **GIL=0 best**: w=8, 6,107 f/h (6.4 GB RSS). **3.6x vs master, 3.3x vs GIL=1.** Plateau at w=8.

### Lab Machine (28 cores, 256 GB) — 200 files

| Workers | GIL=1 | | | GIL=0 | | |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| | Time | f/h | RSS | Time | f/h | RSS |
| 1 | 6.6m | 1,830 | 5.9 GB | 6.7m | 1,798 | 5.8 GB |
| 2 | 4.3m | 2,791 | 8.0 GB | 4.5m | 2,690 | 6.8 GB |
| 4 | 3.4m | 3,568 | 13.7 GB | **4.0m** | **2,984** | 5.8 GB |
| 8 | **3.2m** | **3,727** | 22.3 GB | 5.0m | 2,394 | 6.0 GB |
| 12 | 3.2m | 3,721 | 22.1 GB | 5.9m | 2,050 | 6.3 GB |
| 16 | 3.2m | 3,721 | 22.3 GB | 7.1m | 1,696 | 7.3 GB |
| 20 | 3.2m | 3,711 | 22.2 GB | 8.5m | 1,417 | 7.5 GB |
| 28 | — | — | — | 11.7m | 1,030 | 7.9 GB |

- **GIL=1 best**: w=8, 3,727 f/h. Plateau at w=8, flat through w=20.
- **GIL=0**: Peak at w=4 (2,984 f/h), then **catastrophic degradation** — w=28 is 2.9x slower than w=4. Apple Accelerate spawns ~8 internal threads per matrix op; 28 workers x 8 threads = 224 threads fighting over 28 cores.
- **GIL=1 wins by 1.25x** at optimal workers.

## Worker Sweep: Align (Forced Alignment)

### Test Machine (16 cores, 64 GB) — 18 files

| Workers | Master | | | GIL=1 | | | GIL=0 | | |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| | Time | f/h | RSS | Time | f/h | RSS | Time | f/h | RSS |
| 1 | 7.4m | 146 | 2.4 GB | 8.1m | 134 | 2.9 GB | 5.5m | 195 | 3.0 GB |
| 2 | 3.0m | 363 | 3.5 GB | 2.8m | 388 | 2.8 GB | 5.6m | 194 | 2.9 GB |
| 4 | **1.7m** | **627** | 5.4 GB | 3.0m | 363 | 3.0 GB | 1.5m | 741 | 3.0 GB |
| 8 | 2.1m | 510 | 18.2 GB | **1.1m** | **963** | 3.0 GB | **55s** | **1,187** | 3.0 GB |
| 16 | 2.1m | 512 | 17.9 GB | 1.4m | 773 | 3.0 GB | 55s | 1,185 | 3.0 GB |

- **Master best**: w=4, 627 f/h (5.4 GB). Regresses at w=8 due to model duplication (18 GB!).
- **GIL=1**: Erratic scaling (w=4 slower than w=2). ProcessPool overhead hurts on small jobs. Best w=8, 963 f/h.
- **GIL=0 best**: w=8, 1,187 f/h (3.0 GB). **1.9x vs master, 1.2x vs GIL=1.** Plateau at w=8.

### Lab Machine (28 cores, 256 GB) — 18 files

| Workers | GIL=1 | | | GIL=0 | | |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| | Time | f/h | RSS | Time | f/h | RSS |
| 1 | 7.1m | 152 | 3.0 GB | 6.8m | 158 | 3.0 GB |
| 2 | 2.8m | 381 | 3.0 GB | 3.3m | 324 | 3.0 GB |
| 4 | 1.5m | 745 | 2.9 GB | 2.7m | 403 | 2.9 GB |
| 8 | 55s | 1,174 | 3.0 GB | 1.7m | 651 | 3.0 GB |
| 12 | **53s** | **1,230** | 3.0 GB | **54s** | **1,207** | 3.0 GB |
| 16 | 55s | 1,172 | 3.0 GB | 1.2m | 927 | 3.0 GB |
| 20 | 55s | 1,172 | 2.9 GB | 55s | 1,174 | 3.0 GB |
| 28 | 1.9m | 558 | 3.0 GB | 55s | 1,174 | 3.0 GB |

- **GIL=1 best**: w=12, 1,230 f/h. Plateau at w=8-12. Memory flat at ~3 GB (models shared by prewarmed server).
- **GIL=0 best**: w=12, 1,207 f/h. Essentially **tied with GIL=1** at optimal workers. Unlike morphotag, align doesn't suffer Accelerate contention — wav2vec is I/O-bound (audio decoding).
- GIL=1 w=28 regresses (558 f/h) while GIL=0 w=28 stays at 1,174 f/h — GIL=0 degrades more gracefully for align.

## GIL Mode Analysis

Python 3.14t supports free-threading (GIL=0), which allows true thread-level
parallelism with shared model memory. The GIL mode choice depends on the workload
and core count:

| Machine | Cores | RAM | Morphotag winner | Align winner | Production choice |
| --- | --- | --- | --- | --- | --- |
| Test machine | 16 | 64 GB | GIL=0 (3.3x) | GIL=0 (1.2x) | **GIL=0** |
| Production / lab (28c) | 28 | 256 GB | GIL=1 (1.25x) | Tied | **GIL=1** |

**Why the split**: PyTorch calls Apple Accelerate for BLAS operations, which spawns
~8 internal threads per matrix multiplication. Under GIL=0 (ThreadPool), W workers
share a single process, so W x 8 = total thread count. On a 28-core machine with w=8, that's
64 threads on 28 cores — tolerable. At w=28, it's 224 threads — catastrophic context
switching. Under GIL=1 (ProcessPool), each worker is an isolated process; the OS
schedules them without cross-process thread contention.

Align is less affected because wav2vec spends most time on audio I/O and
short inference bursts, giving Accelerate threads less opportunity to collide.

**Threshold**: `>= 24 cores` → GIL=1, `< 24 cores` → GIL=0.
Auto-configured in `daemon.py` and `deploy_server.sh`.

## vs Master Baseline (16-core test machine)

| Command | Master best | Our best | Speedup | Memory savings |
| --- | --- | --- | --- | --- |
| Morphotag | 1,689 f/h (w=8, 13.6 GB) | 6,107 f/h (GIL=0 w=8, 6.4 GB) | **3.6x** | **53%** |
| Align | 627 f/h (w=4, 5.4 GB) | 1,187 f/h (GIL=0 w=8, 3.0 GB) | **1.9x** | **44%** |

Master's memory scales linearly with workers (separate model copies per process).
Our server shares models across all workers, keeping RSS flat regardless of
worker count.

## Methodology

- All runs use `--override-cache` to bypass the utterance cache
- Master CLI cache DB cleared (including WAL/SHM) before each run
- Server scenarios use a prewarmed server (models loaded before timing starts)
- Master CLI includes cold start (model loading) in timing
- Fresh server started/stopped for each worker count to avoid state leakage
- Peak RSS measured via psutil process tree sampling (2s intervals)
- Parallelism controlled via `max_workers_per_job` in server config (server scenarios)
  and `--workers N` global flag (master CLI)
- Morphotag dataset: 200 CHAT files from CHILDES
- Align dataset: 18 paired .cha/.wav files from CHILDES
- Python 3.14t (free-threaded) for batchalign3 server
- Python 3.12 for master CLI
- Worker sweep script: `~/batchalign-benchmarking/scripts/worker_sweep.py`
