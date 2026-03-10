# Pinned Batchalign Variant Locations (2026-02-25)

This records the exact locations used for 4-way retokenize comparisons.

## Executable Entry Points

- `batchalign` executable: `/Users/chen/.local/bin/batchalign`
  - symlink target: `/Users/chen/.local/share/uv/tools/batchalign/bin/batchalign`
  - version: `0.8.1-post.14`
  - package root: `/Users/chen/.local/share/uv/tools/batchalign/lib/python3.12/site-packages/batchalign`

- `batchalign-next` executable: `/Users/chen/.local/bin/batchalign-next`
  - symlink target: `/Users/chen/.local/share/uv/tools/batchalign-next/bin/batchalign-next`
  - version: `0.8.1-post.12`
  - package root: `/Users/chen/.local/share/uv/tools/batchalign-next/lib/python3.13/site-packages/batchalign`

- `batchalign3` (repo-local via `uv run`):
  - Python launcher: `/Users/chen/talkbank/batchalign3/.venv/bin/batchalign3`
  - Rust CLI fallback binary (release): `/Users/chen/talkbank/batchalign3/rust-next/target/release/batchalign3`
  - Rust CLI fallback binary (debug): `/Users/chen/talkbank/batchalign3/rust-next/target/debug/batchalign3`

## Frozen Wrapper Commands

- January legacy wrapper: `/Users/chen/bin/batchalign-jan84ad500`
  - Runs frozen source snapshot with explicit `PYTHONPATH` isolation
  - Defaults to Python: `/Users/chen/.local/share/uv/tools/batchalign/bin/python`

- Current-master legacy wrapper: `/Users/chen/bin/batchalign-master-fd816d4`
  - Same isolation behavior, pinned to frozen current-master snapshot

## Frozen Source Trees

- January baseline (`batchalign` old):
  - path: `/Users/chen/bin/batchalign-pins/repos/batchalign2-jan84ad500`
  - convenience symlink: `/Users/chen/bin/batchalign2-jan84ad500`
  - commit: `84ad500b09e52a82aca982c41a8ccd46b01f4f2c`
  - date: `2026-01-09 22:13:32 -0800`

- Current-master baseline (pinned as detached worktree):
  - path: `/Users/chen/bin/batchalign-pins/repos/batchalign2-master-fd816d4`
  - convenience symlink: `/Users/chen/bin/batchalign2-master-fd816d4`
  - commit: `fd816d446f0b7da1e7d3aee88065e63c09e870b4`
  - date: `2026-02-11 15:48:01 -0800`

## Frozen Artifacts

- `batchalign-next` wheel snapshot copied from Net:
  - `/Users/chen/batchalign-next-from-net/batchalign_next-0.8.1.post12-py3-none-any.whl`
  - `/Users/chen/batchalign-next-from-net/batchalign_core-0.1.0-cp313-cp313-macosx_11_0_arm64.whl`

## Harness

Use:

```bash
/Users/chen/talkbank/batchalign3/scripts/compare_4way_retokenize.sh --input /path/to/file.cha
```

Defaults are already pinned to:

- old repo: `/Users/chen/bin/batchalign-pins/repos/batchalign2-jan84ad500`
- old python: `/Users/chen/.local/share/uv/tools/batchalign/bin/python`
- current command: `batchalign`
- next command: `batchalign-next`
