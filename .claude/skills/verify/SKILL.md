---
name: verify
description: Run pre-merge verification across TalkBank repos. Use when the user wants to check that everything compiles, passes lint, and tests pass before committing.
disable-model-invocation: true
allowed-tools: Bash, Read, Glob, Grep
---

# Pre-Merge Verification

Run the appropriate verification commands based on what changed. If `$ARGUMENTS` specifies a repo or scope, verify only that. Otherwise, detect changes automatically.

## Step 1: Detect What Changed

If no arguments given, scan both repos for uncommitted changes:

```bash
cd /Users/chen/talkbank
for repo in talkbank-tools batchalign3; do
  if [ -d "$repo/.git" ]; then
    changes=$(cd "$repo" && git diff --name-only HEAD 2>/dev/null | head -5)
    if [ -n "$changes" ]; then
      echo "=== $repo has changes ==="
      echo "$changes"
    fi
  fi
done
```

If arguments are given (e.g., `/verify talkbank-tools`), only verify that repo.

## Step 2: Run Verification Commands

For each repo with changes, run these commands **in order**. Report results clearly with pass/fail for each step.

### talkbank-tools

```bash
cd /Users/chen/talkbank/talkbank-tools
make verify
```

This runs gates G0–G10 (fmt, clippy, nextest, doctests, parser-tests, reference corpus, etc.).

If only grammar changed:
```bash
cd /Users/chen/talkbank/talkbank-tools/grammar
tree-sitter generate
tree-sitter test
cd /Users/chen/talkbank/talkbank-tools
cargo nextest run -p talkbank-parser
cargo nextest run -p talkbank-parser-tests
```

If only VS Code extension changed:
```bash
cd /Users/chen/talkbank/talkbank-tools/vscode
npm run compile && npm run lint && npm test
```

If only spec changed:
```bash
cd /Users/chen/talkbank/talkbank-tools
make test-gen
make verify
```

### batchalign3

Python tests:
```bash
cd /Users/chen/talkbank/batchalign3
uv run pytest batchalign --disable-pytest-warnings -k 'not test_whisper_fa_pipeline' -x -q
```

Rust tests (PyO3 + workspace):
```bash
cd /Users/chen/talkbank/batchalign3
cargo test --manifest-path pyo3/Cargo.toml
cargo test --workspace
```

Type checking:
```bash
cd /Users/chen/talkbank/batchalign3
uv run mypy --strict batchalign/errors.py batchalign/pipeline_api.py batchalign/providers/__init__.py batchalign/worker/_types.py
```

If Rust CLI/server changed:
```bash
cd /Users/chen/talkbank/batchalign3
cargo clippy --all-targets -- -D warnings
```

If frontend changed:
```bash
cd /Users/chen/talkbank/batchalign3
scripts/check_dashboard_api_drift.sh
cd frontend && npm run build
```

### Cross-repo verification

If `talkbank-tools` crates changed that `batchalign3` depends on:
```bash
cd /Users/chen/talkbank/batchalign3
cargo test --manifest-path pyo3/Cargo.toml
cargo test --workspace
```

## Step 3: Report Summary

Print a summary table:

| Repo | Clippy | Tests | Notes |
|------|--------|-------|-------|

Mark each cell with PASS, FAIL, or SKIP (if repo had no changes).

If anything failed, identify the specific failing test or lint error and suggest a fix.
