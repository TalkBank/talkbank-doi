#!/usr/bin/env bash
# Deploy batchalign3[serve] to server machines via SSH.
#
# By default deploys to Net only (production). Can deploy to any/all machines.
# Installs main tool under Python 3.14t (free-threaded) and creates a Python 3.12
# sidecar venv at ~/.batchalign3/sidecar/.venv/ for transcribe support.
#
# Usage:
#   bash scripts/deploy_server.sh                # build + deploy to Net
#   bash scripts/deploy_server.sh bilbo brian     # deploy to specific hosts
#   bash scripts/deploy_server.sh --all           # deploy to all fleet machines
#   bash scripts/deploy_server.sh --dry-run       # build wheels, show what would deploy
#   bash scripts/deploy_server.sh --no-build      # skip wheel build (use existing)

set -euo pipefail

PYTHON_VERSION="3.14t"
SIDECAR_PYTHON="3.12"
SSH_USER="macw"
DEFAULT_HOST="net"
ALL_FLEET_HOSTS=(net bilbo brian davida frodo andrew lilly sue vaishnavi)

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CORE_REPO="$REPO_ROOT/rust"
CORE_CARGO="$CORE_REPO/crates/batchalign-core/Cargo.toml"

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
            echo "Usage: bash scripts/deploy_server.sh [OPTIONS] [HOST...]"
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

# --- RAM-based worker tuning per host ---
# Returns recommended max_concurrent_jobs for a given RAM size (GB)
ram_to_workers() {
    local ram_gb=$1
    if   (( ram_gb >= 200 )); then echo 8
    elif (( ram_gb >= 100 )); then echo 4
    elif (( ram_gb >=  48 )); then echo 2
    else echo 1
    fi
}

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
CP_TAG="cp${PYTHON_VERSION//[^0-9]/}"
BA_CORE_314T_WHEEL="$(ls -t "$CORE_REPO"/target/wheels/batchalign_core-*-${CP_TAG}*-macosx_*.whl 2>/dev/null | head -1)"

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

echo "Deploying batchalign3[serve] to: ${HOSTS[*]}"
echo "  batchalign3:       $BA_NEXT_NAME"
echo "  batchalign-core 3.14t: $BA_CORE_314T_NAME  (main)"
echo "  batchalign-core 3.12:  $BA_CORE_312_NAME  (sidecar)"
echo ""

if $DRY_RUN; then
    echo "=== DRY RUN ==="
    for host in "${HOSTS[@]}"; do
        echo "  [$host] Would scp 3 wheels, install main (3.14t), create sidecar (3.12), restart server"
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

    # Copy all three wheels
    echo "  Copying wheels..."
    scp -q "$BA_NEXT_WHEEL" "$BA_CORE_314T_WHEEL" "$BA_CORE_312_WHEEL" "$SSH_USER@$host:/tmp/"

    # Stop the server before installing — use multiple kill strategies to
    # ensure no old processes survive (stale PID files, unresponsive SIGTERM, etc.)
    echo "  Stopping server..."
    ssh "$SSH_USER@$host" '
        batchalign3 serve stop 2>/dev/null || true
        sleep 1
        # Nuclear option: kill anything still running batchalign3 serve
        pkill -9 -f "batchalign3 serve" 2>/dev/null || true
        sleep 1
        # Verify nothing survived
        if pgrep -f "batchalign3 serve" >/dev/null 2>&1; then
            echo "  WARNING: processes still alive after pkill -9"
            pgrep -f "batchalign3 serve" | xargs kill -9 2>/dev/null || true
            sleep 1
        fi
    '

    # Remove old installations
    echo "  Removing old uv tool installations..."
    ssh "$SSH_USER@$host" "
        uv tool uninstall batchalign3 2>/dev/null || true
        uv tool uninstall batchalign 2>/dev/null || true
        uv tool uninstall batchalignhk 2>/dev/null || true
    " 2>&1 | grep -v "is not installed" || true

    # --- Phase 1: Main tool (Python 3.14t) ---
    echo "  Ensuring Python $PYTHON_VERSION on $host..."
    ssh "$SSH_USER@$host" "uv python install $PYTHON_VERSION 2>/dev/null || true"

    echo "  Installing batchalign3 (Python $PYTHON_VERSION)..."
    ssh "$SSH_USER@$host" "
        uv tool install --python $PYTHON_VERSION --force-reinstall \
            /tmp/$BA_NEXT_NAME \
            --with /tmp/$BA_CORE_314T_NAME 2>&1
    " | tail -5

    # Verify main installation
    echo "  Verifying main installation..."
    version=$(ssh "$SSH_USER@$host" \
        'batchalign3 --help >/dev/null 2>&1 && \
         ~/.local/share/uv/tools/batchalign3/bin/python -c "
from batchalign.pipelines.context import ProcessingContext
print(\"ok\")
" 2>&1' || echo "FAILED")

    if [[ "$version" != "ok" ]]; then
        echo "  ERROR: Installation verification failed on $host: $version"
        FAILED_HOSTS+=("$host")
        continue
    fi
    echo "  OK: Main installation verified"

    # --- Phase 2: Sidecar venv (Python 3.12, ASR extras) ---
    echo "  Setting up transcribe sidecar (Python $SIDECAR_PYTHON)..."
    ssh "$SSH_USER@$host" "uv python install $SIDECAR_PYTHON 2>/dev/null || true"
    ssh "$SSH_USER@$host" "
        uv venv ~/.batchalign3/sidecar/.venv --python $SIDECAR_PYTHON 2>/dev/null || true
        uv pip install --python ~/.batchalign3/sidecar/.venv/bin/python \
            /tmp/$BA_NEXT_NAME /tmp/$BA_CORE_312_NAME \
            openai-whisper 'numba>=0.61.0' 'llvmlite>=0.44.0' 2>&1
    " | tail -5

    # Verify sidecar
    echo "  Verifying sidecar..."
    sidecar_ok=$(ssh "$SSH_USER@$host" '
        ~/.batchalign3/sidecar/.venv/bin/python -c "
import whisper
print(\"sidecar:ok\")
" 2>&1' || echo "FAILED")

    if [[ "$sidecar_ok" != *"sidecar:ok"* ]]; then
        echo "  WARNING: Sidecar verification failed: $sidecar_ok"
        echo "  (Continuing — transcribe will be unavailable on this host)"
    else
        echo "  OK: Sidecar verified"
    fi

    # Generate server.yaml if it doesn't exist
    echo "  Checking server.yaml..."
    ssh "$SSH_USER@$host" '
        CONFIG_DIR="$HOME/.batchalign3"
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

    # Fleet config: only deploy fleet.yaml when --fleet flag is passed.
    # Without fleet.yaml, the dashboard and CLI operate in single-server mode.
    # TODO: Enable fleet deployment once all machines run batchalign3 serve.
    # echo "  Deploying fleet.yaml..."

    # Start the server with GIL mode based on core count.
    # Large machines (24+ cores): GIL=1 — ProcessPool gives true parallelism;
    # PyTorch internal threading contends under ThreadPool.
    # Small machines (<24 cores): GIL=0 — ThreadPool avoids over-subscription.
    echo "  Starting server..."
    ssh "$SSH_USER@$host" '
        cores=$(sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 4)
        if [ "$cores" -ge 24 ]; then
            GIL_VAL=1
        else
            GIL_VAL=0
        fi
        PYTHON_GIL=$GIL_VAL batchalign3 serve start
    '

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

    # Report free-threading status
    ft=$(echo "$health" | python3 -c "import sys,json; print(json.load(sys.stdin).get('free_threaded','unknown'))" 2>/dev/null || echo "unknown")
    echo "  Free-threaded: $ft"

    if ! echo "$health" | grep -q '"status":"ok"'; then
        echo "  WARNING: Server health check failed on $host after ${HEALTH_TIMEOUT}s"
        echo "  Check logs: ssh $SSH_USER@$host 'tail -50 ~/.batchalign3/server.log'"
        FAILED_HOSTS+=("$host")
        continue
    fi

    # Clean up
    ssh "$SSH_USER@$host" "rm -f /tmp/batchalign_*.whl" 2>/dev/null || true

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
