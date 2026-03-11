# rust   docs   validation   README

## Overview
Short summary for `rust/docs/validation/README.md`. This document is intentionally concise; legacy detail has been trimmed.

## Key Commands
```bash
cargo test
```

## Status and Limitations
- This is a brief reference; consult CLAUDE.md files for full workflow guidance.
- Update alongside code changes to keep commands and references accurate.
- 2026-03-06: fixed isolated-header parser drift where fragment `parse_header()` entrypoints lagged behind whole-file parsing for real reference-corpus headers. Bugs fixed included structured `@Comment` bullets in the direct parser and missing tree-sitter fragment support for `@Warning`, `@Videos`, `@Birth of`, `@Birthplace of`, and `@L1 of`. The parser suite now includes a reference-corpus-driven header parity audit.
- 2026-03-06: added a matching dependent-tier fragment parity audit. It round-trips real dependent tiers from the reference corpus through both `parse_dependent_tier()` and the typed fragment entrypoints (`%mor/%gra/%pho/%sin/%act/%cod/%add/%com/%exp/%gpx/%int/%sit/%spa/%wor`) and compares them against each parser's whole-file AST. This run found no new drift, which is now a tested invariant rather than an assumption.
- 2026-03-06: fixed tree-sitter `parse_utterance()` so it now reuses whole-file recovery and preserves preceding headers plus dependent tiers instead of silently collapsing to `Utterance::new(main)`. A new reference-corpus utterance parity audit now enforces that behavior.
- 2026-03-06: tightened context-free fragment parsing for CA omission shorthand. Instead of silently accepting fragments like `*CHI:\t(word) .` without header context, both parser backends now reject them with an explicit error saying file context (`@Options: CA` / `CA-Unicode`) is required. We also added `FragmentSemanticContext` plus `*_with_context(...)` fragment APIs so callers that do know file semantics can parse the same fragments correctly. The reference-corpus main-tier parity audit still excludes `@Options:` files from the context-free default path.
- 2026-03-06: fixed a public API drift in `talkbank-parser`: the crate-level `parse_main_tier()` / `parse_utterance()` convenience wrappers were still bypassing the `ChatParser` trait implementation and silently skipping the new fragment-context checks. Public integration tests now lock those wrappers to the same semantics as the trait-backed parser APIs.

## See Also
- CLAUDE.md
- rust/CLAUDE.md

---
Last Updated: 2026-03-06
