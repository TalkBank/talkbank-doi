# Plan: Train Cantonese-Specific Stanza Model on Net

**Status:** Ready to execute
**Last updated:** 2026-03-23 13:30 EDT

## Why

| Component | Current (Stanza zh) | With Cantonese Model |
|-----------|-------------------|---------------------|
| POS tagging | ~50% (PyCantonese override → ~94%) | Should be >90% natively |
| Dependency parse | Broken on aspect markers, copula | Should handle Cantonese grammar |
| Lemmatization | Mandarin lemmas for Cantonese words | Cantonese-appropriate lemmas |

The PyCantonese POS override is a good short-term fix but doesn't help %gra.
A proper Cantonese Stanza model fixes everything.

## Training Data

**UD_Cantonese-HK treebank:**
- 1,004 sentences, 13,918 tokens
- Full UD annotation (POS, deps, morphology, lemma)
- Traditional Chinese characters
- Film subtitles + legislative proceedings (Hong Kong)
- GitHub: https://github.com/UniversalDependencies/UD_Cantonese-HK

## Machine: net (ssh macw@net)

- **Hardware:** Mac Studio M3 Ultra, 256 GB RAM
- **GPU:** Apple Silicon MPS (no NVIDIA)
- **Python:** System 3.9.6 — need to create training venv with 3.12
- **uv:** Available (0.10.12)
- **Stanza training:** Supports MPS via PyTorch

## Execution Plan

### Step 1: Set up training environment on net

```bash
ssh macw@net
mkdir -p ~/cantonese-stanza-training && cd ~/cantonese-stanza-training

# Create venv with uv
uv venv --python 3.12 .venv
source .venv/bin/activate
uv pip install stanza torch

# Clone treebank
git clone https://github.com/UniversalDependencies/UD_Cantonese-HK
```

### Step 2: Download pretrained word vectors

Stanza training uses pretrained word vectors for initialization. We need
Chinese character/word vectors:

```bash
# Download fasttext Chinese vectors (or use Stanza's built-in downloader)
python -c "
import stanza
stanza.download('zh-hans')  # downloads pretrained vectors
"
```

### Step 3: Prepare training data

```bash
# Convert UD treebank to Stanza format
python -m stanza.utils.datasets.prepare_pos_treebank UD_Cantonese-HK
python -m stanza.utils.datasets.prepare_lemma_treebank UD_Cantonese-HK
python -m stanza.utils.datasets.prepare_depparse_treebank UD_Cantonese-HK
```

### Step 4: Train models

```bash
# POS tagger (~30 min on M3 Ultra)
python -m stanza.utils.training.run_pos UD_Cantonese-HK \
    --wordvec_pretrain_file /path/to/zh_pretrain.pt \
    --device mps

# Lemmatizer (~15 min)
python -m stanza.utils.training.run_lemma UD_Cantonese-HK \
    --device mps

# Dependency parser (~45 min)
python -m stanza.utils.training.run_depparse UD_Cantonese-HK \
    --wordvec_pretrain_file /path/to/zh_pretrain.pt \
    --device mps
```

### Step 5: Evaluate

```bash
# Stanza evaluates automatically on the dev/test split
# Also run our custom tests:
python -c "
import stanza
# Load custom model
nlp = stanza.Pipeline(
    lang='yue',
    dir='/path/to/trained/models',
    processors='tokenize,pos,lemma,depparse',
    tokenize_pretokenized=True,
)
doc = nlp('佢哋 好 鍾意 食 嘢')
for sent in doc.sentences:
    for word in sent.words:
        print(f'{word.text}\t{word.upos}\t{word.deprel}\t{word.head}')
"
```

### Step 6: Integrate into batchalign3

If evaluation shows improvement over current pipeline:

1. Package trained model files
2. Add to batchalign3's model distribution (or host on HuggingFace)
3. Map `yue` → custom model instead of `zh`
4. Remove PyCantonese POS override (no longer needed for POS)
5. Keep PyCantonese for word segmentation and jyutping

## Risks

1. **Small treebank (1,004 sentences)** — may underfit. Mitigation: transfer
   learning from Chinese pretrained vectors.
2. **Domain mismatch** — treebank is formal (film + legislative), our use is
   informal (child speech, aphasia). May need domain adaptation.
3. **MPS training untested** — Stanza training on MPS may have issues.
   Fallback: CPU training (slower but guaranteed to work on 256 GB).
4. **Tokenizer training** — The treebank has its own tokenization. We use
   PyCantonese for tokenization. May need to skip tokenizer training and
   use `pretokenized=True` with the trained POS/dep/lemma models.

## Timeline Estimate

- Environment setup: 30 min
- Data preparation: 15 min
- Training (POS + lemma + depparse): ~90 min on MPS, ~4 hours on CPU
- Evaluation: 30 min
- Integration: 2-3 hours

**Total: ~1 day of focused work.**

## Decision for Franklin

1. **Go ahead with training?** This is a medium-effort task with high impact.
2. **Contact PolyU team?** They may have additional Cantonese annotated data
   (child speech) that would improve the model.
3. **Timeline?** Could do this week if approved.
