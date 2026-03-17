#!/usr/bin/env bash
# Run ground-truth experiments on multilingual +< files.
#
# Strip timing, re-align with both strategies, compare against original.
# Tests: bilingual (fra-eng, cym-eng, cym-eng-spa), Serbian, German, TBI.

set -euo pipefail
cd "$(dirname "$0")/.."

DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
    echo "DRY RUN"
fi

BA3="${BA3:-../../batchalign3/target/release/batchalign3}"
TOOL="./target/release/utr-experiment"

echo "=== Building tools ==="
cargo build --release
(cd ../../batchalign3 && cargo build --release -p batchalign-cli)

# File map: short name -> (corpus path, audio dir, audio filename)
declare -A CHAT_MAP AUDIO_MAP
CHAT_MAP[mle28]="biling-data/MLE-MPF/28.cha"
CHAT_MAP[fusser12]="biling-data/Bangor/Siarad/fusser12.cha"
CHAT_MAP[patagonia30]="biling-data/Bangor/Patagonia/30.cha"
CHAT_MAP[serbian030005]="childes-data/Slavic/Serbian/SCECL/AndjelaNj/030005.cha"
CHAT_MAP[german050814]="childes-data/German/Rigol/Sebastian/050814.cha"
CHAT_MAP[tbi_n22]="tbi-data/English/Coelho/N/n22.cha"
CHAT_MAP[tbi_tb23]="tbi-data/English/Coelho/TB/tb23.cha"

AUDIO_MAP[mle28]="data/audio-biling/28.mp3"
AUDIO_MAP[fusser12]="data/audio-biling/fusser12.mp3"
AUDIO_MAP[patagonia30]="data/audio-biling/30.mp3"
AUDIO_MAP[serbian030005]="data/audio-childes-intl/030005.mp3"
AUDIO_MAP[german050814]="data/audio-childes-intl/050814.mp3"
AUDIO_MAP[tbi_n22]="data/audio-tbi/n22.mp3"
AUDIO_MAP[tbi_tb23]="data/audio-tbi/tb23.mp3"

FILES=(mle28 fusser12 patagonia30 serbian030005 german050814 tbi_n22 tbi_tb23)

# Step 1: Copy originals as ground truth, strip timing
echo ""
echo "=== Step 1: Prepare ground truth and stripped inputs ==="
mkdir -p data/multilang-groundtruth data/multilang-stripped

for f in "${FILES[@]}"; do
    corpus_path="${CHAT_MAP[$f]}"
    src="../../data/${corpus_path}"
    if [[ ! -f "$src" ]]; then
        echo "SKIP: $src not found"
        continue
    fi
    cp "$src" "data/multilang-groundtruth/${f}.cha"
    if [[ "$DRY_RUN" == false ]]; then
        $TOOL strip "data/multilang-groundtruth/${f}.cha" "data/multilang-stripped/${f}.cha"
    else
        echo "  Would strip ${f}"
    fi
done

# Step 2: Set up directories with audio symlinks
echo ""
echo "=== Step 2: Setting up directories ==="
for f in "${FILES[@]}"; do
    dir="data/multilang-individual/${f}"
    mkdir -p "$dir"
    if [[ -f "data/multilang-stripped/${f}.cha" ]]; then
        cp "data/multilang-stripped/${f}.cha" "$dir/${f}.cha"
    fi
    audio="${AUDIO_MAP[$f]}"
    ext="${audio##*.}"
    if [[ -f "$audio" && ! -f "$dir/${f}.${ext}" ]]; then
        ln -sf "$(cd "$(dirname "$audio")" && pwd)/$(basename "$audio")" "$dir/${f}.${ext}"
    fi
done

# Step 3: Align with both strategies
echo ""
echo "=== Step 3: Running alignment ==="

run_align() {
    local label="$1" input_dir="$2" output_dir="$3" strategy="$4"
    mkdir -p "$output_dir"
    echo "  [$label] ${input_dir} -> ${output_dir} (strategy: ${strategy})"
    if [[ "$DRY_RUN" == false ]]; then
        $BA3 align "$input_dir" -o "$output_dir" --utr-strategy "$strategy" || {
            echo "    WARNING: align failed for $label"
        }
    fi
}

for f in "${FILES[@]}"; do
    echo ""
    echo "--- ${f} ---"
    run_align "global" \
        "data/multilang-individual/${f}" \
        "results/multilang-experiment/global/${f}" "global"
    run_align "two-pass" \
        "data/multilang-individual/${f}" \
        "results/multilang-experiment/two-pass/${f}" "two-pass"
done

# Step 4: Measure
echo ""
echo "=== Step 4: Coverage ==="
echo "strategy	file	speaker	total	timed	untimed	coverage"

for f in "${FILES[@]}"; do
    if [[ -f "data/multilang-groundtruth/${f}.cha" ]]; then
        $TOOL measure "data/multilang-groundtruth/${f}.cha" 2>/dev/null | while IFS= read -r line; do
            [[ "$line" != file* ]] && echo "ground-truth	${line}"
        done
    fi
done

for strategy in global two-pass; do
    for f in "${FILES[@]}"; do
        result="results/multilang-experiment/${strategy}/${f}/${f}.cha"
        if [[ -f "$result" ]]; then
            $TOOL measure "$result" 2>/dev/null | while IFS= read -r line; do
                [[ "$line" != file* ]] && echo "${strategy}	${line}"
            done
        fi
    done
done

echo ""
echo "=== Multilang experiment complete ==="
