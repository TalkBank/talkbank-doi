# Utterance-Bullet Timestamp Monotonicity in the `align` Pipeline

## Background

The CHAT format requires that utterance-level timing bullets increase monotonically
through the file. The `chatter validate` tool enforces this as **E362**:

```
E362: Media bullet timestamp 45424ms comes before previous timestamp 70565ms
      (timestamps must increase monotonically)
```

The `batchalign3 align` pipeline (UTR + FA) was discovered to produce E362
violations silently — writing invalid CHAT to disk without warning the user. This
document explains the root cause, the fix, and the invariant that is now enforced.

---

## Why CHAT Requires Monotonic Bullets

Every main-tier utterance that has been force-aligned carries an utterance-level
timing bullet — e.g. `*PAR: hello world . 2000_4500` — indicating the start and
end of that utterance in the audio file. CLAN requires that consecutive bullets
are non-decreasing: each utterance must begin no earlier than the previous one
ended. This is because CLAN tools use bullets to seek into the audio file for
playback; a regression means the tool would have to seek backwards, which is
undefined behaviour in the CLAN player.

---

## The Align Pipeline: UTR then FA

`batchalign3 align` runs two sequential processing engines (task numbers are
execution order):

| Task | Engine | What it does |
|------|--------|--------------|
| 8 — `UTTERANCE_TIMING_RECOVERY` | `WhisperUTREngine` | Assigns coarse utterance-level bullets by aligning the full Whisper ASR transcript to the CHAT utterances |
| 9 — `FORCED_ALIGNMENT` | `Wave2VecFAEngine` / `WhisperFAEngine` | Groups utterances by their UTR bullets, runs the FA model on each audio window, injects word-level inline bullets, then recomputes the utterance bullet from the word timings |

### Phase 1 — UTR: global DP alignment

`whisper_utr.py:process_handle` calls `handle.add_utterance_timing(asr_words_json)`,
which runs `add_utterance_timing_inner` in `batchalign-core/src/lib.rs`.

Inside that function the CHAT words are flattened in **text order** to form the
*reference* sequence, and the Whisper ASR words are in **temporal order** to form
the *payload* sequence. A global Hirschberg DP aligner (`dp_align::align`) finds
the longest common subsequence between the two and records which ASR word each
CHAT word matched:

```rust
// lib.rs — add_utterance_timing_inner
let alignment = dp_align::align(&asr_keys, &ref_keys, dp_align::MatchMode::CaseInsensitive);
```

**Critical property of this aligner:** it preserves the relative order of both
sequences simultaneously. If CHAT word at reference position *i* is matched to
ASR word at payload position *p*, then CHAT word at position *j > i* can only
match to payload position *q > p*. This is the standard LCS ordering constraint.

Consequence: the timestamps the DP assigns to CHAT words are themselves
monotonically increasing when read in text order — *as long as text order matches
temporal audio order.*

### Phase 2 — FA: per-group forced alignment with proportional estimation

`Wave2VecFAEngine._prepare_fa` and `WhisperFAEngine._prepare_fa` both call
`audio_duration_ms(f)` and pass the result as `total_audio_ms` to
`add_forced_alignment`. Inside `forced_alignment.rs`:

```rust
// forced_alignment.rs — group_utterances
pub fn group_utterances(
    chat_file: &ChatFile,
    max_group_ms: u64,
    total_audio_ms: Option<u64>,  // always Some(...) for both FA engines
) -> Vec<FaGroup> {
    let estimates = total_audio_ms.map(|total_ms|
        estimate_untimed_boundaries(chat_file, total_ms));
    ...
    let (utt_start, utt_end) = match &utt.main.content.bullet {
        Some(b) => (b.timing.start_ms, b.timing.end_ms),
        None => {
            // No bullet — use proportional estimate
            match &estimates {
                Some(est) if utt_idx < est.len() => est[utt_idx],
                _ => { utt_idx += 1; continue; }
            }
        }
    };
```

`estimate_untimed_boundaries` distributes the total audio duration proportionally
across utterances by word count, with a ±2 second buffer:

```rust
// forced_alignment.rs — estimate_untimed_boundaries
const BUFFER_MS: u64 = 2000;
let raw_start = (words_before as f64 / total_words as f64 * total_audio_ms as f64) as u64;
let raw_end   = ((words_before + count) as f64 / total_words as f64 * total_audio_ms as f64) as u64;
let start = raw_start.saturating_sub(BUFFER_MS);
let end   = (raw_end + BUFFER_MS).min(total_audio_ms);
```

An utterance without a UTR bullet gets a proportional window. The FA callback
(Wave2Vec or Whisper) then aligns the utterance's words within that window.

---

## The Bug: How Correct Timestamps Become Non-Monotonic

Consider a real multi-speaker conversation where the CHAT file has been transcribed
in *conversational/logical order* rather than strict *temporal order*. This is
common in CLAN transcription practice — especially for backchannels and overlapping
speaker turns — where the transcriber groups related utterances together for
readability rather than strict chronological sequencing.

### Concrete example from `minga040.cha`

In the audio recording:

| Time | Speaker | Content |
|------|---------|---------|
| 45 – 50 s | PAR | "I'm the recruiter for the study" |
| 50 – 51 s | INV | "okay" |
| 60 – 61 s | INV | "mhm" |
| 70 – 73 s | PAR | "well, I'm not really involved in the science" |

In the CHAT transcript (text order):

```
*PAR:  well , &-um &-uh I think what I'll say is <I'm not> [//] ...
       I'm not really involved in the science .          ← text line 37
*PAR:  I'm the recruiter &*INV:oh_okay for the study .  ← text line 39
*INV:  okay .                                           ← text line 41
```

The transcriber wrote the long PAR elaboration first and the short PAR turn second,
even though in the audio the order is reversed.

### Step-by-step through the pipeline

**UTR — DP alignment:**

The flat reference sequence starts with "well I think… science" (utterance at text
line 37) and then "I'm the recruiter… study" (text line 39). The ASR payload has
"I'm the recruiter" at payload position ~P₁ (corresponding to ~45 s) and "well I
think… science" at payload position ~P₂ > P₁ (corresponding to ~70 s).

The DP aligner must match the reference in order. It matches "well I think…
science" to the payload around P₂ = 70 s. Now "I'm the recruiter" (which comes
*after* in the reference) must match to some payload position *after* P₂. The only
occurrence at or after P₂ is not a good match — so the utterance at text line 39
gets **no UTR timing** (all its word matches fail or score poorly against the
ordering constraint).

Result after UTR:

```
text line 37: PAR "well…science"     → bullet 70565_73104   (correct)
text line 39: PAR "I'm the recruiter" → no bullet            (UTR skipped it)
```

**FA — proportional estimation:**

`group_utterances` is called with `total_audio_ms` set to the full length of
`minga040.mp4` (around 540 s). For text line 39 (no UTR bullet),
`estimate_untimed_boundaries` computes a proportional window. Text line 39 is
roughly 8 % of the way through the file by word count — so the estimate lands
around 43–47 s with the ±2 s buffer. The true audio content ("I'm the recruiter")
is at 45–50 s, squarely inside that window.

The FA callback (Wave2Vec) aligns "I'm the recruiter for the study" within the
~43–50 s audio window and correctly returns word-level timings:

```
"I'm"       45424_46526
"the"       46526_46786
"recruiter" 46786_48649
...
```

`update_utterance_bullet` computes the utterance bullet as min(start)–max(end):
**45424_50571**.

Result after FA:

```
text line 37: PAR "well…science"      → 70565_73104   (from UTR, correct)
text line 39: PAR "I'm the recruiter" → 45424_50571   (from FA, also correct)
```

Both timestamps are correct relative to the audio. But read in text order:

```
70565 → 45424   ← regression: E362
```

The file is invalid because the *correct* timestamp of each utterance happens to
decrease when read in the *text* order that CHAT mandates.

---

## Why the Validation Gate Was the Immediate Fix

Before this was analysed, the `pipeline.py` pre-serialization validation chain
(`validate_structured` + round-trip `ParsedChat.parse`) only checked:

1. MOR/GRA/WOR tier alignment (word count mismatches)
2. Strict parse success (E362 is syntactically valid — the parser accepts it)

Neither gate ran the full `ChatFile::validate()`, which is what `chatter validate`
runs and which includes `check_bullet_monotonicity`. As a result, invalid CHAT was
written to disk silently.

The immediate fix added a **Step 1.5** to `pipeline.py`:

```python
# pipeline.py — process_handle_and_serialize
full_json: str = handle.validate_chat_structured()
full_errors_all: list[dict[str, object]] = _json.loads(full_json)
full_errors = [e for e in full_errors_all if e.get("severity") == "error"]
if full_errors:
    raise CHATValidationException(...)
```

`validate_chat_structured()` is a new PyO3 method on `ParsedChat` that calls
`inner.validate(errors, None)` — the exact same validation logic as `chatter
validate`, minus the E531 media-filename check which requires a file path not
available at this point. This gate now fires before any bytes are written to disk.

---

## The Algorithmic Fix: Post-FA Monotonicity Enforcement

The validation gate is a safety net, not a solution. After the gate was in place
the `align` command would fail with a `CHATValidationException` rather than writing
invalid CHAT — but it still failed. The root fix is in `add_forced_alignment_inner`
(`batchalign-core/src/lib.rs`), as a third pass after all utterances have been
post-processed and had their bullets updated.

### What the pass does

After the second loop that calls `update_utterance_bullet` for every grouped
utterance, a final scoped block walks all utterances in text order, tracking the
start timestamp of the most recently accepted utterance:

```rust
// lib.rs — add_forced_alignment_inner (final pass)
{
    use talkbank_model::model::DependentTier;
    let mut last_start_ms: u64 = 0;
    for line in chat_file.lines.iter_mut() {
        let utt = match line {
            Line::Utterance(u) => u,
            _ => continue,
        };
        match utt.main.content.bullet.as_ref().map(|b| b.timing.start_ms) {
            Some(s) if s < last_start_ms => {
                // Non-monotonic: strip all FA-assigned timing from this utterance.
                utt.main.content.bullet = None;
                strip_timing_from_content(&mut utt.main.content.content.0);
                utt.dependent_tiers
                    .retain(|t| !matches!(t, DependentTier::Wor(_)));
            }
            Some(s) => last_start_ms = s,
            None => {}
        }
    }
}
```

An utterance that fails the check has three things removed from the AST:

| What | How |
|------|-----|
| Utterance-level bullet (`*PAR: … . 45424_50571`) | `utt.main.content.bullet = None` |
| Word-level inline bullets in the main tier | `strip_timing_from_content(...)` |
| The entire `%wor` dependent tier | `retain(|t| !matches!(t, DependentTier::Wor(_)))` |

The utterance is left exactly as it would look before alignment ran: plain text,
no bullets, no `%wor`. The in-order utterances around it retain their full
word-level alignment.

### Why strip rather than fix

The non-monotonic timestamp is *correct for the audio* — the utterance really does
occur at 45 s. The problem is that the CHAT file has the utterance in the wrong
textual position relative to its neighbours. Batchalign cannot unilaterally reorder
CHAT utterances; that would change the transcript structure in ways the transcriber
did not intend and that downstream CLAN tools (such as MOR/POST) depend on.

Stripping the timing is the conservative choice: the transcript is valid and the
aligned utterances are correctly timed. The unaligned utterances simply did not
receive timing, which is the same outcome as if UTR had never found them. No
information is corrupted; only alignment coverage is reduced.

### What the user sees

After the fix, running `align` on `minga040.cha` produces valid CHAT. Utterances
whose timing would create a regression are left untimed. The user can identify
which utterances were affected by the absence of bullets or `%wor` tiers.

If the user needs full alignment coverage, the transcript order must be corrected
to match temporal audio order before running `align`. This is a data quality issue
in the original CHAT file.

---

## Invariant Summary

After both fixes (validation gate + algorithmic fix) the following invariants hold:

1. **No invalid CHAT is ever written.** `validate_chat_structured()` runs before
   serialization. Any E362 (or other severity-error) causes a
   `CHATValidationException`; no output file is created.

2. **The FA pipeline never produces non-monotonic bullets.** The final pass in
   `add_forced_alignment_inner` strips timing from any utterance that would
   introduce a regression. The post-condition of `add_forced_alignment_inner` is
   that all utterance bullets are non-decreasing in text order.

3. **Invariant 1 is redundant given invariant 2** for the `align` command, but
   invariant 1 is still necessary as a general guard against future regressions
   and for other commands that might introduce bullets.

---

## Files Changed

| File | Change |
|------|--------|
| `batchalign-core/src/lib.rs` | Added `validate_chat_structured()` PyO3 method; added final monotonicity-enforcement pass in `add_forced_alignment_inner` |
| `batchalign/pipelines/pipeline.py` | Added Step 1.5: calls `validate_chat_structured()` and blocks on any severity="error" result |
| `batchalign/tests/pipelines/test_roundtrip_validation.py` | Added `test_non_monotonic_timestamps_rejected` covering the E362 validation gate |
