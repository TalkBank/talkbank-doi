# Replacing the CLAN Mac App with VS Code

**Status report and completion plan — March 2026**

## Executive Summary

The VS Code extension already replaces the large majority of the CLAN macOS
application. All 30 analysis commands run inside the extension, media playback
and waveform visualization work, transcription mode works, and the editor
experience is significantly better than CLAN's built-in editor.

What remains is mostly specialized GUI dialogs — clinical profiling commands
(KidEval, Eval, Eval-D) with normative database comparison, an @ID metadata
editor, and Coder mode. These are well-scoped features that follow proven
patterns already established in the extension. The plan below breaks the
remaining work into six phases.

---

## Current Status: What Works

### Editor (replaces DocumentWinController — 9,673 lines of Obj-C++)

VS Code is a far better text editor than CLAN's. Our extension adds:

- Real-time validation with diagnostics (squiggles, Problems panel)
- Semantic syntax highlighting via tree-sitter
- Code completions (speaker codes, tier prefixes, postcodes)
- Quick-fix code actions for common errors
- Go-to-definition (speaker code to @Participants, dependent tier to main tier)
- Document formatting via canonical CHAT serializer
- Code folding for utterance blocks
- Document outline / breadcrumb navigation

### Analysis Commands (replaces CommandsController — 993 lines)

All 30 CLAN analysis commands wired end-to-end: user picks a command from a
menu, it runs in the Rust LSP backend, results appear in a styled webview panel
with tables, stat cards, and charts. Commands needing input (KWAL, COMBO,
MORTABLE, etc.) prompt via VS Code's native dialogs.

### Media and Transcription

- Play at cursor, continuous play, walker mode (Alt+Down/Up)
- Rewind (F8), loop (F5), audio and video
- Waveform visualization with clickable segment overlays
- Transcription mode with F4 bullet stamping

### Other Completed Features

| Feature | CLAN Equivalent | Status |
|---------|----------------|--------|
| Validation explorer (corpus-scale) | File Checker | Done (better: works on directories) |
| Dependency graph (Cmd+Shift+G) | Dep Graph window | Done |
| Hover info (main, %mor, %gra, %pho, %sin) | Tier Window pane | Done |
| Cross-tier alignment highlighting | Click-to-highlight | Done (bidirectional) |
| Alignment mismatch inlay hints | Error log only | Done (no CLAN equivalent) |
| Speaker filtering | N/A | Done (no CLAN equivalent) |

---

## What Remains

### Gap 1: Normative Database Comparison (KidEval, Eval, Eval-D)

This is the biggest functional gap and directly addresses Brian's request.

**Current state:** The KidEval and Eval commands run and produce per-speaker
scores (MLU, NDW, TTR, DSS, VOCD, IPSyn, POS counts, etc.), but they do **not**
compare against normative databases. The Rust backends compute the measures but
have no database loading or comparison logic. Eval-D does not exist yet.

**What CLAN does:** Each command has a bespoke dialog where the user selects
a comparison database by specifying language, corpus type (Toyplay/Narrative),
age range, and gender. CLAN then loads the matching `.cut` database file
(~52,000 lines across 18 files for KidEval; ~68,000 lines across 4 files for
Eval/Eval-D), filters entries by the selected criteria, computes mean and SD
for each measure, and reports the child's score as a standard deviation from
the norm.

**Database format** (same for all three commands):
```
=Eng-NA/Bates/Free20/keith.cha          ← source file path
+eng|Bates|CHI|1;08.|male|TD|MC|Target_Child|||  ← @ID metadata
81 84 55 53 73 76 50 94 38 38 94 20.23 15 89 57 ... ← space-separated scores
-                                        ← end of entry
```

The scores are positional (column order matches the command's output fields).
Comparison is: compute mean and SD of the matched DB entries for each measure,
then report `(child_score - mean) / SD` as standard deviations from the norm.

### Gap 2: @ID / Participant Editor

CLAN has a structured form (IdsController.mm — 1,895 lines) for editing @ID
headers: language, corpus, speaker code, age (y;m.d), sex, group, SES, role,
education, custom field. Dropdowns for role and SES are populated from a depfile
(`lib/depfile.cut`).

Our Rust model already has full typed representations (`IDHeader`, `Participant`,
`SesValue`, `ParticipantRole`) in `talkbank-model`. The extension currently
relies on users editing @ID lines as raw text.

### Gap 3: Coder Mode

CLAN's Coder mode (ced_codes.cpp — 2,032 lines) loads a hierarchical codes
file (`.cut` format, whitespace-indented tree), then lets the user step through
utterances selecting codes from the tree. Used in research workflows where
trained coders annotate transcripts with a predefined coding scheme.

### ~~Gap 3: Coder Mode~~ ✅ DONE (2026-03-06)

Implemented as `talkbank.startCoder` in `coderPanel.ts`. Loads `.cut` codes files,
steps through uncoded utterances, inserts `%cod:` tiers via hierarchical QuickPick.
Keybindings: `Cmd+Enter` (next), `Cmd+Shift+C` (insert code).

### Gap 4: Minor Features

| Feature | Priority | Notes |
|---------|----------|-------|
| ~~CSV/Excel export from analysis panels~~ | ~~Medium~~ | ✅ DONE |
| ~~Corpus-level analysis (run on directory)~~ | ~~Medium~~ | ✅ DONE |
| ~~Waveform zoom/scroll~~ | ~~Low~~ | ✅ DONE |
| Audio anonymization | Low | Niche use case |
| ~~Foot-pedal configuration~~ | ~~Low~~ | ✅ DONE (keybindings editor) |
| ~~Picture display~~ | ~~Low~~ | ✅ DONE (`picturePanel.ts`) |

---

## Implementation Plan

### Phase 1: Database Infrastructure (Rust)

**Goal:** Build the shared foundation that KidEval, Eval, and Eval-D all need.

**Repo:** `talkbank-clan`

**Tasks:**

1. **Database parser** — Parse the `.cut` database format into typed structs.
   Each entry has: file path, @ID metadata (language, corpus, speaker code, age,
   sex, group, SES, role), and a vector of f64 scores. The format is simple and
   regular. ~150 lines.

2. **Database filter** — Given user criteria (language, corpus types, age range,
   gender), filter database entries to matching subset. ~80 lines.

3. **Comparison statistics** — For each measure: compute mean and SD across
   matched entries, then report `(score - mean) / SD` as standard deviations.
   Replicate CLAN's `compute_SD()` and percentile logic. ~100 lines.

4. **Database discovery** — Given a lib directory path, enumerate available
   databases and report which language/corpus combinations exist. This drives the
   UI dynamically so we don't hardcode database lists. ~50 lines.

5. **Comparison result types** — Extend the existing `SpeakerKideval` (and
   future `SpeakerEval`, `SpeakerEvald`) structs to include optional comparison
   fields: `db_mean`, `db_sd`, `sd_score` (standard deviations from norm),
   `db_n` (number of matched entries).

6. **Wire into KidEval command** — Extend `KidevalConfig` with database options
   (path, language, corpus types, age range, gender). When a database path is
   provided, load it, filter, compute stats, and populate comparison fields.

**Estimated new code:** ~500 lines Rust in `talkbank-clan`.

**Bundle databases:** Ship the `.cut` files from `OSX-CLAN/src/lib/kideval/` and
`OSX-CLAN/src/lib/eval/` alongside the LSP binary. The LSP needs to know the lib
directory path (configurable via `talkbank.libPath` setting, with a sensible
default).

### Phase 2: KidEval GUI (VS Code)

**Goal:** A dedicated webview panel for KidEval with database comparison — the
feature Brian specifically requested.

**Repo:** `talkbank-chatter`

**Tasks:**

1. **New LSP request: `talkbank/kideval-databases`** — Returns the list of
   available databases (language + corpus type pairs) by scanning the lib
   directory. The webview calls this on open to populate dropdowns dynamically.

2. **Extend `talkbank/analyze` for kideval** — Pass database options (language,
   corpus types, age range, gender, speaker codes) through the existing LSP
   command dispatch. Update `analysis.rs` to populate `KidevalConfig` with these
   options.

3. **KidEval webview panel** — Three new files following the existing pattern:

   `kidevalPanel.ts` (~200 lines): Singleton panel with bidirectional message
   passing. On open, sends `talkbank/kideval-databases` request to LSP, injects
   database list into webview. On "Run", collects form state, sends
   `talkbank/analyze` with options, receives results, posts them to webview.

   `kidevalPanel.html` (~250 lines): Two-phase UI:
   - **Configuration form:** Radio (analyze only / compare with DB), language
     dropdown, corpus type checkboxes (dynamically filtered by language), age
     range slider or checkboxes (1;6 through 5;6), gender radio, speaker code
     fields (up to 8), Run button.
   - **Results display:** Per-speaker sections with score cards showing the
     child's score, database mean, SD, and standard deviations from norm.
     Color-coded (green/yellow/red) for clinical interpretation. Table with all
     measures for detailed view.

   `kidevalPanel.js` (~300 lines): Form logic (dynamic filtering when language
   changes), message handling, results rendering with comparison visualization.

4. **Command registration** — Add `talkbank.runKideval` command in
   `extension.ts`. Wire to right-click context menu and command palette.
   Keep the existing generic `kideval` entry in `runAnalysis` for users who
   want quick results without database comparison.

**Estimated new code:** ~200 lines Rust (LSP), ~750 lines TypeScript/HTML/JS.

### Phase 3: Eval and Eval-D

**Goal:** Extend the profiling panel pattern to adult aphasia (Eval) and
dementia (Eval-D) populations.

**Repo:** `talkbank-clan` + `talkbank-chatter`

**Tasks:**

1. **Eval-D command** — Implement in `talkbank-clan`. Similar to Eval but with
   dementia-specific population types (Control, MCI, Memory, Possible AD,
   Probable AD, Vascular) and different task set (Cat, Cinderella, Cookie,
   Hometown, Rockwell, Sandwich). ~400 lines Rust, modeled on `eval.rs`.

2. **Wire database comparison into Eval and Eval-D** — Same pattern as Phase 1:
   extend configs, load `.cut` files, filter, compute comparison stats.

3. **Profiling panel generalization** — Factor out common panel infrastructure
   from the KidEval panel into a shared base. The three commands share:
   - Speaker code selection (up to 8)
   - Gender filter
   - Age range input
   - "Analyze only" vs "Compare with DB" radio
   - Results display with score cards and comparison visualization

   Command-specific parts:
   - KidEval: language, corpus type (Toyplay/Narrative)
   - Eval: aphasia type, task/gem selection
   - Eval-D: dementia type, task selection

4. **Eval and Eval-D panels** — Using the shared base, build command-specific
   forms. Should be ~200 lines each beyond the shared code.

**Estimated new code:** ~500 lines Rust, ~600 lines TypeScript/HTML/JS.

### Phase 4: @ID / Participant Editor

**Goal:** Structured form for editing @ID metadata, replacing raw text editing.

**Repo:** `talkbank-chatter`

**Tasks:**

1. **New LSP request: `talkbank/document-ids`** — Returns parsed @ID data for
   the current document as structured JSON: list of participants with all fields
   typed. The Rust model (`IDHeader`, `Participant`) already has this; we just
   need to serialize it.

2. **New LSP request: `talkbank/update-ids`** — Accepts modified @ID data,
   computes a text edit that replaces the @ID lines in the document. Returns a
   `WorkspaceEdit` so the change goes through VS Code's undo stack.

3. **New LSP request: `talkbank/id-options`** — Returns valid options for
   dropdowns: participant roles, SES codes, ethnicity codes. Source these from
   the depfile or hardcode the CHAT standard values.

4. **IDs webview panel** — `idsPanel.ts` + `idsPanel.html` + `idsPanel.js`:
   - Participant selector (dropdown of existing speakers)
   - Text fields: language, corpus, age (y;m.d), group, education, custom field
   - Dropdowns: role (from depfile), SES (from depfile)
   - Radio buttons: sex (male/female/unknown)
   - Add/remove participant buttons
   - Done button applies edits to document

**Estimated new code:** ~150 lines Rust (LSP), ~600 lines TypeScript/HTML/JS.

### Phase 5: Coder Mode

**Goal:** Interactive coding panel for research annotation workflows.

**Repo:** `talkbank-chatter` (+ possibly `talkbank-model` for codes file parsing)

**Tasks:**

1. **Codes file parser** — Parse `.cut` codes files into a tree structure.
   Format is whitespace-indented hierarchy. ~100 lines Rust.

2. **LSP support for codes files** — New requests:
   - `talkbank/load-codes-file` — Parse and return the code tree as JSON.
   - `talkbank/insert-code` — Insert a selected code on the current utterance's
     %cod tier. Returns a `WorkspaceEdit`.

3. **Coder panel** — A VS Code side panel (not a webview — use a TreeView for
   the code hierarchy, which is more natural for tree navigation):
   - Load codes file via file picker
   - Display code tree in a TreeView with expand/collapse
   - Click a code to insert it on the current utterance
   - Step forward/backward through utterances (next uncoded, or sequential)
   - Status indicator showing coding progress (N of M utterances coded)

   Alternatively, a webview panel with a tree rendering — depends on whether
   TreeView or webview gives a better UX for this specific workflow.

4. **Keyboard navigation** — Arrow keys to navigate code tree, Enter to select,
   Tab to advance to next utterance. This is critical for coding speed.

**Estimated new code:** ~200 lines Rust, ~500 lines TypeScript/HTML/JS.

### Phase 6: Polish and Distribution

**Goal:** Production-ready release.

**Tasks:**

1. **CSV/Excel export** — Add an "Export" button to the analysis panel toolbar.
   Generate CSV from the JSON data and trigger a download/save dialog. ~100 lines
   JS added to `analysisPanel.js`.

2. **Corpus-level profiling** — Allow running KidEval/Eval/Eval-D on a directory
   of files, aggregating results. Follows the validation explorer pattern. ~200
   lines.

3. **Database bundling** — Package the normative database files with the
   extension or alongside the LSP binary. Add `talkbank.libPath` configuration
   setting with auto-discovery.

4. **User documentation** — Update `GUIDE.md` with the new features. Add
   screenshots.

5. **Testing** — Snapshot tests for database parsing and comparison stats.
   Vitest tests for panel message protocols.

---

## Effort Summary

| Phase | Description | New Rust | New TS/HTML/JS | Dependencies |
|-------|-------------|----------|----------------|--------------|
| 1 | Database infrastructure | ~500 lines | — | None |
| 2 | KidEval GUI | ~200 lines | ~750 lines | Phase 1 |
| 3 | Eval + Eval-D | ~500 lines | ~600 lines | Phases 1-2 |
| 4 | @ID Editor | ~150 lines | ~600 lines | None |
| 5 | Coder Mode | ~200 lines | ~500 lines | None |
| 6 | Polish | ~100 lines | ~300 lines | Phases 1-5 |
| **Total** | | **~1,650 lines** | **~2,750 lines** | |

Phases 4 and 5 are independent of phases 1-3 and can be developed in parallel.

---

## Why VS Code Is the Right Platform

1. **The hard parts are done.** The Rust analysis engine, LSP, editor
   integration, media playback, and waveform visualization all work. What
   remains is building forms and wiring them to existing backends.

2. **Cross-platform.** The CLAN Mac app only runs on macOS. The VS Code
   extension works on macOS, Windows, and Linux.

3. **Webviews beat NIBs.** HTML/CSS/JS builds richer, more responsive forms
   than Interface Builder layouts. Dynamic filtering (showing only databases
   matching a language selection) is trivial. Score visualization with color
   coding and charts is natural in HTML.

4. **Proven pattern.** We have four working webview panels with a consistent
   architecture. Adding new panels is routine.

5. **Already better than CLAN in several areas.** Corpus-scale validation,
   bidirectional alignment highlighting, inlay hints, code actions,
   go-to-definition, speaker filtering — these have no CLAN equivalent.

## Scale Comparison

| | CLAN Mac GUI | VS Code Extension (today) | After completion |
|---|---|---|---|
| Source files | 37 + 22 NIBs | 10 .ts + 8 webview | ~18 .ts + ~16 webview |
| GUI code | ~40,000 lines | ~3,500 lines | ~6,250 lines |
| Analysis backend | ~137,000 lines C++ | Rust crates via LSP | +~1,650 lines Rust |

The extension achieves more functionality in a fraction of the code because
VS Code provides the editor, file management, settings, and extension
infrastructure for free.
