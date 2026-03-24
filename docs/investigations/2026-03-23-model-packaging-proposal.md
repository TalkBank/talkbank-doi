# Proposal: Cantonese Stanza Model Packaging and Distribution

**Status:** Draft for discussion
**Last updated:** 2026-03-23 22:58 EDT

## Context

We trained a Cantonese-specific Stanza pipeline (tokenizer + POS + depparse)
that significantly outperforms the current approach. The models need to be
distributed to batchalign3 users. This document evaluates packaging options
for Brian's review.

## Model Sizes

| Component | File | Size |
|-----------|------|------|
| Tokenizer | `yue_combined_tokenizer.pt` | 971 KB |
| POS tagger | `yue_combined_tagger.pt` | 17 MB |
| Depparse | `yue_hk_depparse.pt` | 101 MB |
| Pretrained vectors (shared) | `fasttext157.pt` | ~300 MB |
| **Total (models only)** | | **~119 MB** |
| **Total (with vectors)** | | **~419 MB** |

The pretrained vectors (`fasttext157.pt`) are shared with Stanza's Chinese
models and may already be cached on user machines if they've used Stanza
for Chinese before.

## Options

### Option A: HuggingFace Hub (Recommended)

Host models on HuggingFace under the TalkBank organization.

**How it works:**
- Upload model files to `huggingface.co/talkbank/stanza-yue-combined`
- batchalign3 downloads on first use (like Stanza's built-in models)
- Cached locally after first download
- Versioned: can push model updates without code changes

**Pros:**
- Standard in NLP community — Stanza itself uses HuggingFace for model hosting
- Free hosting for public models
- Versioned, auditable, citable (DOI possible)
- Users can browse model card, see training data provenance
- No PyPI package size bloat
- Can share with researchers outside TalkBank

**Cons:**
- Requires HuggingFace account for TalkBank org
- First-run download (~119 MB) needs internet
- One more external dependency (HuggingFace Hub)

**Implementation:**
1. Create `talkbank` org on HuggingFace (if not exists)
2. Upload models with model card documenting provenance
3. Add `huggingface_hub` to batchalign3 Python deps (tiny library)
4. In `_stanza_loading.py`: check cache, download if missing, load

### Option B: Stanza Resources Server

Register as an official Stanza language model via PR to `stanfordnlp/stanza-resources`.

**How it works:**
- PR to Stanza adding `yue` language support
- Models hosted on Stanza's own HuggingFace repos
- Users get Cantonese via `stanza.download('yue')`

**Pros:**
- Seamless Stanza integration — `stanza.download('yue')` just works
- No custom loading code needed
- Discoverable by all Stanza users worldwide
- Community maintenance and visibility

**Cons:**
- Requires Stanford NLP team approval (may take weeks/months)
- We lose control over model updates (need PRs for every change)
- Must meet Stanza's quality standards (may need more eval)
- Model naming/versioning follows Stanza conventions, not ours

**Implementation:**
1. Package models per Stanza conventions
2. Submit PR to `stanfordnlp/stanza-resources`
3. Wait for review/merge
4. Update batchalign3 to use `stanza.download('yue')`

### Option C: Bundled in batchalign3 PyPI Package

Include model files directly in the Python wheel.

**How it works:**
- Models shipped as package data in `batchalign3[cantonese-models]` extra
- Available immediately after `pip install` / `uv pip install`

**Pros:**
- Zero first-run download — works offline immediately
- No external hosting dependency
- Simple: models travel with the code

**Cons:**
- **Wheel size: ~119 MB** (currently ~5 MB). PyPI allows up to 100 MB per file
  by default, 200 MB with special request. Models may exceed this.
- Every `pip install` / `uv pip install` downloads 119 MB even for users who
  don't use Cantonese
- Model updates require new package release
- Not discoverable by non-batchalign users

**Implementation:**
1. Add model files to `batchalign/models/yue/`
2. Include in `pyproject.toml` package data
3. Load from package data path in `_stanza_loading.py`

### Option D: Separate PyPI Package

Publish models as a standalone package: `batchalign3-cantonese-models`.

**How it works:**
- `pip install batchalign3-cantonese-models`
- batchalign3 checks for this package and loads models from it

**Pros:**
- Opt-in: only Cantonese users download 119 MB
- Works offline after install
- Versioned independently from batchalign3

**Cons:**
- Another package to maintain and publish
- Users need to know to install it
- PyPI 100 MB limit may still be an issue
- Unusual pattern — confusing for users

### Option E: GitHub Release Assets

Attach model files as release assets on the batchalign3 GitHub repo.

**How it works:**
- Upload .pt files to a GitHub release (e.g., `v0.1.0-cantonese-models`)
- batchalign3 downloads from release URL on first use
- Cached locally after first download

**Pros:**
- No external service (everything on GitHub)
- Versioned with releases
- Free, no size limits on release assets (up to 2 GB per file)

**Cons:**
- GitHub Release download URLs are not as stable as HuggingFace
- Less discoverable by NLP community
- No model card / documentation convention
- Rate-limited for unauthenticated downloads

## Recommendation

**Option A (HuggingFace) as primary, with Option B (Stanza upstream) as
long-term goal.**

Rationale:
1. HuggingFace is the standard for NLP model hosting — researchers expect it
2. Zero cost, unlimited storage for public models
3. Model cards document provenance (training data, eval numbers, license)
4. We keep control over updates (no PR approval needed)
5. Stanza upstream submission should happen in parallel — it benefits the
   wider NLP community but shouldn't block our deployment

For the fleet machines (net, bilbo, etc.), models can be pre-cached during
deploy via `deploy/scripts/deploy_batchalign3.sh`.

## Succession Considerations

Per the succession mandate: a professor inheriting TalkBank must be able to
update these models. The documentation must include:

1. **How to retrain** — `cantonese-unified-training/` project with all scripts
2. **How to upload** — HuggingFace CLI commands in README
3. **How to update batchalign3** — which config to change for new model version
4. **Where training data came from** — METHODOLOGY.md with full provenance

HuggingFace accounts can be transferred. GitHub release assets are tied to
the repo (which transfers with the org). Both are succession-safe.

## License

- UD_Cantonese-HK: CC BY-SA 4.0
- HKCanCor: research use (check PyCantonese license)
- Trained models: derivative work — should be CC BY-SA 4.0 to match UD

Need to verify HKCanCor license terms before public distribution.
