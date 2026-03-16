# Per-Speaker UTR Simulation Experiment

**Status:** Draft
**Last updated:** 2026-03-16

## Goal

Determine whether per-speaker UTR alignment would substantially improve
timing coverage on overlap-heavy CHAT files, **before** building the full
implementation (~500-800 lines of Rust).

## Key Question

Within each speaker's word stream, does the word order match the audio
temporal order?  If yes, per-speaker UTR should recover most untimed
utterances.  If no, per-speaker UTR won't help and we need a different
approach.

## Method

For each test file:

1. **Split** the CHAT file into single-speaker files (one per participant),
   preserving the same audio reference.
2. **Run `align`** on each single-speaker file against the original audio.
3. **Count** timed vs. untimed utterances per speaker.
4. **Compare** the per-speaker total coverage to the global (all-speakers)
   coverage.

This simulates per-speaker UTR without building the full pipeline: each
single-speaker file gives the aligner only one speaker's words, eliminating
cross-speaker interleaving.

### What this simulation does NOT test

- Diarization quality (we use ground-truth speaker labels, not predicted ones).
- Per-speaker audio extraction (we use the full audio, not speaker-specific
  segments).  This is conservative — the aligner sees the full audio including
  other speakers' words as ASR noise.
- `&*` stripping (the single-speaker files still contain `&*` markers from
  the original, but since those belong to other speakers who aren't in the
  file, the aligner won't try to match them).

If per-speaker alignment improves coverage even without diarization-based
audio extraction, the real implementation (with per-speaker audio) should
do at least as well.

## Test Files

| File | Utts | `&*` | Speakers | On no-align list | Audio location (net) |
|------|------|------|----------|------------------|---------------------|
| 2265_T4 | 636 | 44 | 4 (INV, PAR, REL1, REL2) | Yes | `/Volumes/Other/aphasia/English/Protocol/APROCSA/2265_T4.mp4` |
| 2420_T3 | 912 | 72 | 3 (INV, PAR, REL1) | No | `/Volumes/Other/aphasia/English/Protocol/APROCSA/2420_T3.mp4` |
| 2463_T2 | 1070 | 82 | 2 (INV, PAR) | Yes | `/Volumes/Other/aphasia/English/Protocol/APROCSA/2463_T2.mp4` |
| 2432_T2 | 952 | 74 | 2 (INV, PAR) | No | `/Volumes/Other/aphasia/English/Protocol/APROCSA/2432_T2.mp4` |

### Why these files

- **2265_T4** — the documented worst case (36.5% timing loss), 4 speakers,
  Davida's hand-edited transcript.  Already has input/output in `~/davida/`.
- **2420_T3** — highest `&*` density among non-excluded APROCSA files,
  3 speakers.  Tests whether the problem exists in files that aren't on the
  no-align list.
- **2463_T2** — highest `&*` count, 2 speakers, on no-align list.  Tests
  whether 2-speaker files have the same pattern.
- **2432_T2** — high `&*`, 2 speakers, NOT on no-align list.  Control case
  for a file that currently works but might benefit from per-speaker alignment.

### Why NOT Fridriksson-2

The 249 Fridriksson-2 files on the no-align list all have `@Options: NoAlign`
due to unintelligible speech.  Their failure is a data quality issue, not an
algorithmic one.  They are not relevant to this experiment.

## Prerequisites

- Audio files must be copied from `net`.  See `scripts/fetch-audio.sh`.
- `batchalign3` must be installed and working locally (or on net).
- The input CHAT files are copied from the corpus data.

## Tools

### Rust experiment tool (`utr-experiment`)

A small Cargo project using `talkbank-parser` and `talkbank-model` directly.
No text hacking — all CHAT manipulation goes through the typed AST.

```bash
# Build (first time compiles dependencies)
cargo build --release

# Measure coverage: count timed/untimed utterances per speaker
cargo run --release -- measure FILE.cha
cargo run --release -- measure DIR/

# Split a CHAT file into single-speaker files
cargo run --release -- split INPUT.cha OUTPUT_DIR/

# Strip all timing (bullets, inline timing, %wor tiers)
cargo run --release -- strip INPUT.cha OUTPUT.cha
```

### Fetch audio

```bash
bash scripts/fetch-audio.sh   # scp from net:/Volumes/Other/aphasia/...
```

## Running the Experiment

```bash
# 1. Fetch audio
bash scripts/fetch-audio.sh

# 2. For each test file, prepare input (strip timing from corpus copies)
cargo run --release -- strip ../../data/aphasia-data/English/Protocol/APROCSA/2420_T3.cha data/input/2420_T3.cha
# (2265_T4 uses ~/davida/2265_T4-input.cha which is already stripped)

# 3. Split into single-speaker files
cargo run --release -- split data/input/2420_T3.cha data/split/2420_T3/

# 4. Symlink audio into each directory
ln -s ../../audio/2420_T3.mp4 data/input/2420_T3.mp4
for f in data/split/2420_T3/*.cha; do ln -s ../../audio/2420_T3.mp4 "$(dirname "$f")/$(basename "$f" .cha).mp4"; done

# 5. Run global align
batchalign3 align data/input/ -o results/global/ --lang eng

# 6. Run per-speaker align (each speaker file against the same audio)
for spk_dir in data/split/*/; do
    name=$(basename "$spk_dir")
    for spk_file in "$spk_dir"/*.cha; do
        spk=$(basename "$spk_file" .cha | sed "s/${name}_//")
        batchalign3 align "$spk_dir" -o "results/per-speaker/$name/$spk/" --lang eng
    done
done

# 7. Measure everything
cargo run --release -- measure results/global/
cargo run --release -- measure results/per-speaker/*/*/
```

## Expected Output

A table like:

```
File        | Speaker | Global coverage | Per-speaker coverage | Delta
------------|---------|----------------|---------------------|------
2265_T4     | ALL     | 63.5%          | ??%                 | ??
2265_T4     | INV     | ??%            | ??%                 | ??
2265_T4     | PAR     | ??%            | ??%                 | ??
2265_T4     | REL1    | ??%            | ??%                 | ??
2265_T4     | REL2    | ??%            | ??%                 | ??
2420_T3     | ALL     | ??%            | ??%                 | ??
...
```

## Decision Criteria

- If per-speaker total coverage is **>15 percentage points better** than
  global on 2+ files: per-speaker UTR is worth building.
- If per-speaker is **5-15 points better**: backbone extraction first,
  re-evaluate per-speaker after.
- If per-speaker is **<5 points better**: the problem is within-speaker
  divergence and neither approach helps.  Investigate other strategies.
