# Public Doc History Extracts (2026-03-09)

This note preserves historical/provenance-heavy material removed from active
public `batchalign3` docs during the March 9, 2026 release-doc audit.

The public replacements keep current behavior and stable contracts only. This
archive keeps the implementation-history context that was judged too detailed,
time-bound, or provenance-specific for the public book.

## 1. Cantonese normalization history

Source public page:
- `batchalign3/book/src/reference/language-specific-processing.md`

Removed public narrative:
- a branch-by-branch explanation of why Cantonese normalization disappeared in
  one later Python rewrite line
- maintainer-oriented attribution and timeline detail about who introduced and
  later removed the old Python `OpenCC('s2hk')` path

Public replacement:
- the public page now states only the durable contract: current `batchalign3`
  implements Cantonese normalization once in Rust core and applies it across
  ASR engines.

## 2. `%gra` leniency provenance counts

Source public page:
- `batchalign3/book/src/reference/gra-format.md`

Removed public detail:
- exact corpus-audit counts used during February 2026 validation passes,
  including the point-in-time totals that motivated parser-side warning
  severity for malformed `%gra`

Reason for extraction:
- the public page should describe the current parser/generator contract, not
  depend on point-in-time corpus totals unless those totals are reproducibly
  regenerated and maintained.

Public replacement:
- the public page now keeps the durable rule: parser intake remains lenient for
  historical malformed corpora, while newly generated `%gra` is validated
  strictly before writeback.

## 3. Algorithm review follow-up list

Source public page:
- `batchalign3/book/src/reference/algorithm-audit-2026-03-07.md`

Removed public detail:
- the end-of-page "resolution summary" list that enumerated the March 2026
  code follow-ups for M4-M9 as if they were part of the standing public
  contract

Reason for extraction:
- the public page remains useful as a dated design-review snapshot, but the
  fine-grained follow-up checklist is maintainership history rather than active
  user or integrator guidance.

Historical follow-up items preserved here:
- M4: consolidate repeated lemma-cleaning replacements
- M5: avoid repeated lowercasing in MWT rule matching
- M6: collapse sequential translation replacements into a single-pass path
- M7: reduce forced-alignment monotonicity propagation from O(w^2) to O(w)
- M8: replace the old Cantonese sequential replacement path with the Rust
  Aho-Corasick table
- M9: route Aliyun credential validation through the shared config helper

## 4. Morphotag migration audit provenance

Source public page:
- `batchalign3/book/src/reference/morphotag-migration-audit.md`

Removed public detail:
- maintainer-local checkout path (`~/batchalign2-master/...`)
- exact test-count bookkeeping and "added on 2026-03-07" notes

Public replacement:
- the public page now keeps the migration evidence, current divergence notes,
  and remaining test gaps, but avoids local-path provenance and brittle
  date-stamped test inventories.
