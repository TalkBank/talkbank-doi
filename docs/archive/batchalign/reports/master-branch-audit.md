# Master Branch Audit — Python Implementation

Reference document describing what the master branch Python implementation does
for every tool command, including known bugs and deficiencies.

**Branch:** `master` (at `bd21bd9` / bench-baseline merge point)
**Date:** 2026-02-14

---

## Architecture Overview

### Data Model (`document.py`)

All pipeline work operates on `Document` — a Pydantic `BaseModel` containing:
- `content: list[Utterance | CustomLine]` — ordered list of utterances and special lines
- `langs: list[str]` — 3-letter ISO language codes
- `tiers: list[Tier]` — speaker/participant metadata
- `media: Media | None` — optional media file reference

Each `Utterance` contains:
- `content: list[Form]` — words/tokens with `text`, `time`, `type` (TokenType), `morphology`, `dependency`
- `tier: Tier` — speaker reference
- `alignment: tuple[int, int] | None` — utterance-level timing (ms)

`TokenType` enum: `REGULAR`, `RETRACE`, `PUNCT`, `FP` (filled pause), `ANNOT`, `FEAT`, `CORRECTION`.

### CHAT Parsing (`formats/chat/lexer.py`)

`UtteranceLexer` is a hand-written recursive descent parser that:
1. Reads raw CHAT utterance text character-by-character
2. Classifies tokens by prefix: `&-` → FP, `&` → ANNOT, `<` → group, `[` → annotation
3. Handles nesting via recursive `handle_group()` calls
4. The critical `decode()` method flattens nested structures — **this is where the major bug lives**

**The decode() bug:** When a group has a "special" type (e.g., RETRACE from `[/]`),
`decode()` overrides ALL tokens inside to that type. A `&~nonword` (ANNOT) inside
`<word &~nonword> [/]` gets overridden to RETRACE, causing it to pass through the
`phonated_words` filter and appear in `%wor`. This affects 3,735 files with 22,908
individual errors across 12 TalkBank collections.

### CHAT Serialization (`formats/chat/generator.py`)

`generate_chat_utterance()` iterates `utterance.content` and builds:
- Main tier line (via `utterance.tostring()`)
- `%mor` line — joins `Morphology` objects with `~` for MWT, formats as `POS|lemma-feats`
- `%gra` line — flattens dependency list to `id|dep_id|dep_type` format
- `%wor` line — iterates all `Form` objects, emitting `word ␕start_end␕` for timed words
- `%xtra` line — translation text
- Custom dependency tiers

**Known issue:** The `%wor` generation iterates `utterance.content` directly, which
includes whatever survived the `phonated_words` filter in the parser. There is no
separate %wor generation logic — the parser's inclusion/exclusion decisions are the
%wor decisions.

### CHAT File I/O (`formats/chat/parser.py`, `formats/chat/chat.py`)

`CHATFile` reads a `.cha` file, parses headers (`@Languages`, `@Participants`, `@ID`,
`@Media`), then parses each utterance line through `UtteranceLexer`. The parser:
- Filters tokens through `phonated_words`: keeps `REGULAR`, `RETRACE`, `PUNCT`, `FP`
- Drops `ANNOT`, `FEAT`, `CORRECTION` — these never become `Form` objects
- Builds `Form` objects with `text`, `time`, `type`
- Parses `%mor`, `%gra`, `%wor` dependent tiers

**No lenient mode.** Any parse error (`CHATValidationException`) fails the entire file.
There is no error recovery mechanism.

### Pipeline Engine Base (`pipelines/base.py`)

`BatchalignEngine` ABC with three methods:
- `generate(source_path) → Document` — for ASR
- `process(doc) → Document` — for alignment, morphosyntax, etc.
- `analyze(doc) → dict` — for evaluation, feature extraction

The `__call__` method deep-copies the input document before dispatching.
No handle-based methods — all engines operate on the full `Document`.

### Pipeline Dispatch (`pipelines/dispatch.py`)

`resolve_engine_specs()` maps task strings to engine names:
- Auto-adds `disfluency` and `retracing` when `asr` is requested
- Auto-adds `utterance` for supported languages when `asr` is requested
- Auto-adds `utr` when `fa` is requested
- Uses `DEFAULT_PACKAGES` dict with `LANGUAGE_OVERRIDE_PACKAGES` per-language

`dispatch_pipeline()` instantiates all engines via a large if/elif chain with lazy imports.

### CLI Dispatch (`cli/dispatch.py`)

`_dispatch()` is the main CLI entry point. It:
1. Discovers files via `glob` (by extension)
2. For pooled mode: creates `ProcessPoolExecutor`, initializes pipeline in each worker
3. For non-pooled mode: processes files sequentially in the main process
4. Each command defines `loader` and `writer` callbacks inline in `cli.py`
5. Progress displayed via `rich.progress`

**File input:** Strictly `IN_DIR OUT_DIR` — two positional arguments, both must be directories.
No support for individual files, file lists, or `--output` flag.

---

## Command-by-Command Audit

### 1. `align` — Forced Alignment

**Data flow:** CHAT file → `CHATFile.doc` → FA engine `.process(doc)` → `CHATFile.write()`

**Engines:**
- `WhisperFAEngine` (`fa/whisper_fa.py`) — default for non-English
- `Wave2VecFAEngine` (`fa/wave2vec_fa.py`) — default for English

**Algorithm:**
1. Groups utterance words into ~20s (Whisper) or ~15s (Wav2Vec) segments
2. Strips punctuation, detokenizes words
3. Runs acoustic model on audio chunk
4. Aligns model output to reference words using character-level DP alignment (`utils/dp.py`)
5. Post-processes: sets word end times, bounds to utterance timing, drops invalid timings
6. Injects `\x15start_end\x15` timing bullets into utterance text via regex

**Caching:** Group-level caching via `AlignmentCacheKey` (hash of audio chunk + text + pauses flag).

**Known issues:**
- Word splitting: Short uppercase words like `AM` get split into individual characters by the FA model
- Word merging: Adjacent words merged in Chinese/Indonesian due to acoustic model limitations
- Regex text manipulation: `re.sub(r"\x15\d+_\d+\x15", ...)` directly edits CHAT text — fragile
- The 500ms fallback (`w.time[0]+500`) when end time can't be determined is a guess
- UTR (utterance-to-recording) alignment auto-added via `resolve_engine_specs` — uses Whisper ASR to recover utterance-level timing before word-level FA

### 2. `transcribe` — ASR

**Engines:**
- `RevEngine` (`asr/rev.py`) — Rev.AI cloud API (default)
- `WhisperEngine` (`asr/whisper.py`) — local HuggingFace Whisper
- `WhisperXEngine` (`asr/whisperx.py`) — WhisperX with alignment
- `OAIWhisperEngine` — OpenAI's Whisper implementation

**Algorithm (all engines):**
1. Submit/run audio through ASR model
2. Post-process via `process_generation()` in `asr/utils.py`:
   - Merge compound words (`merge_on_wordlist`)
   - Convert numbers to words via `num2words` / `num2chinese`
   - Split multi-space words, handle dash-joined words
   - Retokenize into utterances via `retokenize()` (punctuation-based) or `retokenize_with_engine()` (Stanza-based)
3. Build `Document` with `Form` objects, `Tier` per speaker

**Known issues:**
- Rev.AI is cloud-only (requires API key, network)
- Speaker tiers auto-named `PAR0`, `PAR1`, etc. — no real speaker identification
- Large ASR output (>300 words) is arbitrarily split into 300-word chunks
- Number-to-word conversion has edge cases for languages without `num2words` support

### 3. `morphotag` — Morphosyntactic Analysis

**Engine:** `StanzaEngine` (via the large `ud.py` file)

**Algorithm:**
1. For each utterance, extract cleaned text
2. Run Stanza NLP pipeline (tokenize, pos, lemma, depparse)
3. Align Stanza tokens to CHAT words via character-level DP
4. Map UD POS/features to CHAT morphology format:
   - `handler()` for generic words
   - Language-specific handlers (Japanese verb forms, etc.)
   - Feature string generation with `-Feat1-Feat2` format
5. Map UD dependency relations to CHAT `%gra` format
6. Optionally retokenize main tier to match UD tokenization

**Caching:** Utterance-level via `MorphotagCacheKey` (hash of text + lang + retokenize + mwt).

**Known issues:**
- The `ud.py` file is ~1,200 lines of complex mapping logic with many language-specific handlers
- POS tag mapping produces verbose forms: `verb|eat-Fin-Ind-Past`, `pron|I-Prs-Nom-S1`
- ROOT head uses UD convention (`N|0|ROOT`) — TalkBank convention is self-referencing (`N|N|ROOT`)
- Relation subtypes use UD colons (`acl:relcl`) — TalkBank uses dashes (`ACL-RELCL`)
- 84 mypy errors on master (type safety gaps in this file and elsewhere)
- Retokenize mode has separator word counter desync bugs (documented in CLAUDE.md)
- No batched processing — each utterance is a separate Stanza call

### 4. `translate` — Translation

**Engines:**
- `GoogleTranslateEngine` (`translate/gtrans.py`) — default, uses `googletrans` async API
- `SeamlessTranslationModel` (`translate/seamless.py`) — Facebook SeamlessM4T

**Algorithm:**
1. Extract text from each utterance
2. Send to translation API/model
3. Store result in `utterance.translation`
4. Serialized as `%xtra` tier

**Known issues:**
- Google Translate API is unofficial (googletrans library), may break without notice
- SeamlessM4T requires large model download (~2.5 GB)
- No caching of translation results

### 5. `utseg` — Utterance Segmentation

**Engine:** `StanzaUtteranceEngine` (`utterance/ud_utterance.py`)

**Algorithm:**
1. For each long utterance, run Stanza constituency parser
2. Identify clause boundaries in the parse tree
3. Split utterance at clause boundaries via DP alignment
4. Distribute word timings to new sub-utterances

**Caching:** Utterance-level via `UtteranceSegmentationCacheKey`.

**Known issues:**
- Requires Stanza constituency parser — only available for a subset of languages (English, Italian, Japanese, Portuguese, Spanish, Turkish, Vietnamese, Indonesian, Choctaw)
- Fails silently for unsupported languages (Catalan, Dutch, etc.)
- Large commented-out code block at end of file

### 6. `opensmile` — Audio Feature Extraction

**Engine:** `OpenSMILEEngine` (`opensmile/engine.py`)

**Algorithm:**
1. Load audio file via `opensmile.Smile`
2. Extract features using specified feature set (eGeMAPSv02, GeMAPSv01b, ComParE_2016)
3. Return DataFrame with features

**Known issues:**
- Returns a dict, not a Document — doesn't follow the standard engine pattern cleanly
- Output is CSV, not CHAT

### 7. `avqi` — Acoustic Voice Quality Index

**Engine:** `AVQIEngine` (`avqi/engine.py`)

**Algorithm:**
1. Find paired `.cs` (continuous speech) and `.sv` (sustained vowel) audio files
2. Filter below 34Hz (stop Hann band)
3. Extract voiced segments from continuous speech
4. Trim sustained vowel to last 3 seconds
5. Concatenate voiced CS + SV
6. Compute CPPS, HNR, shimmer, LTAS slope/tilt
7. Calculate AVQI score from formula

**Known issues:**
- Requires paired files with specific naming convention (`.cs.wav` + `.sv.wav`)
- Not supported in server mode (paired-file architecture doesn't map to the job model)
- Bare `except:` clauses in `extract_voiced_segments` swallow errors
- Creates temporary mono WAV files, cleaned up in `finally` block

### 8. `benchmark` — WER Evaluation

**Engine:** `EvaluationEngine` (`analysis/eval.py`)

**Algorithm:**
1. Load gold `.cha` transcript alongside audio file
2. Run ASR pipeline on audio
3. Conform both gold and hypothesis words (expand contractions, normalize)
4. Align via DP to compute substitutions, deletions, insertions
5. Calculate WER = (S + D + I) / N

**Output:** `.wer.txt` (WER score), `.diff` (word-level alignment diff), `.asr.cha` (ASR output)

---

## Cross-Cutting Concerns

### TextGrid Import/Export (`formats/textgrid/`)

**Export (`generator.py`):**
- `_extract_tiers()` iterates `utterance.content` for each word
- Words without `word.time` are **interpolated** from neighboring timed words
- Warning emitted for words that can't be interpolated (no neighbors)
- Times converted from ms to seconds for Praat

**Import (`parser.py`):**
- Word-level: each interval → `Form`, grouped into `Utterance` by speaker turn changes
- Utterance-level: each interval → `Utterance`

### Caching System (`pipelines/cache.py`)

SQLite-based at `~/.cache/batchalign/cache.db`:
- Thread-safe with WAL mode
- Key generators: `MorphotagCacheKey`, `UtteranceSegmentationCacheKey`, `AlignmentCacheKey`
- Batch get/put operations for efficiency
- `--override-cache` flag bypasses reads (still writes)
- No automatic invalidation — cache persists across versions

### Dispatch Architecture (`cli/dispatch.py`)

- `ProcessPoolExecutor` for multi-file runs when engines are pool-safe
- Global `_worker_pipeline` cached in each worker process
- `POOL_UNSAFE_ENGINES` set prevents pooling for GPU-heavy engines
- Adaptive worker management with memory guards and RSS tracking
- No auto-tuning based on RAM — relies on manual `--workers` or defaults to CPU count

### Error Handling

- CHAT parse errors → `CHATValidationException` → entire file fails, no recovery
- Engine errors → typically unhandled, propagate as tracebacks
- Worker crashes → lost results, no retry
- No structured logging — uses Python `logging` with `rich` handler
- Verbose mode (`-v` to `-vvvv`) controls log levels

---

## Known Bugs Summary

| Bug | Severity | Scope | Description |
|-----|----------|-------|-------------|
| `%wor` decode() override | **Major** | 3,735 files, 22,908 errors | Nonwords/fragments in retrace groups leak into %wor |
| `%wor` shortened forms | Minor | Cosmetic | Raw CHAT notation `(r)` kept in %wor instead of expanded form |
| No lenient parsing | **Medium** | All files | One error fails entire file — no error recovery |
| `%gra` ROOT convention | Medium | All morphotag output | Uses UD `0` instead of TalkBank self-reference |
| `%gra` relation separators | Medium | All morphotag output | Uses UD colons instead of TalkBank dashes |
| Separator word desync | Medium | Retokenize mode | Word counter skips separators, causing misalignment |
| 84 mypy errors | Medium | Codebase-wide | No type annotations on most code |
| No structured run logging | Minor | Operations | No way to post-mortem analyze failed runs |
| FA word splitting | Minor | Alignment | Short uppercase words split into characters |
| No server mode | **Major** | Operations | All processing must be local — no remote job submission |
| Fixed IN_DIR OUT_DIR | Medium | CLI | No file-level input, no file lists, no `--output` flag |
| No `--server` option | **Major** | Deployment | Cannot offload processing to a server with media access |
