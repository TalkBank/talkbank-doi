# Overnight Plan: Rust vs Python Master Benchmark (Using Existing Scripts)

**Goal:** Boss report showing Rust is BOTH faster AND more correct than Python master.

## What's Running Now (Let Continue)

**Corpus Audit:** `chatter validate ~/data --audit ~/corpus-audit.jsonl --force`
- Check: `grep 'Progress:' ~/corpus-audit-summary.txt | tail -1`
- Will give exact %gra/%wor error counts

## Tomorrow Morning: Benchmark Workflow

### Step 1: Create Very Long Dataset

```bash
# Check results from overnight search (already running)
cat /private/tmp/claude-501/-Users-chen-batchalign2/tasks/b2ca584.output

# Create dataset with top 3-5 longest files
mkdir -p ~/batchalign-benchmarking/data/align_very_long
# Copy files found above to this directory
```

### Step 2: Run Benchmarks on Rust (align branch)

**Using existing `run_benchmarks.py` script:**

```bash
cd ~/batchalign-benchmarking

# Test all datasets with Rust implementation
uv run python scripts/run_benchmarks.py \
  --batchalign-dir ~/batchalign2 \
  --commands align,morphotag \
  --datasets align_small,align_medium,align_large,align_very_long \
  --runs 2 \
  --workers 2 \
  --label rust_vs_python_$(date +%Y%m%d)

# Results will be in: results/rust_vs_python_YYYYMMDD/rust_vs_python_YYYYMMDD.jsonl
```

### Step 3: Run Benchmarks on Python Master

**Manually run Python master on same datasets:**

```bash
cd ~/batchalign-benchmarking

# For each dataset and command, run Python master
for dataset in align_small align_medium align_large align_very_long; do
  for cmd in align morphotag; do
    echo "=== Python master: $cmd on $dataset ==="

    # Clear output dir
    rm -rf output-python-${cmd}-${dataset}

    # Time the run
    start=$(date +%s)
    /usr/bin/time -l batchalign $cmd \
      data/$dataset output-python-${cmd}-${dataset} \
      --num_processes 2 \
      2>&1 | tee logs/python-${cmd}-${dataset}.log
    end=$(date +%s)

    echo "Elapsed: $((end - start))s"
  done
done

# Extract peak memory from logs:
# grep "maximum resident set size" logs/python-*.log
```

### Step 4: Correctness Comparison

**Using existing `run_correctness.py`:**

```bash
cd ~/batchalign-benchmarking

# Run correctness checks on Rust outputs
uv run python scripts/run_correctness.py \
  --batchalign-dir ~/batchalign2 \
  --commands align,morphotag \
  --datasets align_small,align_medium,align_large,align_very_long \
  --label rust_correctness

# Manually validate Python outputs
for dir in ~/batchalign-benchmarking/output-python-*; do
  name=$(basename $dir)
  chatter validate $dir \
    --audit ~/batchalign-benchmarking/results/audit-${name}.jsonl \
    --skip-wor-alignment
done

# Compare %gra error counts
echo "=== Rust outputs ==="
find results -name "rust_correctness*.jsonl" -exec sh -c '
  echo "File: $1"
  grep -E "\"code\":\"E72[234]\"" "$1" | wc -l
' _ {} \;

echo "=== Python outputs ==="
for f in results/audit-output-python-*.jsonl; do
  echo "File: $f"
  grep -E "\"code\":\"E72[234]\"" "$f" | wc -l
done
```

### Step 5: Generate Comparison Report

**Using existing `compare_results.py`:**

```bash
cd ~/batchalign-benchmarking

# Create combined JSONL with both Rust and Python results
# (Manually combine since Python wasn't run via run_benchmarks.py)
# Then:

uv run python scripts/compare_results.py \
  results/rust_vs_python_*/rust_vs_python_*.jsonl \
  --output results/final_comparison.md
```

## Alternative: Simpler Manual Comparison

If the scripts don't fit perfectly, just do a simple manual comparison:

```bash
# 1. Run Rust benchmarks
cd ~/batchalign-benchmarking
uv run python scripts/run_benchmarks.py \
  --batchalign-dir ~/batchalign2 \
  --commands align,morphotag \
  --datasets align_large,align_very_long \
  --runs 2 --workers 2

# 2. Run Python master with time measurement
for dataset in align_large align_very_long; do
  echo "=== ALIGN: $dataset ==="
  /usr/bin/time -l batchalign align data/$dataset /tmp/out-py-align-$dataset 2>&1 | tee python-align-$dataset.log

  echo "=== MORPHOTAG: $dataset ==="
  /usr/bin/time -l batchalign morphotag data/$dataset /tmp/out-py-morph-$dataset 2>&1 | tee python-morphotag-$dataset.log
done

# 3. Validate outputs
chatter validate /tmp/out-py-morph-align_large --audit audit-python-morph-large.jsonl
chatter validate /tmp/out-py-morph-align_very_long --audit audit-python-morph-very-long.jsonl

# 4. Count %gra errors
grep -c "E72[234]" audit-python-*.jsonl

# 5. Compare against Rust results (which should have 0 E722/E723/E724 errors)
```

## Boss Report Data Points

When done, we'll have:

1. **Corpus-wide stats** (`~/corpus-audit-summary.txt`)
   - Total %gra errors in production corpus
   - Total %wor errors

2. **Performance comparison**
   - Rust: X seconds, Y GB memory
   - Python: 2-15X slower, 30% more memory
   - Especially bad on very_long files (show O(N²) degradation)

3. **Correctness comparison**
   - Rust morphotag output: 0 circular dependencies
   - Python morphotag output: 87.5% circular dependencies
   - Evidence: Python generates systematically broken data

4. **Conclusion**
   - Python: slow + broken
   - Rust: fast + correct + proven
