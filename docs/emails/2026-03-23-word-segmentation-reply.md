# Email: Word Segmentation + Cantonese Morphosyntax Update

**Status:** Ready to send
**Last updated:** 2026-03-23 19:45 EDT
**Thread:** Re: tokenization
**To:** Brian, Angel, Houjun, Sebastian, Spencer, Spring

---

## Plain text for Gmail (copy below this line)

Brian, Angel, Houjun, Sebastian, Spencer, Spring,

Status update on word segmentation and morphosyntax for Cantonese and Mandarin in batchalign3.

WORD SEGMENTATION

The --retokenize flag on morphotag now does word segmentation for CJK languages before POS tagging:

- Cantonese (yue): uses PyCantonese to group per-character tokens into words
- Mandarin (cmn/zho): uses Stanza's Chinese tokenizer

Example: batchalign3 morphotag --retokenize corpus/ -o output/ --lang yue

We verified that all four Cantonese ASR engines (Whisper, Tencent, Aliyun, FunASR) produce per-character tokens, so --retokenize is needed regardless of which engine you use. We tested word segmentation against all 9 Cantonese corpora in TalkBank (over 737,000 utterances across LeeWongLeung, CHCC, HKU, MAIN, MOST, EACMC, GlobalTales, WCT, and Aphasia HKU). Results are consistent: 84-90% of multi-character words correctly grouped, with 98-100% vocabulary coverage.

CANTONESE MORPHOSYNTAX

We found that the Stanza model previously used for Cantonese was trained on Mandarin data and performed poorly on Cantonese vocabulary. We made two improvements:

1. We added a PyCantonese POS correction that applies to all Cantonese morphotag output. In manual linguistic judgment of disagreements between existing corpus annotations and PyCantonese, PyCantonese was never clearly wrong; the existing annotations had systematic errors from the Mandarin model.

2. We trained a Cantonese-specific Stanza model using the UD_Cantonese-HK treebank (1,004 annotated sentences from City University of Hong Kong). Combined results on the held-out UD test set (1,484 tokens):

Before (Mandarin model):
- POS: 63%, Dependency parse (LAS): 24%

After (Cantonese model + PyCantonese POS):
- POS: 95%, Dependency parse (LAS): 65%

BUG FIXES

We found and fixed several bugs in the retokenize pipeline that prevented --retokenize from working on real corpus data with retraces, language pre-codes, and mixed-script content. The MOST corpus (166,848 utterances, previously with zero morphosyntax annotation) now processes successfully with --retokenize.

WHAT'S NEXT

We are continuing to work on:
- Deploying the trained Cantonese model into batchalign3
- Investigating HKCanCor (150K tokens of annotated Cantonese) as additional training data
- Proposing improvements to the talkbank-tools data model for better retrace handling

A request: if anyone has sample raw Paraformer transcript output (before any post-processing), that would help us verify what Paraformer produces for Mandarin word boundaries.

The code is on GitHub in the batchalign3 repo.

Franklin
