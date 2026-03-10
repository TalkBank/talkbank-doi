"""Test free-threaded Python 3.14t with multi-threaded Stanza inference.

Measures whether Stanza Pipeline can safely run in parallel threads
when the GIL is removed. Three test modes:

  1. Sequential baseline — single-thread, no concurrency
  2. Threads + lock — ThreadPoolExecutor, serialized via Lock (shared memory only)
  3. Threads + no lock — ThreadPoolExecutor, true parallel inference

Usage:
    PYTHON_GIL=0 .venv-314t/bin/python scripts/test_314t_threading.py
"""

from __future__ import annotations

import json
import sys
import threading
import time
from concurrent.futures import ThreadPoolExecutor

# ---------------------------------------------------------------------------
# Check environment
# ---------------------------------------------------------------------------

if not hasattr(sys, "_is_gil_enabled"):
    print("WARNING: Not a free-threaded Python build. Results will reflect GIL behavior.")
    free_threaded = False
else:
    free_threaded = not sys._is_gil_enabled()

print(f"Python {sys.version}")
print(f"Free-threaded: {free_threaded}")
print()

# ---------------------------------------------------------------------------
# Load Stanza pipeline (once, shared across threads)
# ---------------------------------------------------------------------------

import stanza  # noqa: E402

print("Loading Stanza English pipeline...")
t0 = time.monotonic()
nlp = stanza.Pipeline(
    "en",
    processors="tokenize,mwt,pos,lemma,depparse",
    tokenize_pretokenized=True,
    use_gpu=False,
    logging_level="WARN",
)
load_time = time.monotonic() - t0
print(f"Pipeline loaded in {load_time:.1f}s")
print(f"GIL enabled after load: {sys._is_gil_enabled() if hasattr(sys, '_is_gil_enabled') else 'N/A'}")
print()

# ---------------------------------------------------------------------------
# Prepare test data — real English utterances
# ---------------------------------------------------------------------------

UTTERANCES = [
    "the dog chased the cat around the yard",
    "I want to go to the store and buy some milk",
    "she was reading a book when the phone rang",
    "they have been working on the project for three months",
    "could you please pass me the salt",
    "the children were playing in the park after school",
    "he decided to take the train instead of driving",
    "we should probably leave before it starts raining",
    "the teacher explained the concept very clearly",
    "I do n't think that 's a good idea at all",
    "my brother gave me a present for my birthday",
    "the movie was better than I expected it to be",
    "she asked him if he wanted to join them for dinner",
    "the cat sat on the mat and licked its paws",
    "we went hiking in the mountains last weekend",
    "the old man sat on the bench and watched the sunset",
    "I need to finish this report by tomorrow morning",
    "she told me she had already finished her homework",
    "the baby started crying when the loud noise woke him",
    "they are planning to renovate the kitchen next month",
]

# Repeat to get enough work for meaningful timing
N_REPEATS = 5
utterances = UTTERANCES * N_REPEATS
N = len(utterances)
print(f"Test data: {N} utterances ({len(UTTERANCES)} unique x {N_REPEATS} repeats)")
print()


def run_stanza(text: str) -> list[dict[str, object]]:
    """Run Stanza pipeline on a single utterance, return UD dict."""
    doc = nlp(text)
    return doc.to_dict()


# ---------------------------------------------------------------------------
# Mode 1: Sequential baseline
# ---------------------------------------------------------------------------

print("=" * 60)
print("MODE 1: Sequential (single-threaded)")
print("=" * 60)

sequential_results: list[list[dict[str, object]]] = []
t0 = time.monotonic()
for utt in utterances:
    sequential_results.append(run_stanza(utt))
sequential_time = time.monotonic() - t0
print(f"  Time: {sequential_time:.3f}s ({N / sequential_time:.1f} utt/s)")
print()


# ---------------------------------------------------------------------------
# Mode 2: Threaded with lock (shared memory, serial inference)
# ---------------------------------------------------------------------------

lock = threading.Lock()


def run_stanza_locked(text: str) -> list[dict[str, object]]:
    """Run Stanza with a lock — serialized but shared memory."""
    with lock:
        doc = nlp(text)
    return doc.to_dict()


for num_threads in [2, 4]:
    print("=" * 60)
    print(f"MODE 2: Threaded + Lock ({num_threads} threads)")
    print("=" * 60)

    locked_results: list[list[dict[str, object]]] = [[] for _ in range(N)]
    t0 = time.monotonic()
    with ThreadPoolExecutor(max_workers=num_threads) as pool:
        futures = []
        for i, utt in enumerate(utterances):
            futures.append((i, pool.submit(run_stanza_locked, utt)))
        for i, fut in futures:
            locked_results[i] = fut.result()
    locked_time = time.monotonic() - t0

    # Verify correctness
    match = all(
        json.dumps(a, sort_keys=True) == json.dumps(b, sort_keys=True)
        for a, b in zip(sequential_results, locked_results)
    )

    print(f"  Time: {locked_time:.3f}s ({N / locked_time:.1f} utt/s)")
    print(f"  Speedup vs sequential: {sequential_time / locked_time:.2f}x")
    print(f"  Output matches sequential: {match}")
    print()


# ---------------------------------------------------------------------------
# Mode 3: Threaded WITHOUT lock (true parallel inference)
# ---------------------------------------------------------------------------

for num_threads in [2, 4]:
    print("=" * 60)
    print(f"MODE 3: Threaded + NO Lock ({num_threads} threads)")
    print("=" * 60)

    unlocked_results: list[list[dict[str, object]]] = [[] for _ in range(N)]
    crashed = False
    t0 = time.monotonic()
    try:
        with ThreadPoolExecutor(max_workers=num_threads) as pool:
            futures = []
            for i, utt in enumerate(utterances):
                futures.append((i, pool.submit(run_stanza, utt)))
            for i, fut in futures:
                unlocked_results[i] = fut.result()
        unlocked_time = time.monotonic() - t0

        # Verify correctness
        match = all(
            json.dumps(a, sort_keys=True) == json.dumps(b, sort_keys=True)
            for a, b in zip(sequential_results, unlocked_results)
        )

        print(f"  Time: {unlocked_time:.3f}s ({N / unlocked_time:.1f} utt/s)")
        print(f"  Speedup vs sequential: {sequential_time / unlocked_time:.2f}x")
        print(f"  Output matches sequential: {match}")
        if not match:
            # Find first mismatch
            for i, (a, b) in enumerate(zip(sequential_results, unlocked_results)):
                if json.dumps(a, sort_keys=True) != json.dumps(b, sort_keys=True):
                    print(f"  MISMATCH at utterance {i}: '{utterances[i]}'")
                    break
    except Exception as e:
        unlocked_time = time.monotonic() - t0
        crashed = True
        print(f"  CRASHED after {unlocked_time:.3f}s: {type(e).__name__}: {e}")

    print()

# ---------------------------------------------------------------------------
# Memory measurement hint
# ---------------------------------------------------------------------------

print("=" * 60)
print("MEMORY")
print("=" * 60)
try:
    import resource

    rusage = resource.getrusage(resource.RUSAGE_SELF)
    rss_mb = rusage.ru_maxrss / (1024 * 1024)  # macOS reports in bytes
    print(f"  Peak RSS: {rss_mb:.0f} MB (single process, shared models)")
except Exception:
    print("  (resource module not available)")

print()
print("Done.")
