# Documentation Map

This file is the canonical index of documentation across the TalkBank workspace.
Updated after the doc migration (2026-02-23) that moved public docs from
`talkbank-private/` to their respective repos.

---

## Quick Reference — Where Docs Live Now

| Repo | Doc system | What's there |
|------|-----------|--------------|
| `talkbank-chat/book/` | mdBook | CHAT spec, architecture, parser internals, audits, contributing guides |
| `batchalign3/book/` | mdBook | Pipeline architecture, benchmarks, decisions, developer guides |
| `talkbank-chatter/docs/` | Plain markdown | LSP improvements |
| `talkbank-clan/` | — | CLAUDE.md only |
| `tree-sitter-talkbank/` | — | CLAUDE.md only |
| `talkbank-private/` | Plain markdown | **Private only:** ops/fleet docs, audit workspaces, archives, and historical notes |

---

## talkbank-chat (`book/src/`)

### Spec-Generated (do not hand-edit)

| Category | Path | Count | Maintained by |
|----------|------|------:|---------------|
| Error specs | `spec/errors/*.md` | ~193 | `make test-gen` |
| Error docs | `docs/errors/*.md` | ~189 | `make test-gen` |
| Construct specs | `spec/constructs/**/*.md` | ~164 | Author of grammar change |

### mdBook Sections

| Section | Path | Contents |
|---------|------|----------|
| User Guide | `book/src/user-guide/` | Installation, CLI, validation errors, VS Code, batch workflows |
| CHAT Format | `book/src/chat-format/` | Headers, utterances, dependent tiers, word syntax, symbols |
| Architecture | `book/src/architecture/` | Spec system, grammar, parsing, data model, error system, crate reference |
| Architecture (analysis) | `book/src/architecture/` | Executive summary, repo architecture, grammar governance, risk register, etc. (moved from `talkbank-private/analysis/`) |
| Architecture (audits) | `book/src/architecture/` | Check parity, error code, sentinel, UTF-8, validation audits (moved from `talkbank-private/audits/`) |
| Architecture (legacy archive) | `docs/archive/book-src/architecture/legacy/` | 22 historical reference docs (moved out of published mdBook) |
| Architecture (repo-split) | `book/src/architecture/repo-split/` | Repo split strategy and scripts (moved from `talkbank-private/repo-split/`) |
| Contributing | `book/src/contributing/` | Setup, workflows, coding standards, quality gates, testing, maintenance model |
| Integrating | `book/src/integrating/` | Library usage, JSON schema, diagnostic contract |

### AI Context Files

| File | Scope |
|------|-------|
| `CLAUDE.md` | Root-level repo guidance |
| `spec/CLAUDE.md` | Spec system |
| `spec/tools/CLAUDE.md` | Spec generators |
| `crates/CLAUDE.md` | Rust coding standards |
| `crates/*/CLAUDE.md` | Per-crate guidance (5 crates) |
| `.github/copilot-instructions.md` | GitHub Copilot |

---

## batchalign3 (`book/src/`)

### mdBook Sections

| Section | Path | Contents |
|---------|------|----------|
| User Guide | `book/src/user-guide/` | Installation, quick start, CLI, server mode, troubleshooting |
| Architecture | `book/src/architecture/` | Pipeline, dispatch, caching, validation, server, error handling |
| Architecture (moved) | `book/src/architecture/` | Caching report, server model loading/perf/known issues, validation UX audit |
| Technical Reference | `book/src/reference/` | CHAT format, morphosyntax, forced alignment, %wor tier, multilingual, concurrency |
| Reference (moved) | `book/src/reference/` | Benchmarks, throughput, align/morphotag analysis, language routing, tier leniency, %wor audits |
| Developer Guide | `book/src/developer/` | Building, testing, engines, Rust core, Python versioning, pre-commit |
| Developer (moved) | `book/src/developer/` | Deployment, 3.14t migration, Python-Rust discrepancies, launchd/server templates |
| Design Decisions | `book/src/decisions/` | Rust migration, server orchestration, correctness campaign, lenient parsing |
| Decisions (moved) | `book/src/decisions/` | 25 decision docs: cache/rust migration, provenance types, string audit, etc. |

### AI Context Files

| File | Scope |
|------|-------|
| `CLAUDE.md` | Root-level pipeline guidance |
| `rust/CLAUDE.md` | Rust extension (batchalign_core) |

---

## talkbank-chatter

| Path | Contents |
|------|----------|
| `docs/lsp/improvements.md` | LSP improvement roadmap (moved from `talkbank-private/lsp-internal/`) |
| `crates/talkbank-cli/CLAUDE.md` | CLI guidance |
| `crates/talkbank-lsp/CLAUDE.md` | LSP guidance |

---

## talkbank-private (this repo)

After the migration, this repo is primarily private/infrastructure content, plus
audit workspaces and explicitly historical archives:

### Fleet & Deployment (private)

| Path | Contents |
|------|----------|
| `batchalign/ansible/` | Ansible playbooks, roles, inventory, group_vars |
| `batchalign/scripts/deploy_server.sh` | Server deploy script (hardcoded hostnames) |
| `batchalign/scripts/deploy_clients.sh` | Client deploy script (hardcoded hostnames) |
| `batchalign/docs/fleet-inventory.md` | Fleet roster with IPs |
| `batchalign/docs/fleet-management-plan.md` | Fleet architecture |
| `batchalign/docs/ssh-key-migration.md` | SSH key state |
| `batchalign/docs/tailscale-cli-migration.md` | Tailscale auth/ACL |
| `batchalign/docs/cluster-setup.md` | Historical BA-next cluster setup note |

### Release Audit Workspace

| Path | Contents |
|------|----------|
| `docs/release-doc-audit/` | inventories, evidence captures, audit matrices, promotion candidates, and the current pause/resume note (`STATUS-2026-03-09.md`) |

### Internal Reports (borderline private)

| Path | Contents |
|------|----------|
| `docs/archive/batchalign/root-notes/` | Archived status reports, overnight plans, private correspondence |
| `docs/archive/batchalign/reports/` | Archived master-branch audit/status snapshots |
| `batchalign/docs/net-job-failures-*.md` | Incident reports |
| `docs/comment-quality-tracker.md` | Workspace-wide plan/tracker for manual comment quality campaign |

### CLAUDE.md Templates

| Path | Contents |
|------|----------|
| `claude-md/*/CLAUDE.md` | Source copies of CLAUDE.md files (now deployed to repos) |
| `copilot-instructions/` | Source copies of copilot instructions |

### Borderline Technical Notes

Some private notes contain durable technical detail that may belong in public
docs after rewrite and verification. Promotion candidates are tracked in the
release audit workspace rather than treated as live public docs.

| Path | Likely destination |
|------|-------------------|
| `batchalign/docs/morphotag-divergences-2026-02-16.md` | batchalign3 reference (after scrub) |
| `docs/batchalign3-reports/root/BATCHALIGN2_DELTA.md` | batchalign3 migration docs |
| `archive/talkbank-tools-docs/dependent-tier-semantics-audit.md` | talkbank-tools CHAT-format docs |

---

## Deleted Files (Phase 7)

The following ephemeral files were deleted on 2026-02-23:

- `agents/*.md` — superseded by per-repo CLAUDE.md files
- `batchalign/FOCUSED_BENCHMARK_PLAN.md` — executed, done
- `batchalign/SESSION_SUMMARY_2026-02-15.md` — session-specific
- `batchalign/SUNDAY_WORK_SUMMARY.md` — session-specific
- `batchalign/sample-command.sh` — snippet
- `batchalign/run-wor-rerun.sh` — one-time script
- `batchalign/docs/cancel-bug.md` — fixed bug
- `batchalign/docs/e704-overlap-bug.md` — fixed bug
- `batchalign/docs/mp4-symlink-bug.md` — fixed bug
- `batchalign/docs/wor-tier-bug-report.md` — fixed bug
- `transcripts/wor-refactor-plan-transcript.jsonl` — 78KB transcript
- `docs/CONSOLIDATION_LOG.md` — migration bookkeeping
- `audits/index.md` — index file for moved audits

---

*Last Updated: 2026-03-09*
