#!/usr/bin/env python3
"""Count how many CHAT files contain each dependent tier type.

Scans a directory of .cha files and reports tier frequency.

Usage:
    python3 scripts/analysis/count_tier_coverage.py ~/data/phon-data
    python3 scripts/analysis/count_tier_coverage.py ~/data/childes-data --top 20
"""

import os
import argparse
from collections import Counter


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("data_dir", help="Root directory of .cha files")
    parser.add_argument(
        "--top", type=int, default=0, help="Show only top N tiers (0=all)"
    )
    args = parser.parse_args()

    tier_counts: Counter[str] = Counter()
    file_count = 0

    for dirpath, _, filenames in os.walk(args.data_dir):
        if "0phon" in dirpath:
            continue
        for fn in filenames:
            if not fn.endswith(".cha"):
                continue
            file_count += 1
            fpath = os.path.join(dirpath, fn)
            tiers_in_file: set[str] = set()
            try:
                with open(fpath, "r", encoding="utf-8", errors="replace") as f:
                    for line in f:
                        if line.startswith("%") and ":" in line:
                            tier_name = line.split(":")[0]
                            tiers_in_file.add(tier_name)
            except OSError:
                continue
            for t in tiers_in_file:
                tier_counts[t] += 1

    print(f"Scanned {file_count} .cha files\n")
    print(f"{'Tier':<20} {'Files':>8} {'%':>7}")
    print("-" * 37)

    items = tier_counts.most_common(args.top if args.top > 0 else None)
    for tier, count in items:
        pct = 100.0 * count / file_count if file_count > 0 else 0
        print(f"{tier:<20} {count:>8} {pct:>6.1f}%")


if __name__ == "__main__":
    main()
