# DOCX Bookmark Workflow

Source manuals should be edited at DOCX level, then regenerated to HTML/PDF.

Do not patch generated `CHAT.html` / `CLAN.html` directly.

## Why This Matters

Rust code and docs deep-link into manual anchors (for example `CHAT.html#Main_Line`).
If the DOCX source does not contain matching bookmarks, regenerated HTML will not expose stable anchor IDs.

## Tooling Added

- Audit references vs published anchors:
  - `scripts/audit_manual_anchors.py`
- List/add DOCX bookmarks from JSON rules:
  - `scripts/docx_bookmark_tool.py`
- Starter bookmark plans:
  - `scripts/manual-bookmarks/chat-priority.json`
  - `scripts/manual-bookmarks/clan-priority.json`
  - `scripts/manual-bookmarks/ba2-priority.json`

## Dry-Run Bookmark Planning

```bash
# CHAT
python3 scripts/docx_bookmark_tool.py \
  --docx ~/web/talkbank-web/site/0info/manuals/CHAT.docx \
  --rules-json scripts/manual-bookmarks/chat-priority.json

# CLAN
python3 scripts/docx_bookmark_tool.py \
  --docx ~/web/talkbank-web/site/0info/manuals/CLAN.docx \
  --rules-json scripts/manual-bookmarks/clan-priority.json

# BA2 usage
python3 scripts/docx_bookmark_tool.py \
  --docx ~/web/talkbank-web/site/0info/BA2-usage.docx \
  --rules-json scripts/manual-bookmarks/ba2-priority.json
```

## Apply Bookmarks (with Backup)

```bash
python3 scripts/docx_bookmark_tool.py \
  --docx ~/web/talkbank-web/site/0info/manuals/CHAT.docx \
  --rules-json scripts/manual-bookmarks/chat-priority.json \
  --apply --backup
```

The tool inserts `w:bookmarkStart` / `w:bookmarkEnd` into `word/document.xml`.

## Regenerate and Re-Audit

1. Regenerate manual outputs from DOCX (`.html` and `.pdf`) in the web/manuals pipeline.
2. Re-run anchor audit:

```bash
python3 scripts/audit_manual_anchors.py \
  --write-md book/src/developer/manual-anchor-audit.md \
  --write-json /tmp/manual-anchor-audit.json
```

3. Confirm missing anchor count decreases.

## Safety Notes

- Keep `--apply` explicit; default mode is dry-run.
- Use `--backup` for in-place updates.
- For ambiguous paragraph matches, tighten rule text or use `"mode": "exact"` in JSON.
