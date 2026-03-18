# CA Overlap Marker Experiment: Provenance

**Status:** Current
**Last updated:** 2026-03-17

## Reproduction

All experiment data can be reproduced from the sources listed below.
Audio files are on `net:/Volumes/Other/ca/`. CHAT files are in
`data/ca-data/` (git.talkbank.org).

### Software Versions

- talkbank-tools: current working tree (includes overlap.rs region-based API)
- batchalign3: current working tree (includes CA-aware UTR windowing)
- utr-experiment: built from `analysis/per-speaker-utr-experiment-2026-03-16/`
- batchalign3 server: local daemon on port 8001

### Experiment Steps

```bash
# 1. Strip timing from ground truth
./target/release/utr-experiment strip <gt>.cha <input>.cha

# 2. Run alignment (needs symlink: audio next to .cha)
batchalign3 align <input>.cha -o <output-dir> --utr-strategy <global|two-pass>

# 3. Compare
./target/release/utr-experiment compare-timing --gt <gt-dir> <output-dir>
./target/release/utr-experiment onset-accuracy <dir>
./target/release/utr-experiment overlap-audit <dir>
```

## SBCSAE (10 files)

**Source:** `ca-data/SBCSAE/` (git.talkbank.org)
**Audio:** `net:/Volumes/Other/ca/SBCSAE/`
**Language:** English
**Selection criteria:** Files with highest onset accuracy measurement counts, spanning best to worst accuracy

| File | CHAT MD5 | Lines | Audio MD5 | Size |
|------|----------|-------|-----------|------|
| 06.cha | a4e8346a775a802d840a90a90048a1f8 | 1083 | 69c1dd2f4f7aae66b2e954908d972a5c | 12M |
| 08.cha | c74e8f50c2266d504bf135041cb40abe | 867 | 2ee93fb720b4f456afa6562d12c0aa2c | 12M |
| 09.cha | eeabde343ca3e6a5139ac11716fcac21 | 507 | 455b77e28360f8043d9af60f54db3722 | 11M |
| 13.cha | f9add518720500356eba51ecab0141d6 | 1869 | 08aab8b226eb6624f81929b21ebfe297 | 13M |
| 15.cha | c067873ef4c09ede600dcb4fb8d1dbc3 | 1350 | e76e3fa6abb8f5b347c501f2d8869fb9 | 12M |
| 28.cha | 8c49d657173fe6325f3f74848ab1e936 | 1024 | 8ed4af7fe63d9d7e1778bfa161509954 | 12M |
| 32.cha | a3af9b5cd6e93004ee5dd99ec24d9710 | 1322 | 456ebf5379c55efca614dd480acf69ef | 13M |
| 36.cha | 0d6b30f8c5f6a40e2c4942c1584daf53 | 1296 | 40838870fe6fec018c9e5c41caf46639 | 12M |
| 38.cha | 4d09a1805bca59ad191b2faded816a66 | 374 | ecbe1b673f2d2bd6220318dba98ca9dc | 11M |
| 49.cha | 76f864494e572ab14ced52603b4c192b | 1004 | 04d32c46a980cc00a65a8e6b15f87af4 | 8.7M |

## Jefferson NB (5 files)

**Source:** `ca-data/Jefferson/NB/` (git.talkbank.org)
**Audio:** `net:/Volumes/Other/ca/Jefferson/NB/`
**Language:** English
**Selection criteria:** Files with most cross-speaker overlap pairs (heavy intra-word markers)

| File | CHAT MD5 | Lines | Audio MD5 | Size | Cross-pairs |
|------|----------|-------|-----------|------|-------------|
| 08assassination2.cha | d79f0683db857f1ace0679431ca99602 | 717 | cd43fecb8c378bca364d4349c10abde6 | 8.3M | 201 |
| 10blinddate.cha | b5a27e8d13686a1a65eeb9287bf446cf | 692 | 22ecf2c0a6f4e7ba15c0afc8fccf553f | 8.1M | 206 |
| 21swimnude.cha | f301f603df4a2ab66417861a4618dcb3 | 1455 | c1e9078cdf27e123e1744f238732ad1f | 41M | 407 |
| 24meatless.cha | be2b39b4dd466d648658ba67bc5f01f5 | 557 | fe6f0e3819247a86f5556b1f8bbd5e09 | 7.6M | 143 |
| 25powertools.cha | f80d1a697b4276b51c828474c9eb2083 | 444 | 88458fc2fca04bb603785061af0b9bc1 | 4.0M | 162 |

## TaiwanHakka/Conversation (5 files)

**Source:** `ca-data/TaiwanHakka/Conversation/` (git.talkbank.org)
**Audio:** `net:/Volumes/Other/ca/TaiwanHakka/Conversation/`
**Language:** Hakka (hak)
**Note:** `@Media` headers say `unlinked` but audio files exist on server
**Selection criteria:** Files with most cross-speaker overlap pairs

| File | CHAT MD5 | Lines | Audio MD5 | Size | Cross-pairs |
|------|----------|-------|-----------|------|-------------|
| 01.cha | 221701c62bb3ee67df80724bd9ba4a52 | 539 | e300cdacb561d9e7752915374bb06b05 | 20M | 184 |
| 02.cha | 18e23f91f5045e7880cab9cea44a3fbb | 735 | f292e556bc6416a95ba5c92dce83d26e | 47M | 292 |
| 03.cha | 55af7ac68847fefef19a6c655a3d9061 | 694 | a9a6b96257abca3ef9de388e861a16c0 | 18M | 155 |
| 10.cha | 1f67111d6c218c816af9a2fa70fe9ea3 | 716 | dae4aeed942c0e0f433f577e7e99a6b4 | 20M | 245 |
| 12.cha | 032e325fed516488c8dce5ebfc849a14 | 412 | e394d1b5d5f82db122254ce401bd5d57 | 19M | 119 |

## Timing Coverage in Ground Truth

| Corpus | Timed utts | Total utts | Coverage |
|--------|-----------|------------|----------|
| SBCSAE | 7,074 | 10,540 | 67.1% |
| Jefferson NB | 270 | 2,561 | 10.5% |
| TaiwanHakka | 0 | 2,961 | 0% |

Jefferson NB has minimal timing (only some files have sparse bullets).
TaiwanHakka has no timing at all — all timing will come from ASR.

## Alignment Blockers (2026-03-17)

**Jefferson NB:** All 5 files use pure CA terminators (⇘ ⇗ → etc.) instead
of CHAT terminators (. ? !). batchalign3 `align` pre-validation rejects them
with "Utterance has no terminator". These files are valid CHAT per the chatter
validator — the batchalign pre-validation is stricter than necessary. **Cannot
be aligned without either fixing the pre-validation or adding terminators.**

**TaiwanHakka:** Most utterances have `.` terminators but 1-2 per file are
missing them. batchalign3 rejects the entire file over these few lines. Same
pre-validation strictness issue. **Could be fixed by adding missing terminators
to the 1-2 affected lines, or by relaxing batchalign pre-validation.**

This is tracked as a separate issue: batchalign3's align pre-validation should
be lenient enough to accept files that chatter validates successfully.
