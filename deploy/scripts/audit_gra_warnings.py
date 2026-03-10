#!/usr/bin/env python3
"""
Audit all CHAT files in ~/data for %gra validation warnings.

Parses each file and collects statistics on E722, E723, E724 warnings.
"""

import sys
from pathlib import Path
from collections import defaultdict, Counter
import json

try:
    from batchalign_core import parse_lenient_pure
except ImportError as e:
    print(f"ERROR: batchalign_core import failed: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)


def audit_file(filepath: Path) -> dict:
    """Parse a file and return warning statistics."""
    try:
        content = filepath.read_text(encoding='utf-8', errors='replace')
        result = parse_lenient_pure(content)

        # Count warnings by code
        warnings = defaultdict(int)
        for error in result.errors:
            if error['severity'] == 'Warning':
                code = error['code']
                warnings[code] += 1

        return {
            'path': str(filepath),
            'warnings': dict(warnings),
            'total_warnings': sum(warnings.values()),
        }
    except Exception as e:
        return {
            'path': str(filepath),
            'error': str(e),
            'warnings': {},
            'total_warnings': 0,
        }


def main():
    """Run main."""
    data_dir = Path.home() / 'data'

    if not data_dir.exists():
        print(f"ERROR: {data_dir} does not exist")
        sys.exit(1)

    print(f"Scanning {data_dir} for .cha files...")
    cha_files = list(data_dir.rglob('*.cha'))
    print(f"Found {len(cha_files)} CHAT files\n")

    # Track statistics
    total_files = 0
    files_with_warnings = 0
    files_with_gra_warnings = 0
    warning_counts = Counter()
    gra_warning_counts = Counter()
    files_by_warning = defaultdict(list)
    collection_stats = defaultdict(lambda: {'total': 0, 'with_warnings': 0, 'warnings': Counter()})

    # Process each file
    for i, filepath in enumerate(cha_files, 1):
        if i % 1000 == 0:
            print(f"Progress: {i}/{len(cha_files)} files...", file=sys.stderr)

        result = audit_file(filepath)
        total_files += 1

        # Extract collection name (first directory under ~/data)
        rel_path = filepath.relative_to(data_dir)
        collection = str(rel_path.parts[0]) if rel_path.parts else 'unknown'

        collection_stats[collection]['total'] += 1

        if result['total_warnings'] > 0:
            files_with_warnings += 1
            collection_stats[collection]['with_warnings'] += 1

            has_gra_warning = False
            for code, count in result['warnings'].items():
                warning_counts[code] += count
                collection_stats[collection]['warnings'][code] += count
                files_by_warning[code].append(result['path'])

                if code in ['E722', 'E723', 'E724']:
                    has_gra_warning = True
                    gra_warning_counts[code] += count

            if has_gra_warning:
                files_with_gra_warnings += 1

    # Generate report
    print("\n" + "=" * 80)
    print("CORPUS-WIDE %GRA VALIDATION AUDIT")
    print("=" * 80 + "\n")

    print(f"Total files scanned: {total_files:,}")
    print(f"Files with warnings: {files_with_warnings:,} ({100*files_with_warnings/total_files:.1f}%)")
    print(f"Files with %gra warnings: {files_with_gra_warnings:,} ({100*files_with_gra_warnings/total_files:.1f}%)")
    print()

    print("=" * 80)
    print("%GRA WARNING BREAKDOWN")
    print("=" * 80)
    print()

    gra_codes = ['E722', 'E723', 'E724']
    for code in gra_codes:
        count = gra_warning_counts.get(code, 0)
        file_count = len(files_by_warning.get(code, []))
        print(f"{code}: {count:,} warnings in {file_count:,} files")

    if not gra_warning_counts:
        print("(No %gra warnings found)")

    print()
    print("=" * 80)
    print("TOP 10 COLLECTIONS BY %GRA WARNING COUNT")
    print("=" * 80)
    print()

    # Sort collections by total %gra warnings
    collection_gra_counts = {}
    for coll, stats in collection_stats.items():
        gra_total = sum(stats['warnings'].get(code, 0) for code in gra_codes)
        if gra_total > 0:
            collection_gra_counts[coll] = gra_total

    sorted_collections = sorted(collection_gra_counts.items(), key=lambda x: x[1], reverse=True)

    for coll, gra_count in sorted_collections[:10]:
        stats = collection_stats[coll]
        print(f"{coll:30s} {gra_count:6,} warnings in {stats['with_warnings']:5,}/{stats['total']:5,} files")

    print()
    print("=" * 80)
    print("DETAILED COLLECTION BREAKDOWN")
    print("=" * 80)
    print()

    for coll in sorted(collection_gra_counts.keys()):
        stats = collection_stats[coll]
        gra_total = collection_gra_counts[coll]

        print(f"\n{coll}:")
        print(f"  Total files: {stats['total']:,}")
        print(f"  Files with %gra warnings: {stats['with_warnings']:,}")
        print(f"  Warning breakdown:")
        for code in gra_codes:
            count = stats['warnings'].get(code, 0)
            if count > 0:
                print(f"    {code}: {count:,}")

    # Save detailed results to JSON
    output_file = Path('results/gra_warnings_audit.json')
    output_file.parent.mkdir(exist_ok=True)

    results = {
        'summary': {
            'total_files': total_files,
            'files_with_warnings': files_with_warnings,
            'files_with_gra_warnings': files_with_gra_warnings,
            'warning_counts': dict(warning_counts),
            'gra_warning_counts': dict(gra_warning_counts),
        },
        'collections': {
            coll: {
                'total': stats['total'],
                'with_warnings': stats['with_warnings'],
                'warnings': dict(stats['warnings']),
            }
            for coll, stats in collection_stats.items()
        },
        'files_by_warning': {
            code: files for code, files in files_by_warning.items()
            if code in gra_codes
        },
    }

    output_file.write_text(json.dumps(results, indent=2))
    print(f"\nDetailed results saved to: {output_file}")
    print(f"\nTo see files with specific warnings:")
    for code in gra_codes:
        if code in files_by_warning:
            print(f"  jq '.files_by_warning.{code}[]' {output_file} | head -20")


if __name__ == '__main__':
    main()
