#!/usr/bin/env python3
"""Fix unmatched underline markers (␂␁/␂␂) in CA CHAT files.

CA transcribers sometimes place underline begin (\\x02\\x01) or end
(\\x02\\x02) markers without a matching pair. The parser correctly
rejects these as E356 (unmatched begin) or E357 (unmatched end).

Fix strategy: scan each utterance (main tier + continuations) for
underline markers, match them as pairs left-to-right, and remove
any unmatched markers.

Usage:
    uv run python scripts/fix_underline_markers.py --dry-run
    uv run python scripts/fix_underline_markers.py
"""

from __future__ import annotations

import argparse
import os
from pathlib import Path

UNDERLINE_BEGIN = "\x02\x01"
UNDERLINE_END = "\x02\x02"
DATA_ROOT = Path(
    os.environ.get("TALKBANK_DATA_ROOT", str(Path.home() / "data"))
).expanduser()

# Files with E356/E357 errors from the corpus validation log
AFFECTED_RELATIVE_FILES = [
    "ca-data/Jefferson/NB/06fungus.cha",
    "ca-data/Jefferson/NB/08assassination2.cha",
    "ca-data/Jefferson/NB/09palmsprings.cha",
    "ca-data/Jefferson/NB/10blinddate.cha",
    "ca-data/Jefferson/NB/11goldbridge.cha",
    "ca-data/Jefferson/NB/12invitation.cha",
    "ca-data/Jefferson/NB/19paper.cha",
    "ca-data/Jefferson/NB/21swimnude.cha",
    "ca-data/Jefferson/NB/22thanksgiving.cha",
    "ca-data/Jefferson/NB/24meatless.cha",
    "ca-data/Jefferson/Watergate/3-21ndh.cha",
    "ca-data/Jefferson/Watergate/4-13ne2.cha",
]


def default_affected_files(data_root: Path) -> list[str]:
    """Build absolute paths for known affected files under a data root."""
    root = data_root.expanduser()
    return [str(root / rel) for rel in AFFECTED_RELATIVE_FILES]


def fix_underline_markers(text: str) -> tuple[str, int, int]:
    """Remove unmatched underline markers from text.

    Returns (fixed_text, begins_removed, ends_removed).
    """
    # Find all marker positions
    markers: list[tuple[int, str]] = []
    i = 0
    while i < len(text) - 1:
        pair = text[i:i + 2]
        if pair == UNDERLINE_BEGIN:
            markers.append((i, "begin"))
            i += 2
        elif pair == UNDERLINE_END:
            markers.append((i, "end"))
            i += 2
        else:
            i += 1

    if not markers:
        return text, 0, 0

    # Match begin/end pairs left-to-right using a stack
    # A begin is matched by the next end that follows it
    unmatched_begins: list[int] = []  # stack of begin positions
    matched: set[int] = set()

    for pos, kind in markers:
        if kind == "begin":
            unmatched_begins.append(pos)
        else:  # end
            if unmatched_begins:
                # Match with most recent begin
                begin_pos = unmatched_begins.pop()
                matched.add(begin_pos)
                matched.add(pos)
            # else: unmatched end, will be removed

    # Unmatched markers are those not in matched set
    to_remove: set[int] = set()
    begins_removed = 0
    ends_removed = 0

    for pos, kind in markers:
        if pos not in matched:
            to_remove.add(pos)
            if kind == "begin":
                begins_removed += 1
            else:
                ends_removed += 1

    if not to_remove:
        return text, 0, 0

    # Rebuild text without unmatched markers
    result: list[str] = []
    i = 0
    while i < len(text):
        if i in to_remove:
            i += 2  # skip the 2-byte marker
        else:
            result.append(text[i])
            i += 1

    return "".join(result), begins_removed, ends_removed


def process_file(
    filepath: str, dry_run: bool,
) -> tuple[int, int, list[str]]:
    """Process a single .cha file.

    Returns (begins_removed, ends_removed, change_descriptions).
    """
    path = Path(filepath)
    if not path.is_file():
        print(f"  WARNING: file not found: {filepath}")
        return 0, 0, []

    content = path.read_text(encoding="utf-8")
    lines = content.split("\n")

    total_begins = 0
    total_ends = 0
    changes: list[str] = []
    modified_lines: list[str] = []

    for line_num, line in enumerate(lines, 1):
        if UNDERLINE_BEGIN in line or UNDERLINE_END in line:
            fixed, begins, ends = fix_underline_markers(line)
            if begins > 0 or ends > 0:
                total_begins += begins
                total_ends += ends
                # Show readable version (replace control chars)
                readable_old = line.replace(UNDERLINE_BEGIN, "␂␁").replace(
                    UNDERLINE_END, "␂␂")
                readable_new = fixed.replace(UNDERLINE_BEGIN, "␂␁").replace(
                    UNDERLINE_END, "␂␂")
                changes.append(
                    f"  Line {line_num}: removed {begins} begin(s), "
                    f"{ends} end(s)\n"
                    f"    OLD: {readable_old[:120]}\n"
                    f"    NEW: {readable_new[:120]}"
                )
                modified_lines.append(fixed)
            else:
                modified_lines.append(line)
        else:
            modified_lines.append(line)

    if (total_begins > 0 or total_ends > 0) and not dry_run:
        new_content = "\n".join(modified_lines)
        path.write_text(new_content, encoding="utf-8")

    return total_begins, total_ends, changes


def main() -> None:
    """Run main."""
    parser = argparse.ArgumentParser(
        description="Fix unmatched underline markers in CA CHAT files")
    parser.add_argument(
        "--data-root", type=Path, default=DATA_ROOT,
        help=f"Corpus data root (default: {DATA_ROOT}; env: TALKBANK_DATA_ROOT)")
    parser.add_argument(
        "--dry-run", action="store_true",
        help="Show changes without writing")
    parser.add_argument(
        "--files", nargs="*",
        help="Specific files to process (default: all affected files)")
    args = parser.parse_args()

    files = args.files if args.files else default_affected_files(args.data_root)

    print(f"Processing {len(files)} files...")
    if args.dry_run:
        print("(DRY RUN — no files will be modified)\n")

    grand_begins = 0
    grand_ends = 0
    files_modified = 0

    for filepath in files:
        begins, ends, changes = process_file(filepath, args.dry_run)
        if begins > 0 or ends > 0:
            files_modified += 1
            grand_begins += begins
            grand_ends += ends
            rel = Path(filepath).name
            print(f"\n{rel}: {begins} unmatched begin(s), "
                  f"{ends} unmatched end(s)")
            for change in changes:
                print(change)

    print(f"\n{'=' * 60}")
    print(f"Summary:")
    print(f"  Files checked:  {len(files)}")
    print(f"  Files modified: {files_modified}")
    print(f"  Begins removed: {grand_begins} (E356)")
    print(f"  Ends removed:   {grand_ends} (E357)")
    if args.dry_run:
        print("  (DRY RUN — no changes written)")
    print(f"{'=' * 60}")

    if grand_begins == 0 and grand_ends == 0:
        print("\nNo unmatched markers found.")


if __name__ == "__main__":
    main()
