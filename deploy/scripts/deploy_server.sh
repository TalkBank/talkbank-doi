#!/usr/bin/env bash
# Deploy batchalign-next[serve] to server machines via SSH.
#
# By default deploys to Net only (production). Can deploy to any/all machines.
# Installs under Python 3.12 with all extras (including whisper for transcribe).
#
# Usage:
#   bash deploy/scripts/deploy_server.sh                # build + deploy to Net
#   bash deploy/scripts/deploy_server.sh bilbo brian     # deploy to specific hosts
#   bash deploy/scripts/deploy_server.sh --all           # deploy to all fleet machines
#   bash deploy/scripts/deploy_server.sh --dry-run       # build wheels, show what would deploy
#   bash deploy/scripts/deploy_server.sh --no-build      # skip wheel build (use existing)

set -euo pipefail

PYTHON_VERSION="3.12"
SSH_USER="macw"
DEFAULT_HOST="net"
ALL_FLEET_HOSTS=(net bilbo brian davida frodo andrew lilly sue vaishnavi)

WORKSPACE_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BA_NEXT_REPO="$HOME/batchalign-next"
BA3_REPO="$WORKSPACE_ROOT/batchalign3"
CORE_CARGO="$BA3_REPO/pyo3/Cargo.toml"

DRY_RUN=false
NO_BUILD=false
DEPLOY_ALL=false

# Parse flags and hosts
HOSTS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)      DRY_RUN=true; shift ;;
        --no-build)     NO_BUILD=true; shift ;;
        --all)          DEPLOY_ALL=true; shift ;;
        --help|-h)
            echo "Usage: bash deploy/scripts/deploy_server.sh [OPTIONS] [HOST...]"
            echo ""
            echo "Options:"
            echo "  --all        Deploy to all fleet machines (${ALL_FLEET_HOSTS[*]})"
            echo "  --no-build   Skip wheel build (use existing)"
            echo "  --dry-run    Show what would deploy without doing it"
            echo ""
            echo "Without hosts: deploys to Net only."
            echo "With hosts: deploys to specified machines."
            exit 0
            ;;
        *)              HOSTS+=("$1"); shift ;;
    esac
done

# Determine target hosts
if $DEPLOY_ALL; then
    HOSTS=("${ALL_FLEET_HOSTS[@]}")
elif [[ ${#HOSTS[@]} -eq 0 ]]; then
    HOSTS=("$DEFAULT_HOST")
fi

# --- Build wheels ---

if ! $NO_BUILD; then
    echo "=== Building wheels ==="

    # 1. Pure Python wheel (batchalign-next)
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
    (cd "$BA3_REPO" && maturin build --release -m "$CORE_CARGO" -i "$PYTHON_BIN" 2>&1 | tail -3)

    echo ""
fi

# --- Discover wheel paths (glob for latest) ---

BA_NEXT_WHEEL="$(ls -t "$BA_NEXT_REPO"/dist/batchalign_next-*-py3-none-any.whl 2>/dev/null | head -1)"

CP_TAG="cp${PYTHON_VERSION//./}"
BA_CORE_WHEEL="$(ls -t "$BA3_REPO"/pyo3/target/wheels/batchalign3-*-${CP_TAG}-${CP_TAG}-macosx_*.whl 2>/dev/null | head -1)"

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

echo "Deploying batchalign-next[serve] to: ${HOSTS[*]}"
echo "  batchalign-next:   $BA_NEXT_NAME"
echo "  batchalign-core:   $BA_CORE_NAME  (Python $PYTHON_VERSION)"
echo ""

if $DRY_RUN; then
    echo "=== DRY RUN ==="
    for host in "${HOSTS[@]}"; do
        echo "  [$host] Would scp 2 wheels, install (Python $PYTHON_VERSION), restart server"
    done
    exit 0
fi

# --- Deploy to each host ---

FAILED_HOSTS=()
OK_HOSTS=()

for host in "${HOSTS[@]}"; do
    echo ""
    echo "=== Deploying to $host ==="

    # Check connectivity
    if ! ssh -o ConnectTimeout=5 "$SSH_USER@$host" true 2>/dev/null; then
        echo "  ERROR: Cannot connect to $host — skipping"
        FAILED_HOSTS+=("$host")
        continue
    fi

    # Copy wheels
    echo "  Copying wheels..."
    scp -q "$BA_NEXT_WHEEL" "$BA_CORE_WHEEL" "$SSH_USER@$host:/tmp/"

    # Stop the server before installing
    echo "  Stopping server..."
    ssh "$SSH_USER@$host" '
        batchalign-next serve stop 2>/dev/null || true
        batchalign3 serve stop 2>/dev/null || true
        sleep 1
        pkill -9 -f "batchalign-next serve" 2>/dev/null || true
        pkill -9 -f "batchalign3 serve" 2>/dev/null || true
        sleep 1
    '

    # Remove old installations (including 3.14t and sidecar remnants)
    echo "  Removing old installations..."
    ssh "$SSH_USER@$host" "
        uv tool uninstall batchalign3 2>/dev/null || true
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
            --with /tmp/$BA_CORE_NAME \
            --with openai-whisper \
            --with 'numba>=0.61.0' \
            --with 'llvmlite>=0.44.0' 2>&1
    " | tail -5

    # Verify installation
    echo "  Verifying installation..."
    version=$(ssh "$SSH_USER@$host" \
        'batchalign-next --help >/dev/null 2>&1 && \
         ~/.local/share/uv/tools/batchalign-next/bin/python -c "
from batchalign.pipelines.context import ProcessingContext
print(\"ok\")
" 2>&1' || echo "FAILED")

    if [[ "$version" != "ok" ]]; then
        echo "  ERROR: Installation verification failed on $host: $version"
        FAILED_HOSTS+=("$host")
        continue
    fi
    echo "  OK: Installation verified"

    # Generate server.yaml if it doesn't exist
    echo "  Checking server.yaml..."
    ssh "$SSH_USER@$host" '
        CONFIG_DIR="$HOME/.batchalign-next"
        CONFIG="$CONFIG_DIR/server.yaml"
        if [ ! -f "$CONFIG" ]; then
            mkdir -p "$CONFIG_DIR"
            cat > "$CONFIG" <<YAML
# Auto-generated by deploy_server.sh
default_lang: eng
port: 8000
warmup: true
job_ttl_days: 7
media_roots: []
media_mappings: {}
YAML
            echo "    Created default server.yaml"
        else
            echo "    server.yaml already exists"
        fi
    '

    # Start the server
    echo "  Starting server..."
    ssh "$SSH_USER@$host" 'batchalign-next serve start'

    # Verify server health (retry up to 3 minutes — warmup loads models)
    echo "  Checking server health (warmup may take a few minutes)..."
    HEALTH_TIMEOUT=180
    HEALTH_INTERVAL=5
    elapsed=0
    health=""
    while [ "$elapsed" -lt "$HEALTH_TIMEOUT" ]; do
        sleep "$HEALTH_INTERVAL"
        elapsed=$((elapsed + HEALTH_INTERVAL))
        health=$(ssh "$SSH_USER@$host" 'curl -s http://localhost:8000/health 2>/dev/null' || echo '{}')
        if echo "$health" | grep -q '"status":"ok"'; then
            echo "  OK: Server healthy on $host (ready after ${elapsed}s)"
            break
        fi
        printf "  Waiting... (%ds/%ds)\r" "$elapsed" "$HEALTH_TIMEOUT"
    done

    if ! echo "$health" | grep -q '"status":"ok"'; then
        echo "  WARNING: Server health check failed on $host after ${HEALTH_TIMEOUT}s"
        echo "  Check logs: ssh $SSH_USER@$host 'tail -50 ~/.batchalign-next/server.log'"
        FAILED_HOSTS+=("$host")
        continue
    fi

    # Clean up
    ssh "$SSH_USER@$host" "rm -f /tmp/batchalign_*.whl /tmp/batchalign3-*.whl" 2>/dev/null || true

    OK_HOSTS+=("$host")
done

# --- Summary ---
echo ""
echo "==============================="
if [[ ${#FAILED_HOSTS[@]} -eq 0 ]]; then
    echo "SUCCESS: All ${#OK_HOSTS[@]} server(s) deployed and healthy"
else
    echo "PARTIAL: ${#OK_HOSTS[@]} succeeded, ${#FAILED_HOSTS[@]} failed"
    echo "  OK:     ${OK_HOSTS[*]:-none}"
    echo "  FAILED: ${FAILED_HOSTS[*]}"
fi
echo "==============================="

[[ ${#FAILED_HOSTS[@]} -eq 0 ]]
