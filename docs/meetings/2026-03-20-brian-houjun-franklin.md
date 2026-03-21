# Zoom Meeting Prep: Brian + Houjun + Franklin

**Status:** Current
**Last updated:** 2026-03-20 13:00

**Date:** 2026-03-20 afternoon
**Participants:** Brian MacWhinney, Houjun Liu (remote), Franklin Chen
**Houjun status:** Still on batchalign2-master (Python). This meeting partly about transitioning him to batchalign3.

---

## 1. Adding Things to Batchalign3 (Houjun Onboarding)

### What's Different in BA3

| Aspect | batchalign2 (Houjun's current) | batchalign3 (new) |
|--------|-------------------------------|-------------------|
| Architecture | Python-primary | Rust-primary: ALL logic (CHAT parsing, DP alignment, WER, ASR post-processing, compound merging, retokenization) in Rust |
| Python role | Everything | Pure stateless ML model server (~5,900 lines, zero domain logic) |
| IPC | Python function calls | stdio JSON-lines protocol between Rust server and Python workers |
| Adding a command | Python pipeline class | Single dispatch site: `crates/batchalign-cli/src/lib.rs::run_command()`, typed options in `types/options.rs` |
| Adding inference | Python module + pipeline registration | Python inference module in `batchalign/inference/`, register in worker dispatch, Rust-side payload+cache+result injection in `batchalign-chat-ops` |

### Architecture Issues (External Reviewer Findings)

Documented in `docs/pre-release-radical-rearchitecture-program.md` (6 epics):

1. **Boundary types** — `LanguageCode3` carries sentinel values ("", "auto"); type doesn't guarantee meaning
2. **Silent fallbacks** — some failure paths silently normalized into defaults
3. **Worker ownership** — `SharedGpuWorker` claims child-lifecycle ownership but doesn't retain the child handle
4. **Adapter layers** — some adapters invent truth instead of reflecting it
5. **Test strategy** — heavy golden suites create noise during structural changes
6. **Cross-repo boundary** — current talkbank-tools/batchalign3 split is workable but not obviously final

### Talking Points

- What does Houjun want to add? (new commands? new languages? new models?)
- Can he stay in Python inference modules, or does he need to modify Rust?
- Should we pair on his first batchalign3 contribution to smooth the transition?

---

## 2. Cantonese

**Status: Working in batchalign3.** All HK engines folded into core repo.

### What Changed from batchalignHK

| Aspect | batchalignHK (old) | batchalign3 (now) |
|--------|-------------------|-------------------|
| Architecture | External plugin (`batchalign-hk-plugin`) | Core repo (`batchalign/inference/hk/`) |
| Text normalization | Python OpenCC library | Rust `zhconv` crate (100-200x faster, no C++ dep) |
| Normalization location | Python side | Rust side (`asr_postprocess/cantonese.rs`) |
| Engine selection | Plugin discovery/entry points | Direct enum dispatch (`AsrEngine`/`FaEngine`) |
| Python bridge | N/A | `batchalign_core.normalize_cantonese()` via PyO3 |

### 4 HK Engines

1. **Tencent Cloud ASR** — `--engine-overrides '{"asr": "tencent"}'` + credentials
2. **Aliyun NLS ASR** — `--engine-overrides '{"asr": "aliyun"}'` + credentials
3. **FunASR/SenseVoice** — `--engine-overrides '{"asr": "funaudio"}'` (local, no credentials)
4. **Cantonese FA** (Wave2Vec jyutping) — `--engine-overrides '{"fa": "wav2vec_canto"}'`

### Text Normalization Pipeline (Rust)

1. Stage 1: Simplified -> HK Traditional via `zhconv` (ZhHK variant)
2. Stage 2: 31-entry domain replacement table (13 multi-char + 18 single-char) via Aho-Corasick

Core normalization runs on ALL ASR output when `lang=yue`, regardless of engine.

### Questions for Brian/Houjun

- Have the HK engines been tested end-to-end on real Cantonese corpora recently?
- Any Cantonese-specific issues reported by users?
- Does Houjun want to add new HK engines or modify existing ones?

---

## 3. Compare

**Status: Implemented in batchalign3** (`crates/batchalign-chat-ops/src/compare.rs`).

### How It Works (BA3)

1. Parse both main and gold CHAT files via AST
2. Extract words from Mor domain, filter punctuation and fillers
3. Normalize via `conform_words` (compound splitting, name replacement, filler expansion)
4. Run Hirschberg DP alignment (case-insensitive)
5. Redistribute alignment results per-utterance
6. Produce `%xsrep` dependent tier with per-word match/insertion/deletion annotations
7. Compute aggregate metrics (WER, accuracy, match/insertion/deletion counts)

### Key Differences from BA2

| Aspect | BA2 (Houjun's Python) | BA3 (Rust) |
|--------|----------------------|------------|
| Implementation | `CompareEngine` + `CompareAnalysisEngine` in Python pipeline | Single `compare()` function in `compare.rs` |
| DP alignment | Python DP | Rust Hirschberg (linear space, divide-and-conquer) |
| Word normalization | Python `conform_words` | Rust `wer_conform::conform_words` (same logic, ported) |
| Orchestration | Pipeline system | Server-side: parse -> morphotag first -> compare against gold |
| Output | `%xsrep` tier | `%xsrep` tier (same format) |
| Morphosyntax | Runs as prerequisite pipeline stage | Server runs morphotag before compare automatically |

**Golden test:** `ml_golden/golden.rs::golden_compare_eng` tests with deliberate insertions/deletions, verifies `%xsrep` and `%mor` tiers present.

---

## 4. Opening Up to Public (GitHub + PyPI)

### What's Ready

- Core CLI and server architecture
- PyPI package name: `batchalign3`
- `pyproject.toml` with metadata, version, dependencies
- `uv` tooling for all Python operations
- CI workflows in `.github/workflows/` (test, release, docs, dashboard-desktop)
- Release wheels for 5 targets (macOS ARM+Intel, Linux x86+ARM, Windows x86)
- 70 ML tests + fast unit tests
- Book documentation (`book/`) with mdBook

### What's NOT Ready

- **Architecture cleanup** — 6 epics in `docs/pre-release-radical-rearchitecture-program.md` (sentinel values, silent failures, worker ownership, adapter layers, test strategy, cross-repo boundary)
- **README.md** — needs review for public consumption
- **Credential audit** — verify no API keys or sensitive paths in committed code
- **Chatter desktop app** — lives in `talkbank-tools/desktop/` (Tauri v2), separate release question
- **batchalign2 compatibility** — transition plan for existing PyPI users

### Questions for Brian/Houjun

- Target timeline for public release?
- Should batchalign3 replace batchalign2 on PyPI, or coexist?
- What's the minimum bar for going public? (functional completeness vs. architectural cleanliness)
- Chatter GUI: include in initial public release or defer?

---

## 5. TRESTLE

**What it is:** TRESTLE 2.0 (Toolkit for Reproducible Execution of Speech Text and Language Experiments) — an ACL submission by Trevor Cohen's group (Changye Li, Weizhe Xu, et al.). Code at `~/harmonized-toolkit` (GitHub: LinguisticAnomalies/harmonized-toolkit).

**Purpose:** Standardized preprocessing toolkit for TalkBank speech data. Handles audio resampling (16 kHz .wav), utterance-level clipping, text preprocessing of .cha files (regex-based cleaning, task-boundary filtering), ASR inference (Wav2Vec2, HuBERT, Whisper), reproducibility via saved config files.

**Evaluated on:** Two PsychosisBank corpora (TOPSY: 151 participants, Discourse: 149 participants).

### Table 3 — The Interesting Measures

Spearman correlations with TLI total score (discourse corpus, significant at alpha=0.05):

| Category | Features | Correlations |
|----------|----------|-------------|
| **Semantic Similarity** | sentCoherence variants (BertSum, CSE, Diff) — 16 features | -0.17 to -0.30 |
| **Perplexity** | contextmodel, topicmodel, avg_ppl — 3 features | 0.21 to 0.33 |
| **Speech Graph** | number_of_edges, degree_average, degree_std — 3 features | -0.17 to -0.25 |

### Assessment

**Overlap with batchalign3:** TRESTLE's preprocessing (audio resampling, CHAT parsing, ASR) duplicates what batchalign3 already does better (Rust-based, production-grade, multi-language).

**What TRESTLE adds that we don't have:** Downstream analysis features (semantic coherence, LLM perplexity, speech graphs) are NOT in batchalign3. These are research-oriented NLP feature extractors for clinical analysis.

### Deep-Dive Findings from Code (`~/harmonized-toolkit`)

- **CHAT parsing is regex string hacking** — `cha_processor.py` uses `re.sub` to extract timestamps and clean text. No AST, no tree-sitter. Fragile compared to batchalign3's Rust parser.
- **ASR is direct HuggingFace model loading** — `asr_pipeline.py` loads Wav2Vec2/HuBERT/Whisper via `transformers`. No server, no worker pool, no caching.
- **Feature extraction uses external microservers** — `FeatureExtractorClient` sends HTTP to `localhost:5003` for coherence/perplexity features. Actual feature computation models run separately (not in the repo).
- **Zero references to batchalign** anywhere in the codebase.
- **Polars-based** data pipeline — all intermediate data as `.parquet` files.

### Recommendation

- **Don't request TRESTLE's preprocessing** — batchalign3 does it vastly better
- **DO consider the downstream analysis features** (semantic coherence, LLM perplexity, speech graphs) as a complementary layer
- **Collaboration angle:** TRESTLE could use batchalign3 for preprocessing and keep its own downstream NLP feature extractors. Much more robust preprocessing backend for Trevor.
- **Ask Trevor:** What microserver runs at `localhost:5003`? That's where the interesting coherence/perplexity computation lives, and it's not in the repo.
- **Long-term:** Could these features become batchalign3 commands? Speech graphs and coherence are lightweight. Perplexity requires LLM inference.

**Brian's assessment:** "Trevor seems to be reinventing some of the wheel, but the additional measures could be interesting."

---

## Franklin-Only Items (Low Priority)

1. **Renaming CHILDES and Phon repos** — Already did the big repo split (16->24). Further renaming is minor.
2. **sync-all working?** — `sync-media --all` works as far as we know.
3. **Future of deploy** — Being replaced by Git push + GitHub Actions + pre-push hooks (already started). `deploy/scripts/` still used for fleet machines but being deprecated.
