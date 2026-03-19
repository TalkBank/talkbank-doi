# Bracket Singleton Review

**Last updated:** 2026-03-19

165 bracket pairs were identified as singletons (no matching partner from a
different speaker). Each was classified by automated analysis:

## Summary

| Category | Count | Action |
|----------|-------|--------|
| SKIP | 66 | Zero-timestamp untimed sections (SBC011) + bogus speakers from broken formatting |
| LEGITIMATE_NESTING | 2 | Correctly unnumbered — different overlap layer |
| FIX_INDEX | 15 | Unnumbered bracket should have index to match partner |
| FIX_PARTNER | 19 | Partner bracket should have index to match this one |
| SOLO | 10 | No plausible partner — onset-only or isolated |
| REVIEW | 53 | Needs manual investigation |

## FIX_INDEX (15 auto-fixable)

These brackets are unnumbered but their partner has a numbered index.
Fix: add the partner's index to this bracket.

- SBC004 line 810: SHARON `[` → `[2` (match KATHY `[2` on line 811)
- SBC006 line 1345: ALINA `[` → `[2` (match LENORE `[2` on line 1346)
- SBC010 line 142: BRAD `[` → `[2` (match PHIL `[2` on line 141)
- SBC010 line 673: BRAD `[` → `[2` (match PHIL `[2` on line 676)
- SBC010 line 674: BRAD `[` → `[2` (match PHIL `[2` on line 676)
- SBC015 line 1570: KEN `[` → `[5` (match JOANNE `[5` on line 1571)
- SBC019 line 16: MELISSA `[` → `[2` (match JAN `[2` on line 14)
- SBC023 line 202: LINDA `[` → `[2` (match JANICE `[2` on line 205)
- SBC023 line 261: JANICE `[` → `[2` (match SUE `[2` on line 263)
- SBC029 line 591: SETH `[` → `[2` (match LARRY `[2` on line 593)
- SBC047 line 973: RICHARD `[` → `[2` (match FRED `[2` on line 974)
- SBC049 line 1030: DAVE `[` → `[2` (match LUCY `[2` on line 1029)
- SBC051 line 147: ALICE `[` → `[7` (match SEAN `[7` on line 146)
- SBC058 line 405: STEVEN `[` → `[2` (match SHERI `[2` on line 404)
- SBC059 line 975: WESS `[` → `[3` (match JO `[3` on line 974)

## FIX_PARTNER (19 auto-fixable)

These brackets have a numbered index but their partner is unnumbered.
Fix: add this bracket's index to the partner.

- SBC009 line 107: NATHAN `[` → `[2` (to match KATHY `[2` on line 110)
- SBC010 line 281: PHIL `[` → `[2` (to match BRAD `[2` on lines 284-285)
- SBC010 line 667: PHIL `[` → `[2` (to match BRAD `[2` on line 669)
- SBC013 line 1368: KEVIN `[` → `[2` (to match WENDY `[2` on line 1370)
- SBC013 line 1601: KENDRA `[` → `[2` (to match MARCI `[2` on line 1602)
- SBC015 line 348: KEN `[` → `[3` (to match JOANNE `[3` on line 346)
- SBC019 line 167: FRANK `[` → `[4` (to match RON `[4` on line 168)
- SBC021 line 245: AUD `[` → `[2` (to match WALT `[2` on line 246)
- SBC023 line 279: SUE `[` → `[2` (to match LINDA `[2` on line 280)
- SBC024 line 714: JENNIFER `[` → `[2` (to match DAN `[2` on line 715)
- SBC031 line 309: SHERRY `[` → `[2` (to match ROSEMARY `[2` on line 310)
- SBC032 line 157: TOM_1 `[` → `[2` (to match TOM_2 `[2` on line 156)
- SBC041 line 65: KRISTIN `[` → `[2` (to match PAIGE `[2` on line 66)
- SBC041 line 911: KRISTIN `[` → `[2` (to match PAIGE `[2` on line 912)
- SBC047 line 973: RICHARD `[` → `[2` (already in FIX_INDEX — both sides need fixing)
- SBC058 line 405: STEVEN `[` → `[2` (already in FIX_INDEX — both sides need fixing)
- SBC059 line 526: CAM `[` → `[4` (to match FRED `[4` on line 527)
- SBC059 line 974: JO `[` → handled via FIX_INDEX on line 975

## REVIEW (53 cases)

Need manual investigation. Categories:
- **Indented, no nearby partner** (33): Bracket is indented (suggesting
  alignment) but no plausible partner found within 5 lines.
- **Index conflict** (~8): Two numbered brackets from different speakers
  that should pair but have different indices.
- **Other** (~12): Various edge cases.

## SOLO (10 cases)

No plausible partner exists. These may be:
- Onset-only overlaps (speaker starts overlapping but the other speaker's
  bracket was never transcribed)
- Isolated brackets from transcription errors
- Brackets whose partner was removed during manual editing
