#!/usr/bin/env python3
"""Extract all files with %wor errors from the corpus validation log.

Parses a validation log (default: ~/test.log) to extract files with E342 (trailing bullet) and E601
(complex %wor) errors, cross-references with the decode()-bug file list
(docs/wor-broken-files.txt), and produces a unified report.

Output:
    results/wor_errors/all_wor_files.txt — one path per line (for --file-list)
    results/wor_errors/summary.md        — boss-ready collection breakdown
    results/wor_errors/by_collection/    — per-collection file lists

Usage:
    uv run python scripts/extract_wor_error_files.py
    uv run python scripts/extract_wor_error_files.py --log ~/test.log
"""

from __future__ import annotations

import argparse
import os
import re
import sys
from collections import defaultdict
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
PROJECT_DIR = SCRIPT_DIR.parent
DATA_ROOT = Path(os.environ.get("TALKBANK_DATA_ROOT", str(Path.home() / "data"))).expanduser()
DEFAULT_LOG = Path.home() / "test.log"
DECODE_BUG_FILE = PROJECT_DIR / "docs" / "wor-broken-files.txt"
OUTPUT_DIR = PROJECT_DIR / "results" / "wor_errors"

# Error codes that indicate %wor problems
WOR_ERROR_CODES = {"E342", "E601", "E714", "E715"}

# Regex to match error/warning file header lines
FILE_HEADER_RE = re.compile(
    r"^[✗✓] (?:Errors found in|Valid:) (.+\.cha)$"
)

# Regex to match error code lines like "E342 (https://talkbank.org/errors/E342)"
ERROR_CODE_RE = re.compile(r"^(E\d{3}|W\d{3}) \(")


def parse_log(log_path: Path) -> dict[str, set[str]]:
    """Parse the validation log, returning {filepath: set(error_codes)}.

    Only collects files with WOR_ERROR_CODES.
    """
    file_errors: dict[str, set[str]] = defaultdict(set)
    current_file: str | None = None

    with open(log_path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")

            # Check for file header
            if line.startswith("✗ Errors found in "):
                current_file = line[len("✗ Errors found in "):]
                continue

            # Check for error code
            m = ERROR_CODE_RE.match(line)
            if m and current_file is not None:
                code = m.group(1)
                if code in WOR_ERROR_CODES:
                    file_errors[current_file].add(code)

    return dict(file_errors)


def read_decode_bug_files(path: Path) -> set[str]:
    """Read the decode()-bug file list (one path per line)."""
    if not path.is_file():
        print(f"WARNING: decode-bug file list not found: {path}")
        return set()

    files: set[str] = set()
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if line and not line.startswith("#"):
                files.add(line)
    return files


def extract_collection(filepath: str) -> str:
    """Extract collection name from a path under DATA_ROOT.

    e.g., /path/to/data/aphasia-data/English/... -> aphasia-data
    """
    try:
        rel = os.path.relpath(filepath, DATA_ROOT)
        return Path(rel).parts[0]
    except (ValueError, IndexError):
        return "unknown"


def extract_corpus_path(filepath: str) -> str:
    """Extract corpus path relative to collection.

    e.g., /path/to/data/aphasia-data/English/Protocol/ACWT/PWA/file.cha
    -> English/Protocol/ACWT/PWA
    """
    try:
        rel = os.path.relpath(filepath, DATA_ROOT)
        parts = Path(rel).parts
        # collection/rest.../filename -> rest...
        if len(parts) >= 3:
            return str(Path(*parts[1:-1]))
        return str(Path(*parts[1:])) if len(parts) >= 2 else ""
    except (ValueError, IndexError):
        return ""


def read_media_header(filepath: str) -> tuple[str, str]:
    """Read @Media header from a .cha file.

    Returns (media_filename, media_type) or ("", "") if not found.
    """
    try:
        with open(filepath, "r", encoding="utf-8") as f:
            for line in f:
                if line.startswith("@Media:"):
                    # @Media:\tfilename, type
                    content = line.split("\t", 1)[1].strip() if "\t" in line else ""
                    parts = [p.strip() for p in content.split(",")]
                    if len(parts) >= 2:
                        return parts[0], parts[1]
                    elif len(parts) == 1:
                        return parts[0], ""
                    break
                # Stop searching after @Begin (headers are before content)
                if line.startswith("*"):
                    break
    except (OSError, UnicodeDecodeError):
        pass
    return "", ""


def main() -> None:
    """Build unified `%wor` rerun inputs and a management-facing summary report."""
    global DATA_ROOT
    parser = argparse.ArgumentParser(
        description="Extract %wor error files from validation log")
    parser.add_argument(
        "--log", type=Path, default=DEFAULT_LOG,
        help=f"Path to validation log (default: {DEFAULT_LOG})")
    parser.add_argument(
        "--data-root", type=Path, default=DATA_ROOT,
        help=f"Corpus data root (default: {DATA_ROOT}; env: TALKBANK_DATA_ROOT)")
    parser.add_argument(
        "--no-media-check", action="store_true",
        help="Skip reading @Media headers from .cha files")
    args = parser.parse_args()
    DATA_ROOT = args.data_root.expanduser()

    if not args.log.is_file():
        print(f"ERROR: log file not found: {args.log}")
        sys.exit(1)

    # --- Step 1: Parse log for E342/E601 files ---
    print(f"Parsing {args.log}...")
    log_errors = parse_log(args.log)
    print(f"  Found {len(log_errors)} files with %wor errors in log")

    e342_files = {f for f, codes in log_errors.items() if "E342" in codes}
    e601_files = {f for f, codes in log_errors.items() if "E601" in codes}
    e714_files = {f for f, codes in log_errors.items() if "E714" in codes}
    e715_files = {f for f, codes in log_errors.items() if "E715" in codes}

    print(f"  E342 (trailing bullet): {len(e342_files)} files")
    print(f"  E601 (complex %wor):    {len(e601_files)} files")
    print(f"  E714 (missing words):   {len(e714_files)} files")
    print(f"  E715 (extra words):     {len(e715_files)} files")

    # --- Step 2: Read decode()-bug file list ---
    print(f"\nReading decode()-bug list from {DECODE_BUG_FILE}...")
    decode_bug_files = read_decode_bug_files(DECODE_BUG_FILE)
    print(f"  {len(decode_bug_files)} files in decode()-bug list")

    # --- Step 3: Build union ---
    log_wor_files = set(log_errors.keys())
    all_wor_files = log_wor_files | decode_bug_files

    # Calculate overlaps
    overlap_log_decode = log_wor_files & decode_bug_files
    only_in_log = log_wor_files - decode_bug_files
    only_in_decode = decode_bug_files - log_wor_files

    print(f"\n--- Overlap Analysis ---")
    print(f"  Files in log (E342/E601/E714/E715): {len(log_wor_files)}")
    print(f"  Files in decode()-bug list:          {len(decode_bug_files)}")
    print(f"  Overlap:                             {len(overlap_log_decode)}")
    print(f"  Only in log:                         {len(only_in_log)}")
    print(f"  Only in decode()-bug:                {len(only_in_decode)}")
    print(f"  UNION (total needing re-alignment):  {len(all_wor_files)}")

    # --- Step 4: Build per-file info ---
    print(f"\nBuilding file details...")

    # For each file: collection, corpus_path, error sources, media info
    file_info: list[dict[str, str]] = []
    collections: dict[str, list[dict[str, str]]] = defaultdict(list)

    for filepath in sorted(all_wor_files):
        collection = extract_collection(filepath)
        corpus_path = extract_corpus_path(filepath)
        filename = Path(filepath).name

        sources: list[str] = []
        if filepath in decode_bug_files:
            sources.append("decode-bug")
        error_codes = log_errors.get(filepath, set())
        if "E342" in error_codes:
            sources.append("E342")
        if "E601" in error_codes:
            sources.append("E601")
        if "E714" in error_codes:
            sources.append("E714")
        if "E715" in error_codes:
            sources.append("E715")

        media_name = ""
        media_type = ""
        if not args.no_media_check:
            media_name, media_type = read_media_header(filepath)

        info = {
            "path": filepath,
            "collection": collection,
            "corpus_path": corpus_path,
            "filename": filename,
            "sources": ",".join(sources),
            "error_codes": ",".join(sorted(error_codes)),
            "media_name": media_name,
            "media_type": media_type,
        }
        file_info.append(info)
        collections[collection].append(info)

    # --- Step 5: Write outputs ---
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    by_collection_dir = OUTPUT_DIR / "by_collection"
    by_collection_dir.mkdir(parents=True, exist_ok=True)

    # 5a. All files list (for --file-list)
    all_files_path = OUTPUT_DIR / "all_wor_files.txt"
    with open(all_files_path, "w", encoding="utf-8") as f:
        for info in file_info:
            f.write(info["path"] + "\n")
    print(f"\nWrote {len(file_info)} file paths to {all_files_path}")

    # 5b. Per-collection file lists
    for collection in sorted(collections.keys()):
        col_files = collections[collection]
        col_path = by_collection_dir / f"{collection}.txt"
        with open(col_path, "w", encoding="utf-8") as f:
            for info in col_files:
                f.write(info["path"] + "\n")
        print(f"  {collection}: {len(col_files)} files -> {col_path.name}")

    # 5c. Summary report
    summary_path = OUTPUT_DIR / "summary.md"
    with open(summary_path, "w", encoding="utf-8") as f:
        _write_summary(f, file_info, collections, log_errors,
                       decode_bug_files, e342_files, e601_files)
    print(f"\nWrote summary to {summary_path}")

    print(f"\nDone. {len(all_wor_files)} total files need %wor regeneration.")


def _write_summary(
    f: object,
    file_info: list[dict[str, str]],
    collections: dict[str, list[dict[str, str]]],
    log_errors: dict[str, set[str]],
    decode_bug_files: set[str],
    e342_files: set[str],
    e601_files: set[str],
) -> None:
    """Write the boss-ready summary report."""
    w = f.write  # type: ignore[union-attr]

    total = len(file_info)
    n_decode = len(decode_bug_files)
    n_e342 = len(e342_files)
    n_e601 = len(e601_files)

    w("# %wor Tier Error Report: Files Needing Re-Alignment\n\n")

    w("## Executive Summary\n\n")
    w(f"**{total:,} CHAT files** across **{len(collections)} collections** ")
    w("have incorrect `%wor` (word-level timing) tiers that need to be ")
    w("regenerated by re-running forced alignment with the corrected Rust backend.\n\n")

    w("Three independent sources of %wor errors were identified and cross-referenced:\n\n")
    w(f"1. **Python `decode()` bug** ({n_decode:,} files) — the old Python lexer leaked ")
    w("non-lexical material (nonwords, untranscribed tokens) into %wor tiers. ")
    w("See `docs/wor-tier-bug-report.md` for the full analysis.\n")
    w(f"2. **Trailing bullet after terminator** ({n_e342} files, E342) — old tools put ")
    w("the utterance-level timing bullet after the %wor terminator (e.g., ")
    w("`gram . •123_456•`), which the parser rejects.\n")
    w(f"3. **Complex %wor content** ({n_e601} files, E601) — legacy CLAN data with ")
    w("paralinguistic markers, gesture annotations, or interlocutor quotes in %wor.\n\n")

    # Overlap table
    log_wor = set(log_errors.keys())
    overlap = log_wor & decode_bug_files
    only_log = log_wor - decode_bug_files
    only_decode = decode_bug_files - log_wor

    w("### Overlap Between Error Sources\n\n")
    w("| Source | Files |\n")
    w("|--------|------:|\n")
    w(f"| decode()-bug list (E714/E715) | {n_decode:,} |\n")
    w(f"| Log E342 (trailing bullet) | {n_e342} |\n")
    w(f"| Log E601 (complex %wor) | {n_e601} |\n")
    w(f"| Overlap (in both log + decode list) | {len(overlap)} |\n")
    w(f"| Only in log (not in decode list) | {len(only_log)} |\n")
    w(f"| Only in decode list (not in log) | {len(only_decode)} |\n")
    w(f"| **Union (total)** | **{total:,}** |\n\n")

    # Collection breakdown
    w("## Collection Breakdown\n\n")
    w("| Collection | Files | decode-bug | E342 | E601 | Media Types |\n")
    w("|------------|------:|-----------:|-----:|-----:|-------------|\n")

    for collection in sorted(collections.keys()):
        col_files = collections[collection]
        n_col = len(col_files)
        n_col_decode = sum(
            1 for fi in col_files if "decode-bug" in fi["sources"])
        n_col_e342 = sum(1 for fi in col_files if "E342" in fi["sources"])
        n_col_e601 = sum(1 for fi in col_files if "E601" in fi["sources"])

        media_types: set[str] = set()
        for fi in col_files:
            if fi["media_type"]:
                media_types.add(fi["media_type"])
        media_str = ", ".join(sorted(media_types)) if media_types else "—"

        w(f"| {collection} | {n_col:,} | {n_col_decode:,} | "
          f"{n_col_e342} | {n_col_e601} | {media_str} |\n")

    w(f"| **Total** | **{total:,}** | **{n_decode:,}** | "
      f"**{n_e342}** | **{n_e601}** | |\n\n")

    # Top corpora
    w("## Most Affected Corpora (top 25)\n\n")
    w("| Corpus | Collection | Files | Error Sources |\n")
    w("|--------|------------|------:|---------------|\n")

    corpus_counts: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    for fi in file_info:
        key = (fi["collection"], fi["corpus_path"])
        corpus_counts[key].append(fi)

    top_corpora = sorted(
        corpus_counts.items(), key=lambda x: len(x[1]), reverse=True)[:25]

    for (collection, corpus_path), files in top_corpora:
        n = len(files)
        all_sources: set[str] = set()
        for fi in files:
            for s in fi["sources"].split(","):
                if s:
                    all_sources.add(s)
        sources_str = ", ".join(sorted(all_sources))
        w(f"| {corpus_path} | {collection} | {n} | {sources_str} |\n")

    # Media type summary
    w("\n## Media Type Summary\n\n")
    media_counts: dict[str, int] = defaultdict(int)
    no_media = 0
    for fi in file_info:
        if fi["media_type"]:
            media_counts[fi["media_type"]] += 1
        else:
            no_media += 1

    w("| Media Type | Files |\n")
    w("|------------|------:|\n")
    for mt in sorted(media_counts.keys()):
        w(f"| {mt} | {media_counts[mt]:,} |\n")
    if no_media:
        w(f"| (no @Media header) | {no_media} |\n")
    w(f"| **Total** | **{total:,}** |\n\n")

    # Action items
    w("## Recommended Action\n\n")
    w("Re-run `batchalign align` on all listed files using the Rust backend. ")
    w("The corrected Rust implementation:\n\n")
    w("1. Excludes nonwords and untranscribed material from %wor (fixes decode() bug)\n")
    w("2. Generates correct %wor format without trailing bullets (fixes E342)\n")
    w("3. Produces flat %wor without paralinguistic markers (fixes E601)\n\n")
    w("Use `scripts/wor_rerun.py --files results/wor_errors/all_wor_files.txt` ")
    w("to submit all files to the server for re-alignment.\n\n")

    w("Per-collection file lists are in `results/wor_errors/by_collection/` ")
    w("for targeted processing.\n\n")

    # Error source descriptions
    w("## Error Source Details\n\n")

    w("### 1. Python decode() Bug (E714/E715)\n\n")
    w("The old Python batchalign's lexer had a `decode()` function that blindly ")
    w("overwrote inner token type classifications when flattening bracketed groups. ")
    w("This caused nonwords (`&~li`), untranscribed tokens (`xxx`, `www`), and ")
    w("phonological fragments (`&+fr`) inside retrace groups (`<...> [/]`) to leak ")
    w("into %wor tiers. Full analysis in `docs/wor-tier-bug-report.md`.\n\n")

    w("### 2. Trailing Bullet After Terminator (E342)\n\n")
    w("Old alignment tools placed the utterance-level timing bullet after the %wor ")
    w("terminator instead of before it:\n\n")
    w("```\n")
    w("# Wrong (E342)\n")
    w("%wor:\tgram . •362149_362269•\n")
    w("\n")
    w("# Correct\n")
    w("%wor:\tgram •362149_362269• .\n")
    w("```\n\n")
    w("The parser expects the terminator to be the final token. The trailing bullet ")
    w("triggers a tree-sitter error recovery path.\n\n")

    w("### 3. Complex %wor Content (E601)\n\n")
    w("Legacy CLAN data contains paralinguistic annotations in %wor that ")
    w("violate the flat structure:\n\n")
    w("```\n")
    w("# E601 examples\n")
    w("%wor:\t&*PAR:wow take 123_456 ...\n")
    w("%wor:\t&=ges:fall and 123_456 ...\n")
    w("%wor:\t&=finger:counts two 123_456 ...\n")
    w("```\n\n")
    w("`%wor` must be flat: just words and inline timing bullets. These paralinguistic ")
    w("markers (`&*`, `&=`) are not valid in %wor.\n")


if __name__ == "__main__":
    main()
