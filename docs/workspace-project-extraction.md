# Workspace Project Extraction Status

**Status:** Current
**Last updated:** 2026-03-22 07:14 EDT

Projects developed inside the `talkbank-dev` meta-workspace that should be
evaluated for extraction into independent GitHub repos. Organized by readiness.

---

## Already Extracted (on GitHub under TalkBank org)

These are gitignored sub-repos cloned via `make clone`:

| Repo | Language | Lines | Notes |
|------|----------|-------|-------|
| `talkbank-tools` | Rust | ~45,000 | Core: grammar, parser, model, CLI, LSP, desktop app. Public. |
| `batchalign3` | Rust + Python | ~25,000 | NLP pipeline. Public. |
| `update-chat-types` | Rust | ~1,000 | Pre-commit hook for @Types headers. |
| `sync-media` | Rust | 846 | rclone wrapper for media file sync. |
| `generate-from-chat` | Python | 1,001 | Builds corpus ZIPs, injects DOIs into HTML. |
| `gra-cgi` | Perl | 153 | MOR/GRA diagram CGI. Tiny, stable. |
| `webdev` | Python | 153 | Web config generation. |
| `talkbank-browser-check` | JS + Python | 486 py / 103k js | Link checker with login. |
| `cdcs-to-csv` | Python | 537 | Legacy DOI tool. **Broken, being replaced by `talkbank-doi`.** |

---

## Ready to Extract

### talkbank-doi

| | |
|---|---|
| **Language** | Rust |
| **Lines** | 2,476 |
| **Dependencies** | All from crates.io — no path deps |
| **Purpose** | DOI lifecycle: mint, sync, audit, review TUI |
| **Users** | Franklin (primary), Brian (via `sync` command) |
| **Succession** | A successor needs this to maintain DOI registrations |

**Status:** Fully functional. Has `check`, `sync`, `query`, `export`, `audit`,
and `review` (TUI) commands. Clean build, 9 unit tests. Uses DataCite REST API
with typed `Doi` newtype and `DoiState` enum.

**To extract:** Add CI workflow. Add README with env var setup and usage
examples. Replaces `cdcs-to-csv` — note that in the README and archive the old
repo.

**Visibility:** Private initially (contains DataCite API integration details),
but could be public — there's nothing secret in the code itself.

---

## Should Extract When Complete

### sbcsae-converter

| | |
|---|---|
| **Language** | Rust |
| **Lines** | 4,203 |
| **Dependencies** | All crates.io (`clap`, `winnow`, `petgraph`, `encoding_rs`) |
| **Purpose** | Extract overlap indices from original SBCSAE `.trn` files and merge into hand-edited CHAT |
| **Users** | Franklin |
| **Succession** | One-time migration tool, but the converter is a reference implementation of TRN parsing |

**Status:** Parser and overlap extraction work. Merge-back into CHAT is
incomplete. 60 TRN files at `~/sbcsae-trn/`.

**To extract:** Finish the merge-back step. Add test fixtures (a few `.trn` +
`.cha` pairs). Write a README explaining the TRN format and what the tool
does. The `trn-format-analysis.md` already in the project is good internal
documentation.

**Visibility:** Public. The SBCSAE corpus is public and so is the TRN format.
Having this tool available would help other researchers working with SBCSAE.

### check-media

| | |
|---|---|
| **Language** | Rust |
| **Lines** | 1,912 |
| **Dependencies** | All crates.io (`clap`, `rayon`, `regex`, `walkdir`, `chrono`) |
| **Purpose** | Verify media files referenced in CHAT headers exist, have correct formats, and match expected patterns |
| **Users** | Franklin, Davida (media QA) |
| **Succession** | Needed for ongoing media maintenance |

**Status:** Functional but still evolving. Used internally for media audits.

**To extract:** Stabilize the CLI interface. Add a README. Add a few test
fixtures.

**Visibility:** Public. Generic CHAT media verification — useful to anyone
working with TalkBank data.

---

## Experiment Projects: A New Category

These are self-contained research experiments with code, data, and written
results. They are candidates for extraction not because they are operational
tools, but because they are **reproducible research artifacts** that could
accompany tech reports or publications.

The case for giving experiments their own repos:

1. **Reproducibility.** A repo with a `README.md`, `PROVENANCE.md`, pinned
   dependencies (`Cargo.lock`), and clear run instructions lets anyone
   reproduce the results. A directory buried in a private meta-workspace does
   not.

2. **Citability.** A GitHub repo can be archived to Zenodo for a DOI. A tech
   report can cite the experiment repo as a permanent artifact. (We mint DOIs
   for corpora — we should mint them for our own research artifacts too.)

3. **Discoverability.** Published experiments show the community what analyses
   are possible with TalkBank data and tools. They serve as worked examples.

4. **Succession.** A successor can understand *why* a feature was built by
   reading the experiment that motivated it.

### per-speaker-utr-experiment (2026-03-16)

| | |
|---|---|
| **Language** | Rust (2,198 lines) + Python (357 lines, scripts) |
| **Dependencies** | Cargo.toml (standalone) |
| **Data** | 14 GB (audio + CHAT + ground truth across 9 corpora) |
| **Documentation** | 17 markdown files: README, PROVENANCE, experiment catalog, results summaries, Brian summary |
| **Purpose** | Determine whether per-speaker utterance-time recovery (UTR) improves timing coverage on overlap-heavy CHAT. Motivated the two-pass UTR feature shipped in batchalign3. |

**Status:** Experiment complete. Results documented. The two-pass UTR feature
it motivated has shipped as the default in batchalign3. Key finding: word-order
within speakers is well-preserved, per-speaker UTR recovers 15–40% more timed
utterances, but non-English regression is caused by forced-alignment grouping
sensitivity, not UTR itself.

**To extract:**

1. **Separate code from data.** The 14 GB `data/` directory cannot go in a git
   repo. Options:
   - Store data on Zenodo or Figshare (get a DOI for the dataset)
   - Store data on the TalkBank media server with a download script
   - Use `PROVENANCE.md` (already exists) to document where each input file
     came from and how to reconstruct the dataset from public TalkBank corpora

2. **Pin the batchalign3 commit.** The experiment depends on specific
   batchalign3 behavior. Record the commit hash in the README so results can
   be reproduced against that exact version.

3. **Add a `Makefile` or `justfile`** with targets like `make data` (download),
   `make run-experiment-1`, `make results` (generate summary tables from raw
   output). The current workflow is documented in markdown but not automated.

4. **Write the tech report.** The 17 existing markdown docs contain all the
   substance; they need to be consolidated into a single narrative document
   suitable for publication or posting as a TalkBank technical report.

**Visibility:** Public. All input corpora are from public TalkBank data. The
experiment demonstrates TalkBank tooling capabilities to the community.

---

## Not Worth Extracting

| Project | Why |
|---------|-----|
| `scripts/analysis/*.py` (5 files, 702 lines) | Generic corpus inspection scripts. Too small and varied for a standalone repo. Better as a `scripts/` directory in `talkbank-tools` or documented in the book. |
| `deploy/tools/demo_cluster.py` (427 lines) | Internal deploy utility. Lives with the deploy scripts it supports. |
| `staging/repos/cdcs-to-csv` | Already on GitHub. Being retired in favor of `talkbank-doi`. Archive, don't maintain. |
| `analysis/*.md` (historical audits) | Point-in-time snapshots from the consolidation. Reference material, not runnable code. |

---

## Proposed Naming Convention for Experiment Repos

To distinguish operational tools from research experiments:

- **Tools:** `talkbank-{name}` (e.g., `talkbank-doi`, `talkbank-tools`)
- **Experiments:** `talkbank-exp-{topic}-{date}` (e.g., `talkbank-exp-utr-overlap-2026-03`)

The date suffix is important: experiments are snapshots, not evolving software.
A new experiment on the same topic should be a new repo, not a branch of the
old one, so the original remains a stable citable artifact.

---

## Extraction Checklist

When extracting a project from the meta-workspace:

- [ ] Verify no path dependencies into other workspace projects
- [ ] Add `README.md` with build instructions, usage, and purpose
- [ ] Add `.github/workflows/ci.yml` (at minimum: `cargo build`, `cargo test`)
- [ ] Add `LICENSE` (MIT for tools, CC-BY for experiment writeups)
- [ ] For experiments: add `PROVENANCE.md` documenting data sources
- [ ] For experiments: separate large data from code (Zenodo, download script, or .gitignore + PROVENANCE)
- [ ] Create the GitHub repo (private or public per the table above)
- [ ] Push initial commit
- [ ] Update `talkbank-dev/Makefile` clone targets if the project should be part of the standard workspace setup
- [ ] Update `docs/inventory.md` with the new repo
