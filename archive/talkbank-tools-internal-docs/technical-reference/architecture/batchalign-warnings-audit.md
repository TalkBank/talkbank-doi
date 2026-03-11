# batchalign-core: Compilation Warning Audit

Date: 2026-02-12
Status: **Pending cross-check with Python consumer**

This document catalogs the current compiler warnings and one compilation error
in the batchalign-core crate and the workspace integration tests. Each item
includes analysis of whether it is safe to fix or whether it indicates
incomplete Python-side integration that needs to be preserved.

---

## 1. Compilation Error: `rust/tests/debug_wor_parsing.rs`

**Error:** `no field 'content' on type '&WorTierMarker'`
**Lines:** 46, 61

The test accesses `wor_tier.content.content` but `WorTierMarker` was refactored
(commit `f4fe4589` — "refactor: convert %wor tier to flat model") to remove the
nested `content` field. The struct now has:

- `pending_words: Option<Vec<WorWord>>` — flat word list
- `pending_terminator`, `pending_language_code`, `pending_bullet`

The test was created in the initial commit (`7b18d8dc`) and only touched once
since (`b2c503f3` — mechanical Result conversion). It was never updated for the
%wor model refactor.

**Assessment:** This is a stale debug test. The model API it tests no longer
exists. It needs to either be updated to use `pending_words` or removed.

**Question for Python consumer:** Does the Python side rely on any specific
%wor word iteration behavior that this test was validating? Specifically: does
the Python FA pipeline expect %wor content as a flat word list or as a nested
utterance content tree?

---

## 2. Warning: Unused import `WordCategory` in `add_disfluency_markers`

**File:** `rust/crates/batchalign-core/src/lib.rs:1109`
**Warning:** `unused import: talkbank_model::model::content::word::category::WordCategory`

`WordCategory` is imported inside `add_disfluency_markers()` but never used
there. However, it IS used in the helper function `apply_disfluency_to_word()`
(line 1198), which has its own identical import.

**Assessment:** This is a leftover from when the disfluency logic was extracted
into the `apply_disfluency_to_word` helper. The import moved to the helper but
the original import was not cleaned up. Safe to remove.

**Question for Python consumer:** Is `add_disfluency_markers` complete, or is
there planned work that would use `WordCategory` directly in the body of that
function (e.g., category-based filtering before delegating to the helper)?

---

## 3. Warning: Unused function `parse_callback_json`

**File:** `rust/crates/batchalign-core/src/lib.rs:628`
**Warning:** `function 'parse_callback_json' is never used`

This is a convenience wrapper around `parse_callback_json_full` that discards
the `tokens` field:

```rust
fn parse_callback_json(json_str: &str) -> Result<(String, String), String> {
    let (mor, gra, _tokens) = parse_callback_json_full(json_str)?;
    Ok((mor, gra))
}
```

It IS used by test code (lines 2563-2581: `test_parse_callback_json`), which
is why the compiler warns (tests don't count for dead-code analysis of non-test
items).

Production code only calls `parse_callback_json_full` directly.

**Assessment:** The function is a valid test helper. Adding `#[cfg(test)]` would
silence the warning while preserving the tests. However:

**Question for Python consumer:** Was `parse_callback_json` (the 2-field
version) previously the production API? If the Python side's morphosyntax
callback originally returned only `{"mor": "...", "gra": "..."}` (no `tokens`),
this function might be needed if older callback formats need to be supported.

---

## 4. Warning: Unused field `Segment::start_ms`

**File:** `rust/crates/batchalign-core/src/lib.rs:1450`
**Warning:** `field 'start_ms' is never read`

The `Segment` struct (private, inside `reassign_speakers_from_diarization`) has
three fields: `start_ms`, `end_ms`, `speaker`. The speaker assignment loop
(lines 1504-1508) only checks `seg.end_ms <= utt_end` — it never reads
`start_ms`.

The current algorithm finds the segment whose `end_ms` is closest to (but not
exceeding) the utterance's end time. A more accurate overlap-based algorithm
would use `start_ms` to compute actual overlap between segments and utterances.

**Assessment:** This looks like an intentionally parsed field that the algorithm
doesn't use yet. The diarization data from Pyannote provides both start and end
times, and a future improvement to speaker assignment would likely need
`start_ms` for proper overlap calculation.

**Question for Python consumer:** Is the current "closest end time" heuristic
the intended final algorithm, or is overlap-based speaker assignment planned?
If the latter, `start_ms` should be kept.

---

## 5. Warning: Unused field `FaWord::word_index`

**File:** `rust/crates/batchalign-core/src/forced_alignment.rs:27`
**Warning:** `field 'word_index' is never read`

`FaWord` has `utterance_index`, `word_index`, and `text`. The `word_index` field
tracks the flat position of a word within its FA group. It is set during
`group_utterances` but never read by any Rust code.

However, `FaWord` is part of `FaGroup::words`, and `FaGroup` is used in
`build_fa_payload` to construct JSON sent to the Python FA callback. Let me
check if `word_index` is included in the payload:

The `build_fa_payload` function (forced_alignment.rs) constructs a JSON payload
from the group. If `word_index` is serialized into the JSON, the Python FA
callback might use it for result mapping.

**Finding:** Checked `build_fa_payload` (forced_alignment.rs:668-683). The JSON
payload sent to Python only contains `words` (array of text strings),
`audio_start_ms`, `audio_end_ms`, and `pauses`. The `word_index` field is NOT
serialized into the payload. The FA response maps timings back to words by
array position, not by any index field.

**Assessment:** `word_index` appears to be a vestigial field from an earlier
design where explicit index tracking was needed. The current design uses
implicit array-position mapping instead. Likely safe to remove.

**Question for Python consumer:** Does any Python code construct `FaWord`
objects or use word indices from the FA pipeline? (Unlikely given it's not in
the JSON payload, but worth confirming.)

---

## Summary

| # | Item | Severity | Safe to fix? | Needs Python cross-check? |
|---|------|----------|-------------|--------------------------|
| 1 | `debug_wor_parsing.rs` compilation error | Error | Likely yes (stale test) | Yes — %wor model expectations |
| 2 | Unused `WordCategory` import | Warning | Likely yes (refactor leftover) | Minor — planned work? |
| 3 | Unused `parse_callback_json` | Warning | Likely yes (add `#[cfg(test)]`) | Minor — old callback format? |
| 4 | Unused `Segment::start_ms` | Warning | Probably not (future use) | Yes — overlap algorithm plans? |
| 5 | Unused `FaWord::word_index` | Warning | Unknown | Yes — Python FA callback usage? |
