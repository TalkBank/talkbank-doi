#!/usr/bin/env bash
# Deploy the chatter CLI to all macOS machines via Ansible.
#
# Usage:
#   bash deploy/scripts/deploy_chatter.sh
#   bash deploy/scripts/deploy_chatter.sh --no-build
#   bash deploy/scripts/deploy_chatter.sh --dry-run
#   bash deploy/scripts/deploy_chatter.sh brian study

set -euo pipefail

DRY_RUN=false
NO_BUILD=false
LIMIT=""
HOSTS=()

WORKSPACE_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TOOLS_REPO="$WORKSPACE_ROOT/talkbank-tools"
ANSIBLE_DIR="$WORKSPACE_ROOT/deploy/ansible"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run) DRY_RUN=true; shift ;;
        --no-build) NO_BUILD=true; shift ;;
        --help|-h)
            echo "Usage: bash deploy/scripts/deploy_chatter.sh [OPTIONS] [HOST...]"
            echo ""
            echo "Options:"
            echo "  --no-build    Skip cargo build"
            echo "  --dry-run     Run ansible in check mode"
            echo ""
            echo "Default target: all_macs"
            exit 0
            ;;
        *) HOSTS+=("$1"); shift ;;
    esac
done

if [[ ! -f "$TOOLS_REPO/Cargo.toml" ]]; then
    echo "ERROR: talkbank-tools repo not found at $TOOLS_REPO"
    exit 1
fi

if [[ ${#HOSTS[@]} -gt 0 ]]; then
    LIMIT="$(IFS=,; echo "${HOSTS[*]}")"
fi

BINARY="$TOOLS_REPO/target/release/chatter"

if ! $NO_BUILD; then
    echo "=== Building chatter (release) ==="
    (cd "$TOOLS_REPO" && cargo build --release -p talkbank-cli --bin chatter)
    echo ""
fi

if [[ ! -f "$BINARY" ]]; then
    echo "ERROR: No binary found at $BINARY"
    echo "Run without --no-build to build first."
    exit 1
fi

echo "Deploying chatter via Ansible"
if [[ -n "$LIMIT" ]]; then
    echo "  limit: $LIMIT"
else
    echo "  target: all_macs"
fi
echo "  binary: $BINARY"
echo ""

ANSIBLE_CMD=(
    ansible-playbook
    playbooks/deploy-chatter.yml
    -e "chatter_binary=$BINARY"
)

if [[ -n "$LIMIT" ]]; then
    ANSIBLE_CMD+=(--limit "$LIMIT")
fi

if $DRY_RUN; then
    ANSIBLE_CMD+=(--check --diff)
fi

(
    cd "$ANSIBLE_DIR"
    "${ANSIBLE_CMD[@]}"
)
