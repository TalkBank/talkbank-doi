# Server Architecture: Performance Gains Over Local CLI

> **Note:** The memory comparison below applies to the **Python server**
> (in-process model sharing via PipelineCache). The Rust server uses
> out-of-process Python workers, each with its own model copy
> (WorkerPool architecture). The Rust server's advantage is lighter
> deployment (no Python on clients), faster startup, and better
> concurrency — not shared-memory model reuse.

## Typical workload

- Hundreds of CHAT/audio files per batch
- 8 worker processes (`--workers 8`)
- Commands: morphotag, align, transcribe
- Run multiple times per day across lab members

## The cost of local CLI (master branch)

On master, every `batchalign` invocation spawns worker processes. Each worker loads its own copy of all models from scratch, processes its share of files, then exits -- discarding everything.

### Per-invocation model loading cost

| Engine | Load time | RAM per copy |
|--------|-----------|-------------|
| Stanza (morphotag) | ~45s | ~2-4 GB |
| WhisperFA (align) | ~25s | ~2-3 GB |
| Whisper ASR (transcribe) | ~20s | ~2-3 GB |
| Pyannote (diarize) | ~5-10s | ~1 GB |

### With 8 workers

Each of the 8 processes loads its own copy:

- **Wall time**: ~45s before any file is processed (all 8 workers loading in parallel)
- **Memory**: 8 copies of every model. Morphotag alone: 8 x 3 GB = **~24 GB** just for Stanza. Align: 8 x 2.5 GB = **~20 GB** for WhisperFA. On a 32 GB Mac, this either crashes or forces heavy swap.
- **Thrown away after every run**: Close the terminal, models are gone. Run again, wait another 45 seconds and re-allocate another 24 GB.

### Over a day

A researcher running morphotag 5 times and align 3 times spends:

- 8 x 45s model loads = **~6 minutes** of dead time
- Peak memory repeatedly spikes to 20-30 GB, competing with other apps
- Every lab member on the same Mac pays the same cost independently

## What the server saves

The server loads each pipeline **once** at startup (~85s total for morphotag + align) and keeps models in memory permanently. All subsequent requests -- from any client, any number of files -- reuse the loaded models.

### Model loading

| | Master (8 workers) | Server |
|---|---|---|
| First run | 45s load + processing | 0s load + processing |
| Second run | 45s load + processing | 0s load + processing |
| Tenth run | 45s load + processing | 0s load + processing |
| Daily total (8 runs) | ~6 min wasted | 0s wasted |

The 85-second server startup is paid once -- at boot, via launchd. Researchers never see it.

### Memory

| | Master (8 workers) | Server |
|---|---|---|
| Stanza models | 8 copies (~24 GB) | 1 copy (~3 GB) |
| WhisperFA models | 8 copies (~20 GB) | 1 copy (~2.5 GB) |
| Total model RAM | 20-40 GB | ~6 GB |
| Fits on 16 GB Mac? | No | Yes |

The server runs 8 concurrent jobs via threading, sharing one set of models. Master spawns 8 processes, each with its own copy.

### Client machines

| | Master | Server |
|---|---|---|
| Models downloaded? | Yes, on every Mac | Only on the production server |
| GPU needed? | Yes | No (the production server has the GPU) |
| Python/torch needed? | Yes | No (just `requests`) |
| Disk for models | ~5-10 GB | 0 |

### File throughput

For hundreds of files, model loading is a fixed overhead -- it doesn't scale with file count. But on master it's paid per invocation, while the server pays it once at boot. The actual per-file processing time is the same either way.

| File count | Master overhead | Server overhead |
|------------|----------------|-----------------|
| 10 files | 45s load, then process | just process |
| 100 files | 45s load, then process | just process |
| 500 files | 45s load, then process | just process |

The savings are purely in eliminating repeated startup cost and reducing memory pressure. For large batches where processing dominates (hundreds of files), the percentage saved is modest. For small frequent runs (the common pattern -- re-running morphotag on a few corpora throughout the day), eliminating 45s per run is significant.

## Summary

| Metric | Master (local, 8 workers) | Server |
|--------|--------------------------|--------|
| Model load per run | ~45s | 0s |
| Daily load waste (8 runs) | ~6 min | 0 |
| Peak model RAM | 20-40 GB | ~6 GB |
| Runs on 16 GB Mac client? | Barely | Yes (no models needed) |
| Multiple users share models? | No | Yes |
| Setup per client machine | Full install + models | `uv tool install batchalign3` |
