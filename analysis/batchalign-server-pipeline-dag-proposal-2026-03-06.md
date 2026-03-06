# Batchalign Server Pipeline DAG Proposal

## Goal

Introduce a small internal pipeline abstraction for `batchalign-server` that makes
per-command stage dependencies explicit without trying to replace:

- the top-level job runner
- cross-file batching
- worker-pool scheduling
- job/file status persistence

The target is command-local orchestration for:

- `transcribe`
- `morphotag`
- `utseg`
- `translate`
- possibly `coref`

This is explicitly **not** a proposal for a generic executor crate.

## Current Status

As of March 6, 2026, a first implementation exists in `batchalign-server`:

- `transcribe` runs through an explicit stage plan
- `morphotag` runs through an explicit stage plan
- `utseg` and `translate` use a smaller shared single-file cached text pipeline
- cross-file batching remains in the command modules
- the top-level job runner remains unchanged

This document is therefore both a proposal and a record of what was actually
built.

## Why This Fits The Current Code

The current server already has explicit stage pipelines, but they are encoded as
handwritten control flow:

- `transcribe`: ASR -> postprocess -> build CHAT -> optional utseg -> optional morphosyntax
  - `batchalign3/rust-next/crates/batchalign-server/src/transcribe.rs`
- `morphotag`: parse -> validate -> clear -> collect -> cache split -> infer -> inject -> cache store -> serialize
  - `batchalign3/rust-next/crates/batchalign-server/src/morphosyntax.rs`
- `utseg`: parse -> validate -> collect -> cache split -> infer -> apply -> cache store -> serialize
  - `batchalign3/rust-next/crates/batchalign-server/src/utseg.rs`
- `translate`: parse -> validate -> collect -> cache split -> infer -> apply -> cache store -> serialize
  - `batchalign3/rust-next/crates/batchalign-server/src/translate.rs`
- `coref`: parse -> validate -> language gate -> collect -> infer -> apply -> serialize
  - `batchalign3/rust-next/crates/batchalign-server/src/coref.rs`

The duplication is real, but it is mostly in stage shape, not in runtime
mechanics. That makes a small in-repo abstraction reasonable.

## Non-Goals

- Do not model `runner::run_job()` as a DAG.
- Do not move `dispatch_batched_infer()` or file-level result recording into the abstraction.
- Do not build a parallel executor for pipeline stages.
- Do not unify cross-file batching and per-file processing into one generic engine.
- Do not expose this as a public crate API.

## Proposed Module Layout

The original target module tree was:

```text
batchalign3/rust-next/crates/batchalign-server/src/pipeline/
  mod.rs
  context.rs
  stage.rs
  plan.rs
  run.rs
  trace.rs
  text_infer.rs
  transcribe.rs
```

### `mod.rs`

The current implemented surface is smaller than originally proposed:

```rust
pub(crate) mod morphosyntax;
pub(crate) mod plan;
pub(crate) mod text_infer;
pub(crate) mod transcribe;
```

Instead of one universal context type, the implementation uses:

- one shared `PipelineServices` bundle
- one generic stage runner
- command-specific context structs where needed
- one smaller shared helper for the simple cached text commands

### `stage.rs`

Each stage is static metadata plus an async function pointer.

```rust
use std::future::Future;
use std::pin::Pin;

use crate::error::ServerError;

use super::context::PipelineContext;
use super::plan::StageId;

pub(crate) type StageFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), ServerError>> + Send + 'a>>;

pub(crate) type StageFn =
    for<'a> fn(&'a mut PipelineContext<'a>) -> StageFuture<'a>;

pub(crate) struct StageSpec {
    pub id: StageId,
    pub deps: &'static [StageId],
    pub enabled: fn(&PipelineContext<'_>) -> bool,
    pub run: StageFn,
}
```

### `plan.rs`

`StageId` is small and explicit. Do not use strings.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum StageId {
    Parse,
    PreValidate,
    ClearExisting,
    CollectPayloads,
    PartitionCache,
    InjectCacheHits,
    Infer,
    ApplyResults,
    CacheStore,
    PostValidate,
    Serialize,
    AsrInfer,
    AsrPostprocess,
    BuildChat,
    OptionalUtseg,
    OptionalMorphosyntax,
}

pub(crate) struct PipelinePlan {
    pub stages: &'static [StageSpec],
}
```

### `run.rs`

This became part of `plan.rs`. It executes in dependency order, but still
sequentially. The value is:

- explicit dependencies
- pruning disabled stages
- consistent tracing and timing
- consistent missing-output checks

```rust
pub(crate) async fn run_plan(
    plan: &PipelinePlan,
    ctx: &mut PipelineContext<'_>,
) -> Result<(), ServerError> {
    // 1. mark enabled stages
    // 2. topo-sort static stage graph
    // 3. execute sequentially
    // 4. record duration and error per stage
    // 5. require final output before returning
}
```

This does not need true DAG infrastructure beyond a tiny topological runner over
`StageId`.

### `transcribe.rs`

The first concrete plan should be `transcribe`, because it has real branching.

```rust
pub(crate) fn transcribe_plan() -> PipelinePlan;
pub(crate) async fn run_transcribe_pipeline(
    audio_path: &str,
    services: PipelineServices<'_>,
    opts: &TranscribeOptions,
) -> Result<String, ServerError>;
```

### `text_infer.rs`

This now holds a shared skeleton for `utseg` and `translate`, but not
`morphotag`.

```rust
pub(crate) trait TextInferPipeline {
    type PayloadSet;
    type CachePartition;
    type WorkerOutput;

    const COMMAND: &'static str;

    fn parse(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;
    fn pre_validate(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;
    fn clear_existing(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;
    fn collect_payloads(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;

    fn needs_cache_partition(ctx: &PipelineContext<'_>) -> bool {
        true
    }

    async fn partition_cache(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;
    fn inject_cache_hits(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;
    async fn infer(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;
    fn apply_results(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;
    async fn cache_store(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;
    fn post_validate(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;
    fn serialize(ctx: &mut PipelineContext<'_>) -> Result<(), ServerError>;
}
```

That proved to be the right constraint line so far. `coref` still probably
stays separate initially because:

- it is document-level
- it does not use the utterance cache
- it has an English-only gate

## Concrete Initial Plans

### Plan 1: `transcribe`

```text
AsrInfer
  -> AsrPostprocess
  -> BuildChat
  -> OptionalUtseg
  -> OptionalMorphosyntax
  -> Serialize
```

Execution notes:

- `OptionalUtseg` is enabled only when `opts.with_utseg`.
- `OptionalMorphosyntax` depends on:
  - `OptionalUtseg` when utseg is enabled
  - otherwise `BuildChat`
- `Serialize` can be a final explicit stage, even if upstream stages often
  already produce `String`. That keeps plan shape consistent.

### Plan 2: Shared text-infer skeleton

```text
Parse
  -> PreValidate
  -> ClearExisting?        // morphotag only
  -> CollectPayloads
  -> PartitionCache
  -> InjectCacheHits?      // morphotag only as a distinct step
  -> Infer?                // skipped when no misses
  -> ApplyResults
  -> CacheStore?           // skipped when no misses
  -> PostValidate
  -> Serialize
```

Command-specific differences:

- `morphotag`
  - was tested against this abstraction and turned out to need its own explicit
    staged module instead of this lighter shared helper
- `utseg`
  - no clear-existing stage
  - cached and inferred assignments both feed one apply step
- `translate`
  - same high-level shape as `utseg`
- `coref`
  - should not be forced into this skeleton in phase 1

## Recommended First Refactor Sequence

### Phase 1: internal runner only

Add the pipeline module and implement:

- `StageId`
- `StageSpec`
- `PipelinePlan`
- `run_plan()`
- stage tracing

Do not migrate any production command yet.

### Phase 2: migrate `transcribe`

Refactor `process_transcribe()` in:

- `batchalign3/rust-next/crates/batchalign-server/src/transcribe.rs`

Keep existing helper functions such as:

- `infer_asr()`
- `convert_asr_response()`
- `generate_participant_ids()`

Only move orchestration flow into the plan.

This is the lowest-risk proof because the graph is real and the function is
already single-file and linear.

### Phase 3: extract `text_infer` skeleton

Migrate:

- `process_utseg()`
- `process_translate()`

Do the single-file variants first. Leave cross-file batch functions alone until
the single-file abstraction proves itself.

### Phase 4: decide on `coref`

Only after the shared text-infer skeleton settles:

- either keep `coref` bespoke
- or add a small second skeleton for document-level infer tasks

## What Should Stay Outside The Pipeline Layer

These responsibilities belong to the existing runner and dispatch code:

- job admission and memory gate
- cancellation tokens
- semaphore permits
- worker pre-scaling
- Rev.AI preflight submission
- path-mode media validation
- per-file database updates
- staging and result file writes
- cross-file batching and response repartitioning

Those are operational concerns. The proposed pipeline layer is for per-command
dataflow inside a single file or transcript.

## Expected Benefits

- Makes stage dependencies explicit instead of implicit in ad hoc control flow.
- Gives one place to add stage timings and traces.
- Reduces duplicated orchestration patterns across `morphotag` / `utseg` / `translate`.
- Makes optional downstream stages in `transcribe` easier to reason about.
- Makes cache-hit pruning a first-class concept rather than distributed branch logic.

## Expected Risks

- Over-generalizing too early will make the code worse.
- A too-generic context type will collapse into `Option` soup.
- Pulling cross-file batching into the same abstraction will increase complexity fast.
- Forcing `coref` into the text-infer skeleton too early will distort the API.

## Success Criteria

The abstraction is worth keeping only if, after migrating `transcribe` and one
text command:

- orchestration code gets shorter or clearer
- tracing gets more consistent
- no command-specific invariants are hidden behind weak generic names
- tests remain at least as readable as before

If those conditions are not met, the proposal should be rolled back and the
useful pieces retained only as conventions.
