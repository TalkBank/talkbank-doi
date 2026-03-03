# TalkBank Development Workspace

Meta-repo for [TalkBank](https://talkbank.org/) — tools for linguistic research on conversational data in CHAT format (Codes for the Human Analysis of Transcripts).

Data flows: spec → grammar (tree-sitter) → Rust (parsers, model, validation) → applications (CLI, LSP, VS Code, Python pipeline).

## Setup

Clone this meta-repo then pull all sub-repos:

```sh
git clone git@github.com:TalkBank/talkbank.git ~/talkbank
cd ~/talkbank
make clone
```

Prerequisites: `gh` CLI (authenticated), Rust toolchain, Node.js, `uv` (Python).

## Repositories

| Directory | GitHub | Description |
|-----------|--------|-------------|
| `tree-sitter-talkbank/` | [TalkBank/tree-sitter-talkbank](https://github.com/TalkBank/tree-sitter-talkbank) | Tree-sitter grammar for CHAT |
| `talkbank-chat/` | [TalkBank/talkbank-chat](https://github.com/TalkBank/talkbank-chat) | CHAT spec and core Rust libraries |
| `talkbank-chatter/` | [TalkBank/chatter](https://github.com/TalkBank/chatter) | CLI, LSP server, VS Code extension |
| `talkbank-clan/` | [TalkBank/clan](https://github.com/TalkBank/clan) | CLAN analysis library |
| `batchalign3/` | [TalkBank/batchalign3](https://github.com/TalkBank/batchalign3) | Alignment and transcription pipeline |
| `batchalign-hk-plugin/` | [TalkBank/batchalign-hk-plugin](https://github.com/TalkBank/batchalign-hk-plugin) | HK deployment plugin |
| `talkbank-private/` | [TalkBank/talkbank-private](https://github.com/TalkBank/talkbank-private) | Internal archive (private) |

## Commands

```sh
make status      # Git status across all repos
make check       # Cargo check all Rust repos
make test        # Run tests across repos
make verify-all  # Full cross-repo verification gate
make clone       # Clone all repos fresh (for new machines)
make pull        # Pull all repos
```

## Environment configuration

Path-sensitive tooling now uses explicit env/config knobs instead of machine-local hardcoded paths.

| Variable | Used by | Default | Purpose |
|----------|---------|---------|---------|
| `CHAT_HTML_PATH` | `talkbank-chat/scripts/check-chat-manual-anchors.sh` | unset | Optional local `chat.html` path for anchor checks. |
| `CHAT_HTML_URL` | `talkbank-chat/scripts/check-chat-manual-anchors.sh` | TalkBank URL | Source URL when `CHAT_HTML_PATH` is not set. |
| `TALKBANK_DATA_ROOT` | `talkbank-private/batchalign/scripts/*` | `~/data` | Root of local corpus checkout for rerun/fix scripts. |
| `WOR_VALIDATION_LOG` | `talkbank-private/batchalign/scripts/analyze_gra_warnings.py` | `~/test.log` | Default validation log path for `%gra` warning analysis. |
| `BA4WAY_OLD_REPO` | `batchalign3/scripts/compare_4way_retokenize.sh` | unset | Optional checkout path for the old Jan baseline variant. |
| `BA4WAY_OLD_PYTHON` | `batchalign3/scripts/compare_4way_retokenize.sh` | `python3` | Python executable used with `BA4WAY_OLD_REPO`. |

## Structure

This meta-repo tracks cross-repo coordination files:
- `Makefile` — cross-repo build/test/status commands
- `RELEASE-PLAN.md` — coordinated release planning
- `analysis/` — workspace-wide audits and reports
- `scripts/` — one-off maintenance scripts

Sub-repos are gitignored with independent git histories. The sibling directory layout is load-bearing — Rust repos use path dependencies that assume this exact structure.

Each sub-repo has its own `CLAUDE.md` with detailed build commands, architecture, and coding standards.
