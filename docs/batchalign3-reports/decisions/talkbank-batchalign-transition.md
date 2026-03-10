# TalkBank Batchalign: Current Situation and Path Forward

**Date:** 2026-02-17

---

## Background

Batchalign is TalkBank's tool for processing CHAT-format language transcripts.
It handles forced alignment, ASR transcription, morphosyntactic analysis, and
translation — core operations used across every TalkBank collection.

There are now two implementations:

- **batchalign** (Python, master branch) — the current production system,
  installed on lab machines and used by researchers
  across multiple institutions.

- **batchalign3** (Rust-first + Python integration, batchalign3) — a modernized
  implementation developed over the past several months. Uses a Rust-based CHAT
  parser for correctness guarantees, adds client/server architecture, and fixes
  long-standing data quality issues.

Both implementations share the same Python codebase for ML model integration
(Stanza, Whisper, Wave2Vec). They differ in how they parse, validate, and
serialize CHAT files, how they dispatch work, and how they handle errors.

---

## Scope of Changes

A common misconception is that batchalign3 is an incremental modification
of batchalign — a set of patches bolted onto the existing codebase. In fact,
batchalign3 is a ground-up rewrite of the core processing logic, with the
original code surviving only in peripheral areas (ML model wrappers and static
data tables).

### Code audit

| | Master (batchalign) | Align (batchalign3) |
|---|---|---|
| Python files | 137 | 189 |
| Python source lines | 21,116 | 25,504 |
| Python test lines | 1,978 | 10,924 |
| **Python total** | **23,094** | **36,428** |
| Rust lines (new) | — | 107,000 |
| **Total lines** | **23,094** | **143,000+** |

### What was deleted

The entire CHAT processing stack was removed from Python and replaced by Rust:

| Deleted file | Lines | What it did |
|---|---|---|
| `formats/chat/parser.py` | 357 | CHAT file parsing |
| `formats/chat/lexer.py` | 219 | Token classification |
| `formats/chat/generator.py` | 182 | CHAT file serialization |
| `formats/chat/file.py` | 149 | File I/O |
| `formats/chat/utils.py` | 156 | Parse utilities |
| `pipelines/morphosyntax/ud.py` | 1,224 | UD→CHAT morphosyntax mapping |
| `document.py` | 451 | Core data model |
| `utils/dp.py` | 224 | Dynamic programming aligner |
| **Total deleted** | **2,962** | |

These files contained the bugs that cannot be backported: the `decode()` type
override, the `actual_indicies` wraparound, the `str.strip()` corruption. They
no longer exist.

### What was rewritten

Several files kept their names but were substantially rewritten:

| File | Master | Align | Change |
|---|---|---|---|
| `cli/dispatch.py` | 1,044 | 119 | Replaced by 4 new dispatch modules (2,064 lines) |
| `pipelines/morphosyntax/ud.py` | 1,224 | *(deleted)* | Replaced by Rust `mapping.rs` + `engine.py` (329 lines) |
| `pipelines/fa/whisper_fa.py` | 238 | 91 | Core logic moved to Rust `forced_alignment.rs` |
| `pipelines/fa/wave2vec_fa.py` | 206 | 103 | Same |
| `pipelines/utterance/ud_utterance.py` | 386 | 143 | Core logic moved to Rust callbacks |
| `pipelines/cache.py` | 748 | 440 | Redesigned for Rust integration |

### What was added

77 new Python files (18,883 lines) that have no counterpart on master:

- **Server** (`serve/`): 3,144 lines — HTTP job server, SQLite persistence,
  media resolution, dashboard, WebSocket updates
- **Dispatch** (`cli/`): 2,064 lines — 4 dispatch modules replacing the
  monolithic 1,044-line `dispatch.py`
- **Daemon** (`cli/daemon.py`): 327 lines — persistent local server management
- **Callbacks**: 1,024 lines — Rust↔Python bridge for morphosyntax, alignment,
  translation, utterance segmentation
- **Tests**: 9,346 lines — 625 tests vs master's 1,978 test lines

### What survives unchanged

60 files (10,732 lines) are identical on both branches. Of these:

| Category | Lines | Examples |
|---|---|---|
| Static data tables | ~9,000 | `names.py` (6,684), `abbrev.py` (425), `num2lang/*.py` (1,295), `en/irr.py` (215), `ja/verbforms.py` (147) |
| ML model wrappers | ~1,200 | `audio_io.py`, `speaker/infer.py`, `utterance/infer.py` |
| `__init__.py` boilerplate | ~500 | Package markers |

The surviving code is overwhelmingly static data (name dictionaries, number
word tables, irregular verb lists) and thin wrappers around third-party ML
libraries (Stanza, Whisper, Wave2Vec). These are areas where batchalign simply
calls an external library and passes through results — there is no batchalign
logic to change.

### The Rust codebase

The 107,000 lines of Rust have no predecessor in master. They were written from
scratch:

| Crate | Lines | Purpose |
|---|---|---|
| `talkbank-model` | 36,879 | CHAT data model, NLP mapping, %wor/%gra rules |
| `talkbank-parser-tests` | 25,368 | Parser integration tests |
| `talkbank-tree-sitter-parser` | 20,721 | Tree-sitter CHAT parser |
| `batchalign-core` | 12,609 | PyO3 bridge (Python↔Rust interface) |
| `talkbank-direct-parser` | 8,690 | Alternative Chumsky parser |
| `grammar.js` | 2,426 | Tree-sitter grammar definition |

### Implications for maintenance

When a bug is found in CHAT processing — for example, a morphosyntax mapping
issue like separator character handling — the fix path is:

| | Batchalign (master) | Batchalign3 (align) |
|---|---|---|
| **Where the bug lives** | `ud.py` (1,224 lines of Python string manipulation) | `mapping.rs` in `talkbank-model` (Rust, typed AST) |
| **How to fix it** | Edit regex/string logic in `ud.py`, hope it doesn't break other cases | Change the relevant mapping rule, compiler verifies type safety |
| **Who needs to be involved** | Maintainer of the Python code | Maintainer of the Rust code |
| **Testing** | 0 tests for `ud.py` on master | 54 mapping tests + 6 validation tests + 763 parser tests |

The two codebases share almost no logic. A fix in one does not translate to the
other. They are independent implementations of the same specification, with
different architectures, different languages, and different maintainers.

---

## The Situation

### Data quality issues in master

Corpus-wide validation of 99,208 CHAT files has revealed data quality problems
in batchalign's output:

- **%wor tier corruption**: 3,735 files (3.8%) have incorrect word-level timing
  tiers containing 22,908 individual errors. The root cause is a token
  classification bug in the Python lexer that leaks nonwords and untranscribed
  material into the timing tier. Affected files span 12 TalkBank collections
  and 396 corpus directories.

- **%gra circular dependencies**: In tested output, 87.5% of morphotag
  utterances contain circular head references in the dependency tier. The root
  cause is an array-indexing bug that wraps around to negative indices when
  token counts don't match.

- **Crash on multilingual files**: Files with non-English secondary languages
  crash the batch. Two independent bugs: one in utterance segmentation (Stanza
  language support check missing) and one in morphosyntax (unsupported language
  in `@Languages` header).

- **Silent data corruption in caching**: Forced alignment cache keys used
  Python object identity (`id()`) — a non-deterministic value that can return
  stale results for different audio. The CHAT parser's `str.strip()` silently
  corrupted language codes.

These bugs are documented with corpus-wide evidence in `docs/wor-tier-bug-report.md`,
`docs/gra-correctness-guarantee.md`, and `docs/correctness-assessment.md`.

### What batchalign3 fixes

The Rust CHAT parser operates on a typed AST rather than a flat token stream.
This eliminates the class of bugs that come from ad-hoc string processing:

- %wor tier extraction uses per-word alignability checks on the AST — group
  membership cannot override a word's type
- %gra generation uses HashMap-based index mapping with pre-generation
  validation — circular dependencies cannot be produced
- CHAT serialization goes through a principled AST-to-text path — no regex
  or line-by-line text hacking

Beyond correctness, batchalign3 adds:

- Client/server architecture (CHAT sent over HTTP, media resolved from server
  mounts — no need to copy 50,000 audio files to each workstation)
- Lenient parsing with error recovery (one malformed utterance doesn't kill the
  whole file)
- Structured error reporting (error codes, line numbers, suggestions propagated
  to CLI and dashboard)
- MWT expansion for morphosyntax (contractions properly split)
- Per-utterance language routing for multilingual files
- 2-7x faster processing, 30% less memory

### What cannot be backported

Of the ~55 bug fixes on the batchalign3, only 13 can be backported to master
as pure-Python patches. The remaining fixes require either the Rust AST (for
correct CHAT handling) or the new dispatch architecture (for server mode).

The most impactful bugs — %wor corruption and %gra circular dependencies —
cannot be fixed in master without rewriting the lexer and morphosyntax mapping
pipeline. See `docs/bug-fixes-and-backport-assessment.md` for the full analysis.

---

## The Dilemma

### Technical case

The technical case for replacing batchalign with batchalign3 is
straightforward: the new implementation produces correct output where the old
one produces corrupt output, and the most serious correctness bugs cannot be
fixed in the old architecture. Continuing to run master means continuing to
produce files with known %wor and %gra errors in a corpus used for published
research.

### Organizational complexity

The transition is not purely technical:

1. **Ownership and maintenance.** Batchalign has a legacy ownership model. A
   transition to batchalign3 changes the maintenance model — the new
   implementation has different architecture, a Rust dependency, and different
   deployment procedures. This has implications for who maintains what.

2. **Deployment scope.** Batchalign is installed on every lab machine and used
   by researchers who may have workflows built around its current behavior.
   Replacing it requires coordination — even if the new tool is better, the
   transition itself has a cost.

3. **Risk tolerance.** Master is known: its bugs are documented and its
   behavior is familiar. Batchalign3 has been validated on real corpus data
   (625 Python tests, 763 Rust parser tests, overnight runs on multiple
   collections), but it has not yet seen production-scale use across all
   TalkBank collections.

4. **Incremental vs. wholesale.** There are intermediate options between "keep
   master forever" and "replace immediately" (see below).

---

## Options

### Option A: Replace master with batchalign3

Install batchalign3 as the production `batchalign` command on all machines.
Retire master.

**Pros:**
- All correctness fixes take effect immediately
- One codebase to maintain
- Server mode, dashboard, structured errors available to all users
- No ongoing effort spent maintaining a known-buggy system

**Cons:**
- Requires coordinated deployment across all machines
- Researchers' existing scripts may need minor updates
- Rust build dependency for the CHAT parser
- No fallback if an undiscovered issue arises in production

### Option B: Parallel deployment

Install batchalign3 alongside master as a separate command (its current
state — `batchalign3` vs `batchalign`). Let researchers choose which to
use. Gradually shift workloads as confidence grows.

**Pros:**
- Zero risk to existing workflows
- Researchers can compare output side-by-side
- Provides a fallback during transition

**Cons:**
- Two systems to maintain indefinitely (or until one is retired)
- Confusion about which tool to use for which task
- Master continues to produce known-incorrect output for users who don't switch
- Bug fixes and features must be coordinated across two codebases (or not,
  causing divergence)

### Option C: Backport critical fixes to master

Cherry-pick the 13 backportable Python fixes into master. Continue running
master as the production tool. Use batchalign3 for server mode and
new features only.

**Pros:**
- Minimal disruption to existing workflows
- Addresses the most accessible bugs (crashes, cache issues, some CLAN
  compatibility)

**Cons:**
- The two most damaging bugs (%wor corruption and %gra circular dependencies)
  cannot be fixed this way
- Master continues to produce known-incorrect %wor and %gra output
- Ongoing maintenance cost for a codebase with fundamental architectural issues
- Does not address the feature gap (no server mode, no lenient parsing, no
  structured errors, no MWT expansion)

### Option D: Freeze master, deploy batchalign3 for new work

Stop developing master but keep it installed for backward compatibility.
All new corpus processing uses batchalign3. Existing processed data is
not reprocessed.

**Pros:**
- No disruption to existing data
- New processing benefits from correctness fixes
- No ongoing development cost for master
- Clear transition path without forced migration

**Cons:**
- Existing corpus data retains known errors
- Two tools installed on every machine (but only one actively used)
- May cause confusion during the transition period

---

## Relevant Data

| Metric | Value |
|--------|-------|
| Files with known %wor errors | 3,735 (3.8% of corpus) |
| Individual %wor errors | 22,908 |
| %gra circular dependency rate (tested) | 87.5% of morphotag output |
| TalkBank collections affected by %wor bug | 12 of 12 tested |
| Bug fixes that can be backported to master | 13 of ~55 |
| Bug fixes that require Rust architecture | 12 |
| Python test suite (batchalign3) | 625 tests passing |
| Rust parser test suite | 763 tests passing, 0 failing |
| Performance improvement | 2-7x faster, 30% less memory |

---

## References

- `docs/correctness-assessment.md` — full inventory of bugs and fix status
- `docs/bug-fixes-and-backport-assessment.md` — backportability analysis
- `docs/wor-tier-bug-report.md` — corpus-wide %wor audit with per-collection data
- `docs/gra-correctness-guarantee.md` — %gra validation and Python comparison
- `docs/comparison-report.md` — side-by-side code quality grades
- `docs/rust-migration-proposal.md` — long-term architecture direction
