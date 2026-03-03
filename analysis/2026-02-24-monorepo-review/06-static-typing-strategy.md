# 06. Static Typing Strategy

## Findings

- Rust codebase is heavily typed and generally strong.
- Python in `batchalign3` is typed but not yet strict enough for reliability-critical pathways.
- Mypy run passes, but output shows unchecked untyped function bodies.
- `Any` and `type: ignore` usage is concentrated in dispatch/server/cache/runtime integration code.

## Recommendations

1. Define typing tiers for Python:
   - Tier 1 (critical runtime): no `Any`, minimal ignores, strict mode
   - Tier 2 (integration): controlled `Any` with wrappers
   - Tier 3 (tests/prototyping): relaxed
2. Move from dict-shaped dynamic payloads to typed contracts:
   - `TypedDict` for intermediate payloads
   - `pydantic` models at API boundaries
3. Add mypy strict flags incrementally for critical modules:
   - `disallow_untyped_defs = True`
   - `check_untyped_defs = True`
   - `no_implicit_optional = True`
4. Replace stringly-typed command/task mappings with enums/newtypes where feasible.
5. In Rust, continue preferring explicit state enums over ambiguous `Option` in multi-state workflows.

## Suggested libraries/patterns

- Python typing: `typing_extensions` (`TypedDict`, `NotRequired`, `TypeGuard`)
- Validation and contracts: `pydantic v2` strict models
- Runtime config typing: `pydantic-settings` or strongly typed config wrapper

## Typing checklist

- [ ] Create module-level typing policy map (strict vs relaxed)
- [ ] Enable strict typing for `batchalign/serve/*` and `batchalign/cli/dispatch_*`
- [ ] Replace top `Any` hotspots with protocol or model types
- [ ] Track and burn down `type: ignore` count by category
- [ ] Add CI gate for new untyped defs in strict modules
