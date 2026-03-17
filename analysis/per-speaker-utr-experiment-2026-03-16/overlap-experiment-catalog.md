# Overlap Experiment: Test File Catalog and Results

**Status:** Draft (updated as experiments run)
**Last updated:** 2026-03-17

## Test File Catalog

### Set 1: APROCSA — `&*` conversion test (aphasia-data)

These files originally use `&*` markers. We convert them to `+<` separate
utterances for the experiment.

| File | Corpus path | Utts | `&*` | Speakers | Lang | Audio | Why interesting |
|------|-------------|------|------|----------|------|-------|----------------|
| 2265_T4 | aphasia-data/English/Protocol/APROCSA/ | 636 | 47 | 4 (INV, PAR, REL1, REL2) | eng | 267MB mp4 | Documented worst case (36.5% timing loss in prior experiments), 4 speakers, Davida's hand-edited transcript |
| 2420_T3 | aphasia-data/English/Protocol/APROCSA/ | 912 | 74 | 3 (INV, PAR, REL) | eng | 691MB mp4 | Highest `&*` density among non-excluded APROCSA files, 3 speakers |
| 2432_T2 | aphasia-data/English/Protocol/APROCSA/ | 952 | 78 | 2 (INV, PAR) | eng | 764MB mp4 | High `&*`, 2 speakers, NOT on no-align list — control case |
| 2463_T2 | aphasia-data/English/Protocol/APROCSA/ | 1070 | 86 | 2 (INV, PAR) | eng | 710MB mp4 | Highest `&*` count, 2 speakers, on no-align list |

Audio location: `macw@net:/Volumes/Other/aphasia/English/Protocol/APROCSA/`

### Set 2: MacWhinney — ground-truth `+<` test (childes-data)

These files natively use `+<` with existing timing. We strip timing and
re-align, using originals as ground truth.

| File | Corpus path | Utts | `+<` | Timed | Speakers | Lang | Audio | Why interesting |
|------|-------------|------|------|-------|----------|------|-------|----------------|
| 060211a2 | childes-data/Eng-NA/MacWhinney/ | 938 | 159 | 100% | 4 (FAT, MAR, MOT, CHI) | eng | 25MB mp3 | Highest `+<` count in MacWhinney, full ground truth, Brian's own data |
| 060211b1 | childes-data/Eng-NA/MacWhinney/ | 865 | 105 | 100% | 3 (CHI, FAT, MAR) | eng | 26MB mp3 | Same session as 060211a2, high `+<` density |
| 060406b2 | childes-data/Eng-NA/MacWhinney/ | 714 | 91 | 100% | 4 (CHI, FAT, MAR, MOT) | eng | 25MB mp3 | Different session, similar density |

Audio location: `macw@net:/Volumes/CHILDES/CHILDES/Eng-NA/MacWhinney/`

### Set 3: Snow — ground-truth `+<` test (childes-data)

| File | Corpus path | Utts | `+<` | Timed | Speakers | Lang | Audio | Why interesting |
|------|-------------|------|------|-------|----------|------|-------|----------------|
| 020518b | childes-data/Eng-NA/Snow/ | 3347 | 131 | 69% | 5 (CHI, FAT, LIA, MOT, UNI) | eng | 74MB mp3 | Large file, partial ground truth, 5 speakers |

Audio location: `macw@net:/Volumes/CHILDES/CHILDES/Eng-NA/Snow/`

### Set 4: Bilingual — extreme `+<` density (biling-data) (PENDING)

These have the highest `+<` density in any corpus. Multi-language, very
conversational with constant overlap.

| File | Corpus path | Utts | `+<` | `+<` % | Timed | Lang | Audio | Why interesting |
|------|-------------|------|------|--------|-------|------|-------|----------------|
| MLE-MPF/28 | biling-data/MLE-MPF/ | 1371 | 855 | **62%** | 99% | fra, eng | (TBD) | Highest `+<` density in entire TalkBank. French-English bilingual conversation |
| MLE-MPF/22 | biling-data/MLE-MPF/ | 1743 | 684 | 39% | 99% | fra, eng | (TBD) | High density French-English |
| Bangor/Siarad/fusser12 | biling-data/Bangor/Siarad/ | 2190 | 736 | 34% | 100% | cym, eng | (TBD) | Welsh-English bilingual, very large file |
| Bangor/Patagonia/30 | biling-data/Bangor/Patagonia/ | 1760 | 690 | 39% | 100% | cym, eng, spa | (TBD) | Welsh-English-Spanish TRILINGUAL |
| MLE-MPF/31 | biling-data/MLE-MPF/ | 979 | 427 | 43% | 100% | fra, ara, eng | (TBD) | French-Arabic-English trilingual |

Audio location: TBD (need to find on net)

### Set 5: French PhonBank — extreme density (phon-data) (PENDING)

| File | Corpus path | Utts | `+<` | Timed | Lang | Audio | Why interesting |
|------|-------------|------|------|-------|------|-------|----------------|
| Paris/Antoine/010715 | phon-data/French/Paris/Antoine/ | ~1600 | 922 | yes | fra | (TBD) | Highest `+<` count in PhonBank. French child-parent interaction |
| Paris/Theophile/040620 | phon-data/French/Paris/Theophile/ | ~1400 | 734 | yes | fra | (TBD) | Another French Paris family |

Audio location: TBD (need to find on net)

### Set 6: Pure `&*` files — conversion candidates (ca-data, aphasia-data)

Files that use ONLY `&*` (no `+<`) — best candidates for the A→B→C→D
comparison. CORAAL is the standout: sociolinguistic interviews with massive
`&*` density.

| File | Corpus path | `&*` | Bullets | Lang | Domain | Why interesting |
|------|-------------|------|---------|------|--------|----------------|
| PRV/se0-ag2/m_02_1 | ca-data/CORAAL/PRV/ | **325** | 1630 | eng | Sociolinguistic interview | Highest pure-`&*` file in TalkBank. Dense backchannel during narrative. |
| PRV/se0-ag3/f_02_1 | ca-data/CORAAL/PRV/ | 303 | 1313 | eng | Sociolinguistic interview | |
| PRV/se0-ag3/f_02_2 | ca-data/CORAAL/PRV/ | 236 | 1027 | eng | Sociolinguistic interview | |
| DTA/se3-ag4/f_02 | ca-data/CORAAL/DTA/ | 185 | 745 | eng | Sociolinguistic interview | Durham, NC |
| UNH/Control/UNH1045 | aphasia-data/English/Protocol/UNH/ | 163 | 1014 | eng | Aphasia control | Non-APROCSA aphasia protocol |
| Penn/afrik | aphasia-data/English/NonProtocol/Penn/ | 157 | 1308 | eng | Aphasia (Penn) | Echo/completion pattern (`&*PAR:red_sports_model`) |
| Palaniyappan/.../066 | psychosis-data/English/Palaniyappan/ | 115 | 794 | eng | Psychosis | Different clinical domain |

Audio location: CORAAL at `macw@net:/Volumes/` (path TBD). Penn/UNH/psychosis at `/Volumes/Other/`.

### Set 7: Mixed encoding files — both `&*` AND `+<`

Files that use BOTH conventions. Interesting for testing whether the aligner
handles mixed input correctly.

| File | Corpus path | `&*` | `+<` | Lang | Domain | Why interesting |
|------|-------------|------|------|------|--------|----------------|
| DCB/se2-ag4/f_02_1 | ca-data/CORAAL/DCB/ | 179 | 9 | eng | Sociolinguistic | Mostly `&*` with a few `+<` |
| NEURAL-2/PWA/99-1 | aphasia-data/English/Protocol/ | 171 | 9 | eng | Aphasia | Clinical with both conventions |
| Lanzi/Treatment/11-14-17 | dementia-data/English/Lanzi/ | 165 | 75 | eng | Dementia | Richest mixed file: substantial use of both |
| NEURAL-2/PWA/171-1 | aphasia-data/English/Protocol/ | 134 | 11 | eng | Aphasia | |

### Set 8: Additional English clinical corpora (PENDING)

| File | Corpus path | Utts | `+<` | Timed | Lang | Domain | Why interesting |
|------|-------------|------|------|-------|------|--------|----------------|
| Lanzi/Treatment/11-14-17 | dementia-data/English/Lanzi/ | ~985 | 75 | yes | eng | Dementia | Dense overlap in clinical setting |
| Coelho/N/n22 | tbi-data/English/Coelho/N/ | ~613 | 156 | 85% | eng | TBI normal control | High `+<` in traumatic brain injury corpus |
| Coelho/TB/tb23 | tbi-data/English/Coelho/TB/ | ~711 | 151 | 87% | eng | TBI patient | Same corpus, patient data |

Audio location: TBD

---

## Experiment Results

### Experiment 1: APROCSA `&*` conversion (4 conditions)

**Date:** 2026-03-17
**Files:** 2265_T4, 2420_T3, 2432_T2, 2463_T2

| Condition | Description |
|-----------|------------|
| A — Original `&*` | Current production. Backchannels invisible, no timing. |
| B — No linker, global | `&*` → separate utterances without `+<`. The original failure mode. |
| C — With `+<`, global | `&*` → separate utterances with `+<`, but GlobalUtr ignores it. |
| D — With `+<`, two-pass | `&*` → separate utterances with `+<`, TwoPassOverlapUtr uses it. |

#### Coverage results (ALL speakers)

| File | Total utts | A | B | C | D |
|------|-----------|---|---|---|---|
| 2265_T4 | 636→683 | 636/636 (100%) | 682/683 (99.9%) | 682/683 (99.9%) | **683/683 (100%)** |
| 2420_T3 | 912→986 | 905/912 (99.2%) | 976/986 (99.0%) | 976/986 (99.0%) | **977/986 (99.1%)** |
| 2432_T2 | 952→1030 | 942/952 (98.9%) | 1013/1030 (98.3%) | 1013/1030 (98.3%) | 1013/1030 (98.3%) |
| 2463_T2 | 1070→1156 | 1065/1070 (99.5%) | 1149/1156 (99.4%) | 1149/1156 (99.4%) | 1149/1156 (99.4%) |

#### Key findings — coverage

1. **B = C exactly** — the `+<` linker makes zero coverage difference to
   GlobalUtr (expected: GlobalUtr doesn't read linkers).
2. **D recovers 1 extra utterance** on 2265_T4 and 2420_T3 vs B/C.
3. **Coverage is nearly identical across all 4 conditions** (98-100%).
   The global DP handles backchannel words in the reference surprisingly well
   on these files.
4. **Coverage is NOT the right metric.** The strategies differ in timing
   quality, not coverage.

#### Timing quality comparison (B global vs D two-pass, 2265_T4)

Of 47 `+<` backchannel utterances:
- **25 identical timing** (both strategies agree)
- **22 differ by >500ms** (up to 7 seconds apart)
- **1 utterance** timed only by D (unmatched in B)

Two-pass consistently places backchannels **earlier** in the audio — closer
to the predecessor utterance's time window where the overlap actually occurred.
Global DP tends to push backchannels **later** because "mhm" tokens match
ambiguously in the global reference sequence.

**Example (idx 43):** PAR says "but" during overlap.
- Global: 115825–116045ms (at predecessor's END)
- Two-pass: 108865–109085ms (WITHIN predecessor's range 115696–115836ms)

The two-pass timing is more plausible for an overlap — the backchannel should
start DURING the predecessor, not at its boundary.

### Experiment 2: CHILDES MacWhinney ground truth

**Date:** 2026-03-17
**Files:** 060211a2, 060211b1, 060406b2, 020518b

| Condition | Description |
|-----------|------------|
| Ground truth | Original hand-verified timing (reference) |
| Global | GlobalUtr on stripped file |
| Two-pass | TwoPassOverlapUtr on stripped file |

#### Coverage results (ALL speakers)

| File | Ground truth | Global | Two-pass |
|------|:---:|:---:|:---:|
| 060211a2 | 938/938 (100%) | 873/938 (93.1%) | 873/938 (93.1%) |
| 060211b1 | 865/865 (100%) | 831/865 (96.1%) | 830/865 (96.0%) |
| 060406b2 | 714/714 (100%) | 694/714 (97.2%) | 694/714 (97.2%) |
| 020518b | 2320/3347 (69.3%) | 3251/3347 (97.1%) | (failed — timeout on large file) |

#### Key findings — coverage

1. **Global and two-pass have identical or near-identical coverage.**
   060211b1 two-pass is 1 worse (830 vs 831).
2. **Neither strategy recovers 100%** — 3-7% of utterances lose timing
   during re-alignment, even without any overlap complication.
3. **020518b two-pass failed** — likely OOM or timeout on the 3347-utterance
   file. Global succeeded.
4. **Coverage is not the differentiator.** Need timing quality analysis.

### Experiment 3: CORAAL pure `&*` conversion (RUNNING)

**Date:** 2026-03-17
**Files:** PRV_m02_1 (325 `&*`), PRV_f02_1 (303 `&*`), DTA_f02 (185 `&*`)
**Design:** Same 4-condition as APROCSA (A/B/C/D)

Results pending.

### Experiment 4: Multilingual ground truth (QUEUED)

**Date:** 2026-03-17
**Files:** mle28 (fra-eng), fusser12 (cym-eng), patagonia30 (cym-eng-spa),
serbian030005 (srp), german050814 (deu), tbi_n22 (eng), tbi_tb23 (eng)
**Design:** Strip timing, re-align global vs two-pass, compare to ground truth.

Results pending.

---

## Audio Locations Reference

| Corpus | Net path pattern |
|--------|-----------------|
| APROCSA | `macw@net:/Volumes/Other/aphasia/English/Protocol/APROCSA/` |
| CHILDES MacWhinney | `macw@net:/Volumes/CHILDES/CHILDES/Eng-NA/MacWhinney/` |
| CHILDES Snow | `macw@net:/Volumes/CHILDES/CHILDES/Eng-NA/Snow/` |
| PhonBank French | `macw@net:/Volumes/PHONBANK/` (path TBD) |
| biling-data | `macw@net:/Volumes/` (path TBD) |
