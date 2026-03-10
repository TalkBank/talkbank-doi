# Repository Guidelines

## Project Structure & Module Organization
- `batchalign/` is the core Python package (CLI, pipelines, formats, models, utils).
- `batchalign/tests/` holds pytest-based tests; keep new tests near the module they cover.
- `docs/` contains user documentation and guides (MkDocs).
- Top-level helpers include `execute.py`, `serve.py`, and packaging metadata like `setup.py`.

## Build, Test, and Development Commands
- `pip install -e ".[dev]"` installs the package in editable mode with test deps.
- `pytest` runs the full test suite.
- `pytest batchalign/tests/pipelines/test_pipeline.py::test_standard_pipeline -v` runs a focused test.
- `batchalign --help` shows CLI usage after installation.
- `mkdocs serve` (with `pip install -e ".[docs]"`) runs the documentation site locally.

## Coding Style & Naming Conventions
- Python-only repo; follow PEP 8 with 4-space indentation.
- Use descriptive module names under `batchalign/` (e.g., `pipelines/`, `formats/`, `cli/`).
- Prefer snake_case for functions/variables, PascalCase for classes, and ALL_CAPS for constants.
- No enforced formatter/linter in repo; keep diffs tidy and consistent with nearby code.

## Testing Guidelines
- Test framework: `pytest` (see `pytest.ini` for ignores).
- Name tests as `test_*.py` and functions as `test_*`.
- Add regression tests alongside new pipeline/format behavior when practical.

## Commit & Pull Request Guidelines
- Commit messages are short and direct; repo history includes merge commits and Dependabot bumps
  (e.g., `Bump urllib3 from 2.6.0 to 2.6.3 in the pip group across 1 directory`).
- PRs should explain intent, include reproduction steps or sample inputs, and link relevant issues.
- Add CLI output snippets or before/after notes when changing pipelines or formats.

## Configuration & Secrets
- Some features require external credentials (e.g., Rev.AI); keep keys out of git.
- If a feature needs a token file (e.g., `rev_key`), document it in the PR description.

## Agent Notes
- For deeper architecture context and dev commands, see `CLAUDE.md`.
