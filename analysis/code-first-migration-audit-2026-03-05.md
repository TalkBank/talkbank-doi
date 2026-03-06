# Code-First Migration Audit (2026-03-05)

## Scope

This audit is intended to replace doc-derived assumptions with code-backed
migration facts.

Comparison anchors:

- `batchalign2` baseline:
  `84ad500b09e52a82aca982c41a8ccd46b01f4f2c` (2026-01-09)
- later released `batchalign2` master-branch point:
  `e8f8bfada6170aa0558a638e5b73bf2c3675fe6d` (2026-02-09)
- current working tree / HEAD of `batchalign3` on 2026-03-05

Public migration-book rule implied by this audit:

- document durable differences from Jan 9 to now;
- optionally mention Feb 9 when it represents the later shipped BA2 surface;
- do not narrate transient unreleased intermediate branch states.

## Method

Used direct code inspection, not prior docs, for:

- `git show` on Jan 9 and Feb 9 anchors in `~/batchalign2-master`
- `git log` and `git diff --stat/--name-only` across:
  - `84ad500..e8f8bfa` in `~/batchalign2-master`
  - `e8f8bfa..HEAD` across BA2 release surface -> current `batchalign3`
- current source inspection in:
  - `batchalign/`
  - `rust/`
  - `rust-next/`

This file is a working audit. It should drive migration-book edits, not be
treated as polished end-user documentation.

## High-Confidence Findings

## 1) Jan 9 -> Feb 9 was already a major `batchalign2` architecture shift

This is not just a bug-fix window. By Feb 9, the released master-branch BA2 had
already moved materially beyond the Jan 9 baseline.

Evidence:

- Jan 9 CLI was a local-only Click interface with `in_dir out_dir` command
  shape in `batchalign/cli/cli.py`.
- Feb 9 CLI had:
  - multi-input path handling via `_resolve_inputs`
  - global `--workers`
  - `--force-cpu`
  - `--lazy-audio`
  - `--server`
  - global `--override-cache`
  - public `cache` and `bench` commands
  - server-directed execution and run-log machinery behind the CLI
  - structured run logging

Code anchors:

- Jan 9: `batchalign/cli/cli.py`
- Feb 9: `batchalign/cli/cli.py`, `batchalign/cli/dispatch.py`,
  `batchalign/cli/cache.py`, `batchalign/cli/bench.py`

Migration implication:

- the migration book must not describe Feb 9 BA2 as "basically Jan 9 plus a
  few fixes";
- users coming from the Feb 9 release surface need a different before/after
  story than users comparing directly to Jan 9.

## 2) Jan 9 -> Feb 9 introduced first-class cache and long-run operations

By Feb 9, caching and server-directed long-run support were already part of
shipped BA2, even though the public CLI had not yet expanded into the full
BA3-era ops command surface.

Evidence:

- cache layer introduced in `batchalign/pipelines/cache.py`
- CLI cache management introduced in `batchalign/cli/cache.py`
- server/job internals introduced in `batchalign/serve/*`
- dispatch router chose among local and server-directed execution paths in
  `batchalign/cli/dispatch.py`

Code anchors:

- Feb 9: `batchalign/pipelines/cache.py`
- Feb 9: `batchalign/serve/app.py`, `batchalign/serve/job_store.py`,
  `batchalign/serve/jobs.py`

Migration implication:

- the Batchalign3 ops story should be framed as:
  - Jan 9 -> Feb 9: Python BA2 gained cache plus server-directed runtime
    support and benchmarking helpers
  - Feb 9 -> now: those surfaces were re-implemented and type-hardened in the
    Rust-first control plane

## 3) Jan 9 -> Feb 9 morphotag evolved materially inside released BA2

The Jan 9 morphosyntax implementation was concentrated in
`batchalign/pipelines/morphosyntax/ud.py`, a large Python file mixing:

- Stanza interaction
- feature mapping
- token handling
- DP utilities
- CHAT-specific logic

By Feb 9, the morphosyntax entry point was still in Python `ud.py`, but that
file and surrounding pipeline code had already changed substantially:

- caching infrastructure had been added around morphotag work
- tokenizer/context handling had been revised
- Hirschberg-based DP improvements had landed
- several performance and robustness changes had already shipped inside BA2

This means the durable migration story for morphotag must compare:

- Jan 9 BA2 baseline
- Feb 9 released BA2 master
- current BA3

not just Jan 9 directly to current.

By contrast, the later thin
`StanzaEngine` wrapper in `batchalign/pipelines/morphosyntax/engine.py`:

- Rust `ParsedChat` handle is used directly
- `%mor`/`%gra` are cleared and re-injected on the structured handle
- per-language Stanza pipelines are cached
- MWT realignment is explicit
- cache keys are computed by Rust and used in Python
- batch callback path exists for cross-utterance processing

Code anchors:

- Jan 9 / Feb 9 BA2: `batchalign/pipelines/morphosyntax/ud.py`
- later internal transition work: `batchalign/pipelines/morphosyntax/engine.py`
- later internal transition work: `batchalign/pipelines/morphosyntax/_tokenizer_realign.py`
- later internal transition work: `batchalign/pipelines/morphosyntax/_stanza_batch_callback.py`

Migration implication:

- some major morphotag correctness/performance changes already happened inside
  released BA2 before batchalign3, but the Rust-owned CHAT/control-plane
  architecture is still a later BA3-era step.

## 4) Jan 9 -> Feb 9 improved performance model in durable ways

The durable performance improvements in this window are architectural, not just
micro-optimizations.

Evidence from commit history and code additions:

- lazy imports in CLI modules
- pooled / parallel local processing
- adaptive worker caps / memory gating
- lazy audio loading
- caching infrastructure
- daemon/server execution to avoid per-run cold starts

Code anchors:

- Feb 9: `batchalign/cli/cli.py`
- Feb 9: `batchalign/cli/dispatch.py`
- Feb 9: `batchalign/models/audio_io.py`
- Feb 9: `batchalign/pipelines/cache.py`
- Feb 9: `batchalign/pipelines/fa/whisper_fa.py`

Migration implication:

- the migration book should distinguish:
  - Jan 9 BA2: simpler local execution
  - Feb 9 BA2: already performance-tuned Python architecture
  - current BA3: further shift to Rust CLI/server orchestration and worker IPC

## 5) Feb 9 -> present is a control-plane rewrite, not a continuation of the same architecture

Current `batchalign3` is not just the Feb 9 Python architecture with more
patches.

Evidence:

- Rust CLI binary front door:
  - `rust-next/crates/batchalign-bin/src/main.rs`
  - `rust-next/crates/batchalign-cli/src/args/mod.rs`
- Rust server orchestrator:
  - `rust-next/crates/batchalign-server/src/*`
- Python worker IPC protocol:
  - `batchalign/worker/_protocol.py`
- typed wire contracts:
  - `rust-next/crates/batchalign-types/src/options.rs`
  - `rust-next/crates/batchalign-types/src/worker.rs`

Current architectural boundary:

- binary parses typed args in Rust
- Rust dispatches to server / daemon / worker layers
- text-only commands are orchestrated in Rust server infer path
- Python workers are inference endpoints, not owners of CHAT lifecycle

Migration implication:

- Feb 9 -> now should be described as a control-plane rewrite with stronger
  types and sharper process boundaries, not as another incremental tuning pass.

## 6) Feb 9 -> present introduced typed command/job/worker contracts

This is one of the most durable engineering differences and should be explicit
in the migration book.

Evidence:

- `CommandOptions` tagged enum replaces untyped option maps in
  `rust-next/crates/batchalign-types/src/options.rs`
- `ProcessInput` enum distinguishes CHAT text from media-path requests in
  `rust-next/crates/batchalign-types/src/worker.rs`
- worker stdio protocol validates request shapes using Pydantic in
  `batchalign/worker/_protocol.py`

Migration implication:

- data structure choices changed materially:
  - less stringly typed routing
  - fewer "bag of kwargs / JSON blob" boundaries
  - more compile-time and validation-time enforcement

## 7) Feb 9 -> present moved text-command orchestration fully into Rust

This is the biggest durable behavior boundary after the last released BA2
master surface.

Evidence:

- current server infer path in
  `rust-next/crates/batchalign-server/src/runner/dispatch/infer.rs`
  does:
  - read file content
  - parse / batch / dispatch text-only commands

## 8) `transcribe` changed in three distinct stages

This needs to be described separately from `align`.

### Jan 9 BA2

`transcribe` was already a Python-owned audio-to-CHAT pipeline.

`batchalign/pipelines/asr/utils.py:process_generation(...)` already did:

- compound merging
- timestamp conversion
- multi-word splitting
- number expansion
- long-turn splitting
- punctuation-based retokenization or utterance-engine retokenization
- CHAT `Document` assembly

So Jan 9 BA2 already bundled inference, post-processing, retokenization, and
CHAT construction into one Python command path.

### Jan 9 -> Feb 9 BA2

The released BA2 line improved runtime robustness and startup behavior, but did
not yet change that ownership model.

Code-backed improvements include:

- lazy imports and lighter startup in `rev.py`, `whisper.py`, `whisperx.py`
- safer fallback when utterance retokenization engine is missing/invalid in
  `asr/utils.py`
- `whisperx.py` chunked long-audio handling and deferred device/model setup
- much larger dispatch/runtime improvements in `batchalign/cli/dispatch.py`

Durable conclusion:

- Feb 9 BA2 was a more robust/faster Python `transcribe`,
- but still not a split inference/orchestration architecture.

### Current BA3

Current BA3 explicitly splits `transcribe` into:

- Python worker infer only:
  - `batchalign/inference/asr.py`
  - raw tokens with timestamps and speaker labels
- Rust postprocess:
  - `rust-next/crates/batchalign-chat-ops/src/asr_postprocess/mod.rs`
- Rust CHAT assembly:
  - `rust-next/crates/batchalign-chat-ops/src/build_chat.rs`
- optional Rust-orchestrated `utseg` and `morphotag`:
  - `rust-next/crates/batchalign-server/src/transcribe.rs`

Migration implication:

- the durable BA3 step is not merely “better ASR”
- it is moving from Python monolithic transcript construction to typed
  infer/postprocess/build/inject stages.

## 9) `translate` changed less inside BA2, and more at the boundary in BA3

### Jan 9 BA2

`translate` already operated utterance-by-utterance in Python:

- extract utterance text by `Utterance.strip(...)`
- call Google Translate or Seamless
- patch punctuation/spacing into `i.translation`
- write translation back through Python CHAT generation

Code anchors:

- `batchalign/pipelines/translate/gtrans.py`
- `batchalign/pipelines/translate/seamless.py`

### Jan 9 -> Feb 9 BA2

The released BA2 changes here were minor:

- lazy import of translation engines and heavy libraries
- no material algorithmic rewrite

So the public migration story should not overstate Feb 9 BA2 translation
changes.

### Current BA3

Current BA3 translation boundary is much sharper:

- Python worker:
  - `batchalign/inference/translate.py`
  - pure `(text, src_lang) -> raw_translation`
- Rust extraction / preprocessing / injection:
  - `rust-next/crates/batchalign-chat-ops/src/translate.rs`
- Rust parse / validate / cache / batch orchestration:
  - `rust-next/crates/batchalign-server/src/translate.rs`

Durable change:

- translation is no longer a Python command that happens to mutate CHAT;
- it is a Rust-owned CHAT workflow with Python serving only inference.

## 10) `utseg` changed from Python tree-walk + DP alignment to Rust-owned assignment/application

### Jan 9 BA2

Jan 9 BA2 `utseg` already did full segmentation decisions in Python:

- Stanza constituency parse
- S-level phrase extraction
- phrase de-duplication
- DP alignment of phrase payloads back to utterance forms
- small-group merging
- new utterance construction

Code anchor:

- `batchalign/pipelines/utterance/ud_utterance.py`

This is a textbook example of the old architecture:

- flatten subtree leaves to strings,
- align them back to utterance forms,
- then rebuild utterances from matched positions.

### Jan 9 -> Feb 9 BA2

Released BA2 improved:

- lazy Stanza loading
- cache plumbing
- some robustness / typing cleanup

But the core algorithm remained the same:

- Python owned constituency parsing,
- Python owned phrase-range extraction,
- Python still used DP alignment to reconcile subtree text with utterance forms.

### Current BA3

Current BA3 splits the job into explicit stages:

- Python worker returns raw constituency tree strings:
  - `batchalign/inference/utseg.py:batch_infer_utseg`
- Rust computes assignments from trees:
  - `rust-next/crates/batchalign-chat-ops/src/utseg_compute.rs`
- Rust applies utterance splits to CHAT AST:
  - `rust-next/crates/batchalign-chat-ops/src/utseg.rs`
- Rust server owns parse / validate / cache / serialize:
  - `rust-next/crates/batchalign-server/src/utseg.rs`

Migration implication:

- the key improvement is not merely speed;
- it is replacing flatten-align-rebuild segmentation logic with explicit tree
  payloads, assignment vectors, and AST mutation.

## 11) `coref` was already document-level in BA2, but BA3 changed the contract and output tier semantics

This corrects an earlier overstatement in the migration draft.

### Jan 9 BA2

BA2 `coref` was already document-level:

- it detokenized the whole document into one string
- ran Stanza coref over the whole text
- flattened sentence words back into payloads
- DP-aligned those payloads back to `(utterance_idx, form_idx)`

Code anchor:

- `batchalign/pipelines/morphosyntax/coref.py`

It then wrote ordinary `%coref:` tier output through the BA2 generator:

- `batchalign/formats/chat/generator.py`

### Jan 9 -> Feb 9 BA2

The released BA2 delta here was minimal:

- lazy `stanza` import
- no substantive algorithmic change

### Current BA3

Current BA3 still treats coref as document-level, but changes almost every
other boundary:

- Python worker input is typed sentence arrays, not raw detokenized document
  text:
  - `batchalign/inference/coref.py`
- Python returns structured chain refs, not already-materialized tier strings
- Rust converts structured chains to bracket notation and injects `%xcoref`
  sparsely:
  - `rust-next/crates/batchalign-chat-ops/src/coref.rs`
- Rust server owns parse / validation / English gating / serialization:
  - `rust-next/crates/batchalign-server/src/coref.rs`

Durable migration facts:

- BA3 did not invent document-level coref;
- BA3 replaced BA2’s detokenize-then-DP-remap workflow with typed
  sentence-level payloads and structured sparse `%xcoref` injection.

## 12) `benchmark`, `opensmile`, and `avqi` also follow a three-stage migration story

These commands are less about CHAT correctness than `align` or `morphotag`, but
they still matter for the public migration book because their runtime and API
boundaries changed.

### `benchmark`

#### Jan 9 BA2

`benchmark` already existed as a CLI command that:

- found a gold `.cha`
- ran ASR through the ordinary Python dispatch path
- wrote `.wer.txt`, `.diff`, and `.asr.cha` outputs

This was still a Python CLI/pipeline workflow, not a typed service boundary.

#### Jan 9 -> Feb 9 BA2

Released BA2 improved surrounding runtime behavior substantially:

- large dispatch/runtime rewrite in `batchalign/cli/dispatch.py`
- dedicated `bench` timing helper in `batchalign/cli/bench.py`
- better worker/runtime controls shared with the rest of BA2

But `benchmark` still remained a Python-owned command flow.

#### Current BA3

Current BA3 splits the problem more explicitly:

- command options are typed in Rust:
  - `rust-next/crates/batchalign-types/src/options.rs`
- per-file execution uses explicit process dispatch:
  - `rust-next/crates/batchalign-server/src/runner/dispatch/process.rs`
- WER computation is now exposed as a dedicated infer task:
  - `batchalign/inference/benchmark.py`
  - worker `_infer.py`
- the scoring core is Rust-backed:
  - `batchalign_core.wer_compute(...)`

Durable change:

- `benchmark` is still a per-file process-style command,
- but the metrics path is no longer ad hoc Python-only glue; it is routed
  through typed worker/infer contracts and Rust-backed scoring.

### `opensmile`

#### Jan 9 BA2

`opensmile` already existed as a Python feature-extraction command:

- initialize `opensmile.Smile`
- process the media file
- write CSV-like feature output

The whole path was Python-owned.

#### Jan 9 -> Feb 9 BA2

The released BA2 change was again mostly runtime-focused:

- lazy `opensmile` / `pandas` loading
- dispatch/runtime improvements from the broader BA2 CLI rewrite

There was no deep algorithmic rewrite of feature extraction itself.

#### Current BA3

Current BA3 keeps `opensmile` as pure inference on the worker side, but adds a
clearer contract around it:

- typed command options in Rust
- infer task advertised explicitly by worker capabilities
- structured `OpenSmileBatchItem` / `OpenSmileResult`
- explicit `"csv"` vs `"chat"` style content handling in the worker/server
  contracts

Code anchors:

- `batchalign/inference/opensmile.py`
- `batchalign/worker/_infer.py`
- `batchalign/worker/_handlers.py`
- `rust-next/crates/batchalign-types/src/worker.rs`

Durable change:

- feature extraction itself is still Python/library-owned,
- but invocation, typing, and output handling are much less ad hoc.

### `avqi`

#### Jan 9 BA2

`avqi` already existed as a Python command over paired `.cs` / `.sv` files:

- convert to mono
- run Praat/parselmouth feature calculations
- compute AVQI score
- write `.avqi.txt`

#### Jan 9 -> Feb 9 BA2

Released BA2 improvements were again mostly operational:

- lazy loading of heavy libraries
- audio loading moved toward shared helpers
- broader dispatch/runtime improvements

The core AVQI calculation remained the same Praat/parselmouth-style algorithm.

#### Current BA3

Current BA3 keeps AVQI as a pure worker inference task but regularizes the
boundary:

- typed `AvqiBatchItem`
- explicit infer dispatch in `worker/_infer.py`
- typed result payload
- typed Rust-side `AvqiOptions`
- per-file process dispatch with explicit media-path input

Code anchors:

- `batchalign/inference/avqi.py`
- `rust-next/crates/batchalign-server/src/runner/dispatch/process.rs`
- `rust-next/crates/batchalign-types/src/options.rs`

Durable change:

- the AVQI math did not fundamentally change between Feb 9 BA2 and current BA3;
- the main BA3 change is typed process/infer integration and cleaner runtime
  boundaries.

## 13) Utility command surface also changed in two public steps

The utility/ops commands need the same Jan 9 / Feb 9 / current separation.

### Jan 9 BA2

Public CLI surface included:

- `setup`
- `version`
- `models`

There was no public `cache` or `bench` command yet, and none of the later BA3
ops commands (`serve`, `jobs`, `logs`, `openapi`, `fleet`, `gui`) were public
CLI commands.

### Feb 9 BA2

Released BA2 added:

- public `cache` command
- public `bench` command
- much richer dispatch/runtime behavior behind ordinary processing commands
  (`--server`, worker/memory controls, run logging)

But the released BA2 master CLI still did not expose the full BA3-style public
ops command family.

### Current BA3

Current BA3 adds the public Rust CLI ops surface:

- `serve`
- `jobs`
- `logs`
- `openapi`
- `fleet`
- `gui`

while preserving:

- `setup`
- `version`
- `models`
- `cache`
- `bench`

More precisely:

- `setup`, `models`, and `version` are continuity commands, but their control
  surfaces shifted:
  - `setup` remains about `~/.batchalign.ini`, but current BA3 validates
    interactive/non-interactive flows in Rust instead of relying on Python
    Click wiring.
  - `models` remains a public training entrypoint, but current BA3 still
    delegates to Python rather than claiming a Rust training rewrite.
  - `version` is now an explicit subcommand that reports both package version
    and build hash.
- `cache` and `bench` are the main Jan 9 -> Feb 9 BA2 utility additions:
  - Feb 9 BA2 `cache` was a Python-side cache manager tool with
    stats/clear/warm behavior.
  - current BA3 `cache` is aligned to the Rust runtime's actual analysis/media
    cache state rather than the older Python cache-management boundary.
  - Feb 9 BA2 `bench` was a repeated-dispatch timing helper.
  - current BA3 `bench` keeps that intent but is now typed, Rust-dispatched,
    and emits structured benchmark output.
- `serve`, `jobs`, `logs`, and `openapi` are genuine BA3-era public utility
  commands driven by the server/job control plane:
  - `serve` exists because server lifecycle is now part of normal runtime use.
  - `jobs` and `logs` exist because asynchronous job state and run inspection
    are now explicit surfaces.
  - `openapi` exists because the Rust HTTP API is a first-class contract that
    needs drift checking.

Migration implication:

- Jan 9 BA2 -> Feb 9 BA2 already matters for users who adopted cache/bench and
  newer runtime controls;
- Feb 9 BA2 -> BA3 then turns operational support into a first-class typed CLI
  surface instead of mostly being hidden behind processing-command flags and
  Python dispatch internals.
  - write output
  - update job/file state
- worker `process` handler now explicitly says all commands should use infer
  path, not generic process path:
  - `batchalign/worker/_handlers.py`

Migration implication:

- current public docs should say:
  - for text-only commands, Rust owns parse -> cache -> infer -> inject ->
    serialize
  - Python is no longer the authoritative owner of CHAT manipulation for those
    commands

## 8) Feb 9 -> present narrowed and constrained runtime DP reconstruction

This is the most important algorithmic migration finding so far, but it needs
to be stated precisely.

Evidence:

- Jan 9 had a general-purpose Levenshtein-style DP utility in
  `batchalign/utils/dp.py`
- Feb 9 still used DP heavily in released BA2 alignment paths, but the DP
  utility had already been upgraded to a Hirschberg-style implementation in
  `batchalign/utils/dp.py`
- current UTR timing transfer in `rust/src/speaker_ops.rs` is explicit:
  - stable `word_id` mapping first
  - unique-window monotonic matching second when utterance bullets exist
  - deterministic global monotonic fallback only when utterance windows are
    absent
  - ambiguous cases remain unassigned instead of triggering a broad global
    remap
- current FA alignment logic in
  `rust-next/crates/batchalign-chat-ops/src/fa/alignment.rs` is explicit:
  - indexed timings are applied by index
  - token-level timings are stitched deterministically in order
  - unmatched words remain `None`
  - this path does not do broad global DP remapping
- current retokenization mapping in
  `rust-next/crates/batchalign-chat-ops/src/retokenize/mapping.rs`:
  - first tries deterministic concatenation/range mapping
  - then falls back to conservative monotonic binning
  - explicitly warns "without DP"
- current codebase still retains a shared Hirschberg DP library and golden /
  allowlist tests around it, so the accurate claim is not "DP no longer
  exists"; it is that broad runtime remap policy has been narrowed and made
  more explicit in the active paths above

Migration implication:

- the migration book should state plainly that current runtime output paths aim
  to avoid silent broad DP tie-breaking in:
  - UTR timing transfer
  - retokenization mapping
  - current `rust-next` FA response transfer

## 9) Feb 9 -> present hardened `%mor` / `%gra` correctness with explicit invariants

Current morphosyntax mapping is no longer just "generate strings and hope they
line up."

Evidence:

- `rust-next/crates/batchalign-chat-ops/src/nlp/mapping.rs`:
  - builds chunk-index map because `%gra` indexes chunks, not raw tokens
  - uses ROOT head `0`
  - validates root structure
  - validates head references
  - validates chunk-count parity between `%mor` and `%gra`
- current tests in that module cover:
  - ROOT conventions
  - MWT chunk indexing
  - invalid roots / invalid heads

Migration implication:

- `%mor`/`%gra` output differences versus BA2 should often be treated as
  correctness fixes backed by explicit graph/chunk invariants, not as arbitrary
  formatting changes.

## 10) The deeper design shift is away from string hacking / array hacking

This is the cross-command engineering theme that best explains the migration.

Jan 9 and released Feb 9 BA2 still contain many paths where correctness depends
on reconstructing structure after flattening it:

- morphotag in `ud.py` builds `mor`, `gra_tmp`, `actual_indicies`, and
  `mor_clone` arrays, then performs positional repair passes for clitics, MWTs,
  and punctuation
- UTR in released BA2 still derives rough utterance bullets from ASR transcript
  output via `bulletize_doc(...)`
- FA in released BA2 maps model output back by Python-side DP over flattened
  character streams

Current BA3 moves toward principled structure and explicit indices instead:

- stable word identity:
  - `u{n}:w{n}` word IDs in UTR and FA payloads
- explicit positional metadata:
  - `word_utterance_indices`
  - `word_utterance_word_indices`
  - chunk-index maps for `%gra`/`%mor` alignment
- typed AST walks and rebuilds:
  - retokenization rebuilds content by iterating AST containers, not by
    flattening/re-splitting text
- deterministic iteration policies:
  - window-constrained monotonic timing transfer
  - deterministic token stitching
  - explicit index-aligned timing application

Migration implication:

- the public migration book should emphasize that the real improvement is not
  "rewrote it in Rust";
- it is that more commands now preserve identity and iterate structured content
  directly instead of flattening, hacking arrays, and later trying to recover
  truth with broad DP or positional repair.

## 11) Feb 9 -> present added pluginized provider extensibility

Current extensibility is no longer based on carrying fork-specific patches.

Evidence:

- plugin descriptors and discovery in `batchalign/plugins.py`
- provider/task contracts are explicit
- command/task mappings can be extended via entry points

Migration implication:

- this belongs in migration documentation for users coming from private forks:
  current extension point is plugins, not a source fork.

## Morphotag Table

| Topic | Jan 9 (`84ad500`) | Feb 9 (`e8f8bfa`) | Current `batchalign3` |
|---|---|---|---|
| Core implementation shape | Large monolithic Python `ud.py` mixes Stanza calls, feature mapping, special-form logic, `%gra` generation, and retokenization | Still a large Python `ud.py`, but substantially revised inside released BA2; surrounding cache/runtime infrastructure was added | Rust/server/chat-ops own payload extraction, injection, validation, and cache semantics; Python worker is an inference endpoint |
| Retokenization | Character-level DP alignment in Python to reconcile Stanza tokenization back to CHAT | Still Python-side retokenization in released BA2, but now backed by the improved Hirschberg DP utility and a revised `ud.py` flow | Deterministic span/range mapping first, conservative monotonic fallback second, explicitly no runtime DP remap in retokenize mapping |
| `%mor`/`%gra` injection target | Python document objects rebuilt from text-centric pipeline state | Still Python-owned injection in `ud.py`; released BA2 improves surrounding robustness but does not yet move this into Rust-owned chat ops | Typed AST / chat-ops injection path with explicit alignment validation |
| Special forms | `xbxxx` placeholder with Python-side `@s -> L2|xxx`, other `@` forms mapped directly in `ud.py`; retokenize path restores `xbxxx` from lemma | Same basic special-form strategy in released BA2, with surrounding cleanup and caching but no fundamental model change | Special-form overrides applied explicitly in Rust inject path from `FormType`; `@s -> L2|xxx`, `@c -> c|...`, `xbxxx` restoration tested |
| Reflexive pronouns | Python emits `reflx` suffix in `handler__PRON` | Still emitted in released BA2 | Explicit feature mapping in Rust `features.rs` emits `reflx` for `Reflex=Yes` |
| `%gra` root convention | Python builds `%gra` by shifted positional indices; root handling is positional and not validated | Same released-BA2 graph family, still without explicit root/head invariant enforcement | ROOT head explicitly `0`; invalid root / invalid head / chunk-count mismatch are hard errors in mapping layer |
| MWT / chunk indexing | Python code manages MWTs with positional assumptions and manual index shifting | Still managed in Python `ud.py` on the released BA2 line | Chunk-based UD->CHAT mapping is explicit and validated; MWT chunk indexing has direct tests |
| Multilingual skip behavior | `skipmultilang` checked during utterance loop | Same high-level policy family in released BA2, inside revised Python morphotag flow | `skipmultilang` is part of typed payload collection / multilingual policy with tests |
| Cache boundary | No dedicated morphotag cache layer on Jan 9 | General pipeline cache infrastructure exists by Feb 9 and morphotag work can benefit from it, but the released BA2 path is still Python-owned | Shared typed cache architecture across Rust/Python, server-owned orchestration, cache contract enforced in chat-ops/server |

## Timing / Alignment Table

| Topic | Jan 9 (`84ad500`) | Feb 9 (`e8f8bfa`) | Current `batchalign3` |
|---|---|---|---|
| `align` command surface | Simple local `align` command with UTR + FA engine selection | Adds `--utr/--no-utr`, global cache bypass, richer multi-input dispatch, server/daemon-aware CLI | Rust CLI typed options feed Rust dispatch/server pipeline |
| FA callback contract | Python FA pipeline calls model and does char-level DP alignment in Python | Still Python-owned model call and mapping path in released BA2, but now with group-level cache checks and better failure handling | Rust chat-ops own FA grouping, response parsing, timing injection, `%wor`, monotonicity, overlap handling |
| Token-to-word mapping | Python `whisper_fa.py` uses DP alignment over characters from model output to transcript text | Still Python DP-based mapping in released BA2, now backed by the Hirschberg utility and cache-aware group flow | Current token-level FA path uses deterministic normalized token stitching; unmatched words remain untimed rather than globally remapped |
| Word-level FA mapping | Mixed Python logic | Still Python-owned word-timing writeback, with cache-aware group reuse and safer exception handling | Indexed word-level timings are applied by index; length mismatch is rejected |
| Stable word identity | Not first-class in Jan 9 alignment path | Still not first-class in released BA2 callback payloads | FA infer item carries `word_ids`, `word_utterance_indices`, and `word_utterance_word_indices` explicitly |
| Untimed utterances / UTR | UTR derives rough utterance bullets from ASR output via `bulletize_doc(...)` before FA | Same released BA2 UTR algorithm family; `--no-utr` makes the policy boundary more explicit | Current UTR maps by stable word IDs first, then unique-window monotonic matching, with deterministic global fallback only when utterance windows are absent |
| `%wor` generation | Python writes timings and later adjusts/repairs word spans manually | Still Python-owned timing writeback and repair, with cache/robustness improvements around the released BA2 path | `%wor` generation is a first-class chat-ops step after injection/postprocess; tests cover bullet presence and grouping |
| Overlap / monotonicity governance | Mostly post-hoc Python repair logic | Still predominantly post-hoc Python repair/governance in the released BA2 line | Current FA pipeline explicitly enforces monotonicity and strips same-speaker E704 overlaps in orchestration layer |
| Cache boundary | No explicit raw-FA callback cache layer on Jan 9 | Raw FA callback results cached by audio identity + span + words + engine | Shared utterance/cache backend plus server-run FA path with explicit audio identity and engine versioning |

## Align Improvements Worth Calling Out Publicly

These are high-confidence, user-visible `align` improvements that belong in the
migration book.

### Jan 9 BA2 -> Feb 9 BA2

- `align` gained released cache-backed FA execution for both Whisper FA and
  Wave2Vec FA group processing.
- FA robustness improved for short/impossible segments:
  - Wave2Vec path logs and skips failures instead of collapsing the whole run.
  - Hirschberg/DP edge-case fixes landed in the released Feb 9 BA2 point.
- the broader CLI/runtime around `align` improved:
  - more performance-focused dispatch work
  - lazy audio loading support
  - global cache bypass support in the runtime
  - merge-abbreviation output option
- this means users comparing Jan 9 BA2 to current BA3 should not skip over the
  fact that released BA2 had already materially improved `align`.

### Feb 9 BA2 -> current BA3

- FA payloads are now explicit typed items with:
  - stable `word_ids`
  - `word_utterance_indices`
  - `word_utterance_word_indices`
  - explicit audio spans and timing mode
- grouping, timing injection, `%wor`, utterance bullet updates, monotonicity,
  and same-speaker overlap cleanup are first-class Rust chat-ops steps rather
  than distributed Python repair logic.
- current token-level FA mapping is intentionally deterministic:
  - normalized token stitching in order
  - no silent global DP remap
  - unmatched words remain untimed explicitly
- current word-level FA mapping is stricter:
  - indexed responses are applied by index
  - response length mismatches are rejected
- untimed utterances now have explicit proportional-boundary estimation logic
  when total audio duration is available.

### Migration wording that is now justified

Safe public claim:

- `align` improved twice:
  - first inside released BA2 (cache, robustness, dispatch/runtime gains),
  - then again in BA3 through Rust-owned orchestration, typed payloads, and
    deterministic timing transfer rules.

## Precision Notes for Migration Book

These statements are now safe to make, based on code inspection:

- Jan 9 -> Feb 9 already included major correctness/performance/runtime
  changes inside BA2.
- Feb 9 -> now is a deeper rewrite centered on Rust control-plane ownership,
  typed contracts, worker IPC, and Rust-owned CHAT orchestration.
- current runtime output paths intentionally avoid reintroducing broad DP-based
  remap policy in UTR, retokenization, and current `rust-next` FA response
  transfer.
- current `%mor`/`%gra` generation is defended by explicit invariants around
  roots, heads, and chunk counts.

Additional command-family statements now supported by code:

- current `transcribe` is explicitly split into:
  - Python ASR inference only
  - Rust post-processing
  - Rust CHAT assembly
  - optional Rust-owned `utseg` and `morphotag` follow-on stages
- current `translate` inference returns raw translated text while Rust owns
  CHAT extraction/injection
- current `utseg` inference returns raw constituency trees while Rust computes
  assignments and mutates CHAT
- current `coref` is document-level, sparse, and Rust-injected as `%xcoref`
  rather than being treated as a per-utterance text decoration task

Additional morphotag-specific corrections now supported by code/tests:

- current `%gra` generation enforces ROOT head `0` and rejects invalid
  root/head/chunk-count structures
- MWT expansions produce per-component `%gra` entries instead of relying on raw
  token-position repair
- `@c` and `@s` special forms are mapped explicitly in injection
- `xbxxx` placeholders are restored during retokenization
- reflexive pronouns explicitly emit `reflx`
- retokenization-vs-preserve behavior is tested explicitly rather than being an
  accidental byproduct of string reconstruction

Important split across the three comparison points:

- already present in Jan 9 BA2 and still present in released Feb 9 BA2:
  - reflexive `reflx`
  - `@s -> L2|xxx` special-form handling
  - direct `@c`-style special-form POS mapping in the old Python path
  - `xbxxx` placeholder restoration in the retokenize path
- improved in released Feb 9 BA2 mainly as cleanup/performance/robustness:
  - safer Python implementation details
  - cache-aware morphotag execution
  - revised `ud.py` flow and tokenizer context handling
- genuinely current-BA3-era correctness hardening:
  - explicit ROOT head `0`
  - invalid root/head/chunk-count rejection
  - chunk-based `%gra` indexing with direct tests
  - structurally validated retokenize-vs-preserve behavior in Rust-owned paths

These still need deeper code inspection before being promoted into user-facing
migration prose:

- exact corpus-facing performance claims with numbers
- exact split of those morphotag divergence fixes between released Feb 9 BA2
  and post-BA2 current BA3
- exact corpus-level impact of the current UTR policy change relative to BA2
- exact fleet-processing status history across Python BA2 server path, early
  rust-next, and current release behavior

## Next Audit Targets

1. Build a code-backed morphotag divergence table:
   - done at high confidence; still refine user-facing wording
2. Build a code-backed timing/alignment table:
   - mostly done; still refine exact UTR transfer wording
3. Build a code-backed runtime/ops table:
   - local-only
   - Python daemon/server BA2
   - current Rust CLI/server/worker split

## Runtime/config audit notes added late 2026-03-05

Additional code-backed corrections from current Rust CLI/server inspection:

- `--file-list` is not additive in current input resolution; when present, it
  supplies the input path set (`resolve.rs`).
- Local daemon port selection is deterministic from `server.yaml` (default
  `8000`), not random/ephemeral (`daemon.rs`).
- Global flag semantics are mixed in current BA3:
  - live: `--force-cpu`, `--override-cache`, `--no-lazy-audio`
  - compatibility no-op: `--use-cache`, `--memlog`, `--mem-guard`,
    `--adaptive-workers`, `--no-adaptive-workers`, `--pool`, `--no-pool`,
    `--adaptive-safety-factor`, `--adaptive-warmup`, `--shared-models`,
    `--no-shared-models`
- `--bank` / `--subdir` are current server-media selectors for `benchmark` and
  `opensmile`; they are not a general transcribe remote-media surface in the
  current CLI (`args/options.rs`, `args/commands.rs`, `dispatch/single.rs`,
  `dispatch/paths.rs`).
- Current Rust discovery behavior is stronger than older Python server-mode
  documentation implied:
  - non-matching files are copied for directory inputs when `out_dir` is used
  - dummy CHAT files are filtered locally and copied unchanged
  - matching inputs are sorted by size descending before submission
  (`discover.rs`, `dispatch/single.rs`, `dispatch/paths.rs`).
- Current FA response handling in `rust-next` is narrower than older broad-DP
  descriptions:
  - indexed word timings are applied positionally
  - Whisper token timings use deterministic in-order stitching
  - unmatched words remain untimed instead of triggering transcript-wide remap
  (`fa/alignment.rs`).
