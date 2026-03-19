#!/usr/bin/env bash
# Experiment 1: Best-of-both per non-English file.
# Runs global, two-pass, and auto strategies on all non-English files.
# Reports timed utterance counts per file to identify which strategy wins.
#
# Usage: bash scripts/experiment1-full-bestofboth.sh
set -euo pipefail
cd "$(dirname "$0")/.."

BA3="${BA3:-../../batchalign3/target/release/batchalign3}"
TOOL="./target/release/utr-experiment"
TIMESTAMP=$(date +%Y%m%d-%H%M)
RESULTS_FILE="results/experiment1-bestofboth-${TIMESTAMP}.txt"
mkdir -p results

if [ ! -x "$BA3" ]; then
    echo "ERROR: batchalign3 not found at $BA3"
    echo "       Run: cd ../../batchalign3 && cargo build --release -p batchalign-cli"
    exit 1
fi

echo "=== Building utr-experiment tool ==="
cargo build --release

# --- Non-English files ---
# TaiwanHakka: audio names match input names
TAIWANHAKKA_FILES=(01 02 03 10 12)

# Multilang: audio names differ from input names
declare -A AUDIO_MAP
AUDIO_MAP[fusser12]="data/audio-biling/fusser12.mp3"
AUDIO_MAP[german050814]="data/audio-childes-intl/050814.mp3"
MULTILANG_FILES=(fusser12 german050814)

# Combined file list with language labels
declare -A LANG_MAP
for f in "${TAIWANHAKKA_FILES[@]}"; do LANG_MAP[$f]="hak"; done
LANG_MAP[fusser12]="cym"
LANG_MAP[german050814]="deu"

ALL_FILES=("${TAIWANHAKKA_FILES[@]}" "${MULTILANG_FILES[@]}")

# --- Alignment function ---
align_file() {
    local name="$1" strategy="$2" input_cha="$3" audio="$4" out_dir="$5"
    mkdir -p "$out_dir"

    local base ext tmpdir
    base=$(basename "$input_cha" .cha)
    ext="${audio##*.}"
    tmpdir=$(mktemp -d)

    cp "$input_cha" "$tmpdir/${name}.cha"
    ln -s "$(cd "$(dirname "$audio")" && pwd)/$(basename "$audio")" "$tmpdir/${name}.${ext}"

    echo "  Aligning $name ($strategy)..."
    "$BA3" --no-open-dashboard align "$tmpdir/${name}.cha" \
        -o "$out_dir" \
        --utr-strategy "$strategy" \
        -v 2>&1 | tail -3 || true

    rm -rf "$tmpdir"
}

# --- Count timed utterances for a single file ---
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

# --- Start server ---
echo ""
echo "Starting batchalign3 server (2 workers)..."
"$BA3" serve start --workers 2 2>/dev/null || true
sleep 3

# --- Run alignments ---
OUT_BASE="data/experiment1"

for strategy in global two-pass auto; do
    echo ""
    echo "=== Strategy: $strategy ==="

    for name in "${ALL_FILES[@]}"; do
        local_input=""
        local_audio=""

        # Determine input and audio paths
        if [[ " ${TAIWANHAKKA_FILES[*]} " == *" $name "* ]]; then
            local_input="data/taiwanhakka-input/${name}.cha"
            local_audio="data/audio-taiwanhakka/${name}.mp3"
        else
            local_input="data/multilang-input/${name}.cha"
            local_audio="${AUDIO_MAP[$name]}"
        fi

        if [ ! -f "$local_input" ]; then
            echo "  SKIP $name: input not found at $local_input"
            continue
        fi
        if [ ! -f "$local_audio" ]; then
            echo "  SKIP $name: audio not found at $local_audio"
            continue
        fi

        align_file "$name" "$strategy" "$local_input" "$local_audio" \
            "${OUT_BASE}/${strategy}/${name}"
    done
done

# --- Stop server ---
"$BA3" serve stop 2>/dev/null || true

# --- Collect results ---
echo ""
echo "=== Results ==="

{
    echo "========================================"
    echo "Experiment 1: Best-of-Both per Non-English File"
    echo "Date: $(date)"
    echo "========================================"
    echo ""
    printf "%-20s %-6s %-15s %-15s %-15s %-10s\n" \
        "file" "lang" "global" "two-pass" "auto" "winner"
    printf "%-20s %-6s %-15s %-15s %-15s %-10s\n" \
        "----" "----" "------" "--------" "----" "------"

    for name in "${ALL_FILES[@]}"; do
        lang="${LANG_MAP[$name]}"
        global_result=$(count_timed_file "${OUT_BASE}/global/${name}/${name}.cha")
        twopass_result=$(count_timed_file "${OUT_BASE}/two-pass/${name}/${name}.cha")
        auto_result=$(count_timed_file "${OUT_BASE}/auto/${name}/${name}.cha")

        # Determine winner by timed count
        g_timed=${global_result%%/*}
        t_timed=${twopass_result%%/*}
        a_timed=${auto_result%%/*}

        winner="tie"
        best=$g_timed
        winner_name="global"
        if [ "$t_timed" -gt "$best" ] 2>/dev/null; then
            best=$t_timed; winner_name="two-pass"
        fi
        if [ "$a_timed" -gt "$best" ] 2>/dev/null; then
            best=$a_timed; winner_name="auto"
        fi
        # Check for ties
        if [ "$g_timed" = "$t_timed" ] && [ "$t_timed" = "$a_timed" ] 2>/dev/null; then
            winner_name="tie"
        fi

        printf "%-20s %-6s %-15s %-15s %-15s %-10s\n" \
            "$name" "$lang" "$global_result" "$twopass_result" "$auto_result" "$winner_name"
    done

    echo ""
    echo "Ground truth coverage (for reference):"
    printf "%-20s %-6s %-15s\n" "file" "lang" "ground-truth"
    printf "%-20s %-6s %-15s\n" "----" "----" "------------"

    for name in "${ALL_FILES[@]}"; do
        lang="${LANG_MAP[$name]}"
        gt_file=""
        if [[ " ${TAIWANHAKKA_FILES[*]} " == *" $name "* ]]; then
            gt_file="data/taiwanhakka-groundtruth/${name}.cha"
        else
            gt_file="data/multilang-groundtruth/${name}.cha"
        fi
        gt_result=$(count_timed_file "$gt_file")
        printf "%-20s %-6s %-15s\n" "$name" "$lang" "$gt_result"
    done
} | tee "$RESULTS_FILE"

echo ""
echo "Results saved to: $RESULTS_FILE"
