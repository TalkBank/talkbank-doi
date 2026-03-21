# CLAUDE.md

**Last modified:** 2026-03-21 12:13 EDT

This file provides guidance to Claude Code (claude.ai/code) when working in the `talkbank-dev` private workspace.

## Succession-Aware Design (applies to ALL decisions)

The entire TalkBank core team (Brian, Leonid, Davida, Chen, John) will retire simultaneously in 3-5 years. An external professor will inherit everything. **Every system must be operable by someone who has never met us.**

These principles override convenience. When in doubt, choose the option that makes handover easier:

1. **No snowflakes.** Every server configuration must be reproducible from code. If a VM dies, rebuild from a repo, not from memory.
2. **No gatekeepers.** Every routine operation (adding a corpus, minting a DOI, deploying) must work without SSH to a specific machine.
3. **No single points of failure.** No person, machine, or account whose loss makes TalkBank inoperable.
4. **Prefer managed services over self-hosted.** GitHub over self-hosted GitLab. Cloud storage over local drives. Fewer things to maintain.
5. **Prefer standard tooling over custom scripts.** Docker, pre-commit framework, well-known CI patterns — not clever bespoke solutions.
6. **No hardcoded paths to CMU machines.** Successor won't be at CMU. Use config/env vars.
7. **No credentials in repos.** Use secret managers or environment variables.
8. **Document or automate every process that currently requires "ask someone."**

Full succession plan: `docs/migration/phase4-succession.md`

## Cross-Repo Engineering Rules (applies to ALL code work)

These rules are intended to keep the codebase survivable for future
contributors, especially when they browse code before docs.

1. **Types are the primary documentation.** Prefer named structs, enums,
   traits, and newtypes over relying on parameter names or comments to explain
   meaning.
2. **No primitive obsession at stable boundaries.** Do not introduce raw
   `String`, `&str`, `usize`, `i32`, or `bool` parameters/fields when the value
   has a domain meaning such as CHAT text, language code, speaker ID, file
   role, nonnegative count, bounded index, or state choice. Parse and validate
   primitives at the edge, then carry typed values internally.
3. **No tuple-packed domain seams.** Replace shapes like `(String, String)` or
   `Vec<(String, Result<String, String>)>` with named structs or newtypes as
   soon as the field meanings are stable.
4. **No panic-based control flow in long-lived code.** Do not add
   `unwrap()`, `expect()`, or equivalents in servers, workers, CLI
   orchestration, persistence, background tasks, or other code that owns real
   state. Return typed domain errors instead.
5. **Use real domain errors.** Prefer `thiserror`-based error types in Rust and
   specific exception/result types in Python over stringly errors.
6a. **`From<T>` must be infallible.** Never implement `From<&str>` or
   `From<String>` on a type whose construction can fail. Use `TryFrom`
   instead. Provide named factory methods for well-known constants (e.g.,
   `LanguageCode3::eng()`).
6b. **No silent defaults via `unwrap_or` / `unwrap_or_else`.** If a value can
   fail validation, propagate the error — never silently substitute a default.
   Silent fallbacks hide bugs. Use `unwrap_or` only when the fallback is
   explicitly documented and semantically correct.
6. **Avoid boolean blindness.** Do not encode multi-state behavior as one or
   more booleans when an enum or state type would make the invariant explicit.
7. **Organize code for browsing.** Keep modules small, name them by workflow or
   concept, and split catch-all files once they start hiding multiple
   responsibilities.
8. **Use methods when they clarify ownership and invariants.** Behavior that is
   intrinsic to a type, owns its state, or depends on its invariants should
   usually live in an `impl`. Use free functions for symmetric transforms,
   adapters, and glue code that does not naturally belong to one owner.
9. **Comments must explain the boundary.** New comments should say why a seam
   exists, who owns the state, and what invariants callers rely on. Avoid
   narration comments that merely restate the next line.
10. **Touched docs need timestamps.** Any documentation file changed in a patch
    should update its `Last modified` field with date and time.

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
├── OSX-CLAN/                         # CLAN C/C++ source, macOS (Leonid)
├── Windows-CLAN/                     # CLAN C/C++ source, Windows (Leonid, VS 2013, MFC, unsigned)
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
│ # Corpus data (24 repos — 12 unsplit + 4 banks split; migrating GitLab → GitHub)
└── data/
    ├── aphasia-data/                 #   aphasia (unsplit)
    ├── asd-data/                     #   ASD (unsplit)
    ├── biling-data/                  #   bilingualism (unsplit)
    ├── ca-candor-data/               #   CANDOR corpus (split from ca-data, 4.8 GB)
    ├── ca-data/                      #   conversation analysis remainder (split, 300 MB)
    ├── childes-eng-na-data/          #   CHILDES: Eng-NA, Eng-AAE
    ├── childes-eng-uk-data/          #   CHILDES: Eng-UK, Clinical-Eng, Clinical-Other
    ├── childes-romance-germanic-data/ #  CHILDES: French, Romance, Spanish, German, DutchAfrikaans, Scandinavian, Celtic
    ├── childes-other-data/           #   CHILDES: Biling, Chinese, EastAsian, Japanese, Slavic, Other, etc.
    ├── class-data/                   #   classroom (unsplit)
    ├── dementia-data/                #   dementia (unsplit)
    ├── fluency-data/                 #   fluency (unsplit)
    ├── homebank-public-data/         #   HomeBank: Public + Secure
    ├── homebank-cougar-data/         #   HomeBank: Password/Cougar
    ├── homebank-bergelson-data/      #   HomeBank: Password/Bergelson
    ├── homebank-password-data/       #   HomeBank: Password/ remainder
    ├── motor-data/                   #   motor (unsplit)
    ├── phon-eng-french-data/         #   phonology: Eng-NA, French
    ├── phon-other-data/              #   phonology: all other languages
    ├── psychosis-data/               #   psychosis (unsplit)
    ├── rhd-data/                     #   RHD (unsplit)
    ├── samtale-data/                 #   samtale (unsplit)
    ├── slabank-data/                 #   SLA (unsplit)
    └── tbi-data/                     #   TBI (unsplit)
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
# Post-split: phon-data is now phon-eng-french-data and phon-other-data
diff <(cargo run --release -p talkbank-cli -- validate ../data/phon-eng-french-data/ ../data/phon-other-data/ --force 2>&1 \
       | grep '✗' | sed 's/✗ Errors found in //' | sort) \
     ../known-issues/phon-data-validation-baseline.txt
```

See `known-issues/README.md` for creating and updating baselines.

## Analysis Scripts

```bash
python3 scripts/analysis/scan_phon_mismatches.py data/phon-eng-french-data data/phon-other-data  # Phon XML↔IPA mismatches
python3 scripts/analysis/count_tier_coverage.py data/phon-eng-french-data data/phon-other-data    # Tier frequency report
scripts/analysis/diff_validator_runs.sh baseline data/phon-eng-french-data                        # Save baseline
scripts/analysis/diff_validator_runs.sh compare data/phon-eng-french-data                         # Compare after changes
```

## Large-Scale Corpus Testing

```bash
# From talkbank-tools/:
cargo run --release -p talkbank-cli -- validate ../data/ --force             # Validation only
cargo run --release -p talkbank-cli -- validate ../data/ --roundtrip --force  # + roundtrip
cargo run --release -p talkbank-cli -- validate ../data/ --skip-alignment     # Faster
```

## Batchalign3 Testing

**LOCAL ML RUNS ARE DANGEROUS.** Each Whisper/Stanza model instance consumes 2–5 GB RAM. The default auto-tuner will spawn multiple concurrent workers that exhaust GPU/system memory, causing **unrecoverable kernel OOM crashes**. This has happened multiple times.

**Rules:**
- **Process one file at a time** on a developer machine — always smoke-test one file first
- **For large corpus runs** (>5 files or >1 GB audio), use net (M3 Ultra, 256 GB RAM), not a developer machine
- **Always pass `--no-open-dashboard`** to prevent browser tab spam
- **`--workers N`** controls per-job file parallelism (wired to `ServerConfig.max_workers_per_job`). Use `--workers 1` for safe local runs.

```bash
# Correct: single file, local
batchalign3 --no-open-dashboard transcribe one_file.wav -o output/ --lang eng -v

# Correct: full corpus on net (256 GB RAM)
ssh macw@net
batchalign3 --no-open-dashboard transcribe /path/to/corpus/ -o output/ --lang auto -v
```

## Deployment

### Deploy Scripts

All deploy scripts live in `deploy/scripts/`. Read these scripts before deploying — they are the source of truth for how deployment works.

| Script | What | Target |
|--------|------|--------|
| `deploy/scripts/deploy_server.sh` | Deploy batchalign-next to server (net) | `bash deploy/scripts/deploy_server.sh` |
| `deploy/scripts/deploy_clients.sh` | Deploy batchalign-next to client machines | `bash deploy/scripts/deploy_clients.sh` |
| `deploy/scripts/deploy_batchalign3.sh` | Deploy batchalign3 (Rust) to any fleet machines | `bash deploy/scripts/deploy_batchalign3.sh` (all), `--server`, `--clients` |

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
| `talkbank.org` | `ssh macw@talkbank` | Ubuntu (CMU Cloud VM) | **Public-facing.** Nginx (all bank websites), GitHub Actions self-hosted runner, web repos at `/var/www/web/` |
| `net` | `ssh macw@net` | macOS (Mac Studio, M3 Ultra, 256 GB) | **CMU LAN only.** Media drives, batchalign server, ML processing. NOT publicly accessible. |
| `git.talkbank.org` | `ssh macw@git-talkbank` | Ubuntu (CMU Cloud VM) | **Being decommissioned.** Currently runs John's Node.js data browsing/ZIP app. GitLab deleted 2026-03-19. |
| `talkbank-02` (`media.talkbank.org`) | `ssh macw@talkbank-02` | Ubuntu (CMU Campus Cloud Plus VM) | Media server. |

`net` runs `batchalign-next` on port 8000 (Python 3.12). Being upgraded to `batchalign3` (Rust) on port 8001 (coexistence), then port 8000 (takeover).

**Full server reference:** `docs/net-talkbank-server.md` — services on net (batchalign, Tailscale, etc.).

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

**GitLab → GitHub:** 16 data repos on git.talkbank.org being split into 24 repos and migrated to GitHub. 24 empty private GitHub repos created (2026-03-18). See `docs/migration/implementation-plan.md` for the full plan, `docs/migration/dependency-map.md` for the complete dependency audit, and `docs/migration/gitlab-to-github-research.md` for research and decisions.

**Legacy docs:** 84 files from ~/Dropbox/documentation/ transferred to `docs/legacy/`. Being reviewed for accuracy — see `docs/legacy/README.md` for status of each.

## Documentation Conventions

**Date every document you touch.** All markdown docs across the workspace must include a date and time header. Use this frontmatter block at the top of every doc (after the `#` title):

```
**Status:** Current | Historical | Reference | Draft
**Last updated:** YYYY-MM-DD HH:MM
```

- **Current** — actively maintained, reflects reality
- **Historical** — preserved for context, no longer reflects current state
- **Reference** — stable reference material (signing guides, build notes, inventories)
- **Draft** — work in progress

**Rules:**
- When you create a new doc, add the date/time header.
- When you edit an existing doc, update `Last updated` to the current date and time. If the doc has no date header, add one.
- Use ISO 8601 dates with 24-hour time and timezone (`2026-03-12 14:30 EDT`), never prose dates (`February 15, 2026`). **Always run `date` to get the actual system time** — do not guess or use the conversation date.
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
Last Updated: 2026-03-18
