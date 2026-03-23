# Proposal: Better Debugging for batchalign3 Morphosyntax Pipeline

**Status:** Draft
**Last updated:** 2026-03-23 18:30 EDT

## Problem

Debugging the morphosyntax pipeline is extremely difficult because:

1. **The daemon is a separate process.** The CLI spawns a daemon which spawns
   Python workers. `tracing::debug!` in Rust goes to the daemon's output,
   not the CLI's stderr. Python `logging.warning` goes to the worker's stderr,
   also invisible from the CLI.

2. **No way to inspect intermediate state.** We can't see:
   - What words the Rust extractor produces for a specific utterance
   - What the Python worker receives in the batch payload
   - What Stanza returns
   - What `map_ud_sentence` produces (MOR count, GRA count)
   - What `retokenize_utterance` does to the AST

3. **The PyO3 bridge is worker-runtime-only.** `batchalign_core` only exposes
   worker dispatch functions. There's no way to call CHAT parsing, word
   extraction, or morphosyntax injection from Python for debugging.

4. **End-to-end tests are slow.** Each CLI test spawns a daemon, loads models,
   processes files. A single test takes 20-40 seconds. Iteration is painful.

## What We Needed Today

During the retrace bug investigation, we needed:

1. **Given a CHAT file, show the extracted words for each utterance** (what
   Rust sends to Python). Currently only possible via Rust unit tests.

2. **Given extracted words, show what Python sends to Stanza** (after
   `_segment_cantonese` and `.replace()`). Currently requires temp logging.

3. **Given Stanza output, show what `map_ud_sentence` produces** (MOR count,
   GRA count, the actual MOR items). Currently only via Rust unit tests.

4. **Given all of the above, show the retokenize/inject result** (final
   word count, AST structure). Currently only via e2e test failure messages.

## Proposed Solutions

### Short-term: Expose diagnostic functions in batchalign_core

Add PyO3 functions for debugging:

```python
# Parse CHAT and extract morphosyntax payloads (same as server does)
payloads_json = batchalign_core.debug_extract_morphosyntax("@UTF8\n@Begin\n...", "yue")
# Returns: [{"line_idx": 0, "words": ["呢", "度", ...], "special_forms": [...]}]

# Given extracted words + UD response, run map_ud_sentence
result_json = batchalign_core.debug_map_ud_sentence(ud_words_json, "yue")
# Returns: {"mor_count": 6, "gra_count": 7, "mors": [...], "gras": [...]}

# Given CHAT + UD responses, run full inject_results and return diagnostics
result_json = batchalign_core.debug_inject_results(chat_text, payloads_json, ud_responses_json, "yue", "retokenize")
# Returns: {"success": false, "error": "MOR count...", "diagnostics": {...}}
```

This would let us debug the full pipeline from Python without needing
a running daemon.

### Medium-term: Structured trace output

Add a `--trace-morphosyntax` flag to the CLI that outputs JSON trace
events for each pipeline stage:

```json
{"stage": "extract", "utterance": 0, "words": ["呢", "度", ...], "word_count": 6}
{"stage": "worker_response", "utterance": 0, "ud_word_count": 6}
{"stage": "map_ud", "utterance": 0, "mor_count": 6, "gra_count": 7}
{"stage": "retokenize", "utterance": 0, "old_word_count": 6, "new_word_count": 5}
{"stage": "inject", "utterance": 0, "result": "error", "message": "MOR count..."}
```

These traces would go to a file or stderr, captured by the CLI process
(not lost in the daemon).

### Long-term: Move morphosyntax out of the daemon

The morphosyntax pipeline (extract → worker call → inject) could run
in the CLI process directly for debugging, bypassing the daemon for
single-file operations. The `--foreground` mode already exists for the
server — a similar `--in-process` mode for morphotag would eliminate
the daemon indirection entirely for debugging.

## Implementation Priority

1. **Debug PyO3 functions** (1 day): immediately useful, low risk
2. **Structured traces** (2 days): useful for production debugging
3. **In-process mode** (larger): architectural change, lower priority
