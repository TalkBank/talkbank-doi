# Permissiveness Decisions Log (Regression Triage)

This document records parser/validator behaviors that were made more permissive during regression triage so the team can intentionally re-evaluate them later.

Date: 2026-02-10
Context: restore reference-corpus roundtrip/validation/alignment after recent tightening regressions.

## 1) `[*]` no longer requires explicit error code
- Previous tightened behavior:
  - Raised `E214` when scoped error annotation had empty code (`[*]`).
- Current permissive behavior:
  - Accept bare `[*]` without emitting `E214`.
- Implementation:
  - `rust/crates/talkbank-model/src/model/annotation/annotated.rs`
  - Removed validation branch that emitted `EmptyAnnotatedScopedAnnotations` for empty `ScopedAnnotation::Error`.
- Why:
  - Reference files such as `errormarkers.cha` and `compound.cha` use bare `[*]` as valid CHAT.
- Revisit criteria:
  - If team decides to require coded error annotations, do it behind explicit strict mode/profile.

## 2) `@t` no longer requires explicit `@s:<lang>`
- Previous tightened behavior:
  - Raised `E248` for `@t` with no explicit language marker.
- Current permissive behavior:
  - `@t` is accepted without requiring `@s:<lang>`.
- Implementation:
  - `rust/crates/talkbank-model/src/validation/word/structure.rs`
  - Removed both `@t` explicit-language enforcement checks.
- Why:
  - Reference file `formmarkers.cha` contains `a@t` and is expected valid.
- Revisit criteria:
  - If stricter tertiary-language policy is desired, scope it to explicit strict validation mode.

## 3) Undeclared inline language codes are currently not rejected
- Previous tightened behavior:
  - Inline word-level language markers (`@s:...`) with codes not declared in `@Languages` emitted `E254`.
- Current permissive behavior:
  - No `E254` emitted from inline marker resolution for explicit/mixed/ambiguous marker forms.
- Implementation:
  - `rust/crates/talkbank-model/src/validation/word/language/resolve.rs`
  - Removed undeclared-code checks in `Explicit`, `Multiple`, `Ambiguous` branches.
- Why:
  - Reference `lang-marker.cha` intentionally exercises extra codes.
- Revisit criteria:
  - Team decision needed: strict declaration enforcement vs permissive inline language experimentation.

## 4) Mixed-language digit legality now uses permissive-any policy
- Previous tightened behavior:
  - Digits had to be legal in **all** applicable languages for mixed/ambiguous markers.
- Current permissive behavior:
  - Digits are accepted if legal in **at least one** applicable language.
- Implementation:
  - `rust/crates/talkbank-model/src/validation/word/language/digits.rs`
  - Switched from `is_valid_in_all(...)` to `any(...)`.
- Why:
  - Prevent false positives in mixed-language reference examples.
- Revisit criteria:
  - Confirm spec intent for mixed/ambiguous validation semantics.

## 5) `@Bg` nesting check narrowed to same-label nesting
- Previous tightened behavior:
  - Any nested `@Bg` while another gem scope was open emitted `E529`.
- Current permissive behavior:
  - `E529` only when nesting the **same label** (or same unlabeled scope key).
- Implementation:
  - `rust/crates/talkbank-model/src/validation/header/structure.rs`
  - Condition changed from `any_scope_open` to `same_scope_open`.
- Why:
  - Avoid false positives on reference gem/header patterns.
- Revisit criteria:
  - Decide whether nesting policy should be global or per-label and align with CHAT/CLAN behavior.

## 6) Temporal bullet checks skipped in CA mode
- Previous tightened behavior:
  - E701/E704 temporal checks ran even for CA-mode files and flagged CA examples.
- Current permissive behavior:
  - Temporal constraints are skipped when file is in CA mode.
- Implementation:
  - `rust/crates/talkbank-model/src/validation/temporal.rs`
  - `validate_temporal_constraints(..., ca_mode, ...)` early-returns when `ca_mode`.
  - Call sites updated in `rust/crates/talkbank-model/src/model/file/chat_file/validate.rs`.
- Why:
  - CA reference files include patterns that triggered false monotonicity/self-overlap diagnostics.
- Revisit criteria:
  - If needed, implement CA-specific temporal policy instead of global skip.

## 7) Pipeline validation now fails only on `Severity::Error`
- Previous tightened behavior:
  - Any validation diagnostic (including warnings) caused `PipelineError::Validation`.
- Current permissive behavior:
  - Pipeline returns validation failure only if at least one diagnostic is `Severity::Error`.
- Implementation:
  - `rust/crates/talkbank-transform/src/pipeline/parse.rs`
- Why:
  - Warnings should not block parse/transform/export pipelines.
- Revisit criteria:
  - Keep as default; if strict mode needed, add explicit flag/profile.

## 8) Spacing warnings (`W210`, `W211`) currently disabled in main-tier validation
- Previous tightened behavior:
  - Added style-level spacing warnings around terminators and overlap markers.
- Current permissive behavior:
  - Checks are disabled in core main-tier validation path.
- Implementation:
  - `rust/crates/talkbank-model/src/model/content/main_tier.rs`
  - `check_spacing_warnings(...)` invocation and helper functions removed.
- Why:
  - Generated unexpected diagnostics on files treated as valid in reference workflow.
- Revisit criteria:
  - User noted these warnings may still be legitimate.
  - Recommended return plan: reintroduce as non-blocking optional lint/profile, not core validator hard path.

## Follow-up Recommendation
- Introduce validation profiles:
  - `reference-compatible` (current permissive baseline),
  - `strict-chat` (re-enable selected tightenings),
  - `lint-style` (spacing/style warnings only).
- Keep roundtrip gate pinned to agreed profile to prevent future ambiguity.
