# Batchalign3 & TalkBank Tools CLI Plumbing Audit (Domain 1)

**Status:** Draft
**Last updated:** 2026-03-16

## 1. Executive Summary & Context

This report documents a deep-dive codebase investigation focusing on Domain 1 of our audit plan: **Command-Line Option Plumbing**. The audit traces the lifecycle of command-line arguments starting from user invocation, passing through the CLI parsing layers of both `batchalign3` and `talkbank-tools`, down to model inference and post-processing.

**Baseline Context:**
As detailed in `batchalign3/book/src/migration/index.md`, the migration from `batchalign2` (specifically the baseline commit `84ad500` from Jan 9, 2026) to `batchalign3` is a structural rewrite rather than a patch. The pipeline shifted from ad-hoc string manipulations in Python to a typed Rust AST (Abstract Syntax Tree) model. A key requirement is that the functioning of CLI parameters remains strictly on par with the Jan 9 baseline, especially regarding non-English extensions (HK/Cantonese architectures) which were previously housed in custom branches or forks (e.g., `BatchalignHK`).

**Summary of Findings:**
Overall, the architectural pivot to centralize parameter resolution via strongly-typed options (`CommandOptions`) in Rust is well executed. The boundaries between Rust control-plane, Python ML workers, and Rust-based post-processing are logically separated. However, we found a few structural nitpicks, silently handled serialization failures, and architectural oddities—especially concerning how non-English processing (like Cantonese) bridges the Rust-Python boundary.

---

## 2. Plumbing Trace: `talkbank-tools`

The `talkbank-tools` repository handles pure CHAT specification, validation, and transformation. Its command-line interface acts as the reference implementation.

### 2.1 Argument Parsing
The entry point is `talkbank-tools/crates/talkbank-cli/src/cli/args/core.rs`. `clap` derives a robust CLI tree with `Cli` acting as the root. It branches into several `Commands` such as `Validate`, `Normalize`, `ToJson`, `FromJson`, `Clan`, etc.

The attributes are thoroughly typed (e.g., `OutputFormat`, `TuiMode`, `ParserKindArg`).
Global flags like `verbose`, `tui_mode`, `log_format` are passed gracefully down the call stack.

### 2.2 Subcommand Dispatch (`Validate`)
For the primary validation workflow:
- The command arguments from `Commands::Validate` are mapped into `ValidateCommandOptions` (see `talkbank-cli/src/commands/validate/mod.rs`).
- `ValidateCommandOptions` aggregates logically separated structs:
  - `ValidateCommandRules` (`alignment`, `roundtrip`, `parser_kind`)
  - `ValidateCommandExecution` (`cache_refresh`, `jobs`, `max_errors`)
  - `ValidateCommandPresentation` (`format`, `quiet`, `audit_output`, `interface`)

**Nitpick / Architecture Review:**
The grouping here is excellent and highly cohesive. By packing raw CLI primitives into Domain logic representations (`Rules`, `Execution`, `Presentation`), the core `talkbank_transform::validation_runner` is completely shielded from `clap` or CLI-specific concerns.

### 2.3 Non-English Special Cases in `talkbank-tools`
Because `talkbank-tools` is an agnostic CHAT engine, it does not do model inference. However, it *does* enforce language policies based on CHAT metadata.
- In `talkbank-tools/crates/talkbank-model/src/validation/context.rs`, the constant `LANGUAGES_ALLOWING_NUMBERS` strictly permits numeric digits (`0-9`) in specific languages like `"zho"`, `"yue"` (Cantonese), `"tha"`, etc. 
- The language codes are properly parsed from the `@Languages` header and injected into the `SharedValidationData` which cascades through the AST validation passes.
- **Verdict:** Safely plumbed. `yue` correctly bypasses E220 digit validation checks natively.

---

## 3. Plumbing Trace: `batchalign3`

The `batchalign3` crate introduces significant complexity, acting as a Rust web/daemon server that orchestrates Python ML inference workers.

### 3.1 CLI Ingestion & Compatibility Aliases
The CLI is defined in `batchalign3/crates/batchalign-cli/src/args/global_opts.rs` and `options.rs`.

- **GlobalOpts:** Contains server flags, `--force-cpu`, `--engine-overrides`, and backward-compatible BA2 no-ops (`--memlog`, `--adaptive-workers`, etc. which are appropriately marked `hide = true`).
- **Command resolution:** The `build_typed_options()` function inside `options.rs` collapses `clap` commands into a heavily modeled `CommandOptions` enum (e.g., `CommandOptions::Align`, `CommandOptions::Transcribe`).
- **Legacy mappings:** `whisperx`, `whisper_fa` boolean flags are consumed and coerced into string identifiers (`"whisperx"`, `"whisper_fa"`) to match BA2 baseline options. Same for `--diarize` overriding `DiarizationMode::Auto`.

**Critique - Silent Degradation:**
In `batchalign3/crates/batchalign-cli/src/args/options.rs`, the `parse_engine_overrides` function parses a JSON string:
```rust
pub fn parse_engine_overrides(input: &Option<String>) -> BTreeMap<String, String> {
    match input.as_deref() {
        None | Some("") => BTreeMap::new(),
        Some(json) => match serde_json::from_str::<BTreeMap<String, String>>(json) {
            Ok(map) => map,
            Err(e) => {
                tracing::warn!("Invalid --engine-overrides JSON, ignoring");
                BTreeMap::new()
            }
        },
    }
}
```
**Finding:** If a user miss-types their JSON (e.g., `--engine-overrides "{asr: tencent}"` lacking quotes around keys), the CLI warns and *returns an empty map*. This causes the pipeline to fall back to default engines silently. 
**Actionable Recommendation:** `clap` parsing should fail hard on invalid JSON. A warning is insufficient for a parameter that drastically changes the runtime output. Return a `Result` and surface it up through `CliError`.

### 3.2 Control Plane & Storage (Rust)
Once `CommandOptions` is instantiated, it is dispatched to the daemon via `dispatch_single_server` or HTTP.
- It is saved into the SQLite database (`batchalign-app/src/db/schema.rs`) as serialized JSON.
- The daemon scheduler (`batchalign-app/src/runner/dispatch/plan.rs`) decodes `CommandOptions` into `DispatchParams`.
- It explicitly retrieves `.engine_overrides` from the `CommonOptions` block and hands it off to the worker pool logic.

### 3.3 Worker Pool Spawning
In `batchalign-app/src/worker/pool/lifecycle.rs`, the worker key is defined by the tuple `(target, lang, engine_overrides)`.
- If a user passes `--lang yue --engine-overrides '{"asr":"tencent"}'`, a dedicated Python worker pool is maintained specifically for that language + engine permutation.
- The Rust subprocess launcher (`batchalign-app/src/worker/handle.rs`) strings the overrides back into command-line arguments:
  ```rust
  if !config.engine_overrides.is_empty() {
      cmd.arg("--engine-overrides").arg(&config.engine_overrides);
  }
  ```
- **Nitpick:** We are serializing a map to JSON in the CLI, deserializing it in the `batchalign-cli`, re-serializing it into SQLite `CommandOptions`, pulling it back out, and passing it as a stringified JSON argument to the `python -m batchalign.worker` process. This is slightly circuitous but acceptable for IPC boundaries.

### 3.4 Python Worker Bootstrap
In `batchalign3/batchalign/worker/_main.py`:
- `argparse` consumes `--task`, `--lang`, `--engine-overrides`.
- These populate `WorkerBootstrapRuntime`.
- `batchalign3/batchalign/worker/_model_loading/` uses these bootstrap states to selectively initialize models (e.g., Stanza pipelines, specific Whisper versions, or FunASR).
- Python providers map inference responses to simple JSON arrays via `PipelineOperation` to communicate back to Rust.

---

## 4. Special Case: Non-English Plumbing (Cantonese)

The handling of Cantonese (`lang="yue"`) was a major architectural delta between BA2 and BA3.

### 4.1 Invocation
Users request Cantonese processing typically via `lang="yue"` combined with `engine_overrides` like `{"asr": "tencent", "fa": "cantonese_fa"}`.

### 4.2 Split-Brain Domain Logic
Interestingly, the models are executed in Python, but the **linguistic text normalization is strictly maintained in Rust**.
- **File:** `batchalign3/crates/batchalign-chat-ops/src/asr_postprocess/mod.rs`
  ```rust
  if lang == "yue" {
      words = normalize_cantonese_words(words);
  }
  ```
- **File:** `batchalign3/crates/batchalign-chat-ops/src/asr_postprocess/cantonese.rs`
  - Performs Simplified to HK Traditional conversion (`zhconv` equivalent).
  - Uses an Aho-Corasick automaton with a highly specific 31-entry domain replacement table (e.g., `"真系" -> "真係"`, `"松" -> "鬆"`).
  - This Rust module actually exposes `batchalign_core.normalize_cantonese()` via `PyO3` so that Python can utilize it without requiring an `OpenCC` pip dependency.

### 4.3 Nitpicks and Review
**The Good:**
1. Rust handling character normalization is blazing fast and drastically reduces the Python dependency tree (avoiding bulky text-norm libraries).
2. The logic correctly activates **only** if `lang == "yue"`, which prevents corrupting Mandarin (`zho`) transcripts.
3. The timestamps for character-level tokens (`cantonese_char_tokens`) are safely calculated at the Rust boundary, meaning dynamic programming aligners have accurate anchors.

**The Critique:**
While technically sound, the *plumbing* feels inverted. Python controls the ML layer but explicitly has to call back into a Rust C-Extension to do string formatting via `_common.py` (which delegates to Rust). 
Furthermore, the `lang="yue"` string is hardcoded directly inside `batchalign-chat-ops/src/asr_postprocess/mod.rs`. If other language extensions are added (e.g., specialized Arabic or Japanese normalizations), `asr_postprocess/mod.rs` will turn into a massive branching switch statement. 
**Actionable Recommendation:** The Rust architecture should introduce a formal `LanguageNormalizer` trait. `cantonese.rs` implements this trait, and a `NormalizerRegistry` resolves the normalizer by `LanguageCode`. This makes the pipeline Open/Closed to new languages.

---

## 5. Potential Dropped Flags & Type Deficiencies

During the trace, several inconsistencies in parameter typing were noted:

1. **Typing Misalignment in `CommandOptions` variants:**
   In `batchalign3/crates/batchalign-app/src/types/options.rs`, look at the variance in how engines are stored:
   ```rust
   pub struct AlignOptions {
       pub fa_engine: String,
       pub utr_engine: Option<AppUtrEngine>, // strongly typed enum
   }
   pub struct TranscribeOptions {
       pub asr_engine: String, // weakly typed string
   }
   ```
   **Issue:** `fa_engine` and `asr_engine` are `String` while `utr_engine` gets a nice strongly typed Enum (`AppUtrEngine::Whisper`, `AppUtrEngine::RevAi`, `AppUtrEngine::Custom(...)`). 
   Because `asr_engine` is a raw String, any misspelling from Python JSON or CLI aliases won't be caught by Rust's type system until the Python worker fails to instantiate the provider.
   **Actionable Recommendation:** Upgrade `fa_engine` and `asr_engine` inside `CommandOptions` to strongly-typed Enums (similar to `AppUtrEngine`) with a `Custom(String)` fallback.

2. **`--merge-abbrev` Boolean Pair Collision:**
   In `options.rs`, we see:
   ```rust
   fn resolve_merge_abbrev_policy(enabled: bool, disabled: bool) -> MergeAbbrevPolicy {
       resolve_flag_pair(enabled, disabled).into()
   }
   ```
   If a user inadvertently provides both `--merge-abbrev` and `--no-merge-abbrev` on the CLI, `resolve_flag_pair` handles it via `enabled && !disabled`. It drops the `enabled` flag silently. Given clap can define mutually exclusive argument groups via `conflicts_with`, it is safer to rely on clap to reject the command outright rather than quietly preferring one side.

3. **Audio Laziness (`--lazy-audio`):**
   `GlobalOpts.lazy_audio` defaults to `true`. This flag is plumbed into `CommonOptions`. `CommandOptions` passes this down, but it's important to ensure Python honors this. In the Jan 9 baseline, `batchalign2` often struggled with eager audio loading OOMing machines. Plumb verification confirms the Rust backend passes the flag to the `batchalign.models.audio` routines safely. No drop detected.

---

## 6. Final Conclusion

The Domain 1 audit confirms that the CLI command-line plumbing satisfies the requirements of the Jan 9 batchalign2-next baseline. The flow is:
**Invocation -> clap parsing -> Typed CommandOptions -> SQLite Job Storage -> Pool Extraction -> Worker Bootstrap -> Python Provider.**

### Summary of Actionable Recommendations:
1. **Remove Graceful Degradation in JSON parsing:** Modify `parse_engine_overrides` to return a `Result` and fail hard if the user supplies invalid JSON.
2. **Type the Engines:** Convert `fa_engine` and `asr_engine` in `CommandOptions` from `String` to typed Enums with `Custom(String)` variants.
3. **Clap Mutual Exclusion:** Apply `conflicts_with` in `clap` for boolean toggle pairs (e.g., `--diarize` and `--nodiarize`) instead of manual silent precedence logic.
4. **Abstract Language Post-Processing:** Replace the hardcoded `if lang == "yue"` in `asr_postprocess/mod.rs` with a `LanguageNormalizer` trait registry to prevent branching bloat as new languages are integrated. 

These adjustments will bring the command-line interface from simply "functioning on par with Jan 9" to being structurally rigorous and fail-safe.