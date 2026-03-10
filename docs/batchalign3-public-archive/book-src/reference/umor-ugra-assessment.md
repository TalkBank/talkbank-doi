# %umor / %ugra Tier Assessment

**Date:** 2026-02-17

---

## What are %umor and %ugra?

`%umor` and `%ugra` are non-standard dependent tiers that are functionally
identical to `%mor` and `%gra`. The "u" prefix stands for **UD (Universal
Dependencies)** — they indicate that the morphosyntactic annotation was generated
using UD conventions rather than the traditional CHAT/CLAN morphology system.

The distinction was introduced by Houjun in commit `9b741c1` (Feb 2024):

> uses umor intsead of mor for CLI

The purpose was to let CLAN tools distinguish between:
- `%mor` / `%gra` — human-written or CLAN-generated morphosyntax (traditional
  tagset)
- `%umor` / `%ugra` — batchalign-generated morphosyntax (UD tagset mapped to
  CHAT conventions)

### How it works in Python master

When `special_mor_=True`, the Python serializer writes `%umor` and `%ugra`
instead of `%mor` and `%gra`. The parser has a corresponding flag to read
either variant.

The second commit (`dc31c9d`, May 2024) refined the behavior:
- `align` and `transcribe` stopped using `special_mor_` (reverted to `%mor`)
- `morphotag` adopted conditional logic: if the input file already has `%mor`
  tiers, write `%umor`/`%ugra`; otherwise write `%mor`/`%gra`

The intent: when batchalign morphotags a file that already has hand-written
`%mor`, the automated output goes to `%umor`/`%ugra` to avoid overwriting the
human annotations. When there's no existing `%mor`, batchalign writes to
`%mor`/`%gra` directly.

### How it works on the align branch

The align branch detects `%umor`/`%ugra` in two places:
1. `extract_metadata()` in Rust (`lib.rs:3117-3126`) — does a raw string check
   for `"%umor:"` or `"%ugra:"` because the tree-sitter grammar treats them as
   `UserDefined` tiers, not structured `%mor` tiers.
2. `has_special_mor()` in Python (`chat_text.py:77-80`) — calls
   `extract_metadata()` and reads the flag.

The flag flows: `load_chat_text()` → `_special_mor` kwarg → `ProcessingContext.has_special_mor`.

**But no engine on the align branch reads `has_special_mor`.** The Rust
serializer always writes `%mor` and `%gra`. The flag is detected and stored but
never consumed — it is dead code.

### Corpus presence

A search of the local data repos found exactly **1 file** with `%umor`/`%ugra`
tiers:

```
~/data/childes-data/Eng-NA/NewmanRatner/Interviews/24/6630TM.cha
```

Example from that file:
```
%umor: pron|I-Prs-Nom-S1~aux|be-Fin-Ind-Pres-S1 adv|just verb|go-Part-Pres-S ...
%ugra: 1|4|NSUBJ 2|4|AUX 3|4|ADVMOD 4|11|ROOT ...
```

Note the suffix format: `I-Prs-Nom-S1` uses dashes, not the `&` ampersand
convention. This is an older batchalign output format, further indicating that
`%umor`/`%ugra` were a transitional experiment.

---

## What the boss proposes

Brian wants to rename all `%umor`/`%ugra` tiers back to `%mor`/`%gra` across
the corpus.

---

## Consequences of removing %umor/%ugra

### For the corpus data

- Only 1 known file is affected. The rename is trivial: `sed` or a script to
  replace `%umor:` → `%mor:` and `%ugra:` → `%gra:`.
- If there are files on the production server that aren't cloned locally, a server-side scan would
  be needed. The rename is still trivial.

### For batchalign-next (align branch)

The removal simplifies things. Changes needed:

1. **`chat_text.py`**: Remove `has_special_mor()` function entirely.
2. **`file_io.py:195-197`**: Remove the `has_special_mor()` call and `_special_mor`
   kwarg propagation (currently does a full Rust parse for a dead flag).
3. **`context.py:45-46`**: Remove `has_special_mor` field from `ProcessingContext`.
4. **`base.py:39`**: Remove `has_special_mor` from `_get_ctx()`.
5. **`pipeline.py:145`**: Remove `has_special_mor` from `process_chat_text()`.
6. **`lib.rs:676-694`**: Remove the `has_special_mor` scan in
   `extract_metadata_from_chat_file_pure()`.
7. **`lib.rs:3117-3127`**: Remove the raw string override hack in
   `extract_metadata()`.
8. **Tests**: Remove `TestHasSpecialMor`, `test_special_mor_forwarded`,
   `test_morphotag_detects_special_mor`, and `SPECIAL_MOR_CHAT` constant.

None of these changes affect behavior — the flag is already dead code.

### For batchalign (Python master)

If `%umor`/`%ugra` are renamed to `%mor`/`%gra` in the data, the `special_mor_`
codepath in Python master becomes permanently unreachable. It can be removed, but
since only Houjun modifies master, that's his call.

### For the tree-sitter grammar

The grammar has dedicated `umor_dependent_tier` and `ugra_dependent_tier` rules
(visible in `grammar.json` / `parser.c`). These parse `%umor` and `%ugra` as
structured tiers with the same internal grammar as `%mor` and `%gra`. They can
be removed from the grammar if the tiers no longer exist in data, or left as
harmless dead rules.

### For CLAN tools

CLAN recognizes `%umor` and `%ugra` as valid tier prefixes (they appear in the
XML schema at `talkbank.xsd:2996,3021`). Once renamed to `%mor`/`%gra`, CLAN
will process them normally. No CLAN changes needed.

---

## Recommendation

Remove `%umor`/`%ugra` support. The distinction was:
- Introduced as a transitional measure (Feb 2024)
- Partially reverted 3 months later (May 2024)
- Never widely adopted (1 file in the corpus)
- Already dead code on the align branch

The cleanup is low-risk and removes a full-parse call from the morphotag loading
path (see `docs/string-check-optimizations.md`).

---

## References

- `9b741c1` — "uses umor instead of mor for CLI" (Houjun, Feb 2024)
- `dc31c9d` — "implements minute umor implementation changes" (Houjun, May 2024)
- `lib.rs:676-694` — Rust special_mor detection
- `lib.rs:3117-3127` — raw string fallback hack
- `chat_text.py:77-80` — Python `has_special_mor()` (dead on align)
- `context.py:45-46` — ProcessingContext field (never read by any engine)
