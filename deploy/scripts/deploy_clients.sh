#!/usr/bin/env bash
# Deploy batchalign-next + batchalign-core to lab client machines via SSH.
#
# Builds fresh wheels (Python 3.12), deploys to client machines.
# (NOT Net — use deploy_server.sh for Net.)
#
# Usage:
#   bash deploy/scripts/deploy_clients.sh                    # build + deploy to all clients
#   bash deploy/scripts/deploy_clients.sh bilbo study        # build + deploy to specific machines
#   bash deploy/scripts/deploy_clients.sh --dry-run          # build wheels, show what would deploy
#   bash deploy/scripts/deploy_clients.sh --no-build         # skip wheel build (use existing)

set -euo pipefail

PYTHON_VERSION="3.12"
SSH_USER="macw"
ALL_HOSTS=(bilbo brian davida frodo andrew lilly sue vaishnavi)

BA_NEXT_REPO="$HOME/batchalign-next"
CORE_REPO="$HOME/talkbank-utils/rust"
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
    echo "  Building batchalign-next wheel..."
    (cd "$BA_NEXT_REPO" && uv build --wheel --quiet)

    # 2. Rust core wheel (batchalign-core from batchalign3 repo)
    echo "  Building batchalign-core wheel (Python $PYTHON_VERSION)..."
    PYTHON_BIN="$(uv python find "$PYTHON_VERSION" 2>/dev/null || true)"
    if [[ -z "$PYTHON_BIN" ]]; then
        echo "  Installing Python $PYTHON_VERSION..."
        uv python install "$PYTHON_VERSION"
        PYTHON_BIN="$(uv python find "$PYTHON_VERSION")"
    fi
    (cd "$CORE_REPO" && maturin build --release -m "$CORE_CARGO" -i "$PYTHON_BIN" 2>&1 | tail -3)

    echo ""
fi

# --- Discover wheel paths (glob for latest) ---

BA_NEXT_WHEEL="$(ls -t "$BA_NEXT_REPO"/dist/batchalign_next-*-py3-none-any.whl 2>/dev/null | head -1)"

CP_TAG="cp${PYTHON_VERSION//./}"
BA_CORE_WHEEL="$(ls -t "$HOME"/talkbank-utils/target/wheels/batchalign_core-*-${CP_TAG}-${CP_TAG}-macosx_*.whl 2>/dev/null | head -1)"

for whl in "$BA_NEXT_WHEEL" "$BA_CORE_WHEEL"; do
    if [[ -z "$whl" || ! -f "$whl" ]]; then
        echo "ERROR: Wheel not found."
        echo "  batchalign-next:   $BA_NEXT_WHEEL"
        echo "  batchalign-core:   $BA_CORE_WHEEL"
        echo "Run without --no-build to build fresh wheels."
        exit 1
    fi
done

BA_NEXT_NAME="$(basename "$BA_NEXT_WHEEL")"
BA_CORE_NAME="$(basename "$BA_CORE_WHEEL")"

echo "Deploying to: ${HOSTS[*]}"
echo "  batchalign-next:   $BA_NEXT_NAME"
echo "  batchalign-core:   $BA_CORE_NAME  (Python $PYTHON_VERSION)"
echo ""

if $DRY_RUN; then
    echo "=== DRY RUN ==="
    for host in "${HOSTS[@]}"; do
        echo "  [$host] Would scp 2 wheels, install (Python $PYTHON_VERSION)"
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

    # Copy wheels
    echo "  Copying wheels..."
    scp -q "$BA_NEXT_WHEEL" "$BA_CORE_WHEEL" "$SSH_USER@$host:/tmp/"

    # Remove old installations (including 3.14t and sidecar remnants)
    echo "  Removing old installations..."
    ssh "$SSH_USER@$host" "
        uv tool uninstall batchalign-next 2>/dev/null || true
        uv tool uninstall batchalign 2>/dev/null || true
        uv tool uninstall batchalignhk 2>/dev/null || true
        rm -rf ~/.batchalign3/sidecar/ 2>/dev/null || true
        rm -f ~/.batchalign3/sidecar-daemon.json ~/.batchalign3/sidecar-daemon.log 2>/dev/null || true
    " 2>&1 | grep -v "is not installed" || true

    # Install (Python 3.12)
    echo "  Ensuring Python $PYTHON_VERSION on $host..."
    ssh "$SSH_USER@$host" "uv python install $PYTHON_VERSION 2>/dev/null || true"

    echo "  Installing batchalign-next (Python $PYTHON_VERSION)..."
    ssh "$SSH_USER@$host" "
        uv tool install --python $PYTHON_VERSION --force-reinstall \
            /tmp/$BA_NEXT_NAME \
            --with /tmp/$BA_CORE_NAME 2>&1
    " | tail -5

    # Kill stale daemons
    echo "  Stopping old daemons..."
    ssh "$SSH_USER@$host" '
        batchalign-next serve stop 2>/dev/null || true
        batchalign3 serve stop 2>/dev/null || true
        sleep 1
        pkill -9 -f "batchalign-next serve" 2>/dev/null || true
        pkill -9 -f "batchalign3 serve" 2>/dev/null || true
        pkill -9 -f "batchalign.serve.run" 2>/dev/null || true
        sleep 1
    ' 2>/dev/null || true

    # Verification
    echo "  Verifying..."
    main_ok=$(ssh "$SSH_USER@$host" '
        batchalign-next --help >/dev/null 2>&1 && \
        ~/.local/share/uv/tools/batchalign-next/bin/python -c "
from batchalign.pipelines.context import ProcessingContext
print(\"ok\")
" 2>&1' || echo "FAILED")

    echo "    $main_ok"

    if [[ "$main_ok" == *"ok"* ]]; then
        echo "  OK: client verified"
        SUCCESSES=$((SUCCESSES + 1))
    else
        echo "  WARNING: verification failed"
        FAILURES=$((FAILURES + 1))
    fi

    # Clean up
    ssh "$SSH_USER@$host" "rm -f /tmp/batchalign_*.whl /tmp/batchalign3-*.whl" 2>/dev/null || true

    echo ""
done

echo "==============================="
echo "Done: $SUCCESSES succeeded, $FAILURES failed"
echo "==============================="
