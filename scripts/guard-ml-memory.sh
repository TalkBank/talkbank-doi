#!/usr/bin/env bash
# Guard script: blocks ML-heavy commands when system memory is stressed
# or other ML workers are already running.
#
# Used as a Claude Code pre-command hook to prevent OOM kernel panics.
#
# Exit 0 = allow, exit 2 = block (with message on stderr).

set -euo pipefail

COMMAND="${1:-}"

# --- Only guard commands that spawn ML workers ---
is_ml_command() {
    case "$COMMAND" in
        *"cargo nextest"*|*"cargo test"*)
            # Only block if targeting batchalign crates (not talkbank-tools etc)
            [[ "$COMMAND" == *batchalign* ]] && return 0
            # cargo test in batchalign3 dir
            [[ "$COMMAND" == *"cd /Users/chen/talkbank/batchalign3"* ]] && return 0
            return 1
            ;;
        *"uv run pytest"*|*"uv run python"*|*pytest*)
            return 0
            ;;
        *batchalign3*transcribe*|*batchalign3*morphotag*|*batchalign3*align*|*batchalign3*utseg*|*batchalign3*benchmark*)
            return 0
            ;;
        *batchalignjan9*|*batchalign2-master*)
            return 0
            ;;
        *generate_ba2_golden*)
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

if ! is_ml_command; then
    exit 0
fi

# --- Check 1: Are ML workers already running? ---
ML_WORKER_COUNT=$(pgrep -f 'python3.*batchalign|python3.*whisper|python3.*stanza|python3.*torch' 2>/dev/null | wc -l | tr -d ' ')
if [ "$ML_WORKER_COUNT" -gt 2 ]; then
    echo "BLOCKED: $ML_WORKER_COUNT ML worker processes already running." >&2
    echo "Another Claude session or batchalign job is using ML models." >&2
    echo "Wait for it to finish, or kill workers: pkill -f 'python3.*batchalign'" >&2
    exit 2
fi

# --- Check 2: Memory pressure ---
# On macOS, check memory_pressure utility
if command -v memory_pressure &>/dev/null; then
    PRESSURE=$(memory_pressure 2>/dev/null | grep "System-wide memory free percentage" | grep -o '[0-9]*' || echo "100")
    if [ "${PRESSURE:-100}" -lt 20 ]; then
        echo "BLOCKED: System memory free is only ${PRESSURE}%." >&2
        echo "Running ML models now risks a kernel panic (OOM)." >&2
        exit 2
    fi
fi

# --- Check 3: Swap pressure (page-outs indicate thrashing) ---
PAGEOUTS=$(vm_stat 2>/dev/null | awk '/Pageouts/ {gsub(/\./,""); print $2}' || echo "0")
# This is cumulative since boot, so we can't use absolute values easily.
# Instead, check compressed memory vs total.
COMPRESSED=$(vm_stat 2>/dev/null | awk '/stored in compressor/ {gsub(/\./,""); print $2}' || echo "0")
# Each page is 16384 bytes on Apple Silicon
COMPRESSED_GB=$(echo "$COMPRESSED * 16384 / 1073741824" | bc 2>/dev/null || echo "0")
if [ "${COMPRESSED_GB:-0}" -gt 30 ]; then
    echo "BLOCKED: ${COMPRESSED_GB}GB in compressed memory — system is under heavy memory pressure." >&2
    exit 2
fi

exit 0
