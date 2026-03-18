#!/bin/bash
# Experiment A end-to-end: Compare UTR strategies on SBCSAE files.
#
# Runs batchalign3 align with global and two-pass UTR strategies on
# timing-stripped SBCSAE files, then compares recovered timing against
# ground truth.
#
# Prerequisites:
#   - Audio files in data/audio-sbcsae/
#   - Stripped .cha files in data/sbcsae-input/
#   - Ground truth .cha files in data/sbcsae-groundtruth/
#   - batchalign3 built (release recommended)
#
# Usage: bash scripts/run_sbcsae_experiment.sh [--step N]
#   --step 1: Run global UTR alignment
#   --step 2: Run two-pass UTR alignment
#   --step 3: Compare results
#   (no --step: run all)

set -euo pipefail
cd "$(dirname "$0")/.."

BATCHALIGN="../../batchalign3/target/release/batchalign3"
INPUT_DIR="data/sbcsae-input"
AUDIO_DIR="data/audio-sbcsae"
GT_DIR="data/sbcsae-groundtruth"
OUT_GLOBAL="data/sbcsae-output-global"
OUT_TWOPASS="data/sbcsae-output-twopass"
DEBUG_GLOBAL="data/sbcsae-debug-global"
DEBUG_TWOPASS="data/sbcsae-debug-twopass"
RESULTS_DIR="results"

STEP="${1:-all}"

# Build release binary if needed
if [ ! -f "$BATCHALIGN" ]; then
    echo "Building batchalign3 release binary..."
    (cd ../../batchalign3 && cargo build --release -p batchalign-cli)
fi

run_alignment() {
    local strategy="$1"
    local output_dir="$2"
    local debug_dir="$3"

    mkdir -p "$output_dir" "$debug_dir"

    echo "=== Running alignment: --utr-strategy $strategy ==="
    echo "  Input:  $INPUT_DIR"
    echo "  Output: $output_dir"
    echo "  Debug:  $debug_dir"
    echo ""

    for cha in "$INPUT_DIR"/*.cha; do
        base=$(basename "$cha" .cha)
        audio="$AUDIO_DIR/$base.mp3"

        if [ ! -f "$audio" ]; then
            echo "  SKIP $base: no audio"
            continue
        fi

        # Copy input + audio to a temp dir (batchalign expects them together)
        local tmpdir
        tmpdir=$(mktemp -d)
        cp "$cha" "$tmpdir/"
        ln -s "$(cd "$AUDIO_DIR" && pwd)/$base.mp3" "$tmpdir/$base.mp3"

        echo "  Aligning $base ($strategy)..."
        "$BATCHALIGN" align \
            "$tmpdir/$base.cha" \
            -o "$output_dir" \
            --utr-strategy "$strategy" \
            --debug-dir "$debug_dir" \
            --lang eng \
            -v 2>&1 | tail -3

        rm -rf "$tmpdir"
        echo ""
    done
}

# Step 1: Global UTR
if [ "$STEP" = "all" ] || [ "$STEP" = "--step" -a "${2:-}" = "1" ] || [ "$STEP" = "1" ]; then
    run_alignment "global" "$OUT_GLOBAL" "$DEBUG_GLOBAL"
fi

# Step 2: Two-pass UTR
if [ "$STEP" = "all" ] || [ "$STEP" = "--step" -a "${2:-}" = "2" ] || [ "$STEP" = "2" ]; then
    run_alignment "two-pass" "$OUT_TWOPASS" "$DEBUG_TWOPASS"
fi

# Step 3: Compare results
if [ "$STEP" = "all" ] || [ "$STEP" = "--step" -a "${2:-}" = "3" ] || [ "$STEP" = "3" ]; then
    echo "=== Comparing results against ground truth ==="
    echo ""
    echo "Ground truth: $GT_DIR"
    echo "Global:       $OUT_GLOBAL"
    echo "Two-pass:     $OUT_TWOPASS"
    echo ""

    # Use the onset-accuracy tool on each output to measure how well
    # recovered timing matches ground truth overlap positions
    echo "--- Global UTR onset accuracy ---"
    ./target/release/utr-experiment onset-accuracy "$OUT_GLOBAL/" 2>/dev/null

    echo ""
    echo "--- Two-pass UTR onset accuracy ---"
    ./target/release/utr-experiment onset-accuracy "$OUT_TWOPASS/" 2>/dev/null

    # Also measure per-file timing recovery
    echo ""
    echo "--- Timing coverage ---"
    echo "strategy	file	timed	untimed	coverage"
    for strategy in global twopass; do
        dir="data/sbcsae-output-$strategy"
        if [ -d "$dir" ]; then
            for cha in "$dir"/*.cha; do
                base=$(basename "$cha" .cha)
                ./target/release/utr-experiment measure "$cha" 2>/dev/null | grep "ALL" | while read -r line; do
                    echo "$strategy	$line"
                done
            done
        fi
    done
fi

echo ""
echo "Done."
