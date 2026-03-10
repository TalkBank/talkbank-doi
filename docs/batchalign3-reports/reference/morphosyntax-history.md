# Morphosyntax Historical Notes

Archived from the public `batchalign3` book on 2026-03-05.
The public reference page now focuses on current implementation. Durable BA2 to
BA3 behavior changes remain in the migration book; transient investigation notes
live here instead.

### The MWT/GRA Bug (Historical)

Files with MWT contractions (e.g., "I'll", "don't") produced E712 validation errors
after morphotagging.  The GRA builder had treated MWT ranges as single words, emitting one
GRA relation per MWT instead of one per component.  Fixed 2026-02-16 by switching to
chunk-based indexing.  All MWT-producing languages were affected: English (contractions),
French (elisions), Italian (articulated prepositions).

## 9. Resolved Divergences (Historical)

Investigation from 2026-02-15 comparing Python and Rust morphotag output.  All actionable
items have been fixed.

| Divergence | Python behavior | Rust behavior | Resolution |
|-----------|----------------|---------------|------------|
| Reflexive `-reflx` suffix | Present (`-reflx`) | Missing | **FIXED** (2026-02-15): Added `map_pronoun_suffixes()` in mapping.rs |
| Communicator POS | `c\|word` | `x\|word` | **FIXED** (2026-02-15): Added FormType-based POS override in lib.rs |
| L2/foreign words | `L2\|xxx` (loses word) | `x\|word` (preserves word) | **Kept Rust behavior**: preserving lexical content is better for analysis |
| %wor suffixes | `@s/@c` kept in %wor | Stripped in %wor | **Kept Rust behavior**: %wor represents spoken forms, not orthographic markers |
| ROOT head index | Self-referential (`3\|3\|ROOT`) | Standard (`3\|0\|ROOT`) | **Kept Rust behavior**: matches UD standard (adopted 2026-02-16) |
| MWT chunk indexing | Off-by-one in array indexing | HashMap-based | **Kept Rust behavior**: eliminated 87.5% circular dependency rate |
