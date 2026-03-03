# TalkBank Development Workspace

Parent directory for all TalkBank repositories.

## Setup

Clone all repos:

```sh
mkdir ~/talkbank && cd ~/talkbank
curl -fsSL https://raw.githubusercontent.com/TalkBank/talkbank-chat/main/Makefile -o Makefile
make clone
```

## Repos

| Directory | GitHub | Description |
|-----------|--------|-------------|
| `tree-sitter-talkbank/` | [TalkBank/tree-sitter-talkbank](https://github.com/TalkBank/tree-sitter-talkbank) | Tree-sitter grammar for CHAT |
| `talkbank-chat/` | [TalkBank/talkbank-chat](https://github.com/TalkBank/talkbank-chat) | CHAT spec and core Rust libraries |
| `talkbank-chatter/` | [TalkBank/chatter](https://github.com/TalkBank/chatter) | CLI, LSP server, VS Code extension |
| `talkbank-clan/` | [TalkBank/clan](https://github.com/TalkBank/clan) | CLAN analysis library |
| `batchalign3/` | [TalkBank/batchalign3](https://github.com/TalkBank/batchalign3) | Alignment and transcription pipeline |
| `talkbank-private/` | [TalkBank/talkbank-private](https://github.com/TalkBank/talkbank-private) | Internal archive (private) |

## Commands

```sh
make status   # Git status across all repos
make check    # Cargo check all Rust repos
make test     # Run tests across repos
make pull     # Pull all repos
```
