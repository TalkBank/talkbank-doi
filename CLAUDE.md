# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working in the `talkbank-dev` private workspace.

## Overview

Private development workspace for [TalkBank](https://talkbank.org/) — the single home for all TalkBank project assets. Contains internal docs, deploy scripts, analysis, and known-issue baselines. All code repos, data repos, and tools are gitignored sub-repos with independent histories, cloned via `make clone`.

Data flows: **spec** (source of truth) → **grammar** (tree-sitter) → **Rust** (parsers, model, validation) → **applications** (CLI, LSP, VS Code, Python pipeline).

## Workspace Layout

```
talkbank-dev/                         # This repo (private, tracked in git)
├── CLAUDE.md                         # Private workspace guidance (this file)
├── Makefile                          # Cross-repo orchestration
├── analysis/                         # Workspace-wide audits and reports
├── archive/                          # Archived docs (plugin system, old tools docs)
├── deploy/                           # Batchalign deploy: Ansible, scripts, tools
├── docs/                             # Internal docs, build notes, reports
│   ├── build-notes/                  # Phon/PhonTalk build instructions
│   ├── inventory.md                  # Complete project inventory (all repos, all platforms)
│   └── legacy/                       # 84 docs from ~/Dropbox/documentation/ (being reviewed)
├── known-issues/                     # Validation baselines for external corpora
├── ops/                              # Operational scripts
├── scripts/                          # Maintenance, analysis, and migration scripts
│   └── analysis/                     # Corpus inspection tools
├── artifacts/                        # Build artifacts archive
│
│ # ── Gitignored sub-repos (cloned by make clone) ──
│
│ # Core development (public)
├── talkbank-tools/                   # Grammar, spec, Rust crates, CLI, LSP, VS Code
├── batchalign3/                      # NLP pipeline: Rust server + Python ML server
│
│ # Infrastructure & deployment
├── staging/                          # Deploy orchestration (GitLab-hosted, migrating)
├── webdev/                           # Web config generation and deploy
├── gra-cgi/                          # MOR/GRA diagram CGI script
├── sync-media/                       # rclone media sync (gandalf↔media server)
├── generate-from-chat/               # Build tool: corpus ZIPs, DOI injection
├── cdcs-to-csv/                      # DOI lifecycle (DataCite API)
│
│ # Pre-commit / build tools
├── update-chat-types/                # Rust: @Types header updater (has pre-commit hook)
├── save-word-html-pdf/               # macOS Word→HTML/PDF export (pre-commit candidate)
├── talkbank-xml-schema/              # Legacy XML Schema (Phon may still reference)
│
│ # CHAT tools
├── java-chatter-stable/              # Canonical Java chatter (ANTLR 3)
├── talkbank-ipa-fragments/           # IPA fragment extraction (Java)
│
│ # Legacy CLAN
├── OSX-CLAN/                         # CLAN C/C++ source (Leonid)
├── clan-info/                        # CLAN supporting materials
│
│ # Collaborator repos
├── phon/                             # Phon Java source (phon-ca/phon)
├── phontalk/                         # PhonTalk converter (phon-ca/phontalk)
│
│ # Web
├── web/                              # Bank websites (mani-managed, 17 sub-repos)
├── talkbank-browser-check/           # Link/404 checker with login
│
│ # Corpus data (16 repos, currently GitLab, migrating to GitHub)
└── data/
    ├── childes-data/                 #   52k files — child language
    ├── phon-data/                    #   26k files — phonology (+ 0phon/ XML sources)
    ├── ca-data/                      #   11k files — conversation analysis
    ├── aphasia-data/                 #   11k files — aphasia
    ├── slabank-data/                 #   10k files — second language acquisition
    ├── dementia-data/                #    7k files — dementia
    ├── homebank-data/                #    6k files — home recordings
    └── ... (+ fluency, class, biling, asd, tbi, psychosis, rhd, samtale, motor)
```

## Cross-Repo Commands

```bash
make help            # Show all available targets
make clone           # Clone ALL repos (code + data + web + collaborator)
make clone-minimal   # Just talkbank-tools + batchalign3
make clone-code      # All code repos (no data, no web)
make clone-data      # Corpus data repos from GitLab
make clone-web       # Web/bank repos
make adopt           # Move existing ~/repo clones into workspace (dry-run first!)
make status          # Git status across all repos
make pull            # Pull all repos
make check           # Cargo check all Rust workspaces
make test            # Run tests across repos
make verify-all      # Full cross-repo verification gate
```

## Cross-Repo Path Dependencies

`talkbank-tools` is self-contained. `batchalign3` uses local path dependencies into `talkbank-tools`:

```
talkbank-tools/crates/           # Source crates
batchalign3/crates/              # batchalign-chat-ops (path deps → talkbank-tools/crates/)
batchalign3/pyo3/                # batchalign-core PyO3 bridge (path deps → talkbank-tools/crates/)
```

`OSX-CLAN` is referenced by talkbank-tools tests (golden tests, CHECK parity audit, database integration). Tests skip gracefully when not present.

## Validation Baselines

`known-issues/` contains expected validation failure lists for external corpora:

```bash
# Compare current results against baseline (run from talkbank-tools/):
diff <(cargo run --release -p talkbank-cli -- validate ../data/phon-data/ --force 2>&1 \
       | grep '✗' | sed 's/✗ Errors found in //' | sort) \
     ../known-issues/phon-data-validation-baseline.txt
```

See `known-issues/README.md` for creating and updating baselines.

## Analysis Scripts

```bash
python3 scripts/analysis/scan_phon_mismatches.py data/phon-data        # Phon XML↔IPA mismatches
python3 scripts/analysis/count_tier_coverage.py data/phon-data          # Tier frequency report
scripts/analysis/diff_validator_runs.sh baseline data/phon-data         # Save baseline
scripts/analysis/diff_validator_runs.sh compare data/phon-data          # Compare after changes
```

## Large-Scale Corpus Testing

```bash
# From talkbank-tools/:
cargo run --release -p talkbank-cli -- validate ../data/ --force             # Validation only
cargo run --release -p talkbank-cli -- validate ../data/ --roundtrip --force  # + roundtrip
cargo run --release -p talkbank-cli -- validate ../data/ --skip-alignment     # Faster
```

## Deployment

### Deploy Scripts

All deploy scripts live in `deploy/scripts/`. Read these scripts before deploying — they are the source of truth for how deployment works.

| Script | What | Target |
|--------|------|--------|
| `deploy/scripts/deploy_server.sh` | Deploy batchalign-next to server (net) | `bash deploy/scripts/deploy_server.sh` |
| `deploy/scripts/deploy_clients.sh` | Deploy batchalign-next to client machines | `bash deploy/scripts/deploy_clients.sh` |
| `deploy/scripts/deploy_batchalign3_server.sh` | Deploy batchalign3 (Rust) to server | `bash deploy/scripts/deploy_batchalign3_server.sh` |
| `deploy/scripts/deploy_batchalign3_clients.sh` | Deploy batchalign3 (Rust) to clients | `bash deploy/scripts/deploy_batchalign3_clients.sh` |

All scripts support `--dry-run`, `--no-build`, and explicit host arguments. Run with `--help` for full usage.

**Fleet install policy:** batchalign3 deploy scripts always install with `[hk]` extras (HK/Cantonese engines: Tencent, Aliyun, FunASR, Cantonese FA). All fleet machines get the full engine set.

### Fleet Machines

| Host | Role | Access |
|------|------|--------|
| `net` | Server (production) | `ssh macw@net` |
| `bilbo`, `brian`, `davida`, `frodo`, `andrew`, `lilly`, `sue`, `vaishnavi` | Clients | `ssh macw@<host>` |

### Servers We Can Access

| Host | Access | OS | Role |
|------|--------|----|------|
| `net.talkbank.org` | `ssh macw@net` or `ssh macw@talkbank` | Ubuntu 26.04 (Mac Studio, M3 Ultra, 256 GB) | Internal (CMU only): local media drives, batchalign server, web (nginx) |

`net` runs `batchalign-next` on port 8000 (Python 3.12). Being upgraded to `batchalign3` (Rust) on port 8001 (coexistence), then port 8000 (takeover).

**Full server reference:** `docs/net-talkbank-server.md` — services (nginx, fcgiwrap, CGI, Tailscale, batchalign), OS upgrade checklist, and comparison with homebank.

### Batchalign Repos and Deployed Versions

| Repo / Path | What | Notes |
|------|------|-------|
| `~/batchalign-next/` | batchalign-next source repo (Python) | `uv build --wheel` produces the wheel |
| `~/talkbank-utils/` | talkbank-utils source repo (Rust + grammar) | Old clone of talkbank-tools; **batchalign-next builds batchalign-core from `~/talkbank-utils/rust/crates/batchalign-core/`** |
| `batchalign3/` (in this workspace) | Rust-primary rewrite source | Also builds batchalign-core at `pyo3/` |
| `~/batchalign2-master/` | Legacy batchalign2 | External/PyPI users only |

**batchalign-next deploy builds two wheels:** the Python wheel from `~/batchalign-next/` and the batchalign-core Rust wheel from `~/talkbank-utils/rust/crates/batchalign-core/` (via maturin). Both are installed together on fleet machines via `uv tool install`.

**Critical baseline commit:** `84ad500b` (2026-01-09) in batchalign2 — the Python optimization push (lazy imports, parallelism, Hirschberg DP, Stanza caching) that is the anchor point for the entire batchalign2→batchalign3 migration. Documented in `batchalign3/book/src/migration/index.md`. A secondary comparison point is `e8f8bfad` (2026-02-09) on batchalign2 master.

Until batchalign3 is released, bug reports and hotfixes may target batchalign-next on the fleet machines.

### Postmortems

Incident reports live in `docs/postmortems/`. Check these before deploying to understand past failures.

## Migration Status

**GitLab → GitHub:** 16 data repos on git.talkbank.org planned for migration. See `staging/docs/migration-plan.md` and `docs/inventory.md`.

**Legacy docs:** 84 files from ~/Dropbox/documentation/ transferred to `docs/legacy/`. Being reviewed for accuracy — see `docs/legacy/README.md` for status of each.

## Documentation Conventions

**Date every document you touch.** All markdown docs across the workspace must include a date header. Use this frontmatter block at the top of every doc (after the `#` title):

```
**Status:** Current | Historical | Reference | Draft
**Last updated:** YYYY-MM-DD
```

- **Current** — actively maintained, reflects reality
- **Historical** — preserved for context, no longer reflects current state
- **Reference** — stable reference material (signing guides, build notes, inventories)
- **Draft** — work in progress

**Rules:**
- When you create a new doc, add the date header.
- When you edit an existing doc, update `Last updated` to today. If the doc has no date header, add one.
- Use ISO 8601 dates only (`2026-03-12`), never prose dates (`February 15, 2026`).
- Do NOT do a bulk sweep to stamp dates on docs you haven't verified — that creates false confidence. Only date docs you've actually read and confirmed are accurate.
- Postmortems use the date-in-filename convention (`YYYY-MM-DD-description.md`) and don't need a separate date header.

## Per-Repo Guidance

Coding standards live in each repo's CLAUDE.md:

| Repo | CLAUDE.md files |
|------|----------------|
| `talkbank-tools/` | root + `grammar/` + `spec/` + `spec/tools/` + `vscode/` (5 files) |
| `batchalign3/` | root + `crates/batchalign-chat-ops/` + `pyo3/` (3 files) |
| `staging/` | root (1 file) |
| `webdev/` | root (1 file) |

Full project inventory: `docs/inventory.md`

---
Last Updated: 2026-03-12

