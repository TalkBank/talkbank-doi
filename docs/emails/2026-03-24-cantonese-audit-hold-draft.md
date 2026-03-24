# Email: Draft — narrower Cantonese status update after audit

**Status:** Draft
**Last updated:** 2026-03-24 18:01 EDT
**Thread:** Re: tokenization
**To:** Brian, Angel, Houjun, Spencer, Spring, Wanlin
**Note:** Not sent. This draft intentionally narrows claims while validation catches up with the recent prototyping work.

---

## Plain text for Gmail (copy below this line)

Brian, Angel, Houjun, Spencer, Spring, Wanlin,

I want to pause before sending another substantive progress update. We moved very quickly over the last two days, and I think the right next step is to tighten the validation before making stronger claims.

So rather than add more headline results right now, I want to summarize only what I think is on solid ground and what still needs work.

CURRENT BATCHALIGN3 STATUS

- `morphotag --retokenize` is implemented in the current batchalign3 pipeline for CJK languages.
- For Cantonese, retokenize currently uses PyCantonese word segmentation.
- For Mandarin, retokenize currently uses Stanza's Chinese tokenizer.
- We also added a PyCantonese POS override for Cantonese in the current pipeline.
- Several real retokenize bugs involving retraces, pre-codes, parentheses, and mixed-script handling were fixed, and the MOST corpus now runs where it previously failed.

WHAT I THINK IS WELL SUPPORTED

- The current Cantonese POS override is materially better than the previous Mandarin-based POS behavior on the tested Cantonese vocabulary.
- The Paraformer Mandarin samples Wanlin sent are per-character on those supplied files and do not include speaker diarization.
- The separate unified Cantonese Stanza training work is promising on held-out training-domain evaluation.
- We now have a cleaner Git-based reproducibility lane for that work, separate from the older ad hoc Bilbo directory.
- We now also have a completed clean baseline rerun on the same Git-based lane, so the current numbers no longer depend only on historical prototype artifacts.

WHAT I DO NOT WANT TO CLAIM STRONGLY YET

- The unified Cantonese Stanza model is not yet integrated into batchalign3.
- Its strongest support is still on held-out UD/HKCanCor-style evaluation, not on full production-style TalkBank validation.
- Cross-domain TalkBank evaluation still needs stronger validation, especially for dependency parsing and for expert-reviewed spot checks.
- I also want to avoid broad engine-wide or corpus-wide quality claims from limited samples or proxy metrics.
- Additional model-comparison work remains unfinished, including charlm/BERT retraining.
- Most importantly, we found a real reproducibility bug in our canonical baseline training script: an earlier clean rerun trained the tokenizer but silently skipped POS and dependency training because of the Stanza wrapper entrypoints it was using. That means some earlier language about the training pipeline being fully reproducible was too strong.
- The completed clean baseline rerun also showed some meaningful caveats that need to be reflected honestly:
  - UD held-out tokenization and POS are still encouraging
  - held-out depparse in the clean baseline is 64.7 LAS, not the historical 67.7 parallel-treebank prototype result
  - HKCanCor held-out tokenization is currently weaker than PyCantonese

WHAT WE ARE DOING NEXT

- Tighten the validation and reproducibility of the current experiments.
- Freeze a cleaner audit trail for the training artifacts and model results.
- Use the completed clean baseline rerun as the new reference point for any future reporting.
- Integrate the unified model into batchalign3 only after we have clearer evaluation on TalkBank data.
- Build a better TalkBank spot-check set so future reports can distinguish coverage/agreement from actual accuracy.
- Keep the charlm/BERT retraining phase separate from the baseline interpretation so we do not mix prototype and reproducible claims again.

So for the moment, the practical advice for testing remains:

- if you are running batchalign3 on Cantonese or Mandarin ASR output and want word-level morphosyntax, use `morphotag --retokenize`;
- if you want the most cautious reading of the current state, treat the existing shipped pipeline and the separate trained unified model as two different stages of work.

As of now, the corrected clean baseline rerun and its held-out evaluation are complete, and the backward charlm is also complete. I would still rather send the next technical update only after those rerun-backed corrections have been incorporated cleanly into our reporting.

Thanks for your patience.

Franklin
