# 04. Developer Experience

## Findings

- Tooling quality varies by repo.
- `talkbank-chat` has strong make targets and disciplined workflows.
- `talkbank-chatter` and `talkbank-clan` need equivalent developer guardrails.
- `batchalign3` uses `uv` and mypy, but mypy config is permissive (`ignore_missing_imports = True`, untyped function bodies not checked).

## Recommendations

1. Define a uniform local developer contract for all repos:
   - `make check`
   - `make test`
   - `make verify`
2. Add pre-commit hooks for formatting/linting/type-checking per language.
3. Add generated file protections to prevent manual edits in generated artifacts.
4. Document expected runtimes for major checks to improve workflow planning.
5. Add a root `tools/` folder with reusable scripts and consistent output formatting.

## Framework/library leverage

- Pre-commit orchestration: `pre-commit`
- Rust multi-repo consistency: `cargo xtask` pattern for custom tasks
- Python env consistency: keep `uv`, add lockfile verification gate

## DX checklist

- [ ] Implement cross-repo command parity (`check`, `test`, `verify`)
- [ ] Add pre-commit hooks across all repos
- [ ] Add machine-readable check summary output (JSON)
- [ ] Publish troubleshooting docs for common local setup failures
- [ ] Add "new contributor in 30 minutes" path
