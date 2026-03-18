#!/bin/bash
# Align all SBCSAE files with a given UTR strategy.
# Usage: bash scripts/align_sbcsae.sh <global|two-pass>
set -euo pipefail
cd "$(dirname "$0")/.."

STRATEGY="${1:?Usage: $0 <global|two-pass>}"
BATCHALIGN="../../batchalign3/target/release/batchalign3"
INPUT_DIR="data/sbcsae-input"
AUDIO_DIR="data/audio-sbcsae"
OUT_DIR="data/sbcsae-output-${STRATEGY//-/}"
DEBUG_DIR="data/sbcsae-debug-${STRATEGY//-/}"

mkdir -p "$OUT_DIR" "$DEBUG_DIR"

for cha in "$INPUT_DIR"/*.cha; do
    base=$(basename "$cha" .cha)
    audio="$AUDIO_DIR/$base.mp3"
    [ -f "$audio" ] || { echo "SKIP $base: no audio"; continue; }

    # batchalign expects audio next to .cha
    tmpdir=$(mktemp -d)
    cp "$cha" "$tmpdir/"
    ln -s "$(cd "$AUDIO_DIR" && pwd)/$base.mp3" "$tmpdir/$base.mp3"

    echo "=== $base ($STRATEGY) ==="
    "$BATCHALIGN" align "$tmpdir/$base.cha" \
        -o "$OUT_DIR" \
        --utr-strategy "$STRATEGY" \
        --debug-dir "$DEBUG_DIR" \
        -v 2>&1 | tail -2

    rm -rf "$tmpdir"
done

echo ""
echo "Done. Output in $OUT_DIR"
