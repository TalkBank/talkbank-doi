# GitHub Actions Usage Audit — TalkBank Organization

**Status:** Current
**Last updated:** 2026-03-13

## Summary

The TalkBank organization (GitHub Team plan) has exhausted its 3,000 free Actions minutes for the March 2026 billing cycle (resets April 1). This audit identifies the sources of consumption and recommends concrete changes to reduce usage.

## Current Billing Cycle (March 2026)

| Repo | Minutes | % of Budget | Notes |
|------|---------|-------------|-------|
| **batchalign3** | 2,359 | 78.6% | 4 CI runs + 4 cancelled Dependabot |
| **talkbank-tools** | 641 | 21.4% | 2 CI runs + 1 cancelled Dependabot |
| batchalign2 | 25 | 0.8% | 2 test runs (both failed) |
| webdev | 1 | <0.1% | 1 CI run |
| **Total** | **3,026** | **100.9%** | Over the 3,000 included limit |

### Historical Context (Previous Cycles)

| Month | Total Min | Top Consumer |
|-------|-----------|-------------|
| January 2026 | 220 | batchalign2 (213 min) |
| February 2026 | 445 | batchalign2 (434 min, 31 test runs) |
| March 2026 | 3,026 | batchalign3 (2,359 min) — **budget exhausted** |

The March spike coincides with batchalign3 and talkbank-tools workflows becoming active on GitHub-hosted runners.

## Root Cause Analysis

### 1. batchalign3 — 2,359 minutes (78.6%)

**Workflow:** `.github/workflows/test.yml` (CI)

**Structural problems:**

- **`on: push` with no branch filter** — every push to *any* branch triggers the full CI suite. Should be limited to `main` + PRs.
- **14 parallel jobs per CI run** — each run launches 14 separate GitHub-hosted runners:
  - 3× Build Wheel (Rust compile for Python 3.12, 3.13, 3.14)
  - 3× Test (one per Python version)
  - Type Check & CI Hygiene
  - Generated Artifacts
  - Dashboard Web Build
  - Dependency Audit
  - Rust Integration
  - Dashboard Smoke (Playwright)
  - Dashboard E2E (real Rust server + Python worker, on main only)
  - Coverage (manual dispatch only — not a problem)
- **Redundant Rust compilation** — 7 of the 14 jobs clone `talkbank-tools` and compile Rust independently. Even with `rust-cache`, cold-cache runs are extremely expensive (~30+ min per job for a full Rust compile).
- **3× Python matrix for wheel builds** — builds the PyO3 wheel for 3.12, 3.13, and 3.14 on every push. Python 3.14 is not yet released; testing it on every push is unnecessary.
- **Dashboard E2E on every main push** — runs a full Playwright + Rust server + Python worker integration test on every push to main, consuming ~12 min of runner time.
- **Dependabot updates** — automatically opened PRs trigger CI (cancelled after 24h timeout but still consume runner allocation time).

**Estimated per-run cost:** ~590 min (cold cache) / ~45 min (warm cache). With 4 runs in March, cold-cache costs dominate.

### 2. talkbank-tools — 641 minutes (21.4%)

**Workflow:** `.github/workflows/ci.yml` (CI)

**Structural problems:**

- **`on: push` with no branch filter** — same issue as batchalign3.
- **10 parallel jobs per CI run:**
  - Rust Check and Test (G0-G9 gates)
  - Reference Corpus Roundtrip (74 files, `--release` build)
  - Spec Tools (separate Cargo workspace)
  - Generated Artifacts (tree-sitter + spec regeneration)
  - CHAT Manual Anchor Check
  - Dependency Audit
  - Metrics (always runs)
  - Grammar (tree-sitter generate + test)
  - VS Code Extension (Node.js)
  - CI Gate Summary
- **Multiple independent Rust compilations** — at least 4 jobs compile the Rust workspace independently (check-and-test, roundtrip, spec-tools, generated-artifacts). The roundtrip job uses `--release` which is significantly more expensive.
- **No path filters** — a docs-only change triggers the full Rust build + test suite.

**Estimated per-run cost:** ~320 min (2 runs in March account for 641 min).

### 3. batchalign2 — 25 minutes this month (but 647 in Jan–Feb)

**Workflow:** `.github/workflows/test.yml` (Run Tests)

**Problems:**

- **`on: [push]` with no branch filter** — every push triggers tests.
- **3× Python matrix** (3.11, 3.12, 3.13) — installs the full ML dependency stack (`pip install -e ".[dev]"` including torch, whisper, etc.) for each version.
- **Tests are consistently failing** — every "Run Tests" run in the visible history (dating back to January) has `conclusion: failure`. The workflow is burning minutes on a test suite that never passes.
- **31 failed test runs in February** consumed 434 minutes — a significant portion of that month's budget.

### 4. Other Repos (Minimal Impact)

| Repo | Workflow | Trigger | Runner | Notes |
|------|----------|---------|--------|-------|
| webdev | CI (pyrefly check) | push to main + PRs | GitHub-hosted | ~1 min/run, well-configured |
| update-chat-types | Rust (check + test + build) | push to main + PRs | GitHub-hosted | ~2 min/run, well-configured |
| balite | Dependabot Updates | dynamic | GitHub-hosted | ~3 min/run |
| Bank repos (×17) | Deploy | push to main | **Self-hosted** (`talkbank`) | Zero billable minutes |
| batchalign2 | docs (mkdocs) | push to master | GitHub-hosted | ~1 min/run |
| batchalign2 | pages-build-deployment | dynamic | GitHub-hosted | ~1 min/run |

### Self-Hosted Runner

The organization has one self-hosted runner:

- **Name:** `talkbank`
- **OS:** Linux (X64)
- **Status:** Online
- **Labels:** `self-hosted`, `Linux`, `X64`, `talkbank`
- **Used by:** All 17 bank deploy workflows (childes-bank, dementia-bank, etc.)
- **Billable impact:** Zero — self-hosted runners don't consume Actions minutes

## Recommendations

### Immediate (save ~80% of monthly budget)

#### R1. Add branch filters to batchalign3 and talkbank-tools CI

Change both from `on: push` to:

```yaml
on:
  push:
    branches: [main]
  pull_request:
  workflow_dispatch:
```

This prevents accidental CI runs from topic branch pushes. **Expected savings: prevents runaway usage from development branches.**

#### R2. Reduce batchalign3 Python matrix on push

Build and test only Python 3.12 on push to main. Run the full 3.12/3.13/3.14 matrix only on PRs or weekly:

```yaml
strategy:
  matrix:
    python-version: ${{ github.event_name == 'pull_request' && fromJSON('["3.12","3.13","3.14"]') || fromJSON('["3.12"]') }}
```

**Expected savings: ~66% reduction in build/test job-minutes** (from 6 wheel build + test jobs to 2).

#### R3. Add path filters to talkbank-tools CI

Skip the full Rust build when only docs, book, or spec prose files change:

```yaml
on:
  push:
    branches: [main]
    paths-ignore:
      - 'book/**'
      - '*.md'
      - 'spec/docs/**'
      - '.github/**'
```

#### R4. Disable or fix batchalign2 test workflow

The test suite has been failing on every run since at least January 2026. Options:
1. **Disable the workflow** until the tests are fixed (add `if: false` or remove the `on: push` trigger)
2. Fix the underlying test failures
3. At minimum, add a branch filter (`branches: [master]`) to stop triggering on all pushes

### Medium-Term (additional savings)

#### R5. Consolidate Rust compilation in batchalign3 CI

Currently 7 of 14 jobs independently clone talkbank-tools and compile Rust. Consider:
- A shared "build Rust workspace" job that uploads compiled artifacts
- Dependent jobs download and reuse the compiled output
- This would eliminate ~5 redundant Rust compilations per CI run

#### R6. Split batchalign3 CI into fast/slow tiers

**Fast tier (every push to main):** cargo check, clippy, fmt, unit tests, type check
**Slow tier (PRs + weekly schedule):** full matrix build, Dashboard E2E, Playwright smoke, dependency audit

#### R7. Move Dashboard E2E to scheduled/manual-only

The Dashboard E2E job (`dashboard-e2e`) runs on every push to main and takes ~12 min. Move to:
```yaml
if: github.event_name == 'workflow_dispatch' || github.event_name == 'schedule'
```

#### R8. Review Dependabot update strategy

Dependabot update PRs for batchalign3 (cargo + npm) trigger CI runs. Consider:
- Grouping Dependabot updates to reduce PR frequency
- Using a lighter CI workflow for Dependabot PRs

### Long-Term Considerations

#### R9. Evaluate self-hosted runner for Rust CI

The existing `talkbank` self-hosted runner handles bank deploys (trivial `git pull` jobs). For heavy Rust compilation workloads, consider:
- A dedicated self-hosted macOS runner (e.g., on `net` which is an M3 Ultra with 256 GB)
- This would make batchalign3 and talkbank-tools CI completely free of billable minutes
- **Trade-off:** maintenance burden, security considerations for CI on shared infrastructure

#### R10. Caching strategy audit

Verify that `Swatinem/rust-cache` is effective across runs. Cold-cache Rust compilations are 5-10× more expensive than cached ones. Consider:
- Pinning cache keys to avoid unnecessary invalidation
- Using a scheduled "warm cache" job that runs nightly

## Projected Impact

| Scenario | Estimated Monthly Minutes |
|----------|--------------------------|
| Current (no changes) | ~3,000+ (budget exceeded) |
| After R1 + R2 + R3 + R4 | ~800–1,200 |
| After R1–R8 | ~400–600 |
| After R9 (self-hosted for Rust) | ~50–100 (only lightweight jobs remain) |

## Appendix: All TalkBank Repos with Active Workflows

| Repo | Workflows | Runner Type | Monthly Impact |
|------|-----------|-------------|----------------|
| batchalign3 | CI, Dashboard Desktop (manual), docs (manual), Release (manual), Dependabot | GitHub-hosted | **High** |
| talkbank-tools | CI, Release (manual), Dependabot | GitHub-hosted | **High** |
| batchalign2 | Run Tests, docs, pages-build-deployment, Copilot review, Dependabot | GitHub-hosted | Low–Medium |
| webdev | CI | GitHub-hosted | Negligible |
| update-chat-types | Rust | GitHub-hosted | Negligible |
| balite | Dependabot | GitHub-hosted | Negligible |
| homebrew-tap | Copilot coding agent | GitHub-hosted | Negligible |
| 17× bank repos | Deploy | **Self-hosted** | Zero |
| TBDBr | 2 workflows | Unknown | Not measured |
| generate-from-chat, sync-media, talkbank-manifests, batchalign, test, psyling-web | Various | Unknown | Not measured |
