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

## SBC007.trn line 631

**Before:** `... And !Nicky helps her .. with him a lot anyway.\x00000000000 000000000 MARY: 1182.90 ...`
**After:** Split into two lines (NUL byte joined two TRN lines into one).
**Issue:** NUL byte (0x00) in the middle of a line concatenated two separate TRN
lines. The encoding repair replaced NUL with 'c', making the second line's
timestamps appear as content.

## SBC007.trn line 519

**Before:** `(YAWN0 Unhunh,`
**After:** `(YAWN) Unhunh,`
**Issue:** Missing close `)` and stray digit `0` in vocalism.

## SBC019.trn line 116

**Before:** `... (SNIFF .. (Hx) (Hx)=)`
**After:** `... (SNIFF) .. (Hx) (Hx)=`
**Issue:** Unbalanced parentheses — outer `(SNIFF` not closed before inner `(Hx)`.

## SBC019.trn line 100

**Before:** `<F<VOX> Mo=m VOX>,`
**After:** `<F<VOX Mo=m VOX>,`
**Issue:** Spurious `>` between `<VOX` begin and content. The `>` is not a close
marker here — the actual close is `VOX>` later.

## SBC024.trn line 250

**Before:** `... <HUMMING>`
**After:** `... (HUMMING)`
**Issue:** `<HUMMING>` used as simple vocalism instead of paired `<HUMMING...HUMMING>`.
Converted to `((HUMMING))` vocalism notation.

## SBC024.trn line 560

**Before:** `[Look okay)].`
**After:** `[Look okay].`
**Issue:** Stray `)` inside overlap bracket — unmatched close parenthesis.
**Ref:** Hand-edited CHAT has `⌈ Look okay ⌉`.

## SBC054.trn line 578

**Before:** `... <VOX Ugh VOX >.`
**After:** `... <VOX Ugh VOX>.`
**Issue:** Space between label `VOX` and closing `>` in long feature.
