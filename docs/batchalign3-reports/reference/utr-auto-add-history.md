# History of Auto-UTR in the Align Pipeline

## Timeline

### December 16, 2023 — First `align` dispatch (`1ac9dc6`)
An early implementation added the first working `align` dispatch. The FA engine was wired up
but UTR was not yet auto-added. Running `align` at this point only did
forced alignment on existing transcripts — the correct behavior.

### January 3, 2024 — Auto-UTR introduced (`fd7ef3a`)
Commit `fd7ef3a` ("birthday tagging and align + bulletize") added the
auto-UTR logic in `dispatch.py`:
```python
if "fa" in packages:
    if "utr" not in packages:
        packages.append("utr")
```
From this point forward, every `align` run silently loaded a full Whisper
ASR model and re-transcribed the audio from scratch *before* forced
alignment. The existing CHAT transcript was discarded by the pipeline's
generator step, which always runs first.

This was likely unintentional — the intent was probably to ensure that
the UTR engine was available when needed, not to force re-transcription.

### 2024–2025 — Unnoticed
The extra UTR step roughly doubled the runtime of `align` jobs but produced
correct output (the re-transcription + alignment was equivalent to
`transcribe` + `align`). No one noticed because `align` still produced
aligned transcripts; users attributed the slowness to FA being inherently
expensive.

### February 5, 2026 — First consequence (`a78ddc9`)
"Fix FA crash on impossibly short segments from UTR" — FA was crashing on
segments where UTR produced extremely short word timings. This was the
first hint that UTR was running during alignment (producing its own word
segmentation that sometimes conflicted with FA's expectations).

### February 11, 2026 — `--no-utr` workaround (`4789bb3`)
Added `--no-utr` flag and `utr=None` opt-out in `dispatch.py` to let
callers explicitly disable auto-UTR. Lazy UTR model loading was also added
so the ~4GB model wasn't loaded until actually needed.

### February 12, 2026 — Document → CHAT text migration (`cf52ee8`)
Migrated from `Document`-based to CHAT text-based processing. The old
`Document` pipeline's `__call__` method:
```python
if self.__generator:
    doc = self.__generator.generate(doc.media.url, **kwargs)
```
The new `process_chat_text` method:
```python
if self.__generator is not None:
    handle = self.__generator.generate_handle(chat_text_or_media, ...)
```
Both had the same bug: the generator always runs when present, treating
input as a media path even when it's CHAT text.

### February 13, 2026 — Bug discovered and fixed
During the `%wor` rerun deployment to the production server, `align` jobs failed
because:
1. The pipeline always ran UTR first (expecting a media path)
2. But the server passed CHAT text (existing transcripts)
3. Files without `@Media:` headers failed immediately
4. Files with `@Media:` hit a PyTorch MPS dtype bug during Whisper inference

Root cause identified: the pipeline had no logic to detect that input was
already CHAT text (not a media path) and skip the generator accordingly.

**Fix**: Added CHAT text detection in the pipeline loop:
```python
input_is_chat = (chat_text_or_media.lstrip().startswith("@UTF8")
                 or chat_text_or_media.lstrip().startswith("@Begin"))
if self.__generator is not None and not input_is_chat:
    # Only run generator for media paths
```
When input starts with `@UTF8` or `@Begin`, the generator is skipped and
only processors (FA) run. This preserves the existing transcript and
eliminates the wasteful re-transcription.

## Impact

- **Runtime**: `align` jobs now take ~50% less time (no Whisper ASR decode)
- **Memory**: ~4GB less RAM (no Whisper model loaded when only FA is needed)
- **Correctness**: `align` now aligns the *existing* transcript rather
  than silently replacing it with a new ASR transcription
- **Server**: Align jobs work correctly via `--server` (CHAT text is
  preserved, not treated as a media path)
