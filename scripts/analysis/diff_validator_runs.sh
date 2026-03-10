#!/usr/bin/env bash
# Compare validation results before and after a code change.
#
# Usage:
#   # Save baseline:
#   scripts/analysis/diff_validator_runs.sh baseline ~/data/phon-data
#
#   # (make code changes)
#
#   # Compare:
#   scripts/analysis/diff_validator_runs.sh compare ~/data/phon-data
#
# Stores results in /tmp/validator-{baseline,current}.txt

set -euo pipefail

MODE="${1:?Usage: $0 <baseline|compare> <data-dir>}"
DATA_DIR="${2:?Usage: $0 <baseline|compare> <data-dir>}"
BASELINE="/tmp/validator-baseline.txt"
CURRENT="/tmp/validator-current.txt"

run_validator() {
    local output_file="$1"
    echo "Running validator on $DATA_DIR..."
    cargo run --release -p talkbank-cli -- validate "$DATA_DIR" --force 2>&1 \
        | grep '✗ Errors found in\|error\[E' \
        | sort \
        > "$output_file"
    echo "Saved to $output_file ($(wc -l < "$output_file") lines)"
}

case "$MODE" in
    baseline)
        run_validator "$BASELINE"
        ;;
    compare)
        run_validator "$CURRENT"
        echo ""
        echo "=== Diff (baseline → current) ==="
        if diff "$BASELINE" "$CURRENT" > /dev/null 2>&1; then
            echo "No differences."
        else
            echo "New errors (not in baseline):"
            comm -13 "$BASELINE" "$CURRENT" | head -20
            echo ""
            echo "Fixed errors (in baseline but not current):"
            comm -23 "$BASELINE" "$CURRENT" | head -20
            echo ""
            echo "Summary:"
            echo "  Baseline: $(wc -l < "$BASELINE") lines"
            echo "  Current:  $(wc -l < "$CURRENT") lines"
            echo "  Added:    $(comm -13 "$BASELINE" "$CURRENT" | wc -l) lines"
            echo "  Removed:  $(comm -23 "$BASELINE" "$CURRENT" | wc -l) lines"
        fi
        ;;
    *)
        echo "Usage: $0 <baseline|compare> <data-dir>"
        exit 1
        ;;
esac
