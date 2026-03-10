# Rust Migration Report: Minimal Python Core Architecture

> **Update (2026-03):** Server migration to axum is complete. The "Current
> Priority" section later in this document reflects Feb 2026 state. Job DB
> now uses `sqlx` (not `rusqlite`). CHAT divorce phases 0–6 are done — the
> Rust server owns CHAT parsing, caching, and serialization for all
> text-only commands.

## Completed Migration: Python to Rust CHAT Handling

Between February 1-15, 2026, a two-week sprint migrated all CHAT parsing,
serialization, and NLP orchestration from Python to Rust. This section documents
what shipped; the remainder of this document proposes future work.

**What was built:**
- Two independent CHAT parsers (tree-sitter + Chumsky) producing the same AST,
  with 100% agreement on 163 curated tests and 500 random corpus files
- `batchalign-core` PyO3 crate integrating the Rust parsers into the Python pipeline
- All NLP engines (morphosyntax, forced alignment, utterance segmentation,
  translation) migrated to handle-based Rust orchestration with Python callbacks
- Deployed to production (Net server + 5 client Macs) on February 18, 2026

**Architecture: Rust orchestrates, Python provides NLP.** The CHAT file is parsed
once into a `ParsedChat` handle (Rust AST), mutated in-place by each engine via
Rust methods that call Python callbacks for ML inference (Stanza, Whisper, etc.),
and serialized once at pipeline exit. No intermediate serialize/reparse cycles.

**January 2026 -- Python optimization phase (before Rust):** Lazy imports cut CLI
startup from 60s to 3s. ProcessPoolExecutor added for file-level parallelism.
Hirschberg DP algorithm replaced O(N^2) alignment. SQLite utterance cache added.
Shared model loading attempted but failed (MPS + fork incompatibility on macOS).

**Four server architecture false starts:**
1. Ray distributed processing (2 days, abandoned -- high overhead, MPS incompatible)
2. Celery + Redis (1 day, abandoned -- external dependency, thundering-herd model loads)
3. Temporal.io (4 hours, abandoned -- 4MB gRPC message limit, operational complexity)
4. ThreadPoolExecutor + SQLite (shipped -- no external dependencies, crash recovery)

**The %gra bug that went undetected for 2+ years:** Python master's 615 pytest tests
all passed, yet 53.6% of the production corpus had corrupted dependency trees from an
array wraparound bug in `ud.py:492`. Tests used synthetic data; the bug only manifested
on real corpus files with skipped words. The corpus-wide validation audit (99,063 files,
20.5M errors) was the first time anyone checked production output at scale.

**Lessons learned:**
1. Test production data, not just unit tests -- corpus audits caught what 615 tests missed
2. String manipulation is fundamentally wrong for complex formats -- use parsers and ASTs
3. Fail fast beats fail silently -- Rust panics before serializing invalid data;
   Python silently corrupted data for years
4. Performance and correctness come from the same source: good architecture
   (zero-reparse pipelines, batched callbacks, GIL release) delivers both

---

## Executive Summary

Batchalign's codebase is 33,348 lines of Python. Of that, **only ~5,400 lines
(16%) actually require Python** — the ML inference wrappers around PyTorch, Stanza,
and related frameworks. The remaining **~28,000 lines (84%) are pure logic**: CLI,
server, CHAT parsing, job orchestration, caching, file I/O, dispatch, utilities.
All of that could move to Rust.

This report describes an architecture where Rust owns everything except a thin
Python ML inference layer, and what that buys us.

---

## Current Codebase Breakdown

### Lines that need Python (ML inference): ~5,400

| Component | Lines | ML Libraries |
|-----------|-------|-------------|
| `models/whisper/` (ASR inference) | 446 | torch, transformers |
| `models/wave2vec/` (FA inference) | 132 | torch, torchaudio |
| `models/utterance/` (segmentation) | 474 | torch, transformers |
| `models/speaker/` (diarization) | 134 | torch, nemo_toolkit |
| `models/audio_io.py` | 194 | torch, torchaudio |
| `models/utils.py` (DTW, timestamps) | 317 | torch, transformers |
| `pipelines/morphosyntax/ud.py` | 1,230 | stanza |
| `pipelines/fa/wave2vec_fa.py` | 209 | torch, torchaudio |
| `pipelines/fa/whisper_fa.py` | 234 | torch, transformers |
| `pipelines/asr/whisper.py` | 76 | torch, transformers |
| `pipelines/asr/whisperx.py` | 228 | torch, whisperx |
| `pipelines/asr/oai_whisper.py` | 97 | whisper (openai) |
| `pipelines/diarization/pyannote.py` | 85 | pyannote.audio |
| `pipelines/utterance/ud_utterance.py` | 383 | stanza, transformers |
| `pipelines/opensmile/engine.py` | 156 | opensmile |
| `pipelines/avqi/engine.py` | 321 | parselmouth, torchaudio |
| `pipelines/translate/seamless.py` | 73 | transformers |
| `pipelines/morphosyntax/coref.py` | 42 | stanza |

### Lines that do NOT need Python: ~28,000

| Component | Lines | What it does |
|-----------|-------|-------------|
| **CLI** (`cli/`) | 3,572 | Command parsing, dispatch, logging |
| **Server** (`serve/`) | 3,127 | FastAPI, job store, SQLite, WebSocket |
| **Formats** (`formats/`) | 1,436 | CHAT parser, TextGrid parser, generators |
| **Document model** | 614 | `Document`, `Utterance`, `Form`, `Task` enum |
| **Pipeline infra** (`pipeline.py`, `cache.py`, etc.) | 1,244 | Pipeline orchestration, SQLite cache |
| **Pure-logic engines** | 2,266 | Retrace detection, WER eval, num2word, etc. |
| **Utils** | 7,729 | DP aligner, media cache, config, names list |
| **Tests** | 6,908 | Entire test suite |
| **Language-specific rules** (irr.py, verbforms, etc.) | 876 | Irregular verbs, verb forms, French rules |

---

## The Minimal Python Core

After migration, the Python codebase reduces to **~300 lines** — a set of thin
model wrappers that each do exactly one thing: load a model and run inference.

```python
# batchalign_models/morphosyntax.py (~40 lines)
import stanza

_cache: dict[tuple, stanza.Pipeline] = {}

def analyze(words: list[str], lang: str) -> list[dict]:
    """Run Stanza on a word list. Return MorAnalysis dicts."""
    key = (lang,)
    if key not in _cache:
        _cache[key] = stanza.Pipeline(lang, processors="tokenize,pos,lemma,depparse",
                                       tokenize_pretokenized=True)
    doc = _cache[key]([words])
    return _extract_analyses(doc.sentences[0])

def _extract_analyses(sentence) -> list[dict]:
    # ~20 lines: extract pos, lemma, feats, head, deprel per token
    ...
```

```python
# batchalign_models/asr.py (~40 lines)
import torch
from transformers import WhisperForConditionalGeneration, WhisperProcessor

_model = None

def transcribe(audio_path: str, lang: str) -> list[dict]:
    """Run Whisper. Return [{text, start_s, end_s}, ...]."""
    global _model
    if _model is None:
        _model = _load_model()
    # ~20 lines: load audio, run inference, extract word timestamps
    ...
```

```python
# batchalign_models/forced_alignment.py (~40 lines)
import torch
import torchaudio

_model = None

def align(audio_path: str, start_ms: int, end_ms: int,
          words: list[str]) -> list[tuple[int, int]]:
    """Run wav2vec2 CTC forced alignment. Return per-word (start_ms, end_ms)."""
    # ~25 lines: load audio segment, run CTC, extract per-word timing
    ...
```

```python
# batchalign_models/diarize.py (~30 lines)
from pyannote.audio import Pipeline

_pipeline = None

def diarize(audio_path: str) -> list[dict]:
    """Run pyannote. Return [{speaker, start_s, end_s}, ...]."""
    ...
```

```python
# batchalign_models/features.py (~30 lines)
import opensmile

def extract(audio_path: str, feature_set: str) -> bytes:
    """Run OpenSMILE. Return CSV bytes."""
    ...
```

```python
# batchalign_models/utterance_seg.py (~40 lines)
import torch
from transformers import BertForTokenClassification

def segment(text: str, lang: str) -> list[int]:
    """Run BERT utterance segmenter. Return split indices."""
    ...
```

```python
# batchalign_models/translate.py (~30 lines)
from transformers import SeamlessM4Tv2ForTextToText

def translate(text: str, src_lang: str, tgt_lang: str) -> str:
    """Run SeamlessM4T translation."""
    ...
```

**Total: ~250-300 lines of Python.** That is the irreducible ML core — the part
that cannot be written in any other language because the models, weights, and
inference frameworks are Python-only.

Everything above the model wrappers — caching, retry logic, batch orchestration,
progress reporting, text cleaning, clitic handling, GRA index computation, gap-fill,
grouping, CHAT parsing, serialization, CLI, server — moves to Rust.

---

## Architecture: How Rust and Python Connect

### Option A: PyO3 Embedded Python (Recommended)

The Rust binary embeds a Python interpreter via PyO3. When ML inference is needed,
it calls into the thin Python wrappers.

```
┌─────────────────────────────────────────────────────┐
│  batchalign (single Rust binary)                     │
│                                                      │
│  CLI (clap)                                          │
│  Server (axum + tokio)                               │
│  CHAT parser (talkbank-model)                        │
│  Job orchestration                                   │
│  Caching (rusqlite)                                  │
│  File I/O, config, logging                           │
│  DP alignment (native Rust)                          │
│  Mor/GRA builder, gap-fill, grouping                 │
│  Dashboard (static files + WebSocket)                │
│                                                      │
│  ┌─────────────────────────────────────────────┐     │
│  │  Embedded Python (PyO3)                      │     │
│  │                                              │     │
│  │  batchalign_models/                          │     │
│  │    morphosyntax.py  (stanza)                 │     │
│  │    asr.py           (whisper)                │     │
│  │    forced_align.py  (wav2vec2)               │     │
│  │    diarize.py       (pyannote)               │     │
│  │    features.py      (opensmile)              │     │
│  │    utterance_seg.py (bert)                   │     │
│  │    translate.py     (seamlessm4t)            │     │
│  │                                              │     │
│  │  ~300 lines total                            │     │
│  └─────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────┘
```

**How it works:**

1. Rust binary starts instantly (~10ms)
2. CLI parses arguments, discovers files, validates input — all in Rust
3. When ML is needed, Rust initializes the Python interpreter (once, ~500ms)
4. Rust calls `batchalign_models.morphosyntax.analyze(words, lang)` via PyO3
5. Python loads the model (first call only), runs inference, returns structured data
6. Rust takes the results, builds Mor/GRA/timing on the AST, serializes CHAT
7. Python interpreter stays alive for subsequent calls (model stays cached)

**Dependency:** The machine still needs Python + PyTorch + Stanza installed.
But this is managed by the Rust binary itself — on first run, it can auto-create
a venv and install `batchalign-models` (the tiny pip package). The user never
interacts with Python directly.

### Option B: Python Subprocess via IPC

Rust spawns a long-lived Python worker process. Communication via Unix socket or
stdin/stdout with JSON/MessagePack framing.

```
┌────────────────────────┐         IPC          ┌──────────────────────┐
│  Rust binary            │ ◄──────────────────► │  Python ML worker    │
│  (CLI, server, parsing) │   JSON/msgpack       │  (~300 lines)        │
│                         │   over Unix socket   │  stanza, whisper,    │
│                         │                      │  wav2vec2, pyannote  │
└────────────────────────┘                      └──────────────────────┘
```

**Pros over PyO3:**
- Clean process isolation — Python crash doesn't take down Rust
- Can run multiple Python workers for parallelism
- Easier to debug (separate processes)
- Python worker can be on a different machine (network IPC)

**Cons:**
- Serialization overhead (though minimal — word lists are small)
- Process management complexity
- Startup latency for spawning the worker

### Option C: HTTP Model Server (for client/server mode)

This is an extension of the existing architecture. The Rust binary IS the client.
The server runs Python for ML.

```
┌──────────────────────┐    HTTP    ┌──────────────────────────┐
│  Rust client binary   │ ────────► │  Server (Rust + Python)   │
│  (pure Rust, no deps) │           │  axum HTTP + PyO3 ML      │
│                       │ ◄──────── │                           │
│  Parses CHAT locally  │   CHAT    │  Runs inference           │
│  Sends to server      │   text    │  Returns processed CHAT   │
└──────────────────────┘           └──────────────────────────┘
```

**Key insight:** The CLIENT becomes a zero-dependency Rust binary. No Python, no
PyTorch, no venv. Researchers install a 15MB binary and point it at their lab's
server. This is the dream UX for non-technical users.

**Recommended: Use all three.** PyO3 for local mode, IPC for worker parallelism,
HTTP for client/server. Same Rust binary, different execution modes.

---

## What Rust Owns: Component by Component

### CLI → clap (replaces rich-click)

| Aspect | Python (current) | Rust |
|--------|-----------------|------|
| Startup | ~3 seconds (lazy imports) | ~10 milliseconds |
| Binary size | ~200MB (venv) | ~15MB (static binary) |
| Argument parsing | rich-click | clap (derive macros) |
| Progress bars | rich.progress | indicatif |
| Colored output | rich.console | colored / owo-colors |
| Tab completion | click built-in | clap_complete |

**Gain:** Instant startup. Users type `batchalign morphotag` and it responds
immediately, not after 3 seconds of Python initialization. This matters for
interactive use and scripting.

### Server → axum + tokio (replaces FastAPI + uvicorn)

| Aspect | Python (current) | Rust |
|--------|-----------------|------|
| HTTP framework | FastAPI | axum |
| Async runtime | asyncio + threading | tokio |
| WebSocket | FastAPI WebSocket | axum + tokio-tungstenite |
| Request throughput | ~200 req/s | ~50,000 req/s |
| Memory per connection | ~1MB | ~8KB |
| Process model | Single uvicorn process | Single tokio runtime |

**Gain:** Not throughput (the server handles ~10 req/hour). The real gain is
**architectural simplicity**: tokio's async model is cleaner than Python's
asyncio + ThreadPoolExecutor + threading.Lock hybrid. No more "is this sync or
async?" confusion. No more `threading.Semaphore` for concurrency control — use
tokio::sync::Semaphore with proper async awareness.

### Job Store → native Rust (replaces in-memory dict + threading.Lock)

| Aspect | Python (current) | Rust |
|--------|-----------------|------|
| Concurrency | threading.Lock | tokio::sync::RwLock |
| Job DB | sqlite3 | rusqlite |
| Worker pool | ThreadPoolExecutor | tokio::task::spawn_blocking |
| Memory | ~50MB baseline | ~5MB baseline |

**Gain:** Lower memory, safer concurrency (Rust's ownership prevents data races
at compile time), and the job store can be truly async without blocking threads.

### CHAT Parsing → talkbank-model (replaces formats/chat/)

Already done. The Rust parser (talkbank-model) is more correct, faster, and
produces a typed AST with validation. The Python parser (1,436 lines) disappears
entirely.

**Gain:** The embedded annotation model we designed. Word owns its morphology,
timing, and phonological annotation. No separate tier structures, no post-hoc
alignment. CHAT parsing that catches errors the Python parser silently ignores.

### File Dispatch → native Rust (replaces dispatch_local.py, dispatch_pipeline.py)

| Aspect | Python (current) | Rust |
|--------|-----------------|------|
| Process pool | ProcessPoolExecutor | tokio + rayon |
| Worker init | Each worker loads Python + models | Workers share model via PyO3 |
| File sorting | sorted() | rayon parallel sort |
| Progress | status_hook callback | tokio::sync::watch channel |

**Gain:** Workers can potentially share a single model instance across threads
(if the model supports it via PyO3's GIL management). Even without model sharing,
Rust's thread pool has lower per-worker overhead than Python's ProcessPoolExecutor
(no interpreter copy).

### Caching → rusqlite (replaces pipelines/cache.py)

| Aspect | Python (current) | Rust |
|--------|-----------------|------|
| SQLite | sqlite3 module | rusqlite |
| Key gen | hash(text + lang + ...) | blake3 hash |
| Thread safety | per-thread connections | rusqlite + tokio |
| Lines of code | 804 | ~200 (estimated) |

**Gain:** Faster hashing (blake3 vs Python hashlib), lower overhead, same
functionality. The cache logic (check before calling ML, store after) moves to
the Rust orchestration layer.

### DP Alignment → native Rust (replaces utils/dp.py)

Already designed. ~200x speedup (3ns/cell vs 500ns/cell). The 40-second UTR
alignment becomes 0.3 seconds.

**Gain:** The single largest performance improvement in the entire migration.
UTR on large files goes from "get coffee" to "instant."

### Audio I/O → symphonia (replaces torchaudio/pydub for non-ML uses)

| Aspect | Python (current) | Rust |
|--------|-----------------|------|
| MP3 decode | pydub (ffmpeg subprocess) | symphonia (pure Rust) |
| WAV read/write | torchaudio or wave | hound (pure Rust) |
| Format detect | file extension | symphonia probe |
| Resample | torchaudio | rubato (pure Rust) |

**Gain:** No ffmpeg dependency for basic audio operations. Users don't need to
install ffmpeg separately. The ML models still use torchaudio for their specific
preprocessing, but file-level operations (detect format, check duration, convert
MP4→WAV for the media cache) become pure Rust.

### Configuration → serde (replaces utils/config.py)

| Aspect | Python (current) | Rust |
|--------|-----------------|------|
| YAML parsing | PyYAML | serde_yaml |
| Config struct | dict / dataclass | typed struct with serde |
| Validation | runtime checks | compile-time types + serde validation |

**Gain:** Config errors caught at parse time with clear messages, not at runtime
deep in a pipeline.

### Logging → tracing (replaces run_log.py)

| Aspect | Python (current) | Rust |
|--------|-----------------|------|
| Structured logging | Custom JSONL writer | tracing + tracing-subscriber |
| Log levels | Python logging | tracing spans + events |
| Performance | ~1us/event | ~50ns/event |

**Gain:** tracing's span model naturally captures the hierarchical structure
(run → file → utterance → engine) that `run_log.py` builds manually.

### Number-to-Word → native Rust (replaces asr/num2lang/)

1,206 lines of pure Python (13 language converters). These are lookup tables
and formatting rules — trivial to port, no ML dependency.

**Gain:** These run during ASR post-processing on every word. Rust makes them
effectively zero-cost. Also eliminates 1,206 lines of Python.

### WER Evaluation → native Rust (replaces analysis/eval.py)

300 lines of pure Python doing edit-distance alignment on transcripts. Uses the
DP aligner and name/abbreviation lists.

**Gain:** Faster evaluation, especially on large corpora. The 6,684-line names
list (`names.py`) and 425-line abbreviation list (`abbrev.py`) become compiled
into the binary.

---

## What This Buys Us

### 1. Distribution: Single Binary + Auto-Managed Python

**Before:**
```bash
# Current installation process
pip install uv                    # or brew install uv
uv tool install batchalign3   # builds wheel, resolves deps
# Hope Python 3.10+ is installed
# Hope no dependency conflicts
# Hope torch finds the right CUDA version
```

**After (local mode):**
```bash
curl -fsSL https://batchalign.talkbank.org/install.sh | sh
batchalign setup  # auto-creates venv, installs batchalign-models (once)
batchalign morphotag input/ output/
```

**After (client mode — the dream):**
```bash
curl -fsSL https://batchalign.talkbank.org/install.sh | sh
batchalign morphotag input/ output/ --server http://lab-server:8000
# No Python needed on the client machine AT ALL
```

The Rust binary handles:
- Downloading and caching the correct Python (via python-build-standalone)
- Creating an isolated venv for `batchalign-models`
- Installing PyTorch with the right CUDA/MPS variant
- Managing model downloads
- All transparently, on first run

### 2. Startup Time: 300x Faster

| | Python | Rust |
|---|---|---|
| `batchalign --help` | 3,000ms | 10ms |
| `batchalign morphotag` (to first file) | 3,500ms | 15ms + model load |
| Tab completion | Laggy | Instant |

The 3-second startup is death for interactive use and shell scripts. Every
`batchalign` invocation pays the Python import tax even if it's just checking
`--help`. Rust eliminates this entirely.

### 3. Memory: Lower Baseline, Potential Model Sharing

| Scenario | Python | Rust |
|----------|--------|------|
| Idle server | ~150MB | ~20MB |
| 1 worker (morphosyntax) | ~4.5GB | ~4.1GB (same model, less framework) |
| 8 workers (ProcessPool) | ~36GB | ~5GB (if models shared via threads) |

The last row is the big one. Python's ProcessPoolExecutor copies the entire
interpreter + model for each worker. If Rust can share a single model across
threads (possible with PyO3's GIL management for CPU models, or with Rust-native
inference for ONNX models), memory drops by 4-8x.

Even without model sharing, the per-worker overhead drops from ~500MB (Python
interpreter + framework) to ~50MB (PyO3 thin layer).

### 4. Concurrency: True Async, No GIL Workarounds

**Current architecture's concurrency problems:**

- `ProcessPoolExecutor`: Full process copies, high memory, IPC serialization cost
- `ThreadPoolExecutor` in server: GIL limits CPU parallelism
- `threading.Lock` / `threading.Semaphore`: Manual synchronization, deadlock-prone
- asyncio + sync code: "Is this function sync or async?" confusion
- `threading.Thread(daemon=True)`: Fire-and-forget threads, hard to debug

**Rust architecture:**

- `tokio::task::spawn`: Lightweight async tasks, millions concurrent
- `rayon`: Work-stealing parallelism for CPU-bound work
- `tokio::sync::Semaphore`: Async-aware concurrency limits
- `Arc<RwLock<T>>`: Shared state with compile-time data race prevention
- `spawn_blocking`: Bridge to synchronous PyO3 calls

The Rust concurrency model prevents data races at compile time. The current
Python codebase has had multiple threading bugs (SQLite thread safety, job store
race conditions). Rust's ownership system makes these impossible.

### 5. Correctness: Compile-Time Guarantees

| Bug Category | Python | Rust |
|-------------|--------|------|
| None/null dereference | Runtime crash | Compile error (`Option<T>`) |
| Unhandled error | Silent failure | Compile error (`Result<T, E>`) |
| Missing enum case | Runtime `else` fallthrough | Compile error (exhaustive match) |
| Data race | Runtime corruption | Compile error (ownership) |
| Type mismatch | mypy warning (opt-in) | Compile error (always) |
| Use-after-free | N/A (GC) | Compile error (lifetime) |

The current codebase has mypy for type checking, but it's partial coverage with
`# type: ignore` escape hatches. Rust's type system is total and enforced.

### 6. Cross-Platform: Compile Once per Target

| Platform | Python | Rust |
|----------|--------|------|
| macOS ARM | Works (if deps compile) | `cargo build --target aarch64-apple-darwin` |
| macOS x86 | Works | `cargo build --target x86_64-apple-darwin` |
| Linux x86 | Works | `cargo build --target x86_64-unknown-linux-musl` |
| Linux ARM | Maybe (some deps lack wheels) | `cargo build --target aarch64-unknown-linux-musl` |
| Windows | Fragile (torch CUDA, path issues) | `cargo build --target x86_64-pc-windows-msvc` |

The Rust binary (non-ML parts) cross-compiles trivially. The Python ML core
still needs platform-specific wheels, but that's managed automatically by the
Rust binary's setup system.

### 7. Server Performance (Marginal but Real)

The server currently handles ~10 jobs/hour. Performance isn't the bottleneck.
But Rust's async runtime provides:

- **WebSocket scalability:** Dashboard connections cost ~8KB each instead of ~1MB
- **Graceful shutdown:** tokio's shutdown signal handling is cleaner than uvicorn's
- **Static file serving:** Built into axum, no separate nginx needed
- **Health checks:** Near-zero overhead, important for container orchestration

---

## What This Does NOT Buy Us

### ML inference speed stays the same

Moving to Rust doesn't make Whisper transcribe faster or Stanza tag faster. The
models run on PyTorch regardless. The inference time is dominated by GPU/CPU
computation on model weights, not by language overhead.

### GPU contention doesn't change

Whether you wait for GPU access in Python or Rust, the GPU can only run one
inference at a time. The ~25GB-per-worker budget exists because of model loading
storms, not Python overhead.

### Model ecosystem access stays Python

When a new ASR model is published, it will have Python bindings first. The thin
Python ML wrappers still need updating. But now it's updating 30 lines in one
file instead of 200+ lines across multiple files.

---

## Current Priority and Sequencing

### What we're doing now

The immediate priority is **getting the Rust CHAT data model right** — embedded
annotations on content nodes (see `docs/talkbank-model-amendments.md`). This is
work in `talkbank-model` itself, independent of batchalign.

The full Rust migration described below is an analysis of **what becomes possible**
once the data model is right. It is not a current plan. Building the pure Rust
CLI, server, or client binary depends on the data model and callback API being
complete — they are strictly sequential, not parallel work streams.

### What's worth building (and what isn't, yet)

**High value now:**
- `batchalign-core` (callback API, Hirschberg aligner, Mor/GRA builder) — this is
  where the massive simplification happens. Python engines shrink from thousands of
  lines to thin ML wrappers. This is worth building as soon as the data model is done.

**Not worth building now:**
- Rust CLI (clap), Rust server (axum) — the current Python CLI and server *work*.
  The gains (startup time, memory, concurrency safety) are real but marginal for
  the current user base. Rewriting them is significant effort for incremental benefit.
- Pure Rust client binary — attractive but depends on batchalign-core being complete.
  Falls out naturally once the rest is done, but isn't a standalone goal.

### The correct ordering

```
1. Embedded data model in talkbank-model         ← CURRENT WORK
   ↓
2. batchalign-core crate (callback API + PyO3)
   ↓
3. Simplify Python engines (thin ML wrappers)
   ↓
4. Fix Python import chain (lazy loading)         ← Quick win, no Rust needed
   ↓
--- Everything above is high value. Below is "someday, if it matters." ---
   ↓
5. Pure Rust client binary (--server mode)
   ↓
6. Rust CLI / Rust server / auto-managed Python
```

---

## Migration Phases (Reference)

The phases below describe the full migration path for reference. Phases 0–2 are
the high-value work. Phases 3–6 are future possibilities.

### Phase 0: Data Model + Callback Architecture (current work → next)

Implement the `talkbank-model` amendments: embedded annotations, content tree
visitor, parse-time embedding, serialization traversal. Then build the callback
API in `batchalign-core`.

**Outcome:** `batchalign-core` Rust crate with PyO3 bindings. Python calls into
Rust for CHAT parsing and orchestration, Rust calls back into Python for ML.

### Phase 1: CHAT Parsing Migration

Replace `batchalign/formats/chat/` (1,436 lines) with calls to `batchalign-core`.
The Python CHAT parser, lexer, and generator are replaced by the Rust equivalents.

**Outcome:** CHAT round-tripping goes through Rust. Python only handles ML.

### Phase 2: Pipeline Simplification

Each engine shrinks to a thin wrapper that:
1. Defines the ML callback
2. Calls the appropriate `batchalign-core` function

`StanzaEngine.process()`: 1,230 lines → ~40 lines
`WhisperFAEngine.process()`: 234 lines → ~30 lines
`WhisperUTREngine`: disappears entirely

**Outcome:** Python pipeline code drops from ~5,400 lines to ~300 lines.

### Phase 3: CLI Migration (future)

Build the Rust CLI binary with clap. It handles:
- Argument parsing
- File discovery
- Worker orchestration
- Progress display
- Calls `batchalign-core` for CHAT processing
- Initializes Python (via PyO3) only when ML is needed

**Outcome:** `batchalign` is a Rust binary. ~10ms startup.

### Phase 4: Server Migration (future)

Build the HTTP server in axum. It handles:
- REST endpoints
- WebSocket for dashboard
- Job store (in-memory + rusqlite)
- Static file serving (React dashboard)
- Worker management
- Initializes Python (via PyO3) for ML inference

**Outcome:** `batchalign serve` is pure Rust + embedded Python for ML.

### Phase 5: Client Binary (future — zero Python)

Build a pure Rust client binary for `--server` mode. No Python at all — it reads
CHAT files, sends them over HTTP, receives results, writes output.

**Outcome:** Researchers install a 15MB binary. No Python, no venv, no deps.

### Phase 6: Auto-Managed Python Environment (future)

The Rust binary handles Python environment setup:
- Downloads python-build-standalone if needed
- Creates an isolated venv
- Installs `batchalign-models` (the ~300-line pip package)
- Detects CUDA/MPS and installs appropriate PyTorch
- All on first `batchalign setup` or first ML command

**Outcome:** `curl | sh` → `batchalign setup` → ready. One command installation.

---

## Risk Assessment

### Low Risk

- **CHAT parsing migration**: talkbank-model already exists and is tested
- **DP aligner**: Pure algorithm, straightforward to implement in Rust
- **CLI**: clap is mature, well-documented
- **Configuration/logging**: Standard Rust crate ecosystem

### Medium Risk

- **PyO3 model sharing**: Sharing ML models across Rust threads via PyO3 requires
  careful GIL management. May need to fall back to process isolation.
- **Auto-managed Python**: Packaging Python + PyTorch + CUDA is notoriously
  platform-specific. May need significant testing across platforms.
- **Server migration**: axum is mature, but migrating WebSocket + job store +
  dashboard requires careful testing of concurrent behavior.

### High Risk

- **Audio preprocessing**: ML models expect specific audio loading (torchaudio's
  feature extraction, WhisperProcessor's mel spectrogram). Some preprocessing
  may need to stay in Python alongside the model, increasing the Python surface
  beyond the estimated ~300 lines.
- **Stanza tokenizer post-processing**: The current `ud.py` has complex
  tokenizer post-processing hooks (`tokenizer_postprocessor` lambda) that
  interact with Stanza's internal pipeline. Isolating this into a clean callback
  may require refactoring Stanza's usage pattern.

---

## Summary

| Metric | Current (Python) | After Migration (Rust + thin Python) |
|--------|-----------------|--------------------------------------|
| Total Python LOC | 33,348 | ~300 (ML wrappers only) |
| Startup time | ~3,000ms | ~10ms |
| Binary size | ~200MB (venv) | ~15MB + auto-managed Python |
| Client (--server) deps | Python + PyTorch + all | Zero (pure Rust binary) |
| Memory (idle server) | ~150MB | ~20MB |
| Memory (8 workers) | ~36GB | ~5-10GB (potential model sharing) |
| DP alignment (large UTR) | ~40 seconds | ~0.3 seconds |
| Concurrency bugs | Runtime discovery | Compile-time prevention |
| Cross-compilation | Per-platform wheels | `cargo build --target` |
| Install command | `uv tool install ...` | `curl \| sh` |
