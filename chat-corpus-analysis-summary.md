# CHAT Corpus Analysis — Empirical Token Classification

**Date:** 2026-03-24 00:35 EDT
**Tool:** `tree-sitter-grammar-utils corpus-analyze`
**Corpus:** `~/talkbank/data/*-data/` — 99,907 CHAT files, 254 MB
**Grammar:** tree-sitter-chat (TalkBank CHAT format)

## Overview

| Metric | Value |
|---|---|
| Files parsed | 99,907 |
| Files with errors | 378 (0.38%) |
| Files failed | 0 |
| Total tokens | 2,763,062,298 (~2.76 billion) |
| ERROR nodes | 3,806 |
| MISSING nodes | 0 |
| Unique token texts | 90,496 |

## Key Findings

### 1. Colon / Lengthening Shadow (confirmed empirically)

The static lint warned that `lengthening` (prec 5, `:{1,}`) shadows `colon` (prec 0, `:`). The corpus confirms this:

```
":" (152,222,237 occurrences)
  colon         151,807,993 (99.7%)  — in: header_sep, main_tier, separator, tier_sep
  lengthening       414,244 ( 0.3%)  — in: ERROR, word_body
```

414,244 occurrences of `:` are parsed as `lengthening` inside `word_body`, and these are associated with ERROR nodes. The shadow is real and causes parse errors in Danish conversation analysis data (`samtale-data/Video/`).

### 2. Zero Classification

```
"0" (42,435,084 occurrences)
  gra_head       22,777,742 (53.7%)  — in: gra_relation
  zero           19,619,731 (46.2%)  — in: nonword, standalone_word
  text_segment       37,199 ( 0.1%)  — in: text_with_bullets
```

`0` is parsed as `gra_head` 54% of the time (in %gra dependency tiers) and `zero` 46% of the time (in main tiers). The 37,199 `text_segment` classifications suggest it's sometimes absorbed into dependent tier text. No ERROR association — this appears correct.

### 3. Pipe Classification

```
"|" (356,156,442 occurrences)
  pipe          356,156,407 (100.0%)  — in: gra_relation, id_contents, mor_word
  text_segment           31 ( 0.0%)  — in: text_with_bullets
  annotation_content      4 ( 0.0%)  — in: explanation_annotation
```

35 occurrences of `|` parsed as `text_segment` or `annotation_content` instead of `pipe`. These are likely file-level bugs (bare `|` in text where it shouldn't appear).

### 4. Error Distribution

Only 378 files (0.38%) have any parse errors. The 3,806 ERROR nodes are concentrated in:
- **Danish conversation analysis** (`samtale-data/Video/`) — CA-specific notation not fully supported
- **Greek morphology** (`childes-other-data/Other/Greek/`) — unusual `%mor` patterns
- A handful of scattered edge cases

Zero MISSING nodes — the grammar's error recovery never needs to insert missing tokens.

## Files with Errors

The 378 error files (0.38% error rate across 99,907 files) suggest the grammar is highly robust. The errors are concentrated in specific language/dataset combinations that use CA (Conversation Analysis) notation or unusual Unicode patterns.

## Methodology

The analysis was performed by `tree-sitter-grammar-utils corpus-analyze` using:
- Rayon parallel parsing with a shared `Language` instance
- Per-thread `Parser` instances (tree-sitter `Parser` is not `Send`)
- Recursive CST walk collecting every leaf node's text, kind, parent, and error state
- Aggregation into per-text-string frequency tables

Runtime: ~3 minutes for 99,907 files on Apple Silicon.

## Full Report

The complete report (372,612 lines, all 90,496 unique token texts) is available by running:

```bash
tree-sitter-grammar-utils corpus-analyze \
  --grammar-dir ~/talkbank/talkbank-tools/grammar \
  --corpus ~/talkbank/data \
  --ext cha \
  --min-count 100 \
  -o chat-corpus-full-report.txt
```

---
Last updated: 2026-03-24 00:35 EDT
