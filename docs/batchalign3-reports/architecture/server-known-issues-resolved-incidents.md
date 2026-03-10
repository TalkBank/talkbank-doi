# Server Known Issues: Resolved Incidents Archive

Archived from the public `batchalign3` book on 2026-03-05.
The public book keeps current open issues only. Historical incident detail and
postmortem-oriented material live here instead.

## Resolved Incidents (Historical)

### 2. MPS bfloat16 crash during Whisper alignment (Apple Silicon)

**Symptom:** The server process aborts during an `align` job with a Metal assertion failure:

```
MPSNDArrayMatrixMultiplication.mm:5028: failed assertion
'Destination NDArray and Accumulator NDArray cannot have different datatype
in MPSNDArrayMatrixMultiplication'
```

The crash kills the entire server process. On systems with launchd auto-restart, this causes a restart storm (10+ rapid restarts as the server loads models each time). Jobs in progress are lost.

**Root cause:** The Whisper ASR model used by `WhisperUTREngine` (the UTR step auto-added by `align`) was loaded with `torch_dtype=torch.bfloat16` on MPS. Apple's Metal Performance Shaders framework has a bug in `MPSNDArrayMatrixMultiplication` where the accumulator and destination tensors end up with mismatched dtypes, triggering a C-level assertion that aborts the process. This is intermittent — it depends on runtime tensor allocation patterns.

The `WhisperFAEngine` (forced alignment) was not affected because it already loaded its model in float32.

**Status:** Fixed (2026-02-25). `infer_asr.py` now forces `torch.float32` when `device.type == "mps"`. Non-MPS devices (CUDA, CPU) retain their original bfloat16/float16 dtypes. Performance impact is negligible — UTR is a quick ASR pass for utterance boundaries, not the alignment bottleneck.

**Affected file:** `batchalign/models/whisper/infer_asr.py`

**Incident:** See `talkbank-private/batchalign/docs/postmortem-2026-02-25-net-mps-crash.md` for the full postmortem. The crash caused a system reboot on Net (Mac Studio M3 Ultra), ~50 minutes of downtime, and one lost align job.

### 3. CHAT parser `str.strip()` bug with line-based input

**Symptom:** Morphotag fails with `IndexError: list index out of range` in `ud.py` at `langs[0]` because `doc.langs` is empty.

**Root cause:** The CHAT parser (`formats/chat/parser.py`) used `line.strip("@Languages:")` to extract the language value. Python's `str.strip()` removes individual *characters* from both ends, not a substring. For `eng`, all three letters appear in `"@Languages:"`, so the entire value gets stripped. This only manifests when lines lack trailing `\n` (i.e., when content comes via `str.split("\n")` as in the server path). The file-read path (`readlines()`) preserves `\n`, which accidentally prevented full stripping.

**Status:** Fixed. All four instances of `line.strip("@Header:")` replaced with `line.split(":", 1)[1].strip()` in `parser.py`. Affects `@Languages`, `@Options`, `@ID`, and `@Media` parsing.

**Affected lines:** `@Languages` (language codes like `eng`, `spa`), `@ID` (participant metadata), `@Media` (media file references), `@Options` (CHAT options).

### 4. Single-file media mapping includes filename in subdir

**Symptom:** Align on a single file fails with `Media file not found via mapping 'childes-data' at: /Volumes/.../MacWhinney/010523.cha` -- the `.cha` filename appears in the media search path.

**Root cause:** `_detect_media_mapping()` in `dispatch_server.py` scans path components of `in_dir`. In single-file mode, `in_dir` is the file path, so the filename becomes part of the subdir sent to the server.

**Status:** Fixed. Single-file mode now uses `Path(in_dir).parent` for mapping detection.

### 5. Deep copy in pipeline.__call__ (not a bug)

**Investigated, not an issue.** The `doc.model_copy(deep=True)` at `pipeline.py:155` costs ~17ms for a typical document. The Document contains only plain Python objects (no tensors, no audio data). The deep copy protects the user-facing API from unexpected mutation (engines mutate documents in place). All production dispatch paths create fresh documents per file, making the copy technically unnecessary but harmless.

### 6. Stale cache warnings on server

**Symptom:** Server stderr fills with `Morphotag cache entry contains timecodes; clearing cache is recommended`.

**Root cause:** The SQLite analysis cache on the server machine contains entries from previous batchalign runs with different pipeline configurations. Cached morphotag results include timecodes that shouldn't be there.

**Fix:** Clear the cache on the server:
```bash
batchalign3 cache --clear
```
