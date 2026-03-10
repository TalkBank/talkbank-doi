# Proposed Amendments to `talkbank-model` for Batchalign Integration

> **Update (2026-03):** The "What Python Is Still Responsible For" table
> later in this document is outdated. Server, CLI, caching, and batch
> orchestration are now Rust (`rust-next/`). Python is limited to ML
> inference workers (Stanza, Whisper, etc.) via the `batch_infer` IPC.

## Core Principles

### 1. Rust Drives, Python Provides ML

Python should never see the AST. Python's sole responsibility is **ML inference**:
load a model, feed it data, return results. Everything else — parsing, structural
reasoning, text cleaning, annotation attachment, serialization — happens in Rust.

The interface is **callbacks**. Rust drives the pipeline. When it needs a neural network,
it calls into Python with clean words (or an audio path), gets structured results back,
and continues.

```
┌──────────────────────────────────────────────────────────────┐
│  Rust (talkbank-model + batchalign-core)                     │
│                                                              │
│  parse CHAT ─► extract words ─► [callback] ─► embed ─► serialize │
│                    │                 ▲                        │
│                    │     structured   │                        │
│                    │     results      │                        │
│                    ▼                 │                        │
│              ┌───────────────────────┐                        │
│              │ Python (ML only)      │                        │
│              │  Stanza / Whisper /   │                        │
│              │  wav2vec / pyannote   │                        │
│              └───────────────────────┘                        │
└──────────────────────────────────────────────────────────────┘
```

### 2. Annotations Live on Content Nodes, Not Separate Tiers

This is the most significant architectural change. The current talkbank-model stores
dependent tiers (%mor, %gra, %wor, %pho, %sin) as **separate structures** and
cross-references them to the main tier via a count-based alignment system
(`AlignmentSet`, `MorAlignment`, `GraAlignment`, `PhoAlignment`, `SinAlignment`).

The correct architecture — already proven by the TalkBank XML format and the Java
CHAT parser — is to **embed annotations directly on the content nodes they annotate**.
The flat CHAT tier lines (`%mor:`, `%gra:`, `%wor:`) are a serialization format,
not a data model.

**Evidence: the XML already embeds everything.**

Morphology and dependency on each word:
```xml
<w>where's
  <mor type="mor">
    <mw><pos><c>adv</c><s>wh</s></pos><stem>where</stem></mw>
    <gra type="gra" index="1" head="2" relation="PRED"/>
    <mor-post>
      <mw><pos><c>v</c><s>cop</s></pos><stem>be</stem><mk type="sfxf">3S</mk></mw>
      <gra type="gra" index="2" head="0" relation="ROOT"/>
    </mor-post>
  </mor>
</w>
```

Sign annotation co-located with words via `<sg>` groups:
```xml
<sg><w>foo</w><sw>b</sw></sg>
<sg><e><action/></e><sw>c</sw><sw>d</sw></sg>
```

Timing embedded as `<media>` on the utterance or word.

The Rust data model should match this logical structure.

---

## Proposed Data Model

### Word — the central annotation carrier

```rust
struct Word {
    // === Text ===
    raw_text: String,
    cleaned_text: String,
    content: WordContents,
    category: Option<WordCategory>,
    form_type: Option<FormType>,
    lang: Option<Language>,
    untranscribed: Option<UntranscribedType>,

    // === Embedded annotations (populated during parsing or by tools) ===

    /// Word-level timing from %wor tier or forced alignment.
    /// Source of truth — %wor tier is serialized from this.
    timing: Option<WordTiming>,

    /// Morphosyntactic analysis from %mor + %gra tiers.
    /// Each entry is a Mor item (main + clitics). Vec for MWTs (e.g., "gonna" → 2 items).
    /// GRA relations are on the Mor items themselves.
    morphology: Option<Vec<Mor>>,

    // (pho and sin handled at group level — see below)
}

struct WordTiming {
    start_ms: u32,
    end_ms: u32,
}
```

`Mor` already has the right structure — `pre_clitics`, `main`, `post_clitics`, each
chunk carrying its own `GrammaticalRelation`. No change needed to `Mor` itself.

### Terminators carry annotations too

The XML shows terminators with embedded mor/gra:
```xml
<t type="p">
  <mor type="mor"><mt type="p"/><gra type="gra" index="3" head="2" relation="PUNCT"/></mor>
</t>
```

The `Terminator` type should carry optional morphology (typically just the PUNCT entry)
and its GRA relation.

### Pho and Sin — group-level embedding

%pho and %sin don't attach to individual words — they attach to **alignment groups**.
In the XML, `<sg>` groups co-locate spoken content with sign annotation:

```xml
<sg><w>foo</w><sw>b</sw></sg>
```

For %pho, the pattern is similar: phonological transcription corresponds to words or
phonological groups.

These annotations can be embedded as fields on the relevant group/word nodes:

```rust
// On words or groups that carry phonological annotation
pho: Option<String>,  // phonological transcription from %pho

// For sign groups — the <sg> pattern from XML
sin: Option<Vec<String>>,  // sign/gesture annotations from %sin
```

The exact structure depends on the complexity of %pho/%sin content. The principle is
the same: the annotation lives on the node it describes, not in a separate tier
structure.

### What `Utterance` looks like

```rust
struct Utterance {
    speaker: String,
    content: TierContent,          // main tier content with embedded annotations
    // No more separate tier fields:
    // mor: Option<MorTier>,       // REMOVED — morphology is on Word
    // gra: Option<GraTier>,       // REMOVED — gra is on Mor
    // wor: Option<WorTier>,       // REMOVED — timing is on Word
    // pho: Option<PhoTier>,       // REMOVED — pho is on content nodes
    // sin: Option<SinTier>,       // REMOVED — sin is on content nodes

    // Serialization hints (what tiers to emit in flat CHAT output)
    has_wor: bool,                 // emit %wor line? (derived: any word has timing)
    has_mor: bool,                 // emit %mor line? (derived: any word has morphology)
    // etc. — or compute these on the fly during serialization

    // Custom/free-form dependent tiers that are NOT word-aligned
    custom_tiers: Vec<CustomTier>, // %spa, %act, %xtra, etc.
}
```

---

## What Gets Eliminated

### Removed from the data model

| Current Structure | Fate |
|------------------|------|
| `MorTier` (separate tier struct) | **Removed** — `Mor` items live on `Word.morphology` |
| `GraTier` (separate tier struct) | **Removed** — `GrammaticalRelation` lives on `Mor` chunks |
| `WorTier { content: TierContent }` | **Removed** — `WordTiming` lives on `Word.timing` |
| `PhoTier` (separate tier struct) | **Removed** — pho content lives on content nodes |
| `SinTier` (separate tier struct) | **Removed** — sin content lives on content nodes |
| `AlignmentSet` (8 optional alignments) | **Removed entirely** |
| `AlignmentUnits` (14 vectors) | **Removed entirely** |
| `MorAlignment` | **Removed** — parsing embeds directly, no post-hoc alignment |
| `GraAlignment` + `GraAlignmentPair` | **Removed** — GRA is on Mor, no cross-reference needed |
| `PhoAlignment` (for wor, pho, mod) | **Removed** — annotations are embedded |
| `SinAlignment` | **Removed** — sin is embedded |
| `AlignmentPair` | **Removed** |
| `is_wor_timing_token()` | **Removed** — timing extracted during parsing |

### Removed from alignment module

| Current Code | Fate |
|-------------|------|
| `align_main_to_mor()` | **Removed** — parsing validates counts and embeds directly |
| `align_mor_to_gra()` | **Removed** — GRA attaches to Mor during parsing |
| `align_main_to_wor()` | **Removed** — timing goes on Word during parsing |
| `align_main_to_pho()` | **Removed** — pho attaches to content nodes during parsing |
| `align_main_to_sin()` | **Removed** — sin attaches to content nodes during parsing |
| `alignment_set.rs` | **Removed entirely** |
| `alignment/gra/` directory | **Removed entirely** |
| `alignment/pho.rs` | **Removed entirely** |
| `alignment/sin.rs` | **Removed entirely** |
| `alignment/mor.rs` | **Removed entirely** |

### What survives from alignment/

| Code | New Role |
|------|----------|
| `AlignmentDomain` enum | Reused by word extraction visitor and serialization traversal |
| `count_alignable_content()` | Reused by serialization (to walk content tree per domain) |
| `word_is_alignable()`, `should_skip_group()` | Reused by extraction/serialization |
| `count.rs`, `rules.rs`, `domain.rs` | Survive as `helpers/` for shared tree-walk logic |

---

## Parsing: How Flat Tiers Get Embedded

When the parser encounters flat CHAT tier lines, it embeds their content onto the
main tier's content nodes at parse time.

### %mor + %gra

1. Parse the `%mor:` line into a sequence of `Mor` items (existing parser logic)
2. Parse the `%gra:` line into a sequence of `GrammaticalRelation` entries
3. Walk the main tier content tree, counting Mor-domain-alignable items
4. Match Mor items to words positionally (position 0 → 0, 1 → 1, ...)
5. Attach GRA relations to Mor chunks (clitics expand the chunk count)
6. Set `word.morphology = Some(mor_items)` on each matched word
7. On count mismatch: report a parse warning, embed what aligns, leave `None` for the rest

### %wor

1. Parse the `%wor:` line — tokenize, extract timing from `digits_digits` markers
2. Walk the main tier content tree, counting Wor-domain-alignable items
3. Match timing to words positionally
4. Set `word.timing = Some(WordTiming { start_ms, end_ms })` on each matched word
5. On count mismatch: report a parse warning, embed what aligns, leave `None` for the rest

### %pho

1. Parse the `%pho:` line into phonological tokens
2. Walk the main tier content tree, counting Pho-domain-alignable items
3. Attach pho content to the corresponding content node
4. On count mismatch: report a parse warning

### %sin

1. Parse the `%sin:` line into sign/gesture tokens
2. Walk the main tier content tree, counting Sin-domain-alignable items
3. Attach sin content to the corresponding content node (or group)
4. On count mismatch: report a parse warning

### Handling invalid files

Mismatched tier counts produce **parse warnings**, not fatal errors. The parser embeds
what it can and leaves `None` on unmatched items. This preserves the ability to process
structurally imperfect files while making the mismatch visible. No data is silently
dropped — the warning includes the raw tier text and count details.

---

## Serialization: How Embedded Annotations Become Flat Tiers

Serialization to CHAT format walks the main tier content tree and emits flat tier
lines. This is the only real cost of the embedded architecture — one tree traversal per
tier per utterance — and it's trivial (O(n) in word count, I/O-bound anyway).

### %mor line

Walk the Mor-domain-alignable items in the content tree. For each word with
`.morphology`, emit the `Mor` in CHAT format (the `WriteChat` trait). Join with spaces.
Handle MWT joining (`~`), compounds (`+`), and terminator PUNCT.

### %gra line

Same traversal as %mor. For each Mor chunk (including clitics), emit its
`GrammaticalRelation` as `index|head|relation`. The index counter increments per
chunk (not per word).

### %wor line

Walk the Wor-domain-alignable items in the content tree. For each item, emit the word's
CHAT text. If the word has `.timing`, emit the `\x15start_ms_end_ms\x15` marker after it.
Preserves non-timeable content (events, pauses, groups) from the main tier.

### %pho, %sin

Same pattern — walk the relevant domain, emit the embedded annotation content.

### When to emit

A tier line is emitted if any word in the utterance carries the corresponding
annotation. This can be checked during the serialization walk (encountered at least
one non-None) or precomputed.

---

## The Batchalign Public API

Unchanged from the previous design — three high-level functions plus callbacks.
The internal implementation is simpler because annotations embed directly on Words
rather than building separate tier structures.

### Function 1: `add_morphosyntax`

```python
def add_morphosyntax(
    chat_text: str,
    lang: str,
    morphosyntax_fn: Callable[[list[str], str], list[MorAnalysis]],
) -> str:
    """
    Rust internally:
    1. Parses chat_text into AST (with any existing embedded annotations)
    2. For each utterance:
       a. Extracts Mor-domain words via content tree walk (using cleaned_text)
       b. Calls morphosyntax_fn(words, lang)
       c. Builds typed Mor items with proper clitic structure and GRA relations
       d. Sets word.morphology on each matched AST Word — direct embedding
    3. Serializes the full AST back to CHAT text
       (serializer walks content tree, emits %mor/%gra lines from embedded data)
    """
```

**Callback signature — `morphosyntax_fn`:**

```python
class MorAnalysis:
    """Analysis for one word (may contain multiple tokens for MWTs)."""
    tokens: list[MorToken]
    dep_head: int          # 1-indexed, 0 = root
    dep_relation: str      # e.g., "SUBJ", "OBJ", "ROOT"

class MorToken:
    """Single morphological token."""
    pos: str               # CHAT-format POS, e.g., "pro:sub", "v", "det:art"
    lemma: str
    features: str          # e.g., "1S", "3S", "PASTP"; empty if none
    clitic_type: str       # "main", "pre", or "post"

def morphosyntax_fn(words: list[str], lang: str) -> list[MorAnalysis]:
    """
    Given clean word strings and a language code, run Stanza (or similar)
    and return one MorAnalysis per input word.
    len(result) MUST equal len(words).
    """
```

**What this replaces in batchalign:**
- `annotation_clean()` — 60 lines of `.replace()` (replaced by `Word.cleaned_text`)
- DP re-alignment in `ud.py` — O(n*m) mapping of retokenized Stanza output (replaced by positional indexing)
- `parse_sentence()` — 120 lines of clitic hacks + string concatenation (replaced by Rust Mor builder)
- `generator.py` %mor/%gra construction (replaced by serialization traversal)

### Function 2: `add_alignment`

```python
def add_alignment(
    chat_text: str,
    audio_path: str,
    lang: str,
    asr_fn: Callable[[str], list[AsrWord]] | None,
    fa_fn: Callable[[str, int, int, list[str]], list[tuple[int, int]]],
) -> str:
    """
    Rust internally:
    1. Parses chat_text into AST
    2. Checks if words already have timing (from embedded .timing fields)
    3. If untimed — UTR phase:
       a. Extracts ALL Wor-domain words from the entire document
       b. Calls asr_fn(audio_path) to get full-file ASR with timestamps
       c. Runs Hirschberg edit-distance alignment (Rust, ~200x faster than Python)
       d. Sets word.timing on matched AST Words — direct embedding
    4. FA phase — for each utterance group (~30s segments):
       a. Computes segment boundaries from word.timing
       b. Extracts Wor-domain words for the segment
       c. Calls fa_fn(audio_path, start_ms, end_ms, words)
       d. Sets word.timing on each matched AST Word
       e. Fills timing gaps (extends each word's end_ms to next word's start_ms)
    5. Serializes (emits %wor lines from embedded timing)
    """
```

**Callback signatures:**

```python
class AsrWord:
    text: str
    start_s: float
    end_s: float

def asr_fn(audio_path: str) -> list[AsrWord]:
    """Full-file ASR. Return every word with timing. Used for UTR."""

def fa_fn(
    audio_path: str,
    start_ms: int,
    end_ms: int,
    words: list[str],
) -> list[tuple[int, int]]:
    """
    Forced alignment on an audio segment.
    Return per-word (start_ms, end_ms). len(result) MUST equal len(words).
    Words that couldn't be aligned: return (0, 0).
    """
```

**What this replaces in batchalign:**
- `whisper_utr.py` — full-file ASR orchestration + bulletize_doc
- `utr/utils.py` — DP alignment + timestamp injection
- `utils/dp.py` — pure Python Hirschberg at ~500ns/cell
- `whisper_fa.py` — utterance grouping + character-level DP + gap-fill
- `generator.py` %wor construction — string concatenation with `\x15` bullets

### Function 3: `create_from_asr`

```python
def create_from_asr(
    asr_words: list[AsrWord],
    speakers: list[SpeakerSegment] | None,
    lang: str,
    media_filename: str,
) -> str:
    """
    Rust internally:
    1. Groups words into utterances (by speaker turns / pause boundaries)
    2. Constructs CHAT headers (@Languages, @Participants, @ID, @Media)
    3. Builds main tiers with Words, sets word.timing on each
    4. Adds terminal bullets per utterance
    5. Serializes (emits %wor lines from embedded timing)
    """

class SpeakerSegment:
    speaker_id: str
    start_s: float
    end_s: float
```

### Utility: `parse_and_serialize`

```python
def parse_and_serialize(chat_text: str) -> str:
    """Parse CHAT into AST, serialize back. Normalizes formatting."""

def validate(chat_text: str) -> list[ValidationError]:
    """Parse and validate a CHAT file. Returns all errors/warnings."""
```

---

## What Python Is Still Responsible For

| Responsibility | Why Python |
|---------------|-----------|
| ML model loading | torch, stanza, transformers, pyannote |
| GPU inference | CUDA/MPS operations |
| Audio I/O | torchaudio, pydub, media format conversion |
| Caching | SQLite per-utterance cache (check before calling callback) |
| Batch orchestration | ProcessPoolExecutor, worker pool management |
| Progress reporting | Status hooks, dashboard WebSocket updates |
| File I/O | Reading/writing .cha files, directory traversal |
| Server/client | FastAPI, HTTP dispatch, job management |
| CLI | Click commands, argument parsing |

**Caching fits naturally in the callback:**

```python
def cached_morphosyntax(words, lang):
    key = hash_key(words, lang)
    cached = cache.get(key)
    if cached:
        return cached
    result = run_stanza(words, lang)
    cache.put(key, result)
    return result

output = batchalign_core.add_morphosyntax(chat_text, lang, cached_morphosyntax)
```

---

## Internal Rust Work Needed

### I1: Content Tree Word Visitor

Walks the `UtteranceContent` tree and collects words per domain. This is the
"collect" version of the existing `count_alignable_content`, producing references
to AST words rather than just counts.

- **Mor domain:** Excludes retraces, fillers, nonwords, fragments, untranscribed.
- **Wor domain:** Includes retraces and fillers (they were spoken); excludes nonwords,
  fragments, untranscribed.
- **Pho domain:** Includes everything phonologically produced.
- **Sin domain:** Includes everything gesturally produced.

Returns `Vec<&mut Word>` (mutable for embedding results) with positional indexing.
Python receives `["I", "want", "cookies"]` and returns results in the same order.
The Rust side knows which AST word each position corresponds to.

Reuses existing `count.rs` / `rules.rs` domain logic.

### I2: Parse-Time Tier Embedding

When the parser encounters a dependent tier line, instead of building a separate
tier structure, it:

1. Parses the tier line into items (existing parser logic for %mor, %gra, etc.)
2. Runs the content tree visitor (I1) to get the positional word list
3. Matches tier items to words by position
4. Embeds each item directly on its word (`word.morphology`, `word.timing`, etc.)
5. On count mismatch: emits a parse warning, embeds what aligns, leaves `None`

This replaces the current approach of storing separate tier structures and
then running post-hoc alignment validation.

### I3: Serialization Traversal

Walks the content tree per domain and emits flat CHAT tier lines from embedded
annotations:

- **%mor**: Walks Mor-domain words, emits `Mor` items via `WriteChat`
- **%gra**: Same walk, emits `GrammaticalRelation` entries with chunk-indexed numbering
- **%wor**: Walks Wor-domain words, emits word text + `\x15timing\x15` markers
- **%pho**: Walks Pho-domain items, emits phonological transcriptions
- **%sin**: Walks Sin-domain items, emits sign/gesture annotations

Handles gap-fill for %wor (extends `end_ms` to next `start_ms`), PUNCT handling
for %mor/%gra, and MWT joining (`~`) for %mor.

### I4: Mor/GRA Builder (for batchalign callbacks)

Takes `MorAnalysis` results from the Python callback and builds typed `Mor` items:

1. Constructs `pre_clitics` + `main` + `post_clitics` from `MorToken`s
2. Builds `GrammaticalRelation` with correct index shifting for omitted words
3. Sets `word.morphology` directly on each AST Word

Replaces batchalign's `parse_sentence()` (120 lines) and all clitic hacks.

### I5: Hirschberg Edit-Distance Aligner

For UTR: aligns ASR word sequence against transcript word sequence. Pure Rust,
~200x faster than the current Python implementation.

Match function: exact string equality after lowercasing. Could later add fuzzy
matching (Levenshtein distance threshold) for better alignment density.

### I6: UTR Orchestration

1. Extract all Wor-domain words from the document
2. Call ASR callback for timestamped words
3. Run Hirschberg alignment (I5)
4. For each match, set `word.timing` on the corresponding AST Word

### I7: FA Grouping + Gap Fill

1. Compute utterance boundaries from `word.timing` values
2. Group consecutive utterances into ~30s segments
3. Call FA callback per group
4. Set `word.timing` on each word
5. Fill timing gaps (extend `end_ms` to next `start_ms`)

---

## Phasing and Priorities

### Current Priority: Get the Rust CHAT Data Model Right

The immediate goal is to get `talkbank-model`'s data model into the right shape
— embedded annotations on content nodes, not separate tier structures. This is
the foundational change that everything else depends on. It is being done in
`talkbank-model` itself, independent of batchalign.

The broader Rust migration (callback API, pipeline simplification, Rust CLI/server)
is a future possibility **enabled by** this data model work, not concurrent with it.
The dependency chain is strictly sequential:

```
Phase 1: Embedded data model (talkbank-model)     ← CURRENT WORK
    ↓
Phase 2: Callback API (batchalign-core crate)
    ↓
Phase 3: Simplify batchalign's Python engines
    ↓
Phase 4-5: Additional pipeline functions
    ↓
(Future) Pure Rust client binary, CLI, server — see rust-migration-report.md
```

### Phase 1: Embedded Data Model (Current Work)

The foundational change: move from separate tiers to embedded annotations.
This is purely a `talkbank-model` change — no batchalign code is modified yet.

**talkbank-model changes:**
- Add `timing: Option<WordTiming>` and `morphology: Option<Vec<Mor>>` to `Word`
  (promote `runtime_time` and `runtime_morphology` to persistent fields)
- Implement parse-time embedding for %mor, %gra, %wor (I2)
- Implement serialization traversal for %mor, %gra, %wor (I3)
- Remove `MorTier`, `GraTier`, `WorTier` as data model entities
- Remove `AlignmentSet`, `AlignmentUnits`, and all alignment types for these tiers
- Remove `align_main_to_mor`, `align_mor_to_gra`, `align_main_to_wor`

**Validates:** The entire embedded architecture. If parsing, embedding, and
serialization round-trip correctly for %mor/%gra/%wor, the approach is proven.
This phase has no dependency on batchalign or Python.

### Phase 2: Callback API (batchalign-core)

Only begins after Phase 1 is complete and validated.

- Create `batchalign-core` Rust crate with PyO3 bindings
- Implement `add_morphosyntax` with content tree visitor (I1) and Mor builder (I4)
- `StanzaEngine.process()` shrinks from ~200 lines to ~30

### Phase 3: Forced Alignment

- Implement `add_alignment` (FA only, no UTR yet — assume files already have timing)
- Implement FA grouping + gap fill (I7)
- `WhisperFAEngine.process()` shrinks from ~250 lines to ~40

### Phase 4: UTR + Transcription

- Implement Hirschberg aligner (I5) and UTR orchestration (I6)
- `WhisperUTREngine` effectively disappears into `add_alignment`
- Implement `create_from_asr`
- CHAT construction from scratch (headers, main tiers, embedded timing)

### Phase 5: Pho + Sin Embedding

- Add pho/sin fields to content nodes
- Implement parse-time embedding and serialization for %pho/%sin
- Remove `PhoTier`, `SinTier`, remaining alignment types
- Remove `align_main_to_pho`, `align_main_to_sin`

(Phase 5 is lower priority — batchalign doesn't use %pho/%sin. But it completes
the architectural migration.)

### Beyond: Full Rust Migration

Everything above (Phases 1–5) produces a `batchalign-core` Rust crate that Python
calls via PyO3. The Python codebase shrinks but remains the entry point (CLI, server,
dispatch, orchestration).

A full Rust migration — Rust CLI, Rust server, pure Rust client binary — is analyzed
in `docs/rust-migration-report.md`. It's valuable but strictly downstream of this
data model work, and is **not the current goal**.

---

## Open Questions

### 1. Callback granularity: per-utterance vs batched?

The current design calls `morphosyntax_fn` once per utterance. Stanza can batch
multiple sentences efficiently. Should the callback receive all utterances at once?

**Per-utterance** is simpler and allows per-utterance caching. **Batched** is faster
for uncached processing. Could support both via an optional `batch_morphosyntax_fn`.

### 2. Error handling in callbacks

What happens when a callback raises a Python exception?
- **Fail the entire file** — simplest, matches current batchalign behavior
- **Skip the utterance** — mark as unprocessed, continue with others
- **Retry with backoff** — for transient failures (OOM on GPU)

### 3. Progress reporting

Rust drives the loop, but Python needs to report progress (dashboard).
- Pass a `progress_fn(completed, total)` callback
- Have Rust call it at natural boundaries (per utterance, per FA group)

### 4. Clitic type detection

Who decides if a Stanza MWT subtoken is a pre-clitic vs post-clitic?
- **Python:** Callback returns `clitic_type` per `MorToken`. Keeps language rules in Python.
- **Rust language table:** Callback returns MWT boundaries, Rust applies the table.
- **Stanza convention:** MWT token order implies direction per language family.

### 5. Audio handling

Keep audio loading in Python. ML frameworks expect their own audio loading
(transformers' `WhisperProcessor`, torchaudio's CTC decoder). Trying to load audio
in Rust and pass PCM to Python adds an unnecessary conversion layer.

### 6. Crate organization

- **`talkbank-model`:** Embedded data model, parse-time embedding, serialization
  traversal, content tree visitor, domain rules. Generic CHAT operations.
- **`batchalign-core`:** Mor/GRA builder from NLP output, Hirschberg aligner,
  UTR/FA orchestration, PyO3 entry points. Batchalign-specific pipeline logic.

### 7. Migration strategy for existing talkbank-model consumers

The embedded model is a breaking change for anything using `MorTier`, `GraTier`,
`WorTier`, or `AlignmentSet`. Migration path:
- The parser still accepts the same CHAT input — only the output structure changes
- Consumers that read `MorTier.items` now read `word.morphology` instead
- Consumers that read `AlignmentSet.mor` no longer need to — data is already embedded
- A compatibility shim could reconstruct the old tier structures from embedded data
  if needed during transition
