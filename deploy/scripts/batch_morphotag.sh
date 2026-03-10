#!/usr/bin/env bash
set -euo pipefail

FILE_LIST="${1:?Usage: batch_morphotag.sh FILE_LIST [SERVER_URL]}"
SERVER="${2:-http://net:8000}"
BATCH_SIZE=500
BATCH_DIR="/tmp/morphotag-batches"
LOG_FILE="$BATCH_DIR/batch_run.log"

# --- Filter out already-processed files ---
# Files written back by batchalign3 have modification times after the
# first batch run started (Feb 16 12:05).  We skip those.
CUTOFF=$(date -j -f "%Y-%m-%d %H:%M:%S" "2026-02-16 12:05:00" "+%s")
FILTERED="$BATCH_DIR/remaining.txt"

mkdir -p "$BATCH_DIR"
# Clean old batch splits
rm -f "$BATCH_DIR"/batch_*

TOTAL_ORIG=$(wc -l < "$FILE_LIST")
SKIPPED=0
: > "$FILTERED"
while IFS= read -r f; do
    if [ -f "$f" ]; then
        mod=$(stat -f '%m' "$f" 2>/dev/null)
        if [ -n "$mod" ] && [ "$mod" -ge "$CUTOFF" ]; then
            SKIPPED=$((SKIPPED + 1))
            continue
        fi
    fi
    echo "$f" >> "$FILTERED"
done < "$FILE_LIST"

REMAINING=$(wc -l < "$FILTERED")

echo "=== Morphotag batch run ===" | tee "$LOG_FILE"
echo "Original file list: $TOTAL_ORIG" | tee -a "$LOG_FILE"
echo "Already processed (skipped): $SKIPPED" | tee -a "$LOG_FILE"
echo "Remaining to process: $REMAINING" | tee -a "$LOG_FILE"
echo "Batch size: $BATCH_SIZE" | tee -a "$LOG_FILE"
echo "Server: $SERVER" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

if [ "$REMAINING" -eq 0 ]; then
    echo "Nothing to do — all files already processed." | tee -a "$LOG_FILE"
    exit 0
fi

# Split filtered list into chunks
split -l "$BATCH_SIZE" "$FILTERED" "$BATCH_DIR/batch_"

BATCHES=("$BATCH_DIR"/batch_*)
TOTAL=${#BATCHES[@]}

SUCCEEDED=0
FAILED=0

for i in "${!BATCHES[@]}"; do
    BATCH="${BATCHES[$i]}"
    N=$((i + 1))
    COUNT=$(wc -l < "$BATCH")
    echo "[$(date '+%H:%M:%S')] Batch $N/$TOTAL ($COUNT files): $BATCH" | tee -a "$LOG_FILE"

    if uv run batchalign3 --server "$SERVER" morphotag \
        --file-list "$BATCH" --in-place 2>&1 | tee -a "$LOG_FILE"; then
        SUCCEEDED=$((SUCCEEDED + 1))
        echo "[$(date '+%H:%M:%S')] Batch $N/$TOTAL: OK" | tee -a "$LOG_FILE"
    else
        FAILED=$((FAILED + 1))
        echo "[$(date '+%H:%M:%S')] Batch $N/$TOTAL: FAILED (exit $?)" | tee -a "$LOG_FILE"
    fi
    echo "" | tee -a "$LOG_FILE"
done

echo "=== Done ===" | tee -a "$LOG_FILE"
echo "Succeeded: $SUCCEEDED / $TOTAL" | tee -a "$LOG_FILE"
echo "Failed: $FAILED / $TOTAL" | tee -a "$LOG_FILE"
