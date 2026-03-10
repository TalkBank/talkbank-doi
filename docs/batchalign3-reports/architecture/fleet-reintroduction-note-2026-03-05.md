# Fleet Reintroduction Note

Date: 2026-03-05

## Decision

Dormant `fleet` support was removed from the public `batchalign3` release
surface and from the active Rust control-plane code.

Removal commit in `batchalign3`:

- `3a7de8b0` — `refactor(rust-next): remove dormant fleet support`

This was the correct release decision. The shipped product is now explicitly:

- single explicit remote server, or
- local daemon / sidecar daemon

and not a partial multi-server system.

## Why It Was Removed

The removed code was not release-quality. It included:

- a dead multi-server dispatch module,
- a `/fleet` server endpoint,
- `fleet.yaml` config loading,
- `jobs --fleet`,
- OpenAPI/schema surface for a feature that was not actually integrated end to end.

Keeping it in-tree as active product code created three problems:

- users could infer support from code/help/schema that did not really exist,
- docs had to keep explaining "disabled for now" behavior,
- the code carried maintenance cost without a supported workflow.

## What Must Be True Before Reintroducing Fleet

Reintroduction should happen only when all of the following are true.

### 1. Real media model

The system must have a clear answer for how media is accessed across servers.
This is the main architectural issue, not parsing a config file.

At minimum:

- either all fleet servers see the same media roots,
- or path remapping is explicit and tested,
- or media is uploaded/transferred as part of the job contract.

If this is vague, fleet should not return.

### 2. Real capability routing

Fleet dispatch must make routing decisions from actual server capabilities,
not just from a static list of URLs.

At minimum:

- health/capability probing is part of dispatch,
- unsupported commands are excluded deterministically,
- sidecar-only commands (`transcribe`, `transcribe_s`, `avqi`, possibly `benchmark`)
  have explicit rules.

### 3. Real failure semantics

Multi-server fan-out needs defined behavior for partial failure.

At minimum:

- per-server submission failure policy,
- retry policy,
- partial-result policy,
- cancellation policy,
- restart semantics,
- stable merged user-facing error reporting.

### 4. Real result merge model

The system must specify how distributed results are merged back into one user
view and one output tree.

At minimum:

- deterministic output placement,
- deterministic file-to-server assignment,
- no silent overwrite behavior,
- merged progress and status reporting.

### 5. Real operator surface

If fleet exists, the operator UX must be real too.

At minimum:

- meaningful `jobs` behavior,
- meaningful logs/diagnostics,
- dashboard behavior that matches CLI behavior,
- documentation that describes supported operations without caveats.

### 6. Real test coverage

Fleet should not be restored on the basis of code structure alone.

At minimum:

- integration tests with multiple live test servers,
- mixed-capability server tests,
- partial failure tests,
- result merge tests,
- media-resolution tests,
- dashboard/API contract tests if those surfaces are public.

## What Can Be Reused Later

There is no need to preserve the removed implementation outside git history.

What is already preserved:

- the old code in git history,
- older fleet context in [fleet-history.md](./fleet-history.md).

That is enough.

If fleet is revisited later, treat the old implementation as reference only,
not as something to restore wholesale.

The likely reusable ideas are:

- server capability weighting via `/health`,
- per-server job fan-out,
- shared config loading patterns,
- progress aggregation concepts.

The likely non-reusable parts are:

- old assumptions around `fleet.yaml` as sufficient product surface,
- exposing `/fleet` before fleet is operationally real,
- CLI fallback/warning behavior for a feature not actually shipped.

## Recommended Reintroduction Order

If fleet is ever resumed, do it in this order:

1. Write a short private requirements note for media, capability, failure, and merge behavior.
2. Build multi-server integration tests first.
3. Reintroduce internal fan-out behind tests only.
4. Add operator surfaces (`jobs`, logs, dashboard) only after core behavior is stable.
5. Add public docs and CLI/public API only once the feature is genuinely releasable.

## Non-Goal

Do not reintroduce:

- `fleet.yaml`,
- `/fleet`,
- `jobs --fleet`,
- multi-URL `--server`

just to "keep the door open".

Git history already keeps the door open. Public/product code should only carry
what is actually supported.
