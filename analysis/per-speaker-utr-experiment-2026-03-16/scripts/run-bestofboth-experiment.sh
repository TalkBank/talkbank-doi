#!/usr/bin/env bash
# Verify best-of-both fallback: re-run problem files.
# Algorithm: TwoPassOverlapUtr now runs both strategies, keeps better.
set -euo pipefail
cd "$(dirname "$0")/.."

BA3="${BA3:-../../batchalign3/target/release/batchalign3}"
TOOL="./target/release/utr-experiment"

echo "=== Best-of-both verification ==="

for f in fusser12 german050814 tbi_n22 tbi_tb23; do
    dir="data/multilang-individual/${f}"
    outdir="results/bestofboth-experiment/two-pass/${f}"
    [ -d "$dir" ] || continue
    mkdir -p "$outdir"
    echo "  ${f}..."
    $BA3 align "$dir" -o "$outdir" --utr-strategy two-pass || echo "  WARN: failed"
done

# APROCSA 2265_T4
dir="data/converted-individual/2265_T4"
outdir="results/bestofboth-experiment/two-pass-aprocsa/2265_T4"
mkdir -p "$outdir"
echo "  2265_T4 (APROCSA)..."
$BA3 align "$dir" -o "$outdir" --utr-strategy two-pass || echo "  WARN: failed"

echo ""
echo "=== Coverage comparison ==="
echo "algorithm	file	total	timed	untimed	coverage"
for f in fusser12 german050814 tbi_n22 tbi_tb23; do
    for variant in "multilang-experiment/global" "multilang-experiment/two-pass" "adaptive-experiment/two-pass" "bestofboth-experiment/two-pass"; do
        result="results/${variant}/${f}/${f}.cha"
        if [ -f "$result" ]; then
            label=$(echo "$variant" | sed 's|.*/||')
            $TOOL measure "$result" 2>/dev/null | grep ALL | while read line; do
                echo "${label}	${line}"
            done
        fi
    done
    echo ""
done

echo "=== Done ==="
