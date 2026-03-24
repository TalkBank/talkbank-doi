# Email: Reply to Thread — Unified Model + Paraformer Analysis

**Status:** Ready to send
**Last updated:** 2026-03-24 07:30 EDT
**Thread:** Re: tokenization
**To:** Brian, Angel, Houjun, Sebastian, Spencer, Spring, Wanlin

---

## Plain text for Gmail (copy below this line)

Brian, Angel, Houjun, Spencer, Spring, Wanlin,

Two updates.

UNIFIED CANTONESE MODEL

Following up on Houjun's suggestion to train a single Stanza model instead of the 3-tool pipeline: he was right, and we've done it.

We trained a Cantonese-specific Stanza pipeline (tokenizer + POS + depparse) using UD_Cantonese-HK (1,004 sentences from CityU HK) combined with HKCanCor (16,162 utterances of spoken Cantonese, 153K tokens). We also added the parallel Mandarin treebank (UD_Chinese-HK) to double the dependency training data, and are training a Cantonese character language model from Cantonese Wikipedia (76.6 million characters).

Results on held-out test data:

Tokenization (token F1):
- Trained Stanza: 90.3% on UD test (dev F1 reached 96.4%)
- PyCantonese dictionary: 77.3%

POS accuracy (UD test, 1,203 tokens):
- Trained Stanza: 93.4%
- PyCantonese: 73.1%

Dependency parsing (LAS on held-out 101 sentences):
- Trained Cantonese model: 67.7% (with parallel Mandarin data)
- Previous Mandarin model: 24%

Cross-domain testing on all 9 TalkBank Cantonese corpora (180 utterances): the trained model has 0% unknown-word rate across all corpora, vs 0.8-2.5% for PyCantonese. It handles spoken Cantonese vocabulary correctly (佢哋, 鍾意, 媽媽, 故事) while also making finer-grained tokenization splits that follow UD conventions.

We're working on integrating this into batchalign3 to replace the current 3-tool pipeline with this single trained model. The training pipeline and methodology are fully documented and reproducible.

PARAFORMER MANDARIN OUTPUT

Thank you Wanlin for the Paraformer samples! We analyzed the 5 raw transcripts (10,145 CJK tokens):

- Paraformer produces 100% per-character output — every Chinese character is a separate token
- This confirms that all 5 ASR engines (Whisper, Tencent, Aliyun, FunASR, Paraformer) produce per-character output for CJK languages
- --retokenize on morphotag is needed for all Mandarin and Cantonese ASR output, regardless of which engine is used

We also measured the Paraformer character error rate against the gold transcripts: overall CER is 5.5%, with T04 at 1.6% (cleanest) and T01 at 7.3%. Paraformer does not produce speaker diarization (all output as a single speaker).

The gold transcripts (manually corrected for character accuracy and speaker diarization) are also mostly per-character (98.4%), with only a few incidentally grouped words — they have not been word-segmented.

Spencer, when you test batchalign3, use `morphotag --retokenize` for word-level tokenization on Cantonese and Mandarin ASR output. Without --retokenize, the per-character tokens will be preserved as-is.

Franklin
