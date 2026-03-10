# Documentation Readiness

**Status:** Current  
**Last verified:** 2026-03-05

This page is a current snapshot of documentation readiness for contributors and
release reviewers. It is not a frozen February 2026 assessment.

## Executive summary

- **Overall status:** strong enough for public use, with the migration book and
  release-surface docs now substantially cleaner than the earlier assessment set
- **Main strength:** current code, user guide, migration book, and architecture
  pages are much more aligned around the Rust control plane plus Python worker
  model
- **Main remaining work:** continued trimming or de-navving of historical pages
  whose migration-relevant substance has already been preserved elsewhere

## Current public documentation shape

The public book now has a cleaner division of responsibility:

- **Migration Book:** authoritative BA2 -> BA3 public history
- **User Guide:** current release-surface and operational behavior
- **Architecture:** implemented control-plane and subsystem behavior
- **Technical Reference:** current reference material plus clearly labeled
  historical pages where public context still matters
- **Developer Guide:** active contributor workflows
- **Design Decisions:** current ADRs plus explicitly labeled historical decisions

## Current scale

- pages in `book/src`: 93
- pages in `SUMMARY.md` navigation: 86
- public archive pages in `docs/archive/book-src`: 33
- Rust source files in `rust/`: 84
- Rust source files in `rust-next/`: 144

## What improved since the earlier assessment

- the migration book is now explicitly anchored to:
  - Jan 9, 2026 BA2 baseline `84ad500...`
  - Feb 9, 2026 released BA2 master point `e8f8bfa...` where needed
  - current BA3
- command-surface and release-surface docs now reflect the first-release stance:
  - `fleet` removed
  - `gui` launcher removed
  - web dashboard kept as real current functionality
  - desktop/Tauri wrapper documented as deferred
- historical pages around UTR, master-tier output, and similar Python behavior
  are now labeled more clearly
- contributor docs now better distinguish current truth from proposal-era
  migration language

## Manual linkage quality

Batchalign documentation remains tightly linked to the official TalkBank manuals:

- [BA2 usage guide](https://talkbank.org/0info/BA2-usage.pdf)
- [CHAT manual](https://talkbank.org/0info/manuals/CHAT.html)
- [CLAN manual](https://talkbank.org/0info/manuals/CLAN.html)

The manual-anchor audit remains part of the documentation quality story:

- [Manual Anchor Audit](manual-anchor-audit.md)
- `analysis/manual-anchor-audit.json`

## What this means for contributors

A contributor can now more reliably:

- find current runtime behavior without stepping through stale rollout prose
- distinguish public migration history from internal archaeology
- map user-visible claims back to current code paths
- navigate from subsystem docs to manual clauses and tests with less ambiguity

## Remaining gaps

The main remaining gaps are now mostly governance and curation, not basic
technical accuracy:

- missing or incomplete repo process docs such as `CONTRIBUTING.md`
- continued public/private triage of historical pages
- continued nav cleanup so historical references do not compete with current
  migration and reference pages

## Related tracking

- [Documentation Scope](documentation-scope.md)
- `analysis/docs-revision-log.md`
- `analysis/docs-revision-matrix-2026-03-05.md`
