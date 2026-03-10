# Postmortem: Single-Word Retrace Regression (2026-02-19)

## Summary

The Rust retrace engine (`add_retrace_markers`) produced incorrect CHAT
output for repeated single words when the repetition count exceeded 2.
Input like `the the the the dog` was serialized as
`<the the> [/] the [/] the dog` instead of the correct
`the [/] the [/] the [/] the dog`.  The bug was caused by the Rust
algorithm's largest-first n-gram scan direction, which differs from
Python master's smallest-first scan.

**Severity**: Low-to-moderate.  Affected only utterances with 3+
consecutive single-word stutters in `transcribe` / `transcribe_s`
output.  Morphotag, align, and other commands were unaffected.

**Detection**: Reported by Davida via user feedback on transcribe output.

**Resolution**: One-line guard clause in `add_retrace_markers_inner`
(Rust) plus three new regression tests (Python).  Fixed same day.

---

## Timeline

| When | What |
|------|------|
| 2026-02-18 | Align branch deployed to production with retrace engine |
| 2026-02-19 AM | Davida reports `<the the> [/] the cat` in transcribe output |
| 2026-02-19 PM | Root cause identified, TDD fix applied, all tests green |

---

## Root Cause: Scan Direction

The retrace algorithm detects consecutive repeated n-grams within an
utterance and marks the earlier occurrences with `[/]` (single word) or
`<...> [/]` (multi-word phrase).  The two implementations differ in one
critical design choice: **which n-gram sizes are tried first**.

### Python master: smallest-first (n=1 upward)

```python
for n in range(1, len(content)):      # n = 1, 2, 3, ...
    begin = 0
    while begin < len(content) - n:
        gram = tuple(i.text for i in content[begin:begin+n])
        while tuple(i.text for i in content[root+n:root+2*n]) == gram:
            # mark content[begin:begin+n] as RETRACE
            ...
```

For `the the the the dog`:

1. **n=1**: Finds `the == the` at positions 0/1, 1/2, 2/3.  Marks
   positions 0, 1, 2 as `RETRACE`.  Position 3 stays `REGULAR` (the
   "kept" copy).
2. **n=2**: `["the","the"]` at positions 0-1 matches 2-3, but those
   tokens are already `RETRACE` — the marking is idempotent.
3. **Serializer**: Sees a chain of same-text `RETRACE` tokens and emits
   individual `word [/]` markers (no angle brackets for single-word
   chains).

**Result**: `the [/] the [/] the [/] the dog` (correct).

The smallest-first strategy works because single-word repetitions are
claimed early.  When the n=2 pass later encounters `["the","the"]`
matching `["the","the"]`, the tokens are already marked and no new
structural decision is made — the serializer's state machine resolves
the formatting from the token types and text equality.

### Rust (align branch, before fix): largest-first (n=max downward)

```rust
for n in (min_n..word_map.len()).rev() {   // n = max, max-1, ..., 1
    while begin + 2 * n <= word_map.len() {
        if gram == next && !overlap {
            retrace_ranges.push((begin, n));   // records (start, span)
            retraced[begin..begin+n].fill(true);
        }
    }
}
```

For `the the the the dog`:

1. **n=3**: `["the","the","the"]` at 0-2 vs `["the","dog","???"]` —
   no match (not enough words).
2. **n=2**: `["the","the"]` at 0-1 matches `["the","the"]` at 2-3.
   Records `(0, 2)` — a **2-word group**.  Marks positions 0-1 as
   retraced.
3. **n=1**: `"the"` at position 2 matches `"the"` at position 3.
   Records `(2, 1)` — a single-word retrace.

Phase 6 then builds AST nodes:

- `(0, 2)` → `AnnotatedGroup` with angle brackets: `<the the> [/]`
- `(2, 1)` → `AnnotatedWord`: `the [/]`
- Position 3 (the "kept" copy): `the`

**Result**: `<the the> [/] the [/] the dog` (wrong).

The largest-first strategy is correct for genuine multi-word phrase
retraces (`I want I want a cookie` → `<I want> [/] I want a cookie`).
It fails only when all words in the n-gram happen to be identical — the
bigram `["the","the"]` "looks like" a phrase retrace but is really just
a single-word stutter.

---

## Why We Used Largest-First

The Rust algorithm deliberately scans from largest to smallest so that
multi-word phrase retraces take priority over single-word matches.
Consider:

```
I want I want a cookie
```

- **Smallest-first** (Python): n=1 finds `I == I` at positions 0/2 and
  marks position 0 as RETRACE.  n=2 then finds `["I","want"]` matching
  `["I","want"]`, but position 0 is already RETRACE.  The serializer
  sees a mix of retrace types and must use a complex state machine to
  decide between `I [/] <want> [/] want` and `<I want> [/] I want`.
  Python's state machine happens to get this right, but the logic is
  fragile and spread across detection + serialization.

- **Largest-first** (Rust): n=2 finds the bigram match first.  The
  structural decision (AnnotatedGroup vs AnnotatedWord) is made at
  detection time, and serialization is mechanical.  Cleaner separation
  of concerns.

The largest-first approach is architecturally sound.  The bug was a
missing guard for the degenerate case where a "multi-word" n-gram
contains only one distinct word.

---

## Why Python Master Didn't Have This Bug

Python avoids the problem through two reinforcing mechanisms:

1. **Smallest-first scan**: n=1 claims single-word repeats before
   larger passes can misinterpret them as phrase retraces.

2. **Serializer state machine**: Even if n=2 marks a `["the","the"]`
   bigram, the `_detokenize()` method checks whether adjacent RETRACE
   tokens have the same text.  Same-text chains always emit individual
   `word [/]` markers, never angle brackets.  This acts as a safety net.

The Rust implementation lacks the second mechanism because it makes the
structural decision (AnnotatedWord vs AnnotatedGroup) at detection time
and records it in the AST.  The serializer faithfully renders whatever
the AST contains — there's no second-chance text-equality check.

This is the right architecture (decisions at detection, mechanical
serialization), but it means the detection algorithm must handle all
edge cases itself.

---

## The Fix

Added a guard in the Rust n-gram matching loop (`lib.rs`, line 2172):

```rust
if gram == next {
    let all_same = n > 1 && gram.iter().all(|w| *w == gram[0]);
    let overlap = retraced[begin..begin + n].iter().any(|&r| r);
    if !overlap && !all_same {
        retrace_ranges.push((begin, n));
        retraced[begin..begin + n].fill(true);
    }
}
```

When `n > 1` and every word in the n-gram is the same token, the match
is skipped.  The n=1 pass then handles each repetition individually,
producing the correct `word [/] word [/] word` output.

Multi-word phrase retraces are unaffected — they contain at least two
distinct words, so `all_same` is always false.

---

## Tests Added

Three new test cases in
`batchalign/tests/pipelines/cleanup/test_cleanup_chat_text.py`:

| Test | Input | Expected | Validates |
|------|-------|----------|-----------|
| `test_quadruple_single_word_retrace` | `the the the the dog` | `the [/] the [/] the [/] the dog` | The exact bug Davida reported |
| `test_triple_bigram_retrace` | `I want I want I want a cookie` | `<I want> [/] <I want> [/] I want a cookie` | Multi-word retraces still correct |
| `test_nested_retrace` (strengthened) | `ice ice ice cream` | `ice [/] ice [/] ice cream` | Triple single-word, no brackets |

---

## Lessons

1. **Largest-first n-gram matching needs a same-word guard.**  Any
   algorithm that prioritizes larger matches must check for the
   degenerate case where a "phrase" is really a repeated single word.

2. **Python master's design was accidentally robust here.**  The
   smallest-first scan naturally avoids the bug, and the serializer's
   text-equality state machine provides a second layer of defense.  But
   the robustness is accidental — the serializer's complexity exists
   because of the smallest-first strategy's own limitations with
   multi-word groups, not as a deliberate guard against this case.

3. **Write tests for repetition counts > 2.**  The original test suite
   covered single repeats (`the the dog`) and triple repeats
   (`ice ice ice cream`) but not quadruple repeats, where n=2 first
   becomes applicable to a same-word sequence.  The bug manifests at
   exactly 4+ repetitions of a single word.

4. **User feedback on real transcribe output catches edge cases that
   synthetic tests miss.**  The quadruple stutter pattern is uncommon
   in curated test data but common in real speech, especially from
   children and people with fluency disorders — exactly the populations
   TalkBank studies.
