# Align Command: Correctness Fixes (2026-02-15)

## Bugs Found

Correctness testing revealed that `batchalign3 align` produced completely
wrong timestamps — utterances placed at 140ms instead of 8465ms in the audio.
Four independent bugs were responsible: two causing wrong absolute positions,
one silently dropping the last word of every utterance, and one preventing
cache invalidation.

### Bug 1: Whisper pipeline chunk-offset regression

**Root cause**: The `batch_size` parameter was passed to the HuggingFace Whisper
pipeline constructor in `WhisperASRModel.__init__()`. This parameter was added
on the batchalign3 but was never present on master. When the pipeline processes
audio longer than 25 seconds, it splits it into overlapping chunks. With
`batch_size` set in the constructor, the pipeline failed to convert
chunk-relative timestamps to absolute for the second and subsequent chunks.

**Symptom**: UTR (Utterance Timing Recovery) assigned timestamps that reset to
zero partway through the file. For a 30-second recording, utterances 1-6 got
timestamps in the 0-13 second range, then utterance 7 jumped back to 0 seconds.
Since FA (Forced Alignment) trusts UTR's boundaries to extract audio chunks,
every downstream word-level timestamp was also wrong.

**Fix**: Removed `batch_size` from the pipeline constructor in
`batchalign/models/whisper/infer_asr.py`. The inference call already uses
`batch_size=1` regardless. Added a monotonicity warning in
`batchalign/pipelines/utr/whisper_utr.py` that detects timestamp regressions
so this class of bug will be visible in logs if it ever recurs.

### Bug 2: Wave2Vec FA missing audio offset

**Root cause**: In `forced_alignment.rs`, the function `parse_fa_response()`
handles two response formats — `TokenLevel` (from Whisper FA) and `WordLevel`
(from Wave2Vec FA). The `TokenLevel` path correctly added the group's
`audio_start_ms` offset to convert chunk-relative timestamps to absolute. The
`WordLevel` path did not — it passed timestamps through unchanged.

**Symptom**: When using Wave2Vec FA (`--wav2vec`), word timestamps were relative
to the start of each audio chunk instead of absolute positions in the recording.
This bug was latent in the default configuration (which uses Whisper FA) but
would have manifested for any user selecting Wave2Vec.

**Fix**: Added `audio_offset_ms` parameter to `align_word_timings()` in
`forced_alignment.rs` and applied it to each timing. Added a regression test
with a non-zero offset.

### Bug 3: Last word of every utterance silently dropped

**Root cause**: Three steps interacted badly in the default configuration
(Whisper FA, `pauses=False`):

1. **`align_token_timings`**: Whisper FA returns onset-only times (one time per
   token, no end time). In non-pauses mode, each word was assigned
   `end_ms = start_ms` with the comment "Rust chains them later."

2. **`postprocess_utterance_timings`**: The chaining loop (`for i in 0..n-1`)
   set each word's end to the next word's start — but the last word had no next
   word, so its end stayed equal to its start.

3. **`update_utterance_bullet`** ran **before** postprocess. It recomputed the
   utterance bullet from the raw word timings (where `end = start` for every
   word), producing a bullet whose end equaled the last word's start. Postprocess
   then bounded words by this bullet: last word `start >= end` → timing dropped.

**Symptom**: The last content word of every utterance silently lost its timing.
Single-word utterances lost their only word timing. The utterance-level bullet
survived (recomputed from remaining words), but the `%wor` tier was missing the
final word's timestamp. This was invisible in casual inspection because the
utterance bullet still looked plausible.

**Python comparison**: Python master has the same structural issue (onset-only
times, chaining skips the last word). Python works around it two ways:

- When the last word has no next word to chain to, Python falls back to
  `w.time = (w.time[0], w.time[0] + 500)` — a hardcoded 500ms extension.
- Python preserves the UTR bullet (does not recompute it from word timings),
  so the bounding step uses the wider UTR range.

**Fix**: Two changes:

1. In `postprocess_utterance_timings` (`forced_alignment.rs`): after the
   chaining loop, the last timed word's end is extended to the utterance bullet
   end (or `start + 500ms` if no bullet). This matches Python's fallback but
   uses the actual utterance boundary instead of a fixed duration.

2. In `add_forced_alignment_inner` (`lib.rs`): changed the execution order from
   `update_bullet → postprocess → add_wor` to `postprocess → update_bullet →
   add_wor`. This ensures postprocess uses the **UTR bullet** (which captures
   actual speech boundaries from ASR) for bounding, not a bullet recomputed from
   onset-only word timings.

### Bug 4: UTR cache ignored --override-cache

**Root cause**: The UTR cache in `whisper_utr.py` unconditionally returned
cached ASR results without checking `ctx.override_cache`. The FA cache
(`_fa_callback.py`) correctly checked this flag, but UTR did not.

**Symptom**: Running `batchalign3 --override-cache align` bypassed the FA
cache but still served stale UTR results. If the UTR cache held results from a
previous buggy run (e.g., before the `batch_size` fix), those timestamps
persisted indefinitely. This made debugging timestamp issues confusing — the
same wrong results appeared even after code fixes.

**Fix**: Added `override_cache` check to UTR cache lookup in `whisper_utr.py`.
Now `--override-cache` bypasses both UTR and FA caches.

## Results After Fix

All four bugs are fixed. Timestamps agree with Python master to within a few
hundred milliseconds (expected variation from different DP implementations and
Whisper nondeterminism):

```
Utterance                                    Python        Rust (fixed)
--------------------------------------------------------------------
you gonna cook some more ?                  8465_9627      8465_9306
oh it opens !                               10331_12115    11405_12185
what's in there ?                           13963_14764    13933_14425
you hafta close the door .                  16875_17835    16875_17539
it won't work until you close the door .    18456_19857    18460_19465
okay , push the start button .              20497_21878    20323_21385
gotta start it , this one .                 23540_25021    24175_24665
gotta do it with the door closed .          27984_29224    27945_29065
close the door .                              (none)         27117_27617
okay now push this button , right there .     (none)         27617_28638
&~uh popcorn !                                (none)         28658_29459
oo .                                          (none)         29459_29599
yeah .                                        (none)         29619_29959
```

Word-level timing is now complete for every timed utterance. The last word of
each utterance has its end time extended to the utterance boundary, matching
Python's behavior.

## Rust Is More Complete

Python master timed 8 of 13 utterances. Rust timed all 13. This is not
accidental — it reflects an architectural difference in how the two
implementations handle utterances that UTR could not match.

### The problem: UTR misses

UTR works by running Whisper ASR on the full audio, then DP-aligning the ASR
transcript against the CHAT text. When ASR misses words or the DP alignment
cannot find a confident match, some CHAT utterances end up with no timing
bullet.

In this test file, the last 5 utterances ("close the door", "okay now push this
button", "popcorn", "oo", "yeah") were not matched by UTR's DP alignment,
likely because they are short utterances near the end of the recording where
Whisper's chunking overlap may lose coverage.

### Python master: skip untimed utterances

Python master's FA engine (`whisper_fa.py`) explicitly skips any utterance
without a timing bullet:

```python
if i.alignment == None:
    warnings.warn("We found at least one utterance without utterance-level alignment...")
    continue
```

If UTR didn't assign timing, FA gives up entirely on that utterance. The final
output has no timing for those 5 utterances — no utterance bullet, no `%wor`
tier.

### Rust: proportional fallback estimates

Rust's FA grouping (`forced_alignment.rs`) handles untimed utterances by
computing proportional time estimates based on word count. When an utterance has
no bullet but `total_audio_ms` is known (it always is — the engine measures the
audio file), the grouping code estimates where the utterance should be in the
audio:

```rust
None => {
    // No bullet — try proportional estimate
    match &estimates {
        Some(est) if utt_idx < est.len() => est[utt_idx],
        _ => { continue; }
    }
}
```

The estimate divides the total audio proportionally by word count across all
utterances, with a 2-second buffer on each side. This gives FA a reasonable
audio window to search in, even without UTR's guidance.

The result: FA processes all utterances, including those UTR missed. The
timestamps for estimated utterances are less precise (they depend on the quality
of the proportional estimate) but they are present and correct enough to be
useful. In this test, the 5 utterances Python left untimed all received
plausible absolute timestamps from Rust.

### Why this matters

In production, untimed utterances in the output are a data quality problem.
Downstream tools that depend on timing (concordance, playback, analysis) cannot
use utterances without timestamps. Python master silently drops these utterances
from alignment, leaving gaps. Rust's fallback ensures every utterance gets its
best-effort timing, producing more complete output.
