#!/usr/bin/env bash
# Re-run the multilingual experiment with adaptive window two-pass.
#
# Algorithm change: instead of fixed ±500ms buffer, pass 2 now tries
# increasingly wider windows (±500ms, ±predecessor_duration, ±2x_duration)
# and accepts the first match. This should help non-English files where
# ASR timing is imprecise.
#
# Compares against the previous fixed-buffer results.
#
# Algorithm version: adaptive-window (2026-03-17)
# Previous version: fixed-500ms-buffer (2026-03-17, batchalign3 f02702e4)

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

# Re-run on the files that showed the worst non-English results
declare -A AUDIO_MAP
AUDIO_MAP[fusser12]="data/audio-biling/fusser12.mp3"
AUDIO_MAP[german050814]="data/audio-childes-intl/050814.mp3"
AUDIO_MAP[tbi_n22]="data/audio-tbi/n22.mp3"
AUDIO_MAP[tbi_tb23]="data/audio-tbi/tb23.mp3"

FILES=(fusser12 german050814 tbi_n22 tbi_tb23)

# Input directories already exist from previous run
# Just need to re-run two-pass with the new adaptive algorithm

echo ""
echo "=== Running two-pass with adaptive window ==="

for f in "${FILES[@]}"; do
    dir="data/multilang-individual/${f}"
    outdir="results/adaptive-experiment/two-pass/${f}"

    if [[ ! -d "$dir" ]]; then
        echo "SKIP: $dir not found"
        continue
    fi

    mkdir -p "$outdir"
    echo "  [two-pass-adaptive] ${dir} -> ${outdir}"

    if [[ "$DRY_RUN" == false ]]; then
        $BA3 align "$dir" -o "$outdir" --utr-strategy two-pass || {
            echo "    WARNING: align failed for $f"
        }
    fi
done

# Also re-run APROCSA 2265_T4 for English comparison
echo ""
echo "=== Re-running APROCSA 2265_T4 for English comparison ==="
for f in 2265_T4; do
    dir="data/converted-individual/${f}"
    outdir="results/adaptive-experiment/two-pass-aprocsa/${f}"
    mkdir -p "$outdir"
    echo "  [two-pass-adaptive] ${dir} -> ${outdir}"
    if [[ "$DRY_RUN" == false ]]; then
        $BA3 align "$dir" -o "$outdir" --utr-strategy two-pass || {
            echo "    WARNING: align failed for $f"
        }
    fi
done

# Measure
echo ""
echo "=== Coverage comparison ==="
echo "variant	file	speaker	total	timed	untimed	coverage"

echo "# Previous fixed-buffer results (from multilang-experiment):"
for f in "${FILES[@]}"; do
    for strat in global two-pass; do
        result="results/multilang-experiment/${strat}/${f}/${f}.cha"
        if [[ -f "$result" ]]; then
            $TOOL measure "$result" 2>/dev/null | while IFS= read -r line; do
                [[ "$line" != file* ]] && echo "prev-${strat}	${line}"
            done
        fi
    done
done

echo ""
echo "# New adaptive-window results:"
for f in "${FILES[@]}"; do
    result="results/adaptive-experiment/two-pass/${f}/${f}.cha"
    if [[ -f "$result" ]]; then
        $TOOL measure "$result" 2>/dev/null | while IFS= read -r line; do
            [[ "$line" != file* ]] && echo "adaptive	${line}"
        done
    fi
done

# APROCSA comparison
echo ""
echo "# APROCSA 2265_T4 comparison:"
for variant_dir in results/overlap-experiment/D-with-linker-twopass/2265_T4 results/adaptive-experiment/two-pass-aprocsa/2265_T4; do
    if [[ -d "$variant_dir" ]]; then
        result="$variant_dir/2265_T4.cha"
        if [[ -f "$result" ]]; then
            label=$(echo "$variant_dir" | sed 's|results/||;s|/2265_T4||')
            $TOOL measure "$result" 2>/dev/null | while IFS= read -r line; do
                [[ "$line" != file* ]] && echo "${label}	${line}"
            done
        fi
    fi
done

echo ""
echo "=== Adaptive window experiment complete ==="
