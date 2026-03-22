# Diagram Audit Report: talkbank-tools & batchalign3

**Status:** Current
**Last updated:** 2026-03-22 10:58 EDT

This audit inventories all existing diagrams across both public repos,
assesses their quality against industry best practices (the Ilograph
"7 More Common Diagram Mistakes" blog series), identifies gaps, and
proposes CLAUDE.md instructions to help LLM agents produce diagrams that
avoid the shallow, hallucinated, or misleading patterns typical of
AI-generated diagrams.

Succession context: a professor who has never met the team will inherit
everything. Diagrams are often the first thing a newcomer reads to build
a mental model. Every recommendation below is framed through that lens.

---

## 1. Current Diagram Inventory

### talkbank-tools: ~25 Mermaid diagrams in 17 files + 6 ASCII art + 3 screenshots

| Location | Count | What | Quality |
|----------|-------|------|---------|
| `book/src/architecture/overview.md` | 1 mermaid + 1 ASCII | Crate dependency graph, data flow pipeline | Good — real crate names with role descriptions |
| `book/src/architecture/parsing.md` | 2 mermaid + 1 ASCII | CST→AST pipeline, ChatFile AST structure | Good — shows real module names |
| `book/src/architecture/data-model.md` | 2 | Tier alignment flows, content walker | Good but incomplete — no type hierarchy |
| `book/src/architecture/error-system.md` | 1 | ErrorSink trait implementations | Good — shows trait dispatch with 5 concrete impls |
| `book/src/architecture/transform-pipeline.md` | 1 | Validation + roundtrip cache lifecycle | Good — shows decision branches |
| `book/src/architecture/algorithms.md` | 4 | CHAT AST, tree-sitter strategy, DP alignment, caching | Good — detailed per-algorithm |
| `book/src/architecture/concurrency.md` | 3 mermaid + 1 ASCII | Threading model, validation parallelism, worker pool, IPC protocol | Very good — focused perspectives |
| `book/src/architecture/memory-and-ownership.md` | 2 | String representation, ChatFile lifecycle | Good |
| `book/src/architecture/wide-structs.md` | 1 | ParseHealth state diagram | Good |
| `book/src/architecture/boundary-vocabulary.md` | 2 | Boundary vocabulary, AuditReporter pattern | Good — conceptual clarity |
| `book/src/architecture/post-bootstrap-parser-testing.md` | 1 | Post-bootstrap test architecture | Good |
| `book/src/architecture/desktop-clan-proposal.md` | 2 | Existing integration, proposed design | Good for proposals |
| `book/src/contributing/documentation-architecture.md` | 1 | Documentation architecture (3 books + satellites) | Good |
| `crates/talkbank-clan/book/src/architecture/framework.md` | 2 | Command runner lifecycle, AnalysisCommand traits | Very good — `classDiagram` |
| `crates/talkbank-clan/book/src/user-guide/filtering.md` | 1 | Filter chain logic | Good |
| `vscode/DEVELOPER.md` | 1+ | Analysis pipeline, layered architecture | Good |
| `CLAUDE.md` (root) | 1 | Crate dependency flow | Good |
| Screenshots: `vscode/images/screenshots/` | 3 | validation.png, completion.png, dependency-graph.png | Real UI captures |

### batchalign3: ~60 Mermaid diagrams in 38+ files

| Category | Files w/ diagrams | Diagram count | Quality |
|----------|-------------------|---------------|---------|
| Architecture (core) | 20+ | 30+ | Very good — multiple focused perspectives per topic, real function names |
| Reference | 10+ | 12+ | Good — detailed decision trees for FA, language resolution |
| Developer | 6+ | 10+ | Good — migration diagrams, debugging flows |
| User Guide | 4+ | 5+ | Adequate but sparse for end-user audience |
| Decisions/Migration | 3+ | 5+ | Historical context, adequate |

**Standout diagrams in batchalign3:**

- `command-flowcharts.md` — 11 per-command decision trees with real
  function names, flag conditions, retry logic (35 nodes for `align`)
- `server-architecture.md` — 4 focused perspectives on one system
  (runtime layout, ownership boundaries, shared-state, route-state)
- `command-lifecycles.md` — 4 detailed `sequenceDiagram` scenarios
  showing real back-and-forth between CLI, server, cache, and workers
- `worker-memory-architecture.md` — Defense-in-depth layers with
  sequence diagrams showing TOCTOU race prevention

### Format summary

- **100% Mermaid** for architecture diagrams (no D2, PlantUML, draw.io)
- Diagram types used: `flowchart TD/LR` (~70%), `sequenceDiagram` (~15%),
  `classDiagram` (~5%), `graph LR/TD` (~5%), ASCII art (~5%)
- All version-controlled as text in markdown — no binary image files for
  architecture

---

## 2. Quality Assessment Against Ilograph Best Practices

Evaluated against the 7 anti-patterns from [7 More Common Mistakes in
Architecture Diagrams](https://www.ilograph.com/blog/posts/more-common-diagram-mistakes/)
and two related posts on AI diagram generation and master diagram
decomposition.

### Anti-Pattern 1: Not Including Resource Names — MOSTLY AVOIDED

Both repos label nodes with real module paths, crate names, and function
names. Good examples:

- `server-architecture.md`: `"JobStore"`, `"RuntimeSupervisor"`,
  `"WorkerPool"` — named subsystems, not generic boxes
- `command-flowcharts.md`: `"run_utr_pass()"`,
  `"process_fa_incremental"`, `"WhisperFa engine\nmax_group_ms=20000"`
  — real function names with parameters
- `concurrency.md`: `"crossbeam bounded channel\n(capacity = num_files)"`
  — named AND typed

**Minor concern:** A few overview diagrams use generic labels
(`"Rust Server"`, `"Python Worker"`) without specifying the concrete
crate/module. Acceptable at overview level but should always be
supplemented by focused diagrams that name concrete resources.

### Anti-Pattern 2: Unconnected Resources — NO VIOLATIONS

Every element in every diagram examined has connections.

### Anti-Pattern 3: Making a "Master" Diagram — ACTIVELY AVOIDED in batchalign3

**Good:** `server-architecture.md` decomposes the server into 4 focused
perspectives (runtime layout, ownership boundaries, shared-state,
route-state). Each tells one coherent story. This is exactly the Ilograph
"Breaking Up the Master Diagram" pattern.

**Acceptable:** `talkbank-tools/overview.md` has a single crate
dependency graph covering the whole workspace. Works because there are
only ~12 crates. As the system grows, the multi-perspective pattern
should be adopted.

### Anti-Pattern 4: Conveyor Belt Syndrome — PARTIALLY PRESENT

**Good:** `concurrency.md` IPC `sequenceDiagram` shows actual
request-response loops. `command-flowcharts.md` shows retry loops,
fallback paths, cache checks — not simple left-to-right.

**Risk area:** `overview.md` in batchalign3 and `transform-pipeline.md`
in talkbank-tools present genuinely sequential pipelines as sequential
flowcharts. This is appropriate when the pipeline IS sequential, but a
complementary `sequenceDiagram` showing the actual orchestrator-cache-worker
exchanges would add fidelity.

### Anti-Pattern 5: Meaningless Animations — NOT APPLICABLE

All diagrams are static Mermaid. No animations anywhere.

### Anti-Pattern 6: Fan Traps — MOSTLY AVOIDED

Diagrams name specific sub-resources (`"moka hot cache"` /
`"SQLite cold cache"` rather than just `"cache"`; specific worker types
rather than just `"Worker"`).

### Anti-Pattern 7: AI-Generated Diagram Quality — THE CORE RISK

The existing diagrams were written by humans with deep system knowledge.
The CLAUDE.md files currently say "Use Mermaid diagrams extensively" with
format guidance but minimal rules about *accuracy*, *verification*, or
*scope control*. This is the gap the Ilograph posts identify.

From "Diagrams AI Can, and Cannot, Generate" (Ilograph):

> AI-generated diagrams are "often vague, contain hallucinations, and
> exhibit many of the issues discussed above." Root causes: "almost
> complete lack of training data," "difficulties analyzing dense source
> code," and "general inability to strategically choose what to include
> and omit."

The proposed CLAUDE.md instructions (Section 5) directly address this
with Rule 7: mandatory source-code verification for every diagram node
and arrow.

---

## 3. Gap Analysis — Prioritized

### HIGH PRIORITY (succession-critical)

| # | Repo | Missing Diagram | Where to Add | Why Critical |
|---|------|-----------------|--------------|-------------|
| 1 | talkbank-tools | **ChatFile type hierarchy** — what a ChatFile looks like as a tree (Headers, Utterances, Speaker + MainTier + DependentTiers + Bullet) | `book/src/architecture/data-model.md` | A newcomer's first question: "what IS a CHAT file?" — currently answered only in prose and Rust structs |
| 2 | talkbank-tools | **Error code taxonomy** — 198 codes across E1xx-E7xx, no visual grouping by validation stage | `book/src/architecture/error-system.md` | Newcomer debugging a validation error needs the landscape |
| 3 | talkbank-tools | **Quality gates pipeline (G0-G10)** — 11 verification gates, not visualized | `book/src/contributing/quality-gates.md` | Newcomer running `make verify` needs to know what it does |
| 4 | batchalign3 | **Tiered cache architecture** — moka hot + SQLite cold read/write/promote paths | `book/src/architecture/caching.md` | Cache is foundational; anyone touching pipeline code needs the tiers |
| 5 | batchalign3 | **Rust crate dependency graph** — 5 workspace crates + cross-repo deps to talkbank-tools | `book/src/developer/rust-core.md` | talkbank-tools has one; batchalign3 should too, especially the cross-repo boundary |

### MEDIUM PRIORITY (significant onboarding value)

| # | Repo | Missing Diagram | Where to Add |
|---|------|-----------------|--------------|
| 6 | talkbank-tools | Grammar regeneration pipeline (grammar.js → tree-sitter → parser.c → Rust) | `book/src/contributing/grammar-workflow.md` |
| 7 | talkbank-tools | chatter CLI command dispatch flowchart | `book/src/user-guide/cli-reference.md` |
| 8 | talkbank-tools | Testing strategy taxonomy (unit, integration, property, golden, spec-generated) | `book/src/contributing/testing.md` |
| 9 | batchalign3 | Test tier pyramid (fast → worker → ML golden, with safety levels) | `book/src/developer/testing.md` |
| 10 | batchalign3 | Dashboard/Tauri component hierarchy | `book/src/developer/tauri-react-dashboard.md` |
| 11 | batchalign3 | Backchannel-aware alignment overlap handling | `book/src/developer/backchannel-aware-alignment.md` |

### LOWER PRIORITY (nice-to-have)

| # | Repo | Missing Diagram |
|---|------|-----------------|
| 12 | talkbank-tools | Performance cache hit/miss flowchart |
| 13 | talkbank-tools | Symbol registry generation pipeline |
| 14 | talkbank-tools | Per-CLAN-command input→output flow diagrams |
| 15 | talkbank-tools | Desktop app Tauri→Rust→frontend flow |
| 16 | batchalign3 | FA debugging decision tree (trouble-window alignment) |
| 17 | batchalign3 | Non-English processing fallback decision tree |

### Gap completion checklist

| # | Status | Diagram | Location |
|---|--------|---------|----------|
| 1 | [x] | ChatFile type hierarchy | talkbank-tools `data-model.md` |
| 2 | [x] | Error code taxonomy | talkbank-tools `error-system.md` |
| 3 | [x] | Quality gates pipeline | talkbank-tools `testing.md` |
| 4 | [x] | Tiered cache architecture | batchalign3 `caching.md` |
| 5 | [x] | batchalign3 crate dependency graph | batchalign3 `rust-workspace-map.md` |
| 6 | [x] | Grammar regeneration pipeline | talkbank-tools `grammar-workflow.md` |
| 7 | [x] | chatter CLI dispatch | talkbank-tools `cli-reference.md` |
| 8 | [x] | Testing strategy taxonomy | talkbank-tools `testing.md` |
| 9 | [x] | Test tier pyramid | batchalign3 `testing.md` |
| 10 | [x] | Dashboard/Tauri hierarchy | batchalign3 `tauri-react-dashboard.md` |
| 11 | [x] | Backchannel alignment | batchalign3 `backchannel-aware-alignment.md` |
| 12 | [x] | Cache hit/miss flow | Already in talkbank-tools `transform-pipeline.md` |
| 13 | [x] | Symbol registry pipeline | talkbank-tools `spec-workflow.md` |
| 14 | N/A | Per-CLAN-command I/O | Covered by existing `framework.md` classDiagram |
| 15 | [x] | Desktop Tauri flow | Already in talkbank-tools `desktop-clan-proposal.md` |
| 16 | [x] | FA debugging decision tree | Already in batchalign3 `trouble-window-alignment.md` |
| 17 | [x] | Non-English fallback tree | batchalign3 `non-english-workarounds.md` |

---

## 4. Redesign / Enhancement Suggestions for Existing Diagrams

### 4a. `batchalign3/book/src/architecture/overview.md` — Add cross-repo dependency perspective

The current Mermaid overview diagram is a clean high-level data flow.
Supplement (not replace) with a second diagram showing the crate
dependency graph with cross-repo boundaries. This follows the Ilograph
multi-perspective pattern: the existing diagram shows runtime data flow;
the new one shows build-time dependencies.

### 4b. `talkbank-tools/book/src/architecture/data-model.md` — Add ChatFile type hierarchy

The page currently shows Rust struct definitions in code blocks but no
visual representation. Add a `classDiagram` showing `ChatFile` owning
`Headers` and `Vec<Utterance>`, each `Utterance` owning `Speaker`,
`MainTierContent`, `Vec<DependentTier>`, and optional `Bullet`. Group
the 20+ `DependentTier` variants by category (alignment: Wor/Pho/Sin;
analysis: Mor/Gra; metadata: Cod/Gem) to stay under 30 nodes.

### 4c. Sequential pipeline diagrams — Complement with sequence diagrams

Pages like `transform-pipeline.md` (talkbank-tools) and
`pipeline-system.md` (batchalign3) show genuinely sequential pipelines,
which is correct. But the Ilograph "conveyor belt" warning applies: the
*orchestration* of these pipelines involves real back-and-forth (cache
checks, worker IPC, error fallbacks). Adding a `sequenceDiagram` showing
the orchestrator's actual interactions with cache and workers would
complement the existing flowcharts.

---

## 5. Proposed CLAUDE.md Diagram Authoring Instructions

This is the centerpiece deliverable — a set of rules to replace the
brief existing diagram sections in both repos' CLAUDE.md files. The
full text is maintained in the CLAUDE.md files themselves (see
`talkbank-tools/CLAUDE.md` and `batchalign3/CLAUDE.md`). Below is the
summary structure.

### Structure

1. **When to Create a Diagram** — trigger conditions (pipelines,
   boundaries, state machines, decision trees, type relationships,
   protocols). A page that describes a pipeline or decision flow in
   prose without a diagram is incomplete.

2. **Diagram Type Selection** — table mapping situations to Mermaid
   diagram types (`flowchart` for pipelines, `sequenceDiagram` for
   request/response, `classDiagram` for types, `stateDiagram-v2` for
   lifecycles).

3. **The Seven Diagram Rules:**
   - **Rule 1: Name every resource** — specific name + type/role, greppable
   - **Rule 2: One concept per diagram** — focused perspectives, not master diagrams
   - **Rule 3: No conveyor belts for interactive flows** — use `sequenceDiagram` for back-and-forth
   - **Rule 4: Show real decision points** — real function/flag names in decision diamonds
   - **Rule 5: Include error and fallback paths** — never show only the happy path
   - **Rule 6: Anchor to source locations** — crate/module/file path in labels or prose
   - **Rule 7: Never generate diagrams from source code without verification** — read source, verify every node and arrow, omit what cannot be verified

4. **Formatting Standards** — node label syntax, decision node syntax,
   edge labels, subgraph policy, no custom colors, 30-node size limit,
   angle-bracket escaping for mdBook.

5. **Placement** — inline near prose, prose introduction required,
   multi-perspective pattern for complex topics.

---

## 6. Evaluating Diagram Quality: A Human UX Checklist

Use during code review or periodic documentation audits.

### Accuracy (does it match reality?)

- [ ] **Existence check:** Every node corresponds to a real module, type,
  function, or resource in the current codebase. Grep for it.
- [ ] **Connection check:** Every arrow corresponds to a real call,
  dependency, import, or data flow. Trace it in the source.
- [ ] **Completeness check:** No critical path is omitted. Error/fallback
  paths shown? All decision branches covered?
- [ ] **Currency check:** Diagram reflects current code, not a past or
  planned state. Check `Last modified` date against recent commits.

### Usefulness (does it help a newcomer?)

- [ ] **5-second test:** Can a reader understand the diagram's subject
  within 5 seconds? If not, simplify or split.
- [ ] **Prose pairing:** Does prose immediately above explain what it
  shows and why it matters?
- [ ] **Greppability:** Can a reader take a node label and find the
  corresponding code with a simple search?
- [ ] **Scope discipline:** One concept, not a master diagram? Under
  30 nodes? No mixed concerns?
- [ ] **Appropriate type:** Sequence diagrams for request/response,
  flowcharts for pipelines, class diagrams for type hierarchies?

### Maintainability (will it stay accurate?)

- [ ] **Text-as-code:** Mermaid or another diffable text format, not a
  binary image?
- [ ] **Inline placement:** Near the prose it illustrates, not in a
  separate "diagrams" directory?
- [ ] **Size constraint:** Under 30 nodes? Larger diagrams accumulate
  inaccuracies because nobody updates them.
