# %wor Tier Python Bug History

Archived from the public `batchalign3` book on 2026-03-05.
Migration-relevant consequences remain in the public migration book. The deeper
forensic analysis of the older Python bug lives here instead.

## Historical: Python %wor Bug

Corpus-wide validation of 99,208 CHAT files revealed **3,735 files (3.8%) with
incorrect %wor tiers** containing 22,908 individual errors -- predominantly
21,935 extra words that should not be present.

**Root cause**: Python's lexer `decode()` method flattens nested bracket groups
by blindly overriding every inner token's type to match the outer group type.
When nonwords (`&~gaga`), phonological fragments (`&+fr`), or untranscribed
tokens (`xxx`, `yyy`, `www`) appear inside retrace groups (`<...> [/]`), their
type is overwritten from `ANNOT` to `RETRACE`, causing the `phonated_words`
filter to include them in %wor.  The same token standalone is correctly excluded
-- the bug is context-dependent and purely a flaw in the type-propagation logic.

**88.9% of the extra words were 3 characters or fewer** -- consistent with
nonword fragments (`&~` forms where the speaker started but did not complete a
word).  These are the hallmark of the `decode()` bug: fragments that leaked into
%wor solely because they happened to be inside retrace groups.

**Rust fix**: The Rust implementation has no type-flattening step.  The AST
walker descends into retrace groups but calls `word_is_alignable()` on each word
individually.  A `&~nonword` inside a retrace group is still a nonword -- its
category is an intrinsic property set by the parser, unchanged by group
membership.  27 tests in `test_wor_alignability.py` verify correctness across
all combinations of word type and context, including the specific cases that
triggered the Python bug.

Affected files span 12 TalkBank collections (CHILDES, AphasiaBank, CABank, etc.)
and require re-alignment with the Rust backend to fix their %wor tiers.
