#!/usr/bin/env python3
"""Add or update 'NoAlign' in the @Options header of CHAT files.

Usage:
    python3 add_noalign.py FILE [FILE ...]
    python3 add_noalign.py --paths-file list.txt

Each CHAT file is rewritten in place. If @Options: already exists and
does not already contain 'NoAlign', it is appended (comma-separated).
If @Options: does not exist, a new line '@Options:\tNoAlign' is inserted
before the first @ID: header. Files that already have NoAlign are skipped.

Exit codes:
    0  All files processed successfully
    1  One or more files could not be processed (errors printed to stderr)
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

OPTIONS_RE = re.compile(r"^@Options:\t(.*)$")
ID_RE = re.compile(r"^@ID:\t")


def add_noalign(path: Path, *, dry_run: bool = False) -> str:
    """Process a single CHAT file.

    Returns a status string: 'updated', 'already', or 'created'.
    Raises ValueError on problems.
    """
    text = path.read_text(encoding="utf-8")
    lines = text.split("\n")

    # Look for existing @Options: line
    for i, line in enumerate(lines):
        m = OPTIONS_RE.match(line)
        if m:
            existing = m.group(1)
            # Check if NoAlign is already present (as a whole word)
            options = [o.strip() for o in existing.split(",")]
            if "NoAlign" in options:
                return "already"
            # Append NoAlign
            new_value = existing + ", NoAlign" if existing.strip() else "NoAlign"
            lines[i] = f"@Options:\t{new_value}"
            if not dry_run:
                path.write_text("\n".join(lines), encoding="utf-8")
            return "updated"

    # No @Options: found — insert before first @ID:
    insert_idx = None
    for i, line in enumerate(lines):
        if ID_RE.match(line):
            insert_idx = i
            break

    if insert_idx is None:
        raise ValueError(f"No @ID: header found in {path}")

    lines.insert(insert_idx, "@Options:\tNoAlign")
    if not dry_run:
        path.write_text("\n".join(lines), encoding="utf-8")
    return "created"


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Add 'NoAlign' to @Options header in CHAT files."
    )
    parser.add_argument("files", nargs="*", help="CHAT files to process")
    parser.add_argument(
        "--paths-file",
        type=Path,
        help="Text file with one CHAT file path per line",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would be done without modifying files",
    )
    args = parser.parse_args()

    paths: list[Path] = []
    if args.paths_file:
        for line in args.paths_file.read_text().splitlines():
            line = line.strip()
            if line and not line.startswith("#"):
                paths.append(Path(line))
    paths.extend(Path(f) for f in args.files)

    if not paths:
        parser.error("No files specified. Provide files or --paths-file.")

    counts = {"created": 0, "updated": 0, "already": 0, "error": 0}
    for p in paths:
        if not p.exists():
            print(f"ERROR: {p} does not exist", file=sys.stderr)
            counts["error"] += 1
            continue
        try:
            status = add_noalign(p, dry_run=args.dry_run)
            counts[status] += 1
            if args.dry_run:
                print(f"[dry-run] {status}: {p}")
        except Exception as e:
            print(f"ERROR: {p}: {e}", file=sys.stderr)
            counts["error"] += 1

    prefix = "[dry-run] " if args.dry_run else ""
    print(
        f"\n{prefix}Done: {counts['created']} created, "
        f"{counts['updated']} updated, "
        f"{counts['already']} already had NoAlign, "
        f"{counts['error']} errors"
    )
    sys.exit(1 if counts["error"] > 0 else 0)


if __name__ == "__main__":
    main()
