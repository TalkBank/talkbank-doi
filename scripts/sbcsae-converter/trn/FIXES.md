# TRN Source File Fixes

Fixes to the original SBCSAE `.trn` files from UCSB, applied to enable clean
parsing. Each fix corrects a transcriber error where overlap brackets were
embedded inside environmental comment names or positioned incorrectly.

## SBC008.trn line 855

**Before:** `[3((KNOCK_KNOCK3]_KNOCK))`
**After:** `[3((KNOCK_KNOCK_KNOCK))3]`
**Issue:** Overlap close `3]` embedded inside `((COMMENT))` name. The transcriber
placed the overlap close marker inside the environmental comment instead of after it.
**Ref:** Hand-edited CHAT has `⌊ &=KNOCK_KNOCK_KNOCK ⌋`.

## SBC056.trn line 838

**Before:** `[((CLAP]_CLAP_CLAP))`
**After:** `[((CLAP_CLAP_CLAP))]`
**Issue:** Overlap close `]` embedded inside `((COMMENT))` name. Should wrap the
entire comment.
**Ref:** Hand-edited CHAT has `⌊ &=CLAP_CLAP_CLAP ⌋`.

## SBC058.trn line 401

**Before:** `((SNAP_SNAP_SNAP_SNAP_[SNAP_SNAP_SNAP))]`
**After:** `[((SNAP_SNAP_SNAP_SNAP_SNAP_SNAP_SNAP))]`
**Issue:** Overlap open `[` embedded inside `((COMMENT))` name. The `_` before `[`
was part of the SNAP concatenation. Moved bracket to wrap entire comment.
**Ref:** Hand-edited CHAT has `⌈ &=SNAP_SNAP_SNAP_SNAP_SNAP_SNAP_SNAP ⌉`.
