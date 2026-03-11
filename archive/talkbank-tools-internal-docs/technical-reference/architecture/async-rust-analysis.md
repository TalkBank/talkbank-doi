# Async Rust Analysis: Would tokio/async benefit TalkBank?

Evaluated 2026-02-23. Covers all Rust components across the TalkBank workspace.

## Summary

The Rust side of TalkBank is overwhelmingly CPU-bound batch processing of small text files. The right parallelism tool is `rayon` (parallel iterators), not `tokio` (async I/O multiplexing). Async would add complexity for near-zero benefit across nearly all components.

## Per-Component Assessment

### No benefit from async

| Component | Crate(s) | Reasoning |
|-----------|----------|-----------|
| Parsers | `talkbank-parser`, `talkbank-direct-parser` | Purely CPU-bound. Read file -> build AST. No I/O to overlap, no waiting. |
| Validators | `talkbank-model` (validation module) | Walk the AST, check invariants, emit errors. Pure computation. |
| Model / Transform | `talkbank-model`, `talkbank-transform` | AST manipulation, CHAT<->JSON roundtrip. Pure computation. |
| CLI | `talkbank-cli` (chatter) | Sequential from the user's perspective. Batch operations benefit from `rayon`, not async. |
| CLAN analysis | `talkbank-clan` | CPU-bound analysis over parsed ASTs. `rayon` for file-level parallelism. |
| PyO3 bridge | `batchalign-core` | Calls into Python for ML inference. The GIL (or free-threaded Python) is the concurrency boundary, not I/O. |

### Marginal benefit

| Component | Crate(s) | Reasoning |
|-----------|----------|-----------|
| LSP server | `talkbank-lsp` | `tower-lsp` and the Rust LSP ecosystem are async/tokio-native. Async provides ergonomic request cancellation (user types while diagnostics are computing -> cancel via `CancellationToken`) and natural debouncing of `didChange` notifications. However, for a single-editor LSP, there is rarely meaningful concurrency -- it's one user generating a stream of sequential requests. The actual work (re-parse, compute diagnostics) is CPU-bound. Cancellation and debouncing are achievable with channels and threads. |
| Rev.AI HTTP client | `talkbank-revai` | Async `reqwest` + tokio would enable batch pre-submission of many files to Rev.AI without spawning threads. But the current `reqwest::blocking` + `py.detach()` (GIL release) is simpler and sufficient. See the pre-submission analysis in `batchalign3/book/src/architecture/python-rust-interface.md`. |

### Real benefit only in a hypothetical Rust HTTP server

If we replaced the Python FastAPI server with a Rust server (axum/actix + tokio), async would handle concurrent client connections efficiently. But the heavy work -- ML inference (Stanza, Whisper, PyTorch) -- is still Python. A Rust HTTP layer calling into Python for actual processing would be *more* complex than the current pure-Python server for no throughput gain. The bottleneck is GPU/CPU inference time, not HTTP connection handling.

## The fundamental insight

TalkBank's Rust workload is: **"process thousands of small text files through CPU-bound parsing, validation, and analysis."** That is embarrassingly parallel batch work. `rayon` is purpose-built for it -- parallel iterators, work-stealing, zero-overhead on sequential paths.

Async/tokio is designed for I/O-multiplexing many concurrent connections (web servers, proxies, crawlers, distributed system coordinators). That problem doesn't exist on TalkBank's Rust side.

## When to choose async for future projects

Async Rust would be the right choice from the start if:
- The Rust code itself makes many concurrent network calls (e.g., a web crawler, an API aggregator)
- The project is a network server handling many simultaneous client connections with lightweight per-request work
- The project coordinates distributed systems (RPC, message queues)

None of these describe TalkBank's architecture, where Rust handles parsing/validation and Python handles ML inference and HTTP serving.

## Current concurrency tools in use

| Tool | Where used | Purpose |
|------|-----------|---------|
| `rayon` | `chatter validate`, `talkbank-clan` batch | File-level CPU parallelism for batch processing |
| `crossbeam` | Various | Scoped threads, channels |
| `reqwest::blocking` | `talkbank-revai` | Blocking HTTP (runs inside `py.detach()` for GIL release) |
| `py.detach()` | `batchalign-core` PyO3 bridge | Release GIL during pure-Rust work |
