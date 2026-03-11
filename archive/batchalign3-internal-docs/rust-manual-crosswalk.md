# Rust Manual Crosswalk

This page maps the main public manuals and migration references to the Rust code areas they govern today.

## Public References

- CHAT manual: structural rules for CHAT files
- CLAN manual: legacy command behavior where Batchalign still intentionally tracks it
- Batchalign3 migration book in this repo: historical crosswalk from Batchalign2 to the current architecture

## Code-to-Reference Map

| Code area | What to consult first |
| --- | --- |
| `crates/batchalign-chat-ops` | CHAT manual plus Batchalign3 migration/reference chapters |
| `crates/batchalign-cli/src/args/` | user CLI reference in this book plus migration book |
| `crates/batchalign-cli/src/dispatch/` | server-mode docs plus migration book |
| `crates/batchalign-app/src/routes/` | server-mode docs, OpenAPI output, migration book |
| `crates/batchalign-app/src/pipeline/` | architecture chapters on orchestration and command contracts |
| `pyo3/` | Python API docs plus CHAT manual sections relevant to the exposed operations |

## Practical Rule

When the public docs disagree with old migration-era notes, treat the current
CLI reference, current architecture chapters, and current test suite as the
authoritative release surface. Historical branch-by-branch notes belong in
private maintainer archives, not in the public contract.
