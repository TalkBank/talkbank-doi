#!/usr/bin/env bash
# Sync staging scripts to git-talkbank.
# Usage: ./scripts/sync-staging.sh [--dry-run]

set -euo pipefail

DRY_RUN=""
if [ "${1:-}" = "--dry-run" ]; then
    DRY_RUN="--dry-run"
    echo "DRY RUN"
fi

rsync -avz --delete \
    --exclude='.git' \
    --exclude='__pycache__' \
    --exclude='.mypy_cache' \
    --exclude='.venv' \
    $DRY_RUN \
    /Users/chen/talkbank/staging/scripts/ \
    macw@git-talkbank:~/staging/scripts/

echo "Done."
