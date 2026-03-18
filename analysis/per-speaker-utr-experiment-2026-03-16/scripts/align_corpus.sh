#!/bin/bash
# Align a corpus with a given UTR strategy.
# Usage: bash scripts/align_corpus.sh <corpus> <global|two-pass>
# Where corpus is: sbcsae, jefferson, taiwanhakka
set -euo pipefail
cd "$(dirname "$0")/.."

CORPUS="${1:?Usage: $0 <sbcsae|jefferson|taiwanhakka> <global|two-pass>}"
STRATEGY="${2:?Usage: $0 <sbcsae|jefferson|taiwanhakka> <global|two-pass>}"
BATCHALIGN="../../batchalign3/target/release/batchalign3"

INPUT_DIR="data/${CORPUS}-input"
AUDIO_DIR="data/audio-${CORPUS}"
OUT_DIR="data/${CORPUS}-output-${STRATEGY//-/}"
DEBUG_DIR="data/${CORPUS}-debug-${STRATEGY//-/}"

if [ ! -d "$INPUT_DIR" ]; then
    echo "ERROR: $INPUT_DIR not found"
    exit 1
fi

mkdir -p "$OUT_DIR" "$DEBUG_DIR"

for cha in "$INPUT_DIR"/*.cha; do
    base=$(basename "$cha" .cha)
    # Find audio (try mp3, mp4, wav)
    audio=""
    for ext in mp3 mp4 wav; do
        [ -f "$AUDIO_DIR/$base.$ext" ] && audio="$AUDIO_DIR/$base.$ext" && break
    done
    [ -n "$audio" ] || { echo "SKIP $base: no audio in $AUDIO_DIR"; continue; }

    tmpdir=$(mktemp -d)
    cp "$cha" "$tmpdir/"
    ln -s "$(cd "$AUDIO_DIR" && pwd)/$(basename "$audio")" "$tmpdir/$(basename "$audio")"

    echo "=== $base ($CORPUS, $STRATEGY) ==="
    "$BATCHALIGN" --no-open-dashboard align "$tmpdir/$base.cha" \
        -o "$OUT_DIR" \
        --utr-strategy "$STRATEGY" \
        --debug-dir "$DEBUG_DIR" \
        -v 2>&1 | tail -2

    rm -rf "$tmpdir"
done

echo ""
echo "Done. Output in $OUT_DIR"
