#!/usr/bin/env bash
# Fetch audio files from net for the per-speaker UTR experiment.
#
# Usage: bash scripts/fetch-audio.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXPERIMENT_DIR="$(dirname "$SCRIPT_DIR")"
AUDIO_DIR="$EXPERIMENT_DIR/data/audio"

mkdir -p "$AUDIO_DIR"

REMOTE="macw@net"
REMOTE_BASE="/Volumes/Other/aphasia/English/Protocol/APROCSA"

FILES=(2265_T4.mp4 2420_T3.mp4 2463_T2.mp4 2432_T2.mp4)

for f in "${FILES[@]}"; do
    if [ -f "$AUDIO_DIR/$f" ]; then
        echo "SKIP $f (already exists)"
    else
        echo "FETCH $f"
        scp "$REMOTE:$REMOTE_BASE/$f" "$AUDIO_DIR/$f"
    fi
done

echo ""
ls -lh "$AUDIO_DIR"
