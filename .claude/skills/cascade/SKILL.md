---
name: cascade
description: Analyze cross-repo impact of a change and determine what needs rebuilding and retesting. Use when the user changed a shared crate and wants to know what downstream repos are affected.
disable-model-invocation: true
allowed-tools: Bash, Read, Glob, Grep
---

# Cross-Repo Impact Analysis

Determine what downstream crates and repos are affected by a change, and run the necessary verification. If `$ARGUMENTS` specifies a crate or file, analyze that. Otherwise, detect changes automatically.

## Step 1: Identify What Changed

If arguments given (e.g., `/cascade talkbank-model`), use that as the changed crate.

Otherwise, scan for uncommitted changes:

```bash
cd /Users/chen/talkbank
for repo in talkbank-tools batchalign3; do
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

### talkbank-tools crate dependencies

```
talkbank-derive → talkbank-model → talkbank-parser
                                  → talkbank-direct-parser
                                  → talkbank-transform → talkbank-clan
                                                        → talkbank-cli (chatter)
                                                        → talkbank-lsp
                                  → talkbank-parser-tests
```

| Changed Crate | Downstream in talkbank-tools | Also affects batchalign3? |
|---------------|------------------------------|---------------------------|
| `talkbank-derive` | ALL crates (via talkbank-model) | Yes (path deps) |
| `talkbank-model` | parser, direct-parser, transform, clan, cli, lsp, parser-tests | Yes (path deps) |
| `talkbank-parser` | transform, cli, lsp, parser-tests | Yes (via batchalign-chat-ops) |
| `talkbank-direct-parser` | transform | Yes (via batchalign-chat-ops) |
| `talkbank-transform` | clan, cli, lsp | Yes (via batchalign-chat-ops, pyo3) |
| `talkbank-clan` | cli, lsp | No |
| `talkbank-cli` | (terminal) | No |
| `talkbank-lsp` | (terminal) | No |

### batchalign3 crate dependencies

batchalign3 has path dependencies into `../../talkbank-tools/crates/`:

```
batchalign-chat-ops → talkbank-model, talkbank-parser, talkbank-direct-parser, talkbank-transform
batchalign-app      → batchalign-chat-ops
batchalign-cli      → batchalign-app
pyo3 (batchalign_core) → talkbank-model, talkbank-parser, talkbank-transform, batchalign-chat-ops
```

| Changed Crate | Downstream in batchalign3 |
|---------------|--------------------------|
| `batchalign-chat-ops` | batchalign-app, batchalign-cli, pyo3 |
| `batchalign-app` | batchalign-cli |
| `batchalign-cli` | (terminal) |
| `pyo3` | Python `batchalign_core` extension |
| Python `batchalign/` | Nothing (picked up by workers automatically) |

### Grammar changes

| Changed | Must Verify |
|---------|------------|
| `grammar/grammar.js` | `tree-sitter generate` → `tree-sitter test` → talkbank-parser → parser-tests (74 files) → batchalign3 pyo3 |

### Spec changes

| Changed | Must Verify |
|---------|------------|
| `spec/constructs/` or `spec/errors/` | `make test-gen` → `make verify` |

## Step 3: Generate Verification Plan

Based on the impact analysis, generate the exact commands needed. Group by repo and order by dependency (upstream first).

Example output for a `talkbank-model` change:

```
Impact: talkbank-model is the core data crate — affects ALL downstream.

Verification plan:
1. talkbank-tools:  make verify
2. batchalign3:     cargo test --manifest-path pyo3/Cargo.toml
3. batchalign3:     cargo test --workspace
4. batchalign3:     uv run pytest batchalign -x -q
```

## Step 4: Run Verification

Run the generated verification plan. Report results as a table:

| Step | Repo | Command | Result |
|------|------|---------|--------|
| 1 | talkbank-tools | make verify | PASS/FAIL |
| 2 | batchalign3 | cargo test (pyo3) | PASS/FAIL |
| ... | ... | ... | ... |

If a step fails, stop and report the failure with enough context to diagnose.

## Step 5: Special Cases

### Spec changes (`talkbank-tools/spec/`)
Must run `make test-gen` BEFORE `make verify`. The generated tests drive verification.

### Grammar changes (`talkbank-tools/grammar/grammar.js`)
Must run `tree-sitter generate` first. Reference corpus must pass 100%.

### Cargo.toml changes (dependency versions)
Run `cargo check` in affected workspaces to regenerate Cargo.lock, then full test suite.
