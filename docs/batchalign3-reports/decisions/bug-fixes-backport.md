# Bug Fixes and Backport Assessment

**Status:** Historical
**Last updated:** 2026-03-15

This is a February 2026 branch/backport assessment. It is preserved for
historical engineering context, not as current release guidance. For current
runtime and build guidance, use:

- `batchalign3/book/src/user-guide/installation.md`
- `batchalign3/book/src/developer/building.md`
- `batchalign3/book/src/developer/python-versioning.md`

**Date:** 2026-02-17
**Branch:** align (239 commits ahead of master)
**Backport branch:** `backport-core-fixes` (7 commits, based on master)

## Branch Topology

```
master ──────────────────────────────────────────── (Python, v0.8.x, production)
  │
  ├── backport-core-fixes ── 7 cherry-picked fixes ─ (merge-ready for master)
  │
  └── align ── 239 commits ──────────────────────── (Rust-backed, batchalign3)
```

The `backport-core-fixes` branch contains pure-Python fixes cherry-picked from
align-branch work. These are self-contained patches that apply cleanly to
master's architecture without requiring Rust or new infrastructure.

---

## Section 1: Already Backported (7 fixes)

These fixes exist on `backport-core-fixes` and can be merged into master with
`git merge backport-core-fixes`.

### 1.1. Git-hash cache invalidation

- **Commit:** `e43e2a3`
- **Severity:** Medium
- **Files:** `pipelines/cache.py`, `tests/pipelines/cache/test_cache.py` (+113, -25)
- **Bug:** Cache entries survived across code changes — stale results from a
  previous batchalign version were silently reused even after bug fixes.
- **Fix:** Cache keys now incorporate the batchalign git hash, so cache entries
  are automatically invalidated when the code changes.

### 1.2. Utseg and coref crash on non-English files

- **Commit:** `521d086`
- **Severity:** **SEVERE** — crashes entire batch
- **Files:** `pipelines/morphosyntax/coref.py`, `pipelines/utterance/ud_utterance.py` (+22, -13)
- **Bug:** `ud_utterance.py` called Stanza's `depparse` processor without
  checking language support. Non-English files (Cantonese, Japanese, etc.)
  caused unhandled exceptions that crashed the entire batch run.
- **Fix:** Language support check before pipeline construction; unsupported
  languages skip gracefully with a warning.

### 1.3. Translation caching + 90-day pruning

- **Commit:** `d09514b`
- **Severity:** Medium (performance)
- **Files:** `cli/cache.py`, `pipelines/cache.py`, `pipelines/translate/gtrans.py`, `pipelines/translate/seamless.py` (+114, -4)
- **Bug:** Translation results were never cached. Running the same file twice
  made redundant API calls (Google Translate) or redundant GPU inference
  (SeamlessM4T).
- **Fix:** Adds translation caching to the SQLite cache with 90-day automatic
  pruning of stale entries.

### 1.4. FA cache key determinism

- **Commit:** `656a537`
- **Severity:** **CRITICAL** — silent data corruption
- **Files:** `models/utils.py`, `pipelines/fa/wave2vec_fa.py`, `pipelines/fa/whisper_fa.py` (+27, -5)
- **Bug:** Forced alignment cache keys used `id(audio_array)` (Python object
  identity) — a non-deterministic value that changes every run. Cache lookups
  never matched, so the cache was useless. Worse, if Python reused an object ID
  for a different audio chunk, stale results would be returned.
- **Fix:** Cache keys now use a deterministic audio fingerprint (hash of a
  sample of the audio content).

### 1.5. Tab continuation line merging

- **Commit:** `9fd75d8`
- **Severity:** Medium
- **Files:** `formats/chat/file.py` (+10, -1)
- **Bug:** The `CHATFile(lines=...)` constructor did not merge CHAT continuation
  lines (lines starting with a tab character). Multi-line utterances and headers
  were split into separate entries, causing downstream parse failures.
- **Fix:** Pre-merge continuation lines before processing, matching the behavior
  of the `CHATFile(path=...)` constructor.

### 1.6. CHAT parser `str.strip()` eats language codes

- **Commit:** `3b5064d`
- **Severity:** **CRITICAL** — silent data corruption
- **Files:** `formats/chat/parser.py` (+5, -4)
- **Bug:** Python's `str.strip()` removes all matching characters from both
  ends. When stripping punctuation from language header values, it also stripped
  characters that happened to be punctuation-like, silently corrupting language
  codes (e.g., `zho` became `zh`).
- **Fix:** Use targeted stripping that only removes the intended characters.

### 1.7. Audio processing perf + OOM fallback

- **Commit:** `cb0b5b9`
- **Severity:** Medium (reliability)
- **Files:** `cli/cli.py`, `models/utils.py`, `models/whisper/infer_asr.py`, `pipelines/asr/rev.py`, `pipelines/asr/whisper.py`, `pipelines/asr/whisperx.py` (+91, -18)
- **Bug:** Multiple audio processing issues: no cache size limit caused disk
  exhaustion; stereo-to-mono conversion was inefficient; WhisperX had no OOM
  recovery; Rev.AI had no exponential backoff on API errors.
- **Fix:** Adds LRU eviction for audio cache, efficient mono conversion, OOM
  fallback (reduce batch size), and Rev.AI retry with backoff.

---

## Section 2: Recommended for Backport (6 more fixes)

These are pure-Python fixes that work within master's existing architecture.
Each can be implemented as a new commit on `backport-core-fixes`.

### 2.1. Unsupported language crash in StanzaEngine

- **Severity:** High — crashes entire batch
- **Master file:** `batchalign/pipelines/morphosyntax/engine.py`
- **Bug:** When a CHAT file declares a secondary language that Stanza doesn't
  support (e.g., Swahili via `@Languages: eng, swa`), `_ensure_pipelines()`
  calls `stanza.Pipeline(lang="sw")` which throws an unhandled exception,
  crashing the entire batch.
- **Fix:** Wrap Stanza pipeline construction in try/except. On failure, log a
  warning with the unsupported language and continue processing with the
  remaining supported languages. ~15 lines.
- **Align-branch ref:** Commit `0c4c493`

### 2.2. UTR cache ignores `--override-cache`

- **Severity:** Medium — stale results served silently
- **Master file:** `batchalign/pipelines/utr/whisper_utr.py`
- **Bug:** The UTR (Utterance Timing Recovery) cache unconditionally returns
  cached ASR results without checking the `override_cache` flag. Running
  `batchalign --override-cache align` bypasses the FA cache but still serves
  stale UTR results. If UTR cached results from a buggy run, those wrong
  timestamps persist indefinitely.
- **Fix:** Add `override_cache` check to the UTR cache lookup, matching what
  the FA cache already does. ~5 lines.
- **Align-branch ref:** `whisper_utr.py` + commit `1933476`

### 2.3. @Media header shows cache hash instead of filename

- **Severity:** Medium — CLAN compatibility broken
- **Master file:** `batchalign/pipelines/asr/whisper.py` (and `whisperx.py`, `rev.py`, `oai_whisper.py`)
- **Bug:** When transcribing MP4 files, the media format conversion cache
  produces a WAV file with a content-hash filename (e.g.,
  `c7bccfb0e0ed4f5ec327b1c8.wav`). ASR engines use this cached filename as
  the `@Media` header value instead of the original filename (e.g., `2256_T4`).
  This breaks CLAN, which uses `@Media` to locate the original media file.
- **Fix:** Pass the original media name as a kwarg from dispatch through to ASR
  engines. Each engine uses the original name for `@Media` while still reading
  audio from the cached WAV. ~20 lines across dispatch + 4 ASR engine files.
- **Align-branch ref:** Commit `0c4c493`

### 2.4. %wor utterance bullet bug (blocks chained workflows)

- **Severity:** High — blocks align→morphotag pipeline
- **Master file:** `batchalign/formats/chat/generator.py` (around line 105-114)
- **Bug:** `generate_wor_tier()` copies the utterance-level timing bullet into
  the %wor line. The Rust strict parser (and CLAN) rejects this because %wor
  should contain only word-level bullets. This blocks chained workflows:
  `batchalign align` followed by `batchalign morphotag` fails because
  morphotag's strict parse rejects the malformed %wor.
- **Fix:** In `generate_wor_tier()`, set `bullet: None` instead of copying the
  utterance bullet. ~10 lines.
- **Align-branch ref:** Correctness assessment item 1.3. The Rust-side fix was
  committed to `talkbank-utils`, but the Python-side fix in `generator.py` is
  pure Python.

### 2.5. Broken %wor tier recovery (legacy CLAN data)

- **Severity:** Medium — file-level failures on legacy data
- **Master file:** `batchalign/formats/chat/parser.py`
- **Bug:** Legacy CLAN data contains complex %wor tiers with retrace groups,
  events, and annotations — structures that violate the %wor specification.
  Python master's parser fails the entire file when it encounters these.
- **Fix:** Catch malformed %wor parse errors, log a warning with the file and
  line number, and skip the broken %wor tier rather than failing the file.
  The file continues processing without word-level timing (which was corrupt
  anyway). ~20 lines.
- **Align-branch ref:** Correctness assessment item 1.3, 5.2. The Rust parser
  handles this via ERROR node recovery in `parsed.rs`.

### 2.6. %gra circular dependency detection (graceful handling)

- **Severity:** High — 87.5% of morphotag output affected
- **Master file:** `batchalign/pipelines/morphosyntax/ud.py` (around line 487-493)
- **Bug:** The `actual_indicies` array in `ud.py` uses Python list indexing
  (`actual_indicies[elem[1]-1]`) to map UD token indices to CHAT word indices.
  When `elem[1]-1` exceeds the array bounds, Python wraps around to negative
  indices, creating circular head references in %gra. In one test file (Dutch,
  32 utterances), 28/32 utterances (87.5%) had circular dependencies.
- **Root-cause fix:** Requires replacing the array with a HashMap-based index
  mapping (implemented in Rust on batchalign3) — **cannot be backported**.
- **Graceful handling (backportable):** Add post-generation cycle detection.
  After building each %gra tier, walk the head-pointer graph to detect cycles.
  If a cycle is found, emit a warning and either omit the %gra tier or break
  the cycle by pointing the offending word to ROOT. ~30 lines.
- **Align-branch ref:** Correctness assessment item 1.6,
  `docs/gra-correctness-guarantee.md`. The Rust implementation uses
  `validate_generated_gra()` with `has_cycle()` detection (commit `28fd5e8`).

---

## Section 3: Cannot Backport (12 fixes requiring Rust/architecture changes)

These fixes are fundamental to the batchalign3's Rust AST architecture and
cannot be implemented in Python master without equivalent architectural changes.

### 3.1. %wor `decode()` bug (3,735 files, 22,908 errors)

Python's `decode()` in `lexer.py` blindly overrides inner token types when
flattening bracketed groups. Nonwords (`&~li`), fragments (`&+fr`), and
untranscribed material (`xxx`) inside retrace groups leak into %wor.

**Why not backportable:** The bug is in the lexer's fundamental approach to
token type propagation. The fix requires per-word alignability checks on an AST
— Python's flat token stream with `decode()` flattening is architecturally
incompatible. A targeted Python fix would require rewriting significant portions
of the lexer and parser, which would be fragile and untestable without the Rust
AST as a reference.

See: `docs/wor-tier-bug-report.md` (full corpus audit with 22,908 errors across
12 TalkBank collections).

### 3.2. %wor shortened form expansion

Python keeps raw CHAT notation in %wor (`b(r)uixa` instead of `bruixa`).

**Why not backportable:** Requires the Rust AST's `cleaned_text` field, which
reconstructs the target word from CHAT markup. Python's lexer doesn't compute
this — it passes raw token text through.

### 3.3. %gra circular dependencies (root cause)

The `actual_indicies` array-wraparound bug in `ud.py:487-493` creates cycles
in 87.5% of morphotag output (28/32 utterances in test file).

**Why not backportable:** The root cause is that Python uses array indexing
(`actual_indicies[elem[1]-1]`) to map UD→CHAT indices. When indices are
out-of-bounds, Python wraps to negative indices, silently creating circular
references. The fix requires a HashMap-based index mapping with explicit
missing-key handling. While a HashMap could theoretically be used in Python, the
surrounding code (special form handling, index tracking, multi-pass processing)
is tightly coupled — the Rust implementation rewrote the entire mapping pipeline.

Note: Section 2.6 describes a **graceful handling** backport (detect and warn)
but the **root-cause fix** (prevent generation) cannot be backported.

### 3.4. %gra ROOT convention (`N|0|ROOT` → `N|N|ROOT`)

Rust was generating UD-standard `head=0` for ROOT instead of TalkBank
convention `head=self`.

**Why not backportable:** This is fixed in Rust's `mapping.rs`. Python master
already uses its own convention — this was a Rust-specific bug, not a Python
bug. However, Python's ROOT handling has its own issues (item 3.3) that
manifest differently.

### 3.5. %gra relation labels (colons → dashes, uppercase)

Rust was outputting UD-style `acl:relcl` instead of TalkBank-style `ACL-RELCL`.

**Why not backportable:** Same as 3.4 — Rust-specific mapping fix. Python
already uses its own (different but also problematic) label handling.

### 3.6. %gra FLAT abuse

Python forces all special forms (`@c`, `@s`, etc.) to the `FLAT` relation,
regardless of their actual syntactic role.

**Why not backportable:** Requires preserving UD dependency relations through
the Rust POS mapping layer. Python's `ud.py` hardcodes `FLAT` for special forms
(line ~489: `elem = (elem[0], elem[1], "FLAT")`). Fixing this in Python would
require rewriting the special-form handling to preserve Stanza's original
relation, which is deeply entangled with the `actual_indicies` logic.

### 3.7. Single-word retrace brackets

Python produces `<word> [/] word` (with angle brackets) for single-word
retraces. CHAT convention: brackets only for multi-word groups (`word [/] word`
for single, `<word word> [//] word word` for multi).

**Why not backportable:** Requires the Rust AST distinction between
`AnnotatedWord` (single word, no brackets) and `AnnotatedGroup` (multi-word,
brackets). Python's serializer doesn't make this distinction.

### 3.8. MWT (Multi-Word Token) expansion

Contractions like "don't", "that's", "I'm" were not being split into
morphological components (`aux|do~part|not`).

**Why not backportable:** Requires three coordinated changes: (1) switching
Stanza to `tokenize_no_ssplit=True` mode, (2) a tokenizer postprocessor that
merges spurious word splits back to original CHAT words (`_tokenizer_realign.py`
— character-position mapping), and (3) Rust `mapping.rs` MWT Range marker
handling. The postprocessor alone is ~200 lines of non-trivial alignment code.

### 3.9. MultilingualPipeline 0-sentence return

Stanza's `MultilingualPipeline` with `tokenize_pretokenized=True` silently
returns 0 sentences for some inputs.

**Why not backportable:** The fix replaced `MultilingualPipeline` entirely with
per-language `Pipeline` instances managed by a custom routing layer (commit
`ae75f9d`, `352b275`). This is deeply integrated with the batchalign3's
batched callback architecture and cache system.

### 3.10. Morphosyntax cache stores final strings

The old cache stored raw Stanza UD output, which became stale when the POS
mapping changed.

**Why not backportable:** The new cache stores post-Rust-mapping %mor/%gra
strings via three PyO3 methods (`extract_morphosyntax_payloads`,
`inject_morphosyntax_from_cache`, `extract_morphosyntax_strings`). This
requires the Rust mapping layer to exist.

### 3.11. %gra multiple roots from skipped-word fallback

Python's skipped-word fallback in `ud.py` creates extra ROOT relations when
words are not matched by the UD→CHAT index mapping.

**Why not backportable:** The Rust implementation enforces single-root via
`validate_generated_gra()` which panics on multiple roots. The fix is in the
Rust mapping pipeline's HashMap-based approach that eliminates the conditions
causing extra roots. Python's array-based approach with its fallback logic
would need the same full rewrite as item 3.3.

### 3.12. Per-utterance language routing (`[- spa]` items)

Files with per-utterance language markers (`[- spa]`) were processed entirely
with the file's primary language model instead of routing each utterance to the
correct Stanza model.

**Why not backportable:** The fix (commits `ae75f9d`, `352b275`) replaced
`MultilingualPipeline` with per-language `Pipeline` instances and a custom
routing layer integrated with the batched callback architecture. This is
tightly coupled with item 3.9 and the batchalign3's batch processing design.

---

## Section 4: Not Applicable to Master

These fixes address infrastructure that only exists on the batchalign3. Master
has no equivalent code to fix.

### Server bugs (4 fixes)

- Job cancellation didn't update status immediately (commit `43f203e`)
- Crash recovery ignored RUNNING jobs (commit `43f203e`)
- Server port config not applied, always used 8000 (commit `1a991da`)
- MP4 symlink path escape in local dispatch (commit `1a991da`)

### Dashboard/UI bugs (11 fixes)

- React infinite re-render from zustand selectors (`85307f2`, `4b0c4d6`)
- Duplicate route rendering (`84e664d`)
- useStore hook ordering (`4757372`)
- SSE dual-connection bug (`0812e6a`)
- Error status display (`d713b75`, `08d24f6`)
- Progress bar stuck states (`01a724d`)
- WebSocket file notification counts (`73303a6`)
- Per-thread SQLite connections (`aed35be`)
- Frontend asset tracking for wheel builds (`a8cb1b1`)
- Hatch build hook removal (`4659b8e`, `09b609e`)

### Local daemon bugs (2 fixes)

- Stale daemons left running after deploy (`9ba131b`)
- Reuse existing manual server instead of starting second daemon (`cd91db4`)

### Deploy script fixes (2 fixes)

- Kill stale daemons on client deploy (`9ba131b`)
- Retry health check during deploy for model warmup (`a1261f5`)

### Alignment engine bugs (Rust code, 3 fixes)

These bugs were in the Rust forced alignment code (`forced_alignment.rs`) on the
batchalign3. Master's Python FA code has different implementations and
different (or no equivalent) issues.

- Whisper pipeline `batch_size` parameter reset chunk offsets to zero — this
  parameter was added on batchalign3 and never existed on master
- Wave2Vec FA `WordLevel` path missing `audio_offset_ms` — Rust-only code path
- Last word of every utterance silently dropped due to postprocess/bullet
  ordering — Python master has different workarounds (500ms extension fallback)

See: `docs/align-correctness-fixes.md` for full details on all four alignment
bugs.

### Other infrastructure (not in master)

- Structured error propagation from Rust→Python→CLI (commit `403341d`)
- Pre-serialization validation gate (commit `661e9c9`)
- Lenient parsing at pipeline entry (commit `49deb0f`)
- File I/O nested-directory creation for server staging (`b8ece78`, `1cb429a`)
- Progress display / stdout line buffering (`6519f85`)
- Rich markup escaping in error output (`365efad`)
- Rev.AI migration to Rust AST parser (`5867b0a`)
- Cache version bump for MWT invalidation (`2daf023`)

### Abandoned infrastructure (iterated past)

Early align-branch iterations used external queue/orchestration experiments for
job scheduling. These were replaced by the current threading-based
architecture. ~15 fix commits addressed experiment-specific bugs that no longer
apply.

---

## Section 5: Summary and Recommendations

### Stats

| Category | Count | Notes |
|----------|-------|-------|
| Already backported | 7 | On `backport-core-fixes`, merge-ready |
| Recommended for backport | 6 | Pure Python, estimated ~100 lines total |
| Cannot backport (needs Rust) | 12 | Architectural incompatibility |
| Not applicable to master | ~30 | Server, dashboard, daemon, deploy, alignment engine, abandoned infra |
| **Total bug fixes on align** | **~55** | Out of 239 total commits |

### Priority Order for Next Backports

1. **2.6 — %gra circular dependency detection** (High) — 87.5% of morphotag
   output has this bug; graceful handling prevents writing corrupt data
2. **2.1 — Unsupported language crash** (High) — crashes entire batch on
   multilingual corpora
3. **2.4 — %wor utterance bullet** (High) — blocks chained align→morphotag
4. **2.3 — @Media header hash** (Medium) — breaks CLAN compatibility for
   transcribe command
5. **2.2 — UTR cache override** (Medium) — stale timestamps persist silently
6. **2.5 — Broken %wor recovery** (Medium) — file failures on legacy CLAN data

### Recommended Workflow

1. Merge existing `backport-core-fixes` into master:
   ```bash
   git checkout master
   git merge backport-core-fixes
   ```

2. Create new commits on `backport-core-fixes` for items 2.1-2.6, one commit
   per fix, with tests.

3. After validation, merge the updated `backport-core-fixes` into master.

### What Backporting Does NOT Fix

Even with all 13 backported fixes (7 existing + 6 recommended), Python master
still has these unfixable issues:

- **%wor decode() bug** — 3,735 files with 22,908 errors (needs Rust AST)
- **%gra circular dependencies** — root cause persists, only detection added
- **%gra FLAT abuse** — all special forms forced to FLAT relation
- **No MWT expansion** — contractions not split into morphological components
- **No lenient parsing** — single malformed utterance kills entire file
- **No server mode** — all processing must be local
- **No structured errors** — error codes/line numbers lost at parse boundary

These require the Rust-backed batchalign3 (`batchalign3`) to fix.
