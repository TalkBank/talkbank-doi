# Plugin Migration Compatibility ADR

## Status

**Superseded** (March 2026). The plugin system was removed entirely. HK engines
were folded into the core as built-in engines with optional extras. See
[HK/Cantonese Engines: Migration and Architecture](../architecture/hk-cantonese-engines.md).

## Original Context (February 2026)

The Rust migration changed where commands are validated and dispatched.
Plugins still provide Python-side extensions via `batchalign.plugins`, including:

1. engine registrations
2. task metadata
3. `cmd2task` and capability probes

Historically, plugin descriptors also exposed Python CLI command hooks
(`PluginDescriptor.commands`).

## Original Decision

1. Keep Python plugin extension points for engines/tasks/runtime metadata.
2. Treat `PluginDescriptor.commands` as legacy compatibility only.
   Rust CLI does not dynamically execute those Python command hooks.
3. Rust server command validation accepts:
   built-in commands OR commands advertised by worker capabilities.
   This preserves plugin command acceptance for HTTP submissions.
4. Discovery logs a warning when legacy Python CLI command hooks are declared.

## Superseding Decision (March 2026)

The plugin system was **deleted entirely**:

- `batchalign/plugins.py` removed (discovery, `PluginDescriptor`, `InferenceProvider`)
- All plugin test infrastructure removed
- The sole plugin (`batchalign-hk-plugin`) was folded into `batchalign/inference/hk/`
- Engine dispatch uses `AsrEngine`/`FaEngine` enums (type-safe, compile-time checked)
- Optional dependencies use pip extras (`batchalign3[hk-tencent]`, etc.)

### Rationale

1. **Single consumer** — only `batchalign-hk-plugin` used the plugin system.
2. **Entry-point fragility** — `importlib.metadata.entry_points()` caused
   silent failures that were difficult to diagnose.
3. **Enum dispatch is safer** — compile-time exhaustiveness, clear errors.
4. **Optional extras are equivalent** — same user experience without the
   machinery.

### Consequences

1. No plugin discovery code exists. Third-party engines must be added as
   built-in modules (or proposed upstream).
2. Engine selection is type-safe via Rust and Python enums.
3. The `batchalign-hk-plugin` repository is archived.
4. If a future need arises for external extensibility, a new mechanism
   (e.g., Rust-side plugin loading) would be designed from scratch rather
   than reviving the Python entry-point approach.

## Follow-up Criteria (Preserved)

Introduce a new extension mechanism only when:

1. there is a concrete user need for third-party inference providers,
2. a typed registration schema ensures safety and discoverability, and
3. parity tests cover the full dispatch path.
