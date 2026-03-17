#!/usr/bin/env python3
"""Analyze timing quality of +< backchannel utterances across experiment results.

Compares bullet positions between strategies and (where available) against
ground truth. Reports per-file and aggregate statistics.

Usage:
    python3 scripts/analyze-timing-quality.py

Reads results from results/ directories. Outputs a timing quality report.
"""

import re
import os
import sys
from pathlib import Path
from dataclasses import dataclass, field
from collections import defaultdict

BULLET_RE = re.compile(r'\x15(\d+)_(\d+)\x15')
OVERLAP_RE = re.compile(r'^\*(\w+):\t\+<')
UTT_RE = re.compile(r'^\*(\w+):\t(.+?)$', re.MULTILINE)


@dataclass
class UttInfo:
    speaker: str
    text: str
    start_ms: int | None
    end_ms: int | None
    has_overlap_linker: bool
    line_num: int


def parse_utterances(path: Path) -> list[UttInfo]:
    """Extract utterances with timing from a CHAT file."""
    utts = []
    with open(path) as f:
        for line_num, line in enumerate(f, 1):
            line = line.rstrip()
            m = re.match(r'^\*(\w+):\t(.+)$', line)
            if not m:
                continue
            speaker = m.group(1)
            text = m.group(2)
            bm = BULLET_RE.search(text)
            start = int(bm.group(1)) if bm else None
            end = int(bm.group(2)) if bm else None
            has_overlap = text.startswith('+<')
            # Strip bullet from text for comparison
            clean_text = BULLET_RE.sub('', text).strip()
            utts.append(UttInfo(speaker, clean_text, start, end, has_overlap, line_num))
    return utts


def find_overlap_utts(utts: list[UttInfo]) -> list[tuple[int, UttInfo]]:
    """Return (index, utt) for all +< utterances."""
    return [(i, u) for i, u in enumerate(utts) if u.has_overlap_linker]


def predecessor_window(utts: list[UttInfo], idx: int) -> tuple[int, int] | None:
    """Find the preceding utterance's bullet range."""
    for i in range(idx - 1, -1, -1):
        if utts[i].start_ms is not None:
            return (utts[i].start_ms, utts[i].end_ms)
    return None


@dataclass
class TimingComparison:
    """Comparison of one +< utterance across two strategies."""
    utt_idx: int
    speaker: str
    text: str
    a_start: int | None
    a_end: int | None
    b_start: int | None
    b_end: int | None
    pred_start: int | None  # predecessor window
    pred_end: int | None
    start_diff_ms: int | None  # abs difference
    end_diff_ms: int | None

    @property
    def both_timed(self) -> bool:
        return self.a_start is not None and self.b_start is not None

    @property
    def max_diff_ms(self) -> int:
        if not self.both_timed:
            return -1
        return max(self.start_diff_ms or 0, self.end_diff_ms or 0)

    @property
    def within_predecessor(self) -> tuple[bool, bool]:
        """Return (a_within, b_within) — is each strategy's timing inside
        the predecessor's range?"""
        if self.pred_start is None:
            return (False, False)
        a_in = (self.a_start is not None and
                self.a_start >= self.pred_start - 500 and
                self.a_end <= self.pred_end + 500)
        b_in = (self.b_start is not None and
                self.b_start >= self.pred_start - 500 and
                self.b_end <= self.pred_end + 500)
        return (a_in, b_in)


def compare_strategies(
    utts_a: list[UttInfo],
    utts_b: list[UttInfo],
    label_a: str = "A",
    label_b: str = "B",
) -> list[TimingComparison]:
    """Compare +< utterance timing between two strategy outputs."""
    overlaps_a = find_overlap_utts(utts_a)
    overlaps_b = find_overlap_utts(utts_b)

    comparisons = []
    # Match by index (both files should have same utterance structure)
    for (idx_a, ua), (idx_b, ub) in zip(overlaps_a, overlaps_b):
        pred = predecessor_window(utts_a, idx_a)
        pred_start = pred[0] if pred else None
        pred_end = pred[1] if pred else None

        start_diff = abs(ua.start_ms - ub.start_ms) if ua.start_ms is not None and ub.start_ms is not None else None
        end_diff = abs(ua.end_ms - ub.end_ms) if ua.end_ms is not None and ub.end_ms is not None else None

        comparisons.append(TimingComparison(
            utt_idx=idx_a,
            speaker=ua.speaker,
            text=ua.text[:60],
            a_start=ua.start_ms,
            a_end=ua.end_ms,
            b_start=ub.start_ms,
            b_end=ub.end_ms,
            pred_start=pred_start,
            pred_end=pred_end,
            start_diff_ms=start_diff,
            end_diff_ms=end_diff,
        ))
    return comparisons


def report_comparison(
    comparisons: list[TimingComparison],
    file_name: str,
    label_a: str,
    label_b: str,
):
    """Print a timing comparison report."""
    total = len(comparisons)
    if total == 0:
        print(f"  {file_name}: no +< utterances")
        return

    both_timed = [c for c in comparisons if c.both_timed]
    identical = [c for c in both_timed if c.max_diff_ms == 0]
    small_diff = [c for c in both_timed if 0 < c.max_diff_ms <= 500]
    large_diff = [c for c in both_timed if c.max_diff_ms > 500]
    huge_diff = [c for c in both_timed if c.max_diff_ms > 3000]

    a_only = [c for c in comparisons if c.a_start is not None and c.b_start is None]
    b_only = [c for c in comparisons if c.a_start is None and c.b_start is not None]
    neither = [c for c in comparisons if c.a_start is None and c.b_start is None]

    # Within-predecessor analysis
    a_within = sum(1 for c in both_timed if c.within_predecessor[0])
    b_within = sum(1 for c in both_timed if c.within_predecessor[1])

    print(f"\n  {file_name}: {total} +< utterances")
    print(f"    Both timed:    {len(both_timed)}")
    print(f"      Identical:   {len(identical)}")
    print(f"      Diff ≤500ms: {len(small_diff)}")
    print(f"      Diff >500ms: {len(large_diff)}")
    print(f"      Diff >3s:    {len(huge_diff)}")
    print(f"    {label_a} only:      {len(a_only)}")
    print(f"    {label_b} only:      {len(b_only)}")
    print(f"    Neither timed: {len(neither)}")
    print(f"    Within predecessor window (±500ms):")
    print(f"      {label_a}: {a_within}/{len(both_timed)} ({100*a_within/len(both_timed):.0f}%)" if both_timed else "")
    print(f"      {label_b}: {b_within}/{len(both_timed)} ({100*b_within/len(both_timed):.0f}%)" if both_timed else "")

    if huge_diff:
        print(f"    Largest differences (>3s):")
        for c in sorted(huge_diff, key=lambda x: x.max_diff_ms, reverse=True)[:5]:
            print(f"      idx {c.utt_idx} {c.speaker}: {label_a}={c.a_start}-{c.a_end}, "
                  f"{label_b}={c.b_start}-{c.b_end}, diff={c.max_diff_ms}ms, "
                  f"pred={c.pred_start}-{c.pred_end}")

    return {
        "total": total,
        "both_timed": len(both_timed),
        "identical": len(identical),
        "large_diff": len(large_diff),
        "huge_diff": len(huge_diff),
        "a_within": a_within,
        "b_within": b_within,
    }


def analyze_experiment(
    results_dir: Path,
    dir_a: str,
    dir_b: str,
    label_a: str,
    label_b: str,
):
    """Analyze one experiment's timing quality."""
    print(f"\n{'='*60}")
    print(f"Timing Quality: {label_a} vs {label_b}")
    print(f"{'='*60}")

    path_a = results_dir / dir_a
    path_b = results_dir / dir_b

    if not path_a.exists() or not path_b.exists():
        print(f"  SKIP: {path_a} or {path_b} not found")
        return

    aggregate = defaultdict(int)

    for subdir in sorted(path_a.iterdir()):
        if not subdir.is_dir():
            continue
        name = subdir.name
        cha_a = list((path_a / name).glob("*.cha"))
        cha_b = list((path_b / name).glob("*.cha"))

        if not cha_a or not cha_b:
            continue

        utts_a = parse_utterances(cha_a[0])
        utts_b = parse_utterances(cha_b[0])

        comparisons = compare_strategies(utts_a, utts_b, label_a, label_b)
        stats = report_comparison(comparisons, name, label_a, label_b)
        if stats:
            for k, v in stats.items():
                aggregate[k] += v

    if aggregate["total"] > 0:
        print(f"\n  AGGREGATE ({aggregate['total']} +< utterances):")
        print(f"    Both timed:    {aggregate['both_timed']}")
        print(f"    Identical:     {aggregate['identical']} ({100*aggregate['identical']/aggregate['both_timed']:.0f}%)")
        print(f"    Diff >500ms:   {aggregate['large_diff']} ({100*aggregate['large_diff']/aggregate['both_timed']:.0f}%)")
        print(f"    Diff >3s:      {aggregate['huge_diff']}")
        bt = aggregate['both_timed']
        if bt:
            print(f"    Within predecessor (±500ms):")
            print(f"      {label_a}: {aggregate['a_within']}/{bt} ({100*aggregate['a_within']/bt:.0f}%)")
            print(f"      {label_b}: {aggregate['b_within']}/{bt} ({100*aggregate['b_within']/bt:.0f}%)")


def analyze_groundtruth(
    results_dir: Path,
    groundtruth_dir: Path,
    strategy_dir: str,
    label: str,
):
    """Compare one strategy's output against ground truth timing."""
    print(f"\n{'='*60}")
    print(f"Ground Truth Comparison: {label}")
    print(f"{'='*60}")

    strat_path = results_dir / strategy_dir
    if not strat_path.exists():
        print(f"  SKIP: {strat_path} not found")
        return

    aggregate = defaultdict(int)

    for subdir in sorted(strat_path.iterdir()):
        if not subdir.is_dir():
            continue
        name = subdir.name
        gt_file = groundtruth_dir / f"{name}.cha"
        strat_files = list((strat_path / name).glob("*.cha"))

        if not gt_file.exists() or not strat_files:
            continue

        utts_gt = parse_utterances(gt_file)
        utts_s = parse_utterances(strat_files[0])

        comparisons = compare_strategies(utts_gt, utts_s, "truth", label)
        stats = report_comparison(comparisons, name, "truth", label)
        if stats:
            for k, v in stats.items():
                aggregate[k] += v

    if aggregate["total"] > 0:
        bt = aggregate['both_timed']
        print(f"\n  AGGREGATE ({aggregate['total']} +< utterances):")
        print(f"    Both timed:    {bt}")
        print(f"    Match truth:   {aggregate['identical']} ({100*aggregate['identical']/bt:.0f}%)" if bt else "")
        print(f"    Diff >500ms:   {aggregate['large_diff']} ({100*aggregate['large_diff']/bt:.0f}%)" if bt else "")
        print(f"    Diff >3s:      {aggregate['huge_diff']}")
        if bt:
            print(f"    Within predecessor (±500ms):")
            print(f"      truth:  {aggregate['a_within']}/{bt} ({100*aggregate['a_within']/bt:.0f}%)")
            print(f"      {label}: {aggregate['b_within']}/{bt} ({100*aggregate['b_within']/bt:.0f}%)")


def main():
    os.chdir(Path(__file__).parent.parent)

    print("OVERLAP ALIGNMENT EXPERIMENT — TIMING QUALITY ANALYSIS")
    print(f"Date: 2026-03-17")

    # Experiment 1: APROCSA — C (global) vs D (two-pass)
    analyze_experiment(
        Path("results/overlap-experiment"),
        "C-with-linker-global", "D-with-linker-twopass",
        "global", "two-pass",
    )

    # Experiment 1: APROCSA — B (no linker) vs D (two-pass)
    analyze_experiment(
        Path("results/overlap-experiment"),
        "B-no-linker-global", "D-with-linker-twopass",
        "no-linker", "two-pass",
    )

    # Experiment 2: CHILDES ground truth
    for strat, label in [("global", "global"), ("two-pass", "two-pass")]:
        analyze_groundtruth(
            Path("results/groundtruth-experiment"),
            Path("data/groundtruth"),
            strat, label,
        )

    # Experiment 3: CORAAL — C vs D
    analyze_experiment(
        Path("results/coraal-experiment"),
        "C-with-linker-global", "D-with-linker-twopass",
        "global", "two-pass",
    )

    # Experiment 4: Multilingual — global vs two-pass
    analyze_experiment(
        Path("results/multilang-experiment"),
        "global", "two-pass",
        "global", "two-pass",
    )

    # Multilingual ground truth comparisons
    for strat, label in [("global", "global"), ("two-pass", "two-pass")]:
        analyze_groundtruth(
            Path("results/multilang-experiment"),
            Path("data/multilang-groundtruth"),
            strat, label,
        )


if __name__ == "__main__":
    main()
