#!/usr/bin/env python3
"""Submit %wor rerun jobs to the batchalign server.

Reads directory paths from docs/wor-rerun-dirs.txt, groups them by
collection (the component under DATA_ROOT), discovers .cha files,
filters out dummy files, and submits one job per collection to the server.
Then polls all jobs until completion, fetching results and writing them
back to the original file paths in place.

Usage:
    python scripts/wor_rerun.py
    python scripts/wor_rerun.py --dry-run
    python scripts/wor_rerun.py --collection aphasia-data
    python scripts/wor_rerun.py --server http://other-host:8000
"""

from __future__ import annotations

import argparse
import os
import sys
import time
from collections import defaultdict
from pathlib import Path

import requests

os.environ["BATCHALIGN_NO_BROWSER"] = "1"

DATA_ROOT = os.path.abspath(os.path.expanduser(
    os.environ.get("TALKBANK_DATA_ROOT", "~/data")))
DIRS_FILE = os.path.join(os.path.dirname(__file__), "..", "docs", "wor-rerun-dirs.txt")
FILES_LIST = os.path.join(os.path.dirname(__file__), "..", "docs", "wor-broken-files.txt")
MAX_FILES_PER_JOB = 5000
POLL_INTERVAL_MIN = 1.0
POLL_INTERVAL_MAX = 10.0
POLL_INTERVAL_STEP = 1.0
MAX_POLL_FAILURES = 20

DUMMY_MARKERS = (
    "@Options:\tdummy",
    "This is a dummy file to permit playback from the TalkBank browser",
)


def read_dirs(dirs_file: str) -> list[str]:
    """Read directory paths from the dirs file, stripping blanks."""
    with open(dirs_file, "r", encoding="utf-8") as f:
        return [line.strip() for line in f if line.strip()]


def extract_collection(dir_path: str) -> str:
    """Extract the collection name from a path under DATA_ROOT.

    For example, /path/to/data/aphasia-data/English/Protocol/ACWT/PWA
    returns 'aphasia-data'.
    """
    rel = os.path.relpath(dir_path, DATA_ROOT)
    parts = Path(rel).parts
    if not parts:
        raise ValueError(f"Cannot extract collection from {dir_path}")
    return parts[0]


def group_by_collection(dirs: list[str]) -> dict[str, list[str]]:
    """Group directory paths by their collection name."""
    groups: dict[str, list[str]] = defaultdict(list)
    for d in dirs:
        collection = extract_collection(d)
        groups[collection].append(d)
    return dict(groups)


def is_dummy(content: str) -> bool:
    """Check if CHAT file content is a dummy file."""
    for marker in DUMMY_MARKERS:
        if marker in content:
            return True
    return False


def discover_cha_files(dirs: list[str]) -> list[str]:
    """Discover all .cha files in the given directories (non-recursive)."""
    files: list[str] = []
    for d in dirs:
        if not os.path.isdir(d):
            print(f"  WARNING: directory does not exist: {d}")
            continue
        for entry in sorted(os.listdir(d)):
            if entry.lower().endswith(".cha"):
                files.append(os.path.join(d, entry))
    return files


def build_file_payloads(
    cha_files: list[str], collection: str,
) -> tuple[list[dict[str, str]], dict[str, str]]:
    """Read .cha files, skip dummies, return (payloads, filename_to_fullpath_map).

    The filename in each payload is relative to DATA_ROOT/collection.
    """
    collection_root = os.path.join(DATA_ROOT, collection)
    payloads: list[dict[str, str]] = []
    path_map: dict[str, str] = {}  # relative filename -> full path
    skipped_dummy = 0
    skipped_read_error = 0

    for full_path in cha_files:
        try:
            with open(full_path, "r", encoding="utf-8") as f:
                content = f.read()
        except Exception as exc:
            print(f"  WARNING: cannot read {full_path}: {exc}")
            skipped_read_error += 1
            continue

        if is_dummy(content):
            skipped_dummy += 1
            continue

        rel = os.path.relpath(full_path, collection_root)
        payloads.append({"filename": rel, "content": content})
        path_map[rel] = full_path

    if skipped_dummy:
        print(f"  Skipped {skipped_dummy} dummy file(s)")
    if skipped_read_error:
        print(f"  Skipped {skipped_read_error} unreadable file(s)")

    return payloads, path_map


def submit_job(
    server: str,
    collection: str,
    payloads: list[dict[str, str]],
    lang: str = "eng",
) -> str:
    """Submit one align job and return its server-assigned `job_id`."""
    body = {
        "command": "align",
        "lang": lang,
        "num_speakers": 1,
        "files": payloads,
        "media_files": [],
        "media_mapping": collection,
        "media_subdir": "",
        "source_dir": os.path.join(DATA_ROOT, collection),
        "options": {},
    }

    resp = requests.post(f"{server}/jobs", json=body, timeout=60)
    resp.raise_for_status()
    info = resp.json()
    return info["job_id"]


def poll_and_write(
    server: str,
    jobs: list[tuple[str, str, dict[str, str]]],
) -> tuple[int, int, int]:
    """Poll all jobs concurrently until done, writing results back in place.

    jobs: list of (job_id, collection, filename_to_fullpath_map)
    Returns (total_written, total_errors, total_skipped).
    """
    # Track state per job
    class JobState:
        """Mutable polling/writing counters for one submitted job."""

        def __init__(self, job_id: str, collection: str,
                     path_map: dict[str, str]) -> None:
            """Initialize per-job bookkeeping used by the polling loop."""
            self.job_id = job_id
            self.collection = collection
            self.path_map = path_map
            self.written: set[str] = set()
            self.written_count = 0
            self.error_count = 0
            self.done = False
            self.consecutive_failures = 0
            self.total_files = len(path_map)

    states = [JobState(jid, col, pm) for jid, col, pm in jobs]
    poll_interval = POLL_INTERVAL_MIN

    while any(not s.done for s in states):
        progress_changed = False

        for st in states:
            if st.done:
                continue

            # Poll job status
            try:
                resp = requests.get(
                    f"{server}/jobs/{st.job_id}", timeout=15)
                resp.raise_for_status()
            except Exception as exc:
                st.consecutive_failures += 1
                if st.consecutive_failures >= MAX_POLL_FAILURES:
                    print(f"\n  ERROR: Lost connection to server for job "
                          f"{st.job_id} ({st.collection}): {exc}")
                    st.error_count += st.total_files - st.written_count - st.error_count
                    st.done = True
                continue

            st.consecutive_failures = 0
            info = resp.json()
            status = info["status"]

            # Check for newly completed files
            for entry in info.get("file_statuses", []):
                fn = entry["filename"]
                if fn in st.written:
                    continue

                if entry["status"] == "done":
                    # Fetch the result
                    ok = _fetch_and_write_one(
                        server, st.job_id, fn, st.path_map)
                    st.written.add(fn)
                    if ok:
                        st.written_count += 1
                        progress_changed = True
                    else:
                        st.error_count += 1

                elif entry["status"] == "error":
                    st.written.add(fn)
                    st.error_count += 1
                    progress_changed = True
                    err_msg = entry.get("error", "unknown")
                    full_path = st.path_map.get(fn, fn)
                    print(f"\n  ERROR [{st.collection}] {full_path}: {err_msg}")

            if status in ("completed", "failed", "cancelled", "interrupted"):
                st.done = True
                progress_changed = True

        # Print progress line
        parts = []
        for st in states:
            done_count = st.written_count + st.error_count
            parts.append(f"{st.collection}: {done_count}/{st.total_files}")
        line = "  " + " | ".join(parts)
        print(f"\r{line}", end="", flush=True)

        # Adaptive backoff
        if progress_changed:
            poll_interval = POLL_INTERVAL_MIN
        else:
            poll_interval = min(
                poll_interval + POLL_INTERVAL_STEP, POLL_INTERVAL_MAX)

        if any(not s.done for s in states):
            time.sleep(poll_interval)

    print()  # newline after progress

    total_written = sum(s.written_count for s in states)
    total_errors = sum(s.error_count for s in states)
    total_skipped = 0
    return total_written, total_errors, total_skipped


def _fetch_and_write_one(
    server: str, job_id: str, filename: str,
    path_map: dict[str, str],
) -> bool:
    """Fetch a single file result from the server and write it back in place.

    Returns `True` on success and `False` when fetch/validation/write fails.
    """
    try:
        resp = requests.get(
            f"{server}/jobs/{job_id}/results/{filename}", timeout=30)
        resp.raise_for_status()
    except Exception as exc:
        full_path = path_map.get(filename, filename)
        print(f"\n  ERROR: Failed to fetch result for {full_path}: {exc}")
        return False

    result = resp.json()

    if result.get("error"):
        full_path = path_map.get(filename, filename)
        print(f"\n  ERROR {full_path}: {result['error']}")
        return False

    # Map filename back to full path and write
    full_path = path_map.get(filename)
    if full_path is None:
        # Try the result filename (server may have changed it)
        result_fn = result.get("filename", filename)
        full_path = path_map.get(result_fn)
        if full_path is None:
            print(f"\n  WARNING: Cannot map result filename '{filename}' "
                  f"back to original path, skipping")
            return False

    content = result.get("content", "")
    if not content:
        print(f"\n  WARNING: Empty result for {full_path}, skipping")
        return False

    try:
        with open(full_path, "w", encoding="utf-8") as f:
            f.write(content)
    except Exception as exc:
        print(f"\n  ERROR: Failed to write {full_path}: {exc}")
        return False

    return True


def chunk_payloads(
    payloads: list[dict[str, str]],
    path_map: dict[str, str],
    max_per_job: int,
) -> list[tuple[list[dict[str, str]], dict[str, str]]]:
    """Split payloads into chunks of at most max_per_job."""
    if len(payloads) <= max_per_job:
        return [(payloads, path_map)]

    chunks: list[tuple[list[dict[str, str]], dict[str, str]]] = []
    for i in range(0, len(payloads), max_per_job):
        chunk = payloads[i:i + max_per_job]
        chunk_map = {p["filename"]: path_map[p["filename"]] for p in chunk}
        chunks.append((chunk, chunk_map))
    return chunks


def read_file_list(files_file: str) -> list[str]:
    """Read individual file paths from a file list, stripping blanks/comments."""
    with open(files_file, "r", encoding="utf-8") as f:
        return [line.strip() for line in f
                if line.strip() and not line.strip().startswith("#")]


def group_files_by_collection(
    file_paths: list[str],
) -> dict[str, list[str]]:
    """Group individual file paths by their collection name.

    Returns `{collection: [file_path, ...]}`.
    """
    groups: dict[str, list[str]] = defaultdict(list)
    for fp in file_paths:
        collection = extract_collection(os.path.dirname(fp))
        groups[collection].append(fp)
    return dict(groups)


def main() -> None:
    """Submit `%wor` rerun jobs, stream progress, and write completed results in place."""
    global DATA_ROOT
    parser = argparse.ArgumentParser(
        description="Submit %wor rerun jobs to the batchalign server.")
    parser.add_argument(
        "--server", default="http://net:8000",
        help="Server URL (default: http://net:8000)")
    parser.add_argument(
        "--data-root", default=DATA_ROOT,
        help=f"Corpus data root (default: {DATA_ROOT}; env: TALKBANK_DATA_ROOT)")
    parser.add_argument(
        "--collection", default=None,
        help="Process only this collection (for testing)")
    parser.add_argument(
        "--dry-run", action="store_true",
        help="Show what would be submitted without actually submitting")
    parser.add_argument(
        "--lang", default="eng",
        help="Language code (default: eng)")
    parser.add_argument(
        "--files", default=None,
        help="File containing individual .cha paths (one per line). "
             "If not specified, uses the dirs file.")
    args = parser.parse_args()
    DATA_ROOT = os.path.abspath(os.path.expanduser(args.data_root))

    start_time = time.time()

    # Determine input mode: individual files or directories
    if args.files:
        files_file = os.path.normpath(args.files)
        if not os.path.isfile(files_file):
            print(f"ERROR: files list not found: {files_file}")
            sys.exit(1)
        all_files = read_file_list(files_file)
        print(f"Read {len(all_files)} file paths from {files_file}")
        file_groups = group_files_by_collection(all_files)
        print(f"Found {len(file_groups)} collection(s): "
              f"{', '.join(sorted(file_groups.keys()))}")
        use_file_list = True
    else:
        # Resolve dirs file path
        dirs_file = os.path.normpath(DIRS_FILE)
        if not os.path.isfile(dirs_file):
            print(f"ERROR: dirs file not found: {dirs_file}")
            sys.exit(1)
        dirs = read_dirs(dirs_file)
        print(f"Read {len(dirs)} directories from {dirs_file}")
        file_groups = {}
        use_file_list = False

    if not use_file_list:
        # 2. Group by collection (directory mode)
        groups = group_by_collection(dirs)
        print(f"Found {len(groups)} collection(s): "
              f"{', '.join(sorted(groups.keys()))}")
    else:
        groups = file_groups

    if args.collection:
        if args.collection not in groups:
            print(f"ERROR: collection '{args.collection}' not found. "
                  f"Available: {', '.join(sorted(groups.keys()))}")
            sys.exit(1)
        groups = {args.collection: groups[args.collection]}
        print(f"Filtering to collection: {args.collection}")

    # 3. Health check
    if not args.dry_run:
        try:
            resp = requests.get(f"{args.server}/health", timeout=10)
            resp.raise_for_status()
            health = resp.json()
            print(f"\nServer: {args.server} (v{health.get('version', '?')})")
            print(f"  Media mappings: {health.get('media_mapping_keys', [])}")
            print(f"  Workers available: {health.get('workers_available', '?')}")
            print(f"  Active jobs: {health.get('active_jobs', '?')}")

            # Verify all collections have media mappings
            mapping_keys = health.get("media_mapping_keys", [])
            for col in sorted(groups.keys()):
                if col not in mapping_keys:
                    print(f"  WARNING: collection '{col}' not in server's "
                          f"media_mapping_keys")
        except Exception as exc:
            print(f"ERROR: Cannot reach server at {args.server}: {exc}")
            sys.exit(1)

    # 4. Discover files and build payloads per collection
    all_jobs: list[tuple[str, str, dict[str, str]]] = []
    # (job_id, collection, path_map)
    total_files_discovered = 0
    total_files_to_submit = 0

    print()
    for collection in sorted(groups.keys()):
        col_items = groups[collection]

        if use_file_list:
            # File list mode: items are individual file paths
            cha_files = [f for f in col_items if os.path.isfile(f)]
            print(f"[{collection}] {len(cha_files)} file(s)")
        else:
            # Directory mode: items are directory paths
            print(f"[{collection}] {len(col_items)} directories")
            cha_files = discover_cha_files(col_items)
            print(f"  Found {len(cha_files)} .cha file(s)")

        total_files_discovered += len(cha_files)

        if not cha_files:
            continue

        # Build payloads (reads files, skips dummies)
        payloads, path_map = build_file_payloads(cha_files, collection)
        total_files_to_submit += len(payloads)
        print(f"  {len(payloads)} file(s) to submit")

        if not payloads:
            continue

        if args.dry_run:
            # Show a sample of files
            sample = payloads[:5]
            for p in sample:
                print(f"    {p['filename']}")
            if len(payloads) > 5:
                print(f"    ... and {len(payloads) - 5} more")
            continue

        # Split into chunks if needed (max 5000 per job)
        chunks = chunk_payloads(payloads, path_map, MAX_FILES_PER_JOB)

        for i, (chunk, chunk_map) in enumerate(chunks):
            suffix = f" (part {i + 1}/{len(chunks)})" if len(chunks) > 1 else ""
            try:
                job_id = submit_job(args.server, collection, chunk, args.lang)
                print(f"  Submitted job {job_id} "
                      f"({len(chunk)} files){suffix}")
                all_jobs.append((job_id, collection, chunk_map))
            except Exception as exc:
                print(f"  ERROR: Failed to submit {collection}{suffix}: {exc}")
                # Try to print server error detail
                if hasattr(exc, "response") and exc.response is not None:  # type: ignore[union-attr]
                    try:
                        detail = exc.response.json().get("detail", "")  # type: ignore[union-attr]
                        if detail:
                            print(f"    Detail: {detail}")
                    except Exception:
                        pass

    print(f"\nTotal: {total_files_discovered} .cha files discovered, "
          f"{total_files_to_submit} to submit")

    if args.dry_run:
        print("\nDry run complete. No jobs submitted.")
        return

    if not all_jobs:
        print("\nNo jobs submitted.")
        return

    print(f"\n{len(all_jobs)} job(s) submitted. Polling for results...\n")

    # 5. Poll all jobs until done
    total_written, total_errors, _ = poll_and_write(
        args.server, all_jobs)

    # 6. Summary
    elapsed = time.time() - start_time
    minutes = int(elapsed // 60)
    seconds = elapsed % 60

    print(f"\n{'=' * 60}")
    print(f"Summary:")
    print(f"  Collections: {len(groups)}")
    print(f"  Jobs submitted: {len(all_jobs)}")
    print(f"  Files written: {total_written}")
    print(f"  Errors: {total_errors}")
    if minutes > 0:
        print(f"  Time: {minutes}m {seconds:.1f}s")
    else:
        print(f"  Time: {seconds:.1f}s")
    print(f"{'=' * 60}")

    if total_errors:
        sys.exit(1)


if __name__ == "__main__":
    main()
