#!/usr/bin/env bash
# Guard script: blocks LOCAL ML-heavy commands when system memory is stressed
# or other LOCAL ML workers are already running (cross-session protection).
#
# Called as a Claude Code PreToolUse hook on Bash.
# Reads JSON from stdin: {"tool_name":"Bash","tool_input":{"command":"..."}}
#
# Exit 0 with no output = allow.
# Exit 0 with {"decision":"block",...} = block.

set -uo pipefail

COMMAND=$(jq -r '.tool_input.command // ""' 2>/dev/null || echo "")
[ -z "$COMMAND" ] && exit 0

# --- Skip remote commands (ssh) — they don't use local RAM ---
case "$COMMAND" in
    ssh*|scp*) exit 0 ;;
esac

# --- Only guard LOCAL commands that spawn ML workers ---
case "$COMMAND" in
    *batchalign3*transcribe*|*batchalign3*morphotag*|*batchalign3*align*|*batchalign3*utseg*|*batchalign3*benchmark*)
        ;;
    *batchalignjan9*|*batchalign2-master*)
        ;;
    *generate_ba2_golden*)
        ;;
    *)
        exit 0
        ;;
esac

# --- Check 1: Are LOCAL ML workers already running? ---
# Use ps + grep instead of pgrep -f to avoid matching ssh/grep/ourselves.
# Look for actual python3.12 processes whose command starts with python3
# (not ssh commands that happen to contain "python3" in their args).
ML_WORKER_COUNT=$(ps -eo comm,args 2>/dev/null \
    | grep '^python3' \
    | grep -c -E 'batchalign\.worker|batchalign\.inference|whisper|stanza')
ML_WORKER_COUNT="${ML_WORKER_COUNT:-0}"
ML_WORKER_COUNT="${ML_WORKER_COUNT// /}"
if [ "${ML_WORKER_COUNT:-0}" -gt 2 ]; then
    jq -n --arg reason "BLOCKED: $ML_WORKER_COUNT local ML workers running. Another session is using ML models — wait or kill: pkill -f 'batchalign.worker'" \
        '{"decision":"block","reason":$reason}'
    exit 0
fi

# --- Check 2: Memory pressure ---
if command -v memory_pressure &>/dev/null; then
    PRESSURE=$(memory_pressure 2>/dev/null | grep "System-wide memory free percentage" | grep -o '[0-9]*' || echo "100")
    if [ "${PRESSURE:-100}" -lt 20 ]; then
        jq -n --arg reason "BLOCKED: Only ${PRESSURE}% memory free. ML models risk kernel OOM panic." \
            '{"decision":"block","reason":$reason}'
        exit 0
    fi
fi

# --- Check 3: Compressed memory (thrashing indicator) ---
COMPRESSED=$(vm_stat 2>/dev/null | awk '/stored in compressor/ {gsub(/\./,""); print $2}' || echo "0")
COMPRESSED_GB=$(echo "${COMPRESSED:-0} * 16384 / 1073741824" | bc 2>/dev/null || echo "0")
if [ "${COMPRESSED_GB:-0}" -gt 30 ]; then
    jq -n --arg reason "BLOCKED: ${COMPRESSED_GB}GB compressed memory — system is thrashing." \
        '{"decision":"block","reason":$reason}'
    exit 0
fi

exit 0
