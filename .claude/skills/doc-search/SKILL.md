---
name: doc-search
description: Search across all TalkBank project documentation — mdBooks, READMEs, CLAUDE.md files, specs, error docs, and internal docs. Use when the user asks "where is the doc for X" or "what does the docs say about Y".
disable-model-invocation: true
allowed-tools: Bash, Read, Glob, Grep, Agent
---

# Search TalkBank Documentation

Find documentation across the entire TalkBank workspace. `$ARGUMENTS` is the search query.

## Documentation Map

### mdBooks (structured, hierarchical)

| Book | Path | Audience |
|------|------|----------|
| TalkBank Tooling | `talkbank-tools/book/src/` | CHAT format, parsers, validation, CLAN |
| Batchalign | `batchalign3/book/src/` | Pipeline, server, deployment |

To see what's in each book:
```bash
cat /Users/chen/talkbank/talkbank-tools/book/src/SUMMARY.md
cat /Users/chen/talkbank/batchalign3/book/src/SUMMARY.md
```

### VS Code Extension Docs
- User guide: `talkbank-tools/vscode/GUIDE.md`
- Developer guide: `talkbank-tools/vscode/DEVELOPER.md`
- Feature parity: `talkbank-tools/vscode/CLAN-FEATURES.md`
- Marketplace README: `talkbank-tools/vscode/README.md`

### Error Code Reference (auto-generated from specs)
- Index: `talkbank-tools/docs/errors/index.md`
- Per-code: `talkbank-tools/docs/errors/E*.md` (~206 files)
- Source specs: `talkbank-tools/spec/errors/` (~214 files)

### CLAUDE.md Files (AI context / architecture)
```bash
find /Users/chen/talkbank -name "CLAUDE.md" -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/target/*" 2>/dev/null
```

Key CLAUDE.md files:
- `talkbank-tools/CLAUDE.md` — grammar, parsers, validation, CLAN, CLI, LSP
- `talkbank-tools/grammar/CLAUDE.md` — tree-sitter grammar specifics
- `talkbank-tools/spec/CLAUDE.md` — spec authoring guide
- `talkbank-tools/vscode/CLAUDE.md` — VS Code extension
- `batchalign3/CLAUDE.md` — Rust server, Python workers, pipeline

### Internal Docs (talkbank-dev, not public)
- Analysis reports: `analysis/`
- Deploy scripts: `deploy/`
- Internal docs: `docs/`
- Known issues baselines: `known-issues/`
- Internal scripts: `scripts/internal/`

### Spec Files
- Construct specs: `talkbank-tools/spec/constructs/` (~104 files)
- Error specs: `talkbank-tools/spec/errors/` (~214 files)
- Symbol registry: `talkbank-tools/spec/symbols/symbol_registry.json`

## Search Strategy

### 1. Full-text search across all docs
```bash
grep -rn "<query>" /Users/chen/talkbank/talkbank-tools/book/src/ /Users/chen/talkbank/batchalign3/book/src/ /Users/chen/talkbank/talkbank-tools/vscode/*.md /Users/chen/talkbank/talkbank-tools/docs/errors/ --include="*.md" | head -30
```

### 2. Search CLAUDE.md files specifically
```bash
find /Users/chen/talkbank -name "CLAUDE.md" -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/target/*" -exec grep -ln "<query>" {} \;
```

### 3. Search error codes
```bash
# By code
cat /Users/chen/talkbank/talkbank-tools/docs/errors/E<NNN>.md

# By keyword
grep -rn "<keyword>" /Users/chen/talkbank/talkbank-tools/docs/errors/ --include="*.md" | head -20
```

### 4. Search specs
```bash
grep -rn "<query>" /Users/chen/talkbank/talkbank-tools/spec/ --include="*.md" | head -20
```

### 5. Search README files
```bash
find /Users/chen/talkbank -name "README.md" -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/target/*" -exec grep -ln "<query>" {} \;
```

### 6. Search internal docs
```bash
grep -rn "<query>" /Users/chen/talkbank/docs/ /Users/chen/talkbank/analysis/ --include="*.md" | head -20
```

## Report

After searching, report:
- Which doc(s) contain the information
- The file path and relevant section
- A brief excerpt of what was found
- Whether the information looks current or potentially stale (check dates)
