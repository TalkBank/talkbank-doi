# Batchalign3 Audit: Domain 2 - Non-English NLP & Special Cases

**Status:** Current
**Last updated:** 2026-03-16

## 1. Executive Summary

This report documents the deep codebase investigation of `batchalign3` and `talkbank-tools` focusing on Domain 2 of the audit plan: the handling of non-English CHAT corpora, phonetic processing, and specific model branching. The investigation comprehensively evaluates the migration from the `batchalign2-next` Jan 9 baseline (commit `84ad500b`) to the current `batchalign3` architecture.

The core finding is a fundamental paradigm shift from Python-based string surgery and broad Dynamic Programming (DP) fallbacks to Rust-based typed Abstract Syntax Trees (ASTs) with deterministic identity/index/interval mapping. This provides higher correctness, especially in processing morphologically complex languages like Japanese and Cantonese.

## 2. Methodology & Baselines

In accordance with TalkBank workspace policies, all parity claims and regressions are evaluated against the dual-baseline anchors:
- **Core / Non-HK Claims:** Evaluated against `batchalign2-master` Jan 9 baseline (`84ad500b...`).
- **HK / Cantonese Claims:** Evaluated against the `BatchalignHK` Jan 9 baseline (`84ad500b...`) using the legacy `batchalignhk` command runner.
- **Secondary Reference:** The later released BA2 point (`e8f8bfad...`) on Feb 9 is referenced strictly for master-branch trailing behaviors.

## 2.1 Reconciliation Update (2026-03-16)

The findings below preserve the original audit narrative. This note records the
current repo truth after the final sweep.

- **Fixed in this sweep:**
  - the blunt `iso3[:2]` fallback described below was replaced with explicit
    mapping plus pass-through behavior, so unknown ISO-639-3 codes are no longer
    silently truncated
  - the thin Python inference surface, including HK adapters, now has focused
    full-coverage test passes; Cantonese / HK routing, dependency errors, and
    special handlers are exercised directly rather than only by happy-path smoke
    tests
- **Confirmed as already true:**
  - Cantonese normalization / tokenization ownership is already Rust-centered
    through `batchalign_core.normalize_cantonese()` and
    `batchalign_core.cantonese_char_tokens()`
  - Japanese override behavior is already materially safer than the Jan 9
    baseline because the rules live in typed Rust mapping code rather than
    fragile Python string surgery
- **Still deferred / follow-up territory:**
  - externalizing `lang_ja.rs` override rules into data files may still be worth
    doing, but it is a maintainability improvement rather than a release blocker
  - legacy `batchalignhk` user migration is operational / packaging follow-up,
    not a core correctness gap in the current repos

## 3. Language Detection and Stanza Loading

### 3.1 Fallback Mechanisms in Language Loading
Language detection and model routing now explicitly manage failures and fallback behavior. In `batchalign3/batchalign/worker/_stanza_loading.py`, Stanza pipeline requests that fail precise ISO3 matching implement a structured fallback strategy:

```python
# batchalign/worker/_stanza_loading.py ~L60
fallback = iso3[:2] if len(iso3) >= 2 else iso3
```

This ensures that missing dialects drop back to macro-language models when appropriate, preventing pipeline collapse while retaining trace visibility of the downgrade.

### 3.2 Leniency in CHAT Language Declarations
The `talkbank-tools` tree-sitter grammar (`src/grammar.json`) and Leniency Policy (`book/src/architecture/leniency-policy.md`) enforce explicit rules for language codes.
- **Undeclared Inline Codes:** Previous versions emitted `E254` for undeclared inline `@s:...` codes. This has been explicitly relaxed to support broader non-English inline code-switching without failing the validation gate.
- **Language Headers:** Grammar definitions explicitly support `multiple_langs` and `ambiguous_langs` tokens, structurally parsing mixed-language utterances for downstream processor routing instead of treating them as generic strings.

## 4. Model Selection and Branching

Model selection in `batchalign3` has evolved from local dynamic imports to explicit dependency injection architectures distributed across Python workers and Rust nodes.

### 4.1 ASR Engine Selection
The ASR selection matrix (`batchalign/worker/_handlers.py`) dynamically maps engines based on available extras and credentials:
- **Whisper (Local & Cloud):** Distinguishes between `local_whisper` and `whisper` backend hosts. The runtime checks for local importability (`_module_importable("whisper")`) and gracefully branches to RevAI or API fallbacks if hardware acceleration constraints demand it.
- **Chinese / HK Overrides:** Explicit branching exists for Chinese dialects. In `batchalign/inference/asr.py`, `{"yue": "Cantonese", "cmn": "chinese"}` maps iso codes to specific handler logic, overriding default Whisper pipelines.

### 4.2 Forced Alignment (FA) Branching
Forced Alignment branches strictly based on the language profile:
- **Default FA (Whisper):** Uses `infer_whisper_fa` for general cases.
- **Cantonese FA (Wav2Vec2):** The system heavily branches for Cantonese (`yue`), forcing a shift from Whisper to a specialized MMS Wav2Vec2 model.

## 5. Cantonese and zh-HK Specialized Pipelines

The transition from the `BatchalignHK` baseline to `batchalign3` introduces major architectural improvements for Cantonese processing, moving from heavy Python-script processing to a hybrid Rust/Python approach.

### 5.1 The `load_cantonese_fa` Architecture
`batchalign/inference/hk/_cantonese_fa.py` encapsulates the Cantonese FA inference provider. It wraps the standard Wave2Vec MMS forced alignment model with a custom Cantonese adapter:
- It uses `_PyCantonese` (via `pycantonese`) to generate jyutping forms.
- Missing dependencies explicitly throw actionable errors: `"Cantonese FA requires batchalign3 extras: pip install 'batchalign3[hk-cantonese-fa]'"` instead of failing silently.
- Unknown characters bypass the romanizer and pass through, preserving data integrity rather than dropping unsupported Hanzi.

### 5.2 Rust-Delegated Normalization and Tokenization
A major regression in BA2 for `yue` was poor Word Error Rate (WER) scoring due to Han-script chunks being processed as giant single tokens.
In `batchalign3`, this is resolved by delegating heavy text manipulation to Rust:
```python
# batchalign/inference/hk/_common.py
def normalize_cantonese_char_tokens(text: str) -> list[str]:
    return batchalign_core.cantonese_char_tokens(text)
```
- **Han-script Splitting:** Rust's `cantonese_char_tokens()` splits Han characters for granular retokenization and scoring. ASCII/code-switched tokens remain intact.
- **Normalization:** `batchalign_core.normalize_cantonese()` applies pure Rust, `zhconv`-based traditional HK normalization.

### 5.3 Aliyun and Tencent API Handlers
- **Aliyun ASR:** `batchalign/inference/hk/_aliyun_asr.py` enforces a strict Cantonese-only policy (`lang="yue"`), throwing `ValueError` if misrouted.
- **Tencent API:** `batchalign/inference/hk/_tencent_api.py` maintains an explicit `_CHINESE_CODES = {"zho", "yue", "wuu", "nan", "hak"}` routing set.

## 6. Japanese Morphosyntax and Tokenization

Japanese CHAT processing received one of the most significant upgrades in the `batchalign3` refactor, moving fragile Python logic into robust Rust pipelines.

### 6.1 Rust-ported Override Rules
In BA2, colloquial Japanese forms frequently broke Stanza POS mapping (e.g., misclassifying conjugations). `batchalign3` ports the `ja/verbforms.py` logic into `nlp/lang_ja.rs` (460+ lines), applying over 50 ordered override rules *before* UD→CHAT POS mapping:
- **Subordinating Conjunctions:** Contracted conditionals (e.g., ちゃ→ば, じゃ→ちゃ) are force-reclassified from VERB/AUX to SCONJ.
- **Auxiliary Verbs:** Colloquial endings (れる→られる, 無い→ない) receive corrected lemmas.
- **Interjections & Backchannels:** Items like はい, うん, おっ are fixed to INTJ.
- **Kanji Lemma Corrections:** Verbs like 撮る, 貼る, 帰る are explicitly patched where Stanza fails.

### 6.2 Stanza Processor Configuration
`test_stanza_config_parity.py` explicitly enforces that Japanese uses the `combined` Stanza processor package (tokenize+pos+lemma+depparse in one model), explicitly prohibiting split processors that cause silent accuracy degradation in agglutinative languages.

## 7. Tokenization Fallbacks and DP Remap Reductions

The single most consequential algorithmic change from the Jan 9 baseline is the elimination of broad string-level Dynamic Programming (DP) for retokenization and morphosyntactic mapping.

### 7.1 Elimination of the Char-Level DP Fallback
In BA2, if the tokenizer's output array drifted from the original string, the system ran a broad DP over the flattened text, often guessing alignments.
In `batchalign3`, the char-level DP fallback mapping path was completely removed. It is replaced by:
- Deterministic interval/index mapping.
- Length-aware monotonic fallback.
- Explicit AST rebuilds (`retokenize/mapping.rs`).

### 7.2 UTR and Global DP Boundaries
While runtime remaps removed DP, the Utterance Timing Recovery (UTR - `fa/utr.rs`) retains a single global Hirschberg DP alignment (ported to Rust, O(mn) time, 10-50x faster).
This is specifically retained for the 407-style hand-edited transcript case, where transcript words and ASR tokens are genuinely independent global sequences, solving the token-starvation failures of BA2.

### 7.3 Multi-Word Token (MWT) Expansion
Stanza's MWT expansions in non-English languages are now mapped via deterministic policy rather than global DP reconciliation. One source token yielding multiple UD tokens generates per-component `%gra` relations structurally, preserving exact provenance.

## 8. Actionable Recommendations

1. **Monitor Rust-Delegated Cantonese Splitting:** The transition of `cantonese_char_tokens` to Rust resolves WER inflation but requires continuous regression testing against mixed ASCII/Hanzi utterances to ensure code-switching logic doesn't over-split English loanwords.
2. **Expand Japanese Override Matrix:** The 50+ override rules in `nlp/lang_ja.rs` should be exposed as an external configuration file or dictionary rather than hardcoded Rust to allow corpus maintainers to add colloquialisms without requiring a full compiler cycle.
3. **Deprecate Legacy HK Runner:** Migrate users entirely off the `batchalignhk` legacy entry point, ensuring the `{"yue": "Cantonese"}` routing completely absorbs the specialized logic.
4. **Audit Stanza Fallback:** The `iso3[:2]` Stanza fallback is a blunt instrument. Implement a structured mapping dictionary for macro-languages to avoid routing specific non-supported dialects to incompatible base models.

## 9. Conclusion
The `batchalign3` pipeline successfully addresses the morphological and phonetic complexities of Domain 2 non-English processing. By transitioning away from ad-hoc Python string-surgery to typed Rust AST operations, the system enforces a strict, deterministic alignment policy. The handling of Cantonese Wav2Vec MMS integration and the explicit Rust-ported Japanese POS override rules mark a substantial improvement over the Jan 9 `batchalign2-next` baseline.
