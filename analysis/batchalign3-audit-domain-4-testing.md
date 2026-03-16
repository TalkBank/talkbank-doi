# Domain 4 Audit: Testing & Coverage Deficiencies

**Status:** Current
**Last updated:** 2026-03-16
**Target:** `batchalign3`, `talkbank-tools`
**Audit Domain:** 4 (Testing & Coverage Deficiencies)

---

## Executive Summary

This report documents a deep, highly-nitpicking codebase investigation into the testing methodologies, coverage metrics, and integration contract matrices of the `batchalign3` and `talkbank-tools` repositories. The investigation focuses on Domain 4 of the project audit plan: identifying gaps in edge cases, exposing brittle test patterns, and highlighting where high line-coverage metrics systematically mask shallow, non-verifying assertions.

While the structural migration from the Jan 9 `batchalign2-next` baseline to the current `batchalign3` Rust-backed architecture (which transitioned from ad-hoc Python string-surgery to typed Rust Abstract Syntax Trees) has brought immense safety benefits, **the testing suite has not conceptually evolved to match the rigor of the new data model.** 

We identified three critical anti-patterns:
1. **The `is_ok()` Fallacy in `talkbank-tools`:** A systemic reliance on structural parsers succeeding without verifying the correctness, completeness, or semantic integrity of the produced AST.
2. **Trivial Mocks in `batchalign3` Pytest Suites:** Python worker and execution tests achieve high coverage by mocking complex AI runners with trivial `["hello", "world"]` return payloads, entirely bypassing the rigorous validation of alignment, phonetic, and morphosyntactic inference.
3. **Superficial Contract Matrices:** The integration tests comparing the current pipeline to the Jan 9 baseline confirm that legacy CLI flags parse correctly, but they explicitly deprioritize the actual deep equivalence tests needed to guarantee behavioral parity.

This report spans an exhaustive analysis of specific test files, pointing to exact lines of code where testing debt has accumulated, and concludes with a prioritized, actionable roadmap to fortify the test suites.

---

## Reconciliation Update (2026-03-16)

The findings below preserve the original audit narrative. This section is the
current disposition after the release-hardening sweep.

- **Fixed or materially improved in this sweep:**
  - `batchalign3` now has focused full-coverage test passes for the thin Python
    inference layer, including HK adapters, plus richer worker V2 tests for
    malformed host output, non-finite metrics, reversed timing ranges, Unicode
    `special_forms`, and runtime-failure classification
  - Jan 9 `transcribe_s` semantics were explicitly audited against the baseline
    commit and then locked down with CLI, dispatch, runtime, and e2e coverage
  - `talkbank-tools` gained stronger CLI integration assertions and invalid-file
    stdout/stderr contract tests instead of relying only on success/failure
    shape
  - PyO3 `ParsedChat` rollback behavior is now covered directly: callback,
    progress-hook, and cache-injection failures are tested to ensure they do not
    partially mutate the AST
- **Stale / incorrect claims:**
  - the blanket statement that `talkbank-tools` lacks fuzz/property testing is
    false; the repo already has both `cargo-fuzz` targets and live `proptest`
    usage
- **Still open / intentionally deferred:**
  - a full byte-for-byte Jan 9 round-trip equivalence suite remains follow-up
    work; the current sweep focused on the highest-risk semantic and boundary
    behaviors first
  - not every shallow `is_ok()` assertion in `talkbank-tools` was eliminated in
    one pass, though the highest-value parser / CLI seams were upgraded

---

## 1. `talkbank-tools/crates`: High Coverage, Shallow Assertions

The `talkbank-tools` repository is responsible for the foundational parsing and generation of CHAT formats. A codebase of this criticality requires exhaustive structural validation. Instead, the test suite is characterized by high coverage numbers achieved through the absolute shallowest possible assertions.

### 1.1 The Epidemic of `is_ok()` and `is_err()`

Throughout the core parsing tests, the primary method of validation is simply checking if the parser returned a `Result::Ok` or `Result::Err`. This verifies that the code didn't panic and that the highest-level state machine reached a terminal state, but it **completely ignores the complex AST transformations** happening beneath the surface.

**Citation:** `talkbank-tools/tests/parse_chat_file_tests/basic.rs`
```rust
// Lines 136:
assert!(result.is_ok(), "Should parse minimal file");
```

**Citation:** `talkbank-tools/tests/parse_chat_file_tests/realistic.rs`
```rust
// Lines 62-77:
assert!(parser.next().is_some());
assert!(parser.next().is_some());
// ...
```

**Critique:** 
Asserting that a realistic file produces *some* AST nodes does not verify that the AST nodes are correct. CHAT files have highly complex dependent tiers (`%mor`, `%gra`, `%pho`). If the parser silently drops a dependent tier or misaligns a word boundary by one character, `result.is_ok()` will still pass. Coverage tools will mark the parsing branches as "covered" because the code executed, but the *logic* is untested.

### 1.2 Brittle Error Context Assertions

When errors are tested, they are often tested by blindly asserting against raw line numbers or literal text indices, which makes the tests incredibly brittle to any upstream changes in the mock data or error formatting.

**Citation:** `talkbank-tools/src/commands/check/mod.rs`
```rust
// Lines 332-335:
assert!(listing.contains("  6:"));
assert!(listing.contains("  7:"));
assert!(listing.contains("161:"));
assert_eq!(listing.lines().count(), 161);
```

**Critique:**
This is an anti-pattern. If the error formatting changes (e.g., adding a prefix, padding, or changing the context window size), these tests will break without any actual logic failing. More importantly, this does not test the *semantic cause* of the error (e.g., `ErrorCode::MissingEndHeader`). It merely tests that the stringifier output a specific line number.

### 1.3 Missing Edge Case Validations in Analyzers

In the CLAN command re-implementations (e.g., `vocd`, `sugar`, `flucalc`), the tests assert final computed floats but fail to test the edge cases of AST traversal.

**Citation:** `talkbank-tools/src/commands/flucalc.rs`
```rust
// Lines 486-487:
assert_eq!(fluency.filled_pauses, 2);
assert_eq!(fluency.total_words, 3);
```

**Critique:**
These tests pass nicely for sunny-day scenarios. However, there are massive gaps in edge cases:
- What happens if a filled pause is attached to a complex morphological compound (`&-um#noun`)?
- What happens if the AST contains a heavily nested sub-utterance group `[<]` overlapping with a disfluency? 
Because the tests only use simple linear token streams, the complex AST tree-walking logic is covered but completely unverified for structural edge cases.

---

## 2. `batchalign3` Pytest Suites: Trivializing the AI Inference

The `batchalign3` pipeline orchestrates highly complex NLP and acoustic models (Whisper, Stanza, openSMILE, Pyannote). The shift from Jan 9's dynamic programming (DP) algorithms to typed Rust payloads was a massive undertaking. However, the Python `pytest` suites mock away all the complexity, creating a false sense of security.

### 2.1 The "Hello World" Mock Payload Illusion

In the worker execution test suite, the payloads used to verify routing and execution logic are trivially simple, bypassing the structural edge cases that actually cause pipeline failures in production.

**Citation:** `batchalign3/batchalign/tests/test_worker_execute_v2.py`
```python
def _write_fa_payload(path: Path) -> None:
    path.write_text(
        json.dumps(
            {
                "words": ["hello", "world"],
                "word_ids": ["u0:w0", "u0:w1"],
                "word_utterance_indices": [0, 0],
                "word_utterance_word_indices": [0, 1],
            }
        ),
        encoding="utf-8",
    )
```

**Critique:**
Testing morphosyntax, forced alignment (FA), and utterance segmentation with `["hello", "world"]` completely ignores the reality of conversational speech transcripts. 
- There are no special CHAT forms (e.g., `hello@s:eng`).
- There are no overlapping words (`[<]`).
- There is no non-ASCII data to test utf-8 byte-offset misalignments (a critical issue in previous BA2 iterations).

### 2.2 Deep Logic Bypassed by Lambdas

To avoid loading heavy ML models during CI, the tests use lambda mocks. However, these mocks are so shallow that they fail to enforce the strict typed contracts expected by the Rust backend.

**Citation:** `batchalign3/batchalign/tests/test_worker_execute_v2.py`
```python
# Lines 142-146:
forced_alignment=ForcedAlignmentExecutionHostV2(
    whisper_runner=lambda audio, text, pauses: [
        ("hello", 0.1 if audio.shape == (4,) else 0.0),
        ("world", 0.3 if text == "hello world" and pauses else 0.0),
    ]
)
```

**Critique:**
This tests the *Python Router*, but it masks the fact that the underlying AST transformation (from Whisper outputs to the `WhisperTokenTimingResultV2` contract) is untested on real, messy outputs. If Whisper generates an empty token, a hallucinated timestamp, or a nan-float, does the AST transformer safely map it to `None` or panic? This test provides 100% coverage of the routing branch while offering 0% confidence in data integrity.

### 2.3 IPC and Protocol Tests Check the Wire, Not the Data

In `test_worker_ipc.py`, the assertions focus purely on the existence of keys rather than the validity of the data structures.

**Citation:** `batchalign3/batchalign/tests/test_worker_ipc.py`
```python
# Lines 91-93
assert data["result"]["mor"].startswith("det|the")
assert data["error"] is None
assert data["elapsed_s"] == 0.5
```

**Critique:**
`.startswith("det|the")` is a lazy assertion. It does not verify the structural mapping of the morphosyntactic tree, the dependency graph (`%gra`), or the alignment of the morphological tags to the original utterance indices. This is exactly the kind of test that allows subtle regressions to slip through when the Stanza models are updated.

---

## 3. Contract Matrices vs. The Jan 9 Baseline

A major goal of the `batchalign3` audit is ensuring parity with the `batchalign2-next` Jan 9 baseline (commit `84ad500b...`). The documentation correctly identifies that `batchalign3` must support 37 hidden legacy flags (e.g., `--whisper`, `--rev`) without breaking scripts.

### 3.1 Over-indexing on the Surface, Under-indexing on Semantics

The testing for the Jan 9 compatibility floor heavily indexes on the CLI parsing matrix rather than the output execution matrix.

**Citation:** `batchalign3/crates/batchalign-cli/tests/command_surface_manifest.rs`
```rust
// Paraphrased logic:
cmd().args(&["align", "--help"]).assert().success()
    .stdout(!contains("--whisper"));
```

**Critique:**
The CLI tests comprehensively verify that `clap` parses the aliases into the correct `CommandOptions` enum (e.g., mapping `--whisper` to `AppUtrEngine::Whisper`). **However, the actual semantic equivalence is untested.** 

In the `batchalign3_COMPAT_QUICK_REF.md`, the section on "Lower priority (integration with server)" admits:
> `[ ] Round-trip equivalence tests (compat alias produces identical results to canonical)`

This is a critical gap. The Jan 9 baseline performed retokenization and morphosyntactic mapping via highly complex string-level Dynamic Programming (DP). The current architecture performs this via Rust AST operations. Parsing the flag correctly does not prove that the AST transformer produces the identical `%mor` tier formatting that the Jan 9 DP string-surgery did. 

### 3.2 Boolean Gate Logic Risks

The compatibility layer employs specific boolean gating logic that is counter-intuitive:
```rust
final_value = positive_flag && !negative_flag
```
*(e.g., passing both `--wor` and `--nowor` evaluates to `false`).*

While this behavior is documented, the unit tests do not rigorously test the interaction of these flags with the deeply nested inference pipeline. If `--override-cache` and `--use-cache` evaluate to false, does the cache infrastructure actually gracefully fall back, or does the lack of explicit intent cause cache-miss thrashing? The current tests stop at the parameter boundary.

---

## 4. Deep Gaps in Edge Cases

Based on the audit of the Rust and Python test directories, the following edge cases represent significant, untested liabilities that could lead to panics or silent data corruption in the AST:

1. **Non-Monotonic Timestamps:** In legacy BA2 (Jan 9), non-monotonic timestamps from Whisper often crashed the DP alignment. BA3's Rust backend is supposed to enforce monotonicity. However, there are no generative tests or explicit fixtures feeding non-monotonic overlapping timestamps into the `ForcedAlignmentExecutionHostV2` to ensure the Rust layer corrects them.
2. **Pathological CHAT Utterances:** The Rust `talkbank-parser-tests` lacks property-based testing (e.g., using `proptest`) for structurally valid but pathologically nested CHAT tags (e.g., `[<1]`, `[>2]`, `[//]`, `[/]`).
3. **Empty Payloads and Zero-Length Audio:** Tests verify that missing attachments return typed errors (`ProtocolErrorCodeV2.MISSING_ATTACHMENT`), but there are no tests verifying the behavior of the NLP pipelines when provided with structurally valid but semantically empty 0-byte audio arrays or empty `items: []` arrays in batch payloads.

---

## 5. Actionable Recommendations & Remediation Plan

To close the testing and coverage deficiencies, the following steps must be immediately implemented.

### Phase 1: Eradicate Shallow `is_ok()` Assertions in Rust (High Priority)
1. **Implement Snapshot Testing:** In `talkbank-tools`, replace `assert!(result.is_ok())` with `insta::assert_debug_snapshot!(result.unwrap())`. This will lock in the exact structural AST output. Any change to a child node will intentionally break the snapshot, preventing silent regressions.
2. **Deep Semantic Asserts:** For CLAN command tests (like `vocd` and `flucalc`), tests must assert the exact AST token boundaries they matched against, not just the aggregate float totals.

### Phase 2: Fortify Python Worker Tests with Real Data (High Priority)
1. **Inject Real CHAT Fixtures:** Replace the trivial `["hello", "world"]` payloads in `test_worker_execute_v2.py` with sanitized, real-world CHAT snippets that include disfluencies, specialized word markers (`@s`), and overlap markers.
2. **Chaos-Monkey the Mocks:** Update the lambda mocks to occasionally return `None`, `NaN`, or slightly out-of-bounds timestamps to verify that the validation layers correctly cast these into `ProtocolErrorCodeV2.RUNTIME_FAILURE` rather than allowing the worker to crash.

### Phase 3: Elevate Jan 9 Baseline Equivalence to Tier 1 (Medium Priority)
1. **Golden Equivalence Suite:** The "Lower Priority" round-trip equivalence tests mentioned in `batchalign3_COMPAT_QUICK_REF.md` must be promoted. Create a golden test suite that runs a complex CHAT file through the Jan 9 Docker container, captures the output, and asserts byte-for-byte equivalence (or explicitly defined diff equivalence) against the current BA3 AST pipeline running with compat flags.
2. **Explicit Boolean Gate Tests:** Add the missing parameter matrix tests (e.g., `test_align_custom_engine_overrides_compat_flag()`, `test_global_lazy_audio_and_gate()`) directly into `src/args/tests.rs` to guarantee the flag resolution invariant holds.

### Phase 4: Property-Based Fuzzing (Long Term)
1. **Fuzz the AST:** Integrate `cargo fuzz` or `proptest` into `talkbank-tools` to generate heavily mutated, semi-valid CHAT strings. The single assertion for these tests should be that the parser never panics and always gracefully returns an AST or an `Err(ErrorCode)`.

---
*End of Domain 4 Audit Report.*
