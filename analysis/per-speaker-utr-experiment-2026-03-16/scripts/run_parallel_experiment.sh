#!/bin/bash
# Run alignment experiments in parallel using batchalign3's built-in concurrency.
#
# Instead of processing files one at a time, this creates a temp directory
# with all files + audio symlinks and submits them as a single batch job.
# batchalign3 handles concurrent file processing internally.
#
# Usage: bash scripts/run_parallel_experiment.sh <strategy> [--utr-fuzzy <threshold>]
#   strategy: global, two-pass, auto
#   Examples:
#     bash scripts/run_parallel_experiment.sh auto
#     bash scripts/run_parallel_experiment.sh auto --utr-fuzzy 0.85
#
# Processes ALL corpora that have audio: sbcsae, jefferson, taiwanhakka, multilang

set -euo pipefail
cd "$(dirname "$0")/.."

STRATEGY="${1:?Usage: $0 <global|two-pass|auto> [extra batchalign flags]}"
shift
EXTRA_FLAGS="$*"

BATCHALIGN="../../batchalign3/target/release/batchalign3"

# Build a label for output directories
LABEL="$STRATEGY"
if echo "$EXTRA_FLAGS" | grep -q "utr-fuzzy"; then
    THRESHOLD=$(echo "$EXTRA_FLAGS" | grep -oE '[0-9]+\.[0-9]+')
    LABEL="${STRATEGY}-fuzzy${THRESHOLD}"
fi

OUT_DIR="data/parallel-output-${LABEL}"
mkdir -p "$OUT_DIR"

# Create a staging directory with all files + audio symlinks
STAGING=$(mktemp -d)
echo "Staging files in $STAGING..."

stage_corpus() {
    local corpus="$1"
    local input_dir="$2"
    local audio_dir="$3"

    [ -d "$input_dir" ] || return 0
    [ -d "$audio_dir" ] || return 0

    local count=0
    for cha in "$input_dir"/*.cha; do
        [ -f "$cha" ] || continue
        local base=$(basename "$cha" .cha)

        # Find matching audio (try mp3 then mp4)
        local audio=""
        for ext in mp3 mp4; do
            if [ -f "$audio_dir/$base.$ext" ]; then
                audio="$audio_dir/$base.$ext"
                break
            fi
        done
        [ -n "$audio" ] || continue

        # Use corpus prefix to avoid name collisions
        cp "$cha" "$STAGING/${corpus}_${base}.cha"
        ln -s "$(cd "$(dirname "$audio")" && pwd)/$(basename "$audio")" \
            "$STAGING/${corpus}_${base}.$(echo "$audio" | sed 's/.*\.//')"
        count=$((count + 1))
    done
    echo "  $corpus: $count files"
}

# Stage all corpora
stage_corpus sbcsae data/sbcsae-input data/audio-sbcsae
stage_corpus jefferson data/jefferson-groundtruth data/audio-jefferson
stage_corpus taiwanhakka data/taiwanhakka-groundtruth data/audio-taiwanhakka
stage_corpus aprocsa data/aprocsa-input data/audio-aprocsa
stage_corpus multilang data/multilang-input data/audio-biling
stage_corpus multilang-intl data/multilang-input data/audio-childes-intl
stage_corpus multilang-tbi data/multilang-input data/audio-tbi

# Multilang: input is groundtruth, audio is scattered
for cha in data/multilang-groundtruth/*.cha; do
    base=$(basename "$cha" .cha)
    audio=""
    for dir in data/audio-biling data/audio-childes data/audio-childes-intl data/audio-tbi; do
        for ext in mp3 mp4; do
            candidate="$dir/$base.$ext"
            [ -f "$candidate" ] && audio="$candidate" && break 2
        done
    done
    [ -n "$audio" ] || continue
    cp "$cha" "$STAGING/multilang_${base}.cha"
    ln -s "$(cd "$(dirname "$audio")" && pwd)/$(basename "$audio")" \
        "$STAGING/multilang_${base}.$(echo "$audio" | sed 's/.*\.//')"
done
echo "  multilang: $(ls "$STAGING"/multilang_*.cha 2>/dev/null | wc -l) files"

TOTAL=$(ls "$STAGING"/*.cha 2>/dev/null | wc -l)
echo ""
echo "Total: $TOTAL files staged"
echo "Strategy: $STRATEGY $EXTRA_FLAGS"
echo "Output: $OUT_DIR"
echo ""

# Submit as a single batch job
"$BATCHALIGN" align "$STAGING"/*.cha \
    -o "$OUT_DIR" \
    --utr-strategy "$STRATEGY" \
    $EXTRA_FLAGS \
    -v 2>&1

rm -rf "$STAGING"
echo ""
echo "Done. $TOTAL files written to $OUT_DIR"
