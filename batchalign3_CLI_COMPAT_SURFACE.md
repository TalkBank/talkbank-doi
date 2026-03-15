# Batchalign3 CLI - Hidden BA2 Compatibility Surface Analysis

**Status:** Reference
**Last updated:** 2026-03-15

## 1. Canonical vs. Compat Option Representation and Resolution

### 1.1 Architecture Pattern

**Three-tier resolution model:**

1. **Parse Layer** (clap in `src/args/`):
   - Canonical enums with explicit `#[arg]` attributes
   - Hidden compat bool flags with `#[arg(long, hide = true)]`
   - Conflicting pairs (e.g., `--wor` / `--nowor`) use `conflicts_with`

2. **Resolution Layer** (`src/args/options.rs::build_typed_options()`):
   - Convert parsed `Commands` + `GlobalOpts` → typed `CommandOptions` enum
   - Compat bools override canonical enums when set
   - Order: custom engine names > compat bools > enum defaults

3. **Dispatch Layer** (`src/dispatch/`):
   - Receive fully resolved `CommandOptions` (no compat awareness needed)
   - Submit to server with canonical string engine names

### 1.2 **ALIGN** Command

**File:** `src/args/commands.rs:81-150`

**Canonical options:**
- `--utr-engine` (Rev|Whisper) → defaults to Rev
- `--fa-engine` (Wav2vec|Whisper) → defaults to Wav2vec
- `--utr-engine-custom <STR>` → overrides enum
- `--fa-engine-custom <STR>` → overrides enum
- `--pauses` (bool)
- `--wor` / `--nowor` (conflicting pair, default true)
- `--utr` / `--no-utr` (conflicting pair, default true)
- `--merge-abbrev` / `--no-merge-abbrev` (conflicting pair, default false)

**Hidden BA2 compat aliases:**
```rust
--whisper          // bool, hide=true → utr_engine=Whisper
--rev              // bool, hide=true → utr_engine=Rev
--whisper-fa       // bool, hide=true → fa_engine=Whisper
--wav2vec          // bool, hide=true → fa_engine=Wav2vec
```

**Resolution in `build_typed_options()` (lines 53-88):**
```rust
// FA engine: custom > compat bool > enum (default wav2vec)
let fa_engine = if let Some(engine) = a.fa_engine_custom.as_deref() {
    engine.to_string()
} else if a.whisper_fa {
    "whisper_fa".into()
} else {
    match a.fa_engine {
        FaEngine::Wav2vec => "wav2vec_fa".into(),
        FaEngine::Whisper => "whisper_fa".into(),
    }
};

// UTR engine: custom > compat bool (two-stage) > enum (default Rev)
let utr_engine = if a.utr && !a.no_utr {
    let utr = if let Some(engine) = a.utr_engine_custom.as_deref() {
        AppUtrEngine::Custom(CustomEngineName::new(engine))
    } else if a.whisper && !a.rev {
        AppUtrEngine::Whisper
    } else {
        match a.utr_engine {
            CliUtrEngine::Rev => AppUtrEngine::RevAi,
            CliUtrEngine::Whisper => AppUtrEngine::Whisper,
        }
    };
    Some(utr)
} else {
    None
};

// Boolean pair handling: final values with AND gates
AlignOptions {
    wor: a.wor && !a.nowor,           // default true
    merge_abbrev: a.merge_abbrev && !a.no_merge_abbrev,
    // ...
}
```

### 1.3 **TRANSCRIBE** Command

**File:** `src/args/commands.rs:152-224`

**Canonical options:**
- `--asr-engine` (Rev|Whisper|WhisperX|WhisperOai) → defaults to Rev
- `--asr-engine-custom <STR>` → overrides enum
- `--diarization` (Auto|Enabled|Disabled) → defaults to Auto
- `--wor` / `--nowor` (default false)
- `--merge-abbrev` / `--no-merge-abbrev` (default false)
- `--lang <STR>` (default "eng")
- `--num-speakers <INT>` or `-n` (default 2)
- `--batch-size <INT>` (default 8)

**Hidden BA2 compat aliases:**
```rust
--whisper          // bool, hide=true → asr_engine=Whisper
--whisperx         // bool, hide=true → asr_engine=WhisperX
--whisper-oai      // bool, hide=true → asr_engine=WhisperOai
--rev              // bool, hide=true → asr_engine=Rev
--diarize          // bool, hide=true → diarization=Enabled
--nodiarize        // bool, hide=true → diarization=Disabled
```

**Resolution in `build_typed_options()` (lines 90-136):**
```rust
// ASR engine: custom > compat bools > enum (default rev)
let asr_engine = if let Some(engine) = a.asr_engine_custom.as_deref() {
    engine.to_string()
} else if a.whisperx {
    "whisperx".into()
} else if a.whisper_oai {
    "whisper_oai".into()
} else if a.whisper {
    "whisper".into()
} else if a.rev {
    "rev".into()
} else {
    match a.asr_engine {
        AsrEngine::Rev => "rev".into(),
        AsrEngine::Whisper => "whisper".into(),
        AsrEngine::WhisperX => "whisperx".into(),
        AsrEngine::WhisperOai => "whisper_oai".into(),
    }
};

// Diarization: compat bools override enum (no precedence, explicit check)
let diarize = if a.diarize {
    true
} else if a.nodiarize {
    false
} else {
    match a.diarization {
        DiarizationMode::Auto | DiarizationMode::Disabled => false,
        DiarizationMode::Enabled => true,
    }
};

// Output variant selection based on diarize state
if diarize {
    Some(CommandOptions::TranscribeS(variant))
} else {
    Some(CommandOptions::Transcribe(variant))
}
```

**Key caveat:** If both `--diarize` and `--nodiarize` are passed, `--diarize` wins (checked first).

### 1.4 **BENCHMARK** Command

**File:** `src/args/commands.rs:334-394`

**Canonical options:**
- `--asr-engine` (Rev|Whisper|WhisperOai) → defaults to Rev (subset of transcribe)
- `--asr-engine-custom <STR>` → overrides enum
- `--wor` / `--nowor` (default false)
- `--merge-abbrev` / `--no-merge-abbrev` (default false)
- `--lang <STR>` (default "eng")
- `--num-speakers <INT>` or `-n` (default 2)
- `--bank <STR>` (optional, for server media mappings)
- `--subdir <STR>` (optional, under bank)

**Hidden BA2 compat aliases:**
```rust
--whisper          // bool, hide=true → asr_engine=Whisper
--whisper-oai      // bool, hide=true → asr_engine=WhisperOai
--rev              // bool, hide=true → asr_engine=Rev
```

**Resolution in `build_typed_options()` (lines 156-181):**
```rust
// ASR engine: custom > compat bools > enum (default rev)
let asr_engine = if let Some(engine) = a.asr_engine_custom.as_deref() {
    engine.to_string()
} else if a.whisper_oai {
    "whisper_oai".into()
} else if a.whisper {
    "whisper".into()
} else if a.rev {
    "rev".into()
} else {
    match a.asr_engine {
        BenchAsrEngine::Rev => "rev".into(),
        BenchAsrEngine::Whisper => "whisper".into(),
        BenchAsrEngine::WhisperOai => "whisper_oai".into(),
    }
};
```

### 1.5 **Global Options (All Commands)**

**File:** `src/args/global_opts.rs:1-94`

**Canonical options:**
- `-v, --verbose` (count, global)
- `--workers <N>` (optional usize)
- `--force-cpu` / `--no-force-cpu` (conflicting pair)
- `--server <URL>` (optional, env: BATCHALIGN_SERVER)
- `--override-cache` / `--use-cache` (conflicting pair)
- `--lazy-audio` / `--no-lazy-audio` (conflicting pair, default true)
- `--tui` / `--no-tui` (conflicting pair, default true)
- `--engine-overrides <JSON>` (optional)

**Hidden BA2 compat aliases (all no-op):**
```rust
--memlog                      // bool, hide=true, global
--mem-guard                   // bool, hide=true, global
--adaptive-workers            // bool, hide=true, global
--no-adaptive-workers         // bool, hide=true, global
--pool                        // bool, hide=true, global
--no-pool                     // bool, hide=true, global
--adaptive-safety-factor <F>  // f64 option, hide=true, global
--adaptive-warmup <N>         // usize option, hide=true, global
--shared-models               // bool, hide=true, global
--no-shared-models            // bool, hide=true, global
```

**Resolution in `build_typed_options()` (lines 45-50):**
```rust
let common = CommonOptions {
    override_cache: global.override_cache && !global.use_cache,
    lazy_audio: global.lazy_audio && !global.no_lazy_audio,
    engine_overrides: parse_engine_overrides(&global.engine_overrides),
    ..Default::default()
};
```

---

## 2. Existing Test Patterns and Helpers for Contract Tests

### 2.1 Test Infrastructure (`tests/common/mod.rs`)

**Key helpers:**

1. **`cli_cmd() → assert_cmd::Command`** (line 25-30)
   - Creates subprocess for `batchalign3` binary
   - Pre-clears `BATCHALIGN_SERVER` and sets `BATCHALIGN_NO_BROWSER=1`
   - Safe baseline for subprocess tests

2. **`CliHarness` struct** (line 32-75)
   - Isolated `HOME` + `.batchalign3` state dir
   - `harness.cmd()` returns command bound to isolated env
   - `harness.home_dir()`, `harness.state_dir()`, `harness.server_config_path()`
   - **Use case:** Tests that need sandbox isolation without touching real home

3. **`resolve_python() → Option<String>`** (line 79-114)
   - Prefers `.venv/bin/python3` in project root
   - Falls back to system `python3` if batchalign importable
   - Macro `require_python!()` for graceful skip if unavailable

4. **`start_test_server(python_path) → (String, TempDir)`** (line 131-186)
   - Spawns real batchalign server in test-echo mode (no ML models)
   - Binds to random port, returns base URL
   - Configures worker pool with `test_echo: true`
   - **Critical:** Test workers do echo IPC, not real NLP; safe for quick contracts

5. **CHAT content templates** (line 189-221)
   - `MINIMAL_CHAT` — bare valid @UTF8/@Begin/@End
   - `DUMMY_CHAT` — with @Options: dummy
   - `NOALIGN_CHAT` — with @Options: NoAlign (skip alignment)

6. **`default_options_for(command) → CommandOptions`** (line 223-297)
   - Factory for each command's default options
   - Covers all 10 processing commands + fallback
   - **Use case:** Avoid duplicating option structs in tests

7. **`run_job_to_completion(client, base_url, command, lang, files, options) → (JobInfo, Vec<FileResult>)`** (line 303-350)
   - Submit job, poll with 60s timeout, fetch results
   - Handles content-mode submissions (for transcribe, compatible with test-echo)
   - **Use case:** End-to-end without worrying about poll loop

8. **`poll_job_done(client, base_url, job_id) → JobInfo`** (line 353-379)
   - Generic poller, 60s deadline, 200ms sleep interval
   - Checks terminal states: Completed|Failed|Cancelled

### 2.2 Command Surface Manifest (`tests/command_surface_manifest.rs`)

**Key test patterns:**

1. **`SurfaceFamily` enum** (line 11-18)
   - `Processing`, `Server`, `Utility`
   - Organizational grouping for coverage expectations

2. **`CoverageExpectation` enum** (line 21-29)
   - `HelpContract` — visible in `--help`
   - `OptionMatrix` — systematic option coverage
   - `LegacyCompatibility` — compat aliases must parse

3. **`HiddenCompatCase` struct** (line 42-48)
   - `args: &[&str]` — CLI args to test
   - `hidden_flag: &str` — name of compat flag being tested
   - `help_scope: &[&str]` — which help context to check (e.g., `&["--help"]` vs `&["align", "--help"]`)
   - `note: &str` — rationale (e.g., "BA2 align alias for whisper UTR")

4. **`HIDDEN_COMPAT_CASES` constant** (line 98-237)
   - 37 test cases covering all hidden flags
   - **Global** (lines 99-158): `--memlog`, `--mem-guard`, `--adaptive-workers`, etc.
   - **Align** (lines 159-182): `--whisper`, `--rev`, `--whisper-fa`, `--wav2vec`
   - **Transcribe** (lines 183-218): `--whisper`, `--whisperx`, `--whisper-oai`, `--rev`, `--diarize`, `--nodiarize`
   - **Benchmark** (lines 219-237): `--whisper`, `--whisper-oai`, `--rev`

5. **Test implementations:**
   - `test_command_surface_manifest_has_unique_visible_commands()` (line 287-295)
   - `test_top_level_help_lists_all_manifested_commands()` (line 298-306)
   - `test_every_surface_group_declares_coverage_and_rationale()` (line 309-327)
   - **Key test:** `test_hidden_batchalign2_compat_flags_are_accepted_but_not_listed_in_help()` (line 330-342)
     - Runs each `HiddenCompatCase::args` → asserts success
     - Checks help output does NOT contain `hidden_flag`
     - Ensures compat flags parse but stay hidden from users

### 2.3 CLI Subprocess Tests (`tests/cli.rs`)

**Key patterns for fast contracts (no real ML):**

1. **Version/help tests** (line 18-41)
   - Verify binary path resolution, basic output parsing
   - Fast, no server

2. **Argument validation** (line 212-256)
   - Unknown subcommand → failure code 2
   - Missing paths → code 2, message contains "no input paths"
   - Nonexistent path → code 2, message contains "does not exist"

3. **Isolated harness tests** (line 281-313)
   - `cache stats`, `cache clear` on fresh HOME
   - `serve status` with unreachable server
   - All use `CliHarness` to avoid real state

4. **Real server e2e** (line 322-425: `cli_morphotag_real_server()`)
   - Spawns test-echo server
   - Runs `batchalign3 morphotag` as subprocess
   - Verifies CLI success, checks stderr for "All done" or "written"
   - **Pattern:** Spawn async server on separate tokio task, run CLI in blocking task
   - **Note:** Even with test-echo, morphotag may not produce "real" output, so only exit code is checked

### 2.4 Arg Parsing Unit Tests (`src/args/tests.rs`)

**Direct parsing tests (no subprocess):**

```rust
#[test]
fn parse_align_with_options() {
    let cli = Cli::parse_from([
        "batchalign3",
        "--verbose",
        "align",
        "input/",
        "-o",
        "output/",
        "--whisper-fa",  // compat bool
        "--pauses",
    ]);
    assert_eq!(cli.global.verbose, 1);
    if let Commands::Align(a) = &cli.command {
        assert!(a.whisper_fa);
        assert!(a.pauses);
    }
}

#[test]
fn parse_ba2_compat_flags() {
    let cli = Cli::parse_from([
        "batchalign3",
        "align",
        "--rev",      // compat bool
        "--wav2vec",  // compat bool
        "--no-merge-abbrev",
        "input/",
    ]);
    // assertions on parsed fields
}
```

---

## 3. Candidate Contract Tests for Alias/Canonical Equivalence, Precedence, and Invariants

### 3.1 **ALIGN Command Equivalence Tests**

**Location:** `tests/cli.rs` (add new test module or `tests/compat_equivalence.rs`)

**Test suite pattern:**

```rust
#[test]
fn align_whisper_alias_equals_canonical_enum() {
    // Both should parse and resolve to AppUtrEngine::Whisper
    let cli_compat = Cli::parse_from([
        "batchalign3", "align", ".", "--whisper"
    ]);
    let cli_canonical = Cli::parse_from([
        "batchalign3", "align", ".", "--utr-engine", "whisper"
    ]);
    
    if let Commands::Align(a1) = &cli_compat.command,
       let Commands::Align(a2) = &cli_canonical.command {
        assert_eq!(a1.whisper, true);
        assert_eq!(a2.utr_engine, UtrEngine::Whisper);
        
        // Both resolve to same canonical string
        let opts1 = build_typed_options(&cli_compat.command, &cli_compat.global).unwrap();
        let opts2 = build_typed_options(&cli_canonical.command, &cli_canonical.global).unwrap();
        
        if let CommandOptions::Align(AlignOptions { utr_engine: Some(AppUtrEngine::Whisper), .. }) = opts1 {},
        if let CommandOptions::Align(AlignOptions { utr_engine: Some(AppUtrEngine::Whisper), .. }) = opts2 {}
        // Both should be equivalent
    }
}

#[test]
fn align_custom_engine_overrides_compat_flag() {
    // Custom > compat bool > enum (verify precedence)
    let cli = Cli::parse_from([
        "batchalign3", "align", ".",
        "--utr-engine-custom", "custom_utr",
        "--whisper"  // This should be ignored
    ]);
    
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    if let CommandOptions::Align(AlignOptions { 
        utr_engine: Some(AppUtrEngine::Custom(name)), .. 
    }) = opts {
        assert_eq!(name.as_str(), "custom_utr");
    } else {
        panic!("Expected custom UTR engine");
    }
}

#[test]
fn align_fa_engine_whisper_fa_compat_alias() {
    // --whisper-fa should resolve to "whisper_fa" engine name
    let cli = Cli::parse_from([
        "batchalign3", "align", ".", "--whisper-fa"
    ]);
    
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    if let CommandOptions::Align(AlignOptions { fa_engine, .. }) = opts {
        assert_eq!(fa_engine, "whisper_fa");
    }
}

#[test]
fn align_boolean_pair_nowor_overrides_wor() {
    // wor=true && nowor=true → final value false (AND gate)
    let cli = Cli::parse_from([
        "batchalign3", "align", ".", "--wor", "--nowor"
    ]);
    
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    if let CommandOptions::Align(AlignOptions { wor, .. }) = opts {
        assert!(!wor);  // nowor wins (AND gate: true && !true = false)
    }
}

#[test]
fn align_no_utr_disables_utr_engine() {
    // utr=true && no_utr=true → utr_engine=None
    let cli = Cli::parse_from([
        "batchalign3", "align", ".", "--utr", "--no-utr"
    ]);
    
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    if let CommandOptions::Align(AlignOptions { utr_engine, .. }) = opts {
        assert!(utr_engine.is_none());
    }
}
```

**Tests to add:**
- `test_align_rev_alias_equals_canonical_enum()` (--rev vs --utr-engine rev)
- `test_align_wav2vec_alias_equals_canonical_enum()` (--wav2vec vs --fa-engine wav2vec)
- `test_align_whisper_fa_custom_overrides_compat_alias()` (--fa-engine-custom > --whisper-fa)
- `test_align_hidden_flags_not_in_help()` (subprocess test, verify --help output)

### 3.2 **TRANSCRIBE Command Equivalence Tests**

```rust
#[test]
fn transcribe_whisper_alias_equals_canonical_enum() {
    let cli_compat = Cli::parse_from([
        "batchalign3", "transcribe", ".", "--whisper"
    ]);
    let cli_canonical = Cli::parse_from([
        "batchalign3", "transcribe", ".", "--asr-engine", "whisper"
    ]);
    
    let opts1 = build_typed_options(&cli_compat.command, &cli_compat.global).unwrap();
    let opts2 = build_typed_options(&cli_canonical.command, &cli_canonical.global).unwrap();
    
    if let CommandOptions::Transcribe(TranscribeOptions { asr_engine: e1, .. }) = opts1,
       let CommandOptions::Transcribe(TranscribeOptions { asr_engine: e2, .. }) = opts2 {
        assert_eq!(e1, "whisper");
        assert_eq!(e2, "whisper");
    }
}

#[test]
fn transcribe_diarize_vs_nodiarize_boolean_precedence() {
    // If both --diarize and --nodiarize, diarize wins (checked first in resolution)
    let cli = Cli::parse_from([
        "batchalign3", "transcribe", ".", "--diarize", "--nodiarize"
    ]);
    
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    if let CommandOptions::TranscribeS(_) = opts {
        // TranscribeS variant means diarize=true
        // (transcribe.rs checks if diarize then returns TranscribeS)
    } else if let CommandOptions::Transcribe(_) = opts {
        panic!("Expected TranscribeS (diarize variant)");
    }
}

#[test]
fn transcribe_whisperx_alias_resolves_to_whisperx_engine() {
    let cli = Cli::parse_from([
        "batchalign3", "transcribe", ".", "--whisperx"
    ]);
    
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    if let CommandOptions::Transcribe(TranscribeOptions { asr_engine, .. }) = opts {
        assert_eq!(asr_engine, "whisperx");
    }
}

#[test]
fn transcribe_custom_asr_engine_overrides_all_compat() {
    // --asr-engine-custom > --whisperx / --whisper / --rev / etc. > enum
    let cli = Cli::parse_from([
        "batchalign3", "transcribe", ".",
        "--asr-engine-custom", "tencent",
        "--whisper",
        "--rev"
    ]);
    
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    if let CommandOptions::Transcribe(TranscribeOptions { asr_engine, .. }) = opts {
        assert_eq!(asr_engine, "tencent");
    }
}

#[test]
fn transcribe_diarization_enum_vs_compat_bool() {
    // --diarization enabled == --diarize
    let cli1 = Cli::parse_from([
        "batchalign3", "transcribe", ".", "--diarization", "enabled"
    ]);
    let cli2 = Cli::parse_from([
        "batchalign3", "transcribe", ".", "--diarize"
    ]);
    
    let opts1 = build_typed_options(&cli1.command, &cli1.global).unwrap();
    let opts2 = build_typed_options(&cli2.command, &cli2.global).unwrap();
    
    // Both should produce TranscribeS variant
    assert!(matches!(opts1, CommandOptions::TranscribeS(_)));
    assert!(matches!(opts2, CommandOptions::TranscribeS(_)));
}
```

**Tests to add:**
- `test_transcribe_whisper_oai_alias_equals_canonical()` (--whisper-oai vs --asr-engine whisper-oai)
- `test_transcribe_rev_alias_equals_canonical()` (--rev vs --asr-engine rev)
- `test_transcribe_nodiarize_resolves_to_false()` (--nodiarize → diarize=false)
- `test_transcribe_diarization_auto_default()` (no diarize flags → diarize=false per Auto default)
- `test_transcribe_hidden_flags_not_in_help()` (subprocess test)

### 3.3 **BENCHMARK Command Equivalence Tests**

```rust
#[test]
fn benchmark_asr_engine_compat_aliases_resolve_correctly() {
    let engines = vec![
        ("--whisper", "whisper"),
        ("--whisper-oai", "whisper_oai"),
        ("--rev", "rev"),
    ];
    
    for (flag, expected) in engines {
        let cli = Cli::parse_from([
            "batchalign3", "benchmark", ".", flag
        ]);
        let opts = build_typed_options(&cli.command, &cli.global).unwrap();
        if let CommandOptions::Benchmark(BenchmarkOptions { asr_engine, .. }) = opts {
            assert_eq!(asr_engine, expected);
        }
    }
}

#[test]
fn benchmark_custom_asr_engine_overrides_compat() {
    let cli = Cli::parse_from([
        "batchalign3", "benchmark", ".",
        "--asr-engine-custom", "custom_asr",
        "--whisper"  // should be ignored
    ]);
    
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    if let CommandOptions::Benchmark(BenchmarkOptions { asr_engine, .. }) = opts {
        assert_eq!(asr_engine, "custom_asr");
    }
}
```

**Tests to add:**
- `test_benchmark_rev_alias_canonical_equivalence()`
- `test_benchmark_hidden_flags_not_in_help()` (subprocess test)

### 3.4 **Global Options Compat Tests**

```rust
#[test]
fn global_override_cache_with_use_cache_gate() {
    // override_cache=true && use_cache=true → override_cache=false (AND gate)
    let cli = Cli::parse_from([
        "batchalign3", "--override-cache", "--use-cache", "version"
    ]);
    
    let opts = build_typed_options(&Commands::Version, &cli.global);
    // Common options have override_cache: override_cache && !use_cache
    // true && !true = false
}

#[test]
fn global_lazy_audio_default_true_unless_no_lazy_audio() {
    let cli1 = Cli::parse_from(["batchalign3", "version"]);
    let cli2 = Cli::parse_from(["batchalign3", "--no-lazy-audio", "version"]);
    
    // Default: lazy_audio=true
    assert!(cli1.global.lazy_audio);
    // Explicit disable
    assert!(cli2.global.no_lazy_audio);
    
    // Resolution gate: lazy_audio && !no_lazy_audio
    // cli1: true && !false = true
    // cli2: true && !true = false
}

#[test]
fn global_compat_flags_parse_but_are_noop() {
    // These should parse successfully with no effect on resolution
    let flags = vec![
        "--memlog",
        "--mem-guard",
        "--adaptive-workers",
        "--no-adaptive-workers",
        "--pool",
        "--no-pool",
        "--shared-models",
        "--no-shared-models",
    ];
    
    for flag in flags {
        let cli = Cli::parse_from([
            "batchalign3", flag, "version"
        ]);
        assert!(cli.command == Commands::Version);  // Should not affect command parsing
    }
}

#[test]
fn global_compat_options_with_values_parse_silently() {
    let cli1 = Cli::parse_from([
        "batchalign3", "--adaptive-safety-factor", "1.5", "version"
    ]);
    let cli2 = Cli::parse_from([
        "batchalign3", "--adaptive-warmup", "2", "version"
    ]);
    
    assert!(cli1.command == Commands::Version);
    assert!(cli2.command == Commands::Version);
}
```

**Tests to add:**
- `test_global_compat_flags_hidden_from_help()` (subprocess test for all BA2 no-ops)
- `test_global_tui_and_no_tui_defaults()` (--tui default true, --no-tui override)
- `test_global_force_cpu_gate()` (force_cpu && !no_force_cpu precedence)

### 3.5 **Integration Test: Hidden Flags Parse and Help Compliance**

**File:** `tests/command_surface_manifest.rs` (already exists as `test_hidden_batchalign2_compat_flags_are_accepted_but_not_listed_in_help`)

This is the **golden test** that validates the entire compat surface:

```rust
#[test]
fn hidden_batchalign2_compat_flags_are_accepted_but_not_listed_in_help() {
    for case in HIDDEN_COMPAT_CASES {  // 37 cases in constant
        // 1. Verify compat flag parses (binary exit code 0)
        cmd().args(case.args).assert().success();
        
        // 2. Verify it's NOT in relevant help output
        let help = help_output(case.help_scope);
        assert!(
            !help.contains(case.hidden_flag),
            "hidden compatibility flag `{}` leaked into help for {}",
            case.hidden_flag,
            case.note
        );
    }
}
```

**Coverage:** All 37 hidden compat cases verified in single test.

---

## 4. Implementation Checklist for Fast Contract Tests

### Phase 1: Unit-level Parsing Tests (zero dependencies, instant)

- [ ] `test_align_whisper_alias_equals_canonical_enum()` in `src/args/tests.rs`
- [ ] `test_align_custom_engine_overrides_compat_flag()` in `src/args/tests.rs`
- [ ] `test_transcribe_diarize_vs_nodiarize_boolean_precedence()` in `src/args/tests.rs`
- [ ] `test_benchmark_asr_engine_compat_aliases_resolve_correctly()` in `src/args/tests.rs`
- [ ] `test_global_lazy_audio_default_and_gate()` in `src/args/tests.rs`
- [ ] `test_global_compat_flags_parse_silently()` in `src/args/tests.rs`

**Run:** `cargo test -p batchalign-cli --lib args::tests` (< 1s)

### Phase 2: Subprocess Help Contract Tests (CLI binary, ~1s per test)

- [ ] Expand `tests/command_surface_manifest.rs` with explicit per-command subprocess assertions
- [ ] `test_align_hidden_flags_not_in_help()` — verify `--whisper`, `--rev`, `--whisper-fa`, `--wav2vec` absent from `align --help`
- [ ] `test_transcribe_hidden_flags_not_in_help()` — verify 6 compat flags absent from `transcribe --help`
- [ ] `test_benchmark_hidden_flags_not_in_help()` — verify 3 compat flags absent from `benchmark --help`
- [ ] `test_global_compat_flags_hidden()` — verify 10 global no-ops absent from top-level `--help`

**Run:** `cargo test -p batchalign-cli command_surface_manifest` (< 10s total)

### Phase 3: Round-trip Tests (with test-echo server, ~5-30s per test)

- [ ] Add `tests/compat_roundtrip.rs` with `HiddenCompatCase` executor
- [ ] `test_align_whisper_vs_canonical_roundtrip()` — submit same job twice, verify identical results
- [ ] `test_transcribe_whisper_vs_canonical_roundtrip()` — verify ASR output consistency
- [ ] `test_transcribe_diarize_flag_effects()` — verify output variant selection

**Run:** `cargo test -p batchalign-cli --test compat_roundtrip` (< 60s total, requires Python + batchalign)

---

## 5. Caveats and Special Behaviors

### 5.1 Hidden Flags in Help Output

**Mechanism:** clap's `#[arg(hide = true)]` attribute
- **Effect:** Flag is accepted by parser, but not listed in `--help` output
- **Verification:** `test_hidden_batchalign2_compat_flags_are_accepted_but_not_listed_in_help()` in `tests/command_surface_manifest.rs:330-342`
- **Caveat:** Flags are **still** visible in subcommand-specific help if user explicitly requests `align --help --whisper` (help output is generated after parsing, so parsed fields don't leak into help generation)

### 5.2 Boolean Pair Gates (AND logic)

All conflicting boolean pairs resolve with AND gates:

```rust
final_value = positive_flag && !negative_flag
```

**Examples:**
- `--wor && !--nowor` (default: true && !false = true)
- `--override-cache && !--use-cache` (default: false && !false = false)
- `--lazy-audio && !--no-lazy-audio` (default: true && !false = true)

**Edge case:** If user passes both flags, the AND gate enforces mutual exclusion (both true → false):
```bash
batchalign3 align --wor --nowor input/  # wor=true && !true = false
```

### 5.3 Engine Name Resolution Precedence

Three-tier precedence (applied in order, first match wins):

1. **Custom engine name** (`--*-engine-custom <STR>`)
2. **Compat bool flag** (`--whisper`, `--rev`, etc.)
3. **Enum default** (`--asr-engine rev`, defaults to rev)

**Example (transcribe):**
```rust
// Pseudocode from src/args/options.rs:90-112
if asr_engine_custom.is_some() {
    use custom
} else if whisper { use whisper } 
else if whisperx { use whisperx }
else if whisper_oai { use whisper_oai }
else if rev { use rev }
else {
    match asr_engine { ... }  // enum
}
```

### 5.4 Diarization Special Case (two-stage check)

Only **transcribe** has a two-stage diarization check:

```rust
let diarize = if a.diarize {
    true
} else if a.nodiarize {
    false
} else {
    match a.diarization { ... }
};
```

If both `--diarize` and `--nodiarize` are passed, `--diarize` wins (checked first).

### 5.5 Global Compat Flags Are True No-ops

All 10 global BA2 compat flags are **completely ignored** in code:

```rust
pub memlog: bool,                           // never read after parsing
pub mem_guard: bool,                        // never read after parsing
pub adaptive_workers: bool,                 // never read after parsing
pub no_adaptive_workers: bool,              // never read after parsing
pub pool: bool,                             // never read after parsing
pub no_pool: bool,                          // never read after parsing
pub adaptive_safety_factor: Option<f64>,    // never read after parsing
pub adaptive_warmup: Option<usize>,         // never read after parsing
pub shared_models: bool,                    // never read after parsing
pub no_shared_models: bool,                 // never read after parsing
```

They parse successfully but have no effect on `CommonOptions` or dispatch behavior.

### 5.6 Help Scope for Hidden Flags

`HiddenCompatCase::help_scope` specifies which help context to check:

- `&["--help"]` — top-level help (global flags)
- `&["align", "--help"]` — subcommand help (command-specific flags)
- `&["align", "--whisper", "--help"]` — help with compat flag present (should still hide the flag)

**Verification:** `help_output(scope)` in `tests/command_surface_manifest.rs:239-248` constructs output for given scope.

---

## 6. File Summary

| File | Role | Hidden Compat Coverage |
|------|------|------------------------|
| `src/args/mod.rs` | Cli struct, Commands enum | Entry point for parsing |
| `src/args/global_opts.rs` | GlobalOpts with 10 BA2 no-ops | 10 hidden global flags |
| `src/args/commands.rs` | Per-command arg structs (Align, Transcribe, Benchmark, etc.) | 16 command-specific hidden flags |
| `src/args/options.rs` | `build_typed_options()` resolution logic | 3-tier precedence, gate logic |
| `src/args/tests.rs` | Unit parsing tests | Needs expansion for compat coverage |
| `tests/command_surface_manifest.rs` | Golden test for compat surface | 37 HiddenCompatCases, validates parse + hide |
| `tests/common/mod.rs` | Test helpers, server startup, harness | `cli_cmd()`, `CliHarness`, `start_test_server()` |
| `tests/cli.rs` | Subprocess binary tests | Help, version, argument validation |

---

## 7. Summary

**Canonical vs. Compat Resolution:**
- Three-tier: custom > compat bool > enum (see `src/args/options.rs`)
- Boolean pairs use AND gates for mutual exclusion
- Global compat flags parse but are complete no-ops

**Test Infrastructure:**
- Unit tests: `clap::Parser::parse_from()` on arg slices (instant)
- Subprocess tests: `assert_cmd` with isolated HOME (< 1s per test)
- Integration tests: `start_test_server()` with test-echo (fast, no real ML)

**Key Contract Tests to Add:**
1. Engine name equivalence (compat bool == canonical enum)
2. Precedence verification (custom > compat > enum)
3. Boolean pair gates (AND logic)
4. Hidden flag absence from help (37 cases in manifest)
5. Diarization special case (two-stage check, precedence)

**37 Hidden Compat Cases:**
- 10 global no-ops (--memlog, --mem-guard, --adaptive-workers, etc.)
- 4 align flags (--whisper, --rev, --whisper-fa, --wav2vec)
- 6 transcribe flags (--whisper, --whisperx, --whisper-oai, --rev, --diarize, --nodiarize)
- 3 benchmark flags (--whisper, --whisper-oai, --rev)

All are verified to parse and remain hidden from `--help` output via `tests/command_surface_manifest.rs`.
