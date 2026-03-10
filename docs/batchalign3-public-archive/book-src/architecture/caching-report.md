# Batchalign Caching & Performance Report

**Date:** February 12, 2026
**Author:** Franklin Chen (with engineering analysis by Claude)

---

## Executive Summary

Batchalign processes audio recordings by running them through several expensive computational stages. We identified that much of this computation is **redundant** -- the same audio files are being reprocessed from scratch every time a transcript is re-aligned, even though the audio never changes. This report describes what we found, what we've already fixed, and what decisions remain.

---

## Background: What Happens When You Run "align"

When a user runs `batchalign align` on a corpus, each file goes through three stages:

1. **UTR (Utterance Timing Recovery)** -- Runs a full Whisper speech recognition pass over the entire audio file to figure out roughly where each utterance starts and ends. This is the most expensive step: it processes the entire audio through a large neural network.

2. **Forced Alignment (FA)** -- Takes the rough timing from UTR and produces precise word-level timestamps by running Whisper again on smaller audio chunks.

3. **Morphosyntactic Analysis** -- Analyzes the grammar and parts of speech of each utterance using the Stanza NLP toolkit. This is CPU-based and relatively fast.

Until now, **none of these results were cached across runs**. Every time someone re-aligned a corpus -- even without changing anything -- all three stages ran from scratch.

---

## Problem 1: UTR Recomputation (FIXED)

### The Issue

UTR (stage 1) is the most expensive stage. It runs full Whisper ASR on the entire audio file just to get approximate timing boundaries. The key insight: **UTR output depends only on the audio file, not the transcript.** If the audio hasn't changed, the UTR result is always the same, regardless of how many times the transcript is edited and re-aligned.

Despite this, UTR was running from scratch on every `align` invocation.

### What We Did

We added a **permanent cache** for UTR results:

- Each audio file gets a unique identity based on its file path and size
- The first time UTR processes an audio file, Whisper runs and the result is stored in a local database
- Every subsequent run on the same audio file skips Whisper entirely and uses the cached result
- The cache is **permanent** -- it survives all normal cache-clearing operations
- The cache is **model-aware** -- if we upgrade to a better Whisper model in the future, it automatically re-runs on the new model

### Impact

For any audio file that has been processed at least once, UTR becomes essentially free on all future runs. For our ~47,000 audio files, this means the first alignment is expensive but every re-alignment skips the most costly step.

### Remaining Question

We don't yet have per-engine timing data to know exactly what percentage of total processing time UTR accounts for. We've added instrumentation to measure this (see Section 5), but haven't run the benchmarks yet.

---

## Problem 2: Cache Safety (FIXED)

### The Issue

Batchalign has a `cache --clear` command that wipes all cached data. Previously, this would destroy UTR results along with everything else, forcing expensive recomputation.

### What We Did

- The UTR cache is now **protected by default** -- `cache --clear` removes morphosyntax and alignment caches but preserves UTR entries
- A `--all` flag is available for the rare case where someone truly wants to wipe everything
- The UTR cache is also excluded from the automatic 90-day expiration that cleans up stale entries
- The benchmarking suite no longer touches the global cache at all (it previously deleted the entire cache database before each benchmark run)

---

## Problem 3: MP4 Video Conversion (NOT YET ADDRESSED)

### The Issue

Across our media servers, we have approximately **16,700 MP4 video files**:
- CHILDES: ~8,000 MP4 files
- Other banks: ~8,700 MP4 files
- HomeBank: ~65 MP4 files

Every time Batchalign processes an MP4 file, it must first convert it to WAV audio using ffmpeg. This conversion is cached in a local media cache directory, but that cache is cleared alongside the analysis cache when someone runs `cache --clear`.

The MP4 files on the server never change, so this conversion work is always identical -- we're just doing it again because the cached WAV was deleted.

### Options

**Option A: Protect the media conversion cache** (similar to UTR)
- Make `cache --clear` preserve converted WAV files by default
- Pro: No changes to media organization, conversions accumulate over time
- Con: Still requires first-time conversion; ~16,700 WAV files sitting in a cache directory

**Option B: Pre-convert MP4s to MP3 on the server (one-time job)**
- Run a batch ffmpeg conversion on the server to create MP3 versions alongside the MP4s
- Batchalign would then use the MP3 directly, no runtime conversion needed
- Pro: Eliminates the problem entirely; MP3 is ~10x smaller than WAV
- Con: Requires a decision about media organization on the server volumes

**Option C: Both** -- protect the cache now, batch-convert later when convenient

### Decision Needed

This requires a decision about media management on the server volumes.

---

## Problem 4: WAV Originals in 0wav Directories (INFORMATIONAL)

We found **181 `0wav` directories** scattered across the CHILDES volume. These contain the original WAV recordings before they were compressed to MP3. They mirror the regular directory structure:

```
/Volumes/CHILDES/CHILDES/Eng-NA/Brent/c1/000902.mp3     (compressed, used by batchalign)
/Volumes/CHILDES/CHILDES/Eng-NA/Brent/0wav/c1/000902.wav (original, unused)
```

Batchalign currently uses the MP3 files. The WAV originals are not referenced and represent archival copies. No action needed, but worth knowing they exist if there's ever a question about audio quality.

---

## Problem 5: UTR Cache Prewarming (PARTIALLY IMPLEMENTED)

### The Idea

Since the UTR cache is permanent, we could **prewarm** it -- run Whisper on all audio files once in advance, so that future `align` runs never have to wait for UTR.

### What We Built

A `cache prewarm` CLI command that walks a directory of audio files and runs Whisper on any that aren't cached yet. It's resumable (picks up where it left off if interrupted).

### The Problem

The current implementation assumes audio files and CHAT files are in the same directory. In reality, our data is organized differently:
- **CHAT files** are in git repos on user machines (e.g., `~/data/childes-data/Eng-NA/...`)
- **Media files** are on the production server (e.g., `/Volumes/CHILDES/CHILDES/Eng-NA/...`)
- The language (needed to select the right Whisper model) is in the CHAT files, not discoverable from the audio alone

### What's Needed

The prewarm command needs to be redesigned as a **server-side operation**:
1. Client points at a CHAT directory (e.g., `~/data/childes-data/Eng-NA/`)
2. Client reads each `.cha` file to extract the language and media filename
3. Client sends these to the server
4. Server resolves the media path using its `media_mappings` configuration
5. Server runs Whisper and caches the result

This would look like:
```
batchalign-next cache prewarm ~/data/childes-data/ --server http://<server>:8000 -r
```

### Scale Consideration

With ~47,000 audio files and Whisper taking roughly 30-60 seconds per file on GPU, a full prewarm would take approximately **2-4 weeks of continuous GPU time**. This is not an overnight job. However:

- It only needs to happen once per audio file, ever
- It's resumable -- can be run in chunks over time
- The cache accumulates naturally through normal usage (every `align` run caches its UTR results)
- Targeted prewarming of specific corpora before a known batch job is practical

---

## Problem 6: Per-Engine Performance Visibility (FIXED)

### The Issue

We had no way to know which processing stage (UTR, FA, morphosyntax) was the bottleneck for any given run. The run logs only showed total per-file processing time.

### What We Did

Added per-engine instrumentation that records, for each file processed:
- **Time** spent in each engine (UTR, Forced Alignment, Morphosyntax)
- **Peak memory** (RSS) change caused by each engine

This data appears in the structured run logs and in the formatted output of `batchalign-next logs --last`. Example:

```
  1  000902.cha               done       38.39s  parse=0.008s ser=0.0s
      Utterance Timing Recovery  25.1s (+2100 MB)
      Forced Alignment            8.2s (+500 MB)
      Morpho-Syntax               5.0s (+0 MB)
```

### Next Step

Run benchmarks with this instrumentation to get actual numbers and determine where optimization efforts should focus.

---

## Media Server Inventory

| Volume | MP3 Files | WAV Files | MP4 Files | Total Audio |
|--------|-----------|-----------|-----------|-------------|
| CHILDES | ~21,000 | ~11,000* | ~8,000 | ~29,000 |
| Other banks | -- | -- | ~8,700 | ~8,700+ |
| HomeBank | -- | -- | ~65 | 65+ |
| **Total** | | | **~16,700** | **~47,000+** |

*Most WAV files are in `0wav` archival directories, not actively used.

---

## Summary of Actions Taken

| Item | Status | Description |
|------|--------|-------------|
| UTR result caching | **Done** | Whisper ASR results cached permanently per audio file |
| Cache protection | **Done** | UTR cache immune to `--clear`, `--override-cache`, and auto-pruning |
| Per-engine timing | **Done** | Time and memory tracked per engine per file |
| Benchmarking infrastructure | **Done** | Cross-branch comparison suite ready to run |
| Cache prewarm (local) | **Done** | Works for colocated audio+CHAT setups |
| Cache prewarm (server) | **Not started** | Needs redesign for split CHAT/media organization |
| MP4 conversion caching | **Not started** | Needs decision on approach |
| MP4 batch pre-conversion | **Not started** | Needs decision on media organization |

---

## Decisions Needed

1. **MP4 files**: Should we (a) protect the conversion cache, (b) batch-convert to MP3 on the server, or (c) both?
2. **Server-side prewarm**: Should we invest in building the server-side prewarm workflow? If so, which corpora should be prioritized?
3. **Benchmarking**: Should we run the full cross-branch benchmark suite to get concrete per-engine timing numbers before making further optimization decisions?
