# Paraformer Raw Output Analysis

**Status:** Current
**Last updated:** 2026-03-24 06:55 EDT

## Source

5 raw Paraformer Mandarin transcripts from Wanlin (PolyU/Angel Chan's team).
Sent via WeTransfer on 2026-03-24. Processed by Batchalign 0.8.1-post.18.

Files: T01-T04, T06 (NLM-Mandarin). Gold transcripts for T01-T04.

## Finding: Paraformer produces 100% per-character output

| Source | CJK tokens | Multi-char | Single-char |
|--------|-----------|-----------|-------------|
| Raw Paraformer | 10,145 | 0 (0.0%) | 10,145 (100.0%) |
| Gold transcript | 8,125 | 128 (1.6%) | 7,997 (98.4%) |

Every CJK token from Paraformer is a single character. This confirms that
ALL five ASR engines produce per-character output for CJK languages:

| Engine | Language | Per-character |
|--------|----------|--------------|
| Whisper | yue, cmn | Yes |
| Tencent Cloud | yue | Yes |
| Aliyun NLS | yue | Yes |
| FunASR/SenseVoice | yue | Yes |
| **Paraformer** | **cmn** | **Yes (confirmed)** |

## Implication

`--retokenize` on `morphotag` is needed for ALL CJK ASR output, regardless
of engine. The trained Cantonese Stanza tokenizer (F1=96.36) and PyCantonese
(F1=77.3) both provide word segmentation for this purpose.

## Gold Transcript Note

The gold transcripts have been manually corrected for character accuracy
and speaker diarization but NOT word-segmented. They are still 98.4%
single-character. A few multi-char words appear from manual editing
(所以, 尽量, 肚子, 干嘛) but this is incidental, not systematic.
