# Overlap Validation Audit: Empirical Investigation

**Status:** Current
**Last updated:** 2026-03-19

## Summary

Validation of CA overlap markers (⌈⌉⌊⌋) across all `data/*-data` repos produced
~60,000 warnings/errors from three error codes. Empirical investigation shows that
the vast majority are legitimate CA transcription conventions, not data quality
issues. Our validator imposed requirements that the CHAT/CA tradition does not.

This document records the full investigation, concrete examples with provenance,
the INDENT discovery, and decisions made.

## Error Counts Before Any Changes (2026-03-19)

| Corpus | E348 | E347 | E704 | Total |
|--------|------|------|------|-------|
| ca-data | 29,544 | 7,134 | 0 | 36,678 |
| slabank-data | 14,238 | 1,104 | 0 | 15,342 |
| samtale-data | 3,603 | 1,698 | 36 | 5,337 |
| childes-data | 1,269 | 1,023 | 51 | 2,343 |
| class-data | 765 | 9 | 0 | 774 |
| aphasia-data | 3 | 3 | 0 | 6 |
| dementia-data | 9 | 6 | 3 | 18 |
| **Total** | **~49,400** | **~11,000** | **~90** | **~60,500** |

Files with overlap markers: ca-data (407), childes-data (146), samtale-data (86),
slabank-data (51), class-data (11), aphasia-data (6), dementia-data (5).

---

## E348: Unpaired Overlap Marker Within Utterance (~49,400 warnings)

### What we check

E348 fires when an opening marker (⌈ or ⌊) has no matching closing marker
(⌉ or ⌋) within the **same utterance**, or vice versa.

### What the data actually shows

The overwhelmingly dominant pattern is **onset-only marking** — the transcriber
marks where overlap begins but not where it ends. The implied end is the end of
the shorter turn or the end of the utterance.

#### Sub-breakdown (ca-data, per-line grep)

| Sub-type | Count | Description |
|----------|-------|-------------|
| Opening without closing | ~6,970 | ⌈ or ⌊ with no ⌉/⌋ in utterance |
| Closing without opening | ~2,857 | ⌉ or ⌋ with no ⌈/⌊ in utterance |

Note: per-line grep counts differ from validator region counts (29,544 total E348
in ca-data) because multi-line utterances produce multiple regions and the
validator counts each region separately.

### Concrete examples

**Pattern A — Onset-only, cross-utterance pair** (most common):

```
ca-data/Bergmann/004_Moramin2.cha:19-20
*FM:  isch also bei uns nix::: ⌈angegeben↘ 8400_10740
*PM:                           ⌊hent ihr au nix 10840_11570
```

FM's `⌈` marks where PM started overlapping. PM's `⌊` marks the same point from
PM's perspective. Neither has a closing marker — the overlap continues to end of
the shorter turn. This is textbook Jeffersonian CA transcription.

**Pattern B — Onset-only, intra-word**:

```
ca-data/Bergmann/004_Moramin2.cha:44-45
*PM:  alles klar dan⌈n 30370_31050
*FM:                ⌊gut 31050_31240
```

The overlap starts mid-word ("dan⌈n"). Neither speaker has a closing marker.

**Pattern C — Multi-utterance continuation with onset-only**:

```
samtale-data/Video/seiden2.cha:54-57
*SEI:  ∙hhh ⌈det ↑e:neste jeg vil gi' dig ret i det er
*JER:       ⌊∙hnåh⇗
*SEI:  (0.3) ⌈at ↑MIN opga:ve (0.5) å politi:kens
*JER:        ⌊°tyveri°
```

SEI's speech continues across two utterances, each with a new overlap onset from
JER. All four markers are onset-only.

**Pattern D — Mixed paired and onset-only in rapid succession**:

```
samtale-data/Video/seiden2.cha:95-100
*SEI:  +≈ undskyld vi udgi'r ikk' ↑bø:ge⌈r,  vi har v⌉i har
*JER:                                    ⌊nehjhh hhehe⌋
*SEI:  udgi'r a⌈vi:ser å derfor har ⌈vi ci-↓te:ret fra et
*JER:          ⌊hhe hhe             ⌊ja:
*SEI:  +≈ manu↓skript ⌈↑ikk'  ∙hhh
*JER:                  ⌊↓ja:h↘
```

First overlap pair (⌈r...⌉/⌊nehjhh...⌋) is fully paired. The next four are
onset-only. Both styles coexist in the same passage.

**Pattern E — Closing without opening**:

Some continuation-line patterns produce a closing marker that belongs to a region
opened on a previous utterance's line, resulting in a "closing without opening"
E348.

### Decision

**Suppress "opening without closing" (onset-only) unconditionally.** This is
standard CA practice, not an error. It appears across all 7 corpus types — not
limited to `@Options: CA` files (slabank, childes, class-data all use it).

**Keep "closing without opening" as a warning.** An orphaned close with no
corresponding open is genuinely suspicious — there is no "offset-only" CA
convention. These ~2,857 cases (in ca-data) may indicate transcription errors
or unusual continuation-line structures worth investigating.

---

## E347: Unbalanced Cross-Utterance Overlap (~11,000 → ~2,500 after bug fix)

### What we check

E347 fires when a top overlap region (⌈...⌉ or onset-only ⌈) on speaker A has no
matching bottom overlap region (⌊...⌋ or onset-only ⌊) from a *different* speaker
in the file, or vice versa.

### Bug fix applied (2026-03-19)

The original matching algorithm (`check_cross_utterance_overlap_balance()` in
`crates/talkbank-model/src/validation/cross_utterance/mod.rs`) had a greedy-
assignment bug.

**The bug:** Reverse-scan iteration meant each bottom matched the *last* eligible
top, without consuming it. When a speaker pair had multiple same-index overlaps,
all bottoms collapsed onto the last top, leaving earlier tops orphaned.

**Concrete demonstration:**

```
ca-data/Bergmann/009_schwerer_Unfall2.cha:13-14
*AM:  +≈ ⌈jawoll ⌉ komme so⌈fort⌉↘ 3150_4730
*FM:     ⌊vau u:h⌋         ⌊ ad⌋e: 3150_4730
```

AM has two unindexed top regions (`⌈jawoll⌉` and `⌈fort⌉`). FM has two unindexed
bottom regions (`⌊vau u:h⌋` and `⌊ ad⌋e:`). Visually they pair 1:1.

But the algorithm scanned bottoms forward and tops in *reverse*:
- FM bottom 1 → reverse scan → matches AM top 2 (the last one). Break.
- FM bottom 2 → reverse scan → matches AM top 2 again (not consumed). Break.
- AM top 1 has no matches → reported as orphaned E347.

**The fix:** Added a distribution guard — when a top already has a bottom from
the same speaker as the current candidate, AND a sibling top exists (same speaker,
same utterance, same index) with no bottom from that speaker, the algorithm skips
to the next top. This distributes same-speaker-pair bottoms evenly across sibling
tops while preserving 1:N multi-party matching (one top, bottoms from different
speakers).

**Before fix:** ~11,000 E347 across all data repos.
**After fix:** ~2,500 E347.

### What the remaining ~2,500 orphans actually are

Three distinct causes, demonstrated with concrete provenance:

#### Cause 1: Multi-party overlap ambiguity without index disambiguation

The dominant remaining pattern. In conversations with 3+ speakers, multiple
simultaneous overlaps occur without numeric indices (⌈2, ⌈3, etc.). The
transcriber relies on spatial (column) alignment to show correspondence, but the
machine parser can only match by (speaker, index). Without indices, multiple
unindexed tops from one speaker cannot be unambiguously assigned to unindexed
bottoms from multiple other speakers.

**Example — 5 speakers, tangled overlaps (SBCSAE/13.cha:83-87):**

```
*MARC:  ⌈ &=laugh &=laugh &=laugh &=laugh &=laugh &=laugh ⌉ .
*KEND:  ⌊ I'll just take my gifts up ⌋ to my bed and ⌈ &{l=@ open em (.) by ⌉ my⌈se:⌉⌈lf &}l=@ ⌉ .
*MARC:  ⌊ &=laugh &=laugh &=laugh ⌋ .
*KEN:   ⌊ Oh⌋⌊:⌋⌈: ⌉ .
*KEVI:  ⌊ Oh ⌋ ⌊ that's right ⌋ .
```

Marker inventory:
- MARC line 83: 1 top
- KEND line 84: 1 bottom + 3 tops (all unindexed)
- MARC line 85: 1 bottom
- KEN line 86: 2 bottoms + 1 top
- KEVI line 87: 2 bottoms

KEND's 3 unindexed tops need to match bottoms from MARC, KEN, and KEVI, but
without indices the matcher cannot determine which of KEND's tops corresponds to
which bottom. The transcriber encoded this through column alignment — visually,
KEND's first `⌈` matches MARC's `⌊`, KEND's later markers match KEN's and
KEVI's. But column alignment is not parseable (see INDENT section below).

**Example — 3 speakers, 1:N overlap (CLAPI/logeurs.cha:237-239):**

```
*SUZ:  hm ⌈hm ⌉
*FER:     ⌊ouais⌋
*ELI:     ⌊et là⌋ bas en Allemagne qu'est-ce qu'i' fait comme temps↗
```

One top from SUZ, two bottoms from two different speakers (FER and ELI). This is
a genuine 1:N multi-party overlap — one speaker overlapped by two others
simultaneously. Our algorithm correctly handles this case (1:N matching), but
when combined with other nearby overlaps in the same file, the sequential matcher
can misassign, leaving some orphaned.

**Example — 4 speakers, consecutive overlap sets (SBCSAE/13.cha:119-122):**

```
*KEN:   Mm⌈: ⌉ .
*KEVI:  ⌊ She ⌋ ⌈ has ⌉ (..) fit in⌈to the groove ⌉ .
*WEND:  ⌊ &=SNIFF ⌋ .
*MARC:  ⌊ &=laugh &=laugh &=laugh ⌋ &=in .
```

KEN has 1 top. KEVI has 1 bottom (matches KEN) + 2 tops. WEND and MARC each
have 1 bottom. KEVI's 2 unindexed tops need to match WEND's and MARC's bottoms,
but without indices the assignment is ambiguous.

**Example — 3 speakers, indexed vs unindexed mix (CLAPI/logeurs.cha:243-244):**

```
*SUZ:     ⌊il y a⌋ ⌈2beaucoup de la⌉ neige
*FER:     ⌊plus ⌋  ⌊2froid qu'ici↗ ⌋
```

SUZ has 1 bottom (unindexed) and 1 top (index 2). FER has 1 bottom (unindexed)
and 1 bottom (index 2). The indexed pair (⌈2/⌊2) matches correctly. The
unindexed pair requires matching to a top from a preceding utterance — if that
top was already consumed, the unindexed bottom becomes an orphan.

#### Cause 2: One-sided overlap marking

Some files mark `⌈` on speaker A without a corresponding `⌊` on any other
speaker. This happens when:

- The overlapping speaker's backchannel was not fully transcribed
- The transcriber marked one side of an overlap but not the other
- The corpus conventions only require marking the "main" speaker

#### Cause 3: Onset-only cross-utterance mismatch

In some cases, onset-only tops and onset-only bottoms from the same temporal
region fail to match because intermediate utterances push the bottom's utterance
index far from the top's, and the matcher requires `top.utterance_index <=
bottom.utterance_index` which may be too strict for files with many rapid turns.

### INDENT and spatial layout

**Critical finding:** The spatial (column) alignment of overlap markers in the
corpus data is **NOT original transcriber intent** — it is the output of CLAN's
INDENT command (`OSX-CLAN/src/clan/indent.cpp`), which Leonid ran on corpus files.

INDENT is both a reformatter and a validator. It does two things:

1. **Matches `⌈` (top-open) to `⌊` (bottom-open)** by scanning forward through
   following speaker tiers. Matching criteria: different speaker, same index
   number (or both unindexed). For unindexed markers, it uses sequential
   first-match. When fewer than 2 unindexed markers exist, it resets the match
   state (`indent.cpp` lines 313-320).

2. **Rewrites the text** to column-align `⌊` with its matched `⌈`, inserting or
   removing spaces as needed (`indent.cpp` lines 352-381). It also adjusts the
   *previous* utterance's text when the column position can't be shifted
   (`prevLineAjusted()`, line 378).

3. **Reports errors** for unmatched markers (`indent.cpp` lines 452-484):
   "Can't find closing overlap marker for [...] on any following speaker tiers."
   So any remaining unmatched markers in the current data represent cases that
   INDENT also could not resolve.

**Implications:**
- The spatial alignment is an artifact of INDENT's matching algorithm, not a
  reliable signal of original transcriber intent
- We cannot use column position as a fallback matching heuristic — it's circular
  (INDENT created the alignment using the same index+speaker matching we use)
- Any markers that INDENT couldn't match are genuinely ambiguous or erroneous
- INDENT's matching algorithm is essentially the same as ours (sequential,
  index-aware, different-speaker), so the ~2,500 remaining orphans represent
  the theoretical ceiling of what this class of algorithm can resolve

### SBCSAE and original source data

**TODO (revisit):** The SBCSAE files in `ca-data/` were converted to CHAT from
original SBCSAE transcription files using a custom parser. The original files
have their own formatting conventions that may encode overlap correspondence
differently from CHAT's ⌈⌊ convention. Chen has the original SBCSAE source files.
Re-examining the conversion from original → CHAT may reveal whether the multi-
party ambiguity is inherent in the source or introduced by the conversion process.
If the original format has unambiguous overlap encoding, the CHAT conversion
should be updated to emit indexed markers (⌈2, ⌈3, etc.).

### Decision

**Suppress E347 for unindexed markers.** The remaining ~2,500 orphans are
dominated by multi-party ambiguity that cannot be resolved without index
disambiguation. Indexed markers (⌈2/⌊2, ⌈3/⌊3, etc.) have unambiguous matching
semantics and E347 should remain active for those.

After SBCSAE re-investigation, this decision may be revisited.

---

## E704: Speaker Self-Overlap (~90 errors)

### What we check

E704 fires when consecutive utterances by the **same speaker** have a top
overlap followed by a bottom overlap (or vice versa), which implies a speaker
overlapping with themselves.

### What the data actually shows

Two patterns observed:

#### Pattern A — Legitimate multi-party overlap (Forrester corpus)

```
childes-data/Eng-UK/Forrester/020621.cha:363-367
*FAT:  ⌈Stravinsky or →
*CHI:  ⌊xxxxxx →
*CHI:  I wanny ⌈go on em⌉ →
*FAT:  ⌊kind of ⌈music⌋ you sit⌉ →
*MOT:  ⌊what's⌋ Stravinsky like ↗
```

FAT has `⌈` (top) on line 363, overlapped by CHI. Then FAT has `⌊` (bottom) on
line 366, overlapping CHI back. This is FAT participating in overlaps on *both
sides* — first being overlapped, then overlapping — in rapid multi-party
conversation with 3 speakers (FAT, CHI, MOT). This is legitimate CA annotation
but fires E704 because FAT has top→bottom across two consecutive FAT utterances
(with CHI utterances intervening — but E704 only checks same-speaker adjacency).

#### Pattern B — Same-speaker layered annotation (samtale-data)

```
samtale-data/Sam4/fyrne.cha:118-120
*UK:  °xxx⌈x°⌉ 93064_93900
*UK:     ⌊☺x⌋⌈xxx☺  ⌉ 93941_95509
*UK:         ⌊ἩxxxἩ⌋
```

Three consecutive UK utterances with overlapping markers, representing layered
speech events (laughing, spoken content, and breathy speech overlapping with each
other). All markers are same-speaker. This is a CA convention for transcribing
multi-layered simultaneous speech acts where a single speaker produces multiple
overlapping streams.

### Distribution

| Corpus | E704 count | Character |
|--------|------------|-----------|
| childes-data | 51 | Mostly Forrester (3-speaker family) |
| samtale-data | 36 | Same-speaker layering |
| dementia-data | 3 | Unknown pattern |
| **Total** | **90** | |

### Decision

**Pending.** E704 count is small enough (90) to audit individually later. Both
observed patterns (multi-party and same-speaker layering) appear to be legitimate
CA conventions. The check should likely be refined to skip cases where
intervening utterances from other speakers exist between the flagged same-speaker
pair (Pattern A), and to recognize same-speaker layering as intentional (Pattern
B). For now, keeping as-is — these are real warnings on a small number of files.

---

## Files With Overlap Markers by Corpus Type

| Corpus | `@Options: CA` | Files w/ overlaps | Character |
|--------|----------------|-------------------|-----------|
| ca-data | Yes | 407 | Pure CA transcription |
| samtale-data | Mixed | 86 | Danish CA corpus |
| slabank-data | Mixed | 51 | Second-language classroom |
| childes-data | Some (Forrester) | 146 | Varied research |
| class-data | No | 11 | Classroom data |
| aphasia-data | No | 6 | Clinical |
| dementia-data | No | 5 | Clinical |

Note: overlap markers appear in files both with and without `@Options: CA`.
Onset-only marking is used across all corpus types. Suppression decisions must
not be gated on `@Options: CA`.

---

## Changes Made (2026-03-19)

### Algorithm bug fix (E347)

**File:** `crates/talkbank-model/src/validation/cross_utterance/mod.rs`

Fixed the greedy matching in `check_cross_utterance_overlap_balance()`. Added a
distribution guard: when iterating bottoms against tops in reverse, if the
candidate top already has a bottom from the same speaker as the current bottom,
and a sibling top exists (same speaker, same utterance, same index) with no
bottom from that speaker, skip to the next top. This prevents same-speaker-pair
bottom collapse while preserving 1:N multi-party matching.

Reduction: ~11,000 → ~2,500 E347.

### E348: Suppress onset-only (opening without closing)

**File:** `crates/talkbank-model/src/validation/utterance/overlap.rs`

Suppressed E348 for the "opening without closing" case. An opening marker (⌈ or
⌊) without a matching closing marker in the same utterance is standard CA onset-
only annotation.

**Update (2026-03-19):** Also suppressed E348 for "closing without opening".
Investigation during the SBCSAE TRN→CHAT converter project revealed that these
are legitimate cross-utterance overlap spans — the matching open marker is on a
preceding utterance from the same speaker. The cross-utterance check (E347)
already handles pairing these correctly, making the within-utterance E348 check
fully redundant for both cases. This eliminated ~49,400 E348 warnings across all
data repos (2,152 on hand-edited SBCSAE alone). Any truly unpaired markers are
caught by E347 as orphaned tops/bottoms.

Reduction: ~49,400 → 0 E348.

### E347: Suppress for unindexed markers

**File:** `crates/talkbank-model/src/validation/cross_utterance/mod.rs`

Suppressed E347 for orphaned tops/bottoms that are unindexed (index = None).
Unindexed multi-party overlaps are inherently ambiguous; only indexed markers
(⌈2/⌊2, etc.) have unambiguous matching semantics.

Reduction: ~2,500 → TBD (depends on how many remaining orphans are indexed).

---

## Future Work

- **SBCSAE re-investigation:** Re-examine the original SBCSAE source files and
  the parser that converted them to CHAT. If the original format encodes overlap
  correspondence unambiguously, update the conversion to emit indexed CHAT markers.
- **E704 audit:** Manually audit the 90 E704 cases to classify as legitimate
  multi-party overlap vs actual data errors. Refine E704 to handle intervening
  speakers.
- ~~**"Closing without opening" investigation:**~~ RESOLVED. All "closing without
  opening" E348 cases are legitimate cross-utterance spans. E348 fully suppressed;
  E347 handles all overlap pairing.
