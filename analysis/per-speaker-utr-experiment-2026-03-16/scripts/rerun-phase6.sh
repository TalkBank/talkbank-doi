#!/bin/bash
# Phase 6: Rerun all CA corpus experiments with corrected overlap semantics.
# Runs global and auto (best-of-both) strategies on SBCSAE, Jefferson, TaiwanHakka.
# Captures timing coverage for comparison with previous results.
#
# Usage: bash scripts/rerun-phase6.sh
set -euo pipefail
cd "$(dirname "$0")/.."

BATCHALIGN="../../batchalign3/target/release/batchalign3"
TIMESTAMP=$(date +%Y%m%d-%H%M)

if [ ! -x "$BATCHALIGN" ]; then
    echo "ERROR: batchalign3 release binary not found at $BATCHALIGN"
    echo "       Run: cd ../../batchalign3 && cargo build --release -p batchalign-cli"
    exit 1
fi

# Ensure server is running with safe worker count
echo "Starting batchalign3 server (2 workers)..."
"$BATCHALIGN" serve start --workers 2 2>/dev/null || true
sleep 3

RESULTS_FILE="results/phase6-${TIMESTAMP}.txt"
mkdir -p results

align_corpus() {
    local corpus="$1"
    local strategy="$2"
    local input_dir="data/${corpus}-input"
    local audio_dir="data/audio-${corpus}"
    local out_dir="data/${corpus}-phase6-${strategy}"

    mkdir -p "$out_dir"

    for cha in "$input_dir"/*.cha; do
        local base=$(basename "$cha" .cha)
        local audio=""
        for ext in mp3 mp4 wav; do
            [ -f "$audio_dir/$base.$ext" ] && audio="$audio_dir/$base.$ext" && break
        done
        [ -n "$audio" ] || { echo "SKIP $base: no audio"; continue; }

        local tmpdir=$(mktemp -d)
        cp "$cha" "$tmpdir/"
        ln -s "$(cd "$audio_dir" && pwd)/$(basename "$audio")" "$tmpdir/$(basename "$audio")"

        echo "  $base ($corpus, $strategy)..."
        "$BATCHALIGN" --no-open-dashboard align "$tmpdir/$base.cha" \
            -o "$out_dir" \
            --utr-strategy "$strategy" \
            -v 2>&1 | grep -E "timed|untimed|coverage" || true

        rm -rf "$tmpdir"
    done
}

count_timed() {
    local dir="$1"
    local total=0
    local timed=0
    for f in "$dir"/*.cha; do
        [ -f "$f" ] || continue
        local file_total=$(grep -c '^\*' "$f" 2>/dev/null || echo 0)
        local file_timed=$(grep -c '^\*.*[0-9]_[0-9]' "$f" 2>/dev/null || echo 0)
        total=$((total + file_total))
        timed=$((timed + file_timed))
    done
    echo "$timed/$total"
}

echo "========================================" | tee "$RESULTS_FILE"
echo "Phase 6: CA Overlap Experiment Rerun"     | tee -a "$RESULTS_FILE"
echo "Date: $(date)"                             | tee -a "$RESULTS_FILE"
echo "========================================" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

for corpus in sbcsae jefferson taiwanhakka; do
    echo "=== $corpus ===" | tee -a "$RESULTS_FILE"

    for strategy in global auto; do
        echo "--- Strategy: $strategy ---" | tee -a "$RESULTS_FILE"
        align_corpus "$corpus" "$strategy"
        result=$(count_timed "data/${corpus}-phase6-${strategy}")
        echo "  Coverage: $result" | tee -a "$RESULTS_FILE"
    done

    # Compare with previous results
    echo "" | tee -a "$RESULTS_FILE"
    echo "Previous results:" | tee -a "$RESULTS_FILE"
    if [ -d "data/${corpus}-output-global" ]; then
        prev_global=$(count_timed "data/${corpus}-output-global")
        echo "  global (prev): $prev_global" | tee -a "$RESULTS_FILE"
    fi
    if [ -d "data/${corpus}-output-twopass" ]; then
        prev_twopass=$(count_timed "data/${corpus}-output-twopass")
        echo "  two-pass (prev): $prev_twopass" | tee -a "$RESULTS_FILE"
    fi
    if [ -d "data/${corpus}-output-fuzzy0.85" ]; then
        prev_fuzzy=$(count_timed "data/${corpus}-output-fuzzy0.85")
        echo "  fuzzy 0.85 (prev): $prev_fuzzy" | tee -a "$RESULTS_FILE"
    fi
    echo "" | tee -a "$RESULTS_FILE"
done

echo "========================================" | tee -a "$RESULTS_FILE"
echo "Done. Full results in: $RESULTS_FILE"      | tee -a "$RESULTS_FILE"

# Stop server
"$BATCHALIGN" serve stop 2>/dev/null || true
