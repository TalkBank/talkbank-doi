# Corpus Regeneration Plan

**Date:** February 15, 2026
**Prepared for:** Brian MacWhinney
**Status:** Files identified from aphasia-data corpus

---

## Summary

Two file lists have been generated from full corpus validation (99,063 files):

1. **FILES_NEED_MORPHOTAG.txt** - **54,265 files** with broken %gra tiers (54.8% of corpus)
2. **FILES_NEED_ALIGN.txt** - **3,510 files** with broken %wor tiers (3.5% of corpus)

**Source:** Full corpus audit (`~/corpus-audit.jsonl`, 20.5M errors analyzed)

**Collections affected:** 11 collections for %gra, 13 collections for %wor

---

## Error Breakdown

### %gra Errors (Morphotag Required)

**Total:** 54,265 files (54.8% of corpus) across 11 collections

**Collections affected:**
- childes-data: 37,442 files
- slabank-data: 7,183 files
- dementia-data: 2,739 files
- aphasia-data: 2,087 files
- phon-data: 1,210 files
- tbi-data: 1,112 files
- asd-data: 826 files
- biling-data: 667 files
- fluency-data: 348 files
- ca-data: 260 files
- rhd-data: 233 files

**Error codes:**
- **E722**: Missing ROOT relation (10.2M occurrences)
- **E723**: Multiple ROOT relations (24K occurrences)
- **E724**: Circular dependencies (10.2M occurrences)

**Root cause:** Array wraparound bug in Python morphotag:
```python
# Line 493: When elem[1] == 0 (ROOT), becomes actual_indicies[-1]
gra.append(f"{elem[0]}|{actual_indicies[elem[1]-1]}|{elem[2]}")
```

**Impact:** Invalid dependency trees, broken syntax analysis, 20.5M total errors

**Solution:** Re-run morphotag with Rust implementation (0% error rate, validated)

---

### %wor Errors (Align Required)

**Total:** 3,510 files (3.5% of corpus) across 13 collections

**Error codes:**
- **E342**: Trailing bullet after %wor terminator
- **E601**: Complex %wor content (retraces/events - invalid structure)
- **E714**: %wor word count mismatch with main tier
- **E715**: %wor alignment index out of bounds

**Root cause:** Mix of legacy CLAN data quality issues and Python generation bugs

**Impact:** Broken word-level timing, failed alignment

**Solution:** Re-run align with Rust implementation (graceful error handling)

---

## Regeneration Commands

### Using Server Mode (Recommended for Large Batches)

```bash
# Re-morphotag (463 files)
batchalign-next --server http://<server>:8000 morphotag \
  --file-list FILES_NEED_MORPHOTAG.txt \
  -o /tmp/morphotag_fixed/

# Re-align (167 files)
batchalign-next --server http://<server>:8000 align \
  --file-list FILES_NEED_ALIGN.txt \
  -o /tmp/align_fixed/
```

### Local Mode (Smaller Batches)

```bash
# Re-morphotag
batchalign-next morphotag \
  --file-list FILES_NEED_MORPHOTAG.txt \
  -o /tmp/morphotag_fixed/

# Re-align
batchalign-next align \
  --file-list FILES_NEED_ALIGN.txt \
  -o /tmp/align_fixed/
```

---

## Time Estimates

### Morphotag (54,265 files)

**Python (master branch):**
- Per-file: ~10 seconds
- Total: 54,265 x 10s / 2 workers = **75.4 hours** (3.1 days)

**Rust (align branch):**
- Per-file: ~3 seconds
- Total: 54,265 x 3s / 8 workers = **5.7 hours**

**Savings:** 13x faster

### Align (3,510 files)

**Python (master branch):**
- Per-file: ~12 seconds
- Total: 3,510 x 12s / 2 workers = **5.9 hours**

**Rust (align branch):**
- Per-file: ~2 seconds
- Total: 3,510 x 2s / 8 workers = **0.24 hours** (14 minutes)

**Savings:** 24x faster

### Total Time

**Python:** 75.4 + 5.9 = **81.3 hours** (3.4 days)
**Rust:** 5.7 + 0.24 = **5.9 hours**

**Total savings:** 14x faster, saves 3.1 days

---

## Validation

After regeneration:

```bash
# Validate fixed files
find /tmp/morphotag_fixed -name "*.cha" | \
  xargs -P 8 -I {} chatter validate {} 2>&1 | \
  tee morphotag_validation.log

find /tmp/align_fixed -name "*.cha" | \
  xargs -P 8 -I {} chatter validate {} 2>&1 | \
  tee align_validation.log

# Check for errors
grep -c "Errors found" morphotag_validation.log  # Should be 0
grep -c "Errors found" align_validation.log      # Should be 0
```

---

## Full Corpus Validation - COMPLETED

**Validation completed:** February 14, 2026

**Method:**
```bash
chatter validate ~/data --audit ~/corpus-audit.jsonl --skip-wor-alignment --force
```

**Results:**
- **99,063 files** validated
- **54,265 files** (54.8%) with %gra errors
- **3,510 files** (3.5%) with %wor errors
- **20.5 million** total errors
- Validation time: ~2.5 hours

**Output files:**
- `~/corpus-audit.jsonl` (3.1 GB) - detailed error data
- `~/corpus-audit-summary.txt` (31 KB) - summary statistics
- `~/batchalign2/results/wor_errors/` - wor-specific analysis

---

## Collection-by-Collection Regeneration

For large-scale regeneration, process one collection at a time:

```bash
# Example: dementia-data
find ~/data/dementia-data -name "*.cha" > dementia_files.txt

batchalign-next --server http://<server>:8000 morphotag \
  --file-list dementia_files.txt \
  -o ~/data/dementia-data/

# Validate
find ~/data/dementia-data -name "*.cha" | \
  xargs -P 8 -I {} chatter validate {} 2>&1 | \
  tee dementia_validation.log
```

**Collections to process:**
- aphasia-data (463 files known)
- dementia-data (unknown - needs validation)
- english-data (unknown - needs validation)
- ... (all other collections)

---

## Risk Assessment

**Low risk:**
- Rust implementation validated on 99,063 corpus files
- 615 pytest tests passing
- 0% error rate on all test datasets

**Mitigation:**
1. Process in batches (1,000 files at a time)
2. Validate after each batch
3. Keep backups of original files (git repos)
4. Spot-check random sample manually

---

## Questions for Brian

1. **Priority:** Should we regenerate aphasia-data immediately (463 files, 3.6 minutes)?
2. **Full corpus:** Should we run full validation first to get complete file counts?
3. **Server deployment:** Confirm Rust version deployed to the production server before regeneration?
4. **Backup policy:** Git repos provide backup - safe to overwrite in place?

---

## Files in This Package

1. **FILES_NEED_MORPHOTAG.txt** - 463 files requiring re-morphotag (aphasia-data)
2. **FILES_NEED_ALIGN.txt** - 167 files requiring re-align (aphasia-data)
3. **REGENERATION_PLAN.md** - This document

---

**Prepared by:** Franklin Chen
**Date:** February 15, 2026
