# Python vs Rust Discrepancies — Running Log

Observed differences between Python batchalign (master) and Rust-backed align branch.
Maintained as correctness testing progresses.

## Correctness Test Reference

```bash
# Run correctness tests (must use batchalign2 venv for batchalign_core)
uv run --directory ~/batchalign2 python ~/batchalign-benchmarking/scripts/run_correctness.py --commands morphotag --verbose
```

Results at `~/batchalign-benchmarking/results/correctness/`

---

## 1. %wor — Python `decode()` Bug (MAJOR, 3,735 files affected)

**Status:** Rust is correct, Python is buggy. See also `docs/wor-tier-bug-report.md`.

Python's `decode()` in `lexer.py` blindly overrides inner token types when flattening bracketed groups. Nonwords (`&~li`), fragments (`&+fr`), and untranscribed material (`xxx`) inside retrace groups (`<...> [/]`) get their type overridden from ANNOT to RETRACE, leaking them into %wor.

Rust avoids this by using per-word `word_is_alignable()` checks on the AST — group membership doesn't affect word category.

**Scope:** 22,908 individual errors across 12 TalkBank collections (3.8% of all files).

**Action:** Re-run `batchalign align` on affected files with Rust backend.

## 2. %wor — Shortened Form Expansion

**Status:** Rust behavior is correct.

| Input | Python %wor | Rust %wor | Correct? |
|-------|------------|-----------|----------|
| `b(r)uixa` | `b(r)uixa` (raw CHAT notation) | `bruixa` (target word) | Rust is correct |
| `do(r)mir` | `do(r)mir` | `dormir` | Rust is correct |
| `(ei)ne` | `(ei)ne` | `eine` | Rust is correct |

CHAT `(x)` means the speaker omitted `x`. The `cleaned_text` field includes the parenthesized content, giving the target/intended word form. This is correct for %wor because:
- %wor shouldn't contain CHAT markup
- The target form is what NLP/FA models recognize

Python kept raw CHAT notation in %wor, which is wrong — %wor should have cleaned text.

## 3. %wor — Other Behavioral Differences

See `docs/wor-tier-python-vs-rust.md` for exhaustive comparison. Summary:

| Form | Python | Rust | Assessment |
|------|--------|------|------------|
| `&~nonword` | Excluded | Included | Rust arguably correct (phonological material) |
| `&+fragment` | Excluded | Included | Rust arguably correct |
| `xxx/yyy/www` | Excluded | Included | Rust arguably correct (occupies audio time) |
| `word [: replacement]` | Uses replacement | Uses original | Rust correct (tracks what was spoken) |
| `&+fr [: friend]` | Uses `friend` | Excluded | Rust correct (fragment not alignable) |

## 4. %mor — POS Tag Category Differences

**Status:** Mostly resolved. The early Rust rewrite diverged from Python in POS labels/features, but current mapping has largely converged back to Python/TalkBank-style categories and suffix behavior.

The old table of "Rust is simpler" examples is now primarily historical context.
Remaining deltas are typically tokenization/retokenization edge cases rather than
systematic POS category remapping.

## 5. %gra — ROOT Points to 0, Not Self — FIXED

**Status:** Fixed. Rust now matches Python/TalkBank convention (ROOT→self).

TalkBank convention: ROOT head points to the word's own index (`3|3|ROOT`).
UD standard: ROOT head = 0 (`3|0|ROOT`).

Rust was using UD standard but TalkBank tooling (`~/gra-cgi/`) and all existing corpus data expect self-referencing ROOT. Fixed in `mapping.rs` to match Python.

See `docs/gra-format-conventions.md` for full discussion.

## 6. %gra — Relation Label Separators — FIXED

**Status:** Fixed. Rust now matches Python/TalkBank convention (dashes).

TalkBank convention: dashes for subtypes (`ACL-RELCL`, `NMOD-POSS`).
UD standard: colons (`acl:relcl`, `nmod:poss`).

Rust was using UD colons but TalkBank tooling expects dashes. Fixed in `mapping.rs` to match Python.

See `docs/gra-format-conventions.md` for full discussion.

## 7. Header Spacing — Trivial

**Status:** Not a bug. Corpus is inconsistent.

Some corpus files use `MOT Mother , CHI` (space-comma-space), others use `MOT Mother, CHI` (comma-space). Rust normalizes to `, `. The correctness check normalizes both before comparing.

## 8. Trailing Whitespace on Main Tier Lines

**Status:** Cosmetic.

Some Python-generated files have trailing spaces on main tier lines (e.g., `*PEA: text . ` with trailing space). Rust strips trailing whitespace. This shows up in gold diffs but is semantically irrelevant.

## 9. TextGrid Export — Interpolation vs Skip

**Status:** Known behavioral difference. Neither is strictly wrong.

**Python (master):** `_extract_tiers()` in `formats/textgrid/generator.py` reads word timing
from `word.time` on each `Form` in `utterance.content`. When a word has no timing, it
**interpolates** from neighboring timed words (searches backward and forward for the nearest
timed neighbors, uses prev.end → next.start as the interpolated interval). If no neighbors
exist, it warns and skips.

**Rust (align):** The align branch stores word timing in the `%wor` tier (via the Rust AST),
not on the `Form` objects in `utterance.content`. The TextGrid exporter on the align branch
reads from `%wor` tier data. Words without `%wor` timing are skipped (no interpolation).

**Impact:** TextGrid files from the align branch may have fewer word intervals than master
for utterances with partially-timed words. The master approach produces a "complete" TextGrid
by guessing timing for untimed words; the align approach only includes words with actual
acoustic evidence.

**Note:** TextGrid export still uses the legacy `Document` model in both branches.

## 10. CHAT Parsing — Lenient Mode (Align Only)

**Status:** New capability in align branch.

**Python (master):** One parsing mode only. Any `CHATValidationException` fails the entire file.
There is no error recovery — a single malformed utterance kills the whole file.

**Rust (align):** Two modes:
- `parse_strict_pure()` → `parse_chat_file()` — rejects on ANY error (used by morphotag, alignment)
- `parse_lenient_pure()` → `parse_chat_file_streaming()` — error recovery, marks tainted tiers (used by `parse_and_serialize`, translation)

Lenient mode enables processing of legacy corpus files that have minor formatting issues
(e.g., broken %wor tiers from CLAN) without failing the entire file.

## 11. %wor Tier Recovery — Broken Tiers Gracefully Dropped

**Status:** New capability in align branch.

Legacy CLAN data contains complex `%wor` tiers with retrace groups, events, and annotations
that are data quality errors (not valid CHAT structure). The tree-sitter grammar has a
dedicated `wor_tier_body` rule that only accepts flat `word [bullet] word [bullet] ...`
structure.

**Python (master):** Fails the file with a parse error.

**Rust (align):** Broken `%wor` tiers produce ERROR nodes in tree-sitter. The lenient parser
gracefully drops them instead of failing the file. Errors are reported but processing continues.
See `parsed.rs` in `dependent_tier_dispatch/`.

## 12. Dispatch Architecture — Multi-Input and Server Mode

**Status:** New in align branch.

**Python (master):** CLI accepts only `IN_DIR OUT_DIR` — two positional directory arguments.
No support for individual files, file lists, or remote execution. All processing is local.

**Rust (align):**
- Multi-input: `PATHS... -o OUTPUT --file-list FILE`
- Backward compat: 2 args where first is dir → legacy `IN_DIR OUT_DIR`
- `--server URL`: offload processing to a remote server (CHAT sent over HTTP, media resolved from server's `media_roots`)
- `--file-list`: process a list of files from a text file
- Per-file media mapping via `_infer_file_media_mapping()` in `serve/jobs.py`

## 13. Batched Callbacks — Morphotag and Utseg

**Status:** New in align branch.

**Python (master):** Each utterance is processed individually — one Stanza call per utterance.

**Rust (align):** `add_morphosyntax_batched()` and `add_utterance_segmentation_batched()` collect
all utterance payloads in a single AST pass, call the Python callback once with a JSON array,
then inject results. Benefits:
- Stanza processes the entire batch efficiently (better GPU utilization)
- Per-item caching integrated into the batch callback
- Single AST traversal instead of N traversals for N utterances

Batch callback modules:
- `batchalign/pipelines/morphosyntax/_stanza_batch_callback.py`
- `batchalign/pipelines/utterance/_utseg_batch_callback.py`

## 14. Processing Server (Align Only)

**Status:** New capability.

The align branch adds a full HTTP processing server (`batchalign/serve/`):
- FastAPI + uvicorn, single process, no external dependencies (no Redis, no message queue)
- Clients submit CHAT content (~2KB) over HTTP; server processes with local GPU + media
- HTMX dashboard at `/dashboard/` for live job monitoring
- SQLite crash recovery — interrupted jobs auto-resume after restart
- Thread-safe `PipelineCache` with lazy model loading shared across jobs
- Auto-tuned concurrency (~25 GB per worker)

Python master has no server mode — all processing must be local.

## 15. Structured Run Logging (Align Only)

**Status:** New capability.

**Python (master):** Uses standard Python `logging` — no structured output, no post-mortem analysis.

**Rust (align):** Every CLI run writes JSONL to `~/.batchalign-next/logs/run-{timestamp}.jsonl`.
Events: `run_start`, `files_discovered`, `model_loading`, `model_ready`, `workers_configured`,
`file_start`, `file_done`, `file_error`, `run_end`. Per-engine model load timings and
`parse_s`/`serialize_s` in `file_done` events.

---

## Summary

| # | Category | Severity | Rust Correct? |
|---|----------|----------|---------------|
| 1 | %wor decode() bug | Major (3,735 files) | Yes |
| 2 | %wor shortened forms | Minor (cosmetic) | Yes |
| 3 | %wor inclusion rules | Medium (behavioral) | Arguably yes |
| 4 | %mor POS tags | Medium (mapping) | Different, needs review |
| 5 | %gra ROOT convention | Minor | FIXED — matches Python now |
| 6 | %gra relation labels | Minor | FIXED — matches Python now |
| 7 | Header spacing | Trivial | N/A (corpus inconsistent) |
| 8 | Trailing whitespace | Trivial | Yes |
| 9 | TextGrid interpolation | Medium (behavioral) | Different approach (skip vs interpolate) |
| 10 | Lenient parsing | N/A (new feature) | Align only — enables error recovery |
| 11 | %wor tier recovery | N/A (new feature) | Align only — drops broken tiers gracefully |
| 12 | Multi-input / server | N/A (new feature) | Align only — files, lists, --server |
| 13 | Batched callbacks | N/A (new feature) | Align only — batch morphotag/utseg |
| 14 | Processing server | N/A (new feature) | Align only — HTTP job server |
| 15 | Structured logging | N/A (new feature) | Align only — JSONL run logs |
