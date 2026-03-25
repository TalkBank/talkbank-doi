# Email: Draft — cautious external Cantonese update after reruns

**Status:** Draft
**Last updated:** 2026-03-25 09:13 EDT
**Thread:** Re: tokenization
**To:** Brian, Angel, Houjun, Spencer, Spring, Wanlin
**Note:** Not sent. This draft is intended as a narrower, corrected external update after the completed reproducibility reruns.

---

## Plain text for Gmail (copy below this line)

Brian, Angel, Houjun, Spencer, Spring, Wanlin,

I want to send a narrower update on the Cantonese model work now that we have rerun the training more carefully.

We moved quickly in the earlier round of experiments, and I think some of my earlier wording was stronger than it should have been. We have now completed cleaner reruns from a Git-tracked reproducibility lane and have a better read on what is solid and what still needs caution.

What now looks solid:

- The current shipped `batchalign3 morphotag --retokenize` path for Cantonese and Mandarin is still the right practical advice for ASR output.
- We completed a clean reproducible Cantonese baseline rerun on `net`.
- We then completed two follow-on comparisons on the same reproducibility lane:
  - charlm-only
  - charlm+BERT

Current held-out results:

- clean baseline:
  - tokenizer F1: 90.3% on UD held-out
  - tokenizer F1: 90.1% on held-out HKCanCor
  - POS: 93.0%
  - depparse LAS: 64.7%
- charlm-only:
  - POS: 93.9%
  - depparse LAS: 68.6%
- charlm+BERT:
  - POS: 94.7%
  - depparse LAS: 75.1%

What that means:

- The parser story is now stronger than it looked in the clean baseline alone.
- The current best reproducible stack is:
  - baseline tokenizer
  - charlm+BERT POS tagger
  - charlm+BERT dependency parser

Important cautions:

- The tokenizer story is still mixed. On held-out HKCanCor, PyCantonese still outperforms the trained tokenizer, so I do not want to claim that the trained tokenizer is uniformly better.
- Our 9-corpus TalkBank comparisons are still proxy comparisons, not gold-standard accuracy evaluation.
- The improved parser results are encouraging on held-out treebank evaluation, but I would still like targeted spot checks on real TalkBank material before making broader quality claims.

So my current view is:

- we have a much better parser than we had in the clean baseline;
- we do not yet have a basis for broad tokenizer victory claims;
- and we should separate "promising reproducible held-out results" from "ready to claim broad production quality."

For now, the practical testing advice remains the same:

- if you are using batchalign3 on Cantonese or Mandarin ASR output and want word-level morphosyntax, use `morphotag --retokenize`;
- treat the currently shipped pipeline and the separate trained unified model as two different stages of work until we decide on packaging and integration.

I wanted to correct the tone before making stronger external claims. We are in a much better position now than we were a day ago, but I think a narrower summary is the honest one.

Franklin
