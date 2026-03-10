# Public Release Documentation Audit

This workspace tracks the public-release documentation audit for:

- `talkbank-tools`
- `batchalign3`

## Purpose

Use this directory for the private working materials that should not live in the
public repos:

- full artifact inventories
- claim/evidence matrices
- extracted provenance notes
- raw command/help snapshots
- release-blocker tracking during the audit

The public repos should contain only:

- cleaned user/developer/integrator docs
- stable migration guidance
- CI gates and tests that enforce doc accuracy

## Layout

- `inventories/` — generated TSV inventories of public-facing docs and workflows
- `matrices/` — human-maintained audit matrices and blocker tracking
- `evidence/` — raw captured command/help and contract outputs
- `scripts/` — helper scripts for inventory generation and CLI evidence capture
- `STATUS-2026-03-09.md` — current pause/resume note for the in-progress audit

## Workflow

1. Refresh the inventories with `scripts/build_doc_inventory.py`.
2. Capture fresh command/help outputs and generated contracts under `evidence/`.
   Use `scripts/capture_cli_surfaces.py` to snapshot the current public CLI
   surfaces for both repos into a date-stamped directory.
3. Update the matrices section by section as pages are reviewed.
4. When public docs are revised, preserve only extracted provenance or irrelevant
   historical detail here.

## Audit statuses

- `unreviewed` — no serious manual review yet
- `in_review` — page or artifact currently being checked
- `verified` — evidence-backed and safe to ship as written
- `revised` — corrected during this audit and rechecked
- `extracted` — private/historical detail moved out of public docs
- `blocked` — release blocker remains
