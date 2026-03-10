# Analysis 25: Python/Rust Boundary Audit Report

> **Last updated**: 2026-02-13
> **Status**: Post-implementation audit. Original written pre-migration; updated to reflect actual state.

This report documents the interaction points between Python and the Rust core, specifically focusing on how strings are handled, tracked (provenance), and processed.

## 1. Pipeline & Data Flow Table

| Pipeline / Tool | Receive from Python | Processed As | Mechanism | Return to Python | Passive Stub? |
|:---|:---|:---|:---|:---|:---|
| **parse** | `PythonChatText` | Full AST Construction | `talkbank-tree-sitter-parser` (Strict) | `ParsedChat` Handle | N/A |
| **parse --lenient** | `PythonChatText` | Full AST w/ Recovery | `talkbank-tree-sitter-parser` (Lenient) | `ParsedChat` Handle | N/A |
| **build** | `PythonTranscriptJson` | Structured reconstruction | `serde_json` → `talkbank-model` | CHAT Text (String) | N/A |
| **morphosyntax** | Raw Stanza UD JSON | `UdWord` → `Mor`/`Gra` AST | `mapping.rs` (Rust) | Mutates `ParsedChat` | **YES** |
| **forced alignment** | Raw timestamps JSON | `FaWordTiming` → timing AST | `forced_alignment.rs` + DP aligner | Mutates `ParsedChat` | **YES** |
| **translate** | `{"translation": "..."}` | `%xtra` tier injection | `inject.rs` → AST mutation | Mutates `ParsedChat` | **MOSTLY** (minor Python post-processing) |
| **utterance segmentation** | `{"assignments": [...]}` | Utterance boundary splitting | `utterance_segmentation.rs` | Mutates `ParsedChat` | **NO** (Python does tree-walking logic) |
| **coreference** | Stanza coref chains + tier entries JSON | `%xcoref` user-defined tier | `handle.extract_nlp_words("mor")` + `handle.add_dependent_tiers(...)` | Mutates `ParsedChat` | **NO** (Python still performs coref inference) |
| **extract** | N/A (Internal) | NLP-ready text extraction | `extract.rs` (Wor/Mor/Pho domains) | JSON String | N/A |

## 2. Provenance/Phantom Type System — Current State

### What Exists
`Provenance<M, T>` wraps data with a phantom marker type (`provenance.rs`). Marker types:
- **Origins**: `FromPython`, `NlpProduced`, `ChatOriginal`
- **Intents**: `RawChatText`, `TranscriptJson`, `AsrWordsJson`, `LanguageId`, `AlignmentDomainMarker`, `TokenizedWords`, `Morphosyntax`, `NlpResponseJson`
- **Aliases**: `PythonChatText`, `PythonTranscriptJson`, `PythonAsrWordsJson`, `PythonLanguageId`, `PythonAlignmentDomain`, `NlpTokens`, `NlpResponse`

### Actual Usage Audit (lib.rs)

| Function | Marker | Correct? | Notes |
|:---|:---|:---|:---|
| `py_parse()` | `PythonChatText` | YES | |
| `py_parse_lenient()` | `PythonChatText` | YES | |
| `py_build()` | `PythonTranscriptJson` | YES | |
| `py_extract_nlp_words()` | `PythonAlignmentDomain` | YES | |
| `py_add_morphosyntax()` | `PythonLanguageId` | YES | |
| `py_add_morphosyntax_batched()` | `PythonLanguageId` | YES | |
| `py_add_disfluency_markers()` | `Provenance<AsrWordsJson>` x2 | **WRONG** | These are disfluency patterns and replacements, not ASR words |
| `py_add_retrace_markers()` | `PythonLanguageId` | YES | |
| `py_reassign_speakers()` | `Provenance<AsrWordsJson>` | **WRONG** | These are diarization segments, not ASR words |
| `py_add_utterance_timing()` | `PythonAsrWordsJson` | YES | |
| `py_add_dependent_tiers()` | `Provenance<AsrWordsJson>` | **WRONG** | Arbitrary tier JSON (coref, etc.), not ASR words |

### Assessment
The provenance system prevents argument transposition at the PyO3 call site — you can't pass a `PythonChatText` where a `PythonTranscriptJson` is expected. That's real value.

**However, it falls short in three ways:**

1. **Immediate unwrapping**: Every function strips the wrapper (`let text = chat_text.data;`) on the first line. Inner functions all accept bare `&str`. Type safety evaporates one line past the boundary.

2. **`AsrWordsJson` used as catch-all**: Four functions use `Provenance<AsrWordsJson>` for data that is not ASR words (disfluency patterns, diarization segments, dependent tiers). The phantom type is actively lying about provenance.

3. **No internal tracking**: `ChatOriginal` and `NlpProduced` markers exist but are never used to tag data flowing through the pipeline. Extraction returns plain JSON strings. The vision of tracking "which strings came from CHAT vs NLP" is not implemented.

### Recommendations (unchanged, still valid)
- Add dedicated markers: `DisfluencyPatternsJson`, `DiarizationSegmentsJson`, `TierEntriesJson`
- Keep provenance wrappers deeper into inner functions instead of immediate unwrap
- Consider validation on construction (e.g., `PythonChatText::new()` could check for `@UTF8`)

## 3. Identified Code Smells

### A. Orphan Rule Workaround in `talkbank-model`
**Status**: Unchanged. `talkbank-model` has optional `pyo3` dep for `FromPyObject` impl. Acceptable.

### B. Duplicate Logic in `build_chat`
**Status**: Unchanged. Both standalone `build_chat` and `ParsedChat::py_build` call `build_chat_inner`.

### C. Glob Re-export Ambiguity
**Status**: Unknown if resolved. Was a compiler warning about `AlignmentDomain`.

### D. Result Mapping Verbosity
**Status**: NOT FIXED. 24 instances of `map_err(|e| PyValueError::new_err(...))` in lib.rs. The double-map pattern (`.map_err(|e| e.to_string())` inside `py.detach()` closure + `.map_err(|msg| PyValueError::new_err(msg))` outside) is especially verbose. No `From<CoreError> for PyErr` exists.

### E. Dead Code / Unused Fields
**Status**: Likely still present for phantom marker types. Minor.

## 4. Silent/Default Behaviors — Current State

### A. Terminator Skipping in Retokenization
**Status**: IMPLEMENTED and working. `is_ending_punct()` in `retokenize.rs` covers `.`, `?`, `!`, `+...`, `+/.`, `+//.`, `+/?`, `+//?`, `+..?`, `+"."`, `+"/."`.

**Update**: retokenize now compares skipped ending punctuation against the utterance terminator and emits diagnostics + `ParseHealthTier::Main` taint on mismatch.

### B. Fallback to `Word::new_unchecked`
**Status**: STILL USED as fallback in two sites in `retokenize.rs`:
- `parse_token_as_bracketed_item()`
- `parse_token_as_word()`

**Update**: fallback now emits diagnostics and taints the utterance main tier (`ParseHealthTier::Main`) so downstream alignment checks can treat the recovery path explicitly.

### C. Automatic Separator Conversion
**Status**: FULLY IMPLEMENTED and tested. Comma (`,`), tag (`„`), vocative (`‡`) tokens are mapped to `Separator` AST nodes. Regression test at `retokenize.rs` line 1010 verifies this.

### D. Bare Punctuation Rejection
**Status**: IMPLEMENTED. `Word::new` returns `Result` and rejects bare punctuation. `Word::new_unchecked` bypasses this — still used in 2 production fallback sites.

## 5. Integrity Assessment

### The "Permissive but Blind" Problem — PARTIALLY REDUCED
Recent updates added diagnostics and tainting for:
- Terminator mismatch during retokenize skips
- `new_unchecked` fallback in retokenize
- Utseg assignment length/JSON mismatches

Remaining silent-recovery areas still exist outside these paths.

### ParseHealth System — EXISTS BUT NOT WIRED
`ParseHealth` struct exists in `talkbank-model` at `model/file/utterance/parse_health.rs`:
- Per-utterance boolean cleanliness flags for each tier
- `taint()`, `is_clean()`, `can_align_main_to_mor()` methods
- Used by tree-sitter and direct parsers during parsing

**Critical gap (reduced, still open)**: `batchalign-core` now taints parse-health in retokenize and utseg mismatch recoveries, but broad propagation across all mutation paths (especially FA and other recovery branches) is still incomplete.

### Recommendations (still valid)
1. **Expand ParseHealth wiring**: Apply explicit tainting across all mutation/recovery branches, not only retokenize + utseg.
2. **Broaden diagnostics coverage**: Ensure FA and other fallback paths emit structured warnings.
3. **`--strict` mode**: Hard-fail when silent recovery is triggered.

## 6. Data Extraction & Exchange — Current State

### A. Outbound (Rust → Python)
Unchanged from original description. Full AST walk via `extract.rs`, structural selection by `AlignmentDomain`, JSON serialized `ExtractedUtterance`. No string hacking.

### B. Inbound — What Python Actually Returns Now

**Morphosyntax**: Raw Stanza UD JSON (`doc.to_dict()` → `{"sentences": [...]}`). Rust `mapping.rs` does all UD→CHAT mapping. **This is the passive stub in action.** Python does zero linguistic processing.

**Forced Alignment (Whisper/Wave2Vec, NOT MFA)**: Raw model output. Whisper returns `{"tokens": [{"text": ..., "time_s": ...}]}`. Wave2Vec returns `{"timings": [{"word": ..., "start_ms": ..., "end_ms": ...}]}`. Rust does DP alignment.

**Translation (Google Translate)**: Returns `{"translation": "..."}`. Minor Python post-processing remains: fullwidth period → period, smart quotes → straight quotes, space before CHAT punctuation. Mild but not zero.

**Utterance Segmentation**: Returns `{"assignments": [0, 0, 1, 1, ...]}`. **NOT a passive stub** — Python does substantial logic: constituency tree parsing via Stanza, recursive S-level boundary detection, phrase range extraction, short-utterance merging (~100 lines of logic in `_compute_assignments()`).

**Coreference**: Runs on the `ParsedChat` handle API (`process_handle`). Python extracts words via `handle.extract_nlp_words("mor")`, runs Stanza coref, and writes `%xcoref` through `handle.add_dependent_tiers(...)`.

### C. Exchange Loop Assessment
The asymmetry identified in the original report has been **partially resolved** for morphosyntax and FA (Python now returns raw data, Rust does interpretation). It remains asymmetric for utterance segmentation and coreference because Python still performs model-side interpretation before returning compact outputs.

## 7. Regression Prevention

### A. `ud.py` Status
**DELETED** from source tree. The 1000+ line Python UD→CHAT mapping file no longer exists in `batchalign/pipelines/morphosyntax/`. The Rust mapping engine at `talkbank-model/src/model/nlp/mapping.rs` (688 lines) has replaced it with:
- `map_ud_sentence()`: Full UD→Mor+Gra mapping
- `map_ud_word_to_mor()`: Per-word POS/suffix mapping with Validated Direct Construction
- `assemble_mors()`: MWT/clitic assembly
- `apply_language_specific_fixes()`: Gerund fix (English), comma mapping (Japanese)
- `map_pos()`, `map_suffixes()`: UD→CHAT category/feature mapping

**Current status**: stale `ud.py` imports have been removed from the active test tree.

### B. Coverage Gap
The Rust mapping engine now covers Python-style POS labels/features substantially better than the early rewrite. Remaining risk is concentrated in tokenization/retokenization edge cases and language-specific morphology corners rather than basic POS coverage.

## 8. The Pre/Post-Processing "Swapping Game"

### A. Terminator Swap
**Status**: WORKING. Rust extracts terminators as structural fields. Python no longer normalizes them (passive stub returns raw UD). Rust discards any punctuation tokens returned by Stanza via `is_ending_punct`.

### B. Special Form Protection (@c, @b, etc.)
**Status**: WORKING. The passive stub callback still replaces special-form words with `xbxxx` placeholder before calling Stanza. This is the one piece of preprocessing that remains in the "passive" callback — it's minimal and necessary.

### C. Structural Separators (Commas)
**Status**: WORKING. Fully implemented with regression test.

### D. Clitic/Contraction Dilemma
**Status**: WORKING. Rust's `retokenize.rs` handles N:M token mappings via character-level DP alignment.

## 9. The "Surprise" Matrix

Updated to reflect actual engines used:

| Pipeline | Probability | The Surprise | Recovery |
|:---|:---|:---|:---|
| **Stanza (Morpho)** | Moderate (20-30%) | Token count mismatch (splits/merges) | `retokenize.rs` DP alignment + recovery diagnostics + parse-taint on fallback/mismatch |
| **Whisper/Wave2Vec FA** | Moderate (30-40%) | Fewer timestamps than words (OOV drops, hallucinations) | `forced_alignment.rs` DP "best fit". **No diagnostics.** |
| **Google Translate** | Low (5-10%) | Semantic bloat, hallucinated annotations | Imported verbatim into `%xtra`. **No filtering.** |
| **Stanza (Segmentation)** | Low (2-5%) | Assignment array length mismatch | Keep utterance unchanged with warning diagnostics; parse-taint set on mismatched utterances |

*Note: Original doc referenced MFA (Montreal Forced Aligner). We use Whisper FA and Wave2Vec FA, not MFA.*

## 10. Remediation Strategies

Status of each recommendation:

| Recommendation | Status |
|:---|:---|
| Enforce `pretokenized=True` in Stanza | **PARTIAL** — keep-tokens mode for non-MWT languages uses `tokenize_pretokenized=True`; retokenize and MWT-capable keep-tokens paths use `tokenize_no_ssplit=True` with postprocessor/realignment |
| Fuzzy DP alignment with ParseHealth warnings | **PARTIAL** — retokenize now emits diagnostics and taints on fallback/mismatch |
| Custom session dictionary for FA OOV | **NOT DONE** |
| Translation sanity filter | **NOT DONE** |
| Pre-validate segmentation assignments | **PARTIAL** — mismatch/invalid JSON now warned and utterance is parse-tainted |
| Centralized `From<CoreError> for PyErr` | **NOT DONE** — 24 instances of `map_err` boilerplate remain |
| `--strict` mode for hard-fail on massage | **NOT DONE** |
| ParseHealth warnings on silent recovery | **PARTIAL** — wired for retokenize + utseg mismatch recoveries |

## 11. The Passive Stub Architecture — MOSTLY ACHIEVED

### Current State
The original report described the Python callbacks as performing linguistic mapping, string hacking, and information loss. **This has been largely fixed:**

| Callback | Was | Now |
|:---|:---|:---|
| Morphosyntax (`_stanza_callback.py`) | `parse_sentence()` + UD→CHAT mapping | **Passive stub**: `nlp(text).to_dict()` |
| Morphosyntax (`_stanza_batch_callback.py`) | Same | **Passive stub**: batched `nlp(text).to_dict()` |
| FA Whisper (`_fa_callback.py`) | Model-specific processing | **Passive stub**: raw timestamps |
| FA Wave2Vec (`_fa_callback.py`) | Model-specific processing | **Passive stub**: raw timestamps |
| Translation (`_translate_callback.py`) | | **Mostly passive**: calls Google Translate, minor post-processing (fullwidth chars, smart quotes) |
| Utterance segmentation (`_utseg_callback.py`) | | **NOT passive**: ~100 lines of tree-walking, merging logic |
| Coreference (`coref.py`) | | **NOT passive**: drives coref inference, but now uses handle API and writes `%xcoref` |

### Remaining Work
1. **Utterance segmentation**: Move constituency-tree boundary logic to Rust. Python should just call `nlp(text)` and return the raw parse tree JSON.
2. **Coreference**: Keep `%xcoref` as user-defined output; if TalkBank standardizes a coref tier, align labeling/serialization rules at that time.
3. **Translation post-processing**: Move the fullwidth/smart-quote cleanup to Rust.

### The Rust Mapping Engine
**IMPLEMENTED** at `talkbank-model/src/model/nlp/mapping.rs`. `ud.py` has been deleted. The mapping engine uses Validated Direct Construction — no intermediate string serialization, no regex hacking, field-to-field mapping from `UdWord` to `Mor`/`Gra` structs with lemma sanitization.

## 12. Implementation Principles — Validated

### A. Validated Direct Construction
**IMPLEMENTED.** `map_ud_word_to_mor()` maps UD fields directly to `MorWord` and `Mor` struct fields.

### B. Individual Validation & Safe Assembly
**IMPLEMENTED.** Lemma sanitization (`sanitize_mor_text`), MWT/clitic assembly via `assemble_mors()`.

### C. Zero-Hacking Mapping
**PARTIALLY IMPLEMENTED.** The mapping is field-to-field. However, unmappable values are NOT logged as `ParseHealth` warnings — they are silently defaulted or skipped. The "transparent audit trail" does not exist yet.

## 13. Known Bugs Found During This Audit

### A. `%coref` Labeling
**Resolved**: coreference now emits `%xcoref` (user-defined tier naming contract preserved).

### B. Stale `ud.py` Test Imports
**Resolved**: active tests no longer import deleted `ud.py`.

### C. Unknown Tier Handling
No coref-specific parser gap remains after `%xcoref` migration because `%x*` tiers are first-class user-defined dependent tiers.
