# Overnight Plan: Comprehensive Rust vs Python Master Benchmark + Audit

**Goal:** Boss report showing our Rust implementation is BOTH faster AND more correct than Python master.

## What's Running Now (Let Continue)

1. **Corpus Audit** - `chatter validate ~/data --audit ~/corpus-audit.jsonl --force`
   - PID: 27625
   - Progress: `grep 'Progress:' ~/corpus-audit-summary.txt | tail -1`
   - Will complete overnight, giving exact %gra/%wor error counts

## What to Run Tomorrow Morning

### 1. Create Very Long Dataset

Find the longest files for torture testing Python:

```bash
# Files with 700+ utterances (already searching in background)
# Pick top 3-5 for align_very_long dataset
mkdir -p ~/batchalign-benchmarking/data/align_very_long
cp <longest-files> ~/batchalign-benchmarking/data/align_very_long/
```

### 2. Benchmark: Align Performance

**Test on:** `align_small`, `align_medium`, `align_large`, `align_very_long`

**Rust:**
```bash
cd ~/batchalign-benchmarking
for dataset in align_small align_medium align_large align_very_long; do
  echo "=== Rust align $dataset ==="
  /usr/bin/time -l uv run batchalign-next align \
    data/$dataset output-rust-$dataset/ \
    --workers 2 2>&1 | tee results/rust-align-$dataset.log
done
```

**Python Master:**
```bash
for dataset in align_small align_medium align_large align_very_long; do
  echo "=== Python align $dataset ==="
  /usr/bin/time -l batchalign align \
    data/$dataset output-python-$dataset/ \
    --num_processes 2 2>&1 | tee results/python-align-$dataset.log
done
```

**Extract metrics:**
- Wall-clock time (from output)
- Peak memory (from `/usr/bin/time -l`: "maximum resident set size")
- Files processed

### 3. Benchmark: Morphotag Performance

**Test on:** Same datasets

**Rust:**
```bash
for dataset in align_small align_medium align_large align_very_long; do
  echo "=== Rust morphotag $dataset ==="
  /usr/bin/time -l uv run batchalign-next morphotag \
    data/$dataset output-rust-morphotag-$dataset/ \
    --workers 2 2>&1 | tee results/rust-morphotag-$dataset.log
done
```

**Python Master:**
```bash
for dataset in align_small align_medium align_large align_very_long; do
  echo "=== Python morphotag $dataset ==="
  /usr/bin/time -l batchalign morphotag \
    data/$dataset output-python-morphotag-$dataset/ \
    --num_processes 2 2>&1 | tee results/python-morphotag-$dataset.log
done
```

### 4. Correctness Audit

**For each output directory, validate and count errors:**

```bash
# Validate Rust outputs
for dir in output-rust-*/; do
  chatter validate $dir --audit audit-rust-$(basename $dir).jsonl --skip-wor-alignment
done

# Validate Python outputs
for dir in output-python-*/; do
  chatter validate $dir --audit audit-python-$(basename $dir).jsonl --skip-wor-alignment
done
```

**Compare error counts:**
```bash
# Count %gra errors in each
for f in audit-*.jsonl; do
  echo "=== $f ==="
  grep -E "E72[234]" $f | wc -l
done
```

### 5. Generate Boss Report

**Combine all data into final report:**

1. **Corpus-wide statistics** (from overnight corpus audit)
   - Total files with %gra errors (E722/E723/E724)
   - Total files with %wor errors (E342/E601)
   - Percentage of corpus affected

2. **Performance comparison** (from benchmarks)
   - Speedup table: Rust vs Python for each dataset
   - Memory usage table
   - Scale graph: show Python degradation on very_long files

3. **Correctness comparison** (from validation)
   - Python output: X files with circular dependencies
   - Rust output: 0 files with circular dependencies
   - Evidence: Python generates broken data, Rust guarantees correctness

4. **Conclusion**
   - Python master: slow + broken
   - Rust branch: fast + correct
   - Recommendation: deploy immediately, regenerate affected files

## Expected Timeline

- **Tonight:** Corpus audit completes (10-30 min per 10K files = ~3-10 hours)
- **Tomorrow morning:** Run all benchmarks (~2-3 hours total)
- **Tomorrow afternoon:** Generate report

## Files to Check Tomorrow

- `~/corpus-audit-summary.txt` - Corpus-wide statistics
- `~/corpus-audit.jsonl` - Detailed error list
- `~/batchalign-benchmarking/results/rust-*.log` - Rust benchmark logs
- `~/batchalign-benchmarking/results/python-*.log` - Python benchmark logs
- `~/batchalign-benchmarking/results/audit-*.jsonl` - Validation results

## Commands to Check Progress

```bash
# Corpus audit
ps aux | grep "chatter validate" | grep -v grep
grep 'Progress:' ~/corpus-audit-summary.txt | tail -1

# When done
tail -50 ~/corpus-audit-summary.txt
```
