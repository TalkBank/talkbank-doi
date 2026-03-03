---
name: doc-update
description: Update existing documentation to reflect code changes. Use after implementing a feature, fixing a bug, or making architectural changes that affect documentation. Pass the change description as argument.
disable-model-invocation: true
allowed-tools: Bash, Read, Write, Edit, Glob, Grep, Agent
---

# Update Documentation After Code Changes

Identify and update all documentation affected by a code change. `$ARGUMENTS` should describe what changed (e.g., "added new CLAN command 'newcmd'" or "changed LSP binary discovery").

## Step 1: Identify Affected Docs

Based on what changed, determine which documentation needs updating.

### Change in talkbank-chat (core parser/model/validation)

Check and potentially update:
- [ ] `talkbank-chat/book/src/` — architecture pages, user guide
- [ ] `talkbank-chat/CLAUDE.md` — if build commands, test counts, or architecture changed
- [ ] `talkbank-chat/crates/*/CLAUDE.md` — if crate API changed
- [ ] `talkbank-chat/docs/errors/` — run `make test-gen` if error codes changed
- [ ] `batchalign3/CLAUDE.md` — if shared crate interface changed
- [ ] `talkbank-chatter/CLAUDE.md` — if it affects LSP

### Change in talkbank-chatter (CLI/LSP/VS Code)

Check and potentially update:
- [ ] `talkbank-chatter/vscode/README.md` — marketplace description
- [ ] `talkbank-chatter/vscode/GUIDE.md` — user guide
- [ ] `talkbank-chatter/vscode/DEVELOPER.md` — developer guide, module map
- [ ] `talkbank-chatter/vscode/CLAN-FEATURES.md` — feature parity
- [ ] `talkbank-chatter/CLAUDE.md` — wired commands, architecture
- [ ] `talkbank-chatter/crates/talkbank-lsp/ARCHITECTURE.md` — LSP architecture
- [ ] `talkbank-chatter/vscode/package.json` — description field if features changed

### Change in batchalign3 (pipeline/server)

Check and potentially update:
- [ ] `batchalign3/book/src/` — relevant book pages
- [ ] `batchalign3/CLAUDE.md` — test counts, architecture summary
- [ ] `batchalign3/rust-next/CLAUDE.md` — if Rust server changed
- [ ] `batchalign3/book/src/architecture/server-known-issues.md` — if fixing a known issue
- [ ] `batchalign3/book/src/architecture/chat-divorce.md` — if adding divorce phase

### Change in talkbank-clan (analysis commands)

Check and potentially update:
- [ ] `talkbank-clan/README.md` — command list
- [ ] `talkbank-clan/CHANGELOG.md` — version history
- [ ] `talkbank-chatter/CLAUDE.md` — wired command counts
- [ ] `talkbank-chatter/vscode/GUIDE.md` — analysis commands section
- [ ] `talkbank-chatter/vscode/README.md` — command count in description

### Change in tree-sitter-talkbank (grammar)

Check and potentially update:
- [ ] `tree-sitter-talkbank/README.md`
- [ ] `tree-sitter-talkbank/CHANGELOG.md`
- [ ] `talkbank-chat/book/src/architecture/` — grammar pages

### Infrastructure / ops change

Check and potentially update:
- [ ] `talkbank-private/batchalign/docs/` — fleet docs, deploy docs
- [ ] `talkbank-private/batchalign/ansible/README.md`
- [ ] `batchalign3/book/src/developer/` — deployment guide

## Step 2: Read Each Affected Doc

For each doc identified, read it and find the specific sections that need updating. Use Grep to locate relevant passages:

```bash
grep -n "<keyword>" <doc_path>
```

## Step 3: Make Targeted Updates

For each doc:
1. Find the exact section to update
2. Edit only the affected content — don't rewrite unrelated sections
3. Update any counts, command lists, or version numbers
4. Update "Last Updated" dates where present
5. Fix any cross-references that may have broken

## Step 4: Check for Stale Information

While updating, look for other stale content in the same file:
- Old file paths (e.g., `rust/vscode/` → `vscode/`)
- Wrong test counts
- Features listed as "unimplemented" that are now done
- Dead links to moved/renamed files

## Step 5: Verify

```bash
# Check all markdown links resolve (basic check)
grep -rn '\[.*\](.*\.md)' <updated_files> | while read line; do
  file=$(echo "$line" | sed 's/.*(\(.*\.md\)).*/\1/')
  # Check if relative link target exists
done

# If mdBook page was updated
cd /Users/chen/talkbank/<repo>/book && mdbook build 2>&1 | grep -i "error"
```

## Step 6: Report

Summarize what was updated:
- List of files changed
- What was updated in each (section, content type)
- Any stale content discovered and fixed
- Any docs that may need future attention
