# Public vs Private Docs Triage (2026-03-05)

## Scope

Goal: identify material in `batchalign3/book/src` that should remain in the
public docs versus material that should move to `talkbank-private` because it is
primarily historical, investigative, or internal.

Public docs should serve:

1. current `batchalign3` users
2. contributors working on the active codebase
3. users migrating from `batchalign2` to `batchalign3`

If a page does not materially help one of those audiences make a present-tense
decision, it is a move candidate.

Migration-specific rule:

- keep durable behavior-change history in the public migration book when it
  explains what changed between the `batchalign2` baseline on 2026-01-09
  (`84ad500b09e52a82aca982c41a8ccd46b01f4f2c`) and current `batchalign3`
  behavior;
- do not document transient unreleased interim states in the public book;
- the only post-baseline `batchalign2` comparison point worth naming publicly is
  the later released master-branch point,
  `e8f8bfada6170aa0558a638e5b73bf2c3675fe6d` (2026-02-09), when it represents
  the last actual `batchalign2` user-visible release surface before
  `batchalign3` development diverged.

## Triage Labels

- `keep-public`: current or migration-relevant; stay in `book/`
- `compress-public`: keep a short public note, move most detail private
- `move-private`: remove from public nav/book and move full page to `talkbank-private`

## Recommended Move Candidates

### Move Now

These pages are primarily historical/internal unless their durable
before-vs-after substance is first extracted into `book/src/migration/`.

| Page | Recommendation | Reason |
|---|---|---|
| `book/src/reference/python-gra-generation-analysis.md` | `move-private` after extraction | Keep only migration-relevant `%gra` correctness deltas public; move legacy Python bug archaeology private |
| `book/src/reference/multi-file-optimization.md` | `move-private` after extraction | Keep only durable throughput/runtime-model deltas public; move transient regression analysis private |
| `book/src/reference/utr-alignment.md` | `move-private` | Historical Python UTR analysis is no longer needed in public nav once migration-relevant alignment deltas are preserved in the migration book |
| `book/src/reference/master-tier-output.md` | `move-private` | Older Python tier-output reference is archaeology, not current user or migrator guidance |
| `book/src/developer/string-check-optimizations.md` | `move-private` | Rewrite-era optimization work log, not public contributor truth |
| `book/src/decisions/google-translate-migration.md` | `move-private` | Deferred porting discussion, not current product or migration guidance |
| `book/src/developer/python-314t-migration.md` | `move-private` | Paused 3.14t experiment history is not needed in public navigation; keep only a link-stable stub if desired |
| `book/src/developer/pre-commit.md` | `move-private` | Historical proposal, not current enforced workflow |

### Move After Public Compression

These contain some useful public context, but the current pages are too heavy
with internal analysis. Keep a short public summary and move the full analysis.

| Page | Recommendation | Public replacement |
|---|---|---|
| `book/src/architecture/caching.md` | `compress-public` | Current cache architecture + brief note that historical assessment moved private |
| `book/src/architecture/validation.md` | `compress-public` | Current warning/error policy + known open gaps |
| `book/src/reference/textgrid.md` | `compress-public` | Current export behavior + narrow backlog note |
| `book/src/developer/python-versioning.md` | `compress-public` | Current supported Python policy only; move experiment benchmark detail private |

### Keep Public

These are historical in part, but still useful to current contributors or
`batchalign2` migrators if kept concise.

| Page | Recommendation | Reason |
|---|---|---|
| `book/src/decisions/rust-migration-proposal.md` | `move-private` | Raw proposal history is not needed in the public book once implemented architecture and migration pages exist |
| `book/src/migration/index.md` | `keep-public` | Explicit migration crosswalk |
| `book/src/migration/user-migration.md` | `keep-public` | Directly useful to migrators |
| `book/src/migration/developer-migration.md` | `keep-public` | Directly useful to contributors migrating mental models |
| `book/src/migration/algorithms-and-language.md` | `keep-public` | Canonical home for durable correctness/algorithm/data-structure deltas |
| `book/src/reference/proportional-fa-estimation.md` | `keep-public` | Now implemented; useful as design rationale and behavior explanation |

## Suggested Private Destinations

These are proposed paths in `talkbank-private` for moved material.

| Public page | Proposed private path |
|---|---|
| `book/src/reference/python-gra-generation-analysis.md` | `docs/batchalign3-reports/reference/python-gra-generation-analysis.md` |
| `book/src/reference/multi-file-optimization.md` | `docs/batchalign3-reports/reference/multi-file-optimization-regression-2026-02.md` |
| `book/src/developer/python-314t-migration.md` | `docs/batchalign3-reports/developer/python-314t-migration-history.md` |
| `book/src/developer/pre-commit.md` | `docs/batchalign3-reports/developer/pre-commit-proposal.md` |
| `book/src/architecture/caching.md` (long-form assessment sections) | `docs/batchalign3-reports/architecture/cache-assessment-2026-02-16.md` |
| `book/src/architecture/validation.md` (long historical roadmap sections) | `docs/batchalign3-reports/architecture/validation-roadmap-history.md` |
| `book/src/reference/textgrid.md` (backlog/idea sections) | `docs/batchalign3-reports/reference/textgrid-followups.md` |
| `book/src/developer/python-versioning.md` (3.14t experiment detail) | `docs/batchalign3-reports/developer/python-versioning-experiments.md` |

## Replacement Stub Guidance

When a page moves private, keep one of these public outcomes:

### Option A: Remove from nav entirely

Best for pages with little ongoing value after migration-relevant material has
been extracted elsewhere, such as:

- `python-gra-generation-analysis.md`
- `multi-file-optimization.md`
- `pre-commit.md`

### Option B: Replace with short public stub

Recommended when readers may still land on the old path from links or memory.

Suggested stub structure:

```md
# <Title>

**Status:** Historical material moved out of the public book
**Last updated:** 2026-03-05

This page previously contained a detailed internal analysis. Durable migration
relevance has been preserved in the migration book; the remaining detail is not
needed for current users or batchalign2 migrators.

Current public guidance:

- For current behavior, see <current public page>.
- For contributor-facing implementation truth, see <current public page>.

Detailed historical notes now live in internal/private documentation.
```

## Execution Order

1. Remove from `SUMMARY.md`:
   - `reference/python-gra-generation-analysis.md`
   - `reference/multi-file-optimization.md`
   - `developer/pre-commit.md`
   - `reference/utr-alignment.md`
   - `reference/master-tier-output.md`
   - `developer/string-check-optimizations.md`
   - `decisions/google-translate-migration.md`
2. Extract durable migration deltas from:
   - `reference/python-gra-generation-analysis.md`
   - `reference/multi-file-optimization.md`
   into `book/src/migration/`.
3. Remove `developer/python-314t-migration.md` from nav and keep only a short
   paused-status stub if link stability is needed.
   - status: executed in public book
4. Split `architecture/caching.md` into:
   - short current public architecture page
   - private historical assessment
5. Split `architecture/validation.md` into:
   - short current policy page
   - private historical roadmap notes
6. Trim `reference/textgrid.md` backlog sections or move them private.

## Concrete Recommendation

If the goal is to reduce public-doc drift quickly with minimal risk:

1. keep migration-relevant durable deltas public in `book/src/migration/`
2. `move-private` / de-nav only the non-migration remainder of:
   - `reference/python-gra-generation-analysis.md`
   - `reference/multi-file-optimization.md`
   - `developer/pre-commit.md`
3. `compress-public` next:
   - `developer/python-314t-migration.md`
   - `architecture/caching.md`
   - `architecture/validation.md`
4. leave migration pages in place and treat them as the authoritative public
   home for baseline-to-current deltas
5. continue treating only current behavior docs as authoritative outside the
   migration crosswalk

## Execution Update

On 2026-03-05, the first wave of `move-private` pages was physically relocated
out of `batchalign3` into `/Users/chen/talkbank/talkbank-private/docs/batchalign3-reports/`
and removed from the public repo, rather than left behind as de-navved stubs.

Moved out of `batchalign3`:

- `book/src/reference/utr-alignment.md` ->
  `talkbank-private/docs/batchalign3-reports/reference/utr-alignment-history.md`
- `book/src/reference/master-tier-output.md` ->
  `talkbank-private/docs/batchalign3-reports/reference/master-tier-output-history.md`
- `book/src/reference/python-gra-generation-analysis.md` ->
  `talkbank-private/docs/batchalign3-reports/reference/python-gra-generation-analysis.md`
- `book/src/reference/multi-file-optimization.md` ->
  `talkbank-private/docs/batchalign3-reports/reference/multi-file-optimization-regression-2026-02.md`
- `book/src/developer/pre-commit.md` ->
  `talkbank-private/docs/batchalign3-reports/developer/pre-commit-proposal.md`
- `book/src/developer/python-314t-migration.md` ->
  `talkbank-private/docs/batchalign3-reports/developer/python-314t-migration-history.md`
- `book/src/developer/string-check-optimizations.md` ->
  `talkbank-private/docs/batchalign3-reports/developer/string-check-optimizations.md`
- `book/src/decisions/google-translate-migration.md` ->
  `talkbank-private/docs/batchalign3-reports/decisions/google-translate-migration.md`
- `book/src/decisions/rust-migration-proposal.md` ->
  `talkbank-private/docs/batchalign3-reports/decisions/rust-migration-proposal.md`
- `book/src/migration/batchalignhk-to-plugins.md` ->
  `talkbank-private/docs/batchalign3-reports/migration/batchalignhk-to-plugins.md`

This is the correct model going forward: no private or purely internal history
should remain in `batchalign3` merely as a hidden page.

Additional execution on 2026-03-05:

- moved `book/src/developer/documentation-readiness.md` ->
  `talkbank-private/docs/batchalign3-reports/developer/documentation-readiness.md`
- moved `book/src/developer/documentation-scope.md` ->
  `talkbank-private/docs/batchalign3-reports/developer/documentation-scope.md`
- archived resolved incident history from `book/src/architecture/server-known-issues.md` ->
  `talkbank-private/docs/batchalign3-reports/architecture/server-known-issues-resolved-incidents.md`
- archived historical fleet notes from `book/src/architecture/server-architecture.md` ->
  `talkbank-private/docs/batchalign3-reports/architecture/fleet-history.md`
- archived historical sections from:
  - `book/src/reference/proportional-fa-estimation.md` ->
    `talkbank-private/docs/batchalign3-reports/reference/proportional-fa-estimation-history.md`
  - `book/src/reference/wor-tier.md` ->
    `talkbank-private/docs/batchalign3-reports/reference/wor-tier-python-bug-history.md`
  - `book/src/reference/morphosyntax.md` ->
    `talkbank-private/docs/batchalign3-reports/reference/morphosyntax-history.md`

This pass also removed internal documentation-governance pages from
`SUMMARY.md`, so they no longer appear in the public book at all.

Further execution on 2026-03-05:

- moved `batchalign3/analysis/` ->
  `talkbank-private/docs/batchalign3-repo-analysis/`
- moved `batchalign3/docs/archive/` ->
  `talkbank-private/docs/batchalign3-public-archive/`
- moved `book/src/developer/deployment-separation.md` ->
  `talkbank-private/ops/batchalign3/deployment/deployment-separation.md`
- created private deployment/agent structure:
  - `talkbank-private/ops/batchalign3/README.md`
  - `talkbank-private/ops/batchalign3/deployment/README.md`
  - `talkbank-private/ops/batchalign3/agent-guides/CLAUDE.md`

Public `CLAUDE.md` files were retained but revised to remove private deployment
paths and stale 3.14t deployment assumptions. This is the intended rule:
keep repository-appropriate technical guidance public; move environment-specific
operations guidance into `talkbank-private/ops/`.

Additional execution on 2026-03-05:

- moved `book/src/developer/launchd-template.plist` ->
  `talkbank-private/ops/batchalign3/deployment/launchd-template.plist`
- moved `book/src/developer/setup-launchd.sh` ->
  `talkbank-private/ops/batchalign3/deployment/setup-launchd.sh`

Public deployment docs were then sanitized to use generic service-account and
placeholder paths rather than private machine/user paths.

Additional execution on 2026-03-05:

- moved `BATCHALIGN2_DELTA.md` ->
  `talkbank-private/docs/batchalign3-reports/root/BATCHALIGN2_DELTA.md`
- moved `examples/fleet.yaml` ->
  `talkbank-private/docs/batchalign3-public-archive/examples/fleet.yaml`

Public top-level docs were then updated to point readers to the migration book
instead of a separate root delta file, and retained fleet wording was tightened
to explicit disabled/discovery-only status.

- executed move-private: `.claude/skills/`, `.coverage`, `batchalign_core/libbatchalign_core.dylib.dSYM/`, and `artifacts/` out of `batchalign3`; removed empty `.claude/` shell

- organized private destinations with new README/CLAUDE files under `talkbank-private/docs/batchalign3-reports/`, `talkbank-private/artifacts/batchalign3/`, and `talkbank-private/ops/batchalign3/agent-guides/skills/`; removed stale internal audit references from public `textgrid.md`
