#!/bin/bash
# Run Experiment B: Overlap marker consistency audit across all corpora.
#
# Usage: bash scripts/run_overlap_audit.sh [--verbose]
#
# Outputs TSV results to results/overlap-audit-*.tsv

set -euo pipefail
cd "$(dirname "$0")/.."

TOOL="./target/release/utr-experiment"
RESULTS_DIR="results"
DATA_DIR="../../data"

# Build release binary if not present
if [ ! -f "$TOOL" ]; then
    echo "Building release binary..."
    cargo build --release
fi

mkdir -p "$RESULTS_DIR"

VERBOSE_FLAG="${1:---verbose}"

echo "=== CA-DATA ==="
"$TOOL" overlap-audit "$DATA_DIR/ca-data/" $VERBOSE_FLAG 2>"$RESULTS_DIR/overlap-audit-ca-data.log" \
    | tee "$RESULTS_DIR/overlap-audit-ca-data.tsv"

echo ""
echo "=== CHILDES-DATA ==="
"$TOOL" overlap-audit "$DATA_DIR/childes-data/" $VERBOSE_FLAG 2>"$RESULTS_DIR/overlap-audit-childes-data.log" \
    | tee "$RESULTS_DIR/overlap-audit-childes-data.tsv"

echo ""
echo "Done. Results in $RESULTS_DIR/overlap-audit-*.tsv"
