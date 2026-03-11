# End User and Editor Experience

## Objective
Design the project so non-technical and semi-technical users can reliably process and edit CHAT files
without needing to understand parser internals.

## User Segments
1. Transcript editors fixing malformed CHAT in existing corpora.
2. Researchers running validation and batch cleanup workflows.
3. Tool-assisted annotators using CLI or editor integration.
4. Support contributors triaging user-reported parse and validation issues.

## User Experience Principles
1. Show actionable errors, not internal jargon.
2. Preserve user data and avoid destructive rewrites by default.
3. Offer guided correction paths for common failure patterns.
4. Keep advanced parser controls available but optional.

## Core User Journeys

### Journey A: Validate a File and Understand Failures
- Input: one `.cha` file.
- Output:
  - summary status,
  - grouped diagnostics by severity,
  - precise location and suggested fix.
- UX requirement: single command and clear next step.

### Journey B: Batch Validate a Corpus
- Input: directory tree.
- Output:
  - per-file outcomes,
  - aggregate error distribution,
  - machine-readable report for workflow tools.
- UX requirement: stable progress, resumable runs, clear failure policy.

### Journey C: Parse and Rewrite Safely
- Input: valid or near-valid CHAT file.
- Output:
  - optional normalized rewrite,
  - no silent semantic changes,
  - explicit mode (`roundtrip` vs `normalized`).

## Diagnostic UX Standards for Non-Technical Users
- Include plain-language explanations before technical detail.
- Suggest concrete edits with examples.
- Group repeated failures by type to avoid overwhelming output.
- Provide confidence hints when suggestions are heuristic.

## CLI UX Requirements
1. `validate` command optimized for human readability.
2. `validate --json` for machine workflows.
3. `explain <error-code>` command with examples.
4. `doctor` command for environment and setup checks.

## Editor/LSP UX Requirements
- In-editor diagnostics with consistent codes/severity.
- Hover help that mirrors CLI explain content.
- Quick-fix suggestions where transformations are safe.
- Stable behavior with partial or incomplete lines while typing.

## Accessibility and Internationalization
- Color output must have non-color equivalents.
- Error messages should avoid idioms and support localization later.
- Ensure Unicode and right-to-left text handling is tested.

## Safety Requirements
- Never rewrite in-place without explicit opt-in.
- Preserve original files in batch operations unless user requests replacement.
- Include dry-run mode for all mutating commands.

## Acceptance Criteria
- User can resolve common issues without reading source code.
- CLI and editor diagnostics are aligned in content and code semantics.
- Batch workflows provide clear summaries and durable output artifacts.
- Documentation includes a non-technical troubleshooting path.
