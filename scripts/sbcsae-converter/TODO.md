# SBCSAE Converter — TODO

**Last updated:** 2026-03-19

## Hardcoded values to extract into configuration

All of these are SBCSAE-specific and should be configurable:

- `"eng"` — language in CHAT headers (`emit_chat.rs`)
- `"SBCSAE"` — corpus name in @ID lines (`emit_chat.rs`)
- `"audio"` — media type in @Media header (`emit_chat.rs`)
- `"SBC"` — filename prefix stripped for CHAT file numbering (`main.rs`)
- Speaker special-case table: `SHANE`→`SHAN`, `SHARON`→`SHA`, etc. (`speakers.rs`)
- `MAX_WHO = 4` — speaker ID truncation length (`speakers.rs`)
- `max_column_delta = 2` — alignment edge column tolerance (`intermediate.rs`)
- `max_line_distance = 5` — alignment edge max line separation (`intermediate.rs`)

These should become a `ConverterConfig` struct, loadable from a TOML/JSON file
or overridable via CLI flags. This would allow the tool to process other TRN
corpora beyond SBCSAE.

## Remaining validation errors (non-overlap)

- E370 (23): Multi-line long feature labels spanning TRN line boundaries
- E243 (9): Residual control characters in TRN source
- E220 (9): Words with digits (transcriber conventions like `r0h`)
- E259 (5): Comma placement after happenings
- Various (20): Single-digit counts

## Architecture

- Remove old `--chat` pipeline once `--doc-chat` is verified equivalent
- Consolidate `OverlapRole` type (currently in both `types.rs` and `intermediate.rs`)
