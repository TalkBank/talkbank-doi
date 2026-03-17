#!/usr/bin/env bash
# Run the +< ground-truth alignment experiment.
#
# Uses CHILDES files that already have +< utterances WITH timing as ground
# truth. Strips timing, re-aligns with both strategies, compares recovered
# timing against the original bullets.
#
# This tests: does two-pass UTR give better +< backchannel timing than
# global UTR on files that natively use the +< encoding?
#
# Prerequisites:
#   - Audio files in data/audio-childes/
#   - batchalign3 built and available
#
# Usage:
#   bash scripts/run-groundtruth-experiment.sh [--dry-run]

set -euo pipefail
cd "$(dirname "$0")/.."

DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
    echo "DRY RUN — showing commands without executing"
fi

# Paths
BA3="${BA3:-../../batchalign3/target/release/batchalign3}"
TOOL="./target/release/utr-experiment"

# Build tools
echo "=== Building tools ==="
cargo build --release
(cd ../../batchalign3 && cargo build --release -p batchalign-cli)

# Test files: corpus path relative to data root, audio filename
declare -A FILE_MAP
FILE_MAP[060211a2]="childes-data/Eng-NA/MacWhinney/060211a2.cha"
FILE_MAP[060211b1]="childes-data/Eng-NA/MacWhinney/060211b1.cha"
FILE_MAP[060406b2]="childes-data/Eng-NA/MacWhinney/060406b2.cha"
FILE_MAP[020518b]="childes-data/Eng-NA/Snow/020518b.cha"

FILES=(060211a2 060211b1 060406b2 020518b)

# Step 1: Copy originals as ground truth and create stripped inputs
echo ""
echo "=== Step 1: Preparing ground truth and stripped inputs ==="
mkdir -p data/groundtruth data/stripped-input

for f in "${FILES[@]}"; do
    corpus_path="${FILE_MAP[$f]}"
    src="../../data/${corpus_path}"

    if [[ ! -f "$src" ]]; then
        echo "SKIP: $src not found"
        continue
    fi

    # Copy original as ground truth
    cp "$src" "data/groundtruth/${f}.cha"

    # Strip timing to create input
    if [[ "$DRY_RUN" == false ]]; then
        $TOOL strip "data/groundtruth/${f}.cha" "data/stripped-input/${f}.cha"
    else
        echo "  Would strip: data/groundtruth/${f}.cha -> data/stripped-input/${f}.cha"
    fi
done

# Step 2: Set up input directories with audio symlinks
echo ""
echo "=== Step 2: Setting up input directories ==="
for f in "${FILES[@]}"; do
    dir="data/stripped-individual/${f}"
    mkdir -p "$dir"

    if [[ -f "data/stripped-input/${f}.cha" ]]; then
        cp "data/stripped-input/${f}.cha" "$dir/${f}.cha"
    fi

    # Symlink audio
    for ext in mp3 mp4 wav; do
        if [[ -f "data/audio-childes/${f}.${ext}" && ! -f "$dir/${f}.${ext}" ]]; then
            ln -sf "$(cd data/audio-childes && pwd)/${f}.${ext}" "$dir/${f}.${ext}"
        fi
    done
done

# Step 3: Align with both strategies
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

    # Global UTR (original single-pass algorithm)
    run_align "global" \
        "data/stripped-individual/${f}" \
        "results/groundtruth-experiment/global/${f}" \
        "global"

    # Two-pass UTR (overlap-aware)
    run_align "two-pass" \
        "data/stripped-individual/${f}" \
        "results/groundtruth-experiment/two-pass/${f}" \
        "two-pass"
done

# Step 4: Measure coverage
echo ""
echo "=== Step 4: Coverage comparison ==="
echo ""
echo "strategy	file	speaker	total	timed	untimed	coverage"

# Ground truth coverage
for f in "${FILES[@]}"; do
    if [[ -f "data/groundtruth/${f}.cha" ]]; then
        $TOOL measure "data/groundtruth/${f}.cha" 2>/dev/null | while IFS= read -r line; do
            if [[ "$line" != file* ]]; then
                echo "ground-truth	${line}"
            fi
        done
    fi
done

# Aligned coverage
for strategy in global two-pass; do
    for f in "${FILES[@]}"; do
        result="results/groundtruth-experiment/${strategy}/${f}/${f}.cha"
        if [[ -f "$result" ]]; then
            $TOOL measure "$result" 2>/dev/null | while IFS= read -r line; do
                if [[ "$line" != file* ]]; then
                    echo "${strategy}	${line}"
                fi
            done
        fi
    done
done

echo ""
echo "=== Experiment complete ==="
echo ""
echo "Ground truth files: data/groundtruth/"
echo "Global results:     results/groundtruth-experiment/global/"
echo "Two-pass results:   results/groundtruth-experiment/two-pass/"
echo ""
echo "To compare timing quality (not just coverage), examine +< utterance"
echo "bullets in ground truth vs each strategy's output."
