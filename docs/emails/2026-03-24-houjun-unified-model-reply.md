# Email: Reply to Houjun — Unified Cantonese Stanza Model

**Status:** Draft for tomorrow
**Last updated:** 2026-03-23 22:50 EDT
**Thread:** Re: tokenization
**To:** Houjun

---

## Plain text for Gmail (copy below this line)

Houjun,

You're right — we tested it and the unified model works significantly better.

We trained a Cantonese-specific Stanza pipeline (tokenizer + POS + depparse) using UD_Cantonese-HK (1,004 sentences from CityU HK) combined with HKCanCor (16,162 utterances of spoken Cantonese, 153K tokens). Results on held-out test data:

Tokenization (token F1 on UD test, 101 sentences):
- Trained Stanza: 90.3%
- PyCantonese dictionary: 77.3%

POS accuracy (UD test, 1,203 tokens):
- Trained Stanza: 93.4%
- PyCantonese: 73.1%

The trained tokenizer also matches PyCantonese on common spoken Cantonese (佢哋, 鍾意, 媽媽, 故事 all segmented correctly) and makes finer-grained splits that follow UD conventions (e.g., verb + resultative complement as separate tokens).

Dependency parsing (LAS on held-out 101 sentences):
- Trained Cantonese model: 64.7%
- Previous Mandarin model: 24%

Depparse is bottlenecked by the UD_Cantonese-HK treebank size (only 803 training sentences with dependency annotations). HKCanCor has POS but no dependencies. More annotated data would help here.

We're working on integrating this into batchalign3 to replace the current 3-tool pipeline (PyCantonese segment → Stanza Mandarin model → PyCantonese POS override) with this single trained model.

Franklin
