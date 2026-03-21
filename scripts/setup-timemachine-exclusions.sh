#!/bin/bash
# setup-timemachine-exclusions.sh
# Add Time Machine exclusions for regenerable/re-downloadable paths.
# Run once (or after adding new paths). Idempotent — safe to re-run.
#
# Usage: bash scripts/setup-timemachine-exclusions.sh [--dry-run]
#
# Date: 2026-03-20

set -euo pipefail

DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
    echo "[DRY RUN] Would add the following Time Machine exclusions:"
    echo
fi

HOME_DIR="$HOME"

# Paths to exclude. Each entry is: path, reason.
# Only paths that exist will be excluded.
EXCLUSIONS=(
    # Rust build artifacts (regenerable)
    "$HOME_DIR/talkbank/batchalign3/target:Rust build artifacts (batchalign3)"
    "$HOME_DIR/talkbank/talkbank-tools/target:Rust build artifacts (talkbank-tools)"

    # Python venvs (regenerable)
    "$HOME_DIR/talkbank/batchalign3/.venv:Python venv (batchalign3)"

    # IDE/tool caches (regenerable)
    "$HOME_DIR/.copilot/session-state:GitHub Copilot session cache"

    # Build and model caches (re-downloadable)
    "$HOME_DIR/.cache:Build and model caches (uv, huggingface, torch, etc.)"
    "$HOME_DIR/.rustup/toolchains:Rust toolchains (re-installable via rustup)"
    "$HOME_DIR/Library/Caches:macOS application caches"
    "$HOME_DIR/.ollama/models:Ollama local models (re-downloadable)"
    "$HOME_DIR/stanza_resources:Stanza NLP models (re-downloadable)"

    # Language toolchains (re-installable)
    "$HOME_DIR/.ghcup:Haskell toolchains (ghcup)"
    "$HOME_DIR/.elan:Lean 4 toolchains"
    "$HOME_DIR/.stack:Haskell Stack packages"
    "$HOME_DIR/.cabal:Haskell Cabal packages"
    "$HOME_DIR/.esy:OCaml/Reason packages (esy)"
    "$HOME_DIR/.sbt:Scala build tool cache"

    # Corpus data and web repos (all in git, fully recoverable)
    "$HOME_DIR/talkbank/data:Corpus data repos (24 repos, 35 GB, all on GitHub)"
    "$HOME_DIR/talkbank/web:Bank web repos (12 GB, recoverable via make clone-web)"

    # Large non-essential data
    "$HOME_DIR/isos:ISO images"
)

added=0
skipped=0

for entry in "${EXCLUSIONS[@]}"; do
    path="${entry%%:*}"
    reason="${entry#*:}"

    if [[ ! -e "$path" ]]; then
        echo "  SKIP (not found): $path"
        ((skipped++))
        continue
    fi

    if $DRY_RUN; then
        echo "  WOULD EXCLUDE: $path"
        echo "    Reason: $reason"
    else
        tmutil addexclusion "$path"
        echo "  EXCLUDED: $path"
        echo "    Reason: $reason"
    fi
    ((added++))
done

echo
echo "Done. $added paths excluded, $skipped skipped (not found)."

if ! $DRY_RUN; then
    echo
    echo "To verify, run:"
    echo "  tmutil isexcluded ~/talkbank/batchalign3/target"
fi
