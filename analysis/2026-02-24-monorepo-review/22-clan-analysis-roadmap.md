# 22. CLAN Analysis Roadmap (`talkbank-clan`)

## Findings

- Command modularity is good (`commands/*`, `framework/*`, `transforms/*`).
- Build currently passes locally, but CI is missing.
- Shared dependencies on `talkbank-chat` crates are path-based and should be version-governed.

## Recommendations

1. Add CI and release readiness pipeline for `talkbank-clan`.
2. Define command parity matrix vs legacy CLAN behavior.
3. Add golden test corpus for each implemented command with deterministic expected outputs.
4. Add performance baselines for high-cost commands (for example `vocd`).
5. Publish migration guide for users transitioning from legacy CLAN.

## Libraries/frameworks to leverage

- Benchmarking: `criterion`
- Snapshot/golden testing: continue `insta`, add corpus-scale harness

## CLAN checklist

- [ ] Add GitHub CI workflow (`cargo check`, `cargo test`, fmt, clippy)
- [ ] Build and maintain command parity coverage matrix
- [ ] Add benchmark suite for top commands
- [ ] Add docs with examples per command and expected output
- [ ] Plan versioned release process for downstream users
