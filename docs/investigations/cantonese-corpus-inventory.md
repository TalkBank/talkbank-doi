# Cantonese Corpus Inventory and Testing Status

**Status:** Current
**Last updated:** 2026-03-23 19:39 EDT

## Purpose

This document tracks all Cantonese data in TalkBank, what we've tested against,
what we haven't, and findings from each corpus. Every claim about Cantonese NLP
quality must cite which corpora were tested and which weren't.

## Inventory

| Corpus | Repo | Path | Files | Utterances | Has %mor | Domain | Tested |
|--------|------|------|-------|------------|----------|--------|--------|
| **MOST** | childes-other-data | Chinese/Cantonese/MOST/ | 892 | 166,848 | 0 | Child Cantonese | Word seg 84%, POS 1% X |
| **LeeWongLeung** | childes-other-data | Chinese/Cantonese/LeeWongLeung/ | 161 | 254,501 | 243,466 | Child Cantonese | Word seg 89%, POS 0% X |
| **CHCC** | childes-other-data | Biling/CHCC/Winston/Cantonese/ | 237 | 135,036 | 126,866 | Child bilingual (yue+cmn+eng) | Word seg 88%, POS 0% X |
| **EACMC** | childes-other-data | Biling/EACMC/ | 216 | 110,759 | 98,660 | Child bilingual | Word seg 90%, POS 1% X |
| **HKU (CHILDES)** | childes-other-data | Chinese/Cantonese/HKU/ | 70 | 26,850 | 26,305 | Child Cantonese | Word seg 86%, POS 2% X |
| **MAIN** | childes-other-data | Chinese/Cantonese/MAIN/ | 334 | 17,274 | 17,274 | Child narrative | Word seg 87%, POS 2% X |
| **GlobalTales** | childes-other-data | GlobalTales/Cantonese/ | 65 | 19,849 | 19,849 | Cantonese narratives | Word seg 85%, POS 2% X |
| **WCT** | ca-data | WCT/ | 58 | 4,950 | 4,950 | Conversation analysis | POS 2% X (word seg skipped) |
| **HKU (Aphasia)** | aphasia-data | Cantonese/Protocol/HKU/ | 9 | 887 | 876 | Adult clinical | Word seg 90%, POS 0% X |
| **PaidoCantonese** | phon-other-data | Chinese/Cantonese/PaidoCantonese/ | 160 | — | — | Phonology | Not tested (phonology, not morphosyntax) |

**Total: ~2,200 files, ~737,000 utterances, ~538,000 with existing %mor**

### External Cantonese Resources

| Resource | Tokens | POS | Deps | Notes |
|----------|--------|-----|------|-------|
| **UD_Cantonese-HK** | 13,918 | UD | UD | Used for Stanza training. Film + legislative. |
| **HKCanCor** (PyCantonese) | 153,656 | Chinese (95 tags) | No | Built into PyCantonese. 16,162 utterances. POS tags are Chinese-style (v/r/d/n/y), not UD. PyCantonese maps them to UD internally. Could augment POS training after tag conversion, but cannot help depparse (no dependency annotations). |

## What We've Tested (as of 2026-03-23)

### Word segmentation (PyCantonese) — ALL 9 CORPORA
- **Test:** `test_cantonese_all_corpora.py` — extracted unique CJK words from every corpus
- **Results:** 84-90% multi-char word preservation across all 8 testable corpora (WCT skipped for word seg — already has word boundaries)
  - LeeWongLeung: 89%, HKU: 86%, MAIN: 87%, GlobalTales: 85%
  - CHCC: 88%, EACMC: 90%, Aphasia HKU: 90%, MOST: 84%
- **Detailed CHCC analysis:** 91% on 1,363 unique multi-char words (`test_pycantonese_corpus_quality.py`)
- **Conclusion:** Results are consistent across all corpora, not corpus-specific

### POS accuracy (PyCantonese override) — ALL 9 CORPORA
- **Test:** `test_cantonese_all_corpora.py` — POS-tagged vocabulary from each corpus
- **Results:** 0-2% unknown (X) rate across all 9 corpora (98-100% vocabulary coverage)
- **Manual judgments:** 11 cases from CHCC+Aphasia: PyCantonese correct 4, corpus correct 0, ambiguous 7
- **Tagset analysis:** 49% raw disagreement with existing %mor; after normalization, 97% of disagreements are genuine (existing annotations from Mandarin Stanza model)

### Trained Cantonese Stanza model
- **Evaluated on:** UD_Cantonese-HK test set (101 sentences, 1,484 tokens)
- **Results:** POS 93.5%, UAS 70.4%, LAS 65.2% (vs baseline Mandarin model: 62.9%/39.9%/23.5%)
- **Hybrid (trained model + PyCantonese POS):** POS 95%, dependency parse ~80%
- **NOT yet evaluated on TalkBank corpus data** (model not deployed into batchalign3 yet)

### Tencent ASR word segmentation
- **Tested on:** Aphasia HKU A023 clip (1 file)
- **Finding:** 25 CJK words, 0 multi-character — 100% per-character output
- **Conclusion:** All 4 engines (Whisper, Tencent, Aliyun, FunASR) produce per-character Cantonese

### End-to-end morphotag --retokenize
- **MOST corpus:** 166,848 utterances, previously 0% morphosyntax annotation. Now processes successfully after retrace bug fixes (bugs #1-#4).
- **Bugs found and fixed:** 6 bugs total (4 fixed in pipeline code, 2 open — see `cantonese-retokenize-known-bugs.md`)

## What Still Needs Testing

### Priority 1: Trained model on TalkBank data
The trained Cantonese Stanza model (on bilbo) has not been evaluated against TalkBank corpus data. Need to deploy model into batchalign3 and test on real corpora.

### Priority 2: End-to-end morphotag quality spot-checks
Run morphotag --retokenize on sample files from each corpus and manually inspect:
- %mor POS correctness (spot-check against linguistic expectations)
- %gra structural validity (single root, subjects before verbs)

### Priority 3: HKCanCor as additional training data
HKCanCor has 153,656 tokens with Chinese-style POS tags. Need to map to UD and assess whether it can augment the 1,004-sentence UD_Cantonese-HK treebank.

## Findings Log

### 2026-03-23: ALL CORPORA (word segmentation + POS)
- **Script:** `batchalign3/batchalign/tests/pipelines/morphosyntax/test_cantonese_all_corpora.py`
- **Word seg results:** 84-90% multi-char preservation across all 8 testable corpora
  - LeeWongLeung: 89%, HKU: 86%, MAIN: 87%, GlobalTales: 85%
  - CHCC: 88%, EACMC: 90%, Aphasia: 90%, MOST: 84%
- **POS results:** 0-2% unknown (X) rate across all 9 corpora
- **Conclusion:** Results are consistent, not corpus-specific

### 2026-03-23: CHCC (word segmentation, detailed)
- **Script:** `batchalign3/batchalign/tests/pipelines/morphosyntax/test_pycantonese_corpus_quality.py`
- **Result:** 91% multi-char word preservation (1,243/1,363 words)
- **Gaps:** Book titles, idiomatic expressions, proper names split into constituents

### 2026-03-23: Aphasia HKU A016/A017 (POS)
- **Script:** `batchalign3/batchalign/tests/pipelines/morphosyntax/test_cantonese_corpus_gold.py`
- **Result:** PyCantonese correct on 踢, 朋友, 冷氣. Gaps on 油罐車(X), 踢爛(ADJ), 啦(X)

### 2026-03-23: Tencent ASR produces per-character Cantonese
- **Script:** `scripts/check-media/verify_tencent_cantonese.sh` (Aphasia HKU A023 clip)
- **Result:** 25 CJK words, 0 multi-character. 100% per-character output.
- **Conclusion:** All 4 engines (Whisper, Tencent, Aliyun, FunASR) produce per-char.

### 2026-03-23: cconj→adv disagreement investigated
- **Script:** ad hoc (to be converted to proper test)
- **Result:** 唔 (840x) and 咁 (218x) are misclassified as CCONJ in existing annotations. PyCantonese correctly tags both as ADV.
- **Impact:** This is the single largest disagreement pattern (1,069 cases).

### 2026-03-23: Tagset normalization analysis
- **Script:** `batchalign3/batchalign/tests/pipelines/morphosyntax/test_cantonese_tagset_analysis.py`
- **Result:** Raw agreement 49%. After normalizing equivalences (aux↔part, sconj↔cconj, propn↔noun), only rises to 51%. 97% of disagreements are genuine.

### 2026-03-23: Manual POS judgments
- **Script:** `batchalign3/batchalign/tests/pipelines/morphosyntax/test_cantonese_pos_judgments.py`
- **Result:** 11 cases judged: PyCantonese correct 4, corpus correct 0, ambiguous 7.

### 2026-03-23: Corpus %mor provenance
- **MAIN:** `Batchalign 0.7.23, ASR Engine funaudio` (Mandarin Stanza model)
- **GlobalTales:** `Batchalign 0.7.17, ASR Engine tencent` (Mandarin Stanza model)
- **HKU CHILDES:** Hand-transcribed (named transcribers, 1998-99)
- **Aphasia HKU:** Hand-transcribed (named transcribers, 2011-12)

### 2026-03-23: UD_Cantonese-HK test set (trained model)
- **Script:** `~/cantonese-stanza-training/scripts/evaluate_all.py` on bilbo
- **Result:** POS 93.5%, UAS 70.4%, LAS 65.2% (vs baseline 62.9%/39.9%/23.5%)
- **Hybrid (trained + PyCantonese POS):** POS 95%, DEP 80%

### 2026-03-23: retokenize retrace bugs found and fixed
- **Script:** `batchalign3/batchalign/tests/pipelines/morphosyntax/test_retokenize_retrace_e2e.py`
- **Bugs:** morphotag --retokenize failed on utterances with retraces in MOST corpus
- **Root cause:** `rebuild_content` recursed into retrace AnnotatedGroup/AnnotatedWord, desyncing word counter
- **Fixed:** Bugs #1-#4 fixed in commit `3c03fe3b`. MOST corpus (166K utterances) now processes successfully.

### 2026-03-23: String hacking audit
- **Doc:** `docs/investigations/2026-03-23-string-hacking-audit.md`
- **Found:** parenthesis stripping in morphosyntax.py:268 could cause text mismatch
- **Fixed:** _segment_cantonese "".join bug
