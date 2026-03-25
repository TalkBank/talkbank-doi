# Email: Draft — short Brian note on what was overstated

**Status:** Draft
**Last updated:** 2026-03-25 09:13 EDT
**To:** Brian MacWhinney
**Note:** Not sent. Short internal note focused on what earlier external claims got wrong, what still holds up, and what the current reruns actually support.

---

## Plain text for Gmail (copy below this line)

Brian,

Very short version: I do think some of my earlier external wording was too strong, but not all of it was wrong.

What was most overstated:

- I said the training pipeline/methodology were fully documented and reproducible. That was too strong. We later found a real bug in the canonical rerun script: one rerun silently skipped POS and dependency training because of the Stanza wrapper entrypoints it used.
- I presented the 67.7 LAS parser number too confidently. That number came from a historical parallel-treebank prototype, not from the later clean reproducible baseline rerun.
- I also let the 9-corpus cross-domain results sound more like accuracy evidence than they really were. Those comparisons are still proxy comparisons, not gold-standard evaluation.

What still seems directionally right:

- The shipped batchalign3 work itself was real: `morphotag --retokenize` exists, the main retokenize bugs were real and got fixed, and the PyCantonese POS override was a meaningful improvement over using the old Mandarin Stanza model.
- The unified Cantonese parser direction was also real. In fact, after the cleaner reruns, the best parser result is stronger than the earlier prototype story:
  - clean baseline LAS: 64.7
  - charlm-only LAS: 68.6
  - charlm+BERT LAS: 75.1

What still needs caution:

- The tokenizer is not uniformly better than PyCantonese. On held-out HKCanCor, PyCantonese still wins.
- The best parser numbers are on held-out treebank evaluation, not yet on human-reviewed TalkBank gold data.

So my read is:

- the product-side claims were mostly fine;
- the reproducibility and evaluation framing was where I overreached;
- the good news is that the fully rerun parser story is now actually stronger than the earlier prototype claim, while the tokenizer story is weaker and needs more care.

I drafted a cautious external-facing correction/update, but I would rather discuss it with you before sending anything.

Franklin
