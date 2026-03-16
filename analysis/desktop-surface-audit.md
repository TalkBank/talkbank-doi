# Desktop Surface Audit

**Status:** Current
**Last updated:** 2026-03-16

## Scope

This audit covers:

- `talkbank-tools/desktop`
- `batchalign3/apps/dashboard-desktop`
- the TypeScript surfaces those apps depend on
- remaining mutex usage in `talkbank-tools` and `batchalign3`

The goal is to keep the desktop surfaces from sprawling before they accrete
ad hoc runtime state, duplicated business logic, or boundary drift.

## Executive summary

- `talkbank-tools/desktop` is a real app with meaningful Rust ↔ Tauri ↔ React
  boundaries, not just a shell. Its main risks are boundary drift and UI/runtime
  glue growing in local hooks.
- `batchalign3/apps/dashboard-desktop` is intentionally a thin Tauri shell over
  `batchalign3/frontend`. The shell should stay narrowly focused on native
  capabilities and local process/config integration.
- `talkbank-tools/vscode` already uses `effect` successfully. That does **not**
  justify a blanket rollout across the other TypeScript surfaces.
- The remaining mutexes are not uniformly bad. Most are acceptable local
  coordination details; the main policy question is whether the lock stays
  tightly encapsulated and off the architectural surface.

## `talkbank-tools/desktop`

### What is already solid

- Rust-side validation cancellation is already lock-free:
  `src-tauri/src/commands.rs` uses `ArcSwapOption<Sender<()>>`.
- The backend streams typed validation events via `events.rs` and Tauri emit.
- The frontend keeps most state inside one clear hook,
  `src/hooks/useValidation.ts`.
- `desktop/CLAUDE.md` already establishes the right architecture direction:
  TUI parity, Rust-side ownership for validation behavior, and a no-mutex rule.

### Audit findings

1. `validate(paths)` only uses `paths[0]` today in
   `src-tauri/src/commands.rs`.
   The multi-file UI can therefore imply broader support than the backend
   currently provides.

2. `DropZone.tsx` still carries a placeholder-style drag/drop path extraction.
   It uses HTML5 file metadata (`file.name`) rather than a native Tauri drop
   pathway, so this boundary needs a deliberate implementation rather than
   assuming the browser model maps onto desktop.

3. `events.rs` emits `errors: ParseError[]` and `rendered: string[]` as parallel
   arrays. `types.ts` and `useValidation.ts` preserve that shape, which means the
   Rust/TS boundary relies on index alignment rather than a single paired type.

4. Error surfacing was inconsistent. `open_in_clan` failures were already being
   made visible locally in `App.tsx`; this pass extends the same visible failure
   treatment to export failures.

5. `ErrorPanel.tsx` intentionally renders Rust-produced HTML with
   `dangerouslySetInnerHTML`. That remains reasonable because the rendering is
   owned by trusted Rust code, but it makes the Rust-side render contract part
   of the security boundary and worth treating carefully.

### Recommended next slices

- Pair rendered miette HTML with the corresponding parse error at the type
  boundary instead of passing parallel arrays.
- Make the drag/drop pathway explicitly Tauri-native.
- Decide whether multi-path validation should be truly supported or deliberately
  constrained to one path in the UI.
- Move more Tauri command/event plumbing behind a typed adapter boundary before
  `useValidation()` and `App.tsx` grow more imperative special cases.

## `batchalign3/apps/dashboard-desktop`

### What is already solid

- The shell is intentionally small: file discovery, config read/write, and local
  server lifecycle.
- The canonical product UI remains `batchalign3/frontend`.
- The frontend already has a coherent split between React Query (server/cache)
  and Zustand (shared dashboard UI state).

### Audit findings

1. The shell was exposing `ServerProcess(pub Mutex<Option<Child>>)`, which made
   the lock itself part of the public state shape. This pass fixes that by
   encapsulating the mutex behind `ServerProcess::new()` and thin methods.

2. `config.rs` used only `$HOME` for `~/.batchalign.ini`, which is not a safe
   cross-platform assumption for Windows. This pass adds `USERPROFILE` /
   `HOMEDRIVE` + `HOMEPATH` fallback handling.

3. `frontend/src/hooks/useServerLifecycle.ts` directly imports Tauri at the hook
   boundary and duplicates some server-start expectations that also exist in the
   shell `setup` hook. That is still serviceable, but it is where desktop-only
   coupling is most likely to sprawl next.

4. Desktop-vs-web runtime checks are spread across multiple frontend files
   (`runtime.ts`, `lib/tauri.ts`, `app.tsx`, several components/hooks). The code
   is still understandable, but it is a good candidate for a single explicit
   desktop runtime/context seam before more desktop-specific behavior accumulates.

### Recommended next slices

- Centralize desktop-only frontend behavior behind one `useDesktopContext()` or
  equivalent boundary instead of scattering `isDesktop()` and raw Tauri invoke
  calls.
- Keep the shell thin: native capabilities, local process ownership, and config
  persistence only.
- Add focused Rust tests for `discover_files`, config roundtrips, and basic
  server lifecycle behavior.

## Effect assessment

### Where Effect already fits well

`talkbank-tools/vscode` already uses `effect` in a way that makes sense:

- service tags via `Context.GenericTag`
- typed errors via `Data.TaggedError`
- explicit runtime/layer assembly in `effectRuntime.ts`
- command execution composition in `effectCommandRuntime.ts`

That code is a good reference for future bounded adoption.

### Recommendation matrix

| Surface | Recommendation | Rationale |
|---------|----------------|-----------|
| `talkbank-tools/vscode` | Keep current Effect usage | The command/runtime layer already benefits from typed services and tagged errors. |
| `talkbank-tools/desktop` | Consider later, at the Tauri boundary only | Could help if command/event orchestration grows more complex, but React components and hooks should stay idiomatic React. |
| `batchalign3/frontend` | Do not adopt now | React Query + Zustand already fit the current problems better than a new Effect runtime layer would. |
| `batchalign3/apps/dashboard-desktop` shell | Do not adopt now | The shell is too thin to justify an Effect layer of its own. |

### Bottom line

Effect should be treated as a **precision tool**, not a stack-wide style rule.
The strongest immediate seam for it would be a future typed desktop runtime
adapter in `talkbank-tools/desktop`, not a wholesale migration of the React
frontends.

## Mutex classification

### `talkbank-tools`

| File | Classification | Notes |
|------|----------------|-------|
| `crates/talkbank-model/src/errors/collectors.rs` | Acceptable local detail | A small in-memory collector; the mutex is local and not architectural. |
| `crates/talkbank-transform/src/lock_helpers.rs` | Candidate cleanup / reassessment | A poison-recovery helper, but currently unused in production search results. |
| `desktop/` Rust backend | No production mutexes found | Current desktop state follows the existing no-mutex policy. |

### `batchalign3`

| File | Classification | Notes |
|------|----------------|-------|
| `crates/batchalign-app/src/store/registry.rs` | Acceptable bounded wrapper | The mutex is private to the registry and the rest of the server talks in store operations, not lock plumbing. |
| `crates/batchalign-app/src/store/counters.rs` | Acceptable bounded wrapper | Small isolated counter store; not worth actorization on its own. |
| `crates/batchalign-app/src/worker/pool/mod.rs` + `lifecycle.rs` + `checkout.rs` | Important coordination hotspot, but presently justified | The mix of semaphore + short critical-section mutexes is deliberate and far better than the earlier long-held async mutex shape. Keep watching this area, but it is not a knee-jerk rewrite target. |
| `apps/dashboard-desktop/src-tauri/src/server.rs` | Acceptable local detail after encapsulation | The mutex is now private to `ServerProcess` instead of leaking through the public state type. |
| `crates/batchalign-app/src/runtime_supervisor.rs` | Positive reference pattern | Shows the preferred actor-style direction when task ownership truly warrants it. |

## Changes landed in this pass

- `batchalign3/apps/dashboard-desktop/src-tauri/src/server.rs`
  - hid the raw `Mutex<Option<Child>>` behind `ServerProcess`
  - moved start/stop/status/shutdown behavior behind methods
- `batchalign3/apps/dashboard-desktop/src-tauri/src/main.rs`
  - switched shell setup to `ServerProcess::new()`
- `batchalign3/apps/dashboard-desktop/src-tauri/src/config.rs`
  - added Windows-friendly home-directory fallback logic for
    `~/.batchalign.ini`
- `talkbank-tools/desktop/src/App.tsx`
  - made export failures visible to users instead of console-only

## Recommended next implementation wave

1. Tighten the Chatter desktop Rust ↔ TS event boundary by replacing parallel
   rendered/error arrays with a paired representation.
2. Add a single explicit desktop-runtime boundary in `batchalign3/frontend`
   before more desktop-only logic spreads across hooks and components.
3. Add a small test layer for the dashboard desktop shell
   (`discover_files`, config roundtrip, and server lifecycle smoke checks).
4. Revisit `talkbank-tools/desktop` multi-path validation support and native
   drag/drop behavior so the UI contract matches the backend reality.
