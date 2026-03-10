# talkbank-dev

Private development workspace for [TalkBank](https://talkbank.org/) — the single home for all TalkBank project assets: code repos, corpus data, deploy infrastructure, web sites, legacy tools, and documentation.

## Setup

```sh
# Fresh machine:
git clone git@github.com:TalkBank/talkbank-dev.git ~/talkbank-dev
cd ~/talkbank-dev
make clone           # Clone everything (code + data + web + collaborator)

# Or start minimal:
make clone-minimal   # Just talkbank-tools + batchalign3

# Existing machine with scattered clones:
make adopt --dry-run # Preview what would move
make adopt           # Move ~/staging, ~/webdev, ~/data/*, etc. into workspace
```

Prerequisites: `gh` CLI (authenticated), Rust toolchain, Node.js, `uv` (Python).

## Repositories

### Core Development (public)

| Directory | GitHub | Description |
|-----------|--------|-------------|
| `talkbank-tools/` | [TalkBank/talkbank-tools](https://github.com/TalkBank/talkbank-tools) | Grammar, spec, Rust crates, CLI, LSP, VS Code extension |
| `batchalign3/` | [TalkBank/batchalign3](https://github.com/TalkBank/batchalign3) | NLP pipeline: Rust server + Python ML model server |

### Infrastructure, Tools, CLAN, Web, Data

34 additional repos across infrastructure, pre-commit tools, legacy CLAN, collaborator tools, bank websites, and 16 corpus data repos. See `docs/inventory.md` for the complete list.

## Commands

```sh
make help            # All available targets
make clone           # Clone ALL repos
make clone-minimal   # Just code repos
make clone-data      # Corpus data (16 repos from GitLab)
make status          # Git status across all repos
make pull            # Pull all repos
make check           # Cargo check all Rust workspaces
make test            # Run tests
make verify-all      # Full verification gate
```

## Structure (tracked in this repo)

```
analysis/       Workspace-wide audits and reports
archive/        Archived docs and code
deploy/         Batchalign deploy: Ansible playbooks, scripts, tools
docs/           Internal docs, build notes, legacy documentation
known-issues/   Validation baselines for external corpora
ops/            Operational scripts
scripts/        Maintenance, analysis, and migration scripts
artifacts/      Build artifacts archive
```

All sub-repos are gitignored with independent git histories. The sibling layout is load-bearing — `batchalign3` uses Rust path dependencies assuming `talkbank-tools/` is a sibling.
