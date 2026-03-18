# CA Overlap Marker Audit Results

**Status:** Current
**Last updated:** 2026-03-17

## Experiment B: Overlap Consistency Audit

### Method

Parsed all .cha files with CA overlap markers (⌈⌉⌊⌋) across data corpora. For each file:
1. Extracted overlap marker positions relative to alignable words
2. Identified cross-speaker pairs (⌈ on line N, ⌊ on line N+1, different speakers)
3. For files with timing bullets, checked temporal consistency (do paired overlaps actually overlap in time?)

Tool: `utr-experiment overlap-audit` (new subcommand)

### Results: ca-data (366 files with markers out of 6,606 total)

| Metric | Value |
|--------|-------|
| Files with markers | 366 |
| Fully paired (⌈⌉ + ⌊⌋ balanced) | 135 (37%) |
| Mixed (partial pairing) | 214 (58%) |
| Open only (⌈⌊ without ⌉⌋) | 17 (5%) |
| Cross-speaker pairs found | 12,081 |
| Timed pairs (both have bullets) | 2,451 |
| **Temporally consistent** | **2,420 (99%)** |

### Results: childes-data (76 files with markers out of ~52k total)

| Metric | Value |
|--------|-------|
| Files with markers | 76 |
| Fully paired | 30 (39%) |
| Mixed | 41 (54%) |
| Open only | 5 (7%) |
| Cross-speaker pairs | 20 |
| Timed pairs | 2 |
| Temporally consistent | 2 (100%) |

### Key Subcorpora (ca-data)

| Subcorpus | Files | Cross-pairs | Timed pairs | Temporal % |
|-----------|-------|-------------|-------------|-----------|
| SBCSAE | 60 | 8,985 | 2,427 | 99% |
| TaiwanMandarin | 27 | 2,655 | 0 | — |
| TaiwanHakka | 15 | 227 | 0 | — |
| CallFriend/eng-n | 23 | 88 | 0 | — |
| CallFriend/spa | 21 | 77 | 0 | — |
| SCoSE | 4 | 14 | 14 | 100% |
| GulfWar | 8 | 6 | 6 | 100% |

### Conclusions

1. **99% temporal consistency** confirms that CA overlap markers are highly reliable as timing constraints. The 31 inconsistent pairs (out of 2,451) are likely annotation errors or edge cases, not systematic problems.

2. **SBCSAE is the primary experiment corpus** — it has 2,427 timed cross-speaker pairs, making it the ideal test set for Experiment A (proportional onset estimation). It also has timing bullets on most utterances.

3. **TaiwanMandarin has rich overlap annotations but no timing** — 2,655 cross-speaker pairs. These files would benefit from Experiment D (%wor from overlap structure) but can't be used for temporal validation.

4. **The "mixed" category is larger than expected** (58% vs the plan's 30%). This is because many files use CA markers for onset position only (legitimate CA practice) rather than strictly balancing open/close markers. The proportional onset estimation (Experiment A) works with onset-only markers too.

5. **Experiment A should proceed**: the 99% temporal consistency validates that overlap marker positions can be trusted as alignment constraints. SBCSAE files are the recommended test set.
