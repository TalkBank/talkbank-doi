# Batchalign3 vs Batchalign2 CLI Delta

This document tracks user-facing CLI behavior that was added beyond the
`batchalign2` baseline in `~/batchalign2-master/`.

## Baseline Source

- `~/batchalign2-master/batchalign/cli/cli.py`
- `~/batchalign2-master/batchalign/cli/cache.py`

## Additions Beyond Batchalign2

### New top-level commands

- `serve`: start/stop/status for the Rust server runtime
- `jobs`: list and inspect remote jobs
- `fleet`: fleet-wide server health checks
- `logs`: view/export/clear run logs
- `openapi`: emit Rust server OpenAPI schema

### New dispatch capabilities

- `--server`: submit processing jobs to remote/local HTTP servers
- Fleet auto-resolution from fleet config when no explicit server is given
- Paths-mode job submission for local media references

### New input ergonomics

- Multi-path inputs per command (files and directories)
- `--file-list` input manifest support
- `--in-place` processing mode

### Extended server media selection

- `benchmark`/`opensmile`: `--bank` and `--subdir` for server-side media mappings

## Cache Delta (Requested Explicitly)

Relative to Batchalign2 cache behavior:

- Rust cache command supports:
  - `cache stats` and `cache --stats`
  - `cache clear` and `cache --clear`
  - `cache clear --all` / `cache --clear --all` (also clears permanent UTR entries)
- Media conversion cache stats/clearing are included alongside analysis cache stats.
- `cache warm` (prewarm from existing `%mor/%gra`) is intentionally not carried forward.

## BA2 Flag Compatibility (No-op in Rust CLI)

The Rust CLI accepts these BA2-era global flags for script compatibility:

- `--memlog`
- `--mem-guard`
- `--adaptive-workers` / `--no-adaptive-workers`
- `--pool` / `--no-pool`
- `--adaptive-safety-factor`
- `--adaptive-warmup`
- `--shared-models` / `--no-shared-models`

## Notes on Compatibility Intent

- The migration target is Batchalign2 command/flag continuity, plus deliberate new
  features above.
- Python `batchalign3` transitional quirks are not treated as compatibility requirements.
