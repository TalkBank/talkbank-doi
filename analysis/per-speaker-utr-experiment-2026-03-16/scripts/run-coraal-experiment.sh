#!/usr/bin/env bash
# Run the CORAAL pure-&* conversion experiment.
#
# Same 4-condition design as APROCSA but on sociolinguistic interview data.
# These files have ONLY &* (no +<) — the purest conversion test.

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

# CORAAL files: short name -> corpus path
declare -A CHAT_MAP
CHAT_MAP[PRV_m02_1]="ca-data/CORAAL/PRV/se0-ag2/m_02_1.cha"
CHAT_MAP[PRV_f02_1]="ca-data/CORAAL/PRV/se0-ag3/f_02_1.cha"
CHAT_MAP[DTA_f02]="ca-data/CORAAL/DTA/se3-ag4/f_02.cha"

declare -A AUDIO_MAP
AUDIO_MAP[PRV_m02_1]="data/audio-coraal/m_02_1.mp3"
AUDIO_MAP[PRV_f02_1]="data/audio-coraal/f_02_1.mp3"
AUDIO_MAP[DTA_f02]="data/audio-coraal/f_02.mp3"

FILES=(PRV_m02_1 PRV_f02_1 DTA_f02)

# Step 1: Copy originals, strip timing, convert
echo ""
echo "=== Step 1: Prepare inputs ==="
mkdir -p data/coraal-input data/coraal-converted data/coraal-converted-no-linker

for f in "${FILES[@]}"; do
    corpus_path="${CHAT_MAP[$f]}"
    src="../../data/${corpus_path}"
    if [[ ! -f "$src" ]]; then
        echo "SKIP: $src not found"
        continue
    fi

    # Strip timing from corpus original
    if [[ "$DRY_RUN" == false ]]; then
        $TOOL strip "$src" "data/coraal-input/${f}.cha"
        $TOOL convert "data/coraal-input/${f}.cha" "data/coraal-converted/${f}.cha"
        $TOOL convert --no-linker "data/coraal-input/${f}.cha" "data/coraal-converted-no-linker/${f}.cha"
    else
        echo "  Would prepare ${f}"
    fi
done

# Step 2: Set up directories with audio symlinks
echo ""
echo "=== Step 2: Setting up directories ==="
for variant in coraal-input coraal-converted coraal-converted-no-linker; do
    for f in "${FILES[@]}"; do
        dir="data/${variant}-individual/${f}"
        mkdir -p "$dir"
        if [[ -f "data/${variant}/${f}.cha" ]]; then
            cp "data/${variant}/${f}.cha" "$dir/${f}.cha"
        fi
        audio="${AUDIO_MAP[$f]}"
        ext="${audio##*.}"
        if [[ -f "$audio" && ! -f "$dir/${f}.${ext}" ]]; then
            ln -sf "$(cd "$(dirname "$audio")" && pwd)/$(basename "$audio")" "$dir/${f}.${ext}"
        fi
    done
done

# Step 3: Run alignment (4 conditions)
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
    run_align "A-original" \
        "data/coraal-input-individual/${f}" \
        "results/coraal-experiment/A-original/${f}" "auto"
    run_align "B-no-linker-global" \
        "data/coraal-converted-no-linker-individual/${f}" \
        "results/coraal-experiment/B-no-linker-global/${f}" "auto"
    run_align "C-with-linker-global" \
        "data/coraal-converted-individual/${f}" \
        "results/coraal-experiment/C-with-linker-global/${f}" "global"
    run_align "D-with-linker-twopass" \
        "data/coraal-converted-individual/${f}" \
        "results/coraal-experiment/D-with-linker-twopass/${f}" "two-pass"
done

# Step 4: Measure
echo ""
echo "=== Step 4: Coverage ==="
echo "variant	file	speaker	total	timed	untimed	coverage"
for variant in A-original B-no-linker-global C-with-linker-global D-with-linker-twopass; do
    for f in "${FILES[@]}"; do
        result="results/coraal-experiment/${variant}/${f}/${f}.cha"
        if [[ -f "$result" ]]; then
            $TOOL measure "$result" 2>/dev/null | while IFS= read -r line; do
                [[ "$line" != file* ]] && echo "${variant}	${line}"
            done
        fi
    done
done

echo ""
echo "=== CORAAL experiment complete ==="
