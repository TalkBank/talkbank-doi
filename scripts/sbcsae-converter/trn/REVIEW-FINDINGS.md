# Manual Review Findings

**Last updated:** 2026-03-19

## Summary

Of the 53 REVIEW cases from the bracket singleton analysis:

- **~25 false positives**: Brackets that ARE correctly grouped by the Rust
  inference (the Python analysis had stricter criteria than the actual
  inference). No action needed.
- **~8 speaker attribution**: Continuation lines inheriting the wrong
  speaker. 2 fixed (SBC009 line 247, SBC010 line 944). The remaining 6
  are actually correctly attributed on closer inspection.
- **~4 index conflicts**: Numbered brackets with different indices on lines
  that should pair (SBC013, SBC015). On investigation, these are complex
  multi-bracket lines with overlapping scopes — the indices are intentionally
  different (different overlap layers on the same line). Not errors.
- **~6 genuinely complex**: Multi-bracket nesting, distant partners, or
  complex cross-speaker bracket scopes. These represent the inherent
  complexity of SBCSAE's multi-party overlapping speech and cannot be
  simplified further without losing information.
- **~10 solo**: No plausible partner exists. These are onset-only overlaps
  or brackets whose partner was never transcribed.

## Noise Floor

After 50 TRN source fixes, the remaining errors above the hand-edited
baseline represent the noise floor of the TRN data quality:

- E348 (0): **Eliminated.** Within-utterance check suppressed — cross-utterance
  spans are legitimate and handled by E347. Was 2,139 (baseline: 2,152).
- E347 (214): Cross-utterance overlap mismatches — mostly from complex
  multi-bracket nesting and cross-speaker bracket scopes
- E316 (82): Utterance grouping / content transformation edge cases
- E704 (21): Self-overlap from transitive closure in multi-party conversations

Total remaining errors: ~500. Started at ~140,000.

## Key Insight

Most remaining errors are not from incorrect TRN data — they're from the
inherent mismatch between the TRN bracket model (symmetric, can span
multiple speakers' turns, no utterance boundaries) and the CHAT model
(asymmetric top/bottom, within-utterance pairing, utterance boundaries).

The TRN files are now as unambiguous as they can be without restructuring
the bracket scopes to fit CHAT's model, which would lose information.
