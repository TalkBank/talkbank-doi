---
name: verify
description: Run pre-merge verification across TalkBank repos. Use when the user wants to check that everything compiles, passes lint, and tests pass before committing.
disable-model-invocation: true
allowed-tools: Bash, Read, Glob, Grep
---

# Pre-Merge Verification

Run the appropriate verification commands based on what changed. If `$ARGUMENTS` specifies a repo or scope, verify only that. Otherwise, detect changes automatically.

## Step 1: Detect What Changed

If no arguments given, run `git status` and `git diff --name-only` in each repo to detect modifications:

```bash
cd /Users/chen/talkbank
for repo in tree-sitter-talkbank talkbank-chat talkbank-chatter talkbank-clan batchalign3; do
  if [ -d "$repo/.git" ]; then
    changes=$(cd "$repo" && git diff --name-only HEAD 2>/dev/null | head -5)
    if [ -n "$changes" ]; then
      echo "=== $repo has changes ==="
      echo "$changes"
    fi
  fi
done
```

If arguments are given (e.g., `/verify talkbank-chat`), only verify that repo.

## Step 2: Run Verification Commands

For each repo with changes, run these commands **in order**. Report results clearly with pass/fail for each step.

### tree-sitter-talkbank
```bash
cd /Users/chen/talkbank/tree-sitter-talkbank
tree-sitter generate
tree-sitter test
```
Then verify reference corpus equivalence:
```bash
cd /Users/chen/talkbank/talkbank-chat
cargo test -p talkbank-parser-tests
```

### talkbank-chat
```bash
cd /Users/chen/talkbank/talkbank-chat
make verify
```
This runs gates G0–G10 (fmt, clippy, nextest, doctests, parser-tests, reference corpus, etc.).

### talkbank-chatter
```bash
cd /Users/chen/talkbank/talkbank-chatter
cargo clippy --all-targets -- -D warnings 2>&1 | head -50
cargo nextest run
cd vscode && npm run compile && npm run lint
```
Note: If clippy `--all-targets` fails on pre-existing warnings in `talkbank-cli`, narrow to the crate that changed:
```bash
cargo clippy -p talkbank-lsp -- -D warnings
```

### talkbank-clan
```bash
cd /Users/chen/talkbank/talkbank-clan
cargo clippy --all-targets -- -D warnings
cargo nextest run
```

### batchalign3
```bash
cd /Users/chen/talkbank/batchalign3
uv run pytest --ignore=_private -x -q
cargo test --manifest-path rust/Cargo.toml
```
If `rust-next/` was changed:
```bash
cd /Users/chen/talkbank/batchalign3/rust-next
cargo clippy --all-targets -- -D warnings
cargo nextest run
```

## Step 3: Report Summary

Print a summary table:

| Repo | Clippy | Tests | Notes |
|------|--------|-------|-------|

Mark each cell with PASS, FAIL, or SKIP (if repo had no changes).

If anything failed, identify the specific failing test or lint error and suggest a fix.
