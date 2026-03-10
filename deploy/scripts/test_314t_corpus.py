"""Process a set of CHAT files with morphotag on free-threaded Python.

Bypasses the CLI daemon (which needs onnxruntime etc.) and uses StanzaEngine
directly with ThreadPoolExecutor to simulate the production dispatch path.

Usage:
    PYTHON_GIL=0 .venv-314t/bin/python scripts/test_314t_corpus.py
"""

from __future__ import annotations

import sys
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

print(f"Python {sys.version}")
print(f"Free-threaded: {not sys._is_gil_enabled()}")
print()

import batchalign_core  # noqa: E402
from batchalign.pipelines.morphosyntax.engine import StanzaEngine  # noqa: E402

# --- Load engine (shared across all threads) ---
engine = StanzaEngine()
engine.warmup()
print("Engine loaded.\n")

# --- Read file list ---
file_list = Path("/tmp/test_files.txt").read_text().strip().splitlines()
output_dir = Path("/tmp/314t-corpus")
output_dir.mkdir(exist_ok=True)

print(f"Processing {len(file_list)} files with {4} threads...\n")


def process_one(input_path: str) -> str:
    """Process a single CHAT file through morphotag."""
    chat_text = Path(input_path).read_text()
    handle = batchalign_core.ParsedChat.parse_lenient(chat_text)
    engine.process_handle(handle, lang="eng")
    result = handle.serialize()
    out_name = Path(input_path).name
    out_path = output_dir / out_name
    out_path.write_text(result)
    return out_name


# --- Process with ThreadPoolExecutor (the free-threading path) ---
t0 = time.monotonic()
with ThreadPoolExecutor(max_workers=4) as pool:
    futures = [pool.submit(process_one, f) for f in file_list]
    for fut in futures:
        name = fut.result()
        print(f"  Done: {name}")

elapsed = time.monotonic() - t0
print(f"\nAll {len(file_list)} files processed in {elapsed:.1f}s")

# --- Memory ---
import resource  # noqa: E402
rss_mb = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / (1024 * 1024)
print(f"Peak RSS: {rss_mb:.0f} MB")
