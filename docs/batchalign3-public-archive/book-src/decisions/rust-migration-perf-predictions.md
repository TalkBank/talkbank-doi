# Rust Migration Benchmark Retrospective

**Prediction date:** February 11, 2026
**Benchmark date:** February 11, 2026
**Branch:** `align` (commit e16bc50) vs `main` (commit 614a02d)
**Machine:** Apple M4 Max, 64 GB RAM, 16 cores
**Benchmark data:** `docs/bench_correctness_diffs/`

---

## Scope of Changes

The `align` branch migrates from Python-side orchestration to Rust orchestrators
with Python callbacks. Three categories of change:

1. **Rust orchestrators with Python callbacks** — morphosyntax, forced alignment,
   utterance segmentation, translation (commits 6504e86, 73ff753)
2. **CHAT text fast path** — bypasses Document round-trips via
   `process_chat_text()` (commit d085aae)
3. **DP alignment elimination** — replaced Python Hirschberg with pretokenized
   Stanza + Rust `dp_align` (commit e16bc50)

---

## Performance Results

### Raw Timing

| Benchmark | align avg | main avg | Speedup | Notes |
|-----------|----------|---------|---------|-------|
| morphotag_small_w1 (3 files) | 5.6s | 10.9s | 1.9x | |
| morphotag_medium_w1 (8 files) | 7.8s | 13.1s | 1.7x | |
| morphotag_large_w1 (18 files) | 12.4s | 18.2s | 1.5x | |
| morphotag_large_w2 | 10.6s | 16.1s | 1.5x | |
| morphotag_large_w4 | 9.0s | 14.6s | 1.6x | |
| utseg_small_w1 (3 files) | 1.3s | 12.9s | 9.9x | |
| utseg_medium_w1 (8 files) | 1.3s | 24.6s | 18.9x | |
| align_small_w1 (3 files) | 3.6s | 90.0s | 25.0x | Not comparable (see below) |
| align_medium_w1 (8 files) | 4.9s | 154.1s | 31.4x | Not comparable |
| align_large_w1 (18 files) | 7.4s | 404.3s | 54.6x | Not comparable |
| align_medium_w2 | 4.9s | 161.4s | 32.9x | Not comparable |
| align_medium_w4 | 4.8s | 43.0s | 9.0x | Not comparable |

### Predictions vs Actuals

**Morphotag — predicted 2-5% speedup, observed 1.5-1.9x (50-90%).**

The prediction was wildly wrong. The predicted 2-5% assumed Stanza inference
dominated at ~95% of wall time. The actual breakdown (estimated from the data) is
very different:

- Both branches include ~3-4s of process startup + model loading per invocation.
- Subtracting startup, align processes 3 small files in ~2s vs main in ~7s.
- The Python Document path on main has far more overhead than just
  `tokenizer_processor()` DP — there's the entire Document construction, CHAT
  parsing/serialization, and Python object creation. The Rust fast path
  (`process_chat_text()`) eliminates ALL of this, not just the DP.
- The prediction only considered DP elimination; the CHAT text fast path
  (bypassing Document round-trips) was the bigger win.

**Utseg — predicted 1-3% speedup, observed 10-19x.**

Also wildly wrong. Key observation: utseg takes 1.3s for BOTH 3 files and 8
files on the align branch, meaning startup dominates and actual processing is
near-instant. On main, processing scales linearly (12.9s for 3 files, 24.6s for
8 files). The Rust orchestrator + fast path eliminates the Document overhead that
was the actual bottleneck, not the constituency parsing.

Caveat: main branch had no `--override-cache` for utseg, but cache keys are
content-based and these bench files likely weren't cached. Both branches should
have done full computation.

**Align — not directly comparable.**

The align branch uses `--no-utr` (forced alignment only, no utterance timing
recovery), while main runs the full pipeline (FA + UTR). Main's UTR step does
DP alignment of ASR output against the transcript, which dominates its runtime
for these short files. The numbers show the cost of UTR, not a like-for-like
comparison of forced alignment.

**WER/Benchmark and UTR — not tested.**

These were not included in this benchmark round. WER requires ASR (Whisper),
which wasn't available in the test setup. UTR is not usable on the align
branch (`process_chat_text()` not implemented for UTR engine).

---

## Correctness Analysis

### Morphotag: MWT Expansion Bug (MUST FIX)

The align branch has a **correctness regression** for English contractions.
With `tokenize_pretokenized=True`, Stanza cannot perform Multi-Word Token (MWT)
expansion. Contractions are analyzed as opaque single tokens, producing
nonsensical morphology:

| Input | align (WRONG) | main (CORRECT) |
|-------|--------------|----------------|
| `what's` | `det\|what-Def-Int` | `pron\|what-Int-S1~aux\|be-Fin-Ind-Pres-S3` |
| `won't` | `aux\|won-Fin-S` | `aux\|will-Fin-S~part\|not` |
| `don't` | `adv\|dont` | `aux\|do-Fin-Ind-Pres-S1~part\|not` |
| `I'll` | `propn\|Ill` | `pron\|I-Prs-Nom-S1~aux\|will-Fin-S` |
| `there's` | `verb\|there-Fin-Ind-Pres-S3` | `pron\|there-Int-S1~verb\|be-Fin-Ind-Pres-S3` |
| `it's` | `pron\|its-Prs-Gen-S3` | `pron\|it-Prs-Nom-S3~aux\|be-Fin-Ind-Pres-S3` |
| `she's` | `adv\|shes` | `pron\|she-Prs-Nom-S3~aux\|be-Fin-Ind-Pres-S3` |
| `doesn't` | `adv\|doesnt` | `aux\|do-Fin-Ind-Pres-S3~part\|not` |
| `isn't` | `intj\|isnt` | `aux\|be-Fin-Ind-Pres-S3~part\|not` |
| `let's` | `adv\|let verb\|see-Fin-Imp-S` | `verb\|let-Fin-Imp-S~pron\|we-Prs-Acc-P1 verb\|see-Inf-S` |
| `baby's` | `noun\|baby-Plur` | `noun\|baby~part\|s` |
| `whose` | `pron\|whose-Int-S1` | `pron\|who-Int~aux\|be-Fin-Ind-Pres-S3` |
| `that's` (copula) | `aux\|that-Fin-Ind-Pres-S3` | `pron\|that-Dem~aux\|be-Fin-Ind-Pres-S3` |

This was flagged as Risk #2 in the predictions document ("Stanza
pretokenization behavior"). The prediction said "investigate whether the new
output is more correct, not just different." The investigation is clear: **main
is correct, align is wrong.**

**Root cause:** English MWT expansion requires Stanza's tokenizer to split
contractions. With `tokenize_pretokenized=True`, the tokenizer is bypassed and
Stanza sees `won't` as an indivisible token. It then guesses a POS for the whole
string, producing nonsense like `aux|won-Fin-S`.

**Fix needed:** The pretokenized path must still allow MWT expansion. Options:
1. Run Stanza's MWT processor as a post-step after pretokenized input
2. Pre-expand contractions before sending to Stanza (Rust side)
3. Use `retokenize=True` for languages with contractions (forces Rust's
   `retokenize.rs` to handle the 1:N mapping)
4. Only use `tokenize_pretokenized=True` when the input has no contractions

Option 3 is the most pragmatic — it uses existing infrastructure and is already
tested for Italian/French MWT. The performance cost of retokenization needs
measurement.

### Morphotag: %tim Line Positioning

Minor diff: `%tim:` lines appear in slightly different positions relative to
utterances. The align branch attaches `%tim:` to the utterance above, main
attaches it to the utterance below. This is a cosmetic serialization difference
in the CHAT generator, not a semantic error. Low priority but should be
investigated for CHAT format compliance.

### Align: Different Timing Values (Expected)

All timing values differ between branches because they use fundamentally
different algorithms:

- **align branch (`--no-utr`):** Runs forced alignment (Wave2Vec/Whisper) on
  the full audio file, producing word-level timestamps from the acoustic model.
  All utterances get timing, including `xxx .` and single-word utterances.
- **main branch (with UTR):** Runs ASR on the full audio, then uses DP alignment
  to map ASR words to transcript words, transferring ASR timestamps to the
  transcript. Utterances that UTR can't match (short utterances, unintelligible
  speech) are left untimed.

Example from 03PH_18.cha:
```
align: *MOT: get your shirt fixed . 2724_3685     (FA from start of audio)
main:  *MOT: get your shirt fixed . 23465_24849   (UTR from @Time Start offset)
```

The main branch preserves timestamps relative to the original recording (matching
`@Time Start: 02:00`). The align branch computes timestamps from the beginning
of the audio file. This is an expected consequence of `--no-utr`.

### Align: Untimed Utterances on Main

The main branch (with UTR) leaves many utterances completely untimed — no bullet
on the main tier, no `%wor` tier at all:

```
main:  *MOT: close the door .         (no timing)
align: *MOT: close the door . 23689_24630  (FA always produces timing)
```

This happens when UTR's DP alignment can't confidently match the ASR output to
the transcript text. The align branch (`--no-utr`) always produces timing
because forced alignment doesn't depend on ASR-transcript matching.

---

## Model Loading Analysis

Every CLI benchmark invocation (`batchalign-next bench ...`) spawns a fresh
process that loads models from scratch. From the timing data:

**Morphotag model loading estimate:**
- align morphotag_small: 5.6s for 3 files. The correctness run was 5s for
  the same 3 files. Consistent — model loading is a fixed ~3-4s overhead.
- main morphotag_small: 10.9s for 3 files. Correctness was 11s. Same pattern.
- main's model loading appears to take ~3-4s as well (same Stanza model).

**Estimated compute time (model loading subtracted):**

| Benchmark | align compute | main compute | Compute speedup |
|-----------|-------------|-------------|----------------|
| morphotag_small (3 files) | ~2s | ~7s | 3.5x |
| morphotag_medium (8 files) | ~4s | ~9s | 2.3x |
| morphotag_large (18 files) | ~9s | ~15s | 1.7x |

The compute speedup is larger than the wall-clock speedup because model loading
is a fixed cost that dilutes the percentage improvement.

**Implication for server mode:** The processing server loads models once at
startup and shares them across all jobs via `PipelineCache`. Model loading cost
is amortized to zero. This means:

- **Server morphotag speedup ≈ compute speedup (2-3.5x)**, not wall-clock
  speedup (1.5-1.9x)
- **For bulk processing** (hundreds of files), the per-file overhead reduction
  from the CHAT text fast path compounds. 1000 files * 5ms saved = 5s. Small
  but free.
- **GIL benefit is unmeasured.** The server uses `ThreadPoolExecutor`. With the
  Rust fast path doing more work outside the GIL, concurrent throughput should
  improve. This was not tested.

**Utseg model loading anomaly:**

align utseg takes 1.3s for both 3 and 8 files, suggesting nearly all time is
startup overhead. If constituency parser loading takes ~1s and text processing
is instant, the Rust fast path has essentially eliminated Python overhead
entirely for this pipeline. Main's utseg scales linearly (12.9s → 24.6s),
confirming that Document construction/serialization was the dominant cost.

---

## Rust CHAT Parser Limitations

Discovered while creating `bench_data/align_long/` from MacWhinney CHILDES files:

1. **Hyphenated GRA relation names** (e.g., `ADVCL-RELCL`) — the Rust parser
   rejects hyphens in `%gra` dependency relation labels. Workaround: strip
   `%mor`/`%gra` tiers before processing. Fix: update Rust grammar to allow
   hyphens in GRA relation names.

2. **Exotic CHAT annotations on main tier** (e.g., `sima@shun`, `@wp`) — the
   Rust parser doesn't recognize all `@`-annotations defined in the CHAT spec.
   No workaround short of stripping annotations from the text. Fix: extend
   Rust parser's annotation grammar.

These block benchmarking with production-length CHILDES transcripts (the
`bench_data/align_long/` dataset). The files are committed for use once the
parser is extended.

---

## Caching Implications (Updated)

The prediction that "cached results remain valid" needs qualification:

- **Cache keys are unchanged** (hash of text + lang + retokenize + mwt).
- **However, results DIFFER** due to MWT expansion. If the cache was populated
  by main (with MWT expansion), then running align with `--override-cache` will
  produce different output for any utterance containing contractions.
- **Cache invalidation strategy:** Once the MWT bug is fixed, the cache should
  be cleared and rebuilt. Until then, `--override-cache` is recommended for
  correctness testing.

---

## What's Still Untested

### 1. Server mode benchmarks

The CHAT text fast path's primary benefit is for server mode (amortized model
loading, GIL contention reduction, concurrent throughput). This benchmark only
tested CLI mode where model loading dominates.

```bash
# Needed: server-vs-server comparison
# On server (align branch)
batchalign-next serve start --foreground
time batchalign-next --server http://localhost:8000 morphotag bench_data/align_medium/ /tmp/out/
# Repeat on main branch
```

### 2. Long-file forced alignment (align_long)

The biggest predicted speedup (UTR DP on long files) was not tested because:
- align branch lacks UTR support via `process_chat_text()`
- `bench_data/align_long/` files hit Rust parser limitations

Once parser limitations are fixed, this is the highest-priority benchmark.
Expected: massive speedup for UTR DP on 10+ minute files.

### 3. WER evaluation (Rust dp_align)

Not tested — requires Whisper for ASR. Expected: small absolute improvement
for short files, large improvement for long transcripts.

### 4. `retokenize=True` path

All morphotag benchmarks used `retokenize=False` (default). The `retokenize=True`
path (Rust's `retokenize.rs`) was not benchmarked. This matters because fixing
the MWT bug may require enabling retokenization for English.

### 5. Multi-language corpora

Only English was tested. Italian and French have aggressive MWT expansion (e.g.,
`dell'` → `di il`). Need to verify:
- Does the pretokenized path handle non-English MWT correctly?
- Is the `retokenize=True` path needed for all MWT languages?

### 6. Concurrent throughput (server)

The GIL contention prediction was not tested. Measure with:
```bash
# Submit 20 files simultaneously
for f in bench_data/align_large/*.cha; do
  batchalign-next --server http://localhost:8000 morphotag "$f" /tmp/out/ &
done
wait
```

Compare total wall time between branches under concurrent load.

### 7. Proportional FA estimation (`--no-utr`)

The `--no-utr` flag uses proportional FA estimation instead of UTR. Quality
of the resulting timestamps vs UTR needs systematic evaluation — not just
timing, but accuracy of word-level alignment compared to manual annotation.

---

## Action Items (Priority Order)

1. **Fix MWT expansion for pretokenized path** — correctness regression,
   blocks merge to main. Likely solution: enable `retokenize=True` for
   languages with contractions/MWT, or pre-expand contractions in Rust.

2. **Extend Rust CHAT parser** — support hyphenated GRA relations and
   `@`-annotations (`@shun`, `@wp`, etc.) to unblock `align_long` benchmarks.

3. **Run server-mode benchmarks** — the predicted GIL/concurrency wins are
   the key selling point for the server deployment.

4. **Benchmark `retokenize=True` performance** — if MWT fix requires
   retokenization, measure the performance cost.

5. **Test multi-language corpora** — verify Italian, French, Spanish
   morphotag correctness with the pretokenized path.

6. **Benchmark long-file alignment** — once parser limitations are fixed,
   run `bench_data/align_long/` to validate the UTR DP speedup prediction
   (expected: minutes → seconds for 10+ minute recordings).

---

## Summary

The Rust migration delivers significant performance improvements across all
tested pipelines, far exceeding predictions for morphotag (1.5-1.9x vs
predicted 2-5%) and utseg (10-19x vs predicted 1-3%). The primary driver is
the CHAT text fast path eliminating Python Document overhead, not just DP
elimination.

However, the migration introduces a **correctness regression** in morphotag
for English contractions due to pretokenized Stanza bypassing MWT expansion.
This must be fixed before merging to main.

The highest-value untested scenarios are server-mode benchmarks (where model
loading is amortized) and long-file alignment (where UTR DP speedup should be
dramatic).
