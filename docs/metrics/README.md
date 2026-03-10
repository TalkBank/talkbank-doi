# Regression Dashboard and Quality Metrics

This page explains how to view quality metrics and trendlines for TalkBank Utils.

---

## Per-Run Snapshot (GitHub Actions)

Every CI run produces a **Quality Metrics Snapshot** written to the run's
Step Summary (visible in the GitHub Actions UI under each run).  It includes:

| Metric | Source |
|--------|--------|
| Spec error files (total, parser layer, validation layer) | `find spec/errors -name '*.md'` |
| Spec construct files | `find spec/constructs -name '*.md'` |
| Reference corpus `.cha` files | `find rust/corpus/reference -name '*.cha'` |
| Parser-equivalence files tested | Same as reference corpus count |
| Markdown files (total) | All `.md` excluding `target/`, `node_modules/` |

The raw data is also uploaded as a `metrics` artifact (retained 90 days)
for each CI run.

---

## Local Snapshot

Run from the repo root:

```bash
bash scripts/metrics-snapshot.sh
```

Output is JSON on stdout.

---

## Trend View (requires `gh` CLI)

Fetch and display metrics from the last N completed CI runs on `main`:

```bash
# Last 10 runs
bash scripts/fetch-metrics-trend.sh

# Last 20 runs on main branch
bash scripts/fetch-metrics-trend.sh 20 main
```

Prerequisites: `gh` CLI authenticated to this repo, and `jq` installed.

Example output:

```
| Commit  | Date       | Result    | Error Specs | Construct Specs | Corpus Files |
|---------|------------|-----------|-------------|-----------------|--------------|
| fef020a | 2026-02-18 | ✅ success | 193         | 164             | 339          |
| 266ee91 | 2026-02-17 | ✅ success | 193         | 164             | 339          |
```

---

## Quality Gates vs Metrics

| Thing | Type | Where enforced |
|-------|------|----------------|
| Reference corpus 100% roundtrip | **Gate** (CI fails if red) | `reference-corpus-roundtrip` job |
| Parser equivalence 0 divergences | **Gate** (CI fails if red) | `rust-check-and-test` G7 step |
| Spec error file count | **Metric** (informational) | `metrics` job step summary |
| Construct spec count | **Metric** (informational) | `metrics` job step summary |

Metrics grow as spec coverage expands.  Gates must stay green at all times.

---

*Last Updated: 2026-02-18*
