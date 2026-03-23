#!/usr/bin/env bash
# Thin wrapper around the Ansible deployment playbook for internal use.
#
# Usage:
#   bash deploy/scripts/deploy_batchalign3.sh
#   bash deploy/scripts/deploy_batchalign3.sh --server
#   bash deploy/scripts/deploy_batchalign3.sh --clients
#   bash deploy/scripts/deploy_batchalign3.sh bilbo brian
#   bash deploy/scripts/deploy_batchalign3.sh --port 8001
#   bash deploy/scripts/deploy_batchalign3.sh --no-build
#   bash deploy/scripts/deploy_batchalign3.sh --dry-run

set -euo pipefail

PORT=8001
DRY_RUN=false
NO_BUILD=false
LIMIT=""
HOSTS=()

WORKSPACE_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BA3_REPO="$WORKSPACE_ROOT/batchalign3"
ANSIBLE_DIR="$WORKSPACE_ROOT/deploy/ansible"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run) DRY_RUN=true; shift ;;
        --no-build) NO_BUILD=true; shift ;;
        --server) LIMIT="batchalign3_server"; shift ;;
        --clients) LIMIT="batchalign3_clients"; shift ;;
        --all) LIMIT="batchalign3_fleet"; shift ;;
        --port) PORT="$2"; shift 2 ;;
        --help|-h)
            echo "Usage: bash deploy/scripts/deploy_batchalign3.sh [OPTIONS] [HOST...]"
            echo ""
            echo "Options:"
            echo "  --server      Deploy to the internal server group"
            echo "  --clients     Deploy to the internal client group"
            echo "  --all         Deploy to the full internal fleet"
            echo "  --port PORT   Override batchalign_port (default: 8001)"
            echo "  --no-build    Skip wheel build"
            echo "  --dry-run     Run ansible in check mode"
            echo ""
            echo "Default target: batchalign3_fleet"
            exit 0
            ;;
        *) HOSTS+=("$1"); shift ;;
    esac
done

if [[ ! -f "$BA3_REPO/pyproject.toml" ]]; then
    echo "ERROR: batchalign3 repo not found at $BA3_REPO"
    exit 1
fi

if [[ ! -d "$ANSIBLE_DIR" ]]; then
    echo "ERROR: ansible directory not found at $ANSIBLE_DIR"
    exit 1
fi

if [[ ${#HOSTS[@]} -gt 0 ]]; then
    LIMIT="$(IFS=,; echo "${HOSTS[*]}")"
elif [[ -z "$LIMIT" ]]; then
    LIMIT="batchalign3_fleet"
fi

if ! $NO_BUILD; then
    echo "=== Building dashboard + CLI binary + wheel ==="
    (
        cd "$BA3_REPO"
        make build-dashboard
        cargo build --release -p batchalign-cli --bin batchalign3
        cp target/release/batchalign3 batchalign/_bin/batchalign3
        uv run maturin build --release -m pyo3/Cargo.toml -F pyo3/extension-module --out dist/
    )
    echo ""
fi

WHEEL="$(ls -t "$BA3_REPO"/dist/batchalign3-*.whl 2>/dev/null | head -1)"
if [[ -z "$WHEEL" || ! -f "$WHEEL" ]]; then
    echo "ERROR: No wheel found in $BA3_REPO/dist/"
    echo "Run without --no-build to build a fresh wheel."
    exit 1
fi

echo "Deploying batchalign3 via Ansible"
echo "  limit: $LIMIT"
echo "  port:  $PORT"
echo "  wheel: $(basename "$WHEEL")"
echo ""

ANSIBLE_CMD=(
    ansible-playbook
    playbooks/deploy.yml
    --limit "$LIMIT"
    -e "batchalign3_wheel=$WHEEL"
    -e "batchalign_port=$PORT"
)

if $DRY_RUN; then
    ANSIBLE_CMD+=(--check --diff)
fi

(
    cd "$ANSIBLE_DIR"
    "${ANSIBLE_CMD[@]}"
)
