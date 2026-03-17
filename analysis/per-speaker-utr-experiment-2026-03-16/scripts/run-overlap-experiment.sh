#!/usr/bin/env bash
# Run the +< overlap alignment experiment.
#
# Compares alignment coverage using three strategies:
#   A) Original files with &* (backchannels invisible, get no timing)
#   B) Converted to +<, GlobalUtr (backchannels in global DP — failure mode)
#   C) Converted to +<, TwoPassOverlapUtr (backchannels get windowed recovery)
#
# Prerequisites:
#   - Audio files in data/audio/ (fetch with scripts/fetch-audio.sh)
#   - Input CHAT files in data/input/ (stripped of timing)
#   - batchalign3 built and available (cargo build -p batchalign-cli)
#
# Usage:
#   bash scripts/run-overlap-experiment.sh [--dry-run]

set -euo pipefail
cd "$(dirname "$0")/.."

DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
    echo "DRY RUN — showing commands without executing"
fi

# Paths
BA3="${BA3:-../../batchalign3/target/release/batchalign3}"
CONVERT="./target/release/utr-experiment"
MEASURE="./target/release/utr-experiment"

# Build tools
echo "=== Building tools ==="
cargo build --release
(cd ../../batchalign3 && cargo build --release -p batchalign-cli)

# Test files (APROCSA)
FILES=(2265_T4 2420_T3 2432_T2 2463_T2)

# Step 1: Convert &* -> separate utterances (with and without +< linker)
echo ""
echo "=== Step 1: Converting &* ==="
mkdir -p data/converted data/converted-no-linker
for f in "${FILES[@]}"; do
    if [[ ! -f "data/input/${f}.cha" ]]; then
        echo "SKIP: data/input/${f}.cha not found"
        continue
    fi
    echo "  Converting ${f} (with +<)..."
    if [[ "$DRY_RUN" == false ]]; then
        $CONVERT convert "data/input/${f}.cha" "data/converted/${f}.cha"
    fi
    echo "  Converting ${f} (no linker)..."
    if [[ "$DRY_RUN" == false ]]; then
        $CONVERT convert --no-linker "data/input/${f}.cha" "data/converted-no-linker/${f}.cha"
    fi
done

# Step 2: Create input directories with audio symlinks
echo ""
echo "=== Step 2: Setting up input directories ==="
for variant in original converted converted-no-linker; do
    for f in "${FILES[@]}"; do
        dir="data/${variant}-individual/${f}"
        mkdir -p "$dir"

        if [[ "$variant" == "original" ]]; then
            src="data/input/${f}.cha"
        elif [[ "$variant" == "converted" ]]; then
            src="data/converted/${f}.cha"
        else
            src="data/converted-no-linker/${f}.cha"
        fi

        if [[ -f "$src" ]]; then
            cp "$src" "$dir/${f}.cha"
        fi

        # Symlink audio (try mp4 then mp3 then wav)
        for ext in mp4 mp3 wav; do
            if [[ -f "data/audio/${f}.${ext}" && ! -f "$dir/${f}.${ext}" ]]; then
                ln -sf "$(cd data/audio && pwd)/${f}.${ext}" "$dir/${f}.${ext}"
            fi
        done
    done
done

# Step 3: Run alignment with each strategy
echo ""
echo "=== Step 3: Running alignment ==="

run_align() {
    local label="$1"
    local input_dir="$2"
    local output_dir="$3"
    local strategy="$4"

    mkdir -p "$output_dir"
    echo "  [$label] ${input_dir} -> ${output_dir} (strategy: ${strategy})"

    if [[ "$DRY_RUN" == false ]]; then
        $BA3 align "$input_dir" -o "$output_dir" --utr-strategy "$strategy" || {
            echo "    WARNING: align failed for $label"
        }
    fi
}

for f in "${FILES[@]}"; do
    echo ""
    echo "--- ${f} ---"

    # A) Original with &* (auto strategy — will use GlobalUtr since no +<)
    #    Backchannels invisible, get no timing. Current production behavior.
    run_align "A-original" \
        "data/original-individual/${f}" \
        "results/overlap-experiment/A-original/${f}" \
        "auto"

    # B) Separate utterances, NO +< linker, GlobalUtr
    #    The failure mode: backchannels in the global DP at wrong positions,
    #    no signal to tell the aligner they overlap. This is what people tried
    #    before giving up and using &*.
    run_align "B-no-linker-global" \
        "data/converted-no-linker-individual/${f}" \
        "results/overlap-experiment/B-no-linker-global/${f}" \
        "auto"

    # C) Separate utterances WITH +<, forced GlobalUtr
    #    +< present but ignored — shows the DP penalty of backchannel words
    #    in the global reference when the strategy doesn't use the signal.
    run_align "C-with-linker-global" \
        "data/converted-individual/${f}" \
        "results/overlap-experiment/C-with-linker-global/${f}" \
        "global"

    # D) Separate utterances WITH +<, TwoPassOverlapUtr
    #    The improvement: +< used as a signal to exclude from pass 1 and
    #    recover in pass 2.
    run_align "D-with-linker-twopass" \
        "data/converted-individual/${f}" \
        "results/overlap-experiment/D-with-linker-twopass/${f}" \
        "two-pass"
done

# Step 4: Measure coverage
echo ""
echo "=== Step 4: Measuring coverage ==="
echo ""
echo "variant	file	speaker	total	timed	untimed	coverage"

for variant in A-original B-no-linker-global C-with-linker-global D-with-linker-twopass; do
    for f in "${FILES[@]}"; do
        result_dir="results/overlap-experiment/${variant}/${f}"
        if [[ -f "$result_dir/${f}.cha" ]]; then
            $MEASURE measure "$result_dir/${f}.cha" 2>/dev/null | while IFS= read -r line; do
                # Skip header line from measure
                if [[ "$line" != file* ]]; then
                    echo "${variant}	${line}"
                fi
            done
        fi
    done
done

echo ""
echo "=== Experiment complete ==="
echo "Results in results/overlap-experiment/"
