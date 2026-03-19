#!/usr/bin/env bash
# Experiment 4: Debug TaiwanHakka regression mechanism.
# Runs both strategies with --debug-dir to capture FA group counts
# and UTR bullet assignments, then compares the debug output.
#
# Usage: bash scripts/experiment4-hakka-decomposition.sh
set -euo pipefail
cd "$(dirname "$0")/.."

BA3="${BA3:-../../batchalign3/target/release/batchalign3}"
TIMESTAMP=$(date +%Y%m%d-%H%M)
RESULTS_FILE="results/experiment4-hakka-decomposition-${TIMESTAMP}.txt"
mkdir -p results

if [ ! -x "$BA3" ]; then
    echo "ERROR: batchalign3 not found at $BA3"
    exit 1
fi

TAIWANHAKKA_FILES=(01 02 03 10 12)

# --- Start server ---
echo "Starting batchalign3 server (2 workers)..."
"$BA3" serve start --workers 2 2>/dev/null || true
sleep 3

# --- Run with debug output ---
OUT_BASE="data/experiment4"

for strategy in global two-pass; do
    echo ""
    echo "=== Strategy: $strategy ==="

    for name in "${TAIWANHAKKA_FILES[@]}"; do
        input="data/taiwanhakka-input/${name}.cha"
        audio="data/audio-taiwanhakka/${name}.mp3"
        out_dir="${OUT_BASE}/${strategy}/${name}"
        debug_dir="${OUT_BASE}/debug-${strategy}/${name}"

        mkdir -p "$out_dir" "$debug_dir"

        if [ ! -f "$input" ] || [ ! -f "$audio" ]; then
            echo "  SKIP $name: missing input or audio"
            continue
        fi

        tmpdir=$(mktemp -d)
        cp "$input" "$tmpdir/${name}.cha"
        ln -s "$(cd data/audio-taiwanhakka && pwd)/${name}.mp3" "$tmpdir/${name}.mp3"

        echo "  $name ($strategy) with debug..."
        "$BA3" --no-open-dashboard align "$tmpdir/${name}.cha" \
            -o "$out_dir" \
            --utr-strategy "$strategy" \
            --debug-dir "$debug_dir" \
            -vv 2>&1 | tail -5 || true

        rm -rf "$tmpdir"
    done
done

# --- Stop server ---
"$BA3" serve stop 2>/dev/null || true

# --- Analyze debug output ---
count_timed_file() {
    local f="$1"
    if [ -f "$f" ]; then
        local total timed
        total=$(grep -c '^\*' "$f" 2>/dev/null || echo 0)
        timed=$(grep -c '^\*.*[0-9]_[0-9]' "$f" 2>/dev/null || echo 0)
        echo "${timed}/${total}"
    else
        echo "n/a"
    fi
}

{
    echo "========================================"
    echo "Experiment 4: TaiwanHakka Regression Decomposition"
    echo "Date: $(date)"
    echo "========================================"
    echo ""

    # Coverage comparison
    printf "%-8s %-15s %-15s %-10s\n" "file" "global" "two-pass" "delta"
    printf "%-8s %-15s %-15s %-10s\n" "----" "------" "--------" "-----"

    for name in "${TAIWANHAKKA_FILES[@]}"; do
        g_result=$(count_timed_file "${OUT_BASE}/global/${name}/${name}.cha")
        t_result=$(count_timed_file "${OUT_BASE}/two-pass/${name}/${name}.cha")
        g_timed=${g_result%%/*}
        t_timed=${t_result%%/*}
        delta=$((t_timed - g_timed))
        sign=""; [ "$delta" -gt 0 ] && sign="+"
        printf "%-8s %-15s %-15s %-10s\n" "$name" "$g_result" "$t_result" "${sign}${delta}"
    done

    echo ""
    echo "=== Debug artifact inventory ==="
    for strategy in global two-pass; do
        echo ""
        echo "--- $strategy ---"
        for name in "${TAIWANHAKKA_FILES[@]}"; do
            debug_dir="${OUT_BASE}/debug-${strategy}/${name}"
            if [ -d "$debug_dir" ]; then
                count=$(find "$debug_dir" -type f | wc -l | tr -d ' ')
                echo "  $name: $count debug files"
                # List file types
                find "$debug_dir" -type f -name "*.json" -o -name "*.cha" -o -name "*.txt" 2>/dev/null | head -5 | while read -r f; do
                    echo "    $(basename "$f")"
                done
            else
                echo "  $name: no debug output"
            fi
        done
    done

    echo ""
    echo "=== FA group analysis ==="
    echo "(Inspect debug dirs manually for detailed FA group/UTR comparisons)"
    echo ""
    echo "Debug dirs:"
    echo "  Global:   ${OUT_BASE}/debug-global/"
    echo "  Two-pass: ${OUT_BASE}/debug-two-pass/"

} | tee "$RESULTS_FILE"

echo ""
echo "Results saved to: $RESULTS_FILE"
