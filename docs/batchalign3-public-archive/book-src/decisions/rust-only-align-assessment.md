# Rust-Only `align`: Honest Assessment

## Current Architecture

The `align` command runs two stages:

1. **UTR** (optional): Python Whisper ASR -> timed words -> Rust DP alignment -> utterance-level timing
2. **FA**: Python Whisper forced-decode with known text as decoder input -> cross-attention extraction -> DTW -> Rust injects word-level timing

Python ML deps: torch (~2GB), transformers, torchaudio -- all for Whisper inference.

## Proposed Approach

Replace Python Whisper with whisper.cpp (via `whisper-rs` Rust bindings). Since whisper.cpp's DTW mode only timestamps its *own ASR output* (not a pre-existing transcript), the approach would be: run ASR with DTW per chunk -> DP-align ASR tokens against known transcript -> map timestamps.

## Why We Decided Against It

### The quality trade-off is unfavorable for alignment

Current FA uses **forced decoding**: the model is given the exact transcript text as decoder input and extracts cross-attention weights between known tokens and audio frames. This gives near-perfect per-token alignment because the model doesn't have to guess what the text is -- it only has to figure out *when* each word occurs.

The whisper.cpp approach uses **free ASR**: the model generates its own text, DTW timestamps those generated tokens (~80% F1 at 50ms on clean speech per arXiv 2509.09987), then DP alignment maps generated tokens back to the known transcript. Quality degrades when:
- ASR output differs from transcript (specialized vocabulary, code-switching, noisy audio)
- DTW timestamps are imprecise (conversational speech: ~62% F1 at 50ms)
- DP alignment introduces errors at mismatched words

**For forced alignment, forced decoding is the right tool.** The precision advantage is fundamental, not an implementation detail.

### The practical advantages are marginal

| Advantage | Reality |
|-----------|---------|
| Startup: ~3s -> <1s | Users run align in batches. Startup is amortized over many files. |
| Install: ~2GB smaller | Deploy scripts handle wheel distribution. Not a user pain point. |
| Memory: ~4GB -> ~1.5GB/worker | The development machine has 64GB. Memory is not the bottleneck. |
| No GIL during inference | Whisper already releases GIL via CUDA/MPS ops. Callback overhead is negligible vs inference time. |
| Standalone binary | Nice future goal, not needed today. |

### The costs are real

- 1-2 weeks of engineering effort
- GGML model conversion and hosting (second model ecosystem alongside HuggingFace)
- Build complexity: cmake, whisper.cpp C++ compilation, Metal shader compilation
- whisper-rs API instability: 14 breaking changes in 25 releases
- Two code paths to maintain (native + Python fallback)

## Conclusion

The align command's strength is precise word-level timing from forced-decode cross-attention. Replacing it with a less precise ASR+DP approach to save ~2s startup and ~2GB install size is not a good trade-off. The current Python FA path is the right architecture for this task.

## Where whisper.cpp *Would* Make Sense

- **`transcribe` command**: Pure ASR with no known-text constraint. whisper.cpp is a direct replacement with no quality trade-off.
- **Lightweight deployment**: An align-only install that doesn't need the full ML stack.
- **Standalone CLI**: A future pure-Rust `batchalign` binary.
