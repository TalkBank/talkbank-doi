# Rust Migration Benchmark Results

**Date:** 2026-02-11 19:25:01
**Machine:** Apple M4 Max, 64 GB RAM, 16 cores
**Align branch:** e16bc50 refactor: migrate DP alignment to Rust implementation and simplify pipeline architecture
**Main branch:** 614a02d bump
**Results dir:** /Users/chen/batchalign2/bench_results/rust_migration_20260211_184238/

## Notes

- align branch uses `--no-utr` (UTR engine lacks `process_chat_text()`)
- main branch runs full align pipeline (includes UTR via Document path)
- morphotag uses `--override-cache` on both branches (different flag position)
- utseg has no cache bypass on main branch (may be faster if cached)
- bench_data copied from repo to `/Users/chen/batchalign2/bench_results/rust_migration_20260211_184238/bench_data` to survive branch switches

## Benchmark Timing

| Benchmark | align avg | align runs | main avg | main runs | Delta |
|-----------|----------|-----------|---------|----------|-------|
| align_large_w1                           |     7.4s | [7.42, 7.4] |   404.3s | [488.97, 319.54] | +98.2% |
| align_medium_w1                          |     4.9s | [4.88, 4.9] |   154.1s | [161.95, 146.21] | +96.8% |
| align_medium_w2                          |     4.9s | [4.91, 4.84] |   161.4s | [230.28, 92.59] | +97.0% |
| align_medium_w4                          |     4.8s | [4.88, 4.78] |    43.0s | [42.99, 42.96] | +88.8% |
| align_small_w1                           |     3.6s | [3.63, 3.64, 3.6] |    90.0s | [59.25, 120.65, 90.03] | +96.0% |
| morphotag_large_w1                       |    12.4s | [12.38, 12.39, 12.5] |    18.2s | [17.93, 18.07, 18.49] | +31.6% |
| morphotag_large_w2                       |    10.6s | [10.59, 10.61] |    16.1s | [16.13, 16.17] | +34.4% |
| morphotag_large_w4                       |     9.0s | [9.02, 9.0] |    14.6s | [14.57, 14.57] | +38.2% |
| morphotag_medium_w1                      |     7.8s | [7.7, 7.82, 7.76] |    13.1s | [13.17, 13.16, 13.08] | +40.9% |
| morphotag_small_w1                       |     5.6s | [5.74, 5.61, 5.57] |    10.9s | [10.85, 10.9, 10.87] | +48.1% |
| utseg_medium_w1                          |     1.3s | [1.3, 1.31, 1.3] |    24.6s | [26.11, 23.87, 23.78] | +94.7% |
| utseg_small_w1                           |     1.3s | [1.32, 1.29, 1.29] |    12.9s | [15.66, 11.47, 11.56] | +89.9% |

## Correctness Comparison

- **align_small:** 168 lines of diff
- **morphotag_medium:** 136 lines of diff
- **morphotag_small:** 57 lines of diff

## Output Files

All outputs saved to `/Users/chen/batchalign2/bench_results/rust_migration_20260211_184238/`
- align/bench/align_large_w1.jsonl
- align/bench/align_medium_w1.jsonl
- align/bench/align_medium_w2.jsonl
- align/bench/align_medium_w4.jsonl
- align/bench/align_small_w1.jsonl
- align/bench/morphotag_large_w1.jsonl
- align/bench/morphotag_large_w2.jsonl
- align/bench/morphotag_large_w4.jsonl
- align/bench/morphotag_medium_w1.jsonl
- align/bench/morphotag_small_w1.jsonl
- align/bench/utseg_medium_w1.jsonl
- align/bench/utseg_small_w1.jsonl
- comparison/align_small_mor_gra.diff
- comparison/align_small.diff
- comparison/morphotag_medium_mor_gra.diff
- comparison/morphotag_medium.diff
- comparison/morphotag_small_mor_gra.diff
- comparison/morphotag_small.diff
- main/bench/align_large_w1.jsonl
- main/bench/align_medium_w1.jsonl
- main/bench/align_medium_w2.jsonl
- main/bench/align_medium_w4.jsonl
- main/bench/align_small_w1.jsonl
- main/bench/morphotag_large_w1.jsonl
- main/bench/morphotag_large_w2.jsonl
- main/bench/morphotag_large_w4.jsonl
- main/bench/morphotag_medium_w1.jsonl
- main/bench/morphotag_small_w1.jsonl
- main/bench/utseg_medium_w1.jsonl
- main/bench/utseg_small_w1.jsonl
