# Eliminating DP Alignment from Batchalign

**Status:** Living document (last updated: 2026-02-11)

> **Update (2026-03):** Forced alignment has been migrated to the Rust
> server (CHAT divorce Phase 6). The DP alignment for FA was ported to
> Rust (`batchalign-chat-ops/src/fa.rs`), not eliminated — it remains
> necessary for the retokenize path and for mapping model output tokens
> to CHAT words.

---

## Core Insight

DP alignment exists in batchalign because the Python code **destroys structural
information** (word identity, word boundaries, word indices) by flattening CHAT
into strings, then uses expensive O(n*m) dynamic programming to **recover** that
lost information. The Rust AST preserves structure throughout, making DP
unnecessary in every case except one.

---

## Where DP Is Used Today

### 1. Morphosyntax (standard path) — ALREADY ELIMINATED

**Was:** Python flattened CHAT into a string via `annotation_clean()`, sent it to
Stanza, Stanza re-tokenized it, DP character-level alignment mapped Stanza's
output back to original words.

**Now:** Rust extracts words structurally from the AST, sends them to Stanza
pretokenized via callback, gets results back indexed to the same words, injects
%mor/%gra into the same AST nodes. No DP.

### 2. Morphosyntax (retokenize path) — STILL USES DP

**What happens:** User explicitly requests Stanza's UD tokenization to replace
CHAT's word boundaries. Stanza may split "don't" → ["do", "n't"] or merge
tokens. Character-level DP maps between the two tokenizations.

**Why it exists:** This is a deliberate tokenization *change*, not error recovery.
The user is saying "I want Stanza's word boundaries, not CHAT's." Two genuinely
different tokenizations need mapping.

**Can it be eliminated?** Partially. Since we send individual words to Stanza and
get back Stanza's tokens, we know the correspondence at the word level. The
character-level DP is overkill — we could track the mapping structurally by
comparing input word count vs output token count per callback invocation. The
Rust `retokenize.rs` already does this but currently uses DP internally for the
character-level alignment. A simpler word-level mapping would suffice in most
cases.

### 3. Forced Alignment (FA) — UNNECESSARY, CAN BE ELIMINATED

**What happens:** Python flattens group words into a string via `detokenize()` +
punctuation stripping, sends the flat string to Whisper/Wave2Vec, gets back a
list of (word_text, timestamp) tuples, then uses character-level DP to map the
model's words back to the original transcript words.

**Why DP is unnecessary:** We have the words. We know their order and indices. The
FA model is doing *forced* alignment — it's told what text to find. It already
knows the answer. The DP exists because Python destroyed word boundaries by
flattening to a string, and the model re-tokenized that string, producing a
different word list. Then DP recovers the mapping that was thrown away.

**How to eliminate:** Rust orchestrator sends words by index to the FA callback.
The callback receives `{"words": ["I", "want", "cookies"], "audio_range": [1500, 3200]}`.
The FA model aligns these specific words against the audio chunk and returns
timestamps by index: `{"timings": [[1500, 1800], [1800, 2100], [2100, 3200]]}`.
Rust assigns timestamps directly to AST nodes. No DP, no character-level
alignment, no word-list mapping.

**Current code locations:**
- `whisper_fa.py:124-137` — Character-level DP alignment
- `wave2vec_fa.py:121-138` — Identical pattern
- Both use `align()` from `utils/dp.py`

### 4. Utterance Timing Recovery (UTR) — UNNECESSARY, CAN BE ELIMINATED

**What happens:** Python runs *unconstrained* ASR on the full audio file (Whisper
transcribes from scratch), producing a word sequence with timestamps that differs
from the transcript. Then word-level DP aligns the ASR words against the
transcript words to transfer timestamps.

**Why DP is unnecessary:** We have the transcript words. We have the audio. We're
using Whisper, which supports *forced* alignment (not just transcription). Instead
of the roundabout path:

```
Audio → unconstrained ASR → different words → DP align against transcript → timestamps
```

We can do:

```
Audio + transcript words → forced alignment → timestamps (directly for our words)
```

The DP exists because Python took the indirect approach of "transcribe from
scratch, then figure out which words match." With forced alignment, we tell the
model what words to find and get timestamps back for those exact words.

**How to eliminate:** Rust orchestrator extracts transcript words from the AST,
sends them (along with full audio reference) to a forced-alignment callback. The
callback runs Whisper or Wave2Vec in *forced* mode (CTC alignment / cross-attention
alignment against known text) and returns timestamps per word. Rust assigns
timestamps to AST nodes.

This also eliminates UTR as a separate pipeline stage. Currently the pipeline is:

```
UTR (unconstrained ASR + DP) → FA (forced alignment + DP)
```

With the Rust approach:

```
Rust forced-align (full file, chunked) → timestamps directly on AST
```

UTR and FA merge into a single Rust-orchestrated operation.

**Current code locations:**
- `utr/utils.py:9-67` — `bulletize_doc()` with word-level DP
- `utr/whisper_utr.py:29-52` — Orchestration calling `bulletize_doc()`
- Uses `align()` from `utils/dp.py`

**Performance note:** UTR's DP is the worst offender. For a 60-minute file with
20,000 transcript words and 25,000 ASR words, the pure-Python DP computes 500
million cells in ~4 minutes. Eliminating DP entirely removes this cost.

---

## The Only Legitimate Use of DP

**Benchmark/WER evaluation:** Comparing a machine transcript against a human
reference transcript to compute word error rate. These are two genuinely
independent texts with no shared structure. DP (edit distance) is the correct
algorithm. This is the `benchmark` command, not `align`.

---

## Why DP Was a Hack

The pattern is the same everywhere:

1. Python has structured data (words in an AST with indices and boundaries)
2. Python destroys structure by flattening to a string
3. A model processes the string and returns a different string
4. Python uses O(n*m) DP to recover the mapping it threw away

The Rust AST breaks this cycle:

1. Rust has structured data (words in an AST with indices and boundaries)
2. Rust sends words by index to a Python callback
3. Python does ML inference and returns results by index
4. Rust assigns results to the same AST nodes

Structure is preserved throughout. No information is lost. No recovery needed.

---

## Architecture: Format-Agnostic Python

Eliminating DP is part of a broader architectural principle: **Python should know
nothing about CHAT.** Python's role is ML inference — running Stanza, Whisper,
Wave2Vec, PyAnnote. All format handling (parsing, serialization, word extraction,
result injection, timing markers) belongs in the Rust format layer.

```
Format Layer (Rust)            ML Layer (Python)
┌──────────────────┐          ┌──────────────────┐
│ Parse CHAT/TextGrid/...     │ Stanza (morphosyntax)
│ Extract words by index  ──→ │ Whisper (FA/UTR)
│ Assign results by index ←── │ Wave2Vec (FA)
│ Serialize to format         │ PyAnnote (diarization)
└──────────────────┘          └──────────────────┘
```

This makes batchalign format-agnostic. Adding TextGrid morphosyntax or a new
format requires only a new Rust parser + extraction/injection adapters. Python ML
code is unchanged. No DP anywhere.

---

## Implementation Plan

### Phase 1: FA Without DP (High Impact)

Replace `whisper_fa.py` and `wave2vec_fa.py` with a Rust orchestrator
`add_forced_alignment()` that:

1. Parses CHAT into AST
2. Groups utterances into ~20s segments (from utterance timing in AST)
3. Extracts word list per group (by index)
4. Calls Python FA callback: `{"words": [...], "audio_range": [start_ms, end_ms]}`
5. Receives timestamps by index: `{"timings": [[start, end], ...]}`
6. Assigns timestamps to AST word nodes
7. Runs post-processing (fill end-times, bound by utterance, drop invalid)
8. Serializes back to CHAT

Python FA callback is thin: receive words + audio range, run Whisper/Wave2Vec
inference, return timestamps. ~30 lines instead of ~230 lines per engine.

### Phase 2: Merge UTR + FA

Replace the two-stage `UTR → FA` pipeline with a single Rust orchestrator that:

1. Parses CHAT into AST
2. If utterances lack timing: run forced alignment on large chunks (full-file or
   ~5-minute windows) to get rough utterance boundaries
3. Refine with per-group forced alignment (Phase 1 logic)
4. Assign all timestamps in one pass

This eliminates UTR as a separate concept. The forced-alignment callback handles
both rough timing recovery and precise word alignment. Rust manages the chunking
strategy.

### Phase 3: Clean Up DP Infrastructure

Once FA and UTR are migrated:

- `utils/dp.py` is only used by `benchmark` (WER evaluation)
- `batchalign-core/dp_align.rs` is only used by `retokenize.rs`
- The retokenize DP can be simplified to word-level mapping (no character-level)
- Remove DP from the "standard" processing path entirely

### Phase 4: Remove Python CHAT Format Layer

With all pipelines using Rust orchestrators:

- `formats/chat/lexer.py` — dead code (Rust parses)
- `formats/chat/parser.py` — dead code (Rust parses)
- `formats/chat/generator.py` — dead code (Rust serializes)
- `formats/chat/utils.py` — `annotation_clean()` dead (Rust AST provides `cleaned_text`)
- `CHATFile` class becomes a thin wrapper calling Rust

---

## Key Files

### Current DP usage (to be eliminated)
- `batchalign/utils/dp.py` — Hirschberg DP implementation
- `batchalign/pipelines/fa/whisper_fa.py:124-137` — FA character-level DP
- `batchalign/pipelines/fa/wave2vec_fa.py:121-138` — FA character-level DP
- `batchalign/pipelines/utr/utils.py:9-67` — UTR word-level DP
- `batchalign/pipelines/morphosyntax/ud.py:~820-945` — Retokenize DP (Python fallback)

### Rust infrastructure (already built)
- `batchalign-core/src/dp_align.rs` — Rust DP (only needed for retokenize)
- `batchalign-core/src/extract.rs` — Word extraction by index
- `batchalign-core/src/inject.rs` — Result injection by index
- `batchalign-core/src/retokenize.rs` — Retokenization with DP
- `batchalign-core/src/lib.rs` — `add_morphosyntax()` orchestrator (pattern to follow)
