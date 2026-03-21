# Language Code Type System: Research Report & Redesign Recommendations

**Status:** Reference
**Last updated:** 2026-03-20

## Context

Language codes are threaded through every layer of TalkBank: CHAT file headers (`@Languages`, `@ID`, `[- lang]`), ASR engine dispatch, NLP model selection, worker IPC, cache keys, and validation rules. Today we have **5+ different representations** of "language" across the Rust and Python codebases, with ad-hoc string conversions at every boundary. The sentinel `"auto"` has leaked through type boundaries multiple times causing bugs (the `--lang auto` bug being the latest). Each ML engine expects its own language format. It's time to unify.

**Goal of this document:** Inventory all language representations, identify every conversion boundary, catalog all conditional logic that branches on language, and recommend a type-safe redesign.

---

## Part 1: Current Type Inventory

### Rust — talkbank-tools

| Type | Location | Backing | Validated? | "auto" safe? |
|------|----------|---------|------------|--------------|
| `LanguageCode` | `talkbank-model/.../language.rs:83` | `Arc<str>` (interned) | 3 lowercase ASCII, rejects placeholders (xyz/xxx/yyy/zzz). Does NOT check ISO registry. | Yes — rejects "auto" (not 3 chars) |
| `LanguageCodes` | `talkbank-model/.../header.rs` | `Vec<LanguageCode>` | Via LanguageCode | Yes |
| `UtteranceLanguage` | `talkbank-model/.../utterance_language.rs:40` | Enum: Uncomputed/Unresolved/ResolvedDefault/ResolvedTierScoped | Tagged validation | Yes |
| `LanguageSource` | `talkbank-model/.../source.rs:32` | Enum: Default/TierScoped/WordExplicit/WordShortcut/Unresolved | — | Yes |
| `WordLanguageMarker` | `talkbank-model/.../word/language.rs` | Enum for `@s`, `@s:code`, `@s:code1+code2` | — | Yes |

### Rust — batchalign3

| Type | Location | Backing | Validated? | "auto" safe? |
|------|----------|---------|------------|--------------|
| `LanguageCode3` | `batchalign-app/src/types/domain.rs:38` | `String` | 3 ASCII alpha, lowercased. Rejects "auto". | Yes — panics on "auto" via `From`, error via `try_new` |
| `LanguageSpec` | `batchalign-app/src/types/domain.rs:151` | Enum: `Auto` / `Resolved(LanguageCode3)` | Boundary type. Custom serde. | By design — separates "auto" from codes |
| `InvalidLanguageCode` | `batchalign-app/src/types/domain.rs:41` | Error struct | — | — |
| `RevAiLanguageHint` | `batchalign-app/src/revai/preflight.rs:36` | `String` (ISO 639-1 or "auto") | Exhaustive 85+ entry mapping table | Contains "auto" as fallback for unknown langs |
| `LanguageCode3::from_worker_lang()` | `domain.rs:54` | Bypasses validation | **NO** — accepts "auto" for pool keys | **UNSAFE** — only call site must be audited |

### Python — batchalign3

| Type | Location | Backing | Validated? | "auto" safe? |
|------|----------|---------|------------|--------------|
| `LanguageCode` | `_domain_types.py:24` | `TypeAlias = str` | **None** — raw string | **No** — "auto" flows freely |
| `LanguageCode2` | `_domain_types.py:26` | `TypeAlias = str` | **None** — raw string | N/A |
| All Pydantic `lang` fields | Various `_types.py`, `_types_v2.py` | `str` | Pydantic default validation only | **No** — accepts any string |

### Summary: 3 incompatible "language code" types exist

1. **`LanguageCode`** (talkbank-model) — `Arc<str>`, interned, 3 lowercase alpha, no ISO validation
2. **`LanguageCode3`** (batchalign-app) — `String`, 3 lowercase alpha, no ISO validation, has `from_worker_lang()` escape hatch
3. **`LanguageCode`** (Python) — bare `str`, zero validation

There are **zero `From`/`Into` conversions** between talkbank-model's `LanguageCode` and batchalign's `LanguageCode3`.

---

## Part 2: Engine-Specific Language Formats

Each ML engine expects a different language representation:

| Engine | Expected Format | Example | Conversion Function | Location |
|--------|----------------|---------|---------------------|----------|
| **Whisper ASR** | Lowercase English name | `"english"`, `"spanish"` | `iso3_to_language_name()` | `asr.py:92` |
| **Whisper ASR** (Cantonese) | PascalCase special | `"Cantonese"` | hardcoded special case | `asr.py:101` |
| **Whisper ASR** (auto) | Sentinel string | `"auto"` | hardcoded special case | `asr.py:101` |
| **Rev.AI** | ISO 639-1 (mostly) | `"en"`, `"es"`, `"cmn"` | `try_revai_language_hint()` | `preflight.rs:49` |
| **Rev.AI** (auto) | Sentinel string | `"auto"` | passed directly | `asr.rs:96` |
| **Stanza** (morphosyntax) | ISO 639-1 | `"en"`, `"fr"`, `"zh"` | `iso3_to_alpha2()` | `_stanza_loading.py:33` |
| **Stanza** (coref) | ISO 639-1 | `"en"` only | **hardcoded** | `coref.py:91` |
| **Google Translate** | ISO 639-1 | `"en"`, `"es"` | `provider_lang_code()` | `_common.py:63` |
| **HK/Tencent** | ISO 639-1 or "yue" | `"en"`, `"yue"` | `provider_lang_code()` | `_common.py:63` |
| **HK/Aliyun** | "yue" only | `"yue"` | hardcoded check | `_aliyun_asr.py` |
| **HK/FunASR** | ISO 639-3 | `"yue"` | passed as-is | `_funaudio_asr.py:55` |
| **whatlang** (detection) | `whatlang::Lang` enum | `Lang::Spa` | `whatlang_to_iso639_3()` | `lang_detect.rs` |

### Conversion Functions (all ad-hoc, no shared type)

| Function | Input | Output | Location | Mechanism |
|----------|-------|--------|----------|-----------|
| `iso3_to_language_name()` | ISO 639-3 | Whisper name | `asr.py:92` | 3-entry special dict + pycountry |
| `iso3_to_alpha2()` | ISO 639-3 | ISO 639-1 | `_stanza_loading.py:33` | 57-entry hardcoded dict |
| `provider_lang_code()` | ISO 639-3 | ISO 639-1 (varies) | `_common.py:63` | pycountry + exception swallow |
| `try_revai_language_hint()` | `LanguageCode3` | `RevAiLanguageHint` | `preflight.rs:49` | 85-entry match |
| `revai_code_to_iso639_3()` | ISO 639-1 | `LanguageCode3` | `asr.rs:130` | 75-entry match (reverse) |
| `whatlang_to_iso639_3()` | `whatlang::Lang` | String (ISO 639-3) | `lang_detect.rs` | 60-entry match |
| `From<&LanguageCode3> for RevAiLanguageHint` | `LanguageCode3` | `RevAiLanguageHint` | `preflight.rs:147` | Delegates to `try_revai_language_hint` + "auto" fallback |

**Problem:** 6 independent mapping tables, none shared, partially overlapping, no single source of truth.

---

## Part 3: Language-Conditional Logic (All Branches)

### Rev.AI dispatch (Rust)
- `speakers_count`: sent only for `"en"` | `"es"` (`preflight.rs:294`, `asr.rs:108`)
- `skip_postprocessing`: `true` only for `"en"` (`asr.rs:117`)
- Auto mode: neither field sent (`asr.rs:105–117`)

### CHAT validation (talkbank-model)
- **Digits in words** (E220): allowed only for 8 languages: `zho`, `cym`, `vie`, `tha`, `nan`, `yue`, `min`, `hak` (`validation/context.rs:15–28`)
- Composite language codes (e.g., `"eng+yue"`) — permissive: if ANY allows digits, accepted

### Stanza MWT (multi-word tokens)
- 30 languages in `MWT_LANGS` set (`_stanza_loading.py:25–30`) — controls whether MWT processor is included in Stanza pipeline
- All ISO 639-1 codes

### Cantonese normalization
- `lang == "yue"` gates simplified→HK traditional character conversion (`_common.py:49`, `asr_postprocess/cantonese.rs`)
- Applied as pipeline stage 4b in ASR post-processing (`asr_postprocess/mod.rs`)

### Number expansion
- 12 language-specific digit→word tables in `num2lang.json`
- Cantonese/Chinese numbers via `num2chinese.rs` (up to 10^48)

### Disfluency/retrace detection
- Per-language wordlists for filled pauses (`asr_postprocess/cleanup.rs`)
- `"'cause" → "(be)cause"` style replacements are English-specific

### Coref
- **English only** — hardcoded `lang="en"` (`coref.py:91`)

### Aliyun ASR
- **Cantonese only** — `ValueError` if `lang != "yue"` (`_aliyun_asr.py`)

### Cache keys
- Language is part of cache key hash (`morphosyntax.rs:274`, `utseg.rs:161`)
- Always resolved before caching (safe from "auto" leaking)

### Worker pool dispatch
- Pool key = `(command, lang_code)` — "auto" used as pool key via `from_worker_lang()` for pre-scale (`runner/mod.rs:297`)

---

## Part 4: "auto" Sentinel Lifecycle & Leak Points

```
CLI "--lang auto"  (raw String)
    ↓
LanguageSpec::Auto  (type-safe enum, boundary)
    ↓
┌─────────────────────────────────────────────┐
│  Rev.AI path: "auto" → API → detected "es"  │
│  Whisper path: "auto" → gen_kwargs omits     │
│    language key → Whisper auto-detects →      │
│    BUT echoes "auto" back in response.lang   │
│  whatlang fallback: trigram detection         │
└─────────────────────────────────────────────┘
    ↓
LanguageCode3 (resolved, e.g. "spa")
    ↓
NLP stages (utseg, morphotag, translate, coref)
```

**Known leak points (now fixed or guarded):**
1. `pool_lang` defaulted to `"eng"` when Auto → Rev.AI never saw "auto" (fixed)
2. Rev.AI path echoed input lang, ignoring detection (fixed)
3. Whisper path echoes "auto" back → whatlang fallback now catches this
4. `from_worker_lang("auto")` bypasses validation → used only for pool keys (audited)

---

## Part 5: Recommendations

### R1: Shared `LanguageCode` crate

Create a small shared crate (e.g., `talkbank-lang`) that both talkbank-tools and batchalign3 depend on:

```rust
/// Validated ISO 639-3 language code.
pub struct Iso639_3(Arc<str>);  // 3 lowercase alpha, optionally validated against registry

/// Validated ISO 639-1 language code.
pub struct Iso639_1(Arc<str>);  // 2 lowercase alpha

/// Bidirectional mapping between ISO 639-3 and ISO 639-1.
pub fn iso3_to_iso1(code: &Iso639_3) -> Option<Iso639_1>;
pub fn iso1_to_iso3(code: &Iso639_1) -> Option<Iso639_3>;
```

- Replace talkbank-model's `LanguageCode` and batchalign's `LanguageCode3` with `Iso639_3`
- Single mapping table (source of truth) replaces the 6 independent tables
- Optional: embed the ISO 639-3 registry (7,000+ codes) for strict validation, with a fallback `Iso639_3::unchecked()` for codes not in the registry

### R2: Engine-specific language newtypes

Each engine that expects a non-ISO format should have a typed wrapper:

```rust
/// Whisper language name (e.g. "english", "Cantonese", "auto").
pub struct WhisperLang(String);

/// Rev.AI language hint (ISO 639-1 or "auto" or "cmn").
pub struct RevAiLang(String);

/// Stanza pipeline language (ISO 639-1).
pub struct StanzaLang(Iso639_1);
```

With typed conversion functions:
```rust
impl WhisperLang {
    pub fn from_iso3(code: &Iso639_3) -> Self { ... }
    pub fn auto() -> Self { Self("auto".into()) }
}
```

This makes it impossible to accidentally pass a Whisper name to Rev.AI or vice versa.

### R3: Separate `Auto` from language codes everywhere

The `LanguageSpec` pattern (enum with `Auto` variant) is correct at the CLI/API boundary. But downstream, `Auto` should be **resolved before dispatch** — never sent as a string through IPC.

- **Rust:** Already mostly correct. Enforce with `resolved_lang: LanguageCode3` (not `LanguageSpec`) on all post-ASR types.
- **Python:** Replace `LanguageCode = str` with a proper `NewType` or Pydantic constrained type that rejects "auto". Create a separate `LanguageOrAuto` type for the one place it's needed (ASR worker request).

### R4: Single mapping table with codegen

Replace all 6 hardcoded mapping tables with a single JSON/TOML data file:

```json
{
  "eng": { "iso1": "en", "whisper": "english", "revai": "en", "stanza": "en" },
  "spa": { "iso1": "es", "whisper": "spanish", "revai": "es", "stanza": "es" },
  "yue": { "iso1": null, "whisper": "Cantonese", "revai": null, "stanza": "zh" },
  "cmn": { "iso1": "zh", "whisper": "chinese", "revai": "cmn", "stanza": "zh" }
}
```

Generate Rust match tables and Python dicts from this file at build time. One place to update when adding a language or engine.

### R5: Python type safety

- Replace `LanguageCode: TypeAlias = str` with `LanguageCode = NewType("LanguageCode", str)` + runtime validation in Pydantic models
- Add a `validate_iso639_3()` validator to all Pydantic `lang` fields
- Remove empty-string defaults (`""`) from `MorphosyntaxBatchItem.lang` and `UtsegBatchItem.lang` — use explicit `Optional[LanguageCode]` instead

### R6: Validate against ISO 639-3 registry

Currently neither `LanguageCode` nor `LanguageCode3` validates against the actual ISO 639-3 code list. Options:
- **Compile-time:** Embed the ~7,000 code list as a `phf` hash set. Accept any 3-letter code but warn on unrecognized.
- **Runtime:** Load from a data file. Same behavior.
- **Permissive mode:** For CHAT files that use non-standard codes (common in legacy data), accept with a warning rather than error.

---

## Part 6: Implementation Phases

### Phase A: Research doc (this document) — **DONE**

### Phase B: Create `talkbank-lang` crate with shared types
- `Iso639_3`, `Iso639_1`, bidirectional mapping
- Single JSON mapping table for all engines
- Build-time codegen for Rust match tables

### Phase C: Migrate talkbank-tools
- Replace `LanguageCode` internals with `Iso639_3` (or re-export)
- Preserve public API compatibility

### Phase D: Migrate batchalign3 Rust
- Replace `LanguageCode3` with `Iso639_3`
- Add engine-specific newtypes (`WhisperLang`, `RevAiLang`, `StanzaLang`)
- Remove `from_worker_lang()` escape hatch

### Phase E: Migrate batchalign3 Python
- Replace `LanguageCode = str` with validated newtype
- Add Pydantic validators
- Remove empty-string defaults

### Phase F: Add ISO 639-3 registry validation
- Embed code list
- Warn on unrecognized codes

---

## Files Referenced

### talkbank-tools
- `crates/talkbank-model/src/model/header/codes/language.rs` — `LanguageCode`
- `crates/talkbank-model/src/model/file/utterance/utterance_language.rs` — `UtteranceLanguage`
- `crates/talkbank-model/src/model/language_metadata/source.rs` — `LanguageSource`
- `crates/talkbank-model/src/validation/context.rs` — digit-allowing languages
- `crates/talkbank-model/src/validation/word/language/digits.rs` — digit validation

### batchalign3 (Rust)
- `crates/batchalign-app/src/types/domain.rs` — `LanguageCode3`, `LanguageSpec`
- `crates/batchalign-app/src/revai/preflight.rs` — `RevAiLanguageHint`, mapping table
- `crates/batchalign-app/src/revai/asr.rs` — `revai_code_to_iso639_3()`, Rev.AI dispatch
- `crates/batchalign-app/src/pipeline/transcribe.rs` — Auto resolution
- `crates/batchalign-app/src/runner/mod.rs` — Worker pool lang keys
- `crates/batchalign-chat-ops/src/asr_postprocess/lang_detect.rs` — whatlang detection
- `crates/batchalign-chat-ops/src/asr_postprocess/mod.rs` — Cantonese normalization gate
- `crates/batchalign-chat-ops/src/asr_postprocess/num2text.rs` — per-language number expansion
- `crates/batchalign-cli/src/args/commands.rs` — CLI `--lang` parsing

### batchalign3 (Python)
- `batchalign/inference/_domain_types.py` — `LanguageCode`, `LanguageCode2`
- `batchalign/inference/asr.py` — `iso3_to_language_name()`, Whisper inference
- `batchalign/inference/morphosyntax.py` — Stanza dispatch
- `batchalign/inference/coref.py` — English-only hardcode
- `batchalign/inference/types.py` — `WhisperASRHandle.gen_kwargs()`
- `batchalign/inference/hk/_common.py` — `provider_lang_code()`, Cantonese normalization
- `batchalign/worker/_stanza_loading.py` — `iso3_to_alpha2()`, MWT_LANGS
- `batchalign/worker/_types.py` — `_WorkerState.lang`, `WorkerBootstrapRuntime.lang`
- `batchalign/worker/_types_v2.py` — V2 IPC types with lang fields
