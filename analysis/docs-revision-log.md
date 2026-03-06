# Docs Revision Log

Purpose:
- Track every documentation change made during the 2026-03 revision campaign.
- Record the implementation source used to verify each change.

## Entry Template

```
## YYYY-MM-DD HH:MM (local)
- Page: book/src/...
- Change type: revise | split | archive | nav
- Summary: one-line description
- Verified against:
  - code: ...
  - tests: ...
  - docs: ...
- Follow-ups: optional
```

## Entries

## 2026-03-05 13:XX (local)
- Page: `analysis/docs-revision-matrix-2026-03-05.md`
- Change type: `nav`
- Summary: Created initial triage and execution matrix for published docs.
- Verified against:
  - code: `rust-next/crates/batchalign-worker/src/lib.rs`, `batchalign/inference/fa.py`
  - docs: `book/src/SUMMARY.md`, `book/src/architecture/rust-server-migration.md`, `book/src/developer/documentation-scope.md`
- Follow-ups: Execute P0 backlog first.

## 2026-03-05 13:XX (local)
- Page: `book/src/decisions/rust-migration-proposal.md`
- Change type: `revise`
- Summary: Reframed as historical/superseded; added explicit pointers to current architecture docs.
- Verified against:
  - code: `rust-next/crates/batchalign-worker/src/lib.rs`
  - docs: `book/src/architecture/rust-server-migration.md`, `book/src/architecture/server-architecture.md`, `book/src/architecture/python-rust-interface.md`
- Follow-ups: Consider splitting long historical implementation options into archive if this page remains too long for live nav.

## 2026-03-05 13:XX (local)
- Page: `book/src/architecture/command-contracts.md`
- Change type: `revise`
- Summary: Updated status from proposed to partially implemented with concrete enforced command gates and source paths.
- Verified against:
  - code: `rust-next/crates/batchalign-server/src/{morphosyntax,utseg,translate,coref,fa}.rs`, `rust-next/crates/batchalign-server/src/runner/util.rs`
  - docs: `book/src/architecture/validation.md`
- Follow-ups: Validate non-CHAT command contract coverage and tighten wording where still aspirational.

## 2026-03-05 13:XX (local)
- Page: `book/src/SUMMARY.md`
- Change type: `nav`
- Summary: Renamed migration proposal entry to indicate historical status in navigation.
- Verified against:
  - docs: `book/src/decisions/rust-migration-proposal.md`
- Follow-ups: Reorder design decisions section after broader P0/P1 cleanup.

## 2026-03-05 13:XX (local)
- Page: `book/src/developer/pre-commit.md`
- Change type: `revise`
- Summary: Reframed page title/status as historical proposal to avoid presenting it as enforced policy.
- Verified against:
  - docs: `book/src/developer/documentation-scope.md`
- Follow-ups: Decide whether to keep in live nav or move to archive after policy decision.

## 2026-03-05 13:XX (local)
- Page: `book/src/SUMMARY.md`
- Change type: `nav`
- Summary: Labeled pre-commit page as proposal in navigation to reduce policy ambiguity.
- Verified against:
  - docs: `book/src/developer/pre-commit.md`
- Follow-ups: Consolidate all proposal/historical labels in one nav pass.

## 2026-03-05 13:XX (local)
- Page: `book/src/architecture/server-known-issues.md`
- Change type: `revise`
- Summary: Split page into explicit Open Issues vs Resolved Incidents and added status metadata.
- Verified against:
  - docs: `book/src/architecture/rust-server-migration.md`
- Follow-ups: Periodically prune resolved incidents into archive if page grows too long.

## 2026-03-05 13:XX (local)
- Page: `book/src/user-guide/plugins.md`
- Change type: `revise`
- Summary: Removed date-sensitive “not yet on PyPI” wording; replaced with environment-based install guidance.
- Verified against:
  - docs: `book/src/user-guide/installation.md`
- Follow-ups: Validate plugin install examples against current release packaging policy.

## 2026-03-05 13:XX (local)
- Page: `book/src/user-guide/rust-server.md`
- Change type: `revise`
- Summary: Added status metadata header for consistency with revision campaign conventions.
- Verified against:
  - docs: `book/src/architecture/rust-server-migration.md`
- Follow-ups: Run a full command-by-command parity check against `batchalign-cli` help output.

## 2026-03-05 13:XX (local)
- Page: `book/src/user-guide/cli-reference.md`
- Change type: `revise`
- Summary: Added status metadata and corrected key CLI behavior details (verbosity mapping, `--whisper-fa` spelling, benchmark cache flag, weighted fleet distribution wording).
- Verified against:
  - code: `rust-next/crates/batchalign-bin/src/main.rs`, `rust-next/crates/batchalign-cli/src/args/{mod,commands}.rs`, `rust-next/crates/batchalign-cli/src/dispatch/helpers.rs`
- Follow-ups: Perform full per-command option parity pass against generated clap help output.

## 2026-03-05 13:XX (local)
- Page: `book/src/user-guide/server-mode.md`
- Change type: `revise`
- Summary: Added status metadata header for consistency and traceability.
- Verified against:
  - docs: `book/src/user-guide/rust-server.md`, `book/src/architecture/server-architecture.md`
- Follow-ups: Consolidate overlaps between `server-mode.md` and `rust-server.md`.

## 2026-03-05 13:XX (local)
- Page: `book/src/user-guide/cli-reference.md`
- Change type: `revise`
- Summary: Ran clap help parity checks and corrected kebab-case flag names (`--num-speakers`, `--whisper-oai`) plus benchmark/cache wording.
- Verified against:
  - code: `rust-next/crates/batchalign-cli/src/args/commands.rs`
  - runtime: `cargo run -p batchalign-bin -- ... --help` for top-level, align, transcribe, morphotag, bench
- Follow-ups: Continue parity check for remaining command sections (`translate`, `coref`, `utseg`, `benchmark`, `opensmile`, `avqi`, utility commands).

## 2026-03-05 13:XX (local)
- Page: `book/src/user-guide/cli-reference.md`
- Change type: `revise`
- Summary: Completed parity updates for `translate`/`coref`/`utseg`/`benchmark` sections, corrected cache/log behavior wording, and removed stale language claims.
- Verified against:
  - runtime: `cargo run -p batchalign-bin -- translate|coref|utseg|benchmark --help`
  - code: `rust-next/crates/batchalign-cli/src/args/commands.rs`, `rust-next/crates/batchalign-cli/src/cache_cmd.rs`
- Follow-ups: Validate utility-command examples (`serve`, `jobs`, `cache`, `fleet`, `logs`, `setup`, `gui`) against current behavior text.

## 2026-03-05 13:XX (local)
- Page: `book/src/user-guide/cli-reference.md`
- Change type: `revise`
- Summary: Updated cache behavior (translate cached; coref not cached) and expanded `serve start` option table (`--python`, `--warmup-policy`, `--worker-idle-timeout-s`, `--test-echo`).
- Verified against:
  - runtime: `cargo run -p batchalign-bin -- serve start --help`, `cache clear --help`, `cache stats --help`
  - code: `rust-next/crates/batchalign-cli/src/cache_cmd.rs`
- Follow-ups: Add explicit `setup` option table and validate `gui` launch notes against current launcher code path.

## 2026-03-05 13:XX (local)
- Page: `book/src/user-guide/cli-reference.md`
- Change type: `revise`
- Summary: Added explicit `setup` options table and non-interactive example; completed utility-command help parity pass (`logs`, `setup`, `gui`, `version`).
- Verified against:
  - runtime: `cargo run -p batchalign-bin -- logs|setup|gui|version --help`
  - code: `rust-next/crates/batchalign-cli/src/{gui_cmd,setup_cmd}.rs`
- Follow-ups: Final pass for `serve`/`jobs`/`fleet` examples and narrative consistency.

## 2026-03-05 13:XX (local)
- Page: `book/src/architecture/chat-parsing.md`
- Change type: `revise`
- Summary: Added status metadata and replaced proposal-era roadmap link with current architecture links.
- Verified against:
  - docs: `book/src/architecture/{chat-divorce,rust-server-migration,python-rust-interface}.md`
- Follow-ups: P1 architecture synchronization continues in `overview.md` and `python-rust-interface.md`.

## 2026-03-05 13:XX (local)
- Page: `book/src/architecture/overview.md`, `book/src/architecture/python-rust-interface.md`
- Change type: `revise`
- Summary: Added status/last-verified metadata headers to anchor architecture pages to current-state documentation conventions.
- Verified against:
  - docs: `book/src/architecture/{chat-divorce,rust-server-migration}.md`
- Follow-ups: Continue P1 drift audit for `server-architecture.md` and `validation.md`.

## 2026-03-05 13:XX (local)
- Page: `book/src/architecture/server-architecture.md`
- Change type: `revise`
- Summary: Added status/last-verified metadata header to mark fleet architecture as current but opt-in.
- Verified against:
  - docs: `book/src/user-guide/server-mode.md`
- Follow-ups: Validate deployment examples and configuration defaults against `batchalign-types` config schema.

## 2026-03-05 13:XX (local)
- Page: `book/src/architecture/validation.md`
- Change type: `revise`
- Summary: Added status metadata and relabeled plan section as historical snapshot to reduce timeline ambiguity.
- Verified against:
  - docs: `book/src/architecture/error-handling.md`
- Follow-ups: Re-audit this page against current warning emission behavior in parser/recovery paths.

## 2026-03-05 13:XX (local)
- Page: `book/src/architecture/rust-server-migration.md`, `book/src/SUMMARY.md`
- Change type: `revise`, `nav`
- Summary: Reframed migration page as implemented reference and updated navigation label accordingly.
- Verified against:
  - docs: `book/src/architecture/server-architecture.md`
- Follow-ups: Consider eventual split into “current architecture” + “migration history” pages if size grows.

## 2026-03-05 14:XX (local)
- Page: `book/src/architecture/server-architecture.md`
- Change type: `revise`
- Summary: Rewrote page from fleet-forward target design to implementation-first server dispatch architecture (single-server + local daemon current; fleet fan-out explicitly disabled in this release).
- Verified against:
  - code: `rust-next/crates/batchalign-cli/src/{dispatch/mod,daemon,fleet_cmd}.rs`, `rust-next/crates/batchalign-server/src/routes/{mod,health,fleet}.rs`, `rust-next/crates/batchalign-types/src/config.rs`
  - docs: `book/src/user-guide/server-mode.md`, `book/src/user-guide/cli-reference.md`
- Follow-ups: Reconcile any remaining fleet-focused narratives in related architecture pages with this current-state baseline.

## 2026-03-05 14:XX (local)
- Page: `book/src/architecture/dispatch-system.md`, `book/src/user-guide/rust-server.md`, `book/src/user-guide/cli-reference.md`
- Change type: `revise`
- Summary: Removed/rewrote active multi-server fan-out claims; documented current behavior (single remote `--server` or local daemon, with fleet fan-out disabled in dispatch path).
- Verified against:
  - code: `rust-next/crates/batchalign-cli/src/{dispatch/mod,fleet_cmd,jobs_cmd}.rs`
  - docs: `book/src/architecture/server-architecture.md`
- Follow-ups: Run a broader grep pass over remaining architecture/reference docs for latent fan-out claims outside user-guide and dispatch architecture pages.

## 2026-03-05 14:XX (local)
- Page: `book/src/reference/filesystem-paths.md`, `book/src/reference/command-io.md`, `book/src/migration/index.md`, `book/src/migration/developer-migration.md`, `book/src/migration/user-migration.md`
- Change type: `revise`
- Summary: Aligned migration/reference wording to current dispatch reality by removing implied active fleet fan-out and clarifying current fleet surfaces (`/fleet`, `jobs --fleet`) vs disabled processing fan-out.
- Verified against:
  - code: `rust-next/crates/batchalign-cli/src/{dispatch/mod,fleet_cmd,jobs_cmd}.rs`, `rust-next/crates/batchalign-server/src/routes/fleet.rs`
  - docs: `book/src/architecture/{server-architecture,dispatch-system}.md`
- Follow-ups: Sweep remaining migration chapters for any stale runtime topology wording outside fleet topics.

## 2026-03-05 14:XX (local)
- Page: `book/src/developer/{rust-manual-crosswalk,rust-next,rust-workspace-map,python-versioning,python-314t-migration}.md`, `book/src/architecture/rust-server-migration.md`, `book/src/migration/index.md`
- Change type: `revise`
- Summary: Corrected stale `batchalign-cli` module paths (`args/mod.rs`, `dispatch/mod.rs`) and updated developer/migration narrative to reflect current dispatch behavior (single remote server + local daemon; fleet fan-out disabled).
- Verified against:
  - code: `rust-next/crates/batchalign-cli/src/{args/mod.rs,dispatch/mod.rs,fleet_cmd.rs}`, `rust-next/crates/batchalign-server/src/routes/health.rs`
  - docs: `book/src/architecture/server-architecture.md`, `book/src/architecture/dispatch-system.md`
- Follow-ups: Do a final contributor-doc pass for non-fleet drift (`benchmarks`, python-version dates, and old operational timelines).

## 2026-03-05 15:XX (local)
- Page: `book/src/developer/{python-versioning,python-314t-migration,building,rust-contributor-onboarding}.md`, `book/src/user-guide/{rust-server,server-mode}.md`, `book/src/architecture/rust-server-migration.md`, `book/src/reference/multi-file-optimization.md`, `book/src/SUMMARY.md`
- Change type: `revise`, `nav`
- Summary: Applied policy update that 3.14t targeting is paused (as of 2026-03-05), updated high-visibility deployment/runtime guidance to Python 3.12 baseline, and marked 3.14t/optimization pages as paused or historical context.
- Verified against:
  - code: `rust-next/crates/batchalign-cli/src/daemon.rs`, `rust-next/crates/batchalign-server/src/runner/mod.rs`
  - user direction: explicit product policy decision provided in-session (pause 3.14t targeting)
- Follow-ups: Sweep remaining reference pages with 3.14t performance language to ensure all are labeled as experimental/historical where applicable.

## 2026-03-05 15:XX (local)
- Page: `book/src/reference/{morphosyntax,multi-file-optimization}.md`, `book/src/user-guide/quick-start.md`
- Change type: `revise`
- Summary: Added explicit historical/experimental framing to remaining 3.14t references and removed a brittle fixed startup-time claim from quick-start.
- Verified against:
  - code: `rust-next/crates/batchalign-server/src/runner/mod.rs`, `rust-next/crates/batchalign-worker/src/pool/mod.rs`
  - docs: `book/src/developer/{python-versioning,python-314t-migration}.md`
- Follow-ups: Remaining 3.14t mentions are now confined to explicitly paused/experimental pages and historically framed reference text.

## 2026-03-05 15:XX (local)
- Page: `book/src/reference/{proportional-fa-estimation,textgrid,python-gra-generation-analysis}.md`, `book/src/architecture/{caching,validation}.md`
- Change type: `revise`
- Summary: Corrected implementation-state drift: proportional untimed FA estimation marked implemented, TextGrid export marked current (with low-priority follow-ups), legacy Python `%gra` analysis marked historical, caching follow-up phases labeled partially historical, and validation updated to reflect that structured suggestions already cross the Python boundary.
- Verified against:
  - code: `rust/src/parsed_chat/fa.rs`, `rust/src/fa_ops.rs`, `rust/src/pyfunctions.rs`, `rust-next/crates/batchalign-chat-ops/src/fa/grouping.rs`, `rust-next/crates/batchalign-server/src/runner/dispatch/infer.rs`, `batchalign/errors.py`
  - docs: `book/src/reference/multi-file-optimization.md`
- Follow-ups: `textgrid.md` still contains backlog/TODO sections that should be normalized into clearer current-vs-future structure in a later pass.

## 2026-03-05 15:XX (local)
- Page: `analysis/docs-public-private-triage-2026-03-05.md`
- Change type: `nav`
- Summary: Created explicit public-vs-private docs triage with move/compress/keep recommendations and stub strategy for material of only historical/internal interest.
- Verified against:
  - docs: `book/src/SUMMARY.md`, `book/src/developer/documentation-scope.md`
  - docs reviewed in pass: `reference/python-gra-generation-analysis.md`, `reference/multi-file-optimization.md`, `developer/python-314t-migration.md`, `architecture/caching.md`, `architecture/validation.md`, `reference/textgrid.md`
- Follow-ups: Execute the move list by de-navving clear move candidates first, then compressing mixed pages.

## 2026-03-05 16:XX (local)
- Page: `analysis/docs-public-private-triage-2026-03-05.md`, `analysis/docs-revision-matrix-2026-03-05.md`, `book/src/migration/{index,user-migration,developer-migration,algorithms-and-language}.md`
- Change type: `revise`
- Summary: Corrected migration-doc policy: durable BA2-to-BA3 changes remain public in the migration book, anchored to Jan 9 baseline `84ad500...` and optionally the released Feb 9 master-branch point `e8f8bfa...`; transient unreleased branch states are excluded.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/cli/{cli,dispatch}.py`, `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/morphosyntax/ud.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/cli/{cli,dispatch}.py`
  - docs: `book/src/reference/morphosyntax.md`, `book/src/architecture/rust-server-migration.md`
  - user direction: explicit requirement to center migration docs on durable correctness/performance/algorithm/data-structure deltas since 2026-01-09 and to treat the released Feb 9 BA2 master-branch point as the only post-baseline BA2 release anchor
- Follow-ups: Continue extracting migration-relevant substance from historical/stub reference pages before moving any remaining detail private.

## 2026-03-05 17:XX (local)
- Page: `analysis/code-first-migration-audit-2026-03-05.md`, `book/src/migration/{index,user-migration,developer-migration,algorithms-and-language}.md`
- Change type: `revise`
- Summary: Rebased the migration audit and public migration-book language onto the actual released BA2 source in `~/batchalign2-master`, replacing the incorrect internal Feb 20 `align`-branch comparison with the released Feb 9 master-branch point `e8f8bfa...`.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show -s --format='%H %cs %s' 84ad500b09e52a82aca982c41a8ccd46b01f4f2c`, `git -C /Users/chen/batchalign2-master show -s --format='%H %cs %s' e8f8bfada6170aa0558a638e5b73bf2c3675fe6d`, `git -C /Users/chen/batchalign2-master log --oneline --reverse 84ad500b09e52a82aca982c41a8ccd46b01f4f2c..e8f8bfada6170aa0558a638e5b73bf2c3675fe6d`
  - user direction: explicit correction that Feb 20 was only an internal testing boundary and invisible to BA2 users
- Follow-ups: Continue the code-first subsystem audit against Jan 9 BA2, Feb 9 released BA2 master, and current BA3 before making any stronger performance/correctness claims in the public book.

## 2026-03-05 17:XX (local)
- Page: `analysis/code-first-migration-audit-2026-03-05.md`, `book/src/migration/{user-migration,algorithms-and-language}.md`
- Change type: `revise`
- Summary: Added dedicated `align` migration analysis based on released BA2 code: Jan 9 -> Feb 9 improvements (cache, robustness, dispatch/runtime gains) and Feb 9 -> current BA3 improvements (typed FA payloads, Rust-owned orchestration, deterministic timing transfer, structured `%wor`/monotonicity handling).
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/fa/{whisper_fa,wave2vec_fa}.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/pipelines/fa/{whisper_fa,wave2vec_fa}.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/cli/cli.py`
  - code: `rust-next/crates/batchalign-chat-ops/src/fa/{mod,alignment,grouping,injection}.rs`, `rust-next/crates/batchalign-server/src/runner/dispatch/infer.rs`, `batchalign/tests/pipelines/fa/test_rust_fa.py`
- Follow-ups: tighten the exact UTR migration wording once the Jan 9 BA2 and current BA3 transfer logic is compared line-by-line.

## 2026-03-05 17:XX (local)
- Page: `analysis/code-first-migration-audit-2026-03-05.md`, `book/src/migration/{user-migration,algorithms-and-language}.md`
- Change type: `revise`
- Summary: Tightened the UTR and DP-removal narrative to match actual code: released BA2 still used `bulletize_doc(...)`-style rough UTR and Python-side DP-heavy FA mapping, while current BA3 uses ID-first and window-constrained monotonic UTR transfer and deterministic `rust-next` FA response handling without broad global remap claims.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/utr/{whisper_utr,rev_utr}.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/pipelines/utr/{whisper_utr,rev_utr}.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/utils/dp.py`
  - code: `rust/src/speaker_ops.rs`, `rust/src/parsed_chat/speakers.rs`, `batchalign/tests/golden/test_dp_golden.py`, `rust-next/crates/batchalign-chat-ops/src/fa/alignment.rs`, `rust-next/crates/batchalign-chat-ops/src/retokenize/mapping.rs`
- Follow-ups: continue replacing any remaining migration-page claims that still compress released BA2 and current BA3 into a single-step algorithm story.

## 2026-03-05 17:XX (local)
- Page: `analysis/code-first-migration-audit-2026-03-05.md`, `book/src/migration/{index,developer-migration,user-migration,algorithms-and-language}.md`
- Change type: `revise`
- Summary: Added an explicit code-backed migration theme across commands: the durable improvement is moving away from string hacking, array-position repair, and broad flattened-text DP recovery toward stable IDs, explicit indices/chunk maps, AST iteration, and narrower deterministic fallback policies.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/morphosyntax/ud.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/pipelines/morphosyntax/ud.py`
  - code: `rust/src/speaker_ops.rs`, `rust/src/tests/mod.rs`, `rust-next/crates/batchalign-chat-ops/src/{extract.rs,fa/mod.rs,fa/alignment.rs,retokenize/mapping.rs,morphosyntax/inject.rs,nlp/mapping.rs}`
- Follow-ups: tighten the remaining user-facing migration text around specific morphotag fixes that should be called out as corpus-visible corrections.

## 2026-03-05 17:XX (local)
- Page: `analysis/code-first-migration-audit-2026-03-05.md`, `book/src/migration/{user-migration,developer-migration}.md`
- Change type: `revise`
- Summary: Added command-family precision for `transcribe`, `translate`, `utseg`, and `coref`: current BA3 uses task-local Python inference but keeps CHAT lifecycle ownership, structural decisions, and injection/validation on the Rust side; coref is now clearly document-level and sparse.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/cli/cli.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/cli/cli.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/pipelines/asr/{whisper.py,utils.py}`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/pipelines/translate/__init__.py`
  - code: `rust-next/crates/batchalign-server/src/{transcribe.rs,coref.rs,runner/dispatch/mod.rs}`, `batchalign/inference/{asr.py,translate.py,utseg.py,coref.py}`, `rust-next/crates/batchalign-types/src/options.rs`
- Follow-ups: continue into subsystem-specific morphotag correctness deltas so the migration book names the highest-value corpus-visible fixes, not just the architectural pattern.

## 2026-03-05 17:XX (local)
- Page: `analysis/code-first-migration-audit-2026-03-05.md`, `book/src/migration/{user-migration,developer-migration,algorithms-and-language}.md`
- Change type: `revise`
- Summary: Added concrete morphotag correctness fixes to the migration story: `%gra` ROOT/head/chunk validation, per-component MWT `%gra`, explicit `@c`/`@s` handling, `xbxxx` restoration, reflexive `reflx`, and explicit retokenize-vs-preserve behavior.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/morphosyntax/ud.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/pipelines/morphosyntax/ud.py`
  - code: `rust-next/crates/batchalign-chat-ops/src/{nlp/mapping.rs,morphosyntax/inject.rs,nlp/features.rs}`, `rust-next/crates/batchalign-chat-ops/src/retokenize/tests.rs`, `rust/src/tests/mod.rs`
- Follow-ups: verify which of these user-visible fixes were already present by the released Feb 9 BA2 point versus only in current BA3, so the migration book can separate those stages even more precisely.

## 2026-03-05 17:XX (local)
- Page: `analysis/code-first-migration-audit-2026-03-05.md`, `book/src/migration/{user-migration,algorithms-and-language}.md`
- Change type: `revise`
- Summary: Tightened the morphotag fix split across Jan 9 BA2 / released Feb 9 BA2 / current BA3. Clarified that `reflx`, special-form handling, and `xbxxx` restoration already existed on the BA2 line, while the most clearly current-BA3-specific gains are explicit ROOT/head/chunk validation and chunk-based `%gra` mapping.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master diff 84ad500b09e52a82aca982c41a8ccd46b01f4f2c e8f8bfada6170aa0558a638e5b73bf2c3675fe6d -- batchalign/pipelines/morphosyntax/ud.py`
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/morphosyntax/ud.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/pipelines/morphosyntax/ud.py`
  - code: `rust-next/crates/batchalign-chat-ops/src/nlp/mapping.rs`, `rust-next/crates/batchalign-chat-ops/src/retokenize/tests.rs`, `rust/src/tests/mod.rs`
- Follow-ups: do one final migration-book de-duplication pass so the same BA2-vs-BA3 morphotag points are not repeated at slightly different granularity.

## 2026-03-05 15:XX (local)
- Page: `book/src/SUMMARY.md`, `book/src/developer/pre-commit.md`, `book/src/reference/{python-gra-generation-analysis,multi-file-optimization}.md`
- Change type: `nav`, `revise`
- Summary: Executed first-wave public/private triage: removed three historical/internal pages from public navigation and replaced their in-repo content with short public stubs directing readers to current public docs and private/internal history.
- Verified against:
  - docs: `analysis/docs-public-private-triage-2026-03-05.md`, `book/src/developer/documentation-scope.md`
  - docs: `book/src/reference/{gra-format,morphosyntax}.md`, `book/src/user-guide/{rust-server,server-mode}.md`
- Follow-ups: Compress the next mixed pages (`developer/python-314t-migration.md`, `architecture/caching.md`, `architecture/validation.md`, `reference/textgrid.md`) and consider moving their long-form historical detail private.

## 2026-03-05 17:XX (local)
- Page: `analysis/code-first-migration-audit-2026-03-05.md`, `book/src/migration/{user-migration,developer-migration,algorithms-and-language}.md`
- Change type: `revise`
- Summary: Extended the code-first migration story command-by-command for `transcribe`, `translate`, `utseg`, and `coref`; corrected an overclaim by documenting that BA2 `coref` was already document-level and that BA3’s real change is the typed payload / `%xcoref` / Rust-owned parse-inject-validate boundary.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/asr/utils.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/pipelines/asr/utils.py`, `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/translate/{gtrans,seamless}.py`, `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/utterance/ud_utterance.py`, `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/morphosyntax/coref.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/pipelines/morphosyntax/coref.py`
  - code: `batchalign/inference/{asr,translate,utseg,coref}.py`, `rust-next/crates/batchalign-server/src/{transcribe,translate,utseg,coref}.rs`, `rust-next/crates/batchalign-chat-ops/src/{asr_postprocess/mod.rs,build_chat.rs,translate.rs,utseg.rs,utseg_compute.rs,coref.rs}`
- Follow-ups: continue this exact Jan 9 / Feb 9 / current treatment for the remaining command families (`benchmark`, `opensmile`, `avqi`) if they belong in the migration book.

## 2026-03-05 17:XX (local)
- Page: `analysis/code-first-migration-audit-2026-03-05.md`, `book/src/migration/{user-migration,developer-migration}.md`
- Change type: `revise`
- Summary: Extended the same three-stage migration audit to `benchmark`, `opensmile`, and `avqi`. Clarified that Jan 9 -> Feb 9 was mostly runtime/lazy-load improvement for these commands, while current BA3 adds typed command options, explicit process/infer dispatch, and for `benchmark` a Rust-backed WER scoring path.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/cli/cli.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/cli/{cli,bench}.py`, `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/pipelines/{opensmile,avqi}/engine.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/pipelines/{opensmile,avqi}/engine.py`
  - code: `batchalign/inference/{benchmark,opensmile,avqi}.py`, `batchalign/worker/{_infer.py,_handlers.py}`, `rust-next/crates/batchalign-server/src/runner/dispatch/process.rs`, `rust-next/crates/batchalign-types/src/{options,worker}.rs`
- Follow-ups: decide whether `benchmark`, `opensmile`, and `avqi` deserve their own migration subsection in the public book or whether the current summary table is enough.

## 2026-03-05 18:XX (local)
- Page: `book/src/{migration/index.md,migration/batchalignhk-to-plugins.md,user-guide/plugins.md,developer/plugins.md,architecture/python-rust-interface.md,SUMMARY.md}`, `analysis/code-first-migration-audit-2026-03-05.md`, `book/src/migration/user-migration.md`
- Change type: `revise`, `nav`
- Summary: Removed HK-specific guidance from the main batchalign3 book, replacing it with generic plugin documentation and an externalized historical stub. Also corrected the command-surface narrative so released Feb 9 BA2 is no longer over-credited with public `serve/jobs/fleet/logs` commands, and added an explicit utility-command migration table covering `setup`, `version`, `models`, `cache`, `bench`, `serve`, `jobs`, `logs`, `openapi`, `fleet`, and `gui`.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/cli/cli.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/cli/{cli,cache,bench}.py`, `git -C /Users/chen/batchalign2-master ls-tree -r --name-only e8f8bfada6170aa0558a638e5b73bf2c3675fe6d`
  - code: `rust-next/crates/batchalign-cli/src/args/{mod.rs,commands.rs}`, `rust-next/crates/batchalign-bin/tests/cli.rs`
  - user direction: HK material belongs in the external HK plugin repo, not the main batchalign3 book
- Follow-ups: continue the same command-by-command audit for any remaining surfaces that still appear only in command inventory or CLI help but not yet in migration prose.

## 2026-03-05 19:XX (local)
- Page: `rust-next/crates/batchalign-cli/src/{args/mod.rs,args/commands.rs,lib.rs}`, `rust-next/crates/batchalign-bin/src/main.rs`, `rust-next/crates/batchalign-bin/tests/cli.rs`, `rust-next/crates/batchalign-cli/tests/commands.rs`, `book/src/{migration/user-migration.md,architecture/server-architecture.md}`, `analysis/command-retention-audit-2026-03-05.md`
- Change type: `revise`, `remove`
- Summary: Removed the dead `batchalign3 fleet` CLI subcommand and its tests/docs after confirming it only emitted an unavailable message. Added a command-retention audit anchored to the Jan 9 BA2 public CLI so future command pruning respects the compatibility floor while treating only post-Jan-9 additions as removable.
- Verified against:
  - code: `rust-next/crates/batchalign-cli/src/fleet_cmd.rs` (pre-removal stub behavior), `rust-next/crates/batchalign-cli/src/dispatch/mod.rs`, `rust-next/crates/batchalign-bin/src/main.rs`, `rust-next/crates/batchalign-cli/src/args/mod.rs`
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/cli/cli.py`
  - tests: `cargo test -p batchalign-cli -p batchalign-bin --tests --no-run`
- Follow-ups: decide whether `gui` remains a first-class public command and whether `openapi`/`bench` should move to contributor-only docs rather than user-facing help.

## 2026-03-05 19:XX (local)
- Page: `rust-next/crates/batchalign-cli/src/{args/mod.rs,lib.rs}`, `rust-next/crates/batchalign-bin/src/main.rs`, `rust-next/crates/batchalign-bin/tests/cli.rs`, `book/src/{migration/user-migration.md,user-guide/cli-reference.md,decisions/tauri-react-dashboard-adoption.md}`, `analysis/command-retention-audit-2026-03-05.md`
- Change type: `revise`, `remove`
- Summary: Removed the untested `batchalign3 gui` command from the first-release CLI surface and scrubbed user-facing docs/migration tables accordingly. Reclassified `gui` in the command-retention audit from “under review” to “remove from first release”.
- Verified against:
  - code: `rust-next/crates/batchalign-cli/src/gui_cmd.rs` (pre-removal implementation), `rust-next/crates/batchalign-bin/src/main.rs`, `rust-next/crates/batchalign-cli/src/args/mod.rs`
  - tests: `cargo test -p batchalign-cli -p batchalign-bin --tests --no-run`
- Follow-ups: if the dashboard becomes release-critical later, reintroduce it only after explicit install path, packaging, and end-to-end tests exist.

## 2026-03-05 19:XX (local)
- Page: `book/src/{developer/tauri-react-dashboard.md,decisions/tauri-react-dashboard-adoption.md,user-guide/cli-reference.md}`
- Change type: `revise`
- Summary: Reframed dashboard/Tauri documentation to match current release policy: code remains in-tree and the architecture decision stands, but dashboard/Tauri launch is developer/future-release work and not part of the first public `batchalign3` CLI surface. Also clarified that `bench` and `openapi` are contributor-facing tooling.
- Verified against:
  - code: `apps/dashboard-desktop/src-tauri/Cargo.toml`, `apps/dashboard-desktop/package.json`, `rust-next/crates/batchalign-server/src/routes/dashboard.rs`
  - code: `scripts/{build_react_dashboard.sh,generate_dashboard_api_types.sh,check_dashboard_api_drift.sh}`
- Follow-ups: if dashboard release work resumes, add explicit packaging/install docs and end-to-end launch validation before reintroducing any public launcher.

## 2026-03-05 19:XX (local)
- Page: `book/src/user-guide/rust-server.md`
- Change type: `revise`
- Summary: Tightened the server guide so dashboard access is described as optional asset-backed server behavior, not as a first-release shipped CLI surface.
- Verified against:
  - code: `rust-next/crates/batchalign-server/src/routes/dashboard.rs`
  - docs: `book/src/user-guide/cli-reference.md`, `book/src/developer/tauri-react-dashboard.md`
- Follow-ups: continue final user-guide drift sweeps only if new public release-surface decisions change.

## 2026-03-05 19:XX (local)
- Page: `book/src/{developer/tauri-react-dashboard.md,decisions/tauri-react-dashboard-adoption.md,user-guide/cli-reference.md,user-guide/rust-server.md}`
- Change type: `revise`
- Summary: Corrected an overcorrection in dashboard wording. The server-hosted web dashboard remains a real supported surface when assets are installed; only the desktop/Tauri launcher path and `batchalign3 gui` are deferred from the first public release.
- Verified against:
  - code: `rust-next/crates/batchalign-server/src/routes/dashboard.rs`
  - code: `apps/dashboard-desktop/src-tauri/Cargo.toml`, `apps/dashboard-desktop/package.json`
- Follow-ups: keep future release notes precise about web dashboard versus desktop launcher so those surfaces do not get conflated again.

## 2026-03-05 19:XX (local)
- Page: `book/src/{migration/user-migration.md,migration/index.md}`
- Change type: `revise`
- Summary: Fixed the remaining migration-book dashboard ambiguity so it now distinguishes the real server-hosted web dashboard from the deferred desktop/Tauri launcher path.
- Verified against:
  - docs: `book/src/user-guide/{cli-reference,rust-server}.md`, `book/src/developer/tauri-react-dashboard.md`
  - code: `rust-next/crates/batchalign-server/src/routes/dashboard.rs`
- Follow-ups: keep user-facing release-surface language synchronized with actual shipped entry points.

## 2026-03-05 19:XX (local)
- Page: `rust-next/crates/batchalign-cli/src/error.rs`, `book/src/user-guide/cli-reference.md`
- Change type: `revise`
- Summary: Removed stale `GuiLaunch` error handling after `gui` subcommand removal and clarified that `bench`/`openapi` are contributor-facing tooling in the CLI reference.
- Verified against:
  - tests: `cargo test -p batchalign-cli -p batchalign-bin --tests --no-run`
- Follow-ups: demote contributor-only commands further in navigation only if you want an even narrower end-user release story.

## 2026-03-05 20:XX (local)
- Page: `book/src/migration/{user-migration,developer-migration}.md`, `analysis/code-first-migration-audit-2026-03-05.md`
- Change type: `revise`
- Summary: Extended the code-first migration story for utility commands so `setup`, `models`, `version`, `cache`, `bench`, and BA3-only ops commands are now described with the same Jan 9 BA2 / Feb 9 BA2 / current BA3 precision as the processing commands.
- Verified against:
  - code: `git -C /Users/chen/batchalign2-master show 84ad500b09e52a82aca982c41a8ccd46b01f4f2c:batchalign/cli/cli.py`, `git -C /Users/chen/batchalign2-master show e8f8bfada6170aa0558a638e5b73bf2c3675fe6d:batchalign/cli/{cli,cache,bench}.py`
  - code: `rust-next/crates/batchalign-cli/src/{setup_cmd,cache_cmd,bench_cmd,jobs_cmd,logs_cmd,serve_cmd,models_cmd}.rs`, `rust-next/crates/batchalign-bin/src/main.rs`
- Follow-ups: continue the migration-book cleanup by de-duplicating utility-command discussion across `index.md`, `user-migration.md`, and `developer-migration.md` once the remaining command/runtime audit is complete.

## 2026-03-05 20:XX (local)
- Page: `book/src/migration/index.md`
- Change type: `revise`
- Summary: De-duplicated the migration index so it now functions as a summary/map page, with detailed command/runtime/algorithm discussion explicitly delegated to the user, developer, and algorithms migration chapters.
- Verified against:
  - docs: `book/src/migration/{user-migration,developer-migration,algorithms-and-language}.md`
- Follow-ups: if the migration book grows further, continue this same summary-vs-detail split to keep top-level pages readable.

## 2026-03-05 20:XX (local)
- Page: `book/src/user-guide/{server-mode,rust-server,cli-reference}.md`, `book/src/migration/user-migration.md`
- Change type: `revise`
- Summary: Corrected runtime-routing docs against current dispatch code. Explicit remote `--server` is ignored only for `transcribe`, `transcribe_s`, and `avqi`; `benchmark` remains remotely dispatchable but is sidecar-eligible in local auto-daemon workflows when the main daemon lacks capability. Also tightened the migration wording around legacy flags so only genuinely current no-op flags are described that way.
- Verified against:
  - code: `rust-next/crates/batchalign-cli/src/dispatch/mod.rs`, `rust-next/crates/batchalign-cli/src/daemon.rs`, `rust-next/crates/batchalign-cli/src/args/mod.rs`
- Follow-ups: continue code-first auditing of runtime/config claims wherever current docs still compress together remote-dispatch behavior and local sidecar behavior.

## 2026-03-05 16:XX (local)
- Page: `book/src/reference/command-io.md`, `book/src/user-guide/cli-reference.md`, `book/src/user-guide/quick-start.md`, `book/src/architecture/server-architecture.md`, `book/src/reference/forced-alignment.md`
- Change type: `revise`
- Summary: Corrected current Rust CLI/runtime semantics for `--file-list`, daemon port behavior, `--override-cache` vs compatibility `--use-cache`, remote-media command routing, non-matching file copying, dummy CHAT filtering, and current align/UTR wording.
- Verified against:
  - code: `rust-next/crates/batchalign-cli/src/{args/mod.rs,args/commands.rs,dispatch/mod.rs,dispatch/single.rs,dispatch/paths.rs,discover.rs,resolve.rs,daemon.rs}`
  - code: `rust-next/crates/batchalign-chat-ops/src/fa/{alignment.rs,grouping.rs}`
  - code: `rust-next/crates/batchalign-server/src/fa.rs`
- Follow-ups: Continue code-first sweep for remaining legacy wording around older ASR flag spellings and any docs that still imply random-port daemon behavior or broad DP remap in current runtime paths.

## 2026-03-05 16:XX (local)
- Page: `book/src/user-guide/installation.md`, `book/src/reference/benchmarks.md`, `book/src/reference/whisper-asr.md`, `book/src/architecture/python-rust-interface.md`, `book/src/architecture/server-model-loading.md`, `book/src/architecture/chat-divorce.md`, `book/src/migration/algorithms-and-language.md`
- Change type: `revise`
- Summary: Corrected remaining current-state drift around ASR flag spelling, transcribe default-engine claims, FA runtime mapping wording, and overbroad “DP removed” migration language.
- Verified against:
  - runtime: `cargo run -p batchalign-bin -- transcribe|align|benchmark|bench --help`
  - code: `rust-next/crates/batchalign-cli/src/args/{mod.rs,commands.rs,options.rs}`
  - code: `rust-next/crates/batchalign-chat-ops/src/fa/alignment.rs`
  - code: `rust-next/crates/batchalign-cli/src/bench_cmd.rs`
- Follow-ups: Sweep lower-priority historical/reference pages where internal engine registry names (`whisper_oai`, `whisper_fa`) are intentionally retained but may still need clearer distinction from public CLI flags.

## 2026-03-05 16:XX (local)
- Page: `book/src/reference/align-throughput.md`, `book/src/architecture/engine-interface.md`, `book/src/reference/master-tier-output.md`, `book/src/reference/utr-alignment.md`, `book/src/architecture/overview.md`, `book/src/user-guide/cli-reference.md`
- Change type: `revise`
- Summary: Replaced stale Python-executor throughput claims with current Rust-server throughput notes; clarified internal engine names vs public CLI flags; and narrowed old blanket DP wording in UTR/FA summaries.
- Verified against:
  - code: `rust-next/crates/batchalign-server/src/{runner/mod.rs,fa.rs}`
  - code: `rust-next/crates/batchalign-worker/src/pool/mod.rs`
  - code: `rust-next/crates/batchalign-chat-ops/src/fa/alignment.rs`
  - code: `rust/src/{parsed_chat/speakers.rs,speaker_ops.rs}`
  - runtime: `cargo run -p batchalign-bin -- transcribe|align|benchmark|bench --help`
- Follow-ups: Continue sweeping mixed current/historical reference pages (`reference/utr-alignment.md`, `reference/master-tier-output.md`) for broader restructuring if they remain too hybrid for public readers.

## 2026-03-05 17:XX (local)
- Page: `book/src/reference/mwt-handling.md`, `book/src/reference/align-throughput.md`, `book/src/architecture/overview.md`
- Change type: `revise`
- Summary: Converted mixed design-memo language into present-tense current-state documentation for MWT handling and align throughput, while explicitly labeling old Python executor details as historical context.
- Verified against:
  - code: `rust-next/crates/batchalign-chat-ops/src/retokenize/{mapping.rs,tests.rs}`
  - code: `rust-next/crates/batchalign-server/src/{runner/mod.rs,fa.rs}`
  - code: `rust-next/crates/batchalign-worker/src/pool/mod.rs`
- Follow-ups: Continue converting mixed reference pages that still read as migration design notes rather than current implementation references.

## 2026-03-05 17:XX (local)
- Page: `book/src/reference/utr-alignment.md`, `book/src/reference/master-tier-output.md`, `book/src/architecture/rust-server-migration.md`
- Change type: `revise`
- Summary: Labeled older Python UTR and master-branch output pages as historical references and narrowed one more FA migration overclaim from generic “DP alignment” to timing mapping/injection.
- Verified against:
  - code: `rust/src/{parsed_chat/speakers.rs,speaker_ops.rs}`
  - code: `rust-next/crates/batchalign-chat-ops/src/fa/alignment.rs`
  - docs: `book/src/reference/forced-alignment.md`, `book/src/migration/{user-migration,algorithms-and-language}.md`
- Follow-ups: Consider de-navving or compressing these historical reference pages once migration-critical substance has been fully preserved in current migration chapters.

## 2026-03-05 17:XX (local)
- Page: `book/src/migration/{index,user-migration,developer-migration}.md`
- Change type: `revise`
- Summary: Tightened migration-book partitioning: made the index more precise about current release surfaces, consolidated repeated command tables in `user-migration.md`, and clarified that `developer-migration.md` covers architectural consequences rather than re-stating user-facing command history.
- Verified against:
  - docs: `analysis/code-first-migration-audit-2026-03-05.md`, `analysis/command-retention-audit-2026-03-05.md`
  - code anchors already used in prior migration audit passes for Jan 9 BA2, Feb 9 BA2, and current BA3 command/runtime comparisons
- Follow-ups: Continue shrinking residual repetition between `user-migration.md` and `algorithms-and-language.md`, especially around `align`/UTR wording.

## 2026-03-05 17:XX (local)
- Page: `book/src/migration/{index,user-migration,developer-migration}.md`
- Change type: `revise`
- Summary: Consolidated repeated command history in the migration book, made `openapi` contributor-facing in the user chapter, and tightened the index/runtime summary so the migration book has a cleaner division between user-visible deltas and developer-facing architecture changes.
- Verified against:
  - docs: `analysis/code-first-migration-audit-2026-03-05.md`, `analysis/command-retention-audit-2026-03-05.md`
  - code anchors already used in prior migration audit passes for Jan 9 BA2, Feb 9 BA2, and current BA3 command/runtime comparisons
- Follow-ups: Final migration-book pass should focus on shrinking residual repetition between `user-migration.md` and `algorithms-and-language.md` around alignment/UTR wording.

## 2026-03-05 17:XX (local)
- Page: `book/src/migration/{user-migration,algorithms-and-language}.md`
- Change type: `revise`
- Summary: Reduced migration-book overlap by trimming `user-migration.md` down to user-visible alignment/morphotag consequences and leaving UTR/retokenization/%gra mechanism detail in `algorithms-and-language.md`, with explicit cross-references between the two chapters.
- Verified against:
  - docs: `analysis/code-first-migration-audit-2026-03-05.md`
  - code anchors already used in prior audit passes for `rust/src/{parsed_chat/speakers.rs,speaker_ops.rs}` and `rust-next/crates/batchalign-chat-ops/src/{fa/alignment.rs,retokenize/mapping.rs,nlp/mapping.rs}`
- Follow-ups: Do one more migration-book consistency read to eliminate any remaining ambiguous “old/new” wording that is not explicitly Jan 9 BA2, Feb 9 BA2, or current BA3.

## 2026-03-05 17:XX (local)
- Page: `book/src/migration/{user-migration,algorithms-and-language}.md`
- Change type: `revise`
- Summary: Normalized remaining ambiguous “old/new” migration wording into explicit BA2-baseline, Feb 9 BA2, and current BA3 references so the public migration story no longer depends on relative labels.
- Verified against:
  - docs: `book/src/migration/{index,user-migration,developer-migration,algorithms-and-language}.md`
  - docs: `analysis/code-first-migration-audit-2026-03-05.md`
- Follow-ups: Remaining migration-book work is now mainly final consistency reading rather than substantive restructuring.

## 2026-03-05 17:XX (local)
- Page: `book/src/migration/{user-migration,developer-migration}.md`
- Change type: `revise`
- Summary: Final consistency polish for the migration book: clarified that OpenAPI is contributor-facing even when discussing API UX, moved MWT user wording toward consequences with a pointer to the algorithm chapter, and fixed developer-chapter heading structure to distinguish the BA2 mental model from the current BA3 one.
- Verified against:
  - docs: `book/src/migration/{index,user-migration,developer-migration,algorithms-and-language}.md`
- Follow-ups: Migration-book work is effectively in final consistency state; remaining improvements would be optional wording refinement rather than missing factual restructuring.

## 2026-03-05 17:XX (local)
- Page: `book/src/{introduction,SUMMARY}.md`, `book/src/developer/{documentation-scope,documentation-readiness}.md`
- Change type: `revise`
- Summary: Updated the book’s outer layers to match the cleaned migration/release story: introduction now points BA2 migrators to the migration book, navigation labels historical UTR/master-tier references explicitly, and the documentation meta-pages now describe the current public/private boundary and current 2026-03-05 readiness state instead of a stale February assessment.
- Verified against:
  - docs: `book/src/{introduction,SUMMARY}.md`
  - docs: `book/src/developer/{documentation-scope,documentation-readiness}.md`
  - analysis: `analysis/docs-public-private-triage-2026-03-05.md`, `analysis/docs-revision-matrix-2026-03-05.md`
- Follow-ups: Continue scanning other top-level landing/reference pages for historical material that should be labeled in navigation or de-emphasized behind current migration/reference pages.

## 2026-03-05 17:XX (local)
- Page: `book/src/architecture/{chat-parsing,rust-server-migration}.md`, `book/src/decisions/lenient-parsing.md`, `book/src/reference/{gra-format,dynamic-programming,filesystem-paths}.md`, `book/src/SUMMARY.md`
- Change type: `revise`
- Summary: Removed more transient campaign language from outer-layer architecture/reference pages by converting phased migration and recommendation prose into current-state descriptions, relabeling the Rust server page as implemented architecture, rewriting lenient parsing as a current decision page, and compressing `%gra`/DP/filesystem references down to current behavior plus concise legacy notes.
- Verified against:
  - docs: `book/src/migration/{index,user-migration,developer-migration,algorithms-and-language}.md`
  - code anchors already used in prior audit passes for current CHAT parsing, worker/server orchestration, `%gra` mapping, UTR, FA, and cache/runtime path behavior
- Follow-ups: Continue sweeping remaining reference/decision pages that still expose transient experiment names, branch-specific history, or “future work” language more prominently than current behavior.

## 2026-03-05 17:XX (local)
- Page: `book/src/architecture/caching.md`, `book/src/decisions/{google-translate-migration,cache-rust-migration}.md`, `book/src/SUMMARY.md`
- Change type: `revise`
- Summary: Replaced more branch-era assessment/proposal pages with concise current-state or historical/deferred decision notes: caching is now documented as implemented current behavior, Rust-side cache ownership is documented as implemented rather than proposed, and Google Translate port discussion is reduced to a short historical/deferred note instead of live migration-option prose.
- Verified against:
  - docs: `book/src/migration/{index,user-migration}.md`
  - docs: `book/src/architecture/caching.md`
  - docs: `book/src/decisions/{google-translate-migration,cache-rust-migration}.md`
- Follow-ups: Continue auditing remaining decision/reference pages whose titles still use “migration” or whose bodies may still contain transient implementation-option material better suited for private archive.

## 2026-03-05 17:XX (local)
- Page: `book/src/reference/{per-word-language-routing,per-utterance-language-routing,retrace-detection,multilingual,l2-handling,language-handling,textgrid,japanese-morphosyntax}.md`
- Change type: `revise`
- Summary: Collapsed another cluster of investigation-heavy language/routing/reference pages into current behavior plus current limits: removed option-analysis and rewrite-era findings from per-word/per-utterance routing, multilingual handling, retrace detection, and L2 handling; also normalized Japanese and TextGrid wording so they describe present behavior instead of future options or low-priority plans.
- Verified against:
  - docs: `book/src/migration/{user-migration,algorithms-and-language}.md`
  - docs: `book/src/reference/{per-word-language-routing,per-utterance-language-routing,multilingual,language-handling,l2-handling}.md`
- Follow-ups: Continue sweeping remaining architecture/decision pages that still present recommendations or future work more prominently than current implemented behavior.

## 2026-03-05 17:XX (local)
- Page: `book/src/architecture/{validation,server-architecture}.md`
- Change type: `revise`
- Summary: Replaced the remaining validation audit/roadmap page with a current validation architecture page and tightened server-architecture wording so fleet history is clearly historical rather than “planned” public functionality.
- Verified against:
  - docs: `book/src/migration/{index,user-migration,developer-migration}.md`
  - docs: `book/src/architecture/{validation,server-architecture,command-contracts}.md`
- Follow-ups: Remaining outer-layer drift is now mostly limited to a few explicitly historical or accepted decision pages rather than broad current-doc inaccuracies.

## 2026-03-05 17:XX (local)
- Page: `book/src/architecture/python-rust-interface.md`, `book/src/architecture/server-known-issues.md`, `book/src/decisions/{downstream-integration,long-term-architecture-charter}.md`
- Change type: `revise`
- Summary: Tightened another small cluster of outer-layer pages by collapsing Rev.AI pre-submission option analysis into current implemented behavior, rewriting downstream integration as a current accepted decision instead of a backlog/spec sheet, making the charter’s archived roadmap explicitly historical, and converting one server-known-issues note from roadmap language to a current “not implemented” statement.
- Verified against:
  - docs: `book/src/architecture/{python-rust-interface,rust-server-migration,server-known-issues}.md`
  - docs: `book/src/decisions/{downstream-integration,long-term-architecture-charter}.md`
- Follow-ups: Remaining work is now mostly optional polish and possible archival/de-nav of some explicitly historical pages rather than correction of current-reader-facing inaccuracies.

## 2026-03-05 17:XX (local)
- Page: `book/src/decisions/tauri-react-dashboard-adoption.md`, `book/src/developer/string-check-optimizations.md`, `book/src/reference/master-tier-output.md`, `book/src/SUMMARY.md`
- Change type: `revise`
- Summary: Finished another tail-cleanup pass by removing remaining gate/work-log framing from the dashboard ADR and string-check note, clarifying the historical Python tier-output page title, and aligning the dashboard ADR nav label with its simplified current role.
- Verified against:
  - docs: `book/src/decisions/tauri-react-dashboard-adoption.md`
  - docs: `book/src/developer/string-check-optimizations.md`
  - docs: `book/src/reference/master-tier-output.md`
  - docs: `book/src/SUMMARY.md`
- Follow-ups: Most remaining work is now optional historical-page curation and de-nav/archive decisions rather than factual cleanup of current-facing documentation.

## 2026-03-05 17:XX (local)
- Page: `book/src/{SUMMARY,reference/utr-alignment,reference/master-tier-output,developer/string-check-optimizations,decisions/google-translate-migration}.md`
- Change type: `de-nav + stub`
- Summary: Executed another public/private curation move by removing four purely historical pages from public navigation and replacing each with a short link-stable stub that points readers back to current behavior or migration docs. This keeps the public book focused on current state and released BA2 -> BA3 deltas while pushing archaeology toward private/internal storage.
- Verified against:
  - docs: `book/src/SUMMARY.md`
  - analysis: `analysis/docs-public-private-triage-2026-03-05.md`
- Follow-ups: Continue deciding whether additional explicitly historical pages, especially `rust-migration-proposal.md`, should remain public ADR context or receive the same de-nav + stub treatment.

## 2026-03-05 17:XX (local)
- Page: `book/src/{SUMMARY,decisions/rust-migration-proposal}.md`
- Change type: `de-nav + stub`
- Summary: Removed the raw Rust migration proposal from public navigation and replaced it with a minimal historical stub. The public book now points readers to implemented architecture and migration pages instead of exposing proposal history as a first-class decision page.
- Verified against:
  - docs: `book/src/SUMMARY.md`
  - analysis: `analysis/docs-public-private-triage-2026-03-05.md`
- Follow-ups: Continue using the same rule for any remaining proposal-style material: keep current decisions and implemented architecture public, move raw proposal history private.

## 2026-03-05 17:XX (local)
- Page: `book/src/developer/{python-versioning,python-314t-migration}.md`, `book/src/SUMMARY.md`
- Change type: `compress-public`
- Summary: Compressed the remaining public 3.14t material down to current policy only. `python-versioning.md` now states the 3.12 baseline and paused 3.14t policy without retaining the experiment log, and `python-314t-migration.md` is now a short paused-status note instead of a long deployment/compatibility history page.
- Verified against:
  - docs: `book/src/developer/{python-versioning,python-314t-migration}.md`
  - docs: `book/src/SUMMARY.md`
  - analysis: `analysis/docs-public-private-triage-2026-03-05.md`
- Follow-ups: Remaining curation work is now mostly optional de-nav/archive decisions for other historical stubs, not substantive cleanup of current public guidance.

## 2026-03-05 17:XX (local)
- Page: `book/src/{SUMMARY,developer/python-314t-migration}.md`
- Change type: `de-nav + stub`
- Summary: Removed the paused 3.14t page from public navigation and reduced it to a minimal stub. This keeps the public book focused on current supported runtime policy (`python-versioning.md`) instead of a paused experiment track that is not relevant to new BA3 users or BA2 migrators.
- Verified against:
  - docs: `book/src/SUMMARY.md`
  - analysis: `analysis/docs-public-private-triage-2026-03-05.md`
- Follow-ups: Remaining de-nav candidates are now mostly optional historical stubs rather than active public reading paths.

## 2026-03-05 18:XX (local)
- Page: `book/src/{reference/utr-alignment,reference/master-tier-output,reference/python-gra-generation-analysis,reference/multi-file-optimization,developer/pre-commit,developer/python-314t-migration,developer/string-check-optimizations,decisions/google-translate-migration,decisions/rust-migration-proposal,migration/batchalignhk-to-plugins}.md`, `book/src/{reference/gra-format,developer/python-versioning}.md`
- Change type: `move-private`
- Summary: Converted the first-wave public/private triage from de-nav-only handling to actual relocation. Ten purely historical or private pages were moved out of `batchalign3` into `talkbank-private/docs/batchalign3-reports/...` and deleted from the public repo. Remaining public references to moved pages were removed from `gra-format.md` and `python-versioning.md`.
- Verified against:
  - docs: `book/src/reference/gra-format.md`
  - docs: `book/src/developer/python-versioning.md`
  - private archive: `talkbank-private/docs/batchalign3-reports/`
- Follow-ups: Continue scanning for any remaining files in `batchalign3` whose only value is private/internal history and move them out rather than keeping de-navved placeholders.

## 2026-03-05 18:XX (local)
- Page: `book/src/{developer/documentation-readiness,developer/documentation-scope}.md`, `book/src/SUMMARY.md`, `book/src/{architecture/server-known-issues,reference/proportional-fa-estimation,reference/wor-tier,reference/morphosyntax}.md`
- Change type: `move-private + trim-public`
- Summary: Moved the internal documentation-governance pages completely out of `batchalign3` into `talkbank-private/docs/batchalign3-reports/developer/` and removed them from public navigation. Also archived resolved incident history and forensic bug-analysis sections from mixed public pages into private archive files, leaving the public pages focused on current behavior and migration-relevant guidance.
- Verified against:
  - docs: `book/src/SUMMARY.md`
  - docs: `book/src/architecture/server-known-issues.md`
  - docs: `book/src/reference/{proportional-fa-estimation,wor-tier,morphosyntax}.md`
  - private archive: `talkbank-private/docs/batchalign3-reports/{architecture,developer,reference}/`
- Follow-ups: Continue scanning for embedded rewrite-era sections whose only value is internal history; archive those sections rather than leaving them inline in public current-state pages.

## 2026-03-05 18:XX (local)
- Page: `book/src/{architecture/server-architecture,architecture/overview,architecture/caching,reference/align-throughput,reference/multilingual,reference/l2-handling,architecture/rust-server-migration,decisions/cache-rust-migration,decisions/downstream-integration,decisions/long-term-architecture-charter,decisions/tauri-react-dashboard-adoption}.md`
- Change type: `trim-public`
- Summary: Removed residual rewrite-era meta-history from otherwise current public pages. Fleet history was archived privately to `talkbank-private/docs/batchalign3-reports/architecture/fleet-history.md`; remaining public edits removed explanatory note blocks whose only purpose was to describe older drafts or former proposal status.
- Verified against:
  - docs: `book/src/architecture/server-architecture.md`
  - docs: `book/src/decisions/{cache-rust-migration,downstream-integration,tauri-react-dashboard-adoption}.md`
  - private archive: `talkbank-private/docs/batchalign3-reports/architecture/fleet-history.md`
- Follow-ups: Remaining public “historical/deferred” wording should now be limited to legitimate current-status statements rather than archived content.

## 2026-03-05 19:XX (local)
- Page: `analysis/`, `docs/archive/`, `book/src/developer/deployment-separation.md`, `CLAUDE.md`, `rust-next/README.md`, `rust/{,next}/CLAUDE.md`, `book/src/{SUMMARY,developer/rust-workspace-map,reference/textgrid}.md`
- Change type: `move-private + revise-public`
- Summary: Removed the remaining large private/archive trees from the public repo by moving `batchalign3/analysis/` to `talkbank-private/docs/batchalign3-repo-analysis/` and `batchalign3/docs/archive/` to `talkbank-private/docs/batchalign3-public-archive/`. Moved the private deployment page out of the public book into `talkbank-private/ops/batchalign3/deployment/`. Revised public `CLAUDE.md` / README surfaces to keep repository-appropriate guidance only, while creating `talkbank-private/ops/batchalign3/agent-guides/CLAUDE.md` for private deployment addenda.
- Verified against:
  - docs removed from public repo: `analysis/`, `docs/archive/`, `book/src/developer/deployment-separation.md`
  - public docs: `CLAUDE.md`, `rust-next/README.md`, `book/src/SUMMARY.md`
  - private ops: `talkbank-private/ops/batchalign3/`
- Follow-ups: Audit the remaining in-repo `CLAUDE.md` files for public appropriateness, but keep them as repository-local technical guidance rather than moving them wholesale.

## 2026-03-05 19:XX (local)
- Page: `book/src/{user-guide/server-mode,reference/gra-correctness-guarantee,reference/proportional-fa-estimation,reference/wor-tier-bullet-bug,user-guide/plugins,architecture/error-handling,architecture/chat-parsing,architecture/server-model-loading,developer/manual-anchor-audit,developer/server-yaml-template,rust-core}.md`, `book/src/developer/{launchd-template.plist,setup-launchd.sh}.md`, `rust/CLAUDE.md`
- Change type: `trim-public + move-private`
- Summary: Removed remaining machine-specific path leakage from public docs (`~/talkbank`, `/Users/chen/...`, `Users/macw`, production/lab-machine wording) and normalized examples to generic repo-relative or placeholder paths. Moved the raw launchd deployment artifacts out of the public book into `talkbank-private/ops/batchalign3/deployment/`. Kept the public launchd guide but sanitized it into a generic service-account example.
- Verified against:
  - docs: `book/src/user-guide/server-mode.md`
  - docs: `book/src/{architecture/error-handling,developer/manual-anchor-audit,developer/rust-core}.md`
  - private ops: `talkbank-private/ops/batchalign3/deployment/{launchd-template.plist,setup-launchd.sh}`
  - repo-wide grep: no remaining private-path refs in source docs
- Follow-ups: Remaining environment-specific paths should now be limited to legitimate placeholders (for example `/path/to/...`) rather than real local/private paths.

## 2026-03-05 19:XX (local)
- Page: `README.md`, `BATCHALIGN2_DELTA.md`, `examples/fleet.yaml`, `frontend/CLAUDE.md`, `rust-next/{README,CLAUDE}.md`, `rust-next/crates/batchalign-cli/CLAUDE.md`
- Change type: `move-private + revise-public`
- Summary: Removed stale fleet-era guidance from the remaining top-level/public helper docs. `BATCHALIGN2_DELTA.md` was moved out to `talkbank-private/docs/batchalign3-reports/root/` in favor of the migration book, and `examples/fleet.yaml` was moved into the private archive. The retained README/CLAUDE files were updated so fleet is described only as disabled discovery metadata where it still exists in code, not as an active public release surface.
- Verified against:
  - docs: `README.md`, `rust-next/README.md`
  - internal guides: `frontend/CLAUDE.md`, `rust-next/{CLAUDE.md,crates/batchalign-cli/CLAUDE.md}`
  - private archive: `talkbank-private/docs/batchalign3-{reports/public-archive}/`
- Follow-ups: Remaining fleet mentions should now be either code-level retained internals or explicit current disabled-state notes.

## 2026-03-05 20:XX (local)
- Page: `.claude/`, `.coverage`, `batchalign_core/libbatchalign_core.dylib.dSYM/`, `artifacts/`
- Change type: `move-private + remove-empty-shell`
- Summary: Moved the remaining private/internal repo residue out of `batchalign3`. Archived hidden agent skill material from `.claude/skills/` into `talkbank-private/ops/batchalign3/agent-guides/skills/`, moved `.coverage` and the debug-symbol bundle into `talkbank-private/artifacts/batchalign3/`, then moved dated verification/UAT logs from `artifacts/` into `talkbank-private/artifacts/batchalign3/repo-artifacts/`. Removed the now-empty `.claude/` directory from the public repo tree.
- Verified against:
  - public repo tree: `batchalign3/`
  - private archive: `talkbank-private/artifacts/batchalign3/`
  - private ops: `talkbank-private/ops/batchalign3/agent-guides/skills/`
- Follow-ups: Only generic public examples and current docs should remain under `batchalign3`; further cleanup should target tracked public files, not local untracked caches.

## 2026-03-05 20:XX (local)
- Page: `talkbank-private/docs/batchalign3-reports/{README,CLAUDE}.md`, `talkbank-private/artifacts/batchalign3/README.md`, `talkbank-private/ops/batchalign3/agent-guides/skills/README.md`, `book/src/{reference/textgrid,reference/align-throughput,architecture/overview,architecture/command-contracts}.md`
- Change type: `organize-private + trim-public`
- Summary: Added private index/guide files so moved reports, artifacts, and agent skills have an explicit home and policy boundary in `talkbank-private`. Cleaned the remaining public drift in `textgrid.md` by removing proposal-style implementation notes and stale references to moved internal audits. Tightened a few residual public "historical" phrasings so they read as comparison context rather than archive material.
- Verified against:
  - private docs: `talkbank-private/docs/batchalign3-reports/README.md`
  - private artifacts: `talkbank-private/artifacts/batchalign3/README.md`
  - public docs: `book/src/reference/textgrid.md`, `book/src/architecture/overview.md`
- Follow-ups: Public-repo cleanup is now mostly at commit/staging level; further work should focus on any specifically identified page, not broad repo hygiene.

## 2026-03-05 20:XX (local)
- Page: `book/src/{user-guide/rust-server,migration/user-migration,migration/index}.md`
- Change type: `trim-public-release-surface`
- Summary: Removed unnecessary `fleet` framing from user-facing server and migration docs. Public docs now present the released runtime as single-server/daemon oriented, with `fleet` retained only in architecture/developer context where dormant code paths still exist.
- Verified against:
  - public docs: `book/src/user-guide/rust-server.md`
  - migration docs: `book/src/migration/{index,user-migration}.md`
  - grep: no remaining `fleet` mentions under `book/src/user-guide/`
- Follow-ups: Remaining `fleet` mentions are now limited to developer/architecture docs and live code surfaces.

## 2026-03-05 20:XX (local)
- Page: `book/src/{reference/filesystem-paths,reference/command-io,migration/index,migration/developer-migration,decisions/lenient-parsing}.md`, `book/src/SUMMARY.md`
- Change type: `trim-public-release-surface + validate`
- Summary: Reduced the remaining dormant-feature wording in public docs by removing `fleet` from user/reference/migration surfaces except where architecture/developer context still needs to describe live dormant code. Also reviewed the nav-visible Design Decision pages and kept them public because they now read as accepted/current ADRs rather than proposals. Final `mdbook build book` completed successfully.
- Verified against:
  - public docs: `book/src/reference/filesystem-paths.md`, `book/src/migration/developer-migration.md`
  - build: `mdbook build book`
  - grep: remaining `fleet` mentions limited to architecture/developer docs
- Follow-ups: Remaining work is commit/staging discipline, not broad doc restructuring.
