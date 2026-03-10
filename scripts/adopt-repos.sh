#!/usr/bin/env bash
# Move existing repo clones from ~/ into the talkbank-dev workspace.
# Skips repos that already exist in the workspace.
# Usage: bash scripts/adopt-repos.sh [--dry-run]

set -euo pipefail

WORKSPACE="$(cd "$(dirname "$0")/.." && pwd)"
DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
    echo "[DRY RUN] No files will be moved."
    echo ""
fi

MOVED=0
SKIPPED=0
MISSING=0

adopt() {
    local src="$1"
    local dest="$2"

    if [ ! -d "$src" ]; then
        echo "  SKIP (not found): $src"
        MISSING=$((MISSING + 1))
        return
    fi

    if [ -d "$dest" ]; then
        echo "  SKIP (exists):    $dest"
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    # Ensure parent directory exists
    mkdir -p "$(dirname "$dest")"

    if $DRY_RUN; then
        echo "  WOULD MOVE: $src → $dest"
    else
        mv "$src" "$dest"
        echo "  MOVED: $src → $dest"
    fi
    MOVED=$((MOVED + 1))
}

echo "=== Core development ==="
adopt "$HOME/talkbank/talkbank-tools" "$WORKSPACE/talkbank-tools"  # likely already here
adopt "$HOME/talkbank/batchalign3" "$WORKSPACE/batchalign3"        # likely already here

echo ""
echo "=== Infrastructure & deployment ==="
adopt "$HOME/staging" "$WORKSPACE/staging"
adopt "$HOME/webdev" "$WORKSPACE/webdev"
adopt "$HOME/gra-cgi" "$WORKSPACE/gra-cgi"
adopt "$HOME/sync-media" "$WORKSPACE/sync-media"
adopt "$HOME/generate-from-chat" "$WORKSPACE/generate-from-chat"
# cdcs-to-csv: currently inside staging/repos/, clone separately if needed

echo ""
echo "=== Pre-commit / build tools ==="
adopt "$HOME/update-chat-types" "$WORKSPACE/update-chat-types"
adopt "$HOME/save-word-html-pdf" "$WORKSPACE/save-word-html-pdf"
adopt "$HOME/talkbank-xml-schema" "$WORKSPACE/talkbank-xml-schema"

echo ""
echo "=== CHAT tools ==="
adopt "$HOME/java-chatter-stable" "$WORKSPACE/java-chatter-stable"
adopt "$HOME/talkbank-ipa-fragments" "$WORKSPACE/talkbank-ipa-fragments"

echo ""
echo "=== Legacy CLAN ==="
adopt "$HOME/OSX-CLAN" "$WORKSPACE/OSX-CLAN"
adopt "$HOME/clan-info" "$WORKSPACE/clan-info"

echo ""
echo "=== Collaborator repos ==="
adopt "$HOME/phon" "$WORKSPACE/phon"
adopt "$HOME/phontalk" "$WORKSPACE/phontalk"

echo ""
echo "=== Browser & validation ==="
adopt "$HOME/talkbank-browser-check" "$WORKSPACE/talkbank-browser-check"

echo ""
echo "=== Web ==="
adopt "$HOME/web" "$WORKSPACE/web"

echo ""
echo "=== Corpus data ==="
if [ -d "$HOME/data" ]; then
    # Move individual data repos, not the parent directory
    # (user may have other things in ~/data/)
    mkdir -p "$WORKSPACE/data"
    for repo in aphasia-data asd-data biling-data ca-data childes-data class-data \
                dementia-data fluency-data homebank-data motor-data phon-data \
                psychosis-data rhd-data samtale-data slabank-data tbi-data; do
        adopt "$HOME/data/$repo" "$WORKSPACE/data/$repo"
    done
else
    echo "  ~/data/ not found, skipping data repos"
fi

echo ""
echo "================================"
echo "Summary: $MOVED moved, $SKIPPED already present, $MISSING not found"
if $DRY_RUN; then
    echo ""
    echo "This was a dry run. Run without --dry-run to actually move files."
fi
