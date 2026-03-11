#!/usr/bin/env bash
# Phase 1: Extract tree-sitter-talkbank
#
# Prerequisites:
#   - GitHub repo talkbank/tree-sitter-talkbank created (empty)
#   - Core Rust crates NOT yet on crates.io (spec/tools will use git deps initially)
#   - Working directory: the talkbank-tools monorepo root
#
# This script copies files (does NOT use git filter-branch to preserve history).
# History is preserved in the monorepo; the new repo starts fresh.

set -euo pipefail

MONOREPO="$(pwd)"
TARGET_DIR="${1:-../tree-sitter-talkbank}"

echo "=== Phase 1: Extract tree-sitter-talkbank ==="
echo "Source: $MONOREPO"
echo "Target: $TARGET_DIR"
echo ""

# --- Create target directory ---
if [ -d "$TARGET_DIR" ]; then
    echo "ERROR: Target directory already exists: $TARGET_DIR"
    exit 1
fi

mkdir -p "$TARGET_DIR"
cd "$TARGET_DIR"
git init

# --- Copy grammar files (becomes repo root) ---
echo "Copying grammar files..."
# Core grammar files
cp "$MONOREPO/grammar/grammar.js" .
cp "$MONOREPO/grammar/grammar.json" .
cp "$MONOREPO/grammar/node-types.json" .
cp "$MONOREPO/grammar/Cargo.toml" .
cp "$MONOREPO/grammar/Cargo.lock" .
cp "$MONOREPO/grammar/package.json" .
cp "$MONOREPO/grammar/package-lock.json" .
cp "$MONOREPO/grammar/pyproject.toml" .
cp "$MONOREPO/grammar/setup.py" .
cp "$MONOREPO/grammar/tree-sitter.json" .
cp "$MONOREPO/grammar/binding.gyp" .
cp "$MONOREPO/grammar/CMakeLists.txt" .
cp "$MONOREPO/grammar/Package.swift" .
cp "$MONOREPO/grammar/Package.resolved" .
cp "$MONOREPO/grammar/go.mod" .
cp "$MONOREPO/grammar/highlight.py" .
cp "$MONOREPO/grammar/Makefile" .
cp "$MONOREPO/grammar/LICENSE" .
cp "$MONOREPO/grammar/README.md" .
cp "$MONOREPO/grammar/CHANGELOG.md" .
cp "$MONOREPO/grammar/CLAUDE.md" .
cp "$MONOREPO/grammar/.editorconfig" .
cp "$MONOREPO/grammar/.gitattributes" .
cp "$MONOREPO/grammar/.gitignore" .
cp "$MONOREPO/grammar/test-all-examples.sh" .

# Directories
cp -r "$MONOREPO/grammar/src" .
cp -r "$MONOREPO/grammar/bindings" .
cp -r "$MONOREPO/grammar/queries" .
cp -r "$MONOREPO/grammar/test" .
cp -r "$MONOREPO/grammar/scripts" .

# --- Copy spec directory ---
echo "Copying spec directory..."
cp -r "$MONOREPO/spec" .

# --- Update spec/tools Cargo.toml to use git deps ---
echo "Updating spec/tools dependencies to git deps..."
# This will need manual editing since the Cargo.toml has path deps.
# For now, just note what needs to change.
cat << 'DEPS_NOTE'

TODO: Update spec/Cargo.toml workspace deps:
  Replace:
    talkbank-errors = { path = "../rust/crates/talkbank-errors" }
    talkbank-model = { path = "../rust/crates/talkbank-model" }
    talkbank-tree-sitter-parser = { path = "../rust/crates/talkbank-tree-sitter-parser" }
  With (git deps for bootstrap):
    talkbank-errors = { git = "https://github.com/talkbank/talkbank" }
    talkbank-model = { git = "https://github.com/talkbank/talkbank" }
    talkbank-tree-sitter-parser = { git = "https://github.com/talkbank/talkbank" }
  Then after Phase 2 (crates.io publish):
    talkbank-errors = "0.1"
    talkbank-model = "0.1"
    talkbank-tree-sitter-parser = "0.1"

DEPS_NOTE

# --- Verify ---
echo ""
echo "=== Verification ==="

echo "Checking tree-sitter grammar..."
if command -v tree-sitter &>/dev/null; then
    tree-sitter generate
    tree-sitter test
    echo "Grammar: OK"
else
    echo "SKIP: tree-sitter not in PATH"
fi

echo ""
echo "Checking spec/tools build..."
if command -v cargo &>/dev/null; then
    echo "SKIP: spec/tools deps need updating first (see TODO above)"
else
    echo "SKIP: cargo not in PATH"
fi

echo ""
echo "=== Done ==="
echo "Next steps:"
echo "  1. Review and update spec/Cargo.toml deps"
echo "  2. Add CI configuration (.github/workflows/ci.yml)"
echo "  3. git add -A && git commit -m 'Initial extraction from talkbank-tools'"
echo "  4. git remote add origin https://github.com/talkbank/tree-sitter-talkbank.git"
echo "  5. git push -u origin main"
