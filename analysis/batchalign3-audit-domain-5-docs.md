# Batchalign3 Documentation Audit - Domain 5: Parity vs Implementation

**Status:** Current
**Last updated:** 2026-03-16

## 1. Overview
This audit examines the parity between `batchalign3` code implementation and its documentation (specifically `batchalign3/book` and docstrings). The investigation focuses on identifying stale, inaccurate, misleading, or missing details, particularly concerning the architectural migration from `batchalign2` and the handling of CLI plumbing, NLP special cases, and testing.

## 1.1 Reconciliation Update (2026-03-16)

The findings below preserve the original audit text. This note records what was
actually corrected during the sweep.

- **Fixed in this sweep:**
  - `book/src/architecture/error-handling.md` now describes the Rust-owned
    validation gate more accurately, and related forced-alignment docs no longer
    imply that invalid output is gated only by a Python exception wrapper
  - `book/src/architecture/cli-option-wiring.md` now clearly documents the
    extracted-but-not-consumed `wor_tier` fields as intentional
    forward-compatibility scaffolding
  - `book/src/reference/morphosyntax.md`,
    `book/src/architecture/python-rust-interface.md`, and
    `book/src/reference/mwt-handling.md` now reflect Rust-owned orchestration
    and no longer rely on the older `_stanza_batch_callback.py` framing
  - the `talkbank-tools` docs / VS Code surface were already updated earlier in
    the sweep to reflect `chatter lsp` as the only supported LSP entrypoint
- **Current interpretation:**
  - remaining references to a “direct Python pipeline” now refer to the
    compatibility facade / PyO3 surface, not to Python owning the production
    control plane
  - migration/history documents may still mention older Python-owned phases on
    purpose; those are historical references, not active architecture claims

## 2. Findings: Architectural Migration Docs vs Implementation

### 2.1 Error Handling (`error-handling.md` vs Code)
- **Stale Python References:** The documentation extensively describes `validate_structured()` and `CHATValidationException` in Python (`batchalign/errors.py`) as if the Python pipeline is still responsible for validating and gating serialized output. However, the migration to a Rust server moved this logic to the Rust `batchalign-app` and `batchalign-chat-ops` crates. 
- **Stale Error Flow Diagram:** The diagram still depicts Python's `pipeline.py` calling `validate_structured()` and raising `CHATValidationException`. While `batchalign/errors.py` still exists, the Rust backend handles serialization and validation now. The documentation has not caught up to the final Rust migration state.

### 2.2 CLI Option Wiring (`cli-option-wiring.md` vs Code)
- **Inaccurate documentation for Transcribe/Benchmark:** The documentation claims `--wor` is extracted but not consumed for `transcribe` and `benchmark`. A review of `crates/batchalign-app/src/runner/dispatch/options.rs` shows that these fields are extracted and populated in `TranscribeDispatchParams` and `BenchmarkDispatchParams`, but they are indeed not utilized in the actual orchestration plans (`plan.rs`). The documentation correctly identifies this, but the wording is slightly misleading as the unused params are kept for forward compatibility.
- **Incremental Orchestration:** The documentation correctly outlines how `--before PATH` bypasses `CommandOptions` and goes straight to the `Job` struct.

### 2.3 Morphosyntax Pipeline (`morphosyntax.md` vs Code)
- **Over-emphasis on Python Execution Flow:** The document provides a good overview of the legacy PyO3 callback pattern (`_stanza_batch_callback.py`). However, it doesn't clearly reflect that Rust's `batchalign-app` now fully orchestrates caching and batched inference instead of Python. The documentation still frames the workflow around Python inference APIs when the Rust control plane is actually in charge.

## 3. Actionable Recommendations

1. **Update `error-handling.md`:** Rewrite the Error Flow Diagram and Sections 3-6 to reflect that Rust (`batchalign-app`) now exclusively owns CHAT validation and error gating. Clarify that `CHATValidationException` in Python is a legacy compatibility shim for older scripts, not the main server runtime.
2. **Update `cli-option-wiring.md`:** Clarify that unused parameters like `wor_tier` for transcribe are intentionally preserved in the structs for future pipeline wiring, to avoid confusion.
3. **Deprecate Legacy Python Flow Explanations:** Systematically scan `batchalign3/book/` for mentions of `pipeline.py` and old Python-centric execution flows. Update them to reflect the final migration state described in `rust-server-migration.md`, emphasizing the `execute_v2` Rust-Python worker protocol.

## 4. Conclusion
The codebase has evolved faster than the documentation, particularly regarding the final stages of the Rust control plane migration. Many documents correctly describe the transitional state (where Python still handled some orchestration/validation) rather than the current end-state (where Rust handles everything except stateless ML inference).
