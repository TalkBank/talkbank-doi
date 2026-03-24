#!/usr/bin/env python3
"""Classify and analyze multi-annotation patterns — grammar-aware version.

Only considers annotations that are base_annotation in the grammar:
  overlap [<] [>], stressing [!] [!!], best_guess [!*], uncertain [?],
  explanation [= ...], paralinguistic [=! ...], alternative [=? ...],
  percent [% ...], duration [# ...], error [*] [* code],
  retrace [/] [//] [///] [/-] [/?], exclude [e]

Excludes: freecodes [^ ...], postcodes [+ ...], language codes [- ...],
          replacements [: ...], CA continuation [^c].
"""
import re
from collections import Counter, defaultdict

# Regex for a single base_annotation bracket
# Order matters for the alternation — longer matches first
ANNOTATION_RE = re.compile(
    r'\[///\]'          # retrace_multiple
    r'|\[//\]'          # retrace_complete
    r'|\[/\-\]'         # retrace_reformulation
    r'|\[/\?\]'         # retrace_uncertain
    r'|\[/\]'           # retrace_partial
    r'|\[<\d*\]'        # indexed_overlap_precedes
    r'|\[>\d*\]'        # indexed_overlap_follows
    r'|\[!\*\]'         # best_guess
    r'|\[!!\]'          # contrastive_stressing
    r'|\[!\]'           # stressing
    r'|\[\?\]'          # uncertain
    r'|\[=! [^\]]+\]'   # para_annotation
    r'|\[=\? [^\]]+\]'  # alt_annotation
    r'|\[= [^\]]+\]'    # explanation_annotation
    r'|\[% [^\]]+\]'    # percent_annotation
    r'|\[# [^\]]+\]'    # duration_annotation
    r'|\[\* ?[^\]]*\]'  # error_marker_annotation ([*] or [* code])
    r'|\[e\]'           # exclude_marker
)

def classify(bracket: str) -> str:
    if bracket == '[e]': return 'EXCLUDE'
    if bracket == '[!*]': return 'BEST_GUESS'
    if bracket == '[!!]': return 'CONTRASTIVE_STRESS'
    if bracket == '[!]': return 'STRESSING'
    if bracket == '[?]': return 'UNCERTAIN'
    if bracket == '[/]': return 'RETRACE_PARTIAL'
    if bracket == '[//]': return 'RETRACE_FULL'
    if bracket == '[///]': return 'RETRACE_MULTIPLE'
    if bracket == '[/-]': return 'REFORMULATION'
    if bracket == '[/?]': return 'RETRACE_UNCERTAIN'
    if re.match(r'\[<\d*\]', bracket): return 'OVERLAP_BEGIN'
    if re.match(r'\[>\d*\]', bracket): return 'OVERLAP_END'
    if bracket.startswith('[=! '): return 'PARALINGUISTIC'
    if bracket.startswith('[=? '): return 'ALTERNATIVE'
    if bracket.startswith('[= '): return 'EXPLANATION'
    if bracket.startswith('[% '): return 'PERCENT'
    if bracket.startswith('[# '): return 'DURATION'
    if bracket.startswith('[*'): return 'ERROR'
    return f'UNKNOWN({bracket})'

def is_retrace(t: str) -> bool:
    return t.startswith('RETRACE_') or t == 'REFORMULATION'

def find_annotation_sequences(line: str) -> list[list[str]]:
    """Find runs of 2+ consecutive base_annotations on a main tier line.

    Returns list of annotation-bracket sequences (each >= 2 long).
    Consecutive means: ] [  (close bracket, space, open bracket).
    """
    # Find all annotation matches with their positions
    matches = [(m.start(), m.end(), m.group()) for m in ANNOTATION_RE.finditer(line)]
    if len(matches) < 2:
        return []

    sequences: list[list[str]] = []
    current_run: list[str] = [matches[0][2]]

    for i in range(1, len(matches)):
        prev_end = matches[i-1][1]
        curr_start = matches[i][0]
        gap = line[prev_end:curr_start]
        # Consecutive if the gap is just a space
        if gap == ' ':
            current_run.append(matches[i][2])
        else:
            if len(current_run) >= 2:
                sequences.append(current_run)
            current_run = [matches[i][2]]

    if len(current_run) >= 2:
        sequences.append(current_run)

    return sequences


def main():
    import subprocess, os

    data_dir = os.path.expanduser('../data')

    # Use rg to get all main tier lines (fast)
    print("Scanning corpus with rg...", flush=True)
    result = subprocess.run(
        ['rg', '--glob', '*.cha', '-n', r'^\*\w+:', data_dir],
        capture_output=True, text=True, timeout=300
    )

    lines = result.stdout.splitlines()
    total_main_lines = len(lines)
    print(f"Total main tier lines: {total_main_lines:,}")

    # Process each line
    all_sequences: list[tuple[str, list[str], list[str]]] = []  # (file:line, raw, classified)

    for line in lines:
        # Split file:lineno:content
        parts = line.split(':', 2)
        if len(parts) < 3:
            continue
        file_ref = f"{parts[0]}:{parts[1]}"
        content = parts[2]

        for seq in find_annotation_sequences(content):
            classified = [classify(b) for b in seq]
            all_sequences.append((file_ref, seq, classified))

    print(f"Lines with multi-annotation sequences: {len(all_sequences):,}")
    print()

    # === SECTION 1: Pair frequency ===
    print("=" * 70)
    print("SECTION 1: ANNOTATION PAIR FREQUENCY (grammar base_annotation only)")
    print("=" * 70)
    print()

    pair_types = Counter()
    pair_raw = Counter()
    for file_ref, raw, classified in all_sequences:
        for i in range(len(classified) - 1):
            pair_types[(classified[i], classified[i+1])] += 1
            pair_raw[(' '.join(raw[i:i+2]),)] += 1

    total_pairs = sum(pair_types.values())
    print(f"Total annotation pairs: {total_pairs:,}")
    print()

    print("--- Abstract type pairs (top 30) ---")
    print()
    for (a, b), count in pair_types.most_common(30):
        pct = count / total_pairs * 100
        print(f"  {count:>6} ({pct:4.1f}%)  {a} → {b}")
    print()

    # === SECTION 2: Excluding [*][*] (data quality), what's left? ===
    print("=" * 70)
    print("SECTION 2: EXCLUDING DOUBLE ERROR [*][*]")
    print("=" * 70)
    print()

    non_double_error = {k: v for k, v in pair_types.items()
                        if k != ('ERROR', 'ERROR')}
    total_non_error = sum(non_double_error.values())
    print(f"Pairs excluding ERROR→ERROR: {total_non_error:,}")
    print()

    for (a, b), count in Counter(non_double_error).most_common(30):
        pct = count / total_non_error * 100
        print(f"  {count:>6} ({pct:4.1f}%)  {a} → {b}")
    print()

    # === SECTION 3: Ordering analysis ===
    print("=" * 70)
    print("SECTION 3: ORDERING ANALYSIS (≥20 examples, excluding same-type pairs)")
    print("=" * 70)
    print()

    unordered: dict[tuple, dict[tuple, int]] = defaultdict(lambda: defaultdict(int))
    for (a, b), count in pair_types.items():
        key = tuple(sorted([a, b]))
        unordered[key][(a, b)] += count

    print(f"{'Total':>6}  {'Dominant order':40}  {'Reverse':>8}  {'Consistency':>6}")
    print("-" * 75)

    strict_rules = []
    for key, orders in sorted(unordered.items(), key=lambda x: -sum(x[1].values())):
        total = sum(orders.values())
        if total < 20 or key[0] == key[1]:
            continue
        sorted_orders = sorted(orders.items(), key=lambda x: -x[1])
        (dom_a, dom_b), dom_count = sorted_orders[0]
        rev_count = sorted_orders[1][1] if len(sorted_orders) > 1 else 0
        consistency = dom_count / total * 100

        print(f"  {total:>5}  {dom_a:>20} → {dom_b:<20} {rev_count:>6}  {consistency:5.1f}%")

        if consistency >= 95:
            strict_rules.append((dom_a, dom_b, total, consistency))

    # === SECTION 4: Strict ordering rules ===
    print()
    print("=" * 70)
    print("SECTION 4: STRICT ORDERING RULES (≥95% consistent, ≥20 examples)")
    print("=" * 70)
    print()

    for a, b, total, pct in sorted(strict_rules, key=lambda x: -x[2]):
        print(f"  {a} BEFORE {b}  ({total} examples, {pct:.0f}%)")

    # Topological sort for total order
    print()
    print("--- Induced Total Order ---")
    print()

    precedes: dict[str, set[str]] = defaultdict(set)
    all_types: set[str] = set()
    for a, b, _, _ in strict_rules:
        precedes[a].add(b)
        all_types.add(a)
        all_types.add(b)

    in_degree = {t: 0 for t in all_types}
    for a, bs in precedes.items():
        for b in bs:
            in_degree[b] = in_degree.get(b, 0) + 1

    queue = sorted([t for t in all_types if in_degree.get(t, 0) == 0])
    order = []
    while queue:
        node = queue.pop(0)
        order.append(node)
        for b in sorted(precedes.get(node, set())):
            in_degree[b] -= 1
            if in_degree[b] == 0:
                queue.append(b)
                queue.sort()

    if len(order) == len(all_types):
        for i, t in enumerate(order, 1):
            print(f"  {i:>2}. {t}")
        print()
        print("  (No cycles — a strict total order is possible)")
    else:
        print("  Ordered so far:", order)
        remaining = all_types - set(order)
        print("  Cycle in:", remaining)
    print()

    # === SECTION 5: Retrace combos ===
    print("=" * 70)
    print("SECTION 5: RETRACE + OTHER COMBINATIONS")
    print("=" * 70)
    print()

    retrace_pairs = Counter()
    for (a, b), count in pair_types.items():
        if is_retrace(a) or is_retrace(b):
            retrace_pairs[(a, b)] += count

    print(f"Total retrace + other pairs: {sum(retrace_pairs.values()):,}")
    print()
    for (a, b), count in retrace_pairs.most_common(15):
        print(f"  {count:>6}  {a} → {b}")
    print()

    # === SECTION 6: Sequences of 3+ ===
    print("=" * 70)
    print("SECTION 6: SEQUENCES OF 3+ ANNOTATIONS")
    print("=" * 70)
    print()

    triple_plus = [(f, r, c) for f, r, c in all_sequences if len(c) >= 3]
    print(f"Sequences of length 3+: {len(triple_plus):,}")
    print()

    triple_types = Counter()
    for _, _, classified in triple_plus:
        triple_types[tuple(classified)] += 1

    print("--- By abstract type (top 20) ---")
    print()
    for types, count in triple_types.most_common(20):
        print(f"  {count:>6}  {' → '.join(types)}")
    print()

    quad_plus = [x for x in triple_plus if len(x[2]) >= 4]
    print(f"Sequences of length 4+: {len(quad_plus):,}")
    if quad_plus:
        quad_types = Counter()
        for _, _, classified in quad_plus:
            quad_types[tuple(classified)] += 1
        print()
        for types, count in quad_types.most_common(10):
            print(f"  {count:>6}  {' → '.join(types)}")
    print()

    # === SECTION 7: Examples of interesting combos ===
    print("=" * 70)
    print("SECTION 7: EXAMPLES OF KEY PATTERNS")
    print("=" * 70)
    print()

    def show_examples(label, pred, n=5):
        print(f"--- {label} ---")
        examples = [(f, r) for f, r, c in all_sequences if pred(c)][:n]
        for file_ref, raw in examples:
            print(f"  {file_ref}:  {' '.join(raw)}")
        print()

    show_examples("ERROR + RETRACE",
                  lambda c: any(t == 'ERROR' for t in c) and any(is_retrace(t) for t in c))
    show_examples("OVERLAP + RETRACE",
                  lambda c: any('OVERLAP' in t for t in c) and any(is_retrace(t) for t in c))
    show_examples("UNCERTAIN + PARALINGUISTIC",
                  lambda c: 'UNCERTAIN' in c and 'PARALINGUISTIC' in c)
    show_examples("3+ distinct types",
                  lambda c: len(set(c)) >= 3 and len(c) >= 3)

    # === Summary ===
    print("=" * 70)
    print("SUMMARY")
    print("=" * 70)
    print()
    print(f"  Total main tier lines:       {total_main_lines:>12,}")
    print(f"  Multi-annotation sequences:  {len(all_sequences):>12,}")
    print(f"  Total annotation pairs:      {total_pairs:>12,}")
    print(f"    of which ERROR→ERROR:      {pair_types[('ERROR','ERROR')]:>12,} (data quality issue?)")
    print(f"    remaining real pairs:       {total_non_error:>12,}")
    print(f"  Retrace + other:             {sum(retrace_pairs.values()):>12,}")
    print(f"  Strict ordering rules:       {len(strict_rules):>12}")
    print(f"  Ordering cycles:             {'none' if len(order) == len(all_types) else 'YES'}")
    print(f"  Rate: 1 multi-annotation per {total_main_lines // (len(all_sequences) + 1):,} main tier lines")

if __name__ == '__main__':
    main()
