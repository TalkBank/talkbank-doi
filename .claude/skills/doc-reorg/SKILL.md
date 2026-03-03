---
name: doc-reorg
description: Reorganize documentation structure — consolidate scattered docs, restructure mdBook sections, update the doc index, or migrate docs between locations. Use when docs are fragmented, duplicated, or hard to find.
disable-model-invocation: true
allowed-tools: Bash, Read, Write, Edit, Glob, Grep, Agent
---

# Reorganize Documentation

Restructure, consolidate, or migrate documentation. `$ARGUMENTS` should describe the reorganization goal (e.g., "consolidate all deployment docs" or "restructure batchalign book architecture section").

## Step 1: Audit Current State

### Full documentation inventory

```bash
# All markdown files by repo
for repo in talkbank-chat talkbank-chatter talkbank-clan batchalign3 tree-sitter-talkbank talkbank-private; do
  echo "=== $repo ==="
  find /Users/chen/talkbank/$repo -name "*.md" -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/target/*" 2>/dev/null | wc -l
  echo "files"
done
```

### mdBook structure
```bash
echo "=== talkbank-chat book ===" && cat /Users/chen/talkbank/talkbank-chat/book/src/SUMMARY.md
echo "=== batchalign3 book ===" && cat /Users/chen/talkbank/batchalign3/book/src/SUMMARY.md
```

### Find duplicated content
```bash
# Find files with similar names across repos
find /Users/chen/talkbank -name "*.md" -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/target/*" | xargs -I {} basename {} | sort | uniq -d
```

### Check the master doc index
```bash
cat /Users/chen/talkbank/talkbank-private/docs/DOCS_MAP.md
```

## Step 2: Identify Problems

Common documentation problems:
- **Duplication**: Same information in multiple places (e.g., build commands in README + CLAUDE.md + book)
- **Fragmentation**: Related docs scattered across repos
- **Orphaned docs**: Files not linked from any index or SUMMARY.md
- **Stale docs**: Content that no longer matches the code
- **Wrong audience**: Developer docs mixed with user docs
- **Missing docs**: Features without documentation

### Check for orphaned mdBook pages
```bash
# Pages in directory but not in SUMMARY.md
for book in talkbank-chat batchalign3; do
  echo "=== $book orphans ==="
  dir="/Users/chen/talkbank/$book/book/src"
  find "$dir" -name "*.md" -not -name "SUMMARY.md" | while read f; do
    rel=$(echo "$f" | sed "s|$dir/||")
    grep -q "$rel" "$dir/SUMMARY.md" || echo "  ORPHAN: $rel"
  done
done
```

### Check for dead links
```bash
# Simple check: find markdown links and verify targets exist
grep -rn '\[.*\](\./\|\.\./' /Users/chen/talkbank/<repo>/book/src/ --include="*.md" | head -30
```

## Step 3: Plan the Reorganization

Before making changes, create a plan:

1. **What moves where** — source path → destination path
2. **What gets consolidated** — which duplicates become one source of truth
3. **What gets deleted** — truly stale or superseded docs
4. **What gets created** — new index pages, redirects, cross-references
5. **What SUMMARY.md changes** — for mdBook reorganizations

## Step 4: Execute the Plan

### Moving files

When moving docs, update ALL references:
```bash
# Find all references to the old path
grep -rn "<old_filename>" /Users/chen/talkbank/ --include="*.md" -not -path "*/.git/*" -not -path "*/node_modules/*" -not -path "*/target/*"
```

### Consolidating duplicates

Choose the canonical location based on:
- **User docs** → mdBook or repo-level README/GUIDE
- **Developer docs** → CLAUDE.md or mdBook architecture section
- **Ops docs** → talkbank-private/batchalign/docs/
- **Quick reference** → README.md (short) pointing to CLAUDE.md (detailed)

Replace duplicates with a one-line pointer:
```markdown
> See [Canonical Location](path/to/canonical.md) for current documentation.
```

### mdBook reorganization

1. Move files to new locations
2. Update SUMMARY.md
3. Update all internal cross-references
4. Build to verify: `cd book && mdbook build 2>&1 | grep -i error`

### Doc conventions to enforce

| Convention | Rule |
|------------|------|
| Single source of truth | Each fact documented in exactly one place |
| Pointers over copies | Other locations link to canonical, don't duplicate |
| Audience separation | User docs separate from developer docs separate from ops docs |
| Generated files marked | Auto-generated docs say so at top, link to source |
| Dates on living docs | "Last Updated: YYYY-MM-DD" on docs that change |
| CLAUDE.md is for AI | Architecture + commands + gotchas, not user tutorials |

## Step 5: Update the Master Index

After reorganization, update:
```bash
# The master doc map
cat /Users/chen/talkbank/talkbank-private/docs/DOCS_MAP.md
```

Add/update entries for moved or new docs. Remove entries for deleted docs.

## Step 6: Verify

```bash
# Build mdBooks to catch broken links
cd /Users/chen/talkbank/talkbank-chat/book && mdbook build 2>&1 | grep -i error
cd /Users/chen/talkbank/batchalign3/book && mdbook build 2>&1 | grep -i error

# Check for any remaining references to old paths
grep -rn "<old_path>" /Users/chen/talkbank/ --include="*.md" -not -path "*/.git/*" -not -path "*/node_modules/*" -not -path "*/target/*"
```

## Step 7: Report

Summarize:
- Files moved/renamed (old path → new path)
- Files consolidated (N sources → 1 canonical)
- Files deleted (with reason)
- Cross-references updated (count)
- SUMMARY.md changes
- DOCS_MAP.md updates
