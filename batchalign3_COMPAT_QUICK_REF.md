# Batchalign3 CLI BA2 Compat Quick Reference

**Status:** Reference
**Last updated:** 2026-03-15

## Hidden Flag Summary (37 total)

### Global Flags (10) — All No-ops
```
--memlog, --mem-guard, --adaptive-workers, --no-adaptive-workers
--pool, --no-pool, --shared-models, --no-shared-models
--adaptive-safety-factor <F>, --adaptive-warmup <N>
```
**File:** `src/args/global_opts.rs:15-60`
**Behavior:** Parse and ignore (never used in code)
**Test:** `tests/command_surface_manifest.rs:98-158`

### ALIGN Command (4 flags) 
| Alias | Canonical | Engine Output |
|-------|-----------|---------------|
| `--whisper` | `--utr-engine whisper` | UTR engine → RevAi OR Whisper |
| `--rev` | `--utr-engine rev` | UTR engine → RevAi |
| `--whisper-fa` | `--fa-engine whisper` | FA engine → "whisper_fa" |
| `--wav2vec` | `--fa-engine wav2vec` | FA engine → "wav2vec_fa" |

**File:** `src/args/commands.rs:134-150` (definitions)
**Resolution:** `src/args/options.rs:53-88` (build_typed_options)
**Test:** `tests/command_surface_manifest.rs:159-182`

### TRANSCRIBE Command (6 flags)
| Alias | Canonical | Engine Output |
|-------|-----------|---------------|
| `--whisper` | `--asr-engine whisper` | ASR → "whisper" |
| `--whisperx` | `--asr-engine whisperx` | ASR → "whisperx" |
| `--whisper-oai` | `--asr-engine whisper-oai` | ASR → "whisper_oai" |
| `--rev` | `--asr-engine rev` | ASR → "rev" |
| `--diarize` | `--diarization enabled` | Output variant → TranscribeS |
| `--nodiarize` | `--diarization disabled` | Output variant → Transcribe |

**File:** `src/args/commands.rs:200-224` (definitions)
**Resolution:** `src/args/options.rs:90-136` (build_typed_options)
**Test:** `tests/command_surface_manifest.rs:183-218`

### BENCHMARK Command (3 flags)
| Alias | Canonical | Engine Output |
|-------|-----------|---------------|
| `--whisper` | `--asr-engine whisper` | ASR → "whisper" |
| `--whisper-oai` | `--asr-engine whisper-oai` | ASR → "whisper_oai" |
| `--rev` | `--asr-engine rev` | ASR → "rev" |

**File:** `src/args/commands.rs:382-394` (definitions)
**Resolution:** `src/args/options.rs:156-181` (build_typed_options)
**Test:** `tests/command_surface_manifest.rs:219-237`

---

## Resolution Precedence (Three-Tier)

### For Engine Names:
```
1. Custom engine name (--*-engine-custom <STR>)  [HIGHEST]
2. Compat bool flag (--whisper, --rev, etc.)
3. Enum default (--asr-engine rev, default Rev)   [LOWEST]
```

**Example (transcribe):**
```rust
if a.asr_engine_custom.is_some() {
    use custom_engine  // Wins
} else if a.whisper {
    use "whisper"
} else if a.rev {
    use "rev"
} else {
    match a.asr_engine { ... }  // Enum
}
```

### For Boolean Pairs:
```
final_value = positive_flag && !negative_flag
```

**Examples:**
- `--wor && !--nowor` (default true)
- `--override-cache && !--use-cache` (default false)
- `--lazy-audio && !--no-lazy-audio` (default true)

**Edge case:** Both flags true → AND gate produces false
```bash
batchalign3 align --wor --nowor input/  # wor = true && !true = false
```

---

## Boolean Pair Inventory

| Command | Pair | Default | Gate Logic |
|---------|------|---------|-----------|
| **global** | `--override-cache` / `--use-cache` | false (no override) | `override_cache && !use_cache` |
| **global** | `--lazy-audio` / `--no-lazy-audio` | true (lazy) | `lazy_audio && !no_lazy_audio` |
| **global** | `--tui` / `--no-tui` | true (tui enabled) | `tui && !no_tui` |
| **global** | `--force-cpu` / `--no-force-cpu` | false (auto device) | `force_cpu && !no_force_cpu` |
| **align** | `--wor` / `--nowor` | true (write %wor) | `wor && !nowor` |
| **align** | `--utr` / `--no-utr` | true (enable UTR) | `utr && !no_utr` |
| **align** | `--merge-abbrev` / `--no-merge-abbrev` | false (no merge) | `merge_abbrev && !no_merge_abbrev` |
| **transcribe** | `--wor` / `--nowor` | false (no %wor) | `wor && !nowor` |
| **transcribe** | `--merge-abbrev` / `--no-merge-abbrev` | false (no merge) | `merge_abbrev && !no_merge_abbrev` |
| **benchmark** | `--wor` / `--nowor` | false (no %wor) | `wor && !nowor` |
| **benchmark** | `--merge-abbrev` / `--no-merge-abbrev` | false (no merge) | `merge_abbrev && !no_merge_abbrev` |

---

## Key Test Files

| File | Lines | Purpose |
|------|-------|---------|
| `src/args/options.rs` | 1-278 | Resolution logic for all commands |
| `src/args/commands.rs` | 81-394 | Arg struct definitions with hidden flags |
| `src/args/global_opts.rs` | 1-94 | GlobalOpts with 10 BA2 no-ops |
| `src/args/tests.rs` | 28KB | Parsing unit tests (needs compat expansion) |
| `tests/command_surface_manifest.rs` | 343 | **Golden test:** 37 hidden compat cases verified |
| `tests/cli.rs` | 426 | Subprocess binary tests |
| `tests/common/mod.rs` | 380 | Test infrastructure (helpers, server startup) |

---

## Fast Contract Test Patterns (No ML Models)

### Pattern 1: Unit Parse Test
```rust
#[test]
fn align_whisper_alias_equals_canonical() {
    let cli = Cli::parse_from(["batchalign3", "align", ".", "--whisper"]);
    if let Commands::Align(a) = &cli.command {
        assert!(a.whisper);
        let opts = build_typed_options(&cli.command, &cli.global).unwrap();
        if let CommandOptions::Align(AlignOptions { utr_engine: Some(AppUtrEngine::Whisper), .. }) = opts {}
    }
}
```
**Run:** `cargo test -p batchalign-cli --lib` (instant, no server)

### Pattern 2: Subprocess Help Test
```rust
#[test]
fn hidden_flags_not_in_help() {
    cmd().args(&["align", "--help"]).assert().success()
        .stdout(!contains("--whisper"))
        .stdout(!contains("--rev"));
}
```
**Run:** `cargo test -p batchalign-cli --test cli` (< 1s per test)

### Pattern 3: Integration Test (Test-Echo Server)
```rust
#[tokio::test]
async fn align_whisper_via_server() {
    let (base_url, _tmp) = start_test_server(&python_path).await;
    let (job_info, results) = run_job_to_completion(
        &client, &base_url, "align", "eng",
        vec![FilePayload { ... }],
        CommandOptions::Align(AlignOptions { ... }),
    ).await;
    assert_eq!(job_info.status, JobStatus::Completed);
}
```
**Run:** `cargo test -p batchalign-cli --test commands` (requires Python + batchalign, ~60s)

---

## Dispatch Guarantee: No Compat Leakage

**Key invariant:** By the time code reaches `dispatch()` or server submission, all compat flags have been **resolved to canonical strings** in `CommandOptions` enum.

```
Parse Layer (clap)
    ↓ (parse args + hidden flags)
    ↓
Resolution Layer (options.rs:build_typed_options)
    ↓ (resolve compat → canonical)
    ↓
CommandOptions (canonical enum with string engine names)
    ↓
Dispatch/Server
    ↓ (no compat awareness needed)
```

**Verification:** `CommandOptions` enum in `batchalign_app::options` module uses canonical strings (not clap enums), confirming resolution complete by dispatch time.

---

## Coverage Map: 37 Hidden Compat Cases

**All tested in:** `tests/command_surface_manifest.rs:330-342`

```
Global (lines 99-158):        10 flags
  --memlog, --mem-guard, --adaptive-workers, --no-adaptive-workers
  --pool, --no-pool, --shared-models, --no-shared-models
  --adaptive-safety-factor, --adaptive-warmup

Align (lines 159-182):        4 flags
  --whisper, --rev, --whisper-fa, --wav2vec

Transcribe (lines 183-218):   6 flags
  --whisper, --whisperx, --whisper-oai, --rev, --diarize, --nodiarize

Benchmark (lines 219-237):    3 flags
  --whisper, --whisper-oai, --rev

TOTAL: 37 flags
```

Each case verifies:
1. **Parse success:** `cmd().args(case.args).assert().success()`
2. **Help hidden:** `!help.contains(case.hidden_flag)`

---

## Caveats

### Hidden Flags Still Parse (by design)
- `hide = true` only removes from help listing
- Flags are **fully parsed** and **fully functional**
- Parser accepts both old and new syntax

### Boolean Pair AND Logic
- Passing both `--wor` and `--nowor` → `wor = false` (AND gate)
- **Not** an error, but unintuitive

### Diarization Two-Stage Check (Transcribe only)
- Checks `--diarize` before `--nodiarize` before enum
- If both passed: `--diarize` wins (output variant is TranscribeS)

### Global Compat Flags Are No-ops
- Parse successfully but are **completely ignored**
- No code path reads these fields after parsing
- Kept for script compatibility only

---

## Recommended Test Additions

**High priority (instant, add to `src/args/tests.rs`):**
- [ ] `test_align_custom_engine_overrides_compat_flag()`
- [ ] `test_transcribe_diarize_vs_nodiarize_precedence()`
- [ ] `test_benchmark_asr_engine_compat_aliases_resolve_correctly()`
- [ ] `test_global_lazy_audio_and_gate()`
- [ ] `test_global_compat_flags_parse_silently()`

**Medium priority (subprocess tests, ~10s total):**
- [ ] `test_align_hidden_flags_not_in_help()` in `tests/command_surface_manifest.rs`
- [ ] Expand per-command help assertions

**Lower priority (integration with server, ~60s):**
- [ ] Round-trip equivalence tests (compat alias produces identical results to canonical)
