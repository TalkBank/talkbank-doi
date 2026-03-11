# Manual Anchor Maintenance

This page summarizes the manual-anchor health check used to keep CHAT and CLAN
manual links stable inside the codebase and book.

The raw run metadata, local source locations, and generated machine-readable
audit outputs are archived privately for maintainers. The public docs keep only
the durable result and maintenance guidance.

## Latest Public Summary

As of the most recent audit run used for this page:

| Manual | Anchors Used In Code/Docs | Missing In Published HTML |
|---|---:|---:|
| CHAT | 242 | 0 |
| CLAN | 9 | 0 |

The public conclusion is simple: all currently referenced manual anchors had a
matching published HTML anchor at the time of the last audit.

## What This Check Protects

- links from code comments into the TalkBank manuals
- links from book chapters into CHAT and CLAN reference sections
- stability of manual-derived deep links across HTML regeneration

## Maintenance Guidance

- re-run the anchor audit whenever manual links are added or changed
- keep the audit in CI or the release checklist
- prefer stable semantic bookmark names where the manual export supports them
- avoid introducing local-only or maintainer-specific paths into public docs
