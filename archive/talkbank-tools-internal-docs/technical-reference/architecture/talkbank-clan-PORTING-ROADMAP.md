# CLAN Porting Roadmap

## Phase 1: Framework Improvements (benefits all 13 existing commands) -- DONE

### 1.1 Wire up WordFilter in FilterConfig::matches()
### 1.2 Wire up GemFilter CLI args (`--gem`, `--exclude-gem`)
### 1.3 Add `--exclude-speaker` CLI arg
### 1.4 Add `--include-word` / `--exclude-word` CLI args
### 1.5 Add `--per-file` output mode
### 1.6 Add `--range` (utterance range limiting)
### 1.7 Add `--include-retracings` flag

**Files touched:** `filter.rs`, `runner.rs`, `word_filter.rs`, `analyze.rs`, `args.rs`

---

## Phase 2: Remaining Analysis Commands -- 4 DONE, 4 DEFERRED

### 2.1 GEMFREQ -- DONE (covered by FREQ + --gem filter)
No separate command needed. `chatter analyze freq file.cha --gem Story` does it.

### 2.2 PHONFREQ -- DONE
Counts individual phone (character) occurrences from %pho tier with positional
tracking (initial/final/other). Golden-tested against CLAN binary.

### 2.3 VOCD -- DONE
Vocabulary diversity (D statistic) via bootstrap sampling + curve fitting.
Uses `rand` crate. Golden-tested (stochastic, so snapshot-only comparison).

### 2.5 MODREP -- DONE
Model/replica comparison from %mod and %pho tiers. Golden-tested against CLAN binary.

### 2.4 CHAINS -- DEFERRED
Requires `$`-prefixed code markers on dependent tiers. No reference corpus files have
these markers, so there's no practical way to golden-test. Would need:
- Test data with code tiers
- `+c` clause marker specification support
- Two-pass processing (scan codes, then compute chain statistics)

### 2.6 RELY -- DEFERRED
Requires two-file input (comparing two coders' versions). Needs framework extension
for two-file comparison mode (different from `AnalysisRunner`'s single-stream pattern).

### 2.7 KEYMAP -- DEFERRED
Requires specialized code tier configuration (`+t%` option). Needs test data with
code tiers and keymap configuration files.

### 2.8 SCRIPT -- DEFERRED
Requires speaker tier specification (`+t` option). Needs test data with specific
speaker configurations.

---

## Current Command Inventory (16 analysis + 8 transform)

### Analysis Commands

| # | Command | Module | Status | Golden-tested? |
|---|---------|--------|--------|----------------|
| 1 | FREQ | `commands/freq.rs` | Done | Yes (3 variants) |
| 2 | MLU | `commands/mlu.rs` | Done | Yes + CLAN-compat |
| 3 | MLT | `commands/mlt.rs` | Done | Yes + CLAN-compat |
| 4 | WDLEN | `commands/wdlen.rs` | Done | Yes |
| 5 | MAXWD | `commands/maxwd.rs` | Done | Yes |
| 6 | FREQPOS | `commands/freqpos.rs` | Done | Yes |
| 7 | TIMEDUR | `commands/timedur.rs` | Done | No CLAN binary |
| 8 | KWAL | `commands/kwal.rs` | Done | Yes |
| 9 | COMBO | `commands/combo.rs` | Done | No CLAN binary |
| 10 | GEMLIST | `commands/gemlist.rs` | Done | Yes |
| 11 | COOCCUR | `commands/cooccur.rs` | Done | Yes |
| 12 | DIST | `commands/dist.rs` | Done | Yes |
| 13 | CHIP | `commands/chip.rs` | Done | Yes |
| 14 | PHONFREQ | `commands/phonfreq.rs` | Done | Yes |
| 15 | VOCD | `commands/vocd.rs` | Done | Yes (stochastic) |
| 16 | MODREP | `commands/modrep.rs` | Done | Yes |

### Transform Commands

| # | Command | Module | Status | Golden-tested? |
|---|---------|--------|--------|----------------|
| 1 | FLO | `transforms/flo.rs` | Done | Yes |
| 2 | LOWCASE | `transforms/lowcase.rs` | Done | Yes |
| 3 | CHSTRING | `transforms/chstring.rs` | Done | Yes |
| 4 | DATES | `transforms/dates.rs` | Done | Yes |
| 5 | RETRACE | `transforms/retrace.rs` | Done | Yes |
| 6 | DELIM | `transforms/delim.rs` | Done | Yes |
| 7 | FIXBULLETS | `transforms/fixbullets.rs` | Done | Yes |
| 8 | REPEAT | `transforms/repeat.rs` | Done | Yes |

---

## Phase 3: Reformatting/Repair Commands

### 3.0 TransformCommand framework -- DONE
- `TransformCommand` trait in `framework/transform.rs`
- `run_transform()` pipeline: parse -> transform -> serialize -> write
- CLI: `chatter transform <command> input.cha [-o output.cha]`

### 3.1 FLO -- DONE
Simplified fluent output. Strips headers, adds `%flo:` tier with cleaned main
line (retrace targets, pauses, events, fragments, zero words, unintelligible removed).
Keeps all dependent tiers (%mor, %gra, etc.). Golden-tested against `gra.cha`
and `mor-ignore.cha`.

### 3.2 LOWCASE -- DONE
Lowercase all main tier words. Preserves speaker codes, headers, dependent tiers.
Golden-tested against `gem.cha` (42 changes).

### 3.3 CHSTRING -- DONE
String replacement using a changes file (line-pair format: find/replace).
Config: `--changes <path>`. Golden-tested.

### 3.4 DATES -- DONE
Age computation from @Birth and @Date headers. Inserts/updates @Age headers.
Also supports calculator mode (`--birth DATE --date DATE`).

### 3.5 DELIM -- DONE
Add missing terminators (default: period) to main tiers that lack them.
Golden-tested against `ca-missing-terminator.cha`.

### 3.6 FIXBULLETS -- DONE
Fix timing bullet consistency (reorder overlapping/reversed bullets).
Golden-tested against `bullet-dependent.cha`.

### 3.7 FIXIT -- DEFERRED
Reorders dependent tiers (%gra before %mor) and wraps long lines. Our `chatter
normalize` already handles canonical formatting. CLAN's FIXIT behavior (putting
%gra before %mor) is actually non-standard -- the CHAT spec has %mor before %gra.

### 3.8 INDENT -- DEFERRED
Overlap marker alignment with leading spaces. Very niche (only 1 reference file
affected). Already covered by `chatter normalize`.

### 3.9 LONGTIER -- DEFERRED
Join continuation lines into single lines. Already handled at the parser level --
continuation lines are a display concern, and `chatter normalize` already produces
single-line output.

### 3.10 RETRACE -- DONE
Adds `%ret:` dependent tier to each utterance with a verbatim copy of the main
tier content. Preserves all headers and existing dependent tiers.
Golden-tested against `mor-ignore.cha`.

### 3.11 REPEAT -- DONE
Marks utterances containing repetitions/revisions with `[+ rep]` postcodes.
Requires target speaker (`--speaker`). Golden-tested against `mor-ignore.cha`.

---

## Phase 4: Format Converters

Each format needs a parser (for X->CHAT) and/or serializer (for CHAT->X).

### 4.1 SRT <-> CHAT (simplest format)
- SRT: numbered entries with timestamps and text
- CHAT->SRT: extract utterances with timing bullets, format as SRT entries
- SRT->CHAT: parse SRT entries, create utterances with timing bullets
- Crate: `srt_subtitles_parser` or custom (format is trivial)

### 4.2 Praat TextGrid <-> CHAT
- TextGrid: interval tiers with time boundaries and labels
- CHAT->Praat: map speakers to tiers, utterances to intervals with timestamps
- Praat->CHAT: map intervals back to timed utterances
- Crate: `textgrid` crate available on crates.io

### 4.3 ELAN XML <-> CHAT
- ELAN: complex XML with controlled vocabulary, linguistic types, tiers
- CHAT->ELAN: map speakers to tiers, utterances to annotations, dependent tiers to child tiers
- ELAN->CHAT: map annotations to utterances, tier hierarchy to dependent tiers
- Crate: `quick-xml` for parsing/writing

### 4.4 TEXT -> CHAT
- Plain text with speaker labels -> minimal CHAT with @Begin/@End, @Participants
- One of the simplest converters

### 4.5 SALT -> CHAT
- SALT format (used in clinical SLP): similar to CHAT but different conventions
- Requires understanding SALT tier structure

### 4.6 LENA -> CHAT
- LENA CSV output -> CHAT with timing and speaker information

**Architecture:** New module in `talkbank-transform` or dedicated crates per format.
CLI: `chatter to-elan`, `chatter from-elan`, `chatter to-praat`, `chatter from-praat`, `chatter to-srt`, `chatter from-srt`, etc.

---

## Implementation Order

1. **Phase 1** (framework) -- DONE
2. **Phase 2** (analysis commands) -- DONE (4 implemented, 4 deferred for lack of test data)
3. **Phase 3** (transform commands) -- DONE (8 implemented, 3 deferred as covered by normalize/parser)
4. **Phase 4** -- SRT (simplest), Praat, ELAN, TEXT, SALT, LENA

## Dependencies to Add
- `rand` (for VOCD bootstrap sampling) -- ADDED
- `smallvec` (for transform commands) -- ADDED
- `tempfile` (for transform golden tests) -- ADDED
- `quick-xml` (for ELAN)
- `textgrid` (for Praat, if quality is good; else custom parser)
- `srt_subtitles_parser` or custom (for SRT)
