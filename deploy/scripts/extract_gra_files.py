#!/usr/bin/env python3
"""Extract files with %gra errors from corpus-audit.jsonl."""

import json
from pathlib import Path
from collections import defaultdict

GRA_ERRORS = {'E722', 'E723', 'E724'}

audit_file = Path.home() / 'corpus-audit.jsonl'
print(f"Reading {audit_file}...")

gra_files = set()
file_errors = defaultdict(set)
line_count = 0

with open(audit_file, 'r') as f:
    for line in f:
        line_count += 1
        if line_count % 1000000 == 0:
            print(f"  Processed {line_count:,} errors...")

        try:
            error = json.loads(line)
            if error['code'] in GRA_ERRORS:
                file_path = error['file']
                gra_files.add(file_path)
                file_errors[file_path].add(error['code'])
        except (json.JSONDecodeError, KeyError):
            continue

print(f"\nProcessed {line_count:,} total errors")
print(f"Found {len(gra_files):,} files with %gra errors")

# Write output
output = Path.home() / 'batchalign2' / 'FILES_NEED_MORPHOTAG.txt'
print(f"\nWriting {output}...")

with open(output, 'w') as f:
    f.write(f"# Files Requiring Re-Morphotag (Broken %gra Tiers)\n")
    f.write(f"# Total: {len(gra_files):,} files\n")
    f.write(f"# Error codes: {', '.join(sorted(GRA_ERRORS))}\n")
    f.write(f"# Generated from: {audit_file}\n")
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
            errors = sorted(file_errors[fpath])
            f.write(f"{fpath}  # {', '.join(errors)}\n")
        f.write(f"\n")

print(f"Done! {len(gra_files):,} files written to {output.name}")

# Collection breakdown
print(f"\nBy collection:")
for coll in sorted(by_collection.keys()):
    print(f"  {coll}: {len(by_collection[coll]):,}")
