# CA Overlap Markers: Remaining Experiments and Blockers

**Status:** Current
**Last updated:** 2026-03-17

## Context

The CA overlap marker experiment delivered solid results on SBCSAE (English,
well-formatted CHAT). Expanding to Jefferson NB and TaiwanHakka exposed
infrastructure gaps that block further experiments. This document catalogs
what we wanted to test, why it's blocked, and what needs fixing first.

## Experiments On Hold

### Experiment A-2: Jefferson NB (English, Heavy Intra-Word Overlaps)

**Goal:** Test CA marker-aware alignment on files with dense intra-word
overlap markers (`butt⌈er⌉`, `a⌈nd`, `fens⌈ter⌉`). Jefferson NB has the
highest intra-word marker density in ca-data (~535 markers in swimnude alone).

**Files selected:** 08assassination2, 10blinddate, 21swimnude, 24meatless,
25powertools (5 files, 1,119 cross-speaker pairs total).

**Audio:** Downloaded from `net:/Volumes/Other/ca/Jefferson/NB/`. On disk at
`data/audio-jefferson/`.

**Blocker:** All 5 files use pure CA terminators (⇘ ⇗ → etc.) instead of
CHAT terminators (. ? !). Zero standard terminators across 2,561 utterances.
batchalign3's pre-validation (L1 StructurallyComplete) rejects them with
"Utterance has no terminator."

**What needs fixing:** The aligner needs CA-awareness. When `@Options: CA` is
set, the terminator check should accept CA terminators or skip the check. See
"Blocker 1" below.

### Experiment A-3: TaiwanHakka (Hakka, Non-English, Dense Overlaps)

**Goal:** Test CA marker-aware alignment on non-English data. TaiwanHakka has
the second-highest overlap density in ca-data (292 cross-speaker pairs in file
02) and would directly test whether CA markers help with the non-English ASR
regression that motivated this work.

**Files selected:** 01, 02, 03, 10, 12 (5 files, 995 cross-speaker pairs).

**Audio:** Downloaded from `net:/Volumes/Other/ca/TaiwanHakka/Conversation/`.
On disk at `data/audio-taiwanhakka/`. Note: `@Media` headers say `unlinked`
but audio files exist on server.

**Blocker:** batchalign3's pre-validation rejects the files with "Utterance by
*F2 has no terminator" even though chatter validates them as valid CHAT. The
files have `.` terminators on 99%+ of utterances. The discrepancy appears to
be a parser difference between how the batchalign server reads the file vs how
chatter reads it. See "Blocker 2" below.

**Additional note:** TaiwanHakka files have zero existing timing — all timing
would come from ASR. This makes them a pure test of alignment from scratch,
which is a different scenario from SBCSAE (which had 67% existing timing).

### Experiment B-2: Full Corpus Overlap Database

**Goal:** Build a persistent, queryable database of all overlap markers across
all corpora. Record pairing quality, cross-speaker matching, 1:N patterns,
index usage, anomalies. Eliminate the need to re-scan the corpus for every
analysis.

**Blocker:** Needs a proper `chatter overlap-audit` command in talkbank-tools
(not the ad-hoc experiment tool). See "Blocker 4" below.

### Experiment C: Marker-Constrained FA Grouping

**Goal:** Use overlap boundaries as FA group anchors. When overlap markers
provide onset estimates, inject synthetic time bullets at overlap boundaries
as stable anchor points for `estimate_untimed_boundaries`.

**Depends on:** Experiments A-2 and A-3 (need working alignment on CA files).

### Experiment D: %wor from Overlap Structure

**Goal:** Generate approximate %wor tiers for CA files without running FA.
For fully-paired files, compute overlap spans from marker positions and
distribute words proportionally.

**Depends on:** Proper 1:N top↔bottom pairing (Blocker 3).

## Blockers (Fix These First)

### Blocker 1: Aligner Pre-Validation Rejects CA Terminators

**File:** `batchalign3/crates/batchalign-chat-ops/src/validate.rs` line 105

**Problem:** `check_structurally_complete` requires `terminator.is_some()` on
every utterance. CA files using ⇘ ⇗ → etc. as terminators parse successfully
(the parser recognizes CA terminators), but some files use terminators that
the parser represents differently.

**Fix:** When the file has `@Options: CA`, either:
- Skip the terminator check entirely (CA files are valid without . ? !)
- Accept any terminator type (CA or standard)

The `@Options: CA` flag is already parsed and available on `ChatFile.options`.

**Impact:** Unblocks Jefferson NB (5 files, 1,119 overlap pairs).

### Blocker 2: Parser Discrepancy Between Chatter and Batchalign Server

**Problem:** TaiwanHakka files validate as valid CHAT when parsed by chatter
(`chatter validate` reports zero errors), but batchalign3's server rejects
them with "no terminator" errors. Both use the same `parse_chat_file_streaming`
function.

**Hypothesis:** The batchalign server may be reading the file with different
encoding, or the daemon's file handling differs from the CLI. The files contain
CJK characters and mixed scripts.

**Investigation needed:** Run batchalign3 in debug mode to see the actual
parsed AST that the pre-validation rejects. Compare with chatter's parse.

**Impact:** Unblocks TaiwanHakka (5 files, 995 overlap pairs).

### Blocker 3: 1:N Top↔Bottom Overlap Pairing

**Problem:** Current `extract_overlap_info` and E347 validation assume 1:1
pairing: one ⌈ matches one ⌊. But in real CA data, one speaker's ⌈ region
can have multiple respondents with ⌊ regions (1:N).

**Example:**
```
*A: I was ⌈ saying that ⌉ .
*B:       ⌊ yeah ⌋ .
*C:       ⌊ right ⌋ .
```

Both B and C overlap with A's ⌈ region. The current code would match A↔B and
leave C as "unmatched" (E347 warning).

**Fix:** Redesign the cross-utterance matching to allow 1:N. A top region is
"matched" if there is at least one bottom region from a different speaker.
Multiple bottom regions for the same top are valid, not errors.

**Where:** `talkbank-model/validation/cross_utterance/mod.rs`
(`check_cross_utterance_overlap_balance`) and the `OverlapRegion` pairing in
`alignment/helpers/overlap.rs`.

**Impact:** Affects E347 validation accuracy and onset estimation for multi-party
conversations.

### Blocker 4: No Persistent Overlap Analysis Tool

**Problem:** Every time we need overlap data, we re-scan the entire corpus
with ad-hoc scripts. The experiment tool's `overlap-audit` and `onset-accuracy`
commands are one-off analysis tools, not proper infrastructure.

**Fix:** Build a `chatter overlap-audit` command in talkbank-tools that:
1. Parses files with the real parser (not grep)
2. Uses `extract_overlap_info` / `for_each_overlap_point` for all analysis
3. Cross-matches top↔bottom across utterances (with 1:N support)
4. Outputs structured results (JSON lines or TSV)
5. Can be filtered by corpus, language, pairing quality, etc.

**Where:** `talkbank-tools/crates/talkbank-clan/` (alongside other analysis
commands like FREQ, MLU).

**Impact:** Eliminates redundant corpus scanning, provides authoritative
overlap statistics.

### Blocker 5: Serialization Roundtrip for CA Whitespace

**Problem:** The CHAT serializer collapses multi-line continuation lines and
strips leading whitespace used for visual overlap alignment in CA transcription:

```
Original:  *F3:         ⌊ 係啊 ⌋ (.)
           	這 xxx .
Roundtrip: *F3:	⌊ 係啊 ⌋ (.) 這 xxx .
```

Semantically equivalent but loses the visual alignment that CA transcribers
use to show overlap positioning.

**Fix:** Preserve continuation-line whitespace in the serializer. This is a
general roundtrip fidelity issue, not specific to overlaps.

**Impact:** Required for reliable strip→align→compare experiments on CA files.

## Priority Order

1. **Blocker 1** (CA pre-validation) — smallest fix, unblocks Jefferson NB
2. **Blocker 2** (parser discrepancy) — investigate, may be same root cause as 1
3. **Blocker 3** (1:N pairing) — correctness fix for validation and analysis
4. **Blocker 5** (roundtrip whitespace) — roundtrip fidelity
5. **Blocker 4** (overlap-audit command) — infrastructure, depends on 3

After blockers 1-2 are fixed: rerun Experiments A-2 and A-3.
After blocker 3: rerun overlap audit with correct 1:N pairing.
After blocker 4: build persistent database and retire experiment scripts.
