#!/usr/bin/env bash
# Experiment 2: ASR WER per language.
# Runs batchalign3 benchmark on all non-English files with ground truth.
# Reports WER per file to understand ASR quality across languages.
#
# Usage: bash scripts/experiment2-asr-quality.sh
set -euo pipefail
cd "$(dirname "$0")/.."

BA3="${BA3:-../../batchalign3/target/release/batchalign3}"
TIMESTAMP=$(date +%Y%m%d-%H%M)
RESULTS_FILE="results/experiment2-asr-quality-${TIMESTAMP}.txt"
mkdir -p results

if [ ! -x "$BA3" ]; then
    echo "ERROR: batchalign3 not found at $BA3"
    echo "       Run: cd ../../batchalign3 && cargo build --release -p batchalign-cli"
    exit 1
fi

# --- File definitions ---
# TaiwanHakka files
TAIWANHAKKA_FILES=(01 02 03 10 12)

# Multilang files (all 7 have ground truth)
MULTILANG_FILES=(fusser12 german050814 mle28 patagonia30 serbian030005 tbi_n22 tbi_tb23)

# Audio map for name-mismatched files
declare -A AUDIO_MAP
AUDIO_MAP[fusser12]="data/audio-biling/fusser12.mp3"
AUDIO_MAP[german050814]="data/audio-childes-intl/050814.mp3"
AUDIO_MAP[mle28]="data/audio-biling/28.mp3"
AUDIO_MAP[patagonia30]="data/audio-biling/30.mp3"
AUDIO_MAP[serbian030005]="data/audio-childes-intl/030005.mp3"
AUDIO_MAP[tbi_n22]="data/audio-tbi/n22.mp3"
AUDIO_MAP[tbi_tb23]="data/audio-tbi/tb23.mp3"

# Language map (for --lang flag)
declare -A LANG_MAP
for f in "${TAIWANHAKKA_FILES[@]}"; do LANG_MAP[$f]="hak"; done
LANG_MAP[fusser12]="cym"
LANG_MAP[german050814]="deu"
LANG_MAP[mle28]="fra"
LANG_MAP[patagonia30]="cym"
LANG_MAP[serbian030005]="srp"
LANG_MAP[tbi_n22]="eng"
LANG_MAP[tbi_tb23]="eng"

ALL_FILES=("${TAIWANHAKKA_FILES[@]}" "${MULTILANG_FILES[@]}")

# --- Start server ---
echo "Starting batchalign3 server (2 workers)..."
"$BA3" serve start --workers 2 2>/dev/null || true
sleep 3

# --- Run benchmark per file ---
echo ""
echo "=== Running ASR benchmarks ==="

declare -A WER_RESULTS

for name in "${ALL_FILES[@]}"; do
    lang="${LANG_MAP[$name]}"

    # Determine ground truth and audio paths
    gt_file=""
    audio=""
    if [[ " ${TAIWANHAKKA_FILES[*]} " == *" $name "* ]]; then
        gt_file="data/taiwanhakka-groundtruth/${name}.cha"
        audio="data/audio-taiwanhakka/${name}.mp3"
    else
        gt_file="data/multilang-groundtruth/${name}.cha"
        audio="${AUDIO_MAP[$name]}"
    fi

    if [ ! -f "$gt_file" ]; then
        echo "  SKIP $name: ground truth not found at $gt_file"
        WER_RESULTS[$name]="n/a"
        continue
    fi
    if [ ! -f "$audio" ]; then
        echo "  SKIP $name: audio not found at $audio"
        WER_RESULTS[$name]="n/a"
        continue
    fi

    # Set up temp dir with matching names.
    # IMPORTANT: copy audio (not symlink) because benchmark resolves symlinks
    # and looks for the .cha reference next to the resolved audio path.
    tmpdir=$(mktemp -d)
    ext="${audio##*.}"
    cp "$gt_file" "$tmpdir/${name}.cha"
    cp "$audio" "$tmpdir/${name}.${ext}"

    echo "  Benchmarking $name (lang=$lang)..."
    output=$("$BA3" --no-open-dashboard benchmark "$tmpdir/${name}.${ext}" \
        --lang "$lang" -v 2>&1) || true

    # Extract WER — benchmark prints a results table with "WER" or percentage
    wer=$(echo "$output" | grep -iE 'wer|%|word error|accuracy' | grep -v 'Dashboard' | tail -1 || echo "")
    if [ -z "$wer" ]; then
        # Capture full output — look for the results summary line
        wer=$(echo "$output" | grep -E '(succeeded|RESULTS)' | head -1 || echo "(see log)")
        echo "$output" > "results/experiment2-log-${name}.txt"
    fi
    WER_RESULTS[$name]="$wer"
    echo "    $wer"

    rm -rf "$tmpdir"
done

# --- Stop server ---
"$BA3" serve stop 2>/dev/null || true

# --- Report ---
{
    echo "========================================"
    echo "Experiment 2: ASR Quality (WER) per Language"
    echo "Date: $(date)"
    echo "========================================"
    echo ""
    printf "%-20s %-6s %s\n" "file" "lang" "WER"
    printf "%-20s %-6s %s\n" "----" "----" "---"

    for name in "${ALL_FILES[@]}"; do
        lang="${LANG_MAP[$name]}"
        printf "%-20s %-6s %s\n" "$name" "$lang" "${WER_RESULTS[$name]}"
    done
} | tee "$RESULTS_FILE"

echo ""
echo "Results saved to: $RESULTS_FILE"
