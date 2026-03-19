#!/usr/bin/env bash
# Setup script for TalkBank data workspace.
# Clones all 24 data repos into a flat directory structure.
#
# Usage:
#   ./setup-data-workspace.sh [TARGET_DIR]
#
# Default target: ~/0data
# If repos already exist, they are skipped (not overwritten).

set -euo pipefail

TARGET="${1:-$HOME/0data}"

# All 24 data repos (12 unsplit + 12 from splits)
REPOS=(
    # Unsplit (1:1 with bank)
    aphasia-data
    asd-data
    biling-data
    class-data
    dementia-data
    fluency-data
    motor-data
    psychosis-data
    rhd-data
    samtale-data
    slabank-data
    tbi-data

    # childes (4-way split by language group)
    childes-eng-na-data        # Eng-NA, Eng-AAE
    childes-eng-uk-data        # Eng-UK, Clinical-Eng, Clinical-Other
    childes-romance-germanic-data  # French, Romance, Spanish, German, DutchAfrikaans, Scandinavian, Celtic
    childes-other-data         # Biling, Chinese, EastAsian, Japanese, Slavic, Finno-Ugric, Other, Frogs, MAIN, GlobalTales, XLing

    # ca (2-way split: CANDOR vs everything else)
    ca-candor-data             # CANDOR only (4.8 GB)
    ca-data                    # Everything else (40+ corpora)

    # phon (2-way split by language)
    phon-eng-french-data       # Eng-NA, French
    phon-other-data            # All other languages

    # homebank (4-way split by access tier)
    homebank-public-data       # Public + Secure
    homebank-cougar-data       # Password/Cougar
    homebank-bergelson-data    # Password/Bergelson
    homebank-password-data     # Password/ remainder
)

GITHUB_ORG="TalkBank"

echo "Setting up TalkBank data workspace at: $TARGET"
echo "Repos to clone: ${#REPOS[@]}"
echo ""

mkdir -p "$TARGET"

cloned=0
skipped=0
failed=0

for repo in "${REPOS[@]}"; do
    if [ -d "$TARGET/$repo/.git" ]; then
        echo "  skip  $repo (already exists)"
        skipped=$((skipped + 1))
    else
        echo "  clone $repo..."
        if git clone "git@github.com:${GITHUB_ORG}/${repo}.git" "$TARGET/$repo" 2>/dev/null; then
            # Install pre-push hook if hooks directory exists alongside this script
            SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
            HOOK_SRC="$SCRIPT_DIR/hooks/pre-push"
            if [ -f "$HOOK_SRC" ]; then
                cp "$HOOK_SRC" "$TARGET/$repo/.git/hooks/pre-push"
                chmod +x "$TARGET/$repo/.git/hooks/pre-push"
                # Also symlink the hooks directory for the hook scripts to find
                ln -sf "$SCRIPT_DIR/hooks" "$TARGET/$repo/.git/talkbank-hooks"
            fi
            cloned=$((cloned + 1))
        else
            echo "  FAILED to clone $repo"
            failed=$((failed + 1))
        fi
    fi
done

echo ""
echo "Done. Cloned: $cloned, Skipped: $skipped, Failed: $failed"
echo "Workspace: $TARGET"

if [ "$failed" -gt 0 ]; then
    echo ""
    echo "WARNING: $failed repos failed to clone. Check SSH key access to github.com."
    exit 1
fi
