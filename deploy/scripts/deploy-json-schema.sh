#!/usr/bin/env bash
# Deploy the CHAT JSON Schema to talkbank.org.
#
# Prerequisites:
#   - SSH access to the talkbank.org static server
#   - The schema has been regenerated in talkbank-tools:
#       cd talkbank-tools && cargo test --test generate_schema
#
# What this script does:
#   1. Copies chat-file.schema.json to /schemas/v0.1/ on the server
#   2. Creates a /schemas/latest/ redirect (symlink) pointing to the current version
#
# The server should be configured to serve .json files with:
#   Content-Type: application/schema+json
#   Access-Control-Allow-Origin: *
#
# Usage:
#   ./deploy-json-schema.sh <path-to-schema> [ssh-host]
#
# Example:
#   ./deploy-json-schema.sh ../talkbank-tools/schema/chat-file.schema.json
#   ./deploy-json-schema.sh ../talkbank-tools/schema/chat-file.schema.json user@talkbank.org

set -euo pipefail

SCHEMA_FILE="${1:?Usage: $0 <path-to-schema.json> [ssh-host]}"
VERSION="v0.1"

HOST="${2:-talkbank.org}"
REMOTE_BASE="/var/www/talkbank.org/schemas"

if [ ! -f "$SCHEMA_FILE" ]; then
    echo "Error: $SCHEMA_FILE not found. Run 'cargo test --test generate_schema' in talkbank-tools first." >&2
    exit 1
fi

echo "Deploying CHAT JSON Schema to $HOST"
echo "  Source: $SCHEMA_FILE"
echo "  Destination: $REMOTE_BASE/$VERSION/chat-file.json"
echo ""

# Create remote directories and copy
ssh "$HOST" "mkdir -p $REMOTE_BASE/$VERSION $REMOTE_BASE/latest"
scp "$SCHEMA_FILE" "$HOST:$REMOTE_BASE/$VERSION/chat-file.json"

# Create latest symlink
ssh "$HOST" "ln -sf ../$VERSION/chat-file.json $REMOTE_BASE/latest/chat-file.json"

echo ""
echo "Deployed. Verify with:"
echo "  curl -I https://talkbank.org/schemas/$VERSION/chat-file.json"
echo "  curl -I https://talkbank.org/schemas/latest/chat-file.json"
