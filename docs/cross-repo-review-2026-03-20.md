# Cross-Repo Review Status (2026-03-20)

This note tracks the pre-push review and history cleanup across the two nested
repos that changed after their last push to `origin/main`.

## Repos

### `batchalign3`

- history rewritten into thematic code commits covering:
  - alignment/transcribe/language behavior
  - typed IPC schema and generated bindings
  - persistent worker transport and GPU runtime controls
  - dashboard/TUI observability
  - domain newtypes and OOM hardening
- durable review docs live in:
  - `batchalign3/book/src/developer/architecture-audit.md`
  - `batchalign3/book/src/developer/performance-and-rearchitecture-backlog.md`
- Houjun read path remains the migration and architecture book pages

### `talkbank-tools`

- history rewritten into thematic code commits covering:
  - alignment walker and overlap audit redesign
  - header language typing and validation/spec reconciliation
- durable review docs live in:
  - `talkbank-tools/book/src/contributing/architecture-audit.md`
  - `talkbank-tools/book/src/contributing/rearchitecture-backlog.md`

## Shared Themes To Use In The Next Session

- remove remaining silent or stringly typed boundary behavior
- keep type conversion close to entry boundaries
- keep worker/concurrency ownership narrow and explicit
- make heavy test suites and regeneration workflows more deliberate
- prefer actor/reducer-style state transitions over shared mutable coordination

## Next Canonical Input

- ordered radical rewrite program:
  - `docs/pre-release-radical-rearchitecture-program.md`
