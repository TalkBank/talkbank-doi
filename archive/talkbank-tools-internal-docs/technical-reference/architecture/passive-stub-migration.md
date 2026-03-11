# Analysis 26: Passive Stub Migration Plan

> **Last updated**: 2026-02-13
> **Status**: Phases 1-3 largely complete for morphosyntax and FA. Gaps remain for utterance segmentation, coreference, and hardening.

## 1. Goal
Reduce the Python side of `batchalign` to a "passive stub" that merely passes raw data between the Rust core and ML engines (Stanza, Whisper, Wave2Vec, etc.). All "CHAT-specific" logic must reside in Rust.

## 2. Current vs. Target State

| Component | Original State (Python-Heavy) | Current State | Target State (Passive Stub) |
|:---|:---|:---|:---|
| **Morphosyntax callback** | Calls Stanza, runs `parse_sentence()`, returns CHAT `%mor`/`%gra` strings | **DONE**: Returns raw `doc.to_dict()` UD JSON | Same as current |
| **Mapping logic** | `ud.py` (1033 lines) — POS mapping, clitic joining, gerund fixes | **DONE**: `ud.py` deleted. `mapping.rs` (688 lines) in Rust | Same as current |
| **FA callback** | Model-specific timing extraction | **DONE**: Returns raw timestamps JSON | Same as current |
| **Translation callback** | Calls Google Translate, returns string | **MOSTLY DONE**: Returns `{"translation": "..."}` with minor Python post-processing (fullwidth chars, smart quotes) | Move cleanup to Rust |
| **Utseg callback** | Calls Stanza, computes assignments in Python | **NOT DONE**: ~100 lines of tree-walking logic in Python | Return raw constituency tree JSON, compute assignments in Rust |
| **Coreference** | N/A on old master | **BROKEN**: Uses legacy text API, writes non-standard `%coref` tier | Either drop, or migrate to handle API + `%xcoref` |
| **Terminators** | Python "swaps" terminators | **DONE**: Rust sends tokens without terminators, skips terminator tokens returned by Stanza | Same as current |
| **Special forms** | Python replaces with `xbxxx` | **DONE**: Callback substitutes `xbxxx` before Stanza call (minimal, necessary) | Same as current |

## 3. Detailed Migration Steps — With Status

### Phase 1: Define the Raw UD Contract (Rust) — COMPLETE
1. `UdWord`, `UdSentence`, `UdResponse` structs with `serde` exist at `talkbank-model/src/model/nlp/types.rs` (111 lines).
2. The FFI boundary accepts raw UD JSON from the Python callback. `NlpResponse` provenance type wraps it.

### Phase 2: Implement the Rust Mapping Engine — COMPLETE
1. **POS handlers migrated**: `mapping.rs` at `talkbank-model/src/model/nlp/mapping.rs` (688 lines) implements:
   - `map_ud_sentence()` — full UD sentence → Mor + GRA
   - `map_ud_word_to_mor()` — per-word mapping with Validated Direct Construction
   - `assemble_mors()` — MWT/clitic assembly
   - `apply_language_specific_fixes()` — gerund fix (English), comma mapping (Japanese)
   - `map_pos()` — UD UPOS → CHAT POS (PRON→pro, NOUN→n, VERB→v, etc.)
   - `map_suffixes()` — feature → suffix (Number=Plur→PL, Tense=Past→PAST, etc.)
   - `sanitize_mor_text()` — reserved character sanitization
2. **Linguistic fixes migrated**: Gerund fix is in Rust. Clitic/MWT joining uses `assemble_mors()`.
3. **Coverage note**: The Rust engine covers core POS categories and basic features. The legacy `ud.py` had ~20 handler functions with more granular feature extraction. Side-by-side corpus comparison needed to verify parity.

### Phase 3: The Python "De-Hacking" — COMPLETE for morphosyntax
1. `_stanza_callback.py` and `_stanza_batch_callback.py` are passive stubs: `nlp(text).to_dict()`.
2. `ud.py` deleted from source tree.
3. **Stale test imports remain** — 3 test files still import from deleted `ud.py`:
   - `tests/pipelines/morphosyntax/test_parse_sentence.py`
   - `tests/pipelines/test_engine_safety.py`
   - `tests/pipelines/morphosyntax/test_ud_cleaning.py`

### Phase 4: Forced Alignment Parity — COMPLETE
1. Both Whisper FA and Wave2Vec FA callbacks return raw timestamps.
2. Rust-side DP alignment handles mismatches.
3. Timestamp monotonicity validation is in Rust.

## 4. Remaining Work

### A. Utterance Segmentation — NOT PASSIVE
**Current**: `_utseg_callback.py` and `_utseg_batch_callback.py` contain `_compute_assignments()` (~100 lines) that:
- Runs Stanza constituency parsing
- Recursively walks parse tree for S-level coordination (CC/CONJ)
- Extracts leaf-index ranges for phrase boundaries
- Deduplicates overlapping ranges
- Merges short utterances (< 3 words)
- Forward-fills unassigned words

**Target**: Python returns raw Stanza constituency parse tree JSON. Rust implements boundary detection, merging, and assignment computation.

**Difficulty**: Medium-high. The tree-walking logic is linguistically motivated and would need careful porting.

### B. Coreference — BROKEN
**Problems**:
1. Uses legacy text-based API (`batchalign_core.extract_nlp_words()` + `add_dependent_tiers()`), not `ParsedChat` handle API.
2. Writes `%coref:\t...` — not a standard CHAT tier, and not a valid user-defined tier (should be `%xcoref`).
3. Loads its own Stanza pipeline on every call (no caching, no warmup).
4. English-only, hardcoded.
5. Zero tests.

**Options**:
- **Drop it**: If TalkBank doesn't need coreference annotations, remove `CorefEngine` and the `coref` CLI command.
- **Fix it**: Migrate to handle API, change label to `"xcoref"`, add caching, add tests. But even then, `%xcoref` content format has no schema or validation.
- **Promote it**: If TalkBank wants to standardize coreference, add a proper `CorefTier` to the Rust model with structured data (chain indices, start/end markers). This is significant work.

### C. Translation Post-Processing
**Current**: Python callback does minor cleanup (fullwidth periods, smart quotes, zero-width spaces, space before CHAT punctuation).
**Target**: Move this to Rust. Small effort — just character-level replacements.

### D. Unknown Tier Handling
**Problem**: The grammar/parser only recognizes standard tiers and `%x`-prefixed user-defined tiers. An unknown tier like `%coref` in existing data may fail to parse.
**Target**: Add lenient parsing for unrecognized tier labels — carry them as `UserDefined` with a warning. This prevents data loss when round-tripping files that contain non-standard tiers from other tools.

## 5. Integrity & Verification

### A. Dual-Path Testing — NOT DONE
The recommended test runner (execute both legacy Python mapping and Rust mapping on same raw UD JSON, diff results) has not been built.

### B. Corpus Parity — NOT DONE
No side-by-side corpus run comparing `%mor` output between master (Python `ud.py`) and align (Rust `mapping.rs`). This is the highest-priority verification task.

### C. Stale Test Cleanup — NEEDED
Three test files import deleted code. These need to be either deleted or rewritten to test the Rust mapping engine through the Python API.

## 6. ParseHealth Integration — NOT DONE
`ParseHealth` struct exists in `talkbank-model` but is not wired into `batchalign-core` mutation functions. None of the "Evidence-Based Processing" recommendations from Analysis 25 have been implemented:
- No taint when `new_unchecked` fires
- No warning when terminator mismatches
- No diagnostic when segmentation silently aborts
- No structured error propagation (still 24 instances of `map_err` boilerplate)

## 7. Milestones — Updated

| Milestone | Status |
|:---|:---|
| `UdWord` struct and `serde` deserialization in Rust | **COMPLETE** |
| POS handlers migrated to Rust and passing unit tests | **COMPLETE** |
| Python callback simplified to passive stub | **COMPLETE** (morphosyntax, FA) |
| Final workspace cleanup (delete legacy Python mapping) | **MOSTLY COMPLETE** (`ud.py` deleted, stale tests remain) |
| Corpus parity verification | **NOT STARTED** |
| Utterance segmentation passive stub | **NOT STARTED** |
| Coreference fix or removal | **NOT STARTED** |
| ParseHealth wiring | **NOT STARTED** |
| Error handling centralization | **NOT STARTED** |
