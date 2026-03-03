---
name: cascade
description: Analyze cross-repo impact of a change and determine what needs rebuilding and retesting. Use when the user changed a shared crate and wants to know what downstream repos are affected.
disable-model-invocation: true
allowed-tools: Bash, Read, Glob, Grep
---

# Cross-Repo Impact Analysis

Determine what downstream repos and crates are affected by a change, and run the necessary verification. If `$ARGUMENTS` specifies a crate or file, analyze that. Otherwise, detect changes automatically.

## Step 1: Identify What Changed

If arguments given (e.g., `/cascade talkbank-model`), use that as the changed crate.

Otherwise, scan for uncommitted changes:

```bash
cd /Users/chen/talkbank
for repo in tree-sitter-talkbank talkbank-chat talkbank-chatter talkbank-clan batchalign3; do
  if [ -d "$repo/.git" ]; then
    changes=$(cd "$repo" && git diff --name-only HEAD 2>/dev/null)
    if [ -n "$changes" ]; then
      echo "=== $repo ==="
      echo "$changes"
    fi
  fi
done
```

## Step 2: Map the Dependency Graph

Use this dependency map to determine impact:

### Core Crates (talkbank-chat)

| Changed Crate | Downstream in talkbank-chat | Downstream Repos |
|---------------|---------------------------|-------------------|
| `talkbank-errors` | ALL crates | ALL repos |
| `talkbank-model` | transform, parser-api, tree-sitter-parser, direct-parser, highlight, json, pipeline | talkbank-chatter, talkbank-clan, batchalign3 |
| `talkbank-parser-api` | tree-sitter-parser, direct-parser, transform | talkbank-chatter, batchalign3 |
| `talkbank-tree-sitter-parser` | transform | talkbank-chatter |
| `talkbank-direct-parser` | transform | batchalign3 |
| `talkbank-transform` | cli, lsp | talkbank-chatter, batchalign3 |
| `talkbank-highlight` | lsp | talkbank-chatter |
| `talkbank-json` | (standalone) | — |
| `talkbank-derive` | model | (indirect: all) |

### Grammar (tree-sitter-talkbank)

| Changed | Must Verify |
|---------|------------|
| `grammar.js` | tree-sitter tests → talkbank-tree-sitter-parser → parser-tests (73 files) → talkbank-chatter |

### Analysis (talkbank-clan)

| Changed | Must Verify |
|---------|------------|
| Any command | talkbank-clan tests → talkbank-chatter LSP (if wired) |
| `AnalysisCommand` trait | ALL commands + talkbank-chatter LSP |

### batchalign3

| Changed | Must Verify |
|---------|------------|
| `batchalign-chat-ops` | batchalign-server, batchalign-cli, rust-next integration tests |
| `batchalign-cache` | batchalign-server |
| `batchalign-server` | integration tests, CLI |
| Python worker | `uv run pytest tests/serve/` |
| `rust/` (PyO3) | `cargo test --manifest-path rust/Cargo.toml` + `uv run pytest` |

## Step 3: Generate Verification Plan

Based on the impact analysis, generate the exact commands needed. Group by repo and order by dependency (upstream first).

Example output for a `talkbank-model` change:

```
Impact: talkbank-model is the pivot crate — affects ALL downstream repos.

Verification plan:
1. talkbank-chat:     make verify
2. talkbank-clan:     cargo nextest run
3. talkbank-chatter:  cargo nextest run && cd vscode && npm run compile
4. batchalign3:       cargo test --manifest-path rust/Cargo.toml
5. batchalign3:       uv run pytest --ignore=_private -x -q
```

## Step 4: Run Verification

Run the generated verification plan. Report results as a table:

| Step | Repo | Command | Result |
|------|------|---------|--------|
| 1 | talkbank-chat | make verify | PASS/FAIL |
| 2 | talkbank-clan | cargo nextest run | PASS/FAIL |
| ... | ... | ... | ... |

If a step fails, stop and report the failure with enough context to diagnose.

## Step 5: Special Cases

### Spec changes (`talkbank-chat/spec/`)
Must run `make test-gen` BEFORE `make verify`. The generated tests drive verification.

### Grammar changes (`tree-sitter-talkbank/grammar.js`)
Must run `tree-sitter generate` first. Reference corpus must pass 100%.

### Cargo.toml changes (dependency versions)
Run `cargo update` in affected workspaces, then full test suite.
