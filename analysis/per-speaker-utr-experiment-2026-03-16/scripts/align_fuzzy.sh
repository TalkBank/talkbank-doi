#!/bin/bash
# Align a corpus with fuzzy matching at a given threshold.
# Usage: bash scripts/align_fuzzy.sh <corpus> <threshold>
set -euo pipefail
cd "$(dirname "$0")/.."

CORPUS="${1:?Usage: $0 <sbcsae|jefferson|taiwanhakka> <threshold>}"
THRESHOLD="${2:?Usage: $0 <corpus> <threshold, e.g. 0.85>}"
BATCHALIGN="../../batchalign3/target/release/batchalign3"

INPUT_DIR="data/${CORPUS}-input"
AUDIO_DIR="data/audio-${CORPUS}"
# Use ground truth for corpora with no stripped timing (taiwanhakka)
if [ ! -d "$INPUT_DIR" ] && [ -d "data/${CORPUS}-groundtruth" ]; then
    INPUT_DIR="data/${CORPUS}-groundtruth"
fi
OUT_DIR="data/${CORPUS}-output-fuzzy${THRESHOLD}"

mkdir -p "$OUT_DIR"

for cha in "$INPUT_DIR"/*.cha; do
    base=$(basename "$cha" .cha)
    audio="$AUDIO_DIR/$base.mp3"
    [ -f "$audio" ] || { echo "SKIP $base: no audio at $audio"; continue; }

    tmpdir=$(mktemp -d)
    cp "$cha" "$tmpdir/"
    ln -s "$(cd "$AUDIO_DIR" && pwd)/$base.mp3" "$tmpdir/$base.mp3"

    echo "=== $base ($CORPUS, fuzzy $THRESHOLD) ==="
    "$BATCHALIGN" --no-open-dashboard align "$tmpdir/$base.cha" \
        -o "$OUT_DIR" \
        --utr-strategy auto \
        --utr-fuzzy "$THRESHOLD" \
        -v 2>&1 | tail -2

    rm -rf "$tmpdir"
done

echo "Done. Output in $OUT_DIR"
