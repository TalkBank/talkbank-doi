#!/usr/bin/env bash
# Deploy batchalign3 + batchalign-core to lab client machines via SSH.
#
# Builds fresh wheels (3.14t main + 3.12 sidecar), then deploys to client machines
# (NOT Net — use deploy_server.sh for Net).
#
# Each machine gets:
#   1. Main tool (Python 3.14t, free-threaded) — morphotag, align, etc.
#   2. Sidecar venv (Python 3.12) at ~/.batchalign3/sidecar/.venv/ — transcribe
#
# Usage:
#   bash scripts/deploy_clients.sh                    # build + deploy to all clients
#   bash scripts/deploy_clients.sh bilbo study        # build + deploy to specific machines
#   bash scripts/deploy_clients.sh --dry-run          # build wheels, show what would deploy
#   bash scripts/deploy_clients.sh --no-build         # skip wheel build (use existing)

set -euo pipefail

PYTHON_VERSION="3.14t"
SIDECAR_PYTHON="3.12"
SSH_USER="macw"
ALL_HOSTS=(bilbo brian davida frodo andrew lilly sue vaishnavi)

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CORE_REPO="$REPO_ROOT/rust"
CORE_CARGO="$CORE_REPO/crates/batchalign-core/Cargo.toml"

DRY_RUN=false
NO_BUILD=false

# Parse flags
HOSTS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)      DRY_RUN=true; shift ;;
        --no-build)     NO_BUILD=true; shift ;;
        *)              HOSTS+=("$1"); shift ;;
    esac
done

# Default to all hosts if none specified
if [[ ${#HOSTS[@]} -eq 0 ]]; then
    HOSTS=("${ALL_HOSTS[@]}")
fi

# --- Build wheels ---

if ! $NO_BUILD; then
    echo "=== Building wheels ==="

    # 1. Pure Python wheel (version-agnostic)
    echo "  Building batchalign3 wheel..."
    (cd "$REPO_ROOT" && uv build --wheel --quiet)

    # 2. Rust core for 3.14t (main)
    echo "  Building batchalign-core wheel (Python $PYTHON_VERSION)..."
    PYTHON_314T="$(uv python find "$PYTHON_VERSION" 2>/dev/null || true)"
    if [[ -z "$PYTHON_314T" ]]; then
        echo "  Installing Python $PYTHON_VERSION..."
        uv python install "$PYTHON_VERSION"
        PYTHON_314T="$(uv python find "$PYTHON_VERSION")"
    fi
    (cd "$CORE_REPO" && maturin build --release -m "$CORE_CARGO" -i "$PYTHON_314T" 2>&1 | tail -3)

    # 3. Rust core for 3.12 (sidecar)
    echo "  Building batchalign-core wheel (Python $SIDECAR_PYTHON, sidecar)..."
    (cd "$CORE_REPO" && maturin build --release -m "$CORE_CARGO" -i "python$SIDECAR_PYTHON" 2>&1 | tail -3)

    echo ""
fi

# --- Discover wheel paths (glob for latest) ---

BA_NEXT_WHEEL="$(ls -t "$REPO_ROOT"/dist/batchalign3-*-py3-none-any.whl 2>/dev/null | head -1)"

# cp314t wheel (main) — derive tag: "3.14t" → "cp314"
CP_314T="cp${PYTHON_VERSION//[^0-9]/}"
BA_CORE_314T_WHEEL="$(ls -t "$CORE_REPO"/target/wheels/batchalign_core-*-${CP_314T}*-macosx_*.whl 2>/dev/null | head -1)"

# cp312 wheel (sidecar)
CP_312="cp${SIDECAR_PYTHON//./}"
BA_CORE_312_WHEEL="$(ls -t "$CORE_REPO"/target/wheels/batchalign_core-*-${CP_312}-${CP_312}-macosx_*.whl 2>/dev/null | head -1)"

for whl in "$BA_NEXT_WHEEL" "$BA_CORE_314T_WHEEL" "$BA_CORE_312_WHEEL"; do
    if [[ -z "$whl" || ! -f "$whl" ]]; then
        echo "ERROR: Wheel not found."
        echo "  batchalign3:       $BA_NEXT_WHEEL"
        echo "  batchalign-core 3.14t: $BA_CORE_314T_WHEEL"
        echo "  batchalign-core 3.12:  $BA_CORE_312_WHEEL"
        echo "Run without --no-build to build fresh wheels."
        exit 1
    fi
done

BA_NEXT_NAME="$(basename "$BA_NEXT_WHEEL")"
BA_CORE_314T_NAME="$(basename "$BA_CORE_314T_WHEEL")"
BA_CORE_312_NAME="$(basename "$BA_CORE_312_WHEEL")"

echo "Deploying to: ${HOSTS[*]}"
echo "  batchalign3:       $BA_NEXT_NAME"
echo "  batchalign-core 3.14t: $BA_CORE_314T_NAME  (main)"
echo "  batchalign-core 3.12:  $BA_CORE_312_NAME  (sidecar)"
echo ""

if $DRY_RUN; then
    echo "=== DRY RUN ==="
    for host in "${HOSTS[@]}"; do
        echo "  [$host] Would scp 3 wheels, install main (3.14t), create sidecar (3.12)"
    done
    exit 0
fi

SUCCESSES=0
FAILURES=0

for host in "${HOSTS[@]}"; do
    echo "=== $host ==="

    # Check connectivity
    if ! ssh -o ConnectTimeout=5 "$SSH_USER@$host" true 2>/dev/null; then
        echo "  SKIP: cannot connect"
        FAILURES=$((FAILURES + 1))
        echo ""
        continue
    fi

    # Copy all three wheels
    echo "  Copying wheels..."
    scp -q "$BA_NEXT_WHEEL" "$BA_CORE_314T_WHEEL" "$BA_CORE_312_WHEEL" "$SSH_USER@$host:/tmp/"

    # Remove old installations
    echo "  Removing old uv tool installations..."
    ssh "$SSH_USER@$host" "
        uv tool uninstall batchalign3 2>/dev/null || true
        uv tool uninstall batchalign 2>/dev/null || true
        uv tool uninstall batchalignhk 2>/dev/null || true
    " 2>&1 | grep -v "is not installed" || true

    # --- Phase 1: Main tool (Python 3.14t, no ASR extras) ---
    echo "  Ensuring Python $PYTHON_VERSION on $host..."
    ssh "$SSH_USER@$host" "uv python install $PYTHON_VERSION 2>/dev/null || true"

    echo "  Installing main tool (Python $PYTHON_VERSION)..."
    ssh "$SSH_USER@$host" "
        uv tool install --python $PYTHON_VERSION --force-reinstall \
            /tmp/$BA_NEXT_NAME \
            --with /tmp/$BA_CORE_314T_NAME 2>&1
    " | tail -5

    # --- Phase 2: Sidecar venv (Python 3.12, ASR extras) ---
    echo "  Setting up transcribe sidecar (Python $SIDECAR_PYTHON)..."
    ssh "$SSH_USER@$host" "uv python install $SIDECAR_PYTHON 2>/dev/null || true"
    ssh "$SSH_USER@$host" "
        uv venv ~/.batchalign3/sidecar/.venv --python $SIDECAR_PYTHON 2>/dev/null || true
        uv pip install --python ~/.batchalign3/sidecar/.venv/bin/python \
            /tmp/$BA_NEXT_NAME /tmp/$BA_CORE_312_NAME \
            openai-whisper 'numba>=0.61.0' 'llvmlite>=0.44.0' 2>&1
    " | tail -5

    # --- Phase 3: Kill stale daemons (both main + sidecar) ---
    echo "  Stopping old daemons..."
    ssh "$SSH_USER@$host" '
        batchalign3 serve stop 2>/dev/null || true
        sleep 1
        pkill -9 -f "batchalign3 serve" 2>/dev/null || true
        pkill -9 -f "batchalign.serve.run" 2>/dev/null || true
        sleep 1
    ' 2>/dev/null || true

    # --- Verification ---
    echo "  Verifying main (3.14t)..."
    main_ok=$(ssh "$SSH_USER@$host" '
        batchalign3 --help >/dev/null 2>&1 && \
        ~/.local/share/uv/tools/batchalign3/bin/python -c "
import sys, batchalign.runtime as r
assert hasattr(sys, \"_is_gil_enabled\"), \"not 3.14t\"
print(\"main:ok free_threaded=\" + str(r.FREE_THREADED))
" 2>&1' || echo "FAILED")

    echo "    $main_ok"

    echo "  Verifying sidecar (3.12)..."
    sidecar_ok=$(ssh "$SSH_USER@$host" '
        ~/.batchalign3/sidecar/.venv/bin/python -c "
import whisper
from batchalign.pipelines.context import ProcessingContext
print(\"sidecar:ok\")
" 2>&1' || echo "FAILED")

    echo "    $sidecar_ok"

    if [[ "$main_ok" == *"main:ok"* && "$sidecar_ok" == *"sidecar:ok"* ]]; then
        echo "  OK: client fully verified"
        SUCCESSES=$((SUCCESSES + 1))
    else
        echo "  WARNING: verification failed"
        FAILURES=$((FAILURES + 1))
    fi

    # Clean up
    ssh "$SSH_USER@$host" "rm -f /tmp/batchalign_*.whl" 2>/dev/null || true

    echo ""
done

echo "==============================="
echo "Done: $SUCCESSES succeeded, $FAILURES failed"
echo "==============================="
