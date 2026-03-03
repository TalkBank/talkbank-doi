---
name: doc-search
description: Search across all TalkBank project documentation — mdBooks, READMEs, CLAUDE.md files, specs, ops docs, and private docs. Use when the user asks "where is the doc for X" or "what does the docs say about Y".
disable-model-invocation: true
allowed-tools: Bash, Read, Glob, Grep, Agent
---

# Search TalkBank Documentation

Find documentation across the entire TalkBank workspace. `$ARGUMENTS` is the search query.

## Documentation Map

### mdBooks (structured, hierarchical)

| Book | Path | Audience |
|------|------|----------|
| TalkBank Tooling | `/Users/chen/talkbank/talkbank-chat/book/src/` | CHAT format, parsers, validation |
| Batchalign | `/Users/chen/talkbank/batchalign3/book/src/` | Pipeline, server, deployment |

To see what's in each book:
```bash
cat /Users/chen/talkbank/talkbank-chat/book/src/SUMMARY.md
cat /Users/chen/talkbank/batchalign3/book/src/SUMMARY.md
```

### VS Code Extension Docs
- User guide: `/Users/chen/talkbank/talkbank-chatter/vscode/GUIDE.md`
- Developer guide: `/Users/chen/talkbank/talkbank-chatter/vscode/DEVELOPER.md`
- Feature parity: `/Users/chen/talkbank/talkbank-chatter/vscode/CLAN-FEATURES.md`
- Marketplace README: `/Users/chen/talkbank/talkbank-chatter/vscode/README.md`
- Quick start: `/Users/chen/talkbank/talkbank-chatter/vscode/QUICK_START.md`
- Setup: `/Users/chen/talkbank/talkbank-chatter/vscode/SETUP.md`
- Implementation: `/Users/chen/talkbank/talkbank-chatter/vscode/IMPLEMENTATION_GUIDE.md`

### LSP / CLI Docs
- LSP architecture: `/Users/chen/talkbank/talkbank-chatter/crates/talkbank-lsp/ARCHITECTURE.md`
- LSP debugging: `/Users/chen/talkbank/talkbank-chatter/crates/talkbank-lsp/DEBUGGING.md`
- CLI docs: `/Users/chen/talkbank/talkbank-chatter/docs/cli/README.md`
- LSP docs: `/Users/chen/talkbank/talkbank-chatter/docs/lsp/README.md`

### Error Code Reference (auto-generated from specs)
- Index: `/Users/chen/talkbank/talkbank-chat/docs/errors/index.md`
- Per-code: `/Users/chen/talkbank/talkbank-chat/docs/errors/E*.md` (~189 files)
- Source specs: `/Users/chen/talkbank/talkbank-chat/spec/errors/` (~193 files)

### CLAUDE.md Files (AI context / architecture)
```bash
find /Users/chen/talkbank -name "CLAUDE.md" -not -path "*/node_modules/*" -not -path "*/.git/*" 2>/dev/null
```

### Operations & Infrastructure (private)
- Fleet management: `/Users/chen/talkbank/talkbank-private/batchalign/docs/fleet-management-plan.md`
- Fleet inventory: `/Users/chen/talkbank/talkbank-private/batchalign/docs/fleet-inventory.md`
- Tailscale: `/Users/chen/talkbank/talkbank-private/batchalign/docs/tailscale-cli-migration.md`
- SSH keys: `/Users/chen/talkbank/talkbank-private/batchalign/docs/ssh-key-migration.md`
- Postmortems: `/Users/chen/talkbank/talkbank-private/batchalign/docs/postmortem-*.md`
- Incident reports: `/Users/chen/talkbank/talkbank-private/batchalign/docs/net-*.md`
- Known issues: `/Users/chen/talkbank/batchalign3/book/src/architecture/server-known-issues.md`
- Master doc index: `/Users/chen/talkbank/talkbank-private/docs/DOCS_MAP.md`

### CLAN Analysis Docs
- `/Users/chen/talkbank/talkbank-clan/README.md`
- `/Users/chen/talkbank/talkbank-clan/docs/CLAN-IMPROVEMENTS.md`
- `/Users/chen/talkbank/talkbank-clan/docs/clan-replacement-analysis.md`

### Grammar Docs
- `/Users/chen/talkbank/tree-sitter-talkbank/README.md`
- `/Users/chen/talkbank/tree-sitter-talkbank/CHAT_ANCHORS.md`
- `/Users/chen/talkbank/tree-sitter-talkbank/CONTRIBUTING.md`

## Search Strategy

### 1. Full-text search across all docs
```bash
grep -rn "<query>" /Users/chen/talkbank/talkbank-chat/book/src/ /Users/chen/talkbank/batchalign3/book/src/ /Users/chen/talkbank/talkbank-chatter/vscode/*.md /Users/chen/talkbank/talkbank-chatter/docs/ /Users/chen/talkbank/talkbank-chatter/crates/*/ARCHITECTURE.md /Users/chen/talkbank/talkbank-chatter/crates/*/DEBUGGING.md /Users/chen/talkbank/talkbank-private/batchalign/docs/ --include="*.md" | head -30
```

### 2. Search CLAUDE.md files specifically
```bash
find /Users/chen/talkbank -name "CLAUDE.md" -not -path "*/node_modules/*" -not -path "*/.git/*" -exec grep -ln "<query>" {} \;
```

### 3. Search error codes
```bash
# By code
cat /Users/chen/talkbank/talkbank-chat/docs/errors/E<NNN>.md

# By keyword
grep -rn "<keyword>" /Users/chen/talkbank/talkbank-chat/docs/errors/ --include="*.md" | head -20
```

### 4. Search specs
```bash
grep -rn "<query>" /Users/chen/talkbank/talkbank-chat/spec/ --include="*.md" | head -20
```

### 5. Search README files
```bash
find /Users/chen/talkbank -name "README.md" -not -path "*/node_modules/*" -not -path "*/.git/*" -exec grep -ln "<query>" {} \;
```

## Report

After searching, report:
- Which doc(s) contain the information
- The file path and relevant section
- A brief excerpt of what was found
- Whether the information looks current or potentially stale (check dates)
