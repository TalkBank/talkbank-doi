#!/usr/bin/env bash
# Deploy sync-media (Rust binary) to net.
#
# Builds the release binary, copies it to net, and converts the config
# from the old dotenv format to TOML if needed.
#
# Usage:
#   bash deploy/scripts/deploy_sync_media.sh             # build + deploy
#   bash deploy/scripts/deploy_sync_media.sh --no-build   # deploy existing binary
#   bash deploy/scripts/deploy_sync_media.sh --dry-run    # show plan only
#   bash deploy/scripts/deploy_sync_media.sh --uninstall-python  # also remove old Python version

set -euo pipefail

SSH_USER="macw"
HOST="net"
REMOTE_BIN="\$HOME/.local/bin/sync-media"
REMOTE_CONFIG="\$HOME/.sync-media/config"

WORKSPACE_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SYNC_MEDIA_DIR="$WORKSPACE_ROOT/sync-media"
BINARY="$SYNC_MEDIA_DIR/target/release/sync-media"

DRY_RUN=false
NO_BUILD=false
UNINSTALL_PYTHON=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)            DRY_RUN=true; shift ;;
        --no-build)           NO_BUILD=true; shift ;;
        --uninstall-python)   UNINSTALL_PYTHON=true; shift ;;
        --help|-h)
            echo "Usage: bash deploy/scripts/deploy_sync_media.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --no-build           Skip cargo build"
            echo "  --uninstall-python   Remove old Python sync-media (uv tool)"
            echo "  --dry-run            Show plan only"
            exit 0
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

echo "=== sync-media deploy ==="
echo "Target: ${SSH_USER}@${HOST}"
echo ""

# --- Build ---
if [[ "$NO_BUILD" == false ]]; then
    echo "Building release binary..."
    if [[ "$DRY_RUN" == true ]]; then
        echo "  [dry-run] cargo build --release (in $SYNC_MEDIA_DIR)"
    else
        (cd "$SYNC_MEDIA_DIR" && cargo build --release)
    fi
    echo ""
fi

if [[ ! -f "$BINARY" && "$DRY_RUN" == false ]]; then
    echo "ERROR: Binary not found at $BINARY"
    echo "Run without --no-build first."
    exit 1
fi

# --- Uninstall old Python version ---
if [[ "$UNINSTALL_PYTHON" == true ]]; then
    echo "Removing old Python sync-media..."
    if [[ "$DRY_RUN" == true ]]; then
        echo "  [dry-run] ssh ${SSH_USER}@${HOST} uv tool uninstall sync-media"
    else
        ssh "${SSH_USER}@${HOST}" "uv tool uninstall sync-media 2>/dev/null || true"
        echo "  Done."
    fi
    echo ""
fi

# --- Deploy binary ---
echo "Deploying binary to ${SSH_USER}@${HOST}:~/.local/bin/sync-media..."
if [[ "$DRY_RUN" == true ]]; then
    echo "  [dry-run] scp $BINARY ${SSH_USER}@${HOST}:~/.local/bin/"
else
    ssh "${SSH_USER}@${HOST}" "mkdir -p ~/.local/bin"
    scp "$BINARY" "${SSH_USER}@${HOST}:~/.local/bin/sync-media"
    ssh "${SSH_USER}@${HOST}" "chmod +x ~/.local/bin/sync-media && codesign --sign - --force ~/.local/bin/sync-media"
    echo "  Done."
fi
echo ""

# --- Convert config if needed ---
echo "Checking config format on ${HOST}..."
if [[ "$DRY_RUN" == true ]]; then
    echo "  [dry-run] Would check and convert ~/.sync-media/config to TOML if needed"
else
    # Check if config exists and is still dotenv format (has = without quotes after key)
    CONFIG_CONTENT=$(ssh "${SSH_USER}@${HOST}" "cat ~/.sync-media/config 2>/dev/null" || true)
    if echo "$CONFIG_CONTENT" | grep -q '^SYNC_'; then
        echo "  Config is old dotenv format. Converting to TOML..."
        ssh "${SSH_USER}@${HOST}" "cp ~/.sync-media/config ~/.sync-media/config.bak.dotenv"

        # Extract values from dotenv and write TOML
        ssh "${SSH_USER}@${HOST}" 'cat > ~/.sync-media/config << '\''TOML'\''
# sync-media configuration (TOML)
# Converted from dotenv format on '"$(date +%Y-%m-%d)"'
# Backup: ~/.sync-media/config.bak.dotenv

host = "talkbank-02.andrew.cmu.edu"
user = "psych-tb-svc"
remote_name = "mediaserver"
remote_type = "sftp"

source_dir = "/Users/macw/media"
dest_path = "/data"
log_dir = "/Users/macw/.sync-media/logs"

validate_banks = true
valid_banks = [
    "aphasia", "asd", "biling", "ca", "childes", "class",
    "dementia", "fluency", "homebank", "motor", "open",
    "phon", "psyling", "psychosis", "rhd", "samtale", "slabank", "tbi"
]

max_retries = 3
retry_base_secs = 1
retry_max_secs = 10
TOML'
        echo "  Config converted. Backup at ~/.sync-media/config.bak.dotenv"
    else
        echo "  Config is already TOML or not present. No conversion needed."
    fi
fi
echo ""

# --- Verify ---
echo "Verifying deployment..."
if [[ "$DRY_RUN" == true ]]; then
    echo "  [dry-run] ssh ${SSH_USER}@${HOST} sync-media --version"
    echo "  [dry-run] ssh ${SSH_USER}@${HOST} sync-media --help"
else
    echo -n "  Version: "
    ssh "${SSH_USER}@${HOST}" "~/.local/bin/sync-media --version"
    echo -n "  Dry-run test: "
    ssh "${SSH_USER}@${HOST}" "~/.local/bin/sync-media childes --dry-run 2>&1 | head -3"
fi
echo ""
echo "=== Deploy complete ==="
