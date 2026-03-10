#!/usr/bin/env python3
"""
Analyze %gra warnings from test.log.
"""
import argparse
from collections import Counter
import os
from pathlib import Path


DEFAULT_LOG = Path(
    os.environ.get("WOR_VALIDATION_LOG", str(Path.home() / "test.log"))
).expanduser()

def main():
    """Run main."""
    parser = argparse.ArgumentParser(description="Analyze %gra warnings from a validation log.")
    parser.add_argument(
        "--log",
        type=Path,
        default=DEFAULT_LOG,
        help=f"Path to validation log (default: {DEFAULT_LOG}; env: WOR_VALIDATION_LOG)",
    )
    args = parser.parse_args()
    log_file = args.log
    if not log_file.is_file():
        raise SystemExit(f"ERROR: log file not found: {log_file}")

    # Count warnings by type
    warning_counts = Counter()

    # Track which lines have which warnings (to count unique files later)
    files_by_warning = {
        'E722': set(),
        'E723': set(),
        'E724': set(),
    }

    current_file = None

    with open(log_file, 'r', encoding='utf-8') as f:
        for line in f:
            # Look for file paths (if they exist)
            if line.startswith('/'):
                current_file = line.strip()

            # Count warnings
            if 'warning[E722]' in line:
                warning_counts['E722'] += 1
                if current_file:
                    files_by_warning['E722'].add(current_file)
            elif 'warning[E723]' in line:
                warning_counts['E723'] += 1
                if current_file:
                    files_by_warning['E723'].add(current_file)
            elif 'warning[E724]' in line:
                warning_counts['E724'] += 1
                if current_file:
                    files_by_warning['E724'].add(current_file)

    print("=" * 80)
    print("CORPUS-WIDE %GRA VALIDATION WARNINGS")
    print("=" * 80)
    print()

    print(f"Total warnings found: {sum(warning_counts.values()):,}")
    print()

    print("Warning breakdown:")
    print(f"  E722 (no ROOT):          {warning_counts['E722']:,}")
    print(f"  E723 (multiple ROOTs):   {warning_counts['E723']:,}")
    print(f"  E724 (circular deps):    {warning_counts['E724']:,}")
    print()

    if any(files_by_warning.values()):
        print("Unique files affected:")
        for code in ['E722', 'E723', 'E724']:
            print(f"  {code}: {len(files_by_warning[code]):,} files")
    else:
        print("Note: Log does not include file paths - only warning counts available")
    print()

if __name__ == '__main__':
    main()
