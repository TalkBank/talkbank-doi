# Final Last Steps: Batchalign3 Migration Closure Plan

This document defines the final execution checklist to finish migration from the legacy Python CLI/server model to Rust-first operation, while preserving the minimal Python model runtime and adding HK features via plugins.

## Scope Boundary

In scope:
- Rust CLI/server as the only supported production control plane.
- Python used only as worker/model runtime.
- HK-specific engines moved into plugin packages.
- Compatibility with `~/.batchalign.ini` retained.

Out of scope for this execution:
- GitHub repo setup changes.
- Release/tag/publish actions.

## Exit Criteria

1. All user-facing processing workflows run through `batchalign3` (Rust CLI).
2. No active dependency on legacy Python CLI command routing.
3. HK functionality is packaged as plugin(s), not core hacks.
4. Plugin engines are selectable from the Rust CLI.
5. Documentation covers install/config/test flow, including required credentials and permission constraints.
6. CI/local tests pass for touched components.

## Final Execution Checklist

## 1. Core Migration Closure

- [ ] Verify no docs/scripts recommend deprecated command names (`batchalign-next`, etc.).
- [ ] Verify all command references use `batchalign3`.
- [ ] Verify Rust launcher pathing is explicit and deterministic (`BATCHALIGN_RUST_BIN`, installed binary names).
- [ ] Verify `~/.batchalign.ini` is documented as current compatibility config path.

## 2. HK Pluginization

- [ ] Create dedicated HK plugin package as a sibling project (`~/talkbank/batchalign-hk-plugin`).
- [ ] Port Tencent ASR engine.
- [ ] Port Aliyun ASR engine.
- [ ] Port FunASR ASR engine.
- [ ] Port HK UTR engines where needed (Tencent/FunASR).
- [ ] Port Cantonese Wave2Vec forced-alignment preprocessing (`jyutping` normalization path).
- [ ] Register plugin entry point under `batchalign.plugins`.
- [ ] Keep all HK dependencies optional and lazy-imported.

## 3. Rust CLI Engine Selection

- [ ] Add explicit engine override flags so plugin engines can be selected without core modifications.
- [ ] Maintain backward-compatible defaults for existing users.
- [ ] Add parser/build-options tests for new override flags.

## 4. Permissions and Privacy Assessment (HK)

- [ ] Confirm which tests are blocked without credentials.
- [ ] Enumerate required credentials in `~/.batchalign.ini`.
- [ ] Record why HK was private historically:
  - Chinese cloud-provider account requirements (Tencent/Aliyun).
  - Organization-specific operational access and keys.
  - Heavy/region-specific dependency footprint.
  - Private collaboration autonomy and cadence.

## 5. Documentation Closure

- [ ] User guide: install HK plugin, set config keys, run commands.
- [ ] Developer guide: plugin architecture, engine registration patterns, test strategy.
- [ ] Migration notes: clearly list what is added beyond batchalign2 baseline.

## 6. Validation

- [ ] Rust CLI argument tests pass.
- [ ] Python plugin discovery/import tests pass.
- [ ] Server integration path still passes for builtin + plugin command acceptance.
- [ ] Workspace tests for touched files pass.

## 7. Long-Term Governance (10+ Years)

- [ ] Keep host/plugin API compatibility explicitly versioned and documented.
- [ ] Avoid re-inlining provider-specific HK code into `batchalign3` core.
- [ ] Define deprecation policy for plugin extension points before changing contracts.
- [ ] Keep credential-gated integrations optional and isolated by package boundaries.

## 8. Dashboard Long-Term Migration (Dioxus)

- [x] Accept Rust-native Dioxus dashboard direction (ADR).
- [x] Create Dioxus dashboard scaffold crate under `rust-next`.
- [x] Keep static dashboard hosting contract stable (`BATCHALIGN_DASHBOARD_DIR` / `~/.batchalign3/dashboard`).
- [x] Port job detail/actions and websocket updates to Dioxus.
- [x] Add parity test checklist and cutover gate.
- [ ] Deprecate React dashboard only after operator-flow parity is verified.

## Permission/Access Reality for HK Testing

Full end-to-end validation for Tencent/Aliyun requires credentials and active cloud resources. Without them, we can still perform:

- Import-time validation (plugin discovery, engine registration).
- CLI option/dispatch validation.
- Unit-level checks for text/timestamp transforms.

Credential-gated live API validation requires:

- Tencent Cloud: SecretId/SecretKey + COS bucket/region.
- Aliyun NLS: AK ID/secret + AppKey/token issuance.
- Optional service/network access depending on institution policy.

## Immediate Next Action

Implement HK plugin package and Rust CLI engine override flags, then run focused validation.
