# check-media

**Status:** In progress
**Last updated:** 2026-03-24 13:24 EDT

**Planned repo:** `TalkBank/check-media` (GitHub, public). Currently lives at `scripts/check-media/` in talkbank-dev during development. Will be extracted to its own repo before Phase 3 deployment.

Rust replacement for `staging/scripts/chatmedia.py`. Validates CHAT transcript ↔ media file correspondence across all TalkBank data repos. Designed to run as a pre-push hook (Phase 3 of the GitLab → GitHub migration).

## Why

The Python `chatmedia.py` has two problems:
1. **Broken for split repos** — `check_media --bank childes` only checks one repo, missing the 3 other childes split repos.
2. **Slow** — SSHes to `net` and runs `find` across the entire media tree on every invocation.

This tool fixes both by using a **cached media manifest** and accepting arbitrary repo paths.

## Architecture

```
check-media refresh-manifest     # SSH to net once → ~/.cache/talkbank/media-manifest.json
check-media check <paths>        # Local-only: read CHAT files, compare against manifest
check-media fix add-unlinked     # Mutate CHAT files (with --dry-run)
check-media fix fix-corpus       # Mutate CHAT files (with --dry-run)
check-media show-manifest        # Inspect manifest contents
```

The manifest is a JSON file listing all media files on `net` by bank, with metadata (path, size, last-modified). It's refreshed explicitly — not on every check.

## Checks

| Kind | Description |
|------|-------------|
| `missing-media` | CHAT `@Media` references a file not in the manifest (and not marked `missing`) |
| `missing-chat` | Media file exists with no corresponding `.cha` file |
| `media-case` / `chat-case` | Filename exists but with different case |
| `filename-mismatch` | `@Media` name doesn't match the CHAT file's basename |
| `bullets-unlinked` / `bullets-notrans` | Has timing bullets but `@Media` is marked `unlinked` or `notrans` |
| `no-bullets-unlinked` | Media exists, no bullets, not marked `unlinked` |
| `no-bullets-missing` | No media, no bullets, not marked `missing` |
| `corpus-mismatch` | `@ID` corpus field doesn't match directory-derived corpus name |
| `missing-pic` | `%pic` references a file that doesn't exist on disk |
| `stale-manifest` | Manifest is older than the staleness threshold |

## Build

```bash
cd scripts/check-media
cargo build --release
```

Binary: `target/release/check-media`

## Usage

```bash
# One-time: build the media manifest from net
check-media refresh-manifest

# Check a single repo
check-media check ~/data/childes-eng-na-data/

# Check all repos
check-media check ~/data/*-data/

# Check with JSON output (for CI)
check-media check ~/data/aphasia-data/ --format json --fail-on-error

# Fix corpus names (dry run first)
check-media fix fix-corpus ~/data/aphasia-data/ --dry-run
check-media fix fix-corpus ~/data/aphasia-data/

# Add "unlinked" to media headers
check-media fix add-unlinked ~/data/childes-eng-na-data/ --dry-run
```

## Config

| Env var | Default | Description |
|---------|---------|-------------|
| `CHECK_MEDIA_MANIFEST` | `~/.cache/talkbank/media-manifest.json` | Path to cached manifest |
| `MEDIA_HOST` | `macw@net` | SSH target for manifest refresh |
| `MEDIA_ROOT` | `/Users/macw/media` | Media root on the server |

## Migration context

- **Replaces:** `staging/scripts/chatmedia.py` + `staging/check_media` wrapper
- **Phase 3 role:** Pre-push hook (Hook 5) in data repos
- **Phase 3 plan:** `docs/migration/phase3-plan.md`
- **Dependency map:** `docs/migration/dependency-map.md` (section 3.5)
- **Extraction:** Will move from `talkbank-dev/scripts/check-media/` to `TalkBank/check-media` on GitHub. Standalone binary, no workspace dependencies on talkbank-tools or batchalign3.

## Source layout

```
src/
├── main.rs              # Entry point, subcommand dispatch
├── cli.rs               # clap CLI definitions (Commands, CheckKind, FixMutation)
├── config.rs            # Bank-to-repo mapping, excluded paths/extensions
├── diagnostics.rs       # Diagnostic/DiagnosticKind/Severity types
├── extract.rs           # CHAT header parsing (@Media, @ID corpus, %pic, bullets)
├── output.rs            # Terminal and JSON output formatting
├── manifest/
│   ├── mod.rs           # Manifest data model
│   ├── refresh.rs       # SSH-based manifest generation
│   └── show.rs          # Manifest display/stats
├── check/
│   ├── mod.rs           # Check orchestration
│   ├── chat_to_media.rs # CHAT → media existence checks
│   ├── media_to_chat.rs # Media → CHAT existence checks
│   ├── bullet.rs        # Bullet consistency checks
│   ├── corpus_name.rs   # @ID corpus field validation
│   └── pic.rs           # %pic reference validation
└── fix/
    ├── mod.rs           # Fix subcommand dispatch
    ├── add_unlinked.rs  # Add "unlinked" to @Media headers
    └── fix_corpus.rs    # Rewrite @ID corpus fields
```
