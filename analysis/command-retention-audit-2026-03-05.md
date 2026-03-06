# Command Retention Audit (2026-03-05)

## Rule

Retention floor:
- keep every command that was public in released `batchalign2` as of `84ad500b09e52a82aca982c41a8ccd46b01f4f2c` (2026-01-09)
- only post-Jan-9 additions are eligible for removal on command-surface grounds

## Jan 9 compatibility floor

Public in BA2 on 2026-01-09:
- `align`
- `transcribe`
- `translate`
- `morphotag`
- `coref`
- `utseg`
- `benchmark`
- `opensmile`
- `avqi`
- `setup`
- `version`
- `models`

Verified against:
- `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/cli/cli.py`

## Post-Jan-9 command audit

| Command | Public by Jan 9 BA2? | Current usefulness | Recommendation | Evidence |
|---|---|---|---|---|
| `cache` | no | User-facing cache inspection and clearing for real runtime state | keep | `batchalign-cli/src/cache_cmd.rs`, `batchalign-bin/tests/cli.rs` |
| `bench` | no | Developer/perf tooling for repeated timing runs | keep, but contributor-facing | `batchalign-cli/src/bench_cmd.rs`, `batchalign-bin/tests/cli.rs` |
| `serve` | no | Essential server lifecycle control in BA3 runtime model | keep | `batchalign-cli/src/serve_cmd.rs`, `batchalign-cli/tests/commands.rs` |
| `jobs` | no | Essential async job inspection/operations for server-backed runs | keep | `batchalign-cli/src/jobs_cmd.rs`, `batchalign-server/src/routes/jobs/mod.rs`, `batchalign-cli/tests/commands.rs` |
| `logs` | no | Useful operational/support surface for run inspection | keep | `batchalign-cli/src/logs_cmd.rs`, `book/src/user-guide/cli-reference.md` |
| `openapi` | no | Contributor/integration tooling for HTTP API schema drift | keep, but contributor-facing | `batchalign-server/src/openapi.rs`, `batchalign-bin/tests/cli.rs` |
| `gui` | no | Untested first-release idea, not part of release-critical workflow | remove from first release | `batchalign-cli/src/gui_cmd.rs` |
| `fleet` | no | Removed: command only reported unavailable and added confusion | remove | removed from CLI on 2026-03-05; previous implementation was stub-only |

## Result

Recommended public command split after `fleet` removal:
- user-facing operations: `serve`, `jobs`, `logs`, `cache`, `version`
- contributor-facing operations: `openapi`, `bench`
- Jan 9 compatibility commands that must remain: `setup`, `models`, and all processing commands
- removed from first release: `gui`
