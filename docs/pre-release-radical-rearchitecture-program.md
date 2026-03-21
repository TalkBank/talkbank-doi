# Pre-Release Radical Rearchitecture Program

This document is the canonical next-session input for the pre-release cleanup
pass across `batchalign3` and `talkbank-tools`.

The bias is intentional:

- correctness over compatibility
- explicit semantics over convenience fallbacks
- typed boundaries over sentinel overloads
- narrow ownership over broad shared coordination

## Epic 1: Reclaim Trustworthy Boundary Types

Objective:

- make semantic types actually mean something again at repo and runtime
  boundaries

Current evidence:

- `batchalign3` uses `LanguageCode3::from_worker_lang()` to admit arbitrary
  strings, including `""` and `"auto"`
- `talkbank-tools` treats missing parse provenance as implicitly clean

Required changes:

- split worker-dispatch identity from user/domain language identity
- make parse provenance explicit and non-optional, or explicitly tri-state
- remove empty/default sentinels from domain types that claim to be validated

Acceptance criteria:

- no validated domain type accepts arbitrary sentinel strings
- no validator treats "unknown" as "clean"
- adapter/runtime keys use their own types instead of overloading domain types

## Epic 2: Eliminate Silent Fallback at Truth Boundaries

Objective:

- stop normalizing corruption, I/O loss, and parse failure into ordinary
  defaults

Current evidence:

- `batchalign3` recovery coerces invalid persisted state into normal failure
  states
- `batchalign3` WS notifications default payloads and ignore send failures
- `batchalign3` media walking silently skips traversal failures
- `talkbank-tools` LSP participant extraction returns empty state on parse
  failure
- `talkbank-tools` CLI lint silently skips unreadable walk entries

Required changes:

- represent corrupt persisted state explicitly
- turn lossy reporting paths into real error/reporting flows
- make adapter failures visible to callers instead of returning empty/default
  values

Acceptance criteria:

- corrupt persisted rows are diagnosable as corruption
- unreadable files/directories are reported, not skipped
- parse failure and empty data are distinct states in adapter APIs

## Epic 3: Make Worker/Process Ownership Real

Objective:

- make process lifecycle ownership explicit enough that stale/crashed cleanup
  becomes trustworthy

Current evidence:

- `SharedGpuWorker` claims child-lifecycle ownership but does not retain the
  child handle
- stale cleanup and worker liveness logic are still spread across pool, handle,
  and CLI daemon code
- pool observability currently returns approximate answers under lock
  contention

Required changes:

- choose one owner for worker child lifecycle
- keep shutdown, reaping, health, and crash accounting on that boundary
- separate pool summary state from fast-path lock probing

Acceptance criteria:

- every live worker process has one obvious lifecycle owner
- health/shutdown/reap paths operate on owned truth, not inferred side effects
- observability does not report false absence because a lock was busy

## Epic 4: Narrow Adapter Layers

Objective:

- stop CLI/LSP/dashboard/server adapters from carrying local truth that should
  belong to model/control-plane layers

Current evidence:

- `talkbank-tools` adapters manufacture empty fallback semantics
- `batchalign3` still mixes dispatch sentinels, transport keys, and user-facing
  language semantics
- some header parsing still spends too long in raw-string helper form before
  reaching typed model values

Required changes:

- keep parsing/model truth in `talkbank-tools` domain layers
- keep `batchalign3` worker-target semantics in explicit control-plane types
- reduce helper-shaped raw string handling at parser boundaries

Acceptance criteria:

- adapters map domain/control-plane results; they do not invent new fallback
  states
- parser/model boundaries emit typed values earlier
- cross-repo ownership of shared vocabulary is explicit

## Epic 5: Rebuild Test Strategy Around Architectural Risk

Objective:

- make pre-release architectural change cheap to review and safe to execute

Current evidence:

- heavy snapshot/golden suites create noise during structural changes
- current coverage does not specifically pin the new high-risk silent-failure
  classes

Required changes:

- add focused regressions for corrupt recovery, parse failure, walk failure,
  lifecycle ownership, and sentinel-type misuse
- tier suites into fast structural, medium semantic, and heavy runtime/golden
- document regeneration ownership explicitly

Acceptance criteria:

- every release-blocking finding above has a targeted regression
- structural refactors can be reviewed without depending solely on giant
  snapshot churn
- heavy suites remain broad, but they are no longer the only guardrail

## Epic 6: Decide the True Cross-Repo Boundary

Objective:

- make one explicit decision about what should live in `talkbank-tools`,
  `batchalign3`, or neither

Current evidence:

- `talkbank-tools` still has adapter-local fallback logic weakening domain truth
- `batchalign3` still carries control-plane/domain vocabulary that leaks
  sentinel and stringly values
- the current split is workable but not obviously final

Required changes:

- audit duplicated or weakened boundary concepts across both repos
- move or merge ownership where the current split obscures invariants
- allow deletion of thin but noisy boundary layers if they are not earning
  their keep

Acceptance criteria:

- each shared concept has one canonical owner
- repo boundaries help enforce invariants instead of diluting them
- future cleanup work can proceed without reopening the ownership question each
  time
