# Email: Draft — Brian-friendly Cantonese summary after reproducibility reruns

**Status:** Draft
**Last updated:** 2026-03-25 10:05 EDT
**To:** Brian MacWhinney
**Note:** Not sent. Internal, boss-friendly summary of the current Cantonese status after the clean rerun, charlm-only comparison, and charlm+BERT comparison.

---

## Plain text for Gmail (copy below this line)

Brian,

Quick internal summary of where the Cantonese work stands now that we have rerun the training more carefully.

Bottom line:

- we now have a reproducible training/evaluation lane for the unified Cantonese Stanza work;
- the parser story is materially better than it looked in the clean baseline alone;
- the tokenizer story is still mixed and needs caution.

What changed in the validation:

- We found a real bug in the original canonical rerun script: one earlier rerun trained the tokenizer but silently skipped POS and dependency training because of the Stanza wrapper entrypoints it used.
- We fixed that, reran the baseline cleanly on `net`, and then ran two follow-on comparisons:
  - charlm-only
  - charlm+BERT

Current reproducible held-out results:

- clean baseline:
  - tokenizer F1: 90.3% on UD held-out
  - tokenizer F1: 90.0% on held-out HKCanCor
  - POS: 93.0%
  - depparse LAS: 64.7%
- charlm-only:
  - POS: 93.9%
  - depparse LAS: 68.6%
- charlm+BERT:
  - POS: 94.7%
  - depparse LAS: 75.1%

What I think is now on solid ground:

- We do have reproducible evidence that the parser improves substantially once we add charlm, and improves further again with charlm+BERT.
- The current best reproducible stack is:
  - baseline tokenizer
  - charlm+BERT POS tagger
  - charlm+BERT dependency parser

What I still do not want to overclaim:

- The tokenizer is not uniformly better than PyCantonese. On held-out HKCanCor, PyCantonese still does better than the trained tokenizer.
- Our 9-corpus TalkBank comparisons are still proxy comparisons, not gold-standard accuracy evaluation.
- The improved parser results are strong on held-out treebank evaluation, but I would still like some targeted human spot checks before making broad external claims.

My recommendation:

- Internally, I think we can now say that the reproducibility cleanup succeeded and that the best current parser stack is substantially stronger than the clean baseline.
- Externally, I would keep the claim narrower: parser quality improved a lot under reproducible reruns, but tokenizer claims still need caution and cross-domain TalkBank claims should still be framed as preliminary.

If useful, I can next turn this into either:

- a shorter note suitable for sending to the external collaborators, or
- a packaging/integration memo for deciding whether to move the best current stack into batchalign3.

Franklin
