# SBCSAE Overlap Investigation

**Status:** Draft
**Last updated:** 2026-03-19

## Background

The Santa Barbara Corpus of Spoken American English (SBCSAE) is the single
largest source of overlap validation errors in our data. The SBCSAE files in
`ca-data/SBCSAE/` were converted from the original `.trn` transcription format
to CHAT using a custom parser. The original TRN format uses spatial (column)
alignment to encode overlap correspondence — information that was lost in
conversion to CHAT's `⌈⌊` marker system.

## The Problem

Multi-party conversations (up to 8 participants) produce tangled overlaps where
multiple speakers overlap simultaneously. Without numeric indices (⌈2, ⌈3, etc.),
the CHAT representation is inherently ambiguous — the machine parser cannot
determine which top corresponds to which bottom. The original TRN format may
encode this unambiguously via column positions.

## Current Status

SBCSAE overlap errors are suppressed (E347 for unindexed markers is not reported,
per `docs/overlap-validation-audit.md`). This is correct for now — the data is
not wrong, just under-specified.

## Source Data

- **Original TRN files:** Available from UCSB at
  https://linguistics.ucsb.edu/research/santa-barbara-corpus-spoken-american-english
- **Third-party preprocessing:** https://github.com/vectominist/SBCSAE-preprocess
  has a `download.sh` that may still work
- **Chen's parser:** Chen wrote a parser to convert TRN → CHAT. Location TBD.

## Plan (deferred)

1. Obtain original TRN files (download from UCSB or locate Chen's copies)
2. Examine TRN overlap encoding — does it have unambiguous correspondence info?
3. If yes, update the TRN → CHAT converter to emit indexed markers (⌈2, ⌈3, etc.)
   for multi-party overlaps
4. Re-convert SBCSAE, replacing current `ca-data/SBCSAE/` files
5. Re-run validation — indexed markers will be properly matched by E347
6. Revisit E347 suppression decision for unindexed markers
