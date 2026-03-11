#!/usr/bin/env bash
# Phase 2: Publish core Rust crates to crates.io
#
# Prerequisites:
#   - `cargo login` completed
#   - All Cargo.toml metadata is complete (keywords, categories, license, etc.)
#   - tree-sitter-talkbank already published (Phase 1)
#   - Grammar path dep updated to crates.io version
#
# Publishing order follows dependency graph (leaves first).

set -euo pipefail

DRY_RUN="${DRY_RUN:-true}"

if [ "$DRY_RUN" = "true" ]; then
    PUBLISH_CMD="cargo publish --dry-run"
    echo "=== DRY RUN MODE (set DRY_RUN=false to actually publish) ==="
else
    PUBLISH_CMD="cargo publish"
    echo "=== PUBLISHING TO CRATES.IO ==="
fi

echo ""

# Publishing order (dependency-sorted):
CRATES=(
    "talkbank-derive"
    "talkbank-errors"
    "talkbank-model"
    "talkbank-parser-api"
    "talkbank-json"
    "talkbank-pipeline"
    "talkbank-direct-parser"
    "talkbank-tree-sitter-parser"
    "talkbank-transform"
)

for crate in "${CRATES[@]}"; do
    echo "--- Publishing $crate ---"
    $PUBLISH_CMD -p "$crate"
    if [ "$DRY_RUN" = "false" ]; then
        echo "Waiting 30s for crates.io to index $crate..."
        sleep 30
    fi
    echo ""
done

echo "=== All crates published ==="
echo ""
echo "Verification:"
echo "  cargo install talkbank-transform  # Should work"
echo "  cargo add talkbank-model          # Should work"

# Note: These crates are NOT published (internal/non-publishable):
# - talkbank-parser-tests (publish = false, test harness)
# - talkbank-progress (inlined into CLI)
# - doc-tools (publish = false, internal tooling)
# - batchalign-core (publish = false, moving to batchalign repo)
