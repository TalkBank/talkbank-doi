# SBCSAE Overlap Investigation

**Status:** Current
**Last updated:** 2026-03-19

## Background

The Santa Barbara Corpus of Spoken American English (SBCSAE) is the single
largest source of overlap validation errors in our data. The SBCSAE files in
`ca-data/SBCSAE/` were converted from the original `.trn` transcription format
to CHAT by Chen's Java DT parser (written before October 2011, committed in
`java-chatter-stable` as `DT.flex` + `DTParser.g` + `OverlapInfo`/`OverlapRun`/
`OverlapSet`).

**The original conversion did produce indexed overlap markers** (`⌈2`, `⌊3`, etc.).
The full chain — DT.flex tokenizing `[2` → DTParser.g inferring top/bottom →
ChatWalker.g passing `index` → Chat.stg emitting `$overlapPointMap.(all)$$index$`
— is verified in the code. However, **the indices were stripped at some unknown
point between ~2011 and 2022**, before the ca-data git repo was initialized
("Init for GitLab", 2022-03-23). The 2022 initial commit already contains only
unindexed `⌈`/`⌊` markers. Every subsequent commit (15 total through 2026-02-21)
was also checked — zero indexed markers anywhere in the history.

Someone (likely Brian, during manual editing over the intervening decade) removed
all the overlap indices from the CHAT files. The original TRN format uses numbered
brackets (`[2...2]`, `[3...3]`, up to `[9...9]`) that unambiguously encode which
speakers overlap with whom.

## The Problem

Multi-party conversations (up to 8 participants) produce tangled overlaps where
multiple speakers overlap simultaneously. Without numeric indices (⌈2, ⌈3, etc.),
the CHAT representation is inherently ambiguous — the machine parser cannot
determine which top corresponds to which bottom.

## Overlap Group Sequencing

The indices are not just disambiguation tags — they encode **sequential overlap
group layering**. Unnumbered `[...]` is the first overlap group; `[2...2]` opens
a second group while the first is still active; `[3...3]` a third, etc. This
encodes temporal nesting and chaining. Without indices, we lose the ability to
determine which groups are concurrent vs. sequential, and how they nest.

The samtale-data corpus (Danish CA) proves this works in CHAT. E.g.,
`Sam4/moedregruppen1.cha` has 100 indexed markers with indices up to 5 and deep
nesting (overlap groups contained within other groups).

### Indexed overlap markers across TalkBank (survey 2026-03-19)

Only 13 files outside SBCSAE use indexed overlaps:
- **ca-data/CLAPI/** — 4 French CA files (indices 2–3, light usage)
- **samtale-data/** — 9 Danish CA files (indices 2–5, heavy usage with nesting)
- **All other data repos** — zero indexed markers

Once restored, SBCSAE (~22,000 TRN overlap brackets across 60 files) will be the
largest indexed overlap corpus in TalkBank.

## Current Status

**E348 suppressed (2026-03-19):** Cross-utterance overlap spans are now recognized
as legitimate. When a `⌈` appears without a matching `⌉` on the same utterance
(or vice versa), the within-utterance check (E348) defers to the cross-utterance
check (E347), which pairs markers across utterances from the same speaker. This
eliminated 2,152 false positives on the hand-edited SBCSAE CHAT and 2,139 on the
converter-generated CHAT. See `talkbank-tools` commit `ff9e41e`.

**E347 for unindexed markers:** Suppressed per `docs/overlap-validation-audit.md`.
The data is not wrong, just under-specified.

## Source Data

- **Original TRN files:** Downloaded 2026-03-19 from UCSB
  (https://linguistics.ucsb.edu/research/santa-barbara-corpus-spoken-american-english).
  60 files archived at `~/sbcsae-trn/`
- **Third-party preprocessing:** https://github.com/vectominist/SBCSAE-preprocess
- **Legacy Java parser:** `java-chatter-stable/src/main/java/org/talkbank/dt/` — the
  original TRN→CHAT converter (DT.flex + DTParser.g + OverlapInfo/OverlapRun/OverlapSet),
  written before October 2011. **Did** emit indexed overlap markers (`⌈2`, `⌊3`, etc.) —
  verified via ChatWalker.g + Chat.stg template chain.
- **Format analysis:** `scripts/sbcsae-converter/trn-format-analysis.md` — comprehensive
  empirical survey of all 60 files, documenting conventions and violations

## Key Findings (2026-03-19)

1. The TRN format's numbered bracket indices (`[2...2]`, `[3...3]`, up to `[9...9]`) **do**
   encode overlap correspondence unambiguously.
2. Chen's Java DT parser (pre-2011) **did** correctly convert these to indexed CHAT markers
   (`⌈2`, `⌊3`, etc.) — the full pipeline is verified in the code.
3. The indices were **stripped** at some point between ~2011 and the 2022 GitLab init,
   presumably during manual editing. No indexed markers exist anywhere in the git history.
4. The original TRN files contain many data quality issues (mismatched brackets,
   index gaps, encoding corruption, format inconsistencies) that required manual fixes.

The hand-edited CHAT files are the gold standard for **content**; the TRN files are the
gold standard for **overlap correspondence**. The goal is to merge the two.

## Future: Overlap Group Validation

The sequential/nesting semantics of overlap indices suggest new validation rules:
- Non-sequential index (e.g., `⌈3` without `⌈2` in the current run)
- Nesting violation (inner group closes after outer group)
- Index reuse across disjoint overlap runs

These should be **warnings, not errors** — the original TRN data has gaps and
transcriber mistakes, and hand-edited CHAT will too. Deferred until after the
indices are restored and we can assess the real-world violation rate on the full
SBCSAE corpus plus the 13 existing indexed files (CLAPI + samtale).

Our existing mechanical validation (E347 matching `⌈2` with `⌊2`, E348 pairing)
is correct and sufficient for now. The higher-level structural validation can
build on top of it later.

## Impact on Prior Experiment Results

The two-pass UTR and overlap alignment experiments (2026-03-17, documented in
`analysis/per-speaker-utr-experiment-2026-03-16/`) ran against SBCSAE files with
**unindexed markers only**. The code is index-aware and correct — it falls back
to the unindexed "inherently ambiguous" path when indices are absent. The results
are not wrong, but they could not leverage the overlap correspondence information
that should have been present. Once indices are restored, the two-pass UTR will
be able to narrow recovery windows more precisely by matching indexed top/bottom
pairs. The experiments should be **re-run** after the merge to assess whether
indexed markers improve timing quality.

## Output Format Decision (open)

The Rust TRN parser needs to produce something that can be merged with the
existing hand-edited CHAT. Two options under consideration:

**Option A: Emit CHAT text.** Produce `.cha` files directly. Pro: diffable against
existing files, human-readable. Con: the TRN→CHAT mapping involves many content
transformations (glottals, vocalisms, pauses, speaker IDs, headers, etc.) that the
Java parser handled via StringTemplate — reimplementing all of this is substantial
work, and differences from the hand-edited CHAT will mix content divergences with
the overlap information we actually care about.

**Option B: Emit structured overlap data only.** Produce a per-file overlap
correspondence table (JSON or similar): for each overlap run, the sequential
indices, top/bottom speakers, timing, and the bracketed text. Pro: focused on
exactly the information we need to merge (the indices), avoids reimplementing the
full TRN→CHAT content pipeline. Con: merging back into CHAT requires a separate
alignment step to match TRN overlap regions to CHAT utterances.

**Option C: Emit CHAT AST (talkbank-model types).** Parse TRN into the Rust model
types directly, then serialize. Pro: leverages the existing serializer, round-trip
tested. Con: the TRN content conventions differ enough from CHAT that the model
types may not map cleanly (e.g., TRN vocalisms, long features, nonvocals all need
translation), and we'd be coupling the converter to talkbank-model internals.

Decision deferred — depends on how much content fidelity we need beyond overlaps.
If the merge strategy only needs to patch indices onto existing CHAT files, Option B
is likely sufficient and much simpler. If we also want a reference "what the original
conversion should have produced" CHAT, we need Option A or C.

## Plan

1. ~~Obtain original TRN files~~ ✓ Downloaded (60 files in `~/sbcsae-trn/`)
2. ~~Examine TRN overlap encoding~~ ✓ Numbered indices are unambiguous
3. ~~Study legacy Java parser~~ ✓ Documented in format analysis
4. ~~Assess impact on existing tooling~~ ✓ Code is index-aware, no changes needed
5. ~~Survey indexed overlaps across TalkBank~~ ✓ 13 files (CLAPI + samtale)
6. Decide output format (Option A/B/C above)
7. Build new Rust TRN parser (`scripts/sbcsae-converter/`) that:
   a. Parses all 60 files with error tolerance (no crashes)
   b. Extracts overlap correspondence with sequential group semantics
   c. Produces chosen output format
   d. Reports all data quality issues as diagnostics
8. Align TRN overlap data to existing CHAT utterances (by timing + speaker + content)
9. Merge: patch existing hand-edited CHAT files with recovered overlap indices
10. Re-run two-pass UTR experiments with indexed markers
11. Re-run validation — indexed markers will be properly matched by E347
12. Revisit E347 suppression decision for unindexed markers
13. (Later) Add overlap group sequencing validation (warnings for non-sequential
    indices, nesting violations)
