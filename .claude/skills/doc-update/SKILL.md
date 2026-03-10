---
name: doc-update
description: Update existing documentation to reflect code changes. Use after implementing a feature, fixing a bug, or making architectural changes that affect documentation. Pass the change description as argument.
disable-model-invocation: true
allowed-tools: Bash, Read, Write, Edit, Glob, Grep, Agent
---

# Update Documentation After Code Changes

Identify and update all documentation affected by a code change. `$ARGUMENTS` should describe what changed (e.g., "added new CLAN command 'newcmd'" or "changed worker protocol").

## Step 1: Identify Affected Docs

Based on what changed, determine which documentation needs updating.

### Change in talkbank-tools (parser/model/validation/CLAN)

Check and potentially update:
- [ ] `talkbank-tools/book/src/` — architecture pages, user guide
- [ ] `talkbank-tools/CLAUDE.md` — if build commands, test counts, or architecture changed
- [ ] `talkbank-tools/grammar/CLAUDE.md` — if grammar patterns changed
- [ ] `talkbank-tools/spec/CLAUDE.md` — if spec tooling changed
- [ ] `talkbank-tools/vscode/CLAUDE.md` — if it affects VS Code extension
- [ ] `talkbank-tools/docs/errors/` — run `make test-gen` if error codes changed
- [ ] `batchalign3/CLAUDE.md` — if shared crate interface changed (path deps)

### Change in talkbank-tools/vscode (VS Code extension)

Check and potentially update:
- [ ] `talkbank-tools/vscode/README.md` — marketplace description
- [ ] `talkbank-tools/vscode/GUIDE.md` — user guide
- [ ] `talkbank-tools/vscode/DEVELOPER.md` — developer guide, module map
- [ ] `talkbank-tools/vscode/CLAN-FEATURES.md` — feature parity
- [ ] `talkbank-tools/vscode/CLAUDE.md` — wired commands, architecture

### Change in talkbank-tools/grammar

Check and potentially update:
- [ ] `talkbank-tools/grammar/CLAUDE.md` — design patterns, verification sequence
- [ ] `talkbank-tools/book/src/` — grammar architecture pages

### Change in batchalign3 (pipeline/server/worker)

Check and potentially update:
- [ ] `batchalign3/book/src/` — relevant book pages
- [ ] `batchalign3/CLAUDE.md` — test counts, architecture summary, worker protocol

### Change in batchalign3/frontend (React dashboard)

Check and potentially update:
- [ ] `batchalign3/book/src/` — dashboard-related pages

### Infrastructure / ops change

Check and potentially update:
- [ ] `docs/` — internal docs in talkbank-dev
- [ ] `deploy/` — deploy scripts and READMEs
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
- Old repo names (talkbank-chat, talkbank-chatter, talkbank-clan as separate repos)
- Wrong test counts
- Features listed as "unimplemented" that are now done
- Dead links to moved/renamed files

## Step 5: Verify

```bash
# If mdBook page was updated
cd /Users/chen/talkbank/talkbank-tools/book && mdbook build 2>&1 | grep -i "error"
cd /Users/chen/talkbank/batchalign3/book && mdbook build 2>&1 | grep -i "error"
```

## Step 6: Report

Summarize what was updated:
- List of files changed
- What was updated in each (section, content type)
- Any stale content discovered and fixed
- Any docs that may need future attention
