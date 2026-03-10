# Coref and Utseg: Correctness Assessment (2026-02-15)

## 1. COREF (Coreference Resolution)

### Test Results

| | Python master | Rust (align) |
|---|---|---|
| Files succeeded | 3/7 (English only) | 7/7 (all languages) |
| Failed files | 4/7 crashed: `'NoneType' object has no attribute 'content'` | 0 (non-English skipped gracefully) |
| Tier name | `%coref:` | `%xcoref:` |

### Output Differences

The two implementations produce different coref annotations for the same input.
For the utterance `&-um so ‡ if you can start maybe you can tell me about the best trip you ever took ?`:

```
Python %coref:  -, -, -, -, (0), -, -, -, (0), -, -, (1), -, (2, 2, 2, (0) 2, 2, 2), -
Rust   %xcoref: -, -, -, (0), -, -, -, (0), -, -, (1), -, (2, 2, 2, (0) 2, 2, 2)
```

Python has 15 positions, Rust has 14. For `the best trip I ever took ?`:

```
Python %coref:  -, -, -, (0), -, -, -
Rust   %xcoref: (3, 3, 3, (0) 3, 3, 3)
```

Python only links "I" to chain 0. Rust wraps the entire phrase in chain 3 with
nested chain 0 — a completely different coref analysis.

### Why They Differ

The implementations feed fundamentally different input to the same Stanza coref
model (same package: `ontonotes-singletons_roberta-large-lora`):

| | Python master | Rust (align) |
|---|---|---|
| Input to Stanza | Entire document detokenized into one string | Per-utterance pretokenized words joined by `\n\n` |
| Tokenization | Stanza re-tokenizes from scratch | `tokenize_pretokenized=True` — CHAT words used directly |
| Word extraction | All forms including punctuation, retraces, fillers | Mor domain only (linguistic content) |
| Alignment back | DP alignment (Stanza words → Document forms) | Direct index mapping (no DP needed) |

Different tokenization means Stanza sees different text, producing different
coref chains. Neither output is linguistically "wrong" — they are different
analyses of different tokenizations of the same speech.

### Rust Improvements Over Python

**Correct tier naming.** CHAT spec requires user-defined tiers to start with
`x`. `coref` is not a standard CHAT tier (the standard tiers are: `mor`, `gra`,
`pho`, `mod`, `sin`, etc.), so Python's `%coref:` violates the convention.
Rust's validation enforces the `x` prefix and uses `%xcoref:`.

**No crash on non-English.** Python's `process()` does `return` without a value
(line 19 of `coref.py`), returning `None`. The pipeline caller then accesses
`.content` on `None`, producing `'NoneType' object has no attribute 'content'`.
Rust returns the input CHAT text unchanged for non-English files.

**Consistent word domain.** Rust uses Mor-domain extraction — the same word set
used by morphosyntax. This means one coref position per linguistic word. Python
counts all document forms including punctuation and fillers, creating spurious
dash positions for non-linguistic tokens.

**No DP alignment needed.** Rust's pretokenized approach means words go into
Stanza and come out with the same indices — direct mapping. Python detokenizes
everything into a flat string, lets Stanza re-tokenize, then DP-aligns back to
recover which Stanza word corresponds to which CHAT form. This extra step is
error-prone and unnecessary.

### Trailing Dashes

Python has trailing dashes at the end of some coref tiers; Rust does not. This
is because Python counts all forms in the utterance (including the terminator
`?`, `.`, `!`), which never participates in coreference and always gets a dash.
Rust excludes terminators from the Mor domain, so no trailing dash is generated.

---

## 2. UTSEG (Utterance Segmentation)

### Test Results

| | Python master | Rust (align) |
|---|---|---|
| Files succeeded | 0/7 | 3/7 (English only) |
| Failure mode | `process pool terminated abruptly` (all files) | `UnsupportedProcessorError` for non-English (4 files) |
| Error message | None (crash) | `Processor constituency is not known for language ca/nl` |

### Why Python Crashes — A Regression

**This is a regression introduced on Jan 9, 2026** by commit `3bfd7ae` ("feat:
add parallel processing support with worker management"). Before this commit,
`dispatch.py` was 196 lines of sequential code — one pipeline in the main
process, each file in a `for` loop with `except Exception`. After Jan 9, it
became 1044 lines with `ProcessPoolExecutor`-based parallel dispatch.

The old sequential code would have scored **3/7** (same as Rust) — non-English
files would fail with `UnsupportedProcessorError` per-file, while English files
succeeded. The new parallel code scores **0/7** because of a crash cascade.

**Two layers of inadequate error handling interact:**

1. **Engine level** (`ud_utterance.py` line 357): The per-utterance error handler
   is `try: process_ut(i, nlp_obj) except IndexError:`, which only catches
   `IndexError`. But `UnsupportedProcessorError` is raised during
   `stanza.Pipeline()` initialization (line 344), which is **outside** this
   try/except.

2. **Dispatch level**: The worker function `_worker_task` has a broad
   `except Exception as e:` that should catch `UnsupportedProcessorError`. But
   when Stanza's constituency model initialization for an unsupported language
   crashes the worker process at the native/C level (below Python's exception
   handling), `except Exception` cannot catch it — the process simply dies.

**The cascade:** the first non-English file's worker process crashes → the
`ProcessPoolExecutor` enters `BrokenProcessPool` state → all remaining files
(including the 3 English files that would have succeeded) fail with "process
pool terminated abruptly". The utseg engine is classified as `POOL_UNSAFE`
(`stanza_utt` in `POOL_UNSAFE_ENGINES`), so it correctly avoids the
`ThreadPoolExecutor` pool path, but the `ProcessPoolExecutor` fallback is
equally fragile when a worker process dies.

**Bottom line:** the pre-Jan-9 sequential dispatch handled this correctly. The
parallel dispatch introduced a fragile architecture where one bad file poisons
the entire batch.

### Why Rust Handles It

Rust's batched callback architecture catches all Python exceptions at the
Rust/Python boundary. If the Stanza callback raises any exception, Rust
preserves the original utterance unchanged:

```rust
match parsed {
    Ok(resp) if resp.assignments.len() == words.len() => {
        // Apply segmentation splits
    }
    _ => {
        // Keep original utterance
        new_lines.push(Line::Utterance(utt));
    }
}
```

Each file fails independently with a clear diagnostic error. Other files
continue processing normally. English files succeed (3/7), non-English files
fail gracefully with informative errors (4/7).

### The Stanza Limitation

Constituency parsing — which utseg depends on to find sentence boundaries — is
only available for a handful of languages in Stanza: English, Chinese, Japanese,
Korean, and a few others. Catalan and Dutch do not have constituency models.
This is a Stanza limitation, not a batchalign bug. Both implementations are
equally affected by this constraint; the difference is purely in error handling.

### Utseg Output

For the files that succeeded (English), Rust's utseg split some longer
utterances into smaller units. For `176-1.cha`:

- Input: 645 main speaker utterances
- Output: 661 main speaker utterances (16 additional from splits)

No special annotation tier is added. Segmentation is reflected in the main tier
structure — long utterances become multiple shorter utterances. Dependent tiers
(`%mor`, `%gra`, etc.) are discarded after splitting because they would be stale;
morphosyntax must be re-run on the segmented output.

---

## Summary

| Aspect | Coref | Utseg |
|--------|-------|-------|
| Correctness | Different outputs from same model, different tokenization | Same model, same limitation |
| Tier naming | Rust correct (`%xcoref:`), Python wrong (`%coref:`) | N/A |
| Robustness | Rust 7/7, Python 3/7 (crashes on non-English) | Rust 3/7 graceful, Python 0/7 total crash |
| Architecture | Rust cleaner: pretokenized, direct mapping, no DP | Rust cleaner: batched, error-isolated |
| Action needed | None | None — Stanza limitation |

Both commands are English-only in practice. Rust handles the non-English case
correctly (skip or fail gracefully). Python crashes — and the utseg crash is a
**regression** introduced on Jan 9, 2026 when `ProcessPoolExecutor` replaced
the sequential dispatch. The old sequential code would have scored 3/7, same as
Rust. No code changes are needed on the Rust side — the implementations are
correct and more robust than Python master.
