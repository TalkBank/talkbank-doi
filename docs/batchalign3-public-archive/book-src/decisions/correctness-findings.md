# Correctness Testing Results
## Python (bench-baseline) vs Rust (align branch)

**Date**: 2026-02-15
**Test Dataset**: `~/batchalign-benchmarking/data/correctness_morphotag` (7 files: English, Catalan, Dutch)
**Commands Tested**: align, morphotag, translate (running), utseg, coref

---

## Executive Summary

**CRITICAL FINDINGS**:
1. **ALIGN**: Completely different timestamps between Python and Rust implementations
2. **MORPHOTAG**: Multiple systematic differences in linguistic annotation
3. **COREF**: Different tier names (`%coref:` vs `%xcoref:`), Python crashes on non-English
4. **UTSEG**: Python completely crashed, Rust partially succeeded
5. **ROBUSTNESS**: Rust handles non-English files better across all commands

---

## 1. ALIGN (Particularly Critical)

**Status**: CRITICAL - COMPLETELY DIFFERENT OUTPUT

### Python Results
- 3/3 files processed successfully
- Output location: `~/correctness_test/python/align/`

### Rust Results
- 3/3 files processed successfully
- Output location: `~/correctness_test/rust/align/`

### Key Differences

**Timestamps are completely different**:
```diff
Python:  *MOT:	you gonna cook some more ? 8465_9627
         %wor:	you 8465_8545 gonna 8545_8706 cook 8746_8926 some 8987_9127 more 9127_9627 ?

Rust:    *MOT:	you gonna cook some more ? 140_981
         %wor:	you 140_200 gonna 200_420 cook 420_661 some 661_801 more 801_981 ?
```

**Completeness**:
- Python: Some utterances missing `%wor:` tiers
- Rust: All utterances have `%wor:` tiers (more complete)

**File**: `01DM_18.cha`, `02JW_18.cha`, `03PH_18.cha`

### Questions for Review
1. Which timing scheme is correct?
2. Why are the timestamps so different? (8465ms vs 140ms for same word)
3. Is one using absolute time and the other relative?
4. Why does Python omit some %wor tiers?

---

## 2. MORPHOTAG (Particularly Critical)

**Status**: SYSTEMATIC DIFFERENCES

### Python Results
- 7/7 files processed successfully
- Output location: `~/correctness_test/python/morphotag/`

### Rust Results
- 7/7 files processed successfully
- Output location: `~/correctness_test/rust/morphotag/`

### Key Differences

#### A. Reflexive Pronoun Annotation
```diff
- Python: pron|ell-Prs-AccDat-reflx-S3
+ Rust:   pron|ell-Prs-AccDat-S3
```
Rust omits the `-reflx` suffix throughout.

#### B. POS Tag for Communicators
```diff
- Python: c|nonon
+ Rust:   x|nonon
```
Python uses `c|` (communicator), Rust uses `x|` (unknown).

#### C. Unknown/Foreign Word Handling
```diff
- Python: L2|xxx          (second language marker + unintelligible)
+ Rust:   x|biberon       (unknown + actual word)
         x|vale
         x|esto
```
Python marks as L2/unintelligible, Rust marks as unknown but preserves the word.

#### D. %wor Tier Suffixes
```diff
Python %wor:  biberon@s 255528_256069
Rust %wor:    biberon 255528_256069

Python %wor:  vale@s 313140_313560
Rust %wor:    vale 313140_313560

Python %wor:  nonon@c 341990_342170
Rust %wor:    nonon 341990_342170
```
Python adds `@s` (Spanish) and `@c` (communicator) suffixes in %wor tiers, Rust doesn't.

**File**: `010911.cha` (Catalan)

### Questions for Review
1. Which reflexive pronoun annotation is linguistically correct?
2. Should communicators be tagged `c|` or `x|`?
3. Should foreign words in %wor keep language suffixes?
4. Is L2|xxx or x|word better for analysis?

---

## 3. COREF (Coreference Resolution)

**Status**: DIFFERENT OUTPUT + ROBUSTNESS ISSUE

### Python Results
- **3/7 files** succeeded (English only)
- **4/7 files** crashed with `'NoneType' object has no attribute 'content'`
- Failed on: `010911.cha` (Catalan), `asd36.cha`, `asd44.cha`, `cs02mnwa.cha` (Dutch)
- Output location: `~/correctness_test/python/coref/`

### Rust Results
- **7/7 files** succeeded (all languages)
- Output location: `~/correctness_test/rust/coref/`

### Key Differences

#### A. Tier Name
```diff
- Python: %coref:
+ Rust:   %xcoref:
```

#### B. Trailing Dashes
```diff
- Python: %coref:	-, -, -, -, (0), -, -, -, (0), -, -, (1), -, (2, 2, 2, (0) 2, 2, 2), -
+ Rust:   %xcoref:	-, -, -, (0), -, -, -, (0), -, -, (1), -, (2, 2, 2, (0) 2, 2, 2)
```
Python has trailing dashes, Rust doesn't.

#### C. Reference Numbering
Different coreference cluster IDs throughout (Python uses higher numbers).

**File**: `176-1.cha`

### Questions for Review
1. Why is the tier named `%xcoref:` instead of `%coref:`?
2. Which reference numbering scheme is correct?
3. Why does Python crash on non-English files?
4. Should trailing dashes be present or not?

---

## 4. UTSEG (Utterance Segmentation)

**Status**: PYTHON COMPLETELY FAILED

### Python Results
- **0/7 files** succeeded
- All 7 files crashed with: `A process in the process pool was terminated abruptly while the future was running or pending`
- Output location: `~/correctness_test/python/utseg/` (EMPTY)

### Rust Results
- **3/7 files** succeeded (English only: `176-1.cha`, `187-1.cha`, `MSU08b.cha`)
- **4/7 files** failed with expected Stanza limitation: `UnsupportedProcessorError: Processor constituency is not known for language ca/nl`
- Failed on: `010911.cha`, `cs02mnwa.cha` (Catalan), `asd36.cha`, `asd44.cha` (Dutch)
- Output location: `~/correctness_test/rust/utseg/`

### Key Differences
- **Python**: Complete failure (worker processes crashed)
- **Rust**: Graceful failure with informative error messages

### Questions for Review
1. Why did Python worker processes crash?
2. Is this a memory issue or a bug?
3. Rust's graceful error handling is clearly superior here

---

## 5. TRANSLATE

**Status**: RUNNING (both Python and Rust)

Translation is slow (~7-8 minutes for 7 files). Tests started at 17:42, expected completion ~17:50.

Results pending.

---

## Summary Table

| Command    | Python Success | Rust Success | Key Issue |
|------------|----------------|--------------|-----------|
| align      | 3/3           | 3/3          | **Completely different timestamps** |
| morphotag  | 7/7           | 7/7          | Systematic annotation differences |
| translate  | running       | running      | Pending |
| utseg      | 0/7           | 3/7          | Python crashed, Rust partial |
| coref      | 3/7           | 7/7          | Different tier names, Python crashes on non-English |

---

## Robustness Comparison

**Rust wins clearly**:
- Handles non-English files better (coref: 7/7 vs 3/7)
- Graceful error messages (utseg: informative errors vs crashes)
- More complete output (align: all utterances have %wor)

**Python issues**:
- Crashes on non-English coref files
- Complete worker pool crash on utseg
- Missing %wor tiers in align output

---

## Next Steps

1. **Review align timestamps** - CRITICAL, core functionality
2. **Review morphotag linguistic decisions** - affects all downstream analysis
3. **Investigate Python crashes** - utseg and coref reliability issues
4. **Wait for translate results** - should complete in ~2 minutes
5. **Decide which outputs to trust** for production deployment

---

## Test Commands

### Python (bench-baseline)
```bash
cd ~/batchalign2-bench-baseline
uv run batchalign-next align ~/batchalign-benchmarking/data/align_small ~/correctness_test/python/align
uv run batchalign-next morphotag ~/batchalign-benchmarking/data/correctness_morphotag ~/correctness_test/python/morphotag
uv run batchalign-next coref ~/batchalign-benchmarking/data/correctness_morphotag ~/correctness_test/python/coref
uv run batchalign-next utseg ~/batchalign-benchmarking/data/correctness_morphotag ~/correctness_test/python/utseg
```

### Rust (align)
```bash
cd ~/batchalign2
uv run batchalign-next align ~/batchalign-benchmarking/data/align_small ~/correctness_test/rust/align
uv run batchalign-next morphotag ~/batchalign-benchmarking/data/correctness_morphotag ~/correctness_test/rust/morphotag
uv run batchalign-next coref ~/batchalign-benchmarking/data/correctness_morphotag ~/correctness_test/rust/coref
uv run batchalign-next utseg ~/batchalign-benchmarking/data/correctness_morphotag ~/correctness_test/rust/utseg
```
