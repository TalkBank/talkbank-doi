# Known Issues

Validation baselines for external corpora. These files list expected
validation failures so we can distinguish regressions from known issues.

## Files

- `phon-data-validation-baseline.txt` — 128 files in `~/data/phon-data/` that
  fail validation due to Phon tier alignment mismatches (E725–E728). Root cause
  is in PhonTalk's CHAT export. See `talkbank-tools/book/src/chat-format/phon-tiers.md`.

## Updating a Baseline

```bash
# Regenerate after code changes (run from talkbank-dev/):
cd talkbank-tools && cargo run --release -p talkbank-cli -- validate ~/data/phon-data/ --force 2>&1 \
  | grep '✗ Errors found in' \
  | sed 's/✗ Errors found in //' \
  | sort > ../known-issues/phon-data-validation-baseline.txt

# Compare with previous:
git diff known-issues/phon-data-validation-baseline.txt
```

## Adding a New Baseline

For each corpus directory you validate regularly:

```bash
cd talkbank-tools && cargo run --release -p talkbank-cli -- validate ~/data/<corpus>/ --force 2>&1 \
  | grep '✗ Errors found in' \
  | sed 's/✗ Errors found in //' \
  | sort > ../known-issues/<corpus>-validation-baseline.txt
```
