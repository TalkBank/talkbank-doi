# Batchalign3 CLI Compat - Key Code Snippets & Line References

**Status:** Reference
**Last updated:** 2026-03-15

## Overview
This document provides concrete code references for the three critical areas of the hidden BA2 compatibility surface.

---

## 1. Arg Definition Layer (Parse)

### Global Options with BA2 No-ops
**File:** `src/args/global_opts.rs:1-94`

```rust
/// Global options that apply to every command.
#[derive(Args, Debug, Clone)]
pub struct GlobalOpts {
    /// Increase verbosity (-v, -vv, -vvv).
    #[arg(short, long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Maximum worker processes.
    #[arg(long, global = true)]
    pub workers: Option<usize>,

    // === BA2 Compatibility Flags (All No-ops) ===
    
    /// BA2 compatibility flag (currently a no-op in Rust CLI).
    #[arg(long, global = true, hide = true)]
    pub memlog: bool,                                    // LINE 15-16

    /// BA2 compatibility flag (currently a no-op in Rust CLI).
    #[arg(long = "mem-guard", global = true, hide = true)]
    pub mem_guard: bool,                                 // LINE 18-20

    /// BA2 compatibility flag (currently a no-op in Rust CLI).
    #[arg(long = "adaptive-workers", global = true, hide = true)]
    pub adaptive_workers: bool,                         // LINE 22-24

    /// BA2 compatibility flag (currently a no-op in Rust CLI).
    #[arg(long = "no-adaptive-workers", global = true, hide = true)]
    pub no_adaptive_workers: bool,                      // LINE 26-28

    /// BA2 compatibility flag (currently a no-op in Rust CLI).
    #[arg(long, global = true, hide = true)]
    pub pool: bool,                                     // LINE 30-32

    /// BA2 compatibility flag (currently a no-op in Rust CLI).
    #[arg(long = "no-pool", global = true, hide = true)]
    pub no_pool: bool,                                  // LINE 34-36

    /// BA2 compatibility option (currently a no-op in Rust CLI).
    #[arg(long = "adaptive-safety-factor", global = true, hide = true)]
    pub adaptive_safety_factor: Option<f64>,            // LINE 38-40

    /// BA2 compatibility option (currently a no-op in Rust CLI).
    #[arg(long = "adaptive-warmup", global = true, hide = true)]
    pub adaptive_warmup: Option<usize>,                 // LINE 42-44

    /// BA2 compatibility flag (currently a no-op in Rust CLI).
    #[arg(long = "shared-models", global = true, hide = true)]
    pub shared_models: bool,                            // LINE 54-56

    /// BA2 compatibility flag (currently a no-op in Rust CLI).
    #[arg(long = "no-shared-models", global = true, hide = true)]
    pub no_shared_models: bool,                         // LINE 58-60
    
    // ... other canonical options ...
}
```

**Key pattern:** All BA2 no-ops use `#[arg(hide = true)]` and `global = true`.

### Align Command with Hidden Compat Aliases
**File:** `src/args/commands.rs:81-150`

```rust
/// Arguments for the `align` command.
#[derive(Args, Debug, Clone)]
pub struct AlignArgs {
    /// Shared file I/O options.
    #[command(flatten)]
    pub common: CommonOpts,

    /// UTR engine: rev (default) or whisper.
    #[arg(long, value_enum, default_value_t)]
    pub utr_engine: UtrEngine,                          // LINE 90 - CANONICAL

    /// Explicit custom UTR engine name (e.g. tencent_utr).
    /// Overrides --utr-engine when set.
    #[arg(long)]
    pub utr_engine_custom: Option<String>,              // LINE 95 - CUSTOM (HIGHEST PRIORITY)

    /// Forced-alignment engine: wav2vec (default) or whisper.
    #[arg(long, value_enum, default_value_t)]
    pub fa_engine: FaEngine,                            // LINE 99 - CANONICAL

    /// Explicit custom FA engine name (e.g. wav2vec_fa_canto).
    /// Overrides --fa-engine when set.
    #[arg(long)]
    pub fa_engine_custom: Option<String>,               // LINE 104 - CUSTOM (HIGHEST PRIORITY)

    // ... pauses, wor/nowor, merge_abbrev/no_merge_abbrev, utr/no_utr ...

    // -- Hidden BA2 compatibility aliases --
    
    /// BA2 compat: use --utr-engine whisper instead.
    #[arg(long, hide = true)]
    pub whisper: bool,                                  // LINE 136-137 - COMPAT

    /// BA2 compat: use --utr-engine rev instead.
    #[arg(long, hide = true)]
    pub rev: bool,                                      // LINE 140-141 - COMPAT

    /// BA2 compat: use --fa-engine whisper instead.
    #[arg(long, hide = true)]
    pub whisper_fa: bool,                               // LINE 144-145 - COMPAT

    /// BA2 compat: use --fa-engine wav2vec instead.
    #[arg(long, hide = true)]
    pub wav2vec: bool,                                  // LINE 148-149 - COMPAT
}
```

**Precedence visualized:**
```
utr_engine_custom (--utr-engine-custom)   [HIGHEST]
    ↓
whisper || rev (--whisper, --rev)
    ↓
utr_engine enum (--utr-engine)            [LOWEST]
```

### Transcribe Command with Diarization and Engine Compat
**File:** `src/args/commands.rs:152-224`

```rust
/// Arguments for the `transcribe` command.
#[derive(Args, Debug, Clone)]
pub struct TranscribeArgs {
    #[command(flatten)]
    pub common: CommonOpts,

    /// ASR engine: rev (default), whisper, whisperx, or whisper-oai.
    #[arg(long, value_enum, default_value_t)]
    pub asr_engine: AsrEngine,                          // LINE 161 - CANONICAL

    /// Explicit custom ASR engine name (e.g. tencent, funaudio).
    /// Overrides --asr-engine when set.
    #[arg(long)]
    pub asr_engine_custom: Option<String>,              // LINE 166 - CUSTOM

    /// Speaker diarization mode: auto (default), enabled, or disabled.
    #[arg(long, value_enum, default_value_t)]
    pub diarization: DiarizationMode,                   // LINE 170 - CANONICAL ENUM

    // ... wor/nowor, merge_abbrev/no_merge_abbrev, lang, num_speakers, batch_size ...

    // -- Hidden BA2 compatibility aliases --
    
    /// BA2 compat: use --asr-engine whisper instead.
    #[arg(long, hide = true)]
    pub whisper: bool,                                  // LINE 202-203 - COMPAT

    /// BA2 compat: use --asr-engine whisperx instead.
    #[arg(long, hide = true)]
    pub whisperx: bool,                                 // LINE 206-207 - COMPAT

    /// BA2 compat: use --asr-engine whisper-oai instead.
    #[arg(long, hide = true)]
    pub whisper_oai: bool,                              // LINE 210-211 - COMPAT

    /// BA2 compat: use --asr-engine rev instead.
    #[arg(long, hide = true)]
    pub rev: bool,                                      // LINE 214-215 - COMPAT

    /// BA2 compat: use --diarization enabled instead.
    #[arg(long, hide = true)]
    pub diarize: bool,                                  // LINE 218-219 - COMPAT

    /// BA2 compat: use --diarization disabled instead.
    #[arg(long, hide = true)]
    pub nodiarize: bool,                                // LINE 222-223 - COMPAT
}
```

**Key insight:** Both engine resolution AND diarization state have compat bools.

### Benchmark Command (Subset of Transcribe)
**File:** `src/args/commands.rs:334-394`

```rust
/// Arguments for the `benchmark` command.
#[derive(Args, Debug, Clone)]
pub struct BenchmarkArgs {
    #[command(flatten)]
    pub common: CommonOpts,

    /// ASR engine: rev (default), whisper, or whisper-oai.
    #[arg(long, value_enum, default_value_t)]
    pub asr_engine: BenchAsrEngine,                     // LINE 343 - CANONICAL (SUBSET)

    /// Explicit custom ASR engine name (e.g. tencent, funaudio).
    /// Overrides --asr-engine when set.
    #[arg(long)]
    pub asr_engine_custom: Option<String>,              // LINE 348 - CUSTOM

    // ... lang, num_speakers, wor/nowor, merge_abbrev, bank, subdir ...

    // -- Hidden BA2 compatibility aliases --
    
    /// BA2 compat: use --asr-engine whisper instead.
    #[arg(long, hide = true)]
    pub whisper: bool,                                  // LINE 384-385 - COMPAT

    /// BA2 compat: use --asr-engine whisper-oai instead.
    #[arg(long, hide = true)]
    pub whisper_oai: bool,                              // LINE 388-389 - COMPAT

    /// BA2 compat: use --asr-engine rev instead.
    #[arg(long, hide = true)]
    pub rev: bool,                                      // LINE 392-393 - COMPAT
}
```

**Note:** Benchmark only has 3 ASR engine choices (no WhisperX), so only 3 compat flags.

---

## 2. Resolution Layer (Build & Gate Logic)

### The Golden Resolution Function
**File:** `src/args/options.rs:41-193`

```rust
/// Build typed command options from parsed CLI args.
///
/// Returns `None` for non-processing commands (serve, jobs, version, etc.).
pub fn build_typed_options(cmd: &Commands, global: &GlobalOpts) -> Option<CommandOptions> {
    let common = CommonOptions {
        override_cache: global.override_cache && !global.use_cache,       // LINE 46 - GATE
        lazy_audio: global.lazy_audio && !global.no_lazy_audio,           // LINE 47 - GATE
        engine_overrides: parse_engine_overrides(&global.engine_overrides),
        ..Default::default()
    };

    match cmd {
        Commands::Align(a) => {
            // === 3-TIER FA ENGINE RESOLUTION ===
            let fa_engine = if let Some(engine) = a.fa_engine_custom.as_deref() {
                engine.to_string()              // LINE 55 - TIER 1: CUSTOM (HIGHEST)
            } else if a.whisper_fa {
                // BA2 compat alias                // LINE 58 - TIER 2: COMPAT
                "whisper_fa".into()
            } else {
                match a.fa_engine {              // LINE 62 - TIER 3: ENUM (LOWEST)
                    FaEngine::Wav2vec => "wav2vec_fa".into(),
                    FaEngine::Whisper => "whisper_fa".into(),
                }
            };

            // === 3-TIER UTR ENGINE RESOLUTION (WITH GATE) ===
            let utr_engine = if a.utr && !a.no_utr {        // LINE 65 - GATE: utr && !no_utr
                let utr = if let Some(engine) = a.utr_engine_custom.as_deref() {
                    AppUtrEngine::Custom(CustomEngineName::new(engine))  // LINE 67 - TIER 1: CUSTOM
                } else if a.whisper && !a.rev {             // LINE 69 - TIER 2: COMPAT (WITH SECOND CHECK)
                    AppUtrEngine::Whisper
                } else {
                    match a.utr_engine {                     // LINE 73 - TIER 3: ENUM
                        CliUtrEngine::Rev => AppUtrEngine::RevAi,
                        CliUtrEngine::Whisper => AppUtrEngine::Whisper,
                    }
                };
                Some(utr)
            } else {
                None                             // LINE 79 - UTR DISABLED
            };

            Some(CommandOptions::Align(AlignOptions {
                common,
                fa_engine,
                utr_engine,
                pauses: a.pauses,
                wor: a.wor && !a.nowor,          // LINE 86 - GATE: wor && !nowor
                merge_abbrev: a.merge_abbrev && !a.no_merge_abbrev,  // LINE 87 - GATE
            }))
        }
        
        Commands::Transcribe(a) => {
            // === 3-TIER ASR ENGINE RESOLUTION (SEQUENTIAL IF/ELSE) ===
            let asr_engine = if let Some(engine) = a.asr_engine_custom.as_deref() {
                engine.to_string()              // LINE 92 - TIER 1: CUSTOM (HIGHEST)
            } else if a.whisperx {
                // BA2 compat alias                // LINE 94 - TIER 2: COMPAT
                "whisperx".into()
            } else if a.whisper_oai {
                "whisper_oai".into()             // LINE 97 - TIER 2: COMPAT (CHECKED FIRST)
            } else if a.whisper {
                "whisper".into()                 // LINE 100 - TIER 2: COMPAT
            } else if a.rev {
                "rev".into()                     // LINE 103 - TIER 2: COMPAT
            } else {
                match a.asr_engine {             // LINE 106 - TIER 3: ENUM (LOWEST)
                    AsrEngine::Rev => "rev".into(),
                    AsrEngine::Whisper => "whisper".into(),
                    AsrEngine::WhisperX => "whisperx".into(),
                    AsrEngine::WhisperOai => "whisper_oai".into(),
                }
            };

            // === TWO-STAGE DIARIZATION CHECK (UNIQUE TO TRANSCRIBE) ===
            let diarize = if a.diarize {         // LINE 114 - CHECK --diarize FIRST
                true
            } else if a.nodiarize {              // LINE 116 - CHECK --nodiarize SECOND
                false
            } else {
                match a.diarization {            // LINE 119 - CHECK ENUM LAST
                    DiarizationMode::Auto | DiarizationMode::Disabled => false,
                    DiarizationMode::Enabled => true,
                }
            };

            let variant = TranscribeOptions {
                common,
                asr_engine,
                diarize,
                wor: a.wor && !a.nowor,          // LINE 128 - GATE
                merge_abbrev: a.merge_abbrev && !a.no_merge_abbrev,  // LINE 129 - GATE
                batch_size: a.batch_size,
            };

            // === OUTPUT VARIANT SELECTION BASED ON DIARIZE ===
            if diarize {
                Some(CommandOptions::TranscribeS(variant))  // LINE 133 - WITH DIARIZATION
            } else {
                Some(CommandOptions::Transcribe(variant))   // LINE 135 - WITHOUT DIARIZATION
            }
        }

        Commands::Benchmark(a) => {
            // === SIMILAR 3-TIER ASR ENGINE RESOLUTION ===
            let asr_engine = if let Some(engine) = a.asr_engine_custom.as_deref() {
                engine.to_string()              // LINE 158 - TIER 1: CUSTOM
            } else if a.whisper_oai {
                // BA2 compat alias                // LINE 160 - TIER 2: COMPAT (CHECKED FIRST)
                "whisper_oai".into()
            } else if a.whisper {
                "whisper".into()                 // LINE 163 - TIER 2: COMPAT
            } else if a.rev {
                "rev".into()                     // LINE 166 - TIER 2: COMPAT
            } else {
                match a.asr_engine {             // LINE 169 - TIER 3: ENUM
                    BenchAsrEngine::Rev => "rev".into(),
                    BenchAsrEngine::Whisper => "whisper".into(),
                    BenchAsrEngine::WhisperOai => "whisper_oai".into(),
                }
            };

            Some(CommandOptions::Benchmark(BenchmarkOptions {
                common,
                asr_engine,
                wor: a.wor && !a.nowor,          // LINE 178 - GATE
                merge_abbrev: a.merge_abbrev && !a.no_merge_abbrev,  // LINE 179 - GATE
            }))
        }

        // ... other commands return None or their own options ...
        _ => None,
    }
}
```

**Key lines summary:**
- **Align FA engine resolution:** 54-64 (custom > compat > enum)
- **Align UTR engine resolution:** 65-80 (custom > compat > enum, WITH gate on utr && !no_utr)
- **Transcribe ASR engine resolution:** 91-112 (custom > compat (multi-stage) > enum)
- **Transcribe diarization:** 114-122 (two-stage: diarize > nodiarize > enum)
- **Benchmark ASR engine resolution:** 157-174 (same as transcribe, 3 engine subset)
- **Gate logic examples:** 46-47, 86-87, 128-129 (AND logic: positive && !negative)

---

## 3. Test Layer (Verification)

### Golden Hidden Compat Test
**File:** `tests/command_surface_manifest.rs:98-237, 330-342`

```rust
/// One hidden compatibility flag that should parse successfully while staying
/// absent from normal help output.
#[derive(Clone, Copy, Debug)]
struct HiddenCompatCase {
    args: &'static [&'static str],
    hidden_flag: &'static str,
    help_scope: &'static [&'static str],
    note: &'static str,
}

const HIDDEN_COMPAT_CASES: &[HiddenCompatCase] = &[
    // ========== GLOBAL FLAGS (10) ==========
    HiddenCompatCase {
        args: &["--memlog", "version"],           // LINE 100
        hidden_flag: "--memlog",
        help_scope: &["--help"],
        note: "global BA2 no-op flag",
    },
    HiddenCompatCase {
        args: &["--mem-guard", "version"],        // LINE 105
        hidden_flag: "--mem-guard",
        help_scope: &["--help"],
        note: "global BA2 no-op flag",
    },
    // ... (8 more global no-ops: adaptive-workers, no-adaptive-workers, pool, 
    //      no-pool, shared-models, no-shared-models, adaptive-safety-factor, adaptive-warmup) ...

    // ========== ALIGN COMMAND (4) ==========
    HiddenCompatCase {
        args: &["align", "--whisper", "--help"],  // LINE 160
        hidden_flag: "--whisper",
        help_scope: &["align", "--help"],
        note: "BA2 align alias for whisper UTR",
    },
    HiddenCompatCase {
        args: &["align", "--rev", "--help"],      // LINE 165
        hidden_flag: "--rev",
        help_scope: &["align", "--help"],
        note: "BA2 align alias for Rev UTR",
    },
    HiddenCompatCase {
        args: &["align", "--whisper-fa", "--help"],  // LINE 171
        hidden_flag: "--whisper-fa",
        help_scope: &["align", "--help"],
        note: "BA2 align alias for whisper FA",
    },
    HiddenCompatCase {
        args: &["align", "--wav2vec", "--help"],  // LINE 177
        hidden_flag: "--wav2vec",
        help_scope: &["align", "--help"],
        note: "BA2 align alias for wav2vec FA",
    },

    // ========== TRANSCRIBE COMMAND (6) ==========
    HiddenCompatCase {
        args: &["transcribe", "--whisper", "--help"],  // LINE 184
        hidden_flag: "--whisper",
        help_scope: &["transcribe", "--help"],
        note: "BA2 transcribe alias for whisper ASR",
    },
    HiddenCompatCase {
        args: &["transcribe", "--whisperx", "--help"],  // LINE 189
        hidden_flag: "--whisperx",
        help_scope: &["transcribe", "--help"],
        note: "BA2 transcribe alias for whisperx ASR",
    },
    HiddenCompatCase {
        args: &["transcribe", "--whisper-oai", "--help"],  // LINE 195
        hidden_flag: "--whisper-oai",
        help_scope: &["transcribe", "--help"],
        note: "BA2 transcribe alias for whisper-oai ASR",
    },
    HiddenCompatCase {
        args: &["transcribe", "--rev", "--help"],  // LINE 201
        hidden_flag: "--rev",
        help_scope: &["transcribe", "--help"],
        note: "BA2 transcribe alias for Rev ASR",
    },
    HiddenCompatCase {
        args: &["transcribe", "--diarize", "--help"],  // LINE 207
        hidden_flag: "--diarize",
        help_scope: &["transcribe", "--help"],
        note: "BA2 transcribe alias for enabled diarization",
    },
    HiddenCompatCase {
        args: &["transcribe", "--nodiarize", "--help"],  // LINE 213
        hidden_flag: "--nodiarize",
        help_scope: &["transcribe", "--help"],
        note: "BA2 transcribe alias for disabled diarization",
    },

    // ========== BENCHMARK COMMAND (3) ==========
    HiddenCompatCase {
        args: &["benchmark", "--whisper", "--help"],  // LINE 220
        hidden_flag: "--whisper",
        help_scope: &["benchmark", "--help"],
        note: "BA2 benchmark alias for whisper ASR",
    },
    HiddenCompatCase {
        args: &["benchmark", "--whisper-oai", "--help"],  // LINE 225
        hidden_flag: "--whisper-oai",
        help_scope: &["benchmark", "--help"],
        note: "BA2 benchmark alias for whisper-oai ASR",
    },
    HiddenCompatCase {
        args: &["benchmark", "--rev", "--help"],  // LINE 231
        hidden_flag: "--rev",
        help_scope: &["benchmark", "--help"],
        note: "BA2 benchmark alias for Rev ASR",
    },
];

// ========== THE GOLDEN TEST ==========

#[test]
fn hidden_batchalign2_compat_flags_are_accepted_but_not_listed_in_help() {
    for case in HIDDEN_COMPAT_CASES {
        // 1. Verify flag parses (exit code 0)
        cmd().args(case.args).assert().success();        // LINE 332

        // 2. Verify flag NOT in help output
        let help = help_output(case.help_scope);         // LINE 334
        assert!(
            !help.contains(case.hidden_flag),            // LINE 335
            "hidden compatibility flag `{}` leaked into help for {}",
            case.hidden_flag,
            case.note
        );
    }
}
```

**Test invariants verified by this single test:**
1. All 37 hidden flags parse successfully (cmd().args(case.args).assert().success())
2. All 37 flags are hidden from help output (assert!(!help.contains(...)))
3. Help scope checking (both global and command-specific)
4. Comprehensive coverage (10 global + 4 align + 6 transcribe + 3 benchmark = 37)

### Unit Parse Test Example
**File:** `src/args/tests.rs` (needs expansion)

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
        "--whisper-fa",              // BA2 COMPAT FLAG
        "--pauses",
    ]);
    assert_eq!(cli.global.verbose, 1);
    if let Commands::Align(a) = &cli.command {
        assert!(a.whisper_fa);       // VERIFY FLAG PARSED
        assert!(a.pauses);
        assert_eq!(a.common.output.as_deref(), Some("output/"));
    } else {
        panic!("expected Align");
    }
}

#[test]
fn parse_ba2_compat_flags() {
    let cli = Cli::parse_from([
        "batchalign3",
        "align",
        "--rev",                     // BA2 COMPAT: UTR ENGINE
        "--wav2vec",                 // BA2 COMPAT: FA ENGINE
        "--no-merge-abbrev",         // BOOLEAN PAIR
        "input/",
    ]);
    if let Commands::Align(a) = &cli.command {
        assert!(a.rev);              // VERIFY COMPAT FLAGS PARSED
        assert!(a.wav2vec);
        assert!(a.no_merge_abbrev);
    } else {
        panic!("expected Align");
    }
}
```

**Example resolution test (recommended to add):**

```rust
#[test]
fn align_custom_engine_overrides_compat_flag() {
    let cli = Cli::parse_from([
        "batchalign3", "align", ".",
        "--utr-engine-custom", "custom_utr",
        "--whisper"                  // This should be ignored
    ]);
    
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    if let CommandOptions::Align(AlignOptions { 
        utr_engine: Some(AppUtrEngine::Custom(name)), .. 
    }) = opts {
        assert_eq!(name.as_str(), "custom_utr");  // CUSTOM WINS
    } else {
        panic!("Expected custom UTR engine, custom > compat rule violated");
    }
}

#[test]
fn transcribe_diarize_vs_nodiarize_precedence() {
    // If both --diarize and --nodiarize passed, --diarize wins
    let cli = Cli::parse_from([
        "batchalign3", "transcribe", ".",
        "--diarize",                 // Checked first
        "--nodiarize"                // Checked second
    ]);
    
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    // diarize=true means output variant is TranscribeS
    assert!(matches!(opts, CommandOptions::TranscribeS(_)));
}
```

---

## Summary of Key Code Locations

| Concern | File | Lines | Key Function/Struct |
|---------|------|-------|-------------------|
| Parse: Global no-ops | `src/args/global_opts.rs` | 15-60 | `GlobalOpts` struct |
| Parse: Align compat | `src/args/commands.rs` | 134-150 | `AlignArgs` (whisper, rev, whisper_fa, wav2vec) |
| Parse: Transcribe compat | `src/args/commands.rs` | 200-224 | `TranscribeArgs` (whisper, whisperx, whisper_oai, rev, diarize, nodiarize) |
| Parse: Benchmark compat | `src/args/commands.rs` | 382-394 | `BenchmarkArgs` (whisper, whisper_oai, rev) |
| Resolve: All commands | `src/args/options.rs` | 44-193 | `build_typed_options()` |
| Resolve: Align FA/UTR | `src/args/options.rs` | 54-88 | Align branch with 3-tier resolution |
| Resolve: Transcribe ASR | `src/args/options.rs` | 91-136 | Transcribe branch with diarization |
| Resolve: Benchmark ASR | `src/args/options.rs` | 156-181 | Benchmark branch |
| Test: Golden manifest | `tests/command_surface_manifest.rs` | 98-237, 330-342 | `HIDDEN_COMPAT_CASES` + test function |
| Test: Unit parsing | `src/args/tests.rs` | 1-28KB | Various parse tests (needs compat expansion) |
| Test: Subprocess | `tests/cli.rs` | 1-426 | `help_lists_visible_commands()` + others |

---

## Implementation Reference

To add new compat tests, follow this pattern:

```rust
// Step 1: Parse verification (src/args/tests.rs)
#[test]
fn parse_new_compat_flag() {
    let cli = Cli::parse_from(["batchalign3", "command", ".", "--new-compat"]);
    // Assert flag parsed correctly
}

// Step 2: Resolution verification (src/args/tests.rs)
#[test]
fn resolve_new_compat_flag_to_canonical() {
    let cli = Cli::parse_from(["batchalign3", "command", ".", "--new-compat"]);
    let opts = build_typed_options(&cli.command, &cli.global).unwrap();
    // Assert resolved to canonical engine name
}

// Step 3: Help verification (tests/command_surface_manifest.rs)
// Add to HIDDEN_COMPAT_CASES constant:
HiddenCompatCase {
    args: &["command", "--new-compat", "--help"],
    hidden_flag: "--new-compat",
    help_scope: &["command", "--help"],
    note: "BA2 command alias for ...",
}
```

All three must pass for complete contract coverage.
