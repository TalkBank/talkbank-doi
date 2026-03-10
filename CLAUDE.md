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

## Servers We Can Access

| Host | Access | OS | Role |
|------|--------|----|------|
| `net.talkbank.org` | `ssh macw@net` | macOS | Internal (CMU only): local media drives, batchalign server |

`net` is deliberately running `batchalign-next` (Python-only rewrite), installed via `uv tool` at `/Users/macw/.local/bin/batchalign-next`. Will be upgraded to `batchalign3` (Rust) once it's ready. The React dashboard SPA is not deployed there yet.

## Currently Deployed Batchalign Versions

| Path | What | Notes |
|------|------|-------|
| `~/batchalign2-master/` | Current production batchalign2 | Legacy; external/PyPI users |
| `~/batchalign-next/` | Python-only batchalign rewrite (forked from batchalign2) | What net and all internal users run today; different architecture from batchalign3 |
| `batchalign3/` (in this workspace) | Rust-primary rewrite | Not yet released; replaces both of the above |

**Critical baseline commit:** `84ad500b` (2026-01-09) in batchalign2 — the Python optimization push (lazy imports, parallelism, Hirschberg DP, Stanza caching) that is the anchor point for the entire batchalign2→batchalign3 migration. Documented in `batchalign3/book/src/migration/index.md`. A secondary comparison point is `e8f8bfad` (2026-02-09) on batchalign2 master.

Until batchalign3 is released, bug reports and hotfixes may target `~/batchalign-next/` or `~/batchalign2-master/`.

## Migration Status

**GitLab → GitHub:** 16 data repos on git.talkbank.org planned for migration. See `staging/docs/migration-plan.md` and `docs/inventory.md`.

**Legacy docs:** 84 files from ~/Dropbox/documentation/ transferred to `docs/legacy/`. Being reviewed for accuracy — see `docs/legacy/README.md` for status of each.

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
Last Updated: 2026-03-10
