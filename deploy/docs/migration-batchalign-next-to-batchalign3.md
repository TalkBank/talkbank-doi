# Historical Draft: batchalign-next -> batchalign3

> **Status:** Superseded on 2026-03-09.
>
> This private draft predates the final public release docs. It includes
> package/layout assumptions, UI plans, and runtime details that are no longer
> authoritative after the public-doc audit and the final fold-in work.

## Do Not Use This As Current Release Guidance

The original draft described several things that either changed materially or
did not ship in the form written here, including:

- plugin-era HK packaging
- pre-release dashboard and TUI assumptions
- fleet-specific rollout details
- early package-split and config-dir claims that were later revised

## Use These Current Sources Instead

- public migration book:
  `../batchalign3/book/src/migration/`
- public user docs:
  `../batchalign3/book/src/user-guide/`
- private deployment/runbook material:
  `../ops/batchalign3/`

## Durable Points That Remained True

The draft was directionally right about a few stable migration themes:

- the public CLI name is `batchalign3`
- `~/.batchalign.ini` remains the user-facing config file for engine defaults
  and provider credentials
- the current architecture is Rust control plane plus Python ML workers
- relative to the Jan 9, 2026 Batchalign2 baseline, `batchalign3` adds first-class
  operational surfaces such as `serve`, `jobs`, `logs`, `openapi`, and `cache`

## Why This File Is Kept

This file remains only as provenance for internal migration planning. When
pulling material from it into public docs, re-verify every claim against:

- current CLI help
- current code paths
- current public migration pages
