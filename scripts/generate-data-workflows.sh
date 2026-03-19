#!/usr/bin/env bash
# Generate GitHub Actions deploy workflows for all 24 data repos.
# Creates .github/workflows/deploy.yml in each repo, commits, and pushes.
#
# These workflows run on the self-hosted talkbank runner and do
# `git pull` when new commits are pushed to main — same pattern
# as the existing web repo workflows.
#
# Usage:
#   ./scripts/generate-data-workflows.sh [--dry-run] [DATA_DIR]
#
# Default DATA_DIR: ~/0data

set -euo pipefail

DRY_RUN=false
if [ "${1:-}" = "--dry-run" ]; then
    DRY_RUN=true
    shift
    echo "DRY RUN — no changes will be made"
fi

DATA_DIR="${1:-$HOME/0data}"

# Path on talkbank.org where data repos are cloned
DEPLOY_BASE="/home/macw/data"

REPOS=(
    aphasia-data asd-data biling-data class-data dementia-data
    fluency-data motor-data psychosis-data rhd-data samtale-data
    slabank-data tbi-data
    childes-eng-na-data childes-eng-uk-data
    childes-romance-germanic-data childes-other-data
    ca-candor-data ca-data
    phon-eng-french-data phon-other-data
    homebank-public-data homebank-cougar-data
    homebank-bergelson-data homebank-password-data
)

for repo in "${REPOS[@]}"; do
    repo_dir="$DATA_DIR/$repo"
    workflow_dir="$repo_dir/.github/workflows"
    workflow_file="$workflow_dir/deploy.yml"

    if [ ! -d "$repo_dir/.git" ]; then
        echo "SKIP: $repo (not found at $repo_dir)"
        continue
    fi

    # Generate human-readable name from repo name
    display_name=$(echo "$repo" | sed 's/-data$//' | sed 's/-/ /g')

    echo -n "$repo: "

    if [ "$DRY_RUN" = true ]; then
        echo "would create $workflow_file"
        continue
    fi

    mkdir -p "$workflow_dir"

    cat > "$workflow_file" << YAML
name: Deploy $display_name

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: [self-hosted, talkbank]
    steps:
      - name: Deploy
        run: |
          set -euo pipefail
          REPO=$DEPLOY_BASE/$repo
          if [ ! -d "\$REPO" ]; then
            echo "Clone not found at \$REPO — skipping"
            exit 0
          fi
          git -C "\$REPO" fetch --prune origin
          git -C "\$REPO" reset --hard origin/main
YAML

    git -C "$repo_dir" add .github/workflows/deploy.yml
    git -C "$repo_dir" commit -q -m "ci: add GitHub Actions deploy workflow"
    git -C "$repo_dir" push -q
    echo "done"
done

echo ""
echo "All workflows generated."
echo "NOTE: Repos must be cloned at $DEPLOY_BASE/ on talkbank.org for deploys to work."
