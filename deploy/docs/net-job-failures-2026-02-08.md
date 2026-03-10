# Net Server Job Failures — 2026-02-08

All failures submitted by **brian** (100.66.91.120) between 21:54–22:59 UTC.
29 total jobs, 25 succeeded, 4 failed.

## 1. `32ce1637b59a` — align, 28 files: No media tier

**Error:** `We cannot add utterance timings to something that doesn't have a media path! Provided media tier='None'`

**Cause:** CHAT files have no `@Media:` line. Can't do forced alignment without an audio reference.

**Fix:** Data quality — add `@Media:` headers to these CHAT files, or don't run align on them.

---

## 2. `f6579e3b442a` — morphotag, 28 Dutch files: CHAT parse error

**Error:** `Unknown line in input CHAT: '	, maar +...'` (and similar for all 28 files)

**Cause:** Bug in batchalign. CHAT files with tab-indented continuation lines were not being preprocessed when loaded via the `lines=` parameter (used by the HTTP server). The file-path codepath had this preprocessing, but the server codepath did not.

**Fix:** Committed in `ee9ac595`. The `CHATFile(lines=...)` constructor now merges tab-continuation lines before parsing, matching the file-path behavior. Deploy to net and retry.

---

## 3. `60bb8760c873` — align, 15 files: Media not found (DutchAfrikaans/DeHouwer)

**Error:** `Media file '040824' not found via mapping 'childes-data' at: /Volumes/CHILDES/CHILDES/DutchAfrikaans/DeHouwer`

**Cause:** Data quality — audio files are not in the expected directory alongside the CHAT files. Likely in a subdirectory (e.g., `0wav/`) rather than at the corpus root.

**Fix:** Move audio files to the same directory as the CHAT files (parallel structure).

---

## 4. `1ecd34310a24` — align, 132 files: Media not found (Eng-AAE/Edwards)

**Error:** `Media file '105' not found via mapping 'childes-data' at: /Volumes/CHILDES/CHILDES/Eng-AAE/Edwards`

**Cause:** Same as #3 — audio files not at the expected path.

**Fix:** Same as #3 — fix directory structure so audio is alongside CHAT files.

---

## Summary

| # | Job ID | Command | Files | Root Cause | Action |
|---|--------|---------|-------|------------|--------|
| 1 | `32ce1637b59a` | align | 28 | No `@Media:` in CHAT | Data fix |
| 2 | `f6579e3b442a` | morphotag | 28 | Tab-continuation bug | Code fix (deployed) |
| 3 | `60bb8760c873` | align | 15 | Media in subdirectory | Data fix |
| 4 | `1ecd34310a24` | align | 132 | Media in subdirectory | Data fix |

3 of 4 failures are data quality issues (missing/misplaced media). 1 was a real bug, now fixed.
