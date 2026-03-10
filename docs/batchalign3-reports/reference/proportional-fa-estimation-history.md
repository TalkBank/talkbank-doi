# Proportional FA Estimation History

Archived from the public `batchalign3` book on 2026-03-05.
The public page keeps current behavior, implementation status, and current test
coverage. Historical file-by-file rollout and build notes live here instead.

## Historical file changes

| File | Repo | Change |
|------|------|--------|
| `rust/crates/batchalign-core/src/forced_alignment.rs` | talkbank-utils | Add proportional estimation in `group_utterances()` |
| `rust/crates/batchalign-core/src/lib.rs` | talkbank-utils | Add `total_audio_ms` param, update post-processing |
| `batchalign/pipelines/fa/whisper_fa.py` | batchalign2 | Pass `total_audio_ms` to Rust FA |
| `batchalign/pipelines/fa/wave2vec_fa.py` | batchalign2 | Pass `total_audio_ms` to Rust FA |

## Testing

### Rust unit tests (`forced_alignment.rs`)
- Untimed utterances grouped with proportional estimates when `total_audio_ms` is provided
- Untimed utterances still skipped when `total_audio_ms` is `None` (backwards compat)
- Mixed timed/untimed utterances grouped correctly
- Buffer clamped to `[0, total_audio_ms]`

### Python integration tests
- `add_forced_alignment` with untimed CHAT + `total_audio_ms` produces timing
- `add_forced_alignment` with untimed CHAT + no `total_audio_ms` produces no timing (backwards compat)

## Historical build & deploy notes

```bash
# In talkbank-utils
cd ~/talkbank-utils/rust/crates/batchalign-core
maturin develop --release

# In batchalign2
uv run pytest batchalign/tests/pipelines/ -v
```
