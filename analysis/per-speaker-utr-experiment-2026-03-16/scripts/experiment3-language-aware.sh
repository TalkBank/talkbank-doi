#!/usr/bin/env bash
# Experiment 3: Language-aware strategy selection.
# Tests a language→strategy mapping to see if it eliminates non-English
# regressions while preserving English gains.
#
# TEMPLATE: Fill in STRATEGY_MAP after Experiment 2 results are known.
#
# Usage: bash scripts/experiment3-language-aware.sh
set -euo pipefail
cd "$(dirname "$0")/.."

BA3="${BA3:-../../batchalign3/target/release/batchalign3}"
TOOL="./target/release/utr-experiment"
TIMESTAMP=$(date +%Y%m%d-%H%M)
RESULTS_FILE="results/experiment3-language-aware-${TIMESTAMP}.txt"
mkdir -p results

if [ ! -x "$BA3" ]; then
    echo "ERROR: batchalign3 not found at $BA3"
    exit 1
fi

echo "=== Building utr-experiment tool ==="
cargo build --release

# --- Language → Strategy mapping ---
# Fill these in based on Experiment 2 WER results.
# High-WER languages (poor ASR) → global (safer)
# Low-WER languages (good ASR)  → auto (best-of-both)
declare -A STRATEGY_MAP
STRATEGY_MAP[eng]="auto"
STRATEGY_MAP[hak]="global"    # TODO: confirm from Exp 2
STRATEGY_MAP[cym]="global"    # TODO: confirm from Exp 2
STRATEGY_MAP[deu]="global"    # TODO: confirm from Exp 2
STRATEGY_MAP[fra]="global"    # TODO: confirm from Exp 2
STRATEGY_MAP[srp]="global"    # TODO: confirm from Exp 2

# --- File definitions ---
TAIWANHAKKA_FILES=(01 02 03 10 12)

declare -A AUDIO_MAP LANG_MAP INPUT_MAP
# TaiwanHakka
for f in "${TAIWANHAKKA_FILES[@]}"; do
    LANG_MAP[$f]="hak"
    INPUT_MAP[$f]="data/taiwanhakka-input/${f}.cha"
    AUDIO_MAP[$f]="data/audio-taiwanhakka/${f}.mp3"
done

# Multilang (non-English only)
LANG_MAP[fusser12]="cym";       INPUT_MAP[fusser12]="data/multilang-input/fusser12.cha";       AUDIO_MAP[fusser12]="data/audio-biling/fusser12.mp3"
LANG_MAP[german050814]="deu";   INPUT_MAP[german050814]="data/multilang-input/german050814.cha"; AUDIO_MAP[german050814]="data/audio-childes-intl/050814.mp3"

# SBCSAE (English)
SBCSAE_FILES=()
for cha in data/sbcsae-input/*.cha; do
    [ -f "$cha" ] || continue
    base=$(basename "$cha" .cha)
    SBCSAE_FILES+=("sbcsae_${base}")
    LANG_MAP["sbcsae_${base}"]="eng"
    INPUT_MAP["sbcsae_${base}"]="$cha"
    # Audio: find matching file
    for ext in mp3 mp4 wav; do
        if [ -f "data/audio-sbcsae/${base}.${ext}" ]; then
            AUDIO_MAP["sbcsae_${base}"]="data/audio-sbcsae/${base}.${ext}"
            break
        fi
    done
done

# Jefferson (English)
JEFFERSON_FILES=()
for cha in data/jefferson-input/*.cha; do
    [ -f "$cha" ] || continue
    base=$(basename "$cha" .cha)
    JEFFERSON_FILES+=("jeff_${base}")
    LANG_MAP["jeff_${base}"]="eng"
    INPUT_MAP["jeff_${base}"]="$cha"
    for ext in mp3 mp4 wav; do
        if [ -f "data/audio-jefferson/${base}.${ext}" ]; then
            AUDIO_MAP["jeff_${base}"]="data/audio-jefferson/${base}.${ext}"
            break
        fi
    done
done

ALL_FILES=("${TAIWANHAKKA_FILES[@]}" fusser12 german050814 "${SBCSAE_FILES[@]}" "${JEFFERSON_FILES[@]}")

# --- Alignment function ---
align_file() {
    local name="$1" strategy="$2" input_cha="$3" audio="$4" out_dir="$5"
    mkdir -p "$out_dir"
    local ext tmpdir
    ext="${audio##*.}"
    tmpdir=$(mktemp -d)

    cp "$input_cha" "$tmpdir/${name}.cha"
    ln -s "$(cd "$(dirname "$audio")" && pwd)/$(basename "$audio")" "$tmpdir/${name}.${ext}"

    "$BA3" --no-open-dashboard align "$tmpdir/${name}.cha" \
        -o "$out_dir" \
        --utr-strategy "$strategy" \
        -v 2>&1 | tail -2 || true

    rm -rf "$tmpdir"
}

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

# --- Run language-aware alignment ---
OUT_BASE="data/experiment3"
echo ""
echo "=== Running language-aware alignment ==="

for name in "${ALL_FILES[@]}"; do
    lang="${LANG_MAP[$name]}"
    strategy="${STRATEGY_MAP[$lang]:-global}"
    input="${INPUT_MAP[$name]:-}"
    audio="${AUDIO_MAP[$name]:-}"

    if [ -z "$input" ] || [ ! -f "$input" ]; then
        echo "  SKIP $name: input not found"
        continue
    fi
    if [ -z "$audio" ] || [ ! -f "$audio" ]; then
        echo "  SKIP $name: audio not found"
        continue
    fi

    echo "  $name (lang=$lang, strategy=$strategy)"
    align_file "$name" "$strategy" "$input" "$audio" "${OUT_BASE}/lang-aware/${name}"
done

# --- Stop server ---
"$BA3" serve stop 2>/dev/null || true

# --- Collect results ---
{
    echo "========================================"
    echo "Experiment 3: Language-Aware Strategy Selection"
    echo "Date: $(date)"
    echo "========================================"
    echo ""
    echo "Strategy mapping:"
    for lang in "${!STRATEGY_MAP[@]}"; do
        echo "  $lang → ${STRATEGY_MAP[$lang]}"
    done
    echo ""
    printf "%-25s %-6s %-10s %-15s\n" "file" "lang" "strategy" "lang-aware"
    printf "%-25s %-6s %-10s %-15s\n" "----" "----" "--------" "----------"

    # Group by corpus for readability
    echo ""
    echo "--- TaiwanHakka ---"
    for name in "${TAIWANHAKKA_FILES[@]}"; do
        lang="${LANG_MAP[$name]}"
        strategy="${STRATEGY_MAP[$lang]}"
        result=$(count_timed_file "${OUT_BASE}/lang-aware/${name}/${name}.cha")
        printf "%-25s %-6s %-10s %-15s\n" "$name" "$lang" "$strategy" "$result"
    done

    echo ""
    echo "--- Multilang (non-English) ---"
    for name in fusser12 german050814; do
        lang="${LANG_MAP[$name]}"
        strategy="${STRATEGY_MAP[$lang]}"
        result=$(count_timed_file "${OUT_BASE}/lang-aware/${name}/${name}.cha")
        printf "%-25s %-6s %-10s %-15s\n" "$name" "$lang" "$strategy" "$result"
    done

    echo ""
    echo "--- SBCSAE (English) ---"
    eng_total=0 eng_timed=0
    for name in "${SBCSAE_FILES[@]}"; do
        result=$(count_timed_file "${OUT_BASE}/lang-aware/${name}/${name}.cha")
        t=${result%%/*}; tot=${result##*/}
        eng_timed=$((eng_timed + t)); eng_total=$((eng_total + tot))
        printf "%-25s %-6s %-10s %-15s\n" "$name" "eng" "auto" "$result"
    done
    echo "  SBCSAE total: ${eng_timed}/${eng_total}"

    echo ""
    echo "--- Jefferson (English) ---"
    jeff_total=0 jeff_timed=0
    for name in "${JEFFERSON_FILES[@]}"; do
        result=$(count_timed_file "${OUT_BASE}/lang-aware/${name}/${name}.cha")
        t=${result%%/*}; tot=${result##*/}
        jeff_timed=$((jeff_timed + t)); jeff_total=$((jeff_total + tot))
        printf "%-25s %-6s %-10s %-15s\n" "$name" "eng" "auto" "$result"
    done
    echo "  Jefferson total: ${jeff_timed}/${jeff_total}"

    echo ""
    echo "Compare with Phase 6 results:"
    echo "  SBCSAE global:  7776/10540 (73.8%)"
    echo "  SBCSAE auto:    8226/10540 (78.0%)"
    echo "  Jefferson global: 2274/2561 (88.8%)"
    echo "  Jefferson auto:   2371/2561 (92.6%)"
    echo "  TaiwanHakka global: 2322/2661 (87.3%)"
    echo "  TaiwanHakka auto:   2246/2661 (84.4%)"
} | tee "$RESULTS_FILE"

echo ""
echo "Results saved to: $RESULTS_FILE"
