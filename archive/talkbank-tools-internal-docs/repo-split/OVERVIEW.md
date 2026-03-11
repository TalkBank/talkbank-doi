# Repository Split: Overview

## Target Structure

```
talkbank/tree-sitter-talkbank   Grammar + Spec (the language definition)
talkbank/talkbank               Core Rust crates (the implementation)
talkbank/chatter                CLI + LSP + VS Code (the tools)
talkbank/clan                   CLAN analysis commands (the analysis toolkit)
```

Plus: `batchalign-core` moves to the existing `batchalign` repo.

## Migration Order

1. **Phase 1**: Extract grammar + spec -> `tree-sitter-talkbank`
2. **Phase 2**: Publish core Rust crates to crates.io
3. **Phase 3**: Move batchalign-core -> batchalign repo
4. **Phase 4**: Extract clan -> `talkbank/clan`
5. **Phase 5**: Extract tools -> `talkbank/chatter`
6. **Phase 6**: Rename monorepo -> `talkbank/talkbank`

## Manifests

See sibling files for per-repo details:
- `tree-sitter-talkbank.md` - Grammar + Spec repo
- `talkbank-core.md` - Core Rust library crates
- `chatter.md` - CLI + LSP + VS Code
- `clan.md` - CLAN analysis toolkit
