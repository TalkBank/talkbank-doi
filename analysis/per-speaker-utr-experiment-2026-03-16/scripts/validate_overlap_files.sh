#!/bin/bash
# Validate all files with CA overlap markers using chatter.
# Saves per-file summary to results/overlap-validation-summary.tsv
set -euo pipefail

CHATTER="${1:-../../talkbank-tools/target/release/chatter}"
PATHS_FILE="/tmp/ca-overlap-paths.txt"
OUT_FILE="results/overlap-validation-summary.tsv"

if [ ! -f "$CHATTER" ]; then
    echo "Building chatter release..."
    (cd ../../talkbank-tools && cargo build --release -p talkbank-cli)
    CHATTER="../../talkbank-tools/target/release/chatter"
fi

echo "file	status	error_count	error_codes" > "$OUT_FILE"

total=$(wc -l < "$PATHS_FILE")
i=0
while IFS= read -r path; do
    i=$((i + 1))
    if [ $((i % 50)) -eq 0 ]; then
        echo "  [$i/$total] $path" >&2
    fi

    output=$("$CHATTER" validate "$path" --force 2>&1) || true

    if echo "$output" | grep -q "is valid"; then
        echo "$(basename "$path" .cha)	valid	0	" >> "$OUT_FILE"
    else
        # Count error codes
        codes=$(echo "$output" | grep -oE 'E[0-9]+' | sort | uniq -c | sort -rn | awk '{printf "%s(%s) ", $2, $1}' | sed 's/ $//')
        count=$(echo "$output" | grep -cE 'E[0-9]+' || true)
        echo "$(basename "$path" .cha)	errors	$count	$codes" >> "$OUT_FILE"
    fi
done < "$PATHS_FILE"

echo ""
echo "Done. Results in $OUT_FILE"
echo "Summary:"
awk -F'\t' '{print $2}' "$OUT_FILE" | sort | uniq -c | sort -rn
