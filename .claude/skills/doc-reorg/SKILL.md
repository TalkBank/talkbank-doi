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
for repo in talkbank-tools batchalign3; do
  echo "=== $repo ==="
  find /Users/chen/talkbank/$repo -name "*.md" -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/target/*" 2>/dev/null | wc -l
  echo "files"
done
echo "=== talkbank-dev (internal) ==="
find /Users/chen/talkbank/docs /Users/chen/talkbank/analysis -name "*.md" 2>/dev/null | wc -l
echo "files"
```

### mdBook structure
```bash
echo "=== talkbank-tools book ===" && cat /Users/chen/talkbank/talkbank-tools/book/src/SUMMARY.md
echo "=== batchalign3 book ===" && cat /Users/chen/talkbank/batchalign3/book/src/SUMMARY.md
```

### Find duplicated content
```bash
find /Users/chen/talkbank/talkbank-tools /Users/chen/talkbank/batchalign3 -name "*.md" -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/target/*" | xargs -I {} basename {} | sort | uniq -d
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
for book in talkbank-tools batchalign3; do
  echo "=== $book orphans ==="
  dir="/Users/chen/talkbank/$book/book/src"
  find "$dir" -name "*.md" -not -name "SUMMARY.md" | while read f; do
    rel=$(echo "$f" | sed "s|$dir/||")
    grep -q "$rel" "$dir/SUMMARY.md" || echo "  ORPHAN: $rel"
  done
done
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
grep -rn "<old_filename>" /Users/chen/talkbank/talkbank-tools /Users/chen/talkbank/batchalign3 --include="*.md" -not -path "*/.git/*" -not -path "*/node_modules/*" -not -path "*/target/*"
```

### Consolidating duplicates

Choose the canonical location based on:
- **User docs** → mdBook or repo-level README/GUIDE
- **Developer docs** → CLAUDE.md or mdBook architecture section
- **Ops docs** → talkbank-dev `docs/` directory
- **Quick reference** → README.md (short) pointing to CLAUDE.md (detailed)

Replace duplicates with a one-line pointer:
```markdown
> See [Canonical Location](path/to/canonical.md) for current documentation.
```

### Doc conventions to enforce

| Convention | Rule |
|------------|------|
| Single source of truth | Each fact documented in exactly one place |
| Pointers over copies | Other locations link to canonical, don't duplicate |
| Audience separation | User docs separate from developer docs separate from ops docs |
| Generated files marked | Auto-generated docs say so at top, link to source |
| Dates on living docs | "Last Updated: YYYY-MM-DD" on docs that change |
| CLAUDE.md is for AI | Architecture + commands + gotchas, not user tutorials |

## Step 5: Verify

```bash
# Build mdBooks to catch broken links
cd /Users/chen/talkbank/talkbank-tools/book && mdbook build 2>&1 | grep -i error
cd /Users/chen/talkbank/batchalign3/book && mdbook build 2>&1 | grep -i error

# Check for any remaining references to old paths
grep -rn "<old_path>" /Users/chen/talkbank/talkbank-tools /Users/chen/talkbank/batchalign3 --include="*.md" -not -path "*/.git/*" -not -path "*/node_modules/*" -not -path "*/target/*"
```

## Step 6: Report

Summarize:
- Files moved/renamed (old path → new path)
- Files consolidated (N sources → 1 canonical)
- Files deleted (with reason)
- Cross-references updated (count)
- SUMMARY.md changes
