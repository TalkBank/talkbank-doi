#!/usr/bin/env python3
"""Extract files needing re-align or re-morphotag from test.log."""

import re
from pathlib import Path
from collections import defaultdict

# Error codes by category
GRA_ERRORS = {'E722', 'E723', 'E724'}  # Broken %gra tiers (morphotag)
WOR_ERRORS = {'E342', 'E601', 'E714', 'E715'}  # Broken %wor tiers (align)

# Parse test.log
log_path = Path.home() / 'test.log'
print(f"Parsing {log_path}...")

gra_files = set()
wor_files = set()
file_errors = defaultdict(set)

current_file = None
line_count = 0

with open(log_path, 'r', encoding='utf-8', errors='replace') as f:
    for line in f:
        line_count += 1
        if line_count % 100000 == 0:
            print(f"  Processed {line_count:,} lines...")

        # Match file path
        # Format: "✗ Errors found in /path/to/file.cha"
        if 'Errors found in' in line and '.cha' in line:
            match = re.search(r'Errors found in\s+(.+\.cha)', line)
            if match:
                current_file = match.group(1).strip()

        # Match error codes
        # Format: "E722 (https://..." or "warning[E722]:" etc.
        if current_file:
            # Look for E### patterns
            for error_match in re.finditer(r'\b(E\d{3})\b', line):
                error_code = error_match.group(1)
                file_errors[current_file].add(error_code)

                if error_code in GRA_ERRORS:
                    gra_files.add(current_file)
                if error_code in WOR_ERRORS:
                    wor_files.add(current_file)

print(f"Processed {line_count:,} total lines")
print(f"Found {len(gra_files):,} files with %gra errors")
print(f"Found {len(wor_files):,} files with %wor errors")

# Write output files
gra_out = Path.home() / 'batchalign2' / 'FILES_NEED_MORPHOTAG.txt'
wor_out = Path.home() / 'batchalign2' / 'FILES_NEED_ALIGN.txt'

print(f"\nWriting {gra_out}...")
with open(gra_out, 'w') as f:
    f.write(f"# Files Requiring Re-Morphotag (Broken %gra Tiers)\n")
    f.write(f"# Total: {len(gra_files):,} files\n")
    f.write(f"# Error codes: {', '.join(sorted(GRA_ERRORS))}\n")
    f.write(f"# Generated from: {log_path}\n")
    f.write(f"#\n")
    f.write(f"# Root cause: Array wraparound bug in Python master morphotag\n")
    f.write(f"# Impact: Circular dependencies, missing ROOT relations, multiple ROOTs\n")
    f.write(f"# Solution: Re-run morphotag with Rust implementation\n")
    f.write(f"#\n")
    f.write(f"# Usage:\n")
    f.write(f"#   batchalign3 morphotag --file-list FILES_NEED_MORPHOTAG.txt -o /tmp/fixed/\n")
    f.write(f"#   (Or use --server http://net:8000 for server mode)\n")
    f.write(f"#\n\n")

    # Group by collection
    by_collection = defaultdict(list)
    for fpath in sorted(gra_files):
        # Extract collection name (e.g., "aphasia-data", "dementia-data")
        parts = Path(fpath).parts
        if 'data' in parts:
            idx = parts.index('data')
            if idx + 1 < len(parts):
                collection = parts[idx + 1]
                by_collection[collection].append(fpath)

    # Write by collection
    for collection in sorted(by_collection.keys()):
        files = by_collection[collection]
        f.write(f"# === {collection} ({len(files):,} files) ===\n\n")
        for fpath in files:
            errors = sorted(file_errors[fpath] & GRA_ERRORS)
            f.write(f"{fpath}  # {', '.join(errors)}\n")
        f.write(f"\n")

print(f"Writing {wor_out}...")
with open(wor_out, 'w') as f:
    f.write(f"# Files Requiring Re-Align (Broken %wor Tiers)\n")
    f.write(f"# Total: {len(wor_files):,} files\n")
    f.write(f"# Error codes: {', '.join(sorted(WOR_ERRORS))}\n")
    f.write(f"# Generated from: {log_path}\n")
    f.write(f"#\n")
    f.write(f"# Root cause: Mix of legacy CLAN data quality issues and Python generation bugs\n")
    f.write(f"# Error types:\n")
    f.write(f"#   E342: Trailing bullet after %wor terminator\n")
    f.write(f"#   E601: Complex %wor content (retraces/events - invalid structure)\n")
    f.write(f"#   E714: %wor word count mismatch with main tier\n")
    f.write(f"#   E715: %wor alignment index out of bounds\n")
    f.write(f"# Solution: Re-run align with Rust implementation\n")
    f.write(f"#\n")
    f.write(f"# Usage:\n")
    f.write(f"#   batchalign3 align --file-list FILES_NEED_ALIGN.txt -o /tmp/fixed/\n")
    f.write(f"#   (Or use --server http://net:8000 for server mode)\n")
    f.write(f"#\n\n")

    # Group by collection
    by_collection = defaultdict(list)
    for fpath in sorted(wor_files):
        parts = Path(fpath).parts
        if 'data' in parts:
            idx = parts.index('data')
            if idx + 1 < len(parts):
                collection = parts[idx + 1]
                by_collection[collection].append(fpath)

    # Write by collection
    for collection in sorted(by_collection.keys()):
        files = by_collection[collection]
        f.write(f"# === {collection} ({len(files):,} files) ===\n\n")
        for fpath in files:
            errors = sorted(file_errors[fpath] & WOR_ERRORS)
            f.write(f"{fpath}  # {', '.join(errors)}\n")
        f.write(f"\n")

print(f"\nDone!")
print(f"  {gra_out.name}: {len(gra_files):,} files")
print(f"  {wor_out.name}: {len(wor_files):,} files")

# Statistics
overlap = gra_files & wor_files
print(f"\nOverlap: {len(overlap):,} files have BOTH %gra and %wor errors")
print(f"  (These need both re-morphotag AND re-align)")

# Collection breakdown
print(f"\n%gra errors by collection:")
gra_collections = defaultdict(int)
for fpath in gra_files:
    parts = Path(fpath).parts
    if 'data' in parts:
        idx = parts.index('data')
        if idx + 1 < len(parts):
            gra_collections[parts[idx + 1]] += 1
for coll in sorted(gra_collections.keys()):
    print(f"  {coll}: {gra_collections[coll]:,}")

print(f"\n%wor errors by collection:")
wor_collections = defaultdict(int)
for fpath in wor_files:
    parts = Path(fpath).parts
    if 'data' in parts:
        idx = parts.index('data')
        if idx + 1 < len(parts):
            wor_collections[parts[idx + 1]] += 1
for coll in sorted(wor_collections.keys()):
    print(f"  {coll}: {wor_collections[coll]:,}")
