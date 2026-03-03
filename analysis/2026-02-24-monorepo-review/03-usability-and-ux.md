# 03. Usability and UX

## Findings

- Strong command surface exists (`chatter`, `clan`, `batchalign3`), but user pathways are fragmented across repos.
- Dashboard UX in `batchalign3/frontend` is functional and typed, but discoverability and operational affordances can improve.
- VS Code extension has rich command set but potentially high cognitive load without task-oriented grouping.

## Recommendations

1. Publish role-based quickstarts:
   - Researcher (run analysis/transcription)
   - Parser developer
   - Infra/operator
2. Create command bundles (`doctor`, `bootstrap`, `verify`) per repo and at workspace root.
3. Add guided error recovery in CLI output:
   - Show next command to run
   - Link to exact docs section
4. Enhance dashboard usability:
   - Persistent filter presets
   - Error triage workflows (group -> suggested fix -> retry)
   - One-click copy of diagnostic bundle ID
5. Add extension command palette categories with short descriptions and examples.

## Libraries/frameworks to leverage

- CLI UX: `clap` (already), add `clap_complete` for shell completions
- Rich terminal UX in Python: strengthen `rich` structured panels
- Web UX reliability: `react-query` (TanStack Query) for robust polling/cache invalidation

## Usability checklist

- [ ] Add a workspace-level `quickstart-by-role.md`
- [ ] Add `doctor` command in each executable surface
- [ ] Normalize CLI exit codes and error message format across repos
- [ ] Add dashboard saved views and deep links for filtered error states
- [ ] Add extension onboarding walkthrough and command taxonomy
