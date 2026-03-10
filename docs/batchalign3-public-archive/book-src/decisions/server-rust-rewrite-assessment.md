# Server Rust Rewrite Assessment

**Date:** 2026-02-16
**Updated:** 2026-02-23
**Status:** Viable as staged migration (revised from "not recommended")

## Executive Summary

A Rust server rewrite would not deliver meaningful **performance** gains — the
server is pure infrastructure glue (3,210 lines of Python) whose runtime is
entirely dominated by ML inference in Stanza, Whisper, Pyannote, and other
Python-only libraries.  A Rust server waiting on Python ML is no faster than a
Python server waiting on Python ML.

However, performance was never the only consideration.  The maintainer prefers
Rust's type system, ownership model, and language properties for long-lived
infrastructure code.  That is a legitimate reason to migrate the orchestration
layer — provided the IPC cost of calling Python ML engines from a Rust server
is acceptable.

**Updated assessment (2026-02-23):** The IPC tax is negligible for batchalign's
workload.  The callback pattern is already batched (Rust collects all utterances,
Python processes once), so a typical morphotag job crosses the Rust↔Python
boundary only 1-2 times with ~10-100 KB JSON payloads.  At ~3-5 ms of IPC
overhead against ~30,000 ms of Stanza inference, the tax is 0.01%.  The staged
migration described in the server orchestration ADR (Rust control-plane + Python
sidecar workers via HTTP/Unix socket) is the recommended path forward.

## Current Server Architecture

### Infrastructure (3,210 lines, zero ML imports)

| Component             | Lines | Responsibility                              |
|-----------------------|-------|---------------------------------------------|
| HTTP/REST (FastAPI)   | ~724  | REST endpoints, lifespan, static files      |
| Job queue + executor  | ~1,122| In-memory jobs, ThreadPool/ProcessPool      |
| SQLite persistence    | ~434  | WAL mode, crash recovery, schema migrations |
| Per-file processing   | ~380  | Media resolution, format conversion, dispatch|
| Media discovery       | ~164  | Walk cache (60s TTL), stem matching         |
| Pydantic models       | ~136  | Request/response schemas                    |
| Config (YAML)         | ~110  | ServerConfig, omegaconf loading             |
| WebSocket             | ~108  | Async broadcast, JSON routing               |
| Entry point           | ~31   | uvicorn startup                             |

### ML Engines (called via `pipeline.process_chat_text()`)

The server never imports ML libraries directly.  All inference happens inside
`batchalign/pipelines/` engines, lazily loaded by `PipelineCache`:

| Engine                | ML Library           | Rust Status                    |
|-----------------------|----------------------|--------------------------------|
| StanzaEngine          | Stanza (PyTorch)     | Python callback, Rust injects  |
| WhisperEngine         | Whisper (transformers)| Python inference               |
| Wave2VecFAEngine      | wav2vec2 MMS         | Python encoder, Rust DP align  |
| WhisperFAEngine       | Whisper encoder       | Python encoder, Rust DP align  |
| PyannoteEngine        | pyannote.audio       | Python inference, Rust reassign|
| SeamlessTranslation   | Seamless M4T         | Python callback, Rust injects  |
| StanzaUtteranceEngine | Stanza constituency  | Python callback, Rust injects  |
| RevEngine             | Rev.AI HTTP API      | **Fully Rust**                 |
| NgramRetraceEngine    | N/A (rules)          | **Fully Rust**                 |
| DisfluencyEngine      | N/A (rules)          | **Fully Rust**                 |
| OpenSMILEEngine       | openSMILE (C++)      | Python wrapper around C++      |

### Frontend (decoupled)

The React dashboard (~5,200 lines TypeScript) communicates via REST + WebSocket.
It is entirely decoupled from the server implementation language and would work
unchanged with a Rust server exposing the same API.

## What a Rust Rewrite Would Look Like

### Easy Part: Infrastructure (weeks)

| Python Component      | Rust Equivalent                        |
|-----------------------|----------------------------------------|
| FastAPI routes        | `axum` or `actix-web`                  |
| Job queue + threading | `tokio` tasks + channels               |
| SQLite persistence    | `rusqlite`                             |
| Config (YAML)         | `serde_yaml`                           |
| WebSocket             | `axum` built-in                        |
| Pydantic models       | `serde` structs                        |
| Media walk + cache    | `walkdir` + `std::fs`                  |
| Memory pressure gates | `/proc/meminfo` or `sysinfo` crate     |

All of these have mature, well-tested Rust crates.  The translation is
mechanical.

### Hard Part: ML Inference Bridge

A Rust server still needs to call Python for every ML operation.  Two options:

1. **Embed Python via pyo3** -- The Rust server holds a Python interpreter and
   calls pipeline code through it.  This is essentially what we have today but
   inverted (Python is the host, Rust is the library).  Complications: GIL
   management during concurrent jobs, Python version pinning, interpreter
   lifecycle, error propagation across the boundary.

2. **Subprocess / RPC** -- The Rust server spawns Python worker processes and
   communicates via IPC (JSON over pipes, gRPC, Unix sockets).  Adds
   serialization latency, another failure mode, and deployment complexity (must
   ship both a Rust binary and a Python environment).

Both options still require shipping the entire Python ML stack (~4 GB: torch,
stanza, transformers, pyannote, whisperx).

## Cost/Benefit Analysis

### What You'd Gain

- **Language preference**: Rust's type system, ownership model, and exhaustive
  pattern matching make orchestration code easier to reason about and maintain
  long-term.  This is a legitimate motivation for the maintainer.
- **Stronger type safety** in the orchestration layer — compile-time prevention
  of data races, null panics, unhandled errors.
- **Faster request handling**: Microseconds instead of milliseconds for HTTP
  routing and job scheduling.  Irrelevant when jobs run for minutes, but makes
  the code feel snappier during development.
- **Lower server memory**: ~10 MB for Rust vs ~50 MB for Python server process.
  Irrelevant next to 4+ GB of loaded ML models.
- **Easier deployment** -- but only if Python is eliminated entirely (not yet
  feasible; see Stanza blocker below).

### What About Concurrency?

**Update (2026-02-23):** Free-threaded Python 3.14t has already solved the
concurrency problem on Net.  Validated results:

- 2.2x morphotag throughput at 6 threads with shared Stanza models
- 61 GB memory freed (8 ProcessPool workers × 8 GB → 1 ThreadPool × ~3 GB)
- Identical output verified against 3.12 ProcessPool on 10-file corpus
- Running in production on Net since 2026-02-19

The original concern (GIL forces ProcessPoolExecutor, duplicating model weights)
is now moot on 3.14t hosts.  The remaining concern — whether a Rust server
embedding Python via pyo3 would still contend on the GIL — is also moot,
because the recommended architecture is **not** embedded pyo3, but
HTTP/Unix-socket IPC to Python sidecar workers (see "IPC Cost Analysis" below).

**Rust-native inference** (ONNX / `candle`) would still deliver further gains:

- `Arc<Model>` shared across `tokio` tasks (`Send + Sync`)
- GGML quantization could cut model memory by 2-4x
- No Python interpreter overhead at all

But this is blocked by Stanza (see "Rust-Native Inference" section below) and
is not required for the Rust control-plane migration.

### What You'd Lose

- **Development time** for a rewrite that doesn't improve throughput (but may
  improve maintainability).
- **Deployment complexity**: Rust binary + Python sidecar venvs for ML engines +
  IPC protocol versioning.  The 3.14t/3.12 sidecar split already exists, so this
  adds a third runtime (Rust) to coordinate.  Most of this complexity is already
  present, though.
- **Proven reliability**: The current server has 620 passing tests, battle-tested
  crash recovery, and correct concurrency control.  These would need to be
  reimplemented and re-validated.

### Where Time Actually Goes (typical morphotag job)

```
Server HTTP handling:     <1 ms    (0.00%)
Job scheduling:           <1 ms    (0.00%)
CHAT parsing (Rust):      10 ms    (0.03%)
Stanza inference:         30,000 ms (99.9%)
CHAT serialization:       5 ms     (0.02%)
HTTP response:            <1 ms    (0.00%)
```

Rewriting the server in Rust optimizes the 0.03% that isn't ML inference.
The concurrency gains require replacing the inference itself.

### IPC Cost Analysis (added 2026-02-23)

The key question for a Rust control-plane + Python sidecar architecture:
how expensive is the Rust↔Python boundary?

**Mechanism:** HTTP or Unix socket to a long-running Python worker process.
The Python worker holds loaded models in memory.  Rust sends JSON payloads,
Python returns JSON results.

**Per-job overhead (morphotag, typical):**

| Step                          | Payload size   | Frequency | Overhead  |
|-------------------------------|---------------|-----------|-----------|
| Extract morphosyntax payloads | ~10-50 KB JSON | 1× / job  | ~1-2 ms   |
| Return Stanza results         | ~20-100 KB JSON| 1× / job  | ~2-3 ms   |
| **Total IPC overhead**        |               |           | **~3-5 ms**|
| **Stanza inference**          |               |           | **30,000 ms**|

The callback pattern is already batched — Rust collects all utterances, calls
Python once with the full batch.  At 0.01% overhead, the IPC tax is unmeasurable
in practice.

**Non-latency costs:**

- **Startup coordination**: Rust server waits for Python worker to load models
  (~30s warmup).  Already handled by health-check retries in current architecture.
- **Error propagation**: Python worker crashes mid-inference → Rust must detect,
  report, and optionally restart.  More failure modes than in-process, but
  the sidecar architecture already manages daemon lifecycle.
- **Model lifecycle**: Python worker must stay alive holding ~1-8 GB of loaded
  models.  This is what the current daemon already does.

**Conclusion:** The IPC tax is not a meaningful objection to the Rust
control-plane approach.  The deployment complexity is the only genuine cost,
and most of that complexity already exists in the sidecar architecture.

## When It Would Make Sense

### Near-term: Rust control-plane with Python sidecar (viable now)

A Rust server that delegates ML inference to Python sidecar workers via
HTTP/Unix socket IPC.  This is the approach described in the server orchestration
ADR (Stages 3-4).  The IPC cost is negligible (see analysis above).

This delivers: type safety, Rust's concurrency model for orchestration, and the
maintainer's language preference — without requiring any ML engine ports.

### Long-term: Full Rust (blocked by Stanza)

A Rust server becomes **self-contained** (no Python dependency) when all ML
inference runs natively in Rust.  The dependency chain would need:

| Current Python Library | Rust Replacement Needed        | Available Today? |
|------------------------|--------------------------------|------------------|
| Stanza                 | ONNX export + `ort` crate      | No (incomplete)  |
| Whisper                | `whisper-rs` (whisper.cpp)      | Yes (loses v3)   |
| Wav2Vec2 MMS           | ONNX export + `ort` or `candle`| Partial          |
| Pyannote               | No equivalent                  | No               |
| Seamless M4T           | No equivalent                  | No               |
| openSMILE              | FFI to C++ library             | Yes              |

**Blockers**: Stanza and Pyannote have no Rust equivalents and no official ONNX
exports.  Whisper v3 can run via whisper.cpp but with reduced multilingual
quality.  Until these gaps close, Python remains necessary for inference.

### The Compelling Endstate

Once all inference is Rust-native:

- **Single static binary**: `batchalign-next` ships as one file, no venv, no
  pip, no torch.  Deploy by copying a binary.
- **Startup in milliseconds**: No Python interpreter initialization, no model
  import chains.  Models loaded directly from ONNX/GGML files.
- **True concurrency**: No GIL.  Multiple jobs run on multiple cores with no
  serialization.
- **Memory efficiency**: Rust's ownership model + GGML quantization could cut
  model memory by 2-4x.
- **Cross-platform**: Compile for Linux x86, macOS ARM, Windows from one
  codebase.

This is worth pursuing long-term but is blocked by the ML ecosystem, not by
server architecture.

## Rust-Native Inference: Feasibility Assessment

Moving ML inference from Python to Rust is the prerequisite for both the server
rewrite and the concurrency gains discussed above.  Here is the current status
for each engine, assessed as of February 2026.

### ASR: Whisper — Production-Ready

**Rust path:** `whisper-rs` (bindings to `whisper.cpp`)

- whisper.cpp supports Whisper large-v2 and large-v3 with GGML quantization
- Metal acceleration on Apple Silicon (our lab Macs) is mature
- `whisper-rs` crate is well-maintained, 1,300+ GitHub stars
- Memory: GGML q5_0 quantization cuts model size from 3 GB → ~1.1 GB
- Quality: large-v2 via whisper.cpp matches HuggingFace Whisper closely;
  large-v3 turbo is available but has some multilingual quality regressions
  compared to the Python `transformers` implementation

**Verdict:** Ready for production.  Could replace `WhisperEngine` today as a
new `WhisperCppEngine` without touching the server.  Good candidate for a
near-term experiment.

### Forced Alignment: Wav2Vec2 MMS — Viable

**Rust path:** ONNX export + `ort` crate (ONNX Runtime Rust bindings)

- Wav2Vec2 MMS models export cleanly to ONNX (standard HuggingFace export)
- The `ort` crate wraps ONNX Runtime with full CoreML/Metal support on macOS
- Our DP alignment code (`utils/dp.py`) already has a Rust equivalent in
  `batchalign_core` — only the encoder inference is in Python
- Audio preprocessing (resampling, normalization) available via `rubato` and
  `ndarray` crates

**Verdict:** Viable with moderate effort.  Export the MMS model to ONNX, load
via `ort`, feed encoder output to existing Rust DP aligner.  Estimated 2-3
weeks of work.

### Morphosyntax: Stanza — **Blocked Indefinitely**

**Rust path:** Would require ONNX export of Stanza's full NLP pipeline
(tokenizer + MWT expander + POS tagger + lemmatizer + dependency parser)

- Stanza has an open issue for ONNX export (#893) filed in 2021 — **abandoned**,
  no activity since 2022
- Stanza uses custom PyTorch modules (CharRNN, Highway layers, biaffine
  attention) that don't map cleanly to standard ONNX operators
- The tokenizer uses a complex character-level model with hand-tuned
  dictionaries — not a simple export
- No alternative Rust NLP library provides equivalent multilingual coverage
  (40+ languages with lemmatization, POS, dependencies)
- `udpipe` (C++) has Rust FFI potential but uses UDPipe 2 models which are
  incompatible with Stanza's model format and have different linguistic
  conventions
- `candle` (Hugging Face's Rust ML framework) could theoretically run Stanza's
  architecture, but would require manually reimplementing every layer — months
  of work with no guarantee of identical output

**Verdict:** Not feasible.  Stanza is the permanent blocker for full Rust
migration.  No workaround exists short of building a complete NLP pipeline from
scratch in Rust, which would be a multi-year research project.

### Speaker Diarization: Pyannote — No Equivalent

**Rust path:** No Rust or C++ equivalent exists

- Pyannote uses a custom segmentation model + clustering pipeline
- No ONNX export path documented or maintained
- The clustering step (agglomerative + HDBSCAN) is algorithmically complex
  but could be reimplemented — the model inference cannot
- `wespeaker` (C++) covers speaker verification but not diarization

**Verdict:** Not feasible.  Blocked by model availability.

### Translation: Seamless M4T — No Equivalent

**Rust path:** No Rust bindings or ONNX export

- Meta's SeamlessM4T is PyTorch-only with a complex multi-modal architecture
- CTranslate2 supports some translation models but not SeamlessM4T
- `ctranslate2-rs` exists but targets OPUS-MT / NLLB, not Seamless

**Verdict:** Not feasible.  Could potentially switch to NLLB (which has
CTranslate2 support) but with quality tradeoffs.

### Summary Table

| Engine              | Rust Feasibility | Effort    | Blocker                     |
|---------------------|------------------|-----------|-----------------------------|
| Whisper ASR         | Ready            | 2-3 weeks | None                        |
| Wav2Vec2 FA         | Viable           | 2-3 weeks | None (ONNX export works)    |
| Stanza morphosyntax | **Blocked**      | N/A       | No ONNX export, no alt      |
| Pyannote diarize    | **Blocked**      | N/A       | No model export path        |
| Seamless translate  | **Blocked**      | N/A       | No Rust/ONNX support        |
| openSMILE features  | Ready            | 1 week    | FFI to existing C++ lib     |

**Bottom line:** ASR and forced alignment can move to Rust today.  Morphosyntax,
diarization, and translation cannot.  Since morphosyntax is by far the most
heavily used command (and the one with the concurrency bottleneck), the full
Rust migration is blocked at the most critical point.

## Free-Threaded Python: Deployed (no longer hypothetical)

**Update (2026-02-23):** Free-threaded Python 3.14t is deployed and validated
in production on Net since 2026-02-19.  This section is retained for historical
context but the key results are now confirmed.

### Production Results

- **2.2x morphotag throughput** at 6 threads with shared Stanza models
- **61 GB memory freed**: 8 ProcessPool workers (8×8 GB) → ThreadPool (1×~3 GB)
- **Identical output**: 10-file corpus regression confirms 3.14t ThreadPool
  produces byte-identical CHAT to 3.12 ProcessPool
- **Thread safety**: Stanza empirically thread-safe (no crashes, no data races,
  stable since 2026-02-19)
- **Runtime detection**: `batchalign/runtime.py` `FREE_THREADED` flag auto-selects
  ThreadPool vs ProcessPool

### Remaining Limitation

Transcribe is blocked on 3.14t because openai-whisper depends on numba/llvmlite,
which have no cp314t wheels.  This is handled by the 3.12 sidecar venv
(`~/.batchalign3/sidecar/.venv/`).

### Free-Threaded Python vs Rust Control-Plane

These are **not mutually exclusive**.  Free-threaded Python solves the
concurrency/memory problem for ML inference workers.  A Rust control-plane
solves the type safety and language preference problem for orchestration.
The recommended path uses both: Rust control-plane + free-threaded Python
sidecar workers.

| Dimension              | Free-Threaded Python        | Rust Control-Plane          |
|------------------------|-----------------------------|-----------------------------|
| Concurrency fix        | Yes (no GIL in workers)     | N/A (workers still Python)  |
| Memory savings         | Yes (shared models)         | N/A (workers still Python)  |
| Type safety            | No                          | Yes (orchestration layer)   |
| Language preference    | No                          | Yes                         |
| Deployment simplicity  | Same as today               | Slightly more complex       |
| Development effort     | Done (deployed)             | Weeks-months                |

## CLI Migration to Rust (added 2026-02-23)

The CLI (~3,500 lines of `rich-click`) is pure orchestration — no ML imports.
It is a stronger candidate for Rust migration than the server itself.

### Why the CLI benefits most

**Startup time:** Every `batchalign3 --help` pays ~3s for Python interpreter
init + lazy import chains.  A Rust CLI with `clap` starts in <100ms.  This is
the most user-visible improvement Rust could deliver.

**Component mapping:**

| Python (current)             | Rust equivalent         | Status      |
|------------------------------|-------------------------|-------------|
| `rich-click` arg parsing     | `clap`                  | Mature      |
| File discovery               | `walkdir`               | Mature      |
| HTTP dispatch to server      | `reqwest` + `tokio`     | Mature      |
| Fleet health-check + routing | `reqwest` + `tokio`     | Mature      |
| Daemon lifecycle management  | `std::process` + `nix`  | Mature      |
| Progress display             | `indicatif`             | Mature      |
| CHAT I/O                     | `batchalign_core`       | Already Rust|
| Run logging (JSONL)          | `serde_json`            | Mature      |

All of these are crates already used in `talkbank-chatter` / `talkbank-clan`.

### Shared code concern

The CLI currently shares code with the server: `dispatch_common.py`,
`file_io.py`, auto-tuning logic.  If the CLI moves to Rust but the server stays
Python, this shared logic gets duplicated.  The right sequencing avoids this:

1. Build Rust control-plane first (server orchestration)
2. CLI becomes a thin Rust binary that talks to the Rust control-plane
3. Shared logic lives once, in Rust

### What the CLI does NOT need

- ML model loading (handled by Python workers)
- Pipeline instantiation (handled by server/daemon)
- CHAT parsing beyond basic validation (already in `batchalign_core`)

## Sidecar Simplification via Rust Control-Plane (added 2026-02-23)

The 3.14t/3.12 sidecar architecture is the most painful part of the current
deployment.  A Rust control-plane would significantly simplify it.

### Current sidecar complexity

The sidecar exists because 3.14t can't run transcribe (numba/llvmlite have no
cp314t wheels).  This creates:

1. Two daemon processes per host (main 3.14t + sidecar 3.12)
2. Ad-hoc capability detection scattered across `daemon.py`, `dispatch.py`,
   `dispatch_server.py`
3. Three-wheel build system (pure Python + Rust core for 3.14t + Rust core for
   3.12)
4. Deploy scripts managing two Python versions per host
5. `daemon.json` + `sidecar` profile definitions + health-check routing

### How Rust simplifies this

A Rust control-plane becomes the single process you deploy and manage:

```
Rust control-plane (axum, single binary)
  ├── /health — reports aggregate capabilities from all workers
  ├── /jobs — accepts jobs, routes by command capability
  ├── Worker registry
  │     ├── Python 3.14t worker (morphotag, align, utseg, coref, translate)
  │     └── Python 3.12 worker (transcribe, benchmark)
  └── Worker lifecycle (spawn, health-check, restart)
```

**Key simplification:** Python workers become implementation details — spawned
and managed by the Rust process, not user-facing daemons.  No more
`daemon.json`, no more sidecar profile detection, no more "which daemon am I
talking to?" logic in the CLI.

**Deploy becomes:** Copy one Rust binary + ensure two Python venvs exist.  The
Rust binary manages everything else.

**The HTTP/socket approach is essential here.** Embedded pyo3 would pin the Rust
process to one Python version, making the 3.14t/3.12 split harder.  With
HTTP/socket IPC, the Rust process doesn't care what Python version its workers
run — it just sends JSON and receives JSON.

### What stays the same

- Three-wheel build (still need Rust core for both Python versions)
- Two Python venvs per host (still need 3.14t for morphotag, 3.12 for
  transcribe)
- ML model loading in Python workers (unchanged)

### What gets better

| Aspect                    | Current (Python)              | With Rust control-plane      |
|---------------------------|-------------------------------|------------------------------|
| Process management        | Ad-hoc daemon.py + sidecar    | Single Rust binary owns all  |
| Capability routing        | Scattered across 3 files      | Centralized worker registry  |
| Health reporting          | Per-daemon /health             | Aggregate /health            |
| CLI → server communication| Python HTTP client (3s startup)| Rust HTTP client (<100ms)    |
| Failure detection         | PID files + stale checks      | Rust manages child processes |
| Deploy artifact           | 3 wheels + 2 venvs + scripts  | 1 binary + 2 venvs           |

### Migration priority assessment

| Component              | Rust value | Reason                                    |
|------------------------|------------|-------------------------------------------|
| CLI                    | **High**   | 3s → <100ms startup, pure orchestration   |
| Sidecar/worker routing | **High**   | Single binary owns lifecycle, cleaner     |
| Server orchestration   | Medium     | Type safety, language preference          |
| ML engines             | Blocked    | Stanza, no alternative                    |

The CLI and sidecar routing are stronger candidates than the server HTTP layer
itself.  The server is "just" a web framework; the CLI and worker management are
where Python's startup cost and ad-hoc process management hurt the most.

## Plugin Architecture Compatibility (added 2026-02-23)

The plugin system (`batchalign/plugins.py`) uses `importlib.metadata.entry_points`
to discover third-party engines, CLI commands, and task metadata at runtime.
This is a Python-only mechanism.  A Rust migration must account for it.

### What the plugin system provides

Plugins contribute via a `PluginDescriptor` dataclass:

- **Engines** — new pipeline engines (e.g., a custom ASR backend)
- **CLI commands** — new subcommands (e.g., `batchalign3 mytask`)
- **Task metadata** — command-to-task mappings, memory budgets, capability probes
- **Default overrides** — change which engine handles a built-in task

Discovery happens via standard Python packaging (`pyproject.toml` entry points):

```toml
[project.entry-points."batchalign.plugins"]
my_plugin = "my_package:plugin_descriptor"
```

### Impact on the Rust control-plane (server): none

The Rust control-plane doesn't need to know about plugins.  It routes jobs by
command name (an opaque string) to Python workers.  The Python workers discover
and load plugins themselves via `importlib.metadata`.  The Rust binary just sees
"morphotag", "transcribe", "my_custom_task" as command strings and routes them
to the appropriate worker based on capability.

Plugin-contributed engines run inside the Python worker process — they're
standard Python classes that the worker instantiates.  The Rust control-plane
never imports or inspects them.

### Impact on the Rust CLI: requires a capability query

If the CLI is Rust, it cannot use `importlib.metadata` to discover
plugin-contributed commands.  Three options:

**Option 1: `/capabilities` endpoint (recommended)**

The Rust CLI queries the running daemon for its command registry:

```
GET /capabilities → {
  "commands": ["morphotag", "align", "transcribe", "mytask", ...],
  "command_metadata": {
    "mytask": {"task": "my_task", "base_mb": 4096, ...}
  }
}
```

The daemon reports all commands including plugin-contributed ones.  The Rust CLI
builds its argument handling dynamically.

Advantages:
- Plugin authors don't need to know the CLI is Rust
- No Python subprocess at startup
- Natural fit: the CLI already needs to talk to a daemon/server
- The daemon already has the full registry loaded

Disadvantage:
- `batchalign3 --help` needs a running daemon (or falls back to built-in
  commands only).  This is acceptable because the daemon auto-starts on first
  use anyway.

**Option 2: Python shim for discovery only**

Shell out to `python -c "from batchalign.plugins import ..."` at startup.
Adds ~500ms for the Python call.  Works without a daemon but partially defeats
the purpose of the Rust CLI's fast startup.

**Option 3: No plugin support in Rust CLI**

Plugin commands only work via the Python CLI (`batchalign3-py`).  Simple but
limits plugin usefulness.

**Recommendation:** Option 1.  The `/capabilities` endpoint is trivial to
implement (the daemon already knows its full registry), and it cleanly separates
Python-side discovery from Rust-side presentation.  Built-in commands are always
available even without a daemon; plugin commands appear once the daemon starts.

### Summary: plugin system and Rust migration

| Layer              | Plugin impact | Action needed                          |
|--------------------|---------------|----------------------------------------|
| Rust control-plane | None          | Routes opaque command strings          |
| Python workers     | None          | Discover and load plugins as today     |
| Rust CLI           | Moderate      | Add `/capabilities` endpoint to daemon |
| Plugin authors     | None          | Standard `pyproject.toml` entry points |

The plugin architecture is compatible with Rust migration.  The key design
decision — using `importlib.metadata` rather than a Rust-side registry — is
correct because plugins are Python packages contributing Python engines.  The
Rust CLI just needs one HTTP call to learn what commands are available.

## Distribution and Installation (added 2026-02-23)

### Guiding Principle

**One installation method for everyone.**  `uv tool install batchalign3` must
work, and all commands must be available.  There is no distinction between
"fleet" and "external" users.  Behavior differences (like free-threaded
acceleration) are controlled by config, not by identity.

### What ships today

```bash
uv tool install batchalign3
batchalign3 morphotag corpus/ output/    # works immediately
```

The user needs only `uv` (or `pip`).  Under the hood:

1. Pure Python wheel (CLI, server, engines, plugins)
2. `batchalign_core` Rust wheel (CHAT parsing, matched to user's Python version)
3. One Python venv (whatever version `uv` selects: 3.12, 3.13, etc.)

All commands work.  No configuration required.

### What ships after Rust migration

```bash
uv tool install batchalign3              # same command
batchalign3 morphotag corpus/ output/    # Rust binary, <100ms startup
```

The user still needs only `uv`.  Under the hood:

1. Rust binary wheel (CLI + control-plane, per-platform via maturin)
2. Python engine code (in the same wheel or a companion wheel)
3. `batchalign_core` Rust wheel (still needed by Python engines for CHAT ops)
4. One Python venv (same as today)

The Rust binary is packaged as a Python wheel — the same pattern used by `ruff`
and `uv` themselves.  Maturin builds the Rust binary into a wheel with a console
script entry point.  `uv tool install` puts the binary in `~/.local/bin/`.  The
user never knows Rust is involved.

All commands work on the user's existing Python (3.12+).  No second Python
required.  No sidecar.  No configuration.

### Free-threaded acceleration: opt-in, one-time prompt

The 3.14t/3.12 sidecar split is **not a user concern**.  It's an optional
performance optimization.  The default installation uses whatever Python the
user has (3.12+), and all commands work.

Free-threaded Python 3.14t gives 2.2x faster morphotag/align/utseg via shared
model threading.  To offer this without requiring users to know about it, the
Rust binary prompts once on first CPU-heavy command:

```
$ batchalign3 morphotag corpus/ output/
  Free-threaded Python available for 2.2x faster processing.
  Install now? [y/N]: y
  Installing Python 3.14t via uv...
  Done. Using free-threaded acceleration.
  Processing 42 files...
```

Behavior:

- **Interactive terminal, first CPU-heavy command, no free-threaded Python
  configured**: Show prompt once.  If accepted, run `uv python install 3.14t`,
  write `free_threaded_python: <path>` to `~/.batchalign3/config.yaml`.
- **User says no**: Write `free_threaded_prompted: true` to config.  Never ask
  again.  All commands still work at normal speed.
- **Non-interactive** (piped stdin, `--quiet`, running as daemon): Skip prompt
  silently.  Use default Python.
- **Free-threaded Python already detected** (user's system Python is 3.14t+):
  Use it automatically, no prompt needed.  This is what `runtime.py` already
  does.
- **Fleet deployment** (Ansible): Pre-installs 3.14t and writes config.  Prompt
  never fires.

This means:

- **Default path**: Zero config.  Everything works.
- **Accelerated path**: One-time prompt, one "y" keystroke.  Or Ansible for
  fleet.
- **No mandatory `setup` command**.  No separate install steps.  No "fleet vs
  user" distinction.

### How the Rust binary manages Python workers

When the Rust binary needs to run a pipeline command:

1. Check for configured `free_threaded_python` in config → if present and the
   command is compatible (morphotag, align, utseg, coref), spawn worker on
   that interpreter.
2. Otherwise, find the default Python from the venv the binary was installed
   into (standard `sys.prefix` equivalent — the venv `uv tool install` created).
3. Spawn a Python worker process on that interpreter.
4. If the configured free-threaded Python can't run a command (e.g., transcribe
   needs numba), fall back to the default Python automatically.

This is the same logic as today's `runtime.py`, but owned by the Rust binary
instead of Python code.

### Build matrix

| Artifact | Today | After Rust migration |
|----------|-------|---------------------|
| Pure Python wheel | 1 (py3-none-any) | 0 (code moves into binary wheel) |
| Rust binary wheel | 0 | 1 per platform (macOS ARM, Linux x86) |
| batchalign_core wheel | 1 per Python version | 1 per Python version (unchanged) |
| Total wheels on PyPI | ~3 | ~4 |

Platform wheels are standard — maturin + GitHub Actions CI builds for each
target.  PyPI serves the right wheel per platform automatically.

### Genuine new complexity

| Concern              | Today                            | After Rust migration                       |
|----------------------|----------------------------------|--------------------------------------------|
| Build matrix         | 3 wheels (pure Python + 2 core)  | ~4 wheels (platform binary + core per Python) |
| Platform coverage    | macOS ARM only (our lab)         | Need macOS ARM + Linux x86 minimum         |
| Worker venv mgmt     | `uv tool install` handles it     | Rust binary must find Python in its own venv |
| Plugin discovery     | `importlib.metadata` in-process  | `/capabilities` endpoint (see above)       |
| Debugging            | One language, one stack trace    | Rust binary + Python worker, two traces    |

The worker venv management is the trickiest part.  The Rust binary needs to know
where Python is, where the engines are installed, and how to spawn workers with
the right environment.  This is solvable — `uv` itself is a Rust binary that
manages Python venvs — but it is real engineering work.

## Recommendation (revised 2026-02-23)

1. **One installation for everyone.**  `uv tool install batchalign3` must work
   for all users with all commands.  No fleet-vs-external distinction.
   Free-threaded acceleration is opt-in via one-time prompt, not a separate
   install path.
2. **Proceed with staged Rust migration**, prioritizing in this order:
   a. **Rust control-plane** (server orchestration + worker lifecycle) — per ADR
      Stages 3-4.  This is the foundation: it owns Python worker processes,
      handles job routing, and provides the IPC contract.
   b. **Rust CLI** — once the control-plane exists, the CLI becomes a thin
      `clap` binary that talks to it.  Delivers the most user-visible win
      (3s → <100ms startup).
   c. **Sidecar simplification** — falls out naturally from (a): the Rust
      control-plane replaces the ad-hoc daemon/sidecar management.
3. **Ship as a Python wheel** (maturin binary + engine code) following the
   `ruff`/`uv` pattern.  Same `uv tool install` command as today.
4. **Use HTTP/Unix socket IPC** to Python workers — not embedded pyo3.  This is
   essential for the free-threaded split: the Rust binary must be Python-version
   agnostic.
5. **Free-threaded acceleration is opt-in.**  Default Python (3.12+) runs all
   commands.  One-time interactive prompt offers to install 3.14t for 2.2x
   morphotag throughput.  Fleet pre-configures via Ansible.
6. **Continue moving CHAT processing into `batchalign_core`** (already in
   progress).  This is where Rust delivers real value: correctness, speed, and
   type safety for AST manipulation.
7. **Consider whisper.cpp** as a near-term experiment for ASR-only workloads.
   A `WhisperCppEngine` alongside the existing `WhisperEngine` would reduce
   memory, improve startup time, and validate the Rust inference path — all
   without touching the server.
8. **Consider ONNX export for Wav2Vec2 MMS** forced alignment.  The encoder
   is the only Python piece; the DP aligner is already Rust.  Moderate effort,
   clear payoff.
9. **Monitor the Rust ML ecosystem** for Stanza/Pyannote ONNX exports and
   `candle`/`ort` maturity.  A fully self-contained Rust binary (no Python)
   remains the ideal endstate but is blocked indefinitely by Stanza.
