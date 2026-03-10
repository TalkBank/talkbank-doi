# Master Branch Status: Why We Are Not Merging

**Date:** 2026-02-13
**Branch under development:** `align`
**Upstream master:** `TalkBank/batchalign2:master`

---

## Summary

I fetched the latest upstream master and compared it against our `align` branch.
Master has **3 new commits** since we diverged. None of them need to be
incorporated. Here is why.

---

## What is on master that we don't have

### 1. `c3079d5` — "fix alignment output" (Houjun, Feb 11)

This fixes a bug in the Python CHAT generator (`generator.py`) where %wor
timing bullets with very short timestamps (single-digit start and end) were
being written as malformed text. The fix strips these bad bullets and cleans
up extra whitespace:

```python
# Before (one line):
result.append(("%wor:\t"+" ".join(wor_elems)).replace("-\x15", "- \x15"))

# After (loop that filters bad entries):
for b in wor_elems:
    b = re.sub("\x15\d_\d\x15", "", b)   # strip 1-digit timing bullets
    b = re.sub(" +", " ", b)              # collapse whitespace
    if b.strip():
        checked_wor_elems.append(b.strip())
```

**Why we don't need it:** The `align` branch does not use the Python CHAT
generator at all. Our entire CHAT serialization pipeline — including %wor
generation — was rewritten in Rust (`generate_wor_tier` in talkbank-model).
The Rust serializer produces timing bullets from structured `(start_ms, end_ms)`
integer pairs, so malformed single-digit bullets cannot occur. The bug Houjun
fixed literally cannot happen in our code path.

### 2. `1d27c3f` — Pillow 12.1.0 → 12.1.1 (dependabot, Feb 11)

Automated security bump in `requirements.txt`. We use `pyproject.toml` with
`uv` for dependency management; `requirements.txt` is not used. Pillow is not
a direct dependency of batchalign.

### 3. `fd816d4` — Merge commit for the Pillow bump

No code. Just the merge.

---

## What is on `align` that master does not have

169 commits, including:

- Full Rust backend for CHAT parsing and serialization (no more Python CHAT parser/generator)
- Zero-reparse pipeline architecture (parse once, mutate in Rust, serialize once)
- Batched morphosyntax and utterance segmentation via Rust-Python callbacks
- Multi-input CLI (files, directories, file lists)
- Server crash recovery fix
- UTR caching, engine timing instrumentation, structured run logging
- Full mypy compliance (84 errors → 0)
- Python 3.11 → 3.12 upgrade

---

## Conclusion

Master is effectively frozen. The only code change since we diverged is a
patch to the Python CHAT generator — a file we deleted. There is nothing to
merge back, and attempting to do so would create unnecessary conflicts across
every file we have rewritten.

When the `align` branch is ready, it replaces master entirely.
