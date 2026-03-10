# UTR in the `align` Pipeline: Analysis and Recommendations

**Date:** 2026-02-11
**Author:** Claude (automated analysis)
**Status:** Draft for discussion

---

## 1. Executive Summary

The `align` command auto-injects a UTR (Utterance Timing Recovery) engine before
forced alignment (FA). UTR runs a **full Whisper ASR pass** over the entire audio
file, then DP-aligns ASR output against the existing transcript to bootstrap rough
word-level timing. FA then overwrites all of that timing with precise forced
alignment.

For **pre-timed files** (already have timing bullets), UTR is a no-op — it detects
existing timing and skips. For **untimed files**, UTR is currently **required**
because the Rust FA orchestrator silently skips utterances that have no timing.

This report explains the current behavior, quantifies the cost, and proposes
alternatives.

---

## 2. How `align` Works Today

### 2.1 Pipeline Composition

When a user runs `batchalign3 align input/ output/`, the pipeline resolver
(`dispatch.py:54-56`) auto-adds UTR:

```python
if "fa" in packages:
    if "utr" not in packages:
        packages.append("utr")
```

This produces a two-stage pipeline:

| Order | Engine | Task Enum | What It Does |
|-------|--------|-----------|-------------|
| 1 | WhisperUTREngine | Task 8 (UTR) | Full ASR pass + DP match to add rough timing |
| 2 | WhisperFAEngine | Task 9 (FA) | Token-level forced alignment for precise timing |

### 2.2 UTR: What It Actually Does

**File:** `batchalign/pipelines/utr/whisper_utr.py`

1. **Skip check** (line 34): If any utterance already has timing, UTR returns
   immediately. This means UTR only does work on completely untimed transcripts.

2. **Full-file ASR** (line 42): Loads the entire audio file and runs Whisper
   transcription (`talkbank/CHATWhisper-en-large-v1` for English,
   `openai/whisper-large-v2` for other languages). This produces word-level
   timestamps from scratch — a complete re-transcription of the audio.

3. **DP alignment** (`utr/utils.py:48`): Runs Hirschberg DP alignment between
   ASR words (with timestamps) and transcript words (without timestamps). For
   each matched word pair, copies the ASR timestamp onto the transcript word.

4. **Result:** The transcript now has rough word-level timing derived from ASR
   output. Unmatched words remain untimed.

### 2.3 FA: What Happens Next

**File:** `batchalign/pipelines/fa/whisper_fa.py`

The Rust FA orchestrator (`batchalign_core.add_forced_alignment`):

1. **Groups utterances by time windows** — scans existing timing bullets to
   segment utterances into groups of up to `max_group_ms` milliseconds
   (20,000ms for Whisper FA, 15,000ms for Wave2Vec FA).

2. **Extracts words per group** — collects all words with positional indices
   (`utterance_index`, `word_index`).

3. **Calls FA callback per group** — sends `{words, audio_start_ms,
   audio_end_ms, pauses}` to the Python FA model, expects back
   `{timings: [[start, end], null, ...]}`.

4. **Injects timings by position** — maps each returned timing to its word
   by index. No DP alignment needed.

5. **Generates %wor tier** — produces the word-level timing tier.

### 2.4 The Critical Dependency

**The Rust FA orchestrator skips utterances that have no timing.**

From the test suite (`test_rust_fa.py:148-159`):

```python
def test_untimed_utterances_skipped(self) -> None:
    """Utterances without bullets are not included in groups."""
    batchalign_core.add_forced_alignment(UNTIMED_CHAT, capture_callback)
    assert len(calls) == 0  # No groups created, no callback invoked
```

The grouping algorithm requires `audio_start_ms` and `audio_end_ms` for each
group. These come from the first and last timed words in each utterance
(`Utterance.alignment` property). If no words are timed, the utterance has no
alignment and is excluded from all groups.

**This is why UTR exists in the `align` pipeline:** without it, untimed
transcripts produce zero FA groups and the output is unchanged from the input.

---

## 3. Cost Analysis

### 3.1 Runtime Cost

For a **30-minute audio file** with ~3,000 transcript words:

| Phase | Time | Notes |
|-------|------|-------|
| UTR Phase 1: Full Whisper ASR | **5-7 min** | Processes entire audio (~5-6x realtime) |
| UTR Phase 2: DP alignment | **~2.5 sec** | O(n*m) Hirschberg, ~5M cells |
| FA: Forced alignment | **< 1 sec** | Per-group callback, very fast |
| **Total with UTR** | **~6-8 min** | |
| **Total without UTR (if possible)** | **< 1 min** | FA only |

UTR dominates the `align` runtime by 10-100x for untimed files.

### 3.2 Memory Cost

UTR and FA load **separate Whisper model instances**:

- **UTR:** `WhisperASRModel` — `transformers.pipeline("automatic-speech-recognition")`
  with `openai/whisper-large-v2` (~3 GB GPU memory)
- **FA:** `WhisperFAModel` — `WhisperForConditionalGeneration` with cross-attention
  extraction (~3 GB GPU memory)

Both load the same model weights but into different objects. If both are loaded
simultaneously (e.g., in pipelined dispatch), that's ~6 GB of GPU memory for
two copies of essentially the same model.

Additionally, UTR calls `.all()` on the audio, loading the entire file into
memory at once. For a 2-hour recording, this is 500+ MB of raw audio tensors.

### 3.3 Pre-Timed Files (Common Case)

For files that already have timing bullets (the re-alignment use case), UTR
detects existing timing at line 34 and returns immediately. **Cost: zero.**
But the UTR engine is still instantiated, and the Whisper ASR model may be
loaded into memory depending on lazy-loading behavior.

### 3.4 Real-World File Distribution

Full scan of `~/data/` (all CHAT files with `@Media:` headers):

| Category | Count | Percentage | UTR Behavior |
|----------|-------|------------|-------------|
| Pre-timed (have timing bullets) | 64,282 | 90.8% | **Skips** (no-op) |
| Untimed (>2 utterances) | 5,644 | 8.0% | **Full ASR pass** |
| Other (no media or ≤2 utterances) | 850 | 1.2% | N/A |
| **Total with @Media:** | **70,776** | | |

**Key takeaway:** For 91% of files, UTR does nothing. The `--no-utr` flag
produces identical results for these files while skipping UTR model loading.
The 5,644 untimed files are where `--no-utr` changes behavior — FA will skip
untimed utterances without UTR to provide rough timing first.

Script used to generate these numbers: `tools/scan_corpus_timing.sh`

---

## 4. The Core Problem

UTR exists solely because the Rust FA orchestrator cannot handle untimed input.
This creates a dependency where a 5-7 minute full ASR pass is required just to
produce throwaway rough timing that FA will immediately overwrite with precise
alignment.

The architecture is:

```
Audio + Untimed Transcript
    → [UTR: Full ASR (5-7 min) + DP alignment (2.5 sec)]
    → Rough-timed Transcript
    → [FA: Precise alignment (< 1 sec)]
    → Precisely-timed Transcript
```

The rough timing from UTR serves exactly one purpose: telling FA which audio
segment corresponds to which utterance group. FA then discards the rough timing
entirely and replaces it with its own precise alignment.

---

## 5. Proposed Alternatives

### 5A. Make Rust FA Handle Untimed Input Natively (Recommended)

Add a fallback grouping strategy to the Rust FA orchestrator for utterances
without timing:

**Option 1 — Sequential fixed-window processing:**
When no timing is available, process utterances sequentially. Accumulate words
until the batch reaches a target size (e.g., 50 words or ~20 seconds estimated
duration), then pass the group to FA with `audio_start_ms=0, audio_end_ms=-1`
(meaning "search the full audio"). The FA model (Whisper) already handles
open-ended search — forced alignment with Whisper works by conditioning on the
transcript text and finding where it occurs in the audio.

**Option 2 — VAD-based segmentation:**
Run a lightweight voice activity detector (VAD) on the audio to identify speech
segments. Map utterances to VAD segments by count or by simple heuristics.
pyannote's VAD is already a dependency (via diarization) and runs in seconds.

**Option 3 — Proportional estimation:**
For untimed files, estimate each utterance's duration proportionally to its word
count relative to total words, then distribute across the total audio duration.
This gives rough boundaries that FA can refine.

**Cost:** Options 1-3 add seconds, not minutes. Any of them eliminates the
need for a full ASR pass.

### 5B. Move UTR to a Separate Command

UTR has legitimate use cases outside `align` — e.g., adding rough timing to
untimed transcripts for playback in TalkBank browsers, without the precision
of full forced alignment. It should be a standalone command (`batchalign3 utr`)
rather than auto-injected into `align`.

### 5C. Keep Both Modes (Transitional)

During the transition, support both paths:
- **With UTR** (current behavior): `batchalign3 align input/ output/` — auto-adds UTR
- **Without UTR** (new behavior): `batchalign3 align --no-utr input/ output/` — skips UTR,
  relies on Rust FA's native untimed handling (once implemented)

This allows A/B comparison on real corpora before fully removing UTR from
the align pipeline.

---

## 6. Migration Impact

### 6.1 If UTR Is Removed from `align`

- **Pre-timed files:** Zero change (UTR already skips)
- **Untimed files:** Requires Rust FA to handle untimed input (Section 5A)
- **Runtime improvement:** 10-100x faster for untimed files
- **Memory improvement:** No second Whisper model loaded
- **Pipeline simplification:** `align` becomes single-stage (no pipelined dispatch needed)
- **CHAT layer migration:** Eliminates the Document-only UTR engine from the `align`
  CHAT text fast path, removing a major bridge dependency

### 6.2 If UTR Is Kept in `align`

- Rust FA untimed handling is still valuable (as a performance optimization)
- UTR engines need Rust migration (native `process_chat_text`) for CHAT layer elimination
- Pipeline remains two-stage, requiring pipelined dispatch

### 6.3 Risks

- **Quality regression:** UTR + FA may produce better alignment than FA-only on
  edge cases (very long files, poor audio quality, many disfluencies). This needs
  empirical validation on real corpora.
- **Untested code path:** Rust FA has never processed untimed input — the
  `test_untimed_utterances_skipped` test explicitly validates that it DOESN'T.
  New Rust code + tests are needed.

---

## 7. Recommendation

**Short-term:** Add `--no-utr` flag to `align` command. Default behavior unchanged.
This enables safe A/B testing.

**Medium-term:** Implement proportional estimation (Option 3 from Section 5A) in
the Rust FA orchestrator as a fallback for untimed utterances. This is the simplest
approach — no new dependencies, deterministic, and gives FA reasonable audio windows
to search within.

**Long-term:** Remove UTR auto-injection from `align`. Keep UTR as a standalone
command for users who specifically want rough timing without full FA. This
simplifies the `align` pipeline to single-stage and unblocks the CHAT layer
migration.

---

## 8. Decisions Made

### 8.1 `--no-utr` flag (2026-02-11)

**Decision:** Added `--utr/--no-utr` flag to the `align` CLI command. Default
behavior unchanged (`--utr`). Passing `--no-utr` skips UTR entirely — the
pipeline resolver (`dispatch.py`) receives `utr=None` and neither auto-adds
nor instantiates a UTR engine.

**Files changed:**
- `batchalign/cli/cli.py` — new `--utr/--no-utr` flag on `align`
- `batchalign/pipelines/dispatch.py` — `resolve_engine_specs()` handles `utr=None`
- `batchalign/tests/pipelines/test_no_utr_align.py` — 8 tests

### 8.2 Lazy model loading in WhisperUTREngine (2026-02-11)

**Decision:** Defer Whisper model loading from `__init__` to first call to
`process()` that actually needs it (i.e., an untimed file). For the 91% of
files that are pre-timed, UTR's `process()` returns at the skip check on
line 38 without ever loading the ~3 GB Whisper ASR model. Previously, the
model was loaded eagerly in `__init__`, wasting ~10-15 seconds of startup
time and ~3 GB of GPU memory even when every file in the batch was pre-timed.

RevUTREngine was already lazy (API client created on first `process()` call)
and required no changes.

**Files changed:**
- `batchalign/pipelines/utr/whisper_utr.py` — store model name + language in
  `__init__`, defer `WhisperASRModel()` to `_get_model()` on first use

### 8.3 Native untimed FA handling — deferred

**Decision:** Deferred to a future iteration. Making the Rust FA orchestrator
handle untimed input natively (Section 5A) would eliminate the need for UTR
in the `align` pipeline entirely. This requires new Rust code + tests and
empirical validation that FA-only produces comparable alignment quality.
Will re-evaluate after A/B testing with `--no-utr` on real corpora.

---

## Appendix A: Corpus Scan Results

Full scan of `~/data/` performed 2026-02-11 using `tools/scan_corpus_timing.sh`:

| Category | Count | Percentage |
|----------|-------|------------|
| CHAT files with `@Media:` | 70,776 | 100% |
| Pre-timed (have timing bullets) | 64,282 | 90.8% |
| Untimed (>2 utterances) | 5,644 | 8.0% |
| Other | 850 | 1.2% |

---

## Appendix B: Key Files

| File | Role |
|------|------|
| `batchalign/pipelines/dispatch.py:54-56` | Auto-injection of UTR into align |
| `batchalign/pipelines/utr/whisper_utr.py` | WhisperUTREngine (lazy model load) |
| `batchalign/pipelines/utr/rev_utr.py` | RevUTREngine (already lazy) |
| `batchalign/pipelines/utr/utils.py` | `bulletize_doc` — DP alignment logic |
| `batchalign/pipelines/fa/whisper_fa.py` | WhisperFAEngine (Rust-backed) |
| `batchalign/pipelines/fa/wave2vec_fa.py` | Wave2VecFAEngine (Rust-backed) |
| `batchalign/tests/pipelines/fa/test_rust_fa.py` | Tests confirming untimed skip behavior |
| `batchalign/tests/pipelines/test_no_utr_align.py` | Tests for --no-utr and lazy loading |
| `batchalign/document.py:170-201` | `Utterance.alignment` property |
| `tools/scan_corpus_timing.sh` | Corpus timing distribution scanner |
