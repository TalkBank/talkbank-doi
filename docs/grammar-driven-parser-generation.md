# Grammar-Driven Parser Generation for TalkBank

**Status:** Current
**Last updated:** 2026-03-23 06:54 EDT

A research analysis of whether the hand-written Rust tree-sitter CST walker
(`talkbank-parser`, ~21,000 lines) can be substantially auto-generated from the
tree-sitter grammar definition (`grammar.js`, 2,045 lines), and whether the
Chumsky-based direct parser (`talkbank-direct-parser`, ~9,400 lines) should be
replaced by a different approach to fragment parsing.

**Implementation status:** Chumsky eliminated. Multi-root grammar with
structured word parsing. 77/77 roundtrip, 100,039/100,039 real files clean.
See the long-term roadmap at
`/Users/chen/.claude/plans/federated-crunching-spark.md` for the execution plan.

---

## Table of Contents

1. [Anatomy of the Grammar](#chapter-1-anatomy-of-the-grammar)
2. [Anatomy of the Hand-Written Parser](#chapter-2-anatomy-of-the-hand-written-parser)
3. [The Grammar as an Intermediate Representation](#chapter-3-the-grammar-as-an-intermediate-representation)
4. [What Can Be Auto-Generated (and What Cannot)](#chapter-4-what-can-be-auto-generated-and-what-cannot)
5. [Error Code Generation from Grammar Context](#chapter-5-error-code-generation-from-grammar-context)
6. [Data Model Generation Feasibility](#chapter-6-data-model-generation-feasibility)
7. [The Chumsky Parser: Assessment and Alternatives](#chapter-7-the-chumsky-parser-assessment-and-alternatives)
8. [Fragment Parsing Without tree-sitter](#chapter-8-fragment-parsing-without-tree-sitter)
9. [Test Generation from Grammar](#chapter-9-test-generation-from-grammar)
10. [The Generator Architecture](#chapter-10-the-generator-architecture)
11. [Succession and Maintenance Implications](#chapter-11-succession-and-maintenance-implications)

---

## Chapter 1: Anatomy of the Grammar

### 1.1 Overview

The CHAT tree-sitter grammar (`grammar/grammar.js`) defines a **concrete syntax
tree** (CST) for the CHAT transcription format. Every byte of the input ---
whitespace, delimiters, continuation lines --- is represented as a node. The
grammar contains **367 rules** (per `grammar.json`), producing **361 named node
types** (per `node-types.json`).

The design philosophy is **"parse, don't validate."** The grammar accepts all
plausible CHAT input. Invalid values (wrong date formats, unknown option names,
misspelled tiers) parse successfully into the CST. Downstream Rust code
(`talkbank-parser` + `talkbank-model`) performs validation and reports typed
errors.

### 1.2 Three Tiers of Rules

The 367 rules fall into three architectural tiers:

**Tier 1: Leaf tokens** (140 TOKEN/IMMEDIATE_TOKEN rules) --- atomic regex or
string matches that are opaque to tree-sitter's structural parser. Internal
structure is parsed downstream. Examples:

- `standalone_word` --- captures everything between word boundaries as one token
  (word-internal prosody, CA markers, shortenings, compounds are parsed later)
- `inline_bullet` --- `\u0015\d+_\d+\u0015` timestamp markers
- Bracket annotations like `[= text]`, `[=! text]`, `[* code]`
- Terminators like `+...`, `+/.`, `+//.`

**Tier 2: Structural compositions** (116 SEQ, 28 CHOICE, 8 REPEAT rules) ---
combine leaf tokens into CHAT constructs:

- `utterance` = `main_tier` + `dependent_tier*`
- `main_tier` = `star` + `speaker` + `colon` + `tab` + `tier_body`
- `word_with_optional_annotations` = `standalone_word` + optional `replacement`
  + optional `base_annotations`

**Tier 3: Supertypes** (6 abstract CHOICE rules) --- group related node types
for query convenience:

| Supertype | Subtypes |
|-----------|----------|
| `terminator` | 17 variants (`.`, `?`, `!`, `+...`, `+/.`, `+//.`, etc.) |
| `linker` | 7 variants (`++`, `+<`, `+^`, `+"`, `+,`, `+≈`, `+≋`) |
| `base_annotation` | 18 variants (`[!]`, `[?]`, `[//]`, `[= ...]`, `[* ...]`, etc.) |
| `dependent_tier` | 31 variants (`%mor`, `%gra`, `%pho`, `%sin`, `%com`, etc.) |
| `header` | 34 variants (`@Languages`, `@Participants`, `@ID`, `@Date`, etc.) |
| `pre_begin_header` | 4 variants (`@PID`, `@Color words`, `@Window`, `@Font`) |

Additionally, the parser defines its own compound supertype matchers for
`ca_delimiter`, `ca_element`, and `overlap_point_marker`, bringing the total to
9 supertype-like dispatchers.

### 1.3 The `extras: []` Decision

Tree-sitter normally auto-skips whitespace via `extras`. This grammar sets
`extras: []` and handles all whitespace explicitly. This is necessary because
CHAT uses whitespace structurally:

- Tab characters separate header prefixes from content (`@Languages:\teng`)
- Continuation lines (newline + tab) indicate multi-line content
- Whitespace between content items must be preserved for roundtrip serialization

The consequence: every SEQ rule that allows spaces must thread `whitespaces`
tokens explicitly. This is a major source of boilerplate in both the grammar and
the Rust parser.

### 1.4 Precedence Strategy

Tokenization ambiguity is resolved with six precedence levels:

| Level | Used by | Purpose |
|-------|---------|---------|
| `prec(10)` | Terminators, linkers, CA markers, group delimiters | Structural tokens that must never be word content |
| `prec(8)` | Bracket annotations, `langcode` | Beat word tokens but not structural |
| `prec(5)` | `standalone_word`, overlap points | Main word tokens |
| `prec(3)` | `zero` (omission marker `0`) | Beat `natural_number` |
| `prec(1)` | `event_segment`, `id_ses` catch-alls | Lowest priority |
| `prec(-1)` | `separator` | Free-floating punctuation in CA mode |

### 1.5 Conflict Declarations

Five rules require explicit `conflicts` declarations for tree-sitter's GLR
parser:

1. `contents` --- trailing whitespace before a terminator vs. start of next item
2. `word_with_optional_annotations` --- bracket after word: belongs to word or
   is separate?
3. `nonword_with_optional_annotations` --- same ambiguity as words
4. `base_annotations` --- multiple annotations in sequence
5. `final_codes` --- postcodes after terminators

### 1.6 The Strict + Catch-All Pattern

For header fields with a closed set of valid values, the grammar provides both
strict string matches and a generic regex catch-all. Tree-sitter's DFA gives
string literals priority over regexes at the same length, so known values win
automatically. Unknown values fall through to the catch-all and are flagged by
the Rust validator.

Ten rules use this pattern:

| Rule | Known values | Catch-all | Error code |
|------|-------------|-----------|------------|
| `option_name` | `CA`, `NoAlign` | `generic_option_name` | E534 |
| `media_type` | `video`, `audio`, `missing` | `generic_media_type` | E535 |
| `media_status` | `missing`, `unlinked`, `notrans` | `generic_media_status` | E536 |
| `recording_quality_option` | `1`-`5` | `generic_recording_quality` | E538 |
| `transcription_option` | `eye_dialect`, `partial`, `full` | `generic_transcription` | E539 |
| `number_option` | `1`-`5`, `more`, `audience` | `generic_number` | E537 |
| `date_contents` | DD-MMM-YYYY pattern | `generic_date` | E518 |
| `time_duration_contents` | digit patterns | `generic_time` | E540/E541 |
| `id_sex` | `male`, `female` | `generic_id_sex` | E542 |
| `id_ses` | SES codes | `generic_id_ses` | E546 |

`id_ses` uses `token(prec(1, regex))` instead of string literals to avoid
keyword conflicts --- words like White, Black, Native appear in utterance text.

### 1.7 Document Structure

```
document
+-- utf8_header              "@UTF8\n"
+-- pre_begin_header*        "@PID", "@Font", "@Window", "@Color words"
+-- begin_header             "@Begin\n"
+-- line*
|   +-- header               "@Languages:\teng\n"
|   +-- utterance
|   |   +-- main_tier        "*CHI:\twant more cookie ."
|   |   |   +-- star, speaker, colon, tab
|   |   |   +-- tier_body
|   |   |       +-- linkers?
|   |   |       +-- contents (words, events, groups, pauses, annotations)
|   |   |       +-- utterance_end (terminator, postcodes, media_url, newline)
|   |   +-- dependent_tier*  "%mor:\tv|want qn|more n|cookie"
|   +-- unsupported_line     catch-all for unrecognized lines
+-- end_header               "@End\n"
```

### 1.8 Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| Opaque word tokens | Word-internal syntax (prosody, CA, shortenings, compounds) is too complex for tree-sitter's lexer. Parsed downstream by Chumsky/direct parser. |
| Atomic annotation tokens | Bracket annotations are simple enough to extract with string operations. Avoids state explosion from nested bracket disambiguation. |
| Explicit whitespace | CHAT uses tabs structurally; continuation lines are meaningful. Cannot auto-skip. |
| Parse, don't validate | Grammar accepts all plausible input. Validation is a separate phase with typed error codes. |

---

## Chapter 2: Anatomy of the Hand-Written Parser

### 2.1 Scale

The tree-sitter-based parser crate (`talkbank-parser`) contains approximately
**20,978 lines** of Rust code across ~100 source files. Its sole job: walk the
tree-sitter CST and produce typed values from `talkbank-model`.

For comparison:
- The grammar that defines the CST: **2,045 lines** of JavaScript
- The data model that receives the output: **~51,750 lines** of Rust
- The alternative Chumsky parser: **~9,400 lines** of Rust

The parser is roughly **10x the size of the grammar** --- an order-of-magnitude
amplification that is the central motivation for this research.

### 2.2 Module Organization

```
crates/talkbank-parser/src/
+-- api/                          # Public API (parse_file, parse_word, etc.)
|   +-- parser_api.rs             # 537 lines - main API surface
|   +-- file.rs                   # 107 lines - file-level helpers
|   +-- main_tier.rs              # main tier fragment helpers
|   +-- synthetic_fragments.rs    # tree-sitter fragment wrappers
+-- parser/
|   +-- chat_file_parser/         # File-level orchestration
|   |   +-- chat_file/            # Top-level parse flow
|   |   +-- header_parser/        # Pre-@Begin structural parsing
|   |   +-- header_dispatch/      # Header node dispatch (572 lines)
|   |   +-- utterance_parser.rs   # Main tier conversion (400 lines)
|   |   +-- dependent_tier_dispatch/  # Tier type dispatch
|   |   +-- single_item/          # Fragment parse helpers
|   +-- tree_parsing/             # CST-to-model conversion (core)
|   |   +-- header/               # ID, Participants, Languages, Media, etc.
|   |   +-- main_tier/            # The largest subtree
|   |   |   +-- content/          # Word, group, event, pause, nonvocal
|   |   |   +-- annotations/      # Error, replacement, retrace, scoped
|   |   |   +-- structure/        # Prefix, body, terminator, utterance-end
|   |   |   +-- word/             # Word-level CST conversion
|   |   +-- dependent_tier/       # Placeholder (dispatch lives elsewhere)
|   |   +-- parser_helpers/       # Reusable traversal utilities
|   |   |   +-- cst_assertions.rs # 352 lines - expect_child, assert_*
|   |   |   +-- error_analysis/   # Context-sensitive ERROR classification
|   |   |   +-- supertypes/       # 7 files, 9 is_*() functions
|   |   |   +-- node_dispatch/    # Separator, CA, overlap, pause handlers
|   |   +-- bullet_content/       # Media bullet and timestamp parsing
|   |   +-- freecode/             # Freecode parsing
|   +-- tier_parsers/             # Typed dependent-tier parsers
|   +-- participants/             # Participant declaration handling
+-- node_types.rs                 # 827 lines - GENERATED string constants
+-- error/                        # Error types (delegates to talkbank-model)
```

### 2.3 The Dominant Boilerplate Pattern

The single most common pattern in the parser is **child-indexed sequential
traversal**. Here is the canonical example from `word.rs`:

```rust
pub(crate) fn parse_word_content(
    node: Node,
    source: &str,
    errors: &impl ErrorSink,
) -> ParseOutcome<UtteranceContent> {
    let child_count = node.child_count();
    let mut word = None;
    let mut replacement = ParseOutcome::rejected();
    let mut annotations = Vec::with_capacity(2);
    let mut idx: u32 = 0;

    // Position 0: standalone_word (required)
    if let ParseOutcome::Parsed(child) = expect_child(
        node, idx, STANDALONE_WORD, source, errors,
        "word_with_optional_annotations",
    ) {
        if let ParseOutcome::Parsed(w) = convert_word_node(child, source, errors) {
            word = Some(w);
        }
        idx += 1;
    }

    // Position 1+: optional whitespaces, replacement, base_annotations
    while (idx as usize) < child_count {
        if let Some(child) = node.child(idx) {
            match child.kind() {
                WHITESPACES => { idx += 1; }
                REPLACEMENT => {
                    replacement = parse_replacement(child, source, errors);
                    idx += 1;
                }
                BASE_ANNOTATIONS => {
                    let annots = parse_scoped_annotations(child, source, errors);
                    annotations.extend(annots);
                    idx += 1;
                }
                _ => {
                    errors.report(unexpected_node_error(child, source, "..."));
                    idx += 1;
                }
            }
        } else { break; }
    }

    // Reconstruct model value from collected optionals
    if let Some(w) = word {
        if let ParseOutcome::Parsed(repl) = replacement {
            ParseOutcome::parsed(UtteranceContent::ReplacedWord(Box::new(
                ReplacedWord::new(w, repl).with_scoped_annotations(annotations))))
        } else if annotations.is_empty() {
            ParseOutcome::parsed(UtteranceContent::Word(Box::new(w)))
        } else {
            ParseOutcome::parsed(UtteranceContent::AnnotatedWord(Box::new(
                Annotated::new(w).with_scoped_annotations(annotations))))
        }
    } else {
        ParseOutcome::rejected()
    }
}
```

This pattern has these invariant elements:

1. **Index variable** `idx: u32` (never `usize` --- tree-sitter API uses `u32`)
2. **Pre-allocated vectors** with `Vec::with_capacity()`
3. **Sequential child loop** `while (idx as usize) < child_count`
4. **Kind-based dispatch** `match child.kind() { WHITESPACES => ..., X => ... }`
5. **Whitespace skipping** `WHITESPACES => { idx += 1; }`
6. **Unexpected node reporting** `errors.report(unexpected_node_error(...))`
7. **Model reconstruction** at the end from collected `Option<T>` fields

### 2.4 Quantitative Boilerplate Analysis

Measured across the parser crate:

| Pattern | Count |
|---------|-------|
| Child-indexed traversal loops (`while idx < child_count`) | 22 |
| `match child.kind()` dispatch blocks | 29 |
| `expect_child()` calls (positional child access with kind check) | 32 |
| `WHITESPACES =>` match arms (whitespace skipping) | 19 |
| `unexpected_node_error()` calls (catch-all error reporting) | 50 |
| `assert_child_count_*()` calls (structural validation) | 15 |
| `ParseOutcome::rejected()` return points | 169 |

Each of these is structurally derivable from the grammar: the grammar defines
which children appear at which positions, which are optional, and which node
kinds are valid. The parser hand-codes this information.

### 2.5 CST Assertion Helpers

The `cst_assertions.rs` module (352 lines) provides safety checks that
catch grammar changes:

- `assert_child_count_exact(node, expected, source, errors, context)` --- verifies
  exact child count for fixed-structure SEQ rules
- `assert_child_count_min(node, minimum, source, errors, context)` --- verifies
  minimum for variable-length rules
- `assert_child_kind(node, position, expected_kind, source, errors, context)` ---
  verifies child type at a specific position
- `expect_child(node, position, expected_kind, source, errors, context)` ---
  combined existence + kind check + MISSING node detection, returns
  `ParseOutcome<Node>`
- `check_not_missing(node, source, errors, context)` --- inline MISSING check
- `extract_utf8_text(node, source, errors, context, fallback)` --- safe text
  extraction with error reporting

These helpers are themselves boilerplate --- each encodes the same "check
position, report error" pattern with different parameters. A generator would
emit the equivalent checks inline.

### 2.6 Supertype Matchers

Seven files in `parser_helpers/supertypes/` define 9 `is_*()` functions that
manually list all subtypes of grammar supertypes:

| Function | Subtypes listed |
|----------|----------------|
| `is_terminator()` | 17 string literals |
| `is_linker()` | 7 |
| `is_base_annotation()` | 18 |
| `is_dependent_tier()` | 31 |
| `is_header()` | 34 |
| `is_pre_begin_header()` | 4 |
| `is_ca_element()` | ~10 |
| `is_ca_delimiter()` | ~15 |
| `is_overlap_point_marker()` | ~8 |

Every one of these mirrors a `choice()` rule in `grammar.js` and could be
generated from `node-types.json`'s `subtypes` arrays.

### 2.7 Error Handling Architecture

The parser uses two complementary error handling systems:

**ErrorSink trait** --- a streaming error collector that avoids intermediate
allocation. Functions report errors via `errors.report(ParseError::new(...))` as
they encounter problems. This enables the parser to continue past errors without
collecting them all in a `Vec` first.

**ParseOutcome&lt;T&gt;** --- a result-like monad with two variants:
`Parsed(T)` and `Rejected`. Unlike `Result`, `Rejected` carries no error
payload --- errors are already reported via `ErrorSink`. This separation means
the parser can reject a value (returning `Rejected`) while having already
reported a detailed diagnostic to the user.

**Context-sensitive ERROR analysis** --- five modules in `error_analysis/`
classify tree-sitter `ERROR` nodes based on where they appear:

- `file.rs` --- document-level errors
- `line.rs` --- routes to header or utterance analyzers based on context
- `header.rs` --- header-specific error patterns
- `utterance.rs` --- utterance-specific patterns (e.g., `@` without form type,
  empty replacement `[:]`, unknown annotation `[@`)
- `dependent_tier.rs` --- tier-specific patterns

These heuristics examine the ERROR node's text content and surrounding context
to produce domain-specific diagnostics rather than generic "syntax error"
messages. This is the part of the parser that is hardest to auto-generate.

### 2.8 The Parser as Grammar Interpreter

The core insight of this chapter: **the parser is a hand-coded interpreter of
the grammar's structure**. Each grammar rule `R` has a corresponding Rust
function `parse_R()` that:

1. Checks the node kind matches `R`
2. Iterates children in the order defined by `R`'s production
3. Dispatches each child based on its kind (mirroring `R`'s CHOICE/SEQ/REPEAT)
4. Skips whitespace nodes
5. Reports unexpected nodes
6. Assembles a typed model value from the collected children

This 1:1 correspondence between grammar rules and parser functions is the
foundation for auto-generation.

---

## Chapter 3: The Grammar as an Intermediate Representation

### 3.1 grammar.json: The Tree-Sitter-Native IR

When `tree-sitter generate` processes `grammar.js`, it produces
`grammar.json` (5,886 lines) --- a complete, typed, JSON serialization of every
grammar rule. This is not a lossy summary; it is the **canonical intermediate
representation** that tree-sitter's own code generator consumes to produce
`parser.c`.

Every grammar.js construct maps to a tagged JSON object with a `type` field:

| grammar.js | grammar.json `type` |
|------------|---------------------|
| `seq(a, b, c)` | `SEQ { members: [a, b, c] }` |
| `choice(a, b)` | `CHOICE { members: [a, b] }` |
| `repeat(x)` | `REPEAT { content: x }` |
| `repeat1(x)` | `REPEAT1 { content: x }` |
| `optional(x)` | `CHOICE { members: [x, BLANK] }` |
| `$.rule_name` | `SYMBOL { name: "rule_name" }` |
| `'literal'` | `STRING { value: "literal" }` |
| `/regex/` | `PATTERN { value: "regex" }` |
| `token(x)` | `TOKEN { content: x }` |
| `token.immediate(x)` | `IMMEDIATE_TOKEN { content: x }` |
| `prec(n, x)` | `PREC { value: n, content: x }` |
| `prec.left(n, x)` | `PREC_LEFT { value: n, content: x }` |
| `prec.right(n, x)` | `PREC_RIGHT { value: n, content: x }` |
| `prec.dynamic(n, x)` | `PREC_DYNAMIC { value: n, content: x }` |
| `alias(x, name)` | `ALIAS { content: x, named: bool, value: name }` |
| `field(name, x)` | `FIELD { name: name, content: x }` |

Example: the `document` rule in grammar.json:
```json
{
  "type": "SEQ",
  "members": [
    { "type": "SYMBOL", "name": "utf8_header" },
    { "type": "REPEAT", "content": { "type": "SYMBOL", "name": "pre_begin_header" } },
    { "type": "SYMBOL", "name": "begin_header" },
    { "type": "REPEAT", "content": { "type": "SYMBOL", "name": "line" } },
    { "type": "SYMBOL", "name": "end_header" }
  ]
}
```

This tells us exactly: the document has 5 positional children --- `utf8_header`
at position 0, a repeat of `pre_begin_header`, `begin_header`, a repeat of
`line`, and `end_header`. A generator can produce the traversal scaffolding
directly from this.

### 3.2 node-types.json: The Schema

`node-types.json` (5,218 lines, 380 entries, 361 named) describes the **CST
schema** --- what children each node type can have, with cardinality information:

```json
{
  "type": "word_with_optional_annotations",
  "named": true,
  "fields": {
    "word": {
      "required": true,
      "multiple": false,
      "types": [{ "type": "standalone_word", "named": true }]
    },
    "replacement": {
      "required": false,
      "multiple": false,
      "types": [{ "type": "replacement", "named": true }]
    },
    "annotations": {
      "required": false,
      "multiple": false,
      "types": [{ "type": "base_annotations", "named": true }]
    }
  }
}
```

For nodes **with named fields** (10 rules use `field()` in the grammar), the
schema tells us:

- **required**: whether the child is always present
- **multiple**: whether it can appear more than once
- **types**: which node types can fill that position

For the remaining 143 nodes **with unnamed children**, the schema provides the
same cardinality info for the aggregate child list.

Supertype nodes carry **subtypes** arrays listing all alternatives:

```json
{
  "type": "dependent_tier",
  "named": true,
  "subtypes": [
    { "type": "act_dependent_tier", "named": true },
    { "type": "add_dependent_tier", "named": true },
    ...
  ]
}
```

### 3.3 tree-sitter Internal IRs

Inside tree-sitter's code generator (`crates/generate/src/`), the grammar
passes through several internal representations:

1. **InputGrammar** --- direct deserialization of grammar.json into Rust structs.
   `Variable` has `name`, `kind` (Hidden/Auxiliary/Anonymous/Named), and `rule`
   (recursive enum matching the JSON types).

2. **SyntaxGrammar** --- after normalization. Each variable has `productions:
   Vec<Production>`, where each `Production` has `steps:
   Vec<ProductionStep>` carrying symbol, precedence, associativity, alias, and
   field name.

3. **LexicalGrammar** --- NFA for token extraction. Variables have
   `implicit_precedence` and start states in the NFA.

These IRs are internal to tree-sitter and not directly accessible to external
tools. However, **grammar.json and node-types.json provide equivalent
information** for our purposes --- we need production structure and child
schemas, both of which are fully represented in the JSON artifacts.

### 3.4 What tree-sitter-grammar-utils Already Provides

Franklin's toolkit (`~/tree-sitter-grammar-utils/`, ~13 source files) already
provides significant infrastructure for grammar analysis:

| Module | Capability |
|--------|-----------|
| `ast.rs` | Parse `grammar.js` via oxc; extract grammar object, rules, properties; detect `$.rule_name` references |
| `dependency_graph.rs` | BFS transitive dependency graph from any entry rule; track referrers |
| `prune.rs` | Extract grammar subset rooted at an entry rule; surgical source patching |
| `audit.rs` | Topology analysis; identify protected rules (from .scm queries), disposable rules (single-caller wrappers), supertype candidates |
| `optimize.rs` | Automated transformation loop with verification (precedence, inlining, conflict pruning) |
| `transform.rs` | Apply grammar transforms (remove rules, update precedence, inline) |
| `verification.rs` | Verify transforms via `tree-sitter generate` compilation |
| `types.rs` | `RuleName`, `DependencyGraph`, `PruneResult`, `OptimizationProfile` |

### 3.5 The Gap: JS AST vs. grammar.json

The grammar-utils toolkit works at the **JavaScript AST level** via oxc. It
parses `grammar.js` as JavaScript and traverses the AST to find `$.rule_name`
references, function calls like `seq()`, `choice()`, etc.

For code generation, **grammar.json is a better input** because:

1. It is a stable, versioned format (tree-sitter's own schema)
2. It is already fully normalized (no JavaScript evaluation needed)
3. It can be deserialized with plain `serde_json` --- no JS parser dependency
4. It carries the same structural information as the JS AST
5. It is cross-language (works with any grammar, not just JavaScript-authored ones)

The natural architecture is: grammar-utils continues to use the JS AST for
source-level manipulations (pruning, inlining, formatting), while the **code
generator** reads grammar.json + node-types.json.

### 3.6 What Can Be Extracted

From grammar.json + node-types.json together, we can extract:

| Information | Source | Currently hand-coded? |
|-------------|--------|----------------------|
| Node type string constants | grammar.json rule names | Yes --- `node_types.rs` (827 lines, already generated) |
| Supertype membership lists | node-types.json `subtypes` | Yes --- 7 files, 9 `is_*()` functions |
| SEQ child positions and expected kinds | grammar.json SEQ members | Yes --- every `expect_child()` call |
| CHOICE alternatives for dispatch | grammar.json CHOICE members | Yes --- every `match child.kind()` block |
| Required vs. optional children | node-types.json `required`/`multiple` | Yes --- `Option<T>` vs `T` in model reconstruction |
| REPEAT loop structure | grammar.json REPEAT/REPEAT1 | Yes --- `while` loops in parser |
| TOKEN/opaque nodes (parse downstream) | grammar.json TOKEN wrapper | Yes --- delegation to Chumsky/direct parser |
| Whitespace positions in SEQ | grammar.json SEQ members containing `whitespaces` | Yes --- `WHITESPACES => { idx += 1; }` arms |

---

## Chapter 4: What Can Be Auto-Generated (and What Cannot)

### 4.1 Generation Tiers

Not everything in the parser is equally amenable to auto-generation. We
distinguish four tiers:

### Tier 1: Already Generated

**`node_types.rs`** (827 lines, 361 constants) is already generated by
`scripts/generate-node-types.js` from `node-types.json`. This proves the
concept: grammar artifacts can drive Rust code generation.

### Tier 2: Mechanically Generatable

These require no human annotation --- the grammar.json + node-types.json structure
is sufficient.

**Supertype membership functions.** Replace 9 hand-written `is_*()` functions
with generated `matches!()` expressions. Input: `node-types.json` subtypes
arrays. Example of current hand-written code in `terminators.rs`:

```rust
pub fn is_terminator(kind: &str) -> bool {
    matches!(kind,
        "terminator" | "break_for_coding" | "broken_question" |
        "ca_no_break" | "ca_no_break_linker" | /* ... 12 more ... */
        "trailing_off" | "trailing_off_question"
    )
}
```

Generated version would read subtypes from node-types.json and produce
identical code.

**Sequential traversal scaffolds.** For each SEQ rule in grammar.json, generate
a function that extracts children by position with kind checking. The grammar
tells us exactly which child appears at which position.

For `word_with_optional_annotations` (which has named fields), the grammar tells
us:

- Position "word": `standalone_word`, required, single
- Position "replacement": `replacement`, optional, single
- Position "annotations": `base_annotations`, optional, single

A generator can produce:

```rust
struct WordWithOptionalAnnotationsChildren<'a> {
    pub word: Option<Node<'a>>,          // required but may be MISSING
    pub replacement: Option<Node<'a>>,   // optional
    pub annotations: Option<Node<'a>>,   // optional
}

fn extract_word_with_optional_annotations_children<'a>(
    node: Node<'a>, source: &str, errors: &impl ErrorSink,
) -> WordWithOptionalAnnotationsChildren<'a> {
    // Generated: iterate children, match kinds, skip whitespace, report unexpected
}
```

**Child count assertions.** For fixed-length SEQ rules, generate
`assert_child_count_exact()` calls. For variable-length rules (containing
REPEAT), generate `assert_child_count_min()` calls.

**Choice dispatch tables.** For each CHOICE rule, generate a `match` block
dispatching on `child.kind()` to the appropriate handler. Currently hand-written
as:

```rust
match child.kind() {
    CONTENT_ITEM => { ... }
    OVERLAP_POINT | SEPARATOR | COLON | COMMA | ... => { ... }
    WHITESPACES => continue,
    _ => { errors.report(unexpected_node_error(...)); }
}
```

**REPEAT loop patterns.** For REPEAT/REPEAT1 children, generate `while` loops
that collect elements into `Vec<T>`.

**Whitespace skipping.** Detect `whitespaces` symbols in SEQ members and generate
skip logic.

### Tier 3: Generatable with Annotation

These require a small annotation file mapping grammar rules to semantic
information that isn't in grammar.json.

**Error codes per grammar position.** Knowing which production failed and at
which position, we can assign specific error codes. But the *specific* code
(E302 vs E305 vs E342) requires human decision. An annotation file maps:

```toml
[rules.main_tier]
children.speaker.missing_error = "E304"    # MissingSpeaker
children.terminator.missing_error = "E305" # MissingTerminator
```

**MISSING node messages.** Tree-sitter MISSING nodes carry the expected `kind()`.
Message templates can be generated: "Expected `{kind}` at position {pos} in
`{parent_rule}`". But the user-friendly phrasing ("Missing speaker code ---
utterances must begin with `*SPEAKER:`") requires annotation.

**Recovery strategies.** For optional children (`required: false` in
node-types.json), the generated code can skip gracefully. For required children,
it can report and return `ParseOutcome::rejected()`. But some rules need custom
recovery (e.g., trying to salvage partial word content from an ERROR node).

### Tier 4: Cannot Be Generated

These require semantic understanding that is not expressible in the grammar:

**Model enum construction.** The grammar has `word_with_optional_annotations`
with optional children. The model has three distinct types: `Word` (bare),
`AnnotatedWord` (with annotations), `ReplacedWord` (with replacement). The
mapping from "which optional children are present" to "which enum variant to
construct" is semantic logic.

**Cross-node coordination.** Overlap matching (`[<1]...[>1]`), scoped
annotation nesting, quotation balancing, and gem scope tracking all require
state that spans multiple nodes.

**Post-parse normalization.** Speaker validation against `@Participants`,
header ordering checks, `@Options: CA` mode detection --- these depend on
file-level context.

**Delegation to word-internal parser.** The grammar captures words as opaque
tokens. Parsing word internals (prosody, CA markers, shortenings, compounds)
requires a separate parser with different recovery semantics.

**Context-sensitive ERROR analysis.** The heuristics in `error_analysis/` that
classify ERROR node text ("@ without form type", "empty replacement", "unknown
annotation marker") require domain knowledge.

### 4.2 Estimated Savings

If Tiers 1-3 were generated:

| Component | Current lines | After generation |
|-----------|--------------|-----------------|
| `node_types.rs` (Tier 1) | 827 | 0 (already generated) |
| Supertype matchers (Tier 2) | ~300 | 0 |
| Sequential traversal + assertions (Tier 2) | ~5,000-7,000 | ~500 (annotation file) |
| Choice dispatch tables (Tier 2) | ~2,000 | 0 |
| Error code mapping (Tier 3) | ~1,000 | ~200 (annotation file) |
| **Total parser reduction** | **~8,000-10,000** | **~700 annotation** |

The parser would shrink from ~21,000 lines to ~11,000-13,000 lines, with the
remaining code being semantic transformation logic (Tier 4) that genuinely
requires human authorship.

### 4.3 Architecture: Generated Scaffolds + Semantic Hooks

The generated and hand-written code would compose as follows:

```
grammar.json + node-types.json + annotations.toml
            |
            v
    [Code Generator]
            |
            v
  generated_traversals.rs       (Tier 2: child extraction, dispatch, loops)
  generated_supertypes.rs       (Tier 2: is_*() functions)
  generated_error_map.rs        (Tier 3: position -> error code)
            |
            | calls hand-written hooks
            v
  semantic_hooks/               (Tier 4: model construction, cross-node logic)
    word_builder.rs             (Word | AnnotatedWord | ReplacedWord)
    overlap_tracker.rs          (overlap matching)
    scope_tracker.rs            (scoped annotation nesting)
    error_analyzers.rs          (context-sensitive ERROR heuristics)
```

The generated code handles the mechanical "walk tree, extract children, skip
whitespace, report unexpected nodes" work. The semantic hooks receive
pre-extracted children and produce model values.

---

## Chapter 5: Error Code Generation from Grammar Context

### 5.1 Current State

The parser defines **198 error codes** (E0xx through E7xx). Many are assigned
manually with hand-written context-specific messages. The assignment is a mix of:

- **Structural errors** derived from grammar positions (E302=MissingNode,
  E305=MissingTerminator, E342=MissingRequiredElement)
- **Semantic errors** from validation logic (E308=UndeclaredSpeaker,
  E501=DuplicateHeader)
- **Domain-specific heuristics** (E202=MissingFormType, detected by examining
  ERROR node text)

### 5.2 The ERROR Node Model

When tree-sitter encounters input that doesn't match any production, it creates
an `ERROR` node. The location of this ERROR tells us which production was being
attempted and where it failed. Tree-sitter's error recovery tries to isolate the
error to the smallest possible subtree.

For the production `main_tier = star + speaker + colon + tab + tier_body`:
- ERROR at position 0: missing `*` --- the line doesn't start with an asterisk
- ERROR at position 1: missing or invalid speaker code
- ERROR at position 2: missing `:` after speaker
- ERROR at position 3: missing tab separator
- ERROR at position 4: malformed tier body

Each position implies a specific error code and message.

### 5.3 The MISSING Node Model

When tree-sitter's error recovery can continue by inserting a placeholder, it
creates a `MISSING` node. A MISSING node has the expected `kind()` (e.g.,
`"terminator"`) but a zero-length span. The `cst_assertions.rs` helpers already
detect these:

```rust
if child.is_missing() {
    errors.report(ParseError::new(
        ErrorCode::MissingRequiredElement,
        Severity::Error,
        SourceLocation::from_offsets(child.start_byte(), child.end_byte()),
        ErrorContext::new(source, child.start_byte()..child.end_byte(), child.kind()),
        format!(
            "Tree-sitter error recovery: MISSING '{}' node inserted at {} position {}",
            expected_kind, context, position
        ),
    ));
}
```

The expected `kind()` is exactly the information needed to generate a specific
error message: "Expected terminator (., ?, !) at end of utterance."

### 5.4 What Can Be Generated

For each production step in grammar.json, a generator could produce a mapping:

```
Rule: main_tier
  Position 0: star (STRING "*")     -> E304 "Missing '*' before speaker code"
  Position 1: speaker (SYMBOL)      -> E304 "Missing speaker code"
  Position 2: colon (STRING ":")    -> E323 "Missing ':' after speaker"
  Position 3: tab (STRING "\t")    -> E302 "Missing tab separator"
  Position 4: tier_body (SYMBOL)   -> E321 "Unparsable utterance content"
```

The error code assignment (E304, E323, etc.) would come from the annotation
file. The message templates could be generated from the grammar structure: "Expected
`{expected_kind}` at position {pos} in `{rule_name}`."

### 5.5 What Cannot Be Generated

**ERROR text heuristics.** The current `error_analysis/utterance.rs` examines
the actual text inside ERROR nodes to produce domain-specific diagnostics:

```rust
// Check for @ without form type (e.g., "hello@")
if error_text.rfind('@').is_some_and(|idx| idx + 1 == error_text.len()) {
    errors.report(ParseError::new(ErrorCode::MissingFormType, ...));
    return;
}

// Check for empty replacement (e.g., "[:]")
if error_text.contains("[:]") {
    errors.report(ParseError::new(ErrorCode::EmptyReplacement, ...));
    return;
}
```

These heuristics encode CHAT-specific domain knowledge: what does a user
probably *mean* when they write `hello@` (they forgot the form type suffix).
This cannot be derived from the grammar.

**Error recovery attachment.** The `attach_error_suffix_to_previous_word()`
function in `contents.rs` glues ERROR fragments to preceding words when the
parser splits tokens like `@x`. This recovery logic is grammar-specific but not
derivable from grammar.json.

### 5.6 Hybrid Approach

The practical approach is layered:

1. **Generated base layer:** For every MISSING node, emit a message based on the
   expected `kind()` and its position in the parent rule. This covers the
   common case.
2. **Annotated refinement layer:** Override specific positions with
   human-authored error codes and messages from the annotation file.
3. **Hand-written heuristic layer:** For ERROR nodes (not MISSING), delegate to
   hand-written analyzers that examine error text and surrounding context.

---

## Chapter 6: Data Model Generation Feasibility

### 6.1 node-types.json as a Type Schema

`node-types.json` provides enough information to generate a "raw" CST-mirror
type for every named node:

**Nodes with named fields** (10 in the grammar) map directly to structs:

```json
{
  "type": "gra_relation",
  "fields": {
    "index":    { "required": true,  "multiple": false, "types": [{"type": "gra_index"}] },
    "head":     { "required": true,  "multiple": false, "types": [{"type": "gra_head"}] },
    "relation": { "required": true,  "multiple": false, "types": [{"type": "gra_relation_name"}] }
  }
}
```

Generated struct:
```rust
struct RawGraRelation<'a> {
    pub index: Node<'a>,          // required, single
    pub head: Node<'a>,           // required, single
    pub relation: Node<'a>,       // required, single
}
```

**Nodes with unnamed children** (143 in the grammar) map to structs with
typed child vectors:

```json
{
  "type": "contents",
  "children": {
    "required": true, "multiple": true,
    "types": [
      {"type": "content_item"}, {"type": "overlap_point"},
      {"type": "separator"}, {"type": "whitespaces"}
    ]
  }
}
```

Generated struct:
```rust
struct RawContents<'a> {
    pub children: Vec<Node<'a>>,  // required, multiple
    // types: content_item | overlap_point | separator | whitespaces
}
```

**Supertype nodes** map to enums:

```rust
enum RawTerminator<'a> {
    Period(Node<'a>),
    Question(Node<'a>),
    Exclamation(Node<'a>),
    TrailingOff(Node<'a>),
    // ... 13 more variants
}
```

### 6.2 The Gap Between CST Mirror and Semantic Model

The semantic model (`talkbank-model`) differs from a CST mirror in several
fundamental ways:

**Semantic renaming.** The grammar uses `standalone_word`; the model uses `Word`.
The grammar uses `word_with_optional_annotations`; the model uses `Word |
AnnotatedWord | ReplacedWord` depending on which optional children are present.

**Flattening and restructuring.** The grammar has deep nesting:
`utterance > main_tier > tier_body > contents > content_item >
word_with_optional_annotations > standalone_word`. The model flattens this to:
`Utterance { content: Vec<UtteranceContent> }` where `UtteranceContent::Word(Box<Word>)`.

**Cross-cutting concerns.** Every model type carries `Span` (byte-range
location). `Utterance` carries `ParseHealth`, alignment metadata, and language
codes. `Word` carries `Vec<WordContent>` (parsed word internals). None of these
exist in the CST.

**Validation invariants.** The model enforces invariants the grammar doesn't: a
`SpeakerId` is validated against `@Participants`, a `LanguageCode` is validated
as ISO 639-3, timestamps are checked for monotonicity.

### 6.3 A Generated CST-Mirror Layer

A practical approach: generate a **raw CST-mirror layer** that sits between
tree-sitter and the semantic model:

```
tree-sitter CST (untyped nodes)
        |
        v
  [Generated] RawCstTypes        <- generated from node-types.json
        |
        v
  [Hand-written] SemanticBuilder  <- transforms raw types into model types
        |
        v
  talkbank-model types           <- semantic model (hand-written, unchanged)
```

The generated layer handles all the "walk tree, extract children, skip
whitespace" work. The semantic builder handles naming, flattening,
restructuring, and cross-cutting concerns.

### 6.4 Conclusion

A full semantic model cannot be generated from grammar artifacts alone. But a
CST-mirror layer can, and it would absorb most of the boilerplate currently in
`talkbank-parser`. The semantic model (`talkbank-model`, ~51,750 lines) would
remain hand-written --- it encodes domain knowledge that is not present in the
grammar.

---

## Chapter 7: The Chumsky Parser: Assessment and Alternatives

### 7.1 Why It Was Written

Tree-sitter is inherently a **whole-document** parser. Its API takes a complete
source string and returns a complete CST. There is no supported way to parse a
fragment (a single `%mor` tier, a single word, a single header) in isolation.

Fragment parsing is needed for:
- **LSP** --- re-parsing a single modified line without re-parsing the entire file
- **Inline validation** --- checking a word or tier value in isolation
- **Programmatic construction** --- building CHAT structures from code
- **batchalign3 hot paths** --- parsing word tokens in the alignment loop

The Chumsky-based direct parser (`talkbank-direct-parser`, ~9,400 lines) was
written to fill this gap.

### 7.2 Architecture

```
crates/talkbank-direct-parser/src/
+-- lib.rs                    # Public API
+-- word.rs                   # Word-internal parser (CA, prosody, shortenings)
+-- main_tier/
|   +-- words.rs              # Word content parser, event parser
|   +-- annotations.rs        # Replacement, scoped annotation parsers
|   +-- groups.rs             # Group, pho_group, sin_group, quotation
+-- mor_tier.rs               # %mor word parsing
+-- pho_tier.rs               # %pho word parsing
+-- gra_tier.rs               # %gra relation parsing
+-- sin_tier.rs               # %sin parsing
+-- wor_tier.rs               # %wor parsing
+-- text_tier.rs              # Generic text tier
+-- dependent_tier/           # Tier dispatch
+-- header/                   # Header parsing (ID, Media, Date, etc.)
+-- file/                     # File-level orchestration
+-- tokens.rs                 # Character classification
+-- whitespace.rs             # Whitespace handling
+-- recovery.rs               # Error recovery
+-- primitives.rs             # Span utilities
```

### 7.3 Current Issues

**Duplication.** The 9,400 lines duplicate logic already encoded in grammar.js
and talkbank-parser. Every grammar change requires updating both parsers.

**No global context.** When the Chumsky word parser runs, it doesn't know
whether the word is inside a quotation, a group, or the main tier. The
tree-sitter grammar handles these contexts differently (e.g., different token
priority in quotation context). The Chumsky parser must make
context-independent decisions, leading to subtle behavioral divergences.

**Error model mismatch.** Chumsky uses `Rich<char>` errors. The TalkBank
toolchain uses `ErrorSink` + `ParseError` with typed `ErrorCode`s. Bridging
these requires conversion code that loses context.

**Divergent behavior.** Parser equivalence tests (`talkbank-parser-tests`)
catch some divergences between the tree-sitter and Chumsky parsers, but not
all. The two parsers may produce different results for edge cases, especially
around error recovery.

**Maintenance burden.** Grammar changes must be reflected in three places:
grammar.js, talkbank-parser (tree-sitter walker), and talkbank-direct-parser
(Chumsky combinators).

### 7.4 Alternatives Assessed

**Option 1: Tree-sitter with synthetic documents.** Wrap a fragment in minimal
valid CHAT structure, parse with tree-sitter, extract the relevant subtree. This
is already partially implemented (`synthetic_fragments.rs`, `single_item/`).

- Pro: Reuses the grammar exactly. No behavioral divergence.
- Pro: Already partially working.
- Con: Overhead of constructing wrapper text and re-parsing.
- Con: Span offsets must be adjusted (the fragment starts at an offset within
  the synthetic document).
- Con: The synthetic wrapper must be valid CHAT, which means knowing which
  speaker to use, which headers to include, etc.

**Option 2: Tree-sitter multi-root grammar.** Change the grammar root to
`choice(document, mor_contents, gra_contents, standalone_word, ...)`.

- Pro: Single grammar, single parser.
- Con: Pollutes the LR parse table. Each alternative root adds states.
- Con: May introduce new conflicts (the grammar's 5 existing conflicts are
  carefully tuned for document-mode parsing).
- Con: Confuses editor integration (tree-sitter expects the first rule to be
  the document root).

**Option 3: Generated standalone Rust parsers.** Use grammar.json to generate
hand-rolled recursive descent parsers in plain Rust for each fragment type.

- Pro: No library dependency (no Chumsky, no tree-sitter C parser).
- Pro: Fast compilation, `#[inline]`-able, no thread-local state.
- Pro: Grammar-exact semantics (generated from the same source of truth).
- Con: Significant generator engineering effort.
- Con: Recursive descent may not handle all GLR ambiguities (but fragments are
  typically unambiguous subtrees).

**Option 4: Keep Chumsky, generate from grammar.json.** Translate grammar.json
SEQ/CHOICE/REPEAT into Chumsky combinators automatically.

- Pro: Reuses existing Chumsky infrastructure.
- Con: Chumsky's context-free nature remains (no global context).
- Con: Error model mismatch persists.
- Con: Chumsky compilation times are notoriously slow for complex grammars.

### 7.5 Recommendation

**Primary path: Option 1** (synthetic documents) for most fragment parsing use
cases. The tree-sitter parser is fast, correct by construction (uses the same
grammar), and the synthetic wrapper approach is already partially implemented.
The span offset adjustment is a solved problem.

**Performance-critical path: Option 3** (generated standalone) for the small set
of fragment types that appear in hot loops:
- `mor_word` parsing in batchalign3 alignment
- `gra_relation` triple parsing
- `standalone_word` re-parsing

These are small, unambiguous, leaf-level grammars that map directly to recursive
descent.

**Deprecation path for Chumsky:** As synthetic document support matures and
standalone parsers are generated for hot paths, the Chumsky parser can be
incrementally removed. Parser equivalence tests would gate each removal step.

---

## Chapter 8: Fragment Parsing Without tree-sitter

### 8.1 Fragment Use Cases

The specific fragments that need standalone parsing:

| Fragment | Example | Used by |
|----------|---------|---------|
| `%mor` word | `v\|want` | batchalign3 alignment |
| `%gra` relation | `1\|2\|SUBJ` | batchalign3, tier validation |
| `%pho` word | `w aa n t` | phonology alignment |
| `%sin` word | `POINT:obj` | signed language alignment |
| Word internal | `want@b` → prosody, form types | word validation |
| Header fields | `01-JAN-2000` | header validation |

### 8.2 Why tree-sitter Is Problematic for Fragments

Tree-sitter's design assumes whole-document parsing:

1. **Single root rule.** The grammar has one entry point (`document`). There is
   no supported way to start parsing at `mor_word` or `gra_relation`.

2. **Global lexer state.** The DFA tokenizer is built for the complete grammar.
   Token priorities (prec levels) are resolved globally. A `standalone_word`
   token at prec(5) beats `event_segment` at prec(1) --- but this priority only
   makes sense in the context of a full utterance, not an isolated fragment.

3. **GLR conflict resolution.** The 5 declared conflicts are resolved by
   tree-sitter's GLR machinery with document-level lookahead. Fragment parsing
   loses this context.

4. **Thread-local parser.** The tree-sitter parser uses a thread-local
   `RefCell<Parser>` to avoid re-initialization overhead. Fragment parsing in a
   hot loop pays the `RefCell` borrow overhead per call.

### 8.3 grammar.json as Recursive Descent Source

For unambiguous fragment grammars, grammar.json rules map directly to recursive
descent Rust code:

| grammar.json | Generated Rust |
|-------------|----------------|
| `SEQ { members: [A, B, C] }` | `let a = parse_a(input)?; let b = parse_b(input)?; let c = parse_c(input)?;` |
| `CHOICE { members: [A, B] }` | `if let Ok(a) = parse_a(input) { return Ok(a); } parse_b(input)` |
| `REPEAT { content: X }` | `let mut items = Vec::new(); while let Ok(x) = parse_x(input) { items.push(x); }` |
| `REPEAT1 { content: X }` | `let first = parse_x(input)?; let mut items = vec![first]; while let Ok(x) = parse_x(input) { items.push(x); }` |
| `STRING { value: "xyz" }` | `if input.starts_with("xyz") { input = &input[3..]; Ok(()) } else { Err(...) }` |
| `PATTERN { value: "regex" }` | `static RE: LazyLock<Regex> = ...; match RE.find(input) { ... }` |
| `TOKEN { content: X }` | Flatten X into a single atomic regex match |

The PREC/PREC_LEFT/PREC_RIGHT wrappers are irrelevant for unambiguous fragments
--- they only affect tree-sitter's LR table generation. A recursive descent
parser resolves ambiguity by try-order.

### 8.4 Which Fragments to Generate vs. Hand-Write

**Generate standalone (simple, hot-path):**
- `%mor` word: `SEQ(mor_pos, "|", mor_lemma, REPEAT("-", mor_feature))`
- `%gra` triple: `SEQ(gra_index, "|", gra_head, "|", gra_relation_name)`
- `%pho` word: whitespace-delimited IPA segments
- Header fields: the 10 strict+catch-all patterns are small enough to generate
  as standalone validators

**Keep hand-written (complex, many special cases):**
- Word-internal parsing: prosody markers, CA features, shortenings,
  compounds, form/language suffixes. The word parser handles 13+ `WordContent`
  variants with complex interaction rules. This is 700+ lines of logic that
  cannot be derived from the grammar (words are opaque tokens in the grammar).

**Keep synthetic (full-document context needed):**
- Full utterance re-parsing in LSP
- Dependent tier parsing where tier-type dispatch matters
- Any fragment where `@Options: CA` mode affects parsing

### 8.5 Advantages of Standalone Parsers

1. **No C dependency.** The generated parser is pure Rust --- no `parser.c`, no
   `tree_sitter::Parser`, no FFI.
2. **No thread-local state.** Each parse call is a pure function with no shared
   mutable state.
3. **Inlineable.** Small fragment parsers can be `#[inline]` for hot loops.
4. **Exact grammar semantics.** Generated from the same grammar.json, so the
   token definitions match the tree-sitter parser exactly.
5. **Fast compilation.** No Chumsky macro expansion. Plain function calls.

---

## Chapter 9: Test Generation from Grammar

### 9.1 Grammar as Test Specification

Each production rule in grammar.json implies a set of valid and invalid inputs.
A generator can use the grammar structure to produce test cases automatically.

### 9.2 Categories of Generatable Tests

**Positive structural tests.** For each SEQ rule, generate minimal valid input
matching each child in sequence:

```
Rule: date_header = date_prefix + header_sep + date_contents + newline
Test: "@Date:\t01-JAN-2000\n"
```

For each CHOICE rule, generate one test per alternative:

```
Rule: terminator = period | question | exclamation | ...
Tests: ".", "?", "!", "+...", "+/.", "+//.", ...
```

**Negative structural tests.** For each required child in a SEQ, generate input
missing that child:

```
Rule: main_tier = star + speaker + colon + tab + tier_body
Tests:
  - Missing star: "CHI:\thello ."          -> expect E304
  - Missing speaker: "*:\thello ."         -> expect E304
  - Missing colon: "*CHI\thello ."         -> expect E323
  - Missing tab: "*CHI:hello ."            -> expect E302
```

**Boundary tests.** For each TOKEN rule with a regex, generate edge-case inputs:

```
Rule: standalone_word = /[regex]/
Tests:
  - Minimal: single valid character
  - With all special characters (prosody, CA, etc.)
  - Just outside the character class
```

**Supertype completeness tests.** For each supertype, verify that every subtype
listed in node-types.json actually produces a node of that type when parsed:

```
For each subtype S in terminator.subtypes:
  Parse a document with S as the terminator
  Assert the CST contains a node of kind S
  Assert its parent is a terminator supertype
```

**Roundtrip tests.** For each generated positive test, verify that
parse -> serialize -> re-parse produces an identical CST.

### 9.3 Integration with Existing Test Infrastructure

The TalkBank project already has spec-driven testing:
- `spec/constructs/` --- valid CHAT examples
- `spec/errors/` --- invalid CHAT examples with expected error codes
- `make test-gen` regenerates tests from specs

Generated grammar-structural tests would **complement** spec tests:
- Spec tests encode CHAT-manual semantics (human-authored, domain-specific)
- Grammar tests encode structural coverage (machine-generated, exhaustive)

A new `make grammar-tests` target could generate structural test fixtures into
`crates/talkbank-parser-tests/tests/generated/grammar/`.

### 9.4 Implementation Path

Extend `tree-sitter-grammar-utils` with a `test-gen` command:

```
tree-sitter-grammar-utils test-gen \
  --grammar grammar/src/grammar.json \
  --node-types grammar/src/node-types.json \
  --output crates/talkbank-parser-tests/tests/generated/grammar/
```

The command would:
1. Read grammar.json to understand rule structure
2. For each SEQ rule, generate a minimal valid input by recursively
   instantiating each child (using STRING/PATTERN values as leaf content)
3. For each CHOICE rule, generate one test per alternative
4. For each required child, generate a negative test with that child missing
5. Output as tree-sitter-format test files or as Rust test functions

---

## Chapter 10: The Generator Architecture

### 10.1 Tool Placement

Two options:

**Option A: Extend tree-sitter-grammar-utils.** Add a `codegen` subcommand.
Pro: reuses existing infrastructure (dependency graph, oxc parser). Con: mixes
concerns (grammar analysis vs. code generation).

**Option B: Create a sibling tool** (`tree-sitter-parser-gen` or
`talkbank-parser-gen`). Pro: clean separation. Con: some code duplication.

Recommendation: **Option B** for the initial implementation. The code generator
reads grammar.json (simple JSON, no oxc needed), not grammar.js. It is a
fundamentally different tool from the grammar-utils which manipulate JavaScript
source.

### 10.2 Inputs

1. **grammar.json** --- production structure (what children, in what order, with
   what combinators)
2. **node-types.json** --- child cardinality (required/optional/multiple), field
   names, subtypes
3. **annotations.toml** --- human-authored mappings from grammar rules to:
   - Rust handler function names
   - Model type names
   - Error codes per position
   - Recovery strategy overrides
   - Skip/delegate markers (e.g., "this is an opaque token, parse downstream")

### 10.3 Outputs

| File | Content | Replaces |
|------|---------|----------|
| `generated_node_types.rs` | String constants for all 361 node types | `node_types.rs` (already exists) |
| `generated_supertypes.rs` | `is_*()` functions from subtypes arrays | 7 supertype matcher files |
| `generated_traversals.rs` | Child extraction structs + traversal functions for each SEQ/CHOICE rule | ~50+ hand-written traversal functions |
| `generated_error_map.rs` | Grammar-position to error-code mapping | Scattered `ErrorCode::*` assignments |
| `generated_tests.rs` | Structural test cases | (new) |

### 10.4 Annotation File Design

The annotation file bridges grammar structure and semantic intent:

```toml
# annotations.toml

[meta]
# Which crate this generates for
target_crate = "talkbank-parser"
# Import path for error types
error_import = "crate::error::{ErrorCode, ErrorSink, ParseError}"
# Import path for model types
model_import = "crate::model"

# --- Supertype overrides ---

[supertypes.ca_element]
# Not in grammar supertypes, but parser uses it
subtypes = ["ca_breathy_voice", "ca_creaky", "ca_laughing", "..."]

[supertypes.overlap_point_marker]
subtypes = ["overlap_precedes_marker", "overlap_follows_marker", "..."]

# --- Rule-specific annotations ---

[rules.word_with_optional_annotations]
handler = "build_word_content"
model_type = "UtteranceContent"
skip_whitespace = true

[rules.word_with_optional_annotations.children.word]
# standalone_word -> delegate to word-internal parser
delegate = "convert_word_node"
missing_error = "E302"

[rules.word_with_optional_annotations.children.replacement]
handler = "parse_replacement"
# optional, no error code needed for absence

[rules.word_with_optional_annotations.children.annotations]
handler = "parse_scoped_annotations"

# --- Opaque token markers ---

[rules.standalone_word]
opaque = true  # don't generate traversal; this is a leaf token parsed downstream

[rules.inline_bullet]
opaque = true

# --- Simple leaf rules (no traversal needed) ---

[rules.period]
leaf = true  # STRING "." - just extract text

[rules.trailing_off]
leaf = true  # TOKEN "+..." - just extract text
```

### 10.5 Generation Algorithm

For each rule in grammar.json:

1. **Check annotations.** If marked `opaque` or `leaf`, skip traversal
   generation.

2. **Classify rule type** from the top-level `type` field:
   - SEQ → generate child extraction struct + traversal function
   - CHOICE → generate dispatch function with `match child.kind()`
   - REPEAT/REPEAT1 → generate collection loop
   - TOKEN → leaf (no traversal)
   - STRING/PATTERN → leaf (no traversal)

3. **For SEQ rules**, enumerate members:
   - For each SYMBOL member: generate positional child access
   - For each optional member (CHOICE with BLANK): generate `Option<Node>`
   - For each REPEAT member: generate `Vec<Node>`
   - For `whitespaces` members: generate skip logic
   - Look up annotation for handler/error code/delegate

4. **For CHOICE rules**, enumerate alternatives:
   - Generate `match child.kind()` with one arm per alternative
   - Each arm calls the alternative's handler (from annotation or generated)
   - Generate `_ =>` catch-all with `unexpected_node_error()`

5. **Emit Rust code** as a string, formatted with `prettyplease` or similar.

### 10.6 Verification

The generator's output must pass the existing verification gates:

```bash
# After regeneration:
cargo nextest run -p talkbank-parser                    # Unit tests
cargo nextest run -p talkbank-parser-tests              # Equivalence tests
cargo nextest run -p talkbank-parser-tests \
  --test roundtrip_reference_corpus                     # 74-file roundtrip
make verify                                             # Full gate suite
```

A `make regen-parser` target would:
1. Run the generator
2. Format output with `cargo fmt`
3. Run verification gates
4. Report any failures

---

## Chapter 11: Succession and Maintenance Implications

### 11.1 How Grammar-Driven Generation Supports Succession

The succession mandate requires every system to be operable by someone who has
never met the team. Grammar-driven generation directly supports this:

**Grammar changes propagate automatically.** When a new header or tier type is
added to grammar.js, `tree-sitter generate` updates grammar.json and
node-types.json. The code generator then produces updated traversal code,
supertype matchers, and structural tests. The contributor only needs to:
1. Edit grammar.js (add the new rule)
2. Add an annotation entry (map to handler + error codes)
3. Write the semantic hook (model construction logic)
4. Run `make regen-parser` and fix any failures

They do **not** need to:
- Understand the 21K-line parser crate's internal organization
- Know which files to modify for child extraction, dispatch, assertions
- Manually update supertype matcher functions
- Write structural tests for the new construct

**Error codes are traceable.** The annotation file provides a single source of
truth for "which error code fires at which grammar position." A newcomer can
search the annotation file to find where E305 (MissingTerminator) is assigned
and trace it back to the grammar rule.

**Test coverage tracks grammar coverage.** Generated structural tests ensure
that every grammar rule has at least one positive and one negative test. Gaps in
test coverage become gaps in grammar coverage --- visible and measurable.

### 11.2 Risk Assessment

**Generator becomes a maintenance burden.** Mitigated by: the generator reads
grammar.json (stable JSON format with an official schema) and node-types.json
(stable, versioned). It does not parse JavaScript. Changes to tree-sitter's
internal IR do not affect it.

**Annotation file drifts from grammar.** Mitigated by: the `make regen-parser`
target validates that every rule referenced in annotations.toml exists in
grammar.json, and every rule in grammar.json either has an annotation or is
marked as a leaf/opaque token.

**Generated code is less readable.** Mitigated by: keeping semantic hooks
hand-written and well-documented. The generated code is mechanical scaffolding
--- a contributor should never need to read or debug it. If they do, the
generation is structured (one function per rule) and the output is formatted.

**Grammar.json format changes.** Mitigated by: grammar.json has an official
JSON Schema (`https://tree-sitter.github.io/tree-sitter/assets/schemas/grammar.schema.json`)
and the format is effectively frozen --- it is tree-sitter's public contract.

### 11.3 Phased Adoption

The transition from hand-written to generated parser should be incremental, with
each step verified by the existing test suite:

**Phase 1: Generate supertypes.** Replace the 7 hand-written supertype matcher
files with generated code. This is the smallest change with the lowest risk ---
the generated `is_*()` functions are trivial `matches!()` expressions.

Verification: all existing tests pass unchanged.

**Phase 2: Generate child count assertions.** For each SEQ rule, generate the
`assert_child_count_*()` calls that currently scatter through the codebase.

Verification: parser equivalence tests + reference corpus roundtrip.

**Phase 3: Generate traversal scaffolds for simple rules.** Start with rules
that have no semantic hooks --- headers with fixed structure, simple dependent
tiers, leaf-level constructs.

Verification: parser equivalence for affected rule types.

**Phase 4: Generate traversal scaffolds for complex rules.** Move to rules with
semantic hooks (word content, group content, annotation nesting). The generated
code calls hand-written hooks.

Verification: full verification gate suite.

**Phase 5: Deprecate Chumsky parser.** As synthetic document support covers all
fragment use cases and standalone parsers handle hot paths, remove
talkbank-direct-parser.

Verification: parser equivalence tests confirm no behavioral regression.

### 11.4 What Remains Hand-Written

After full adoption, approximately **11,000-13,000 lines** of hand-written
parser code would remain:

- Semantic hooks (model construction from raw CST children)
- Cross-node coordination (overlap, scoped annotations, quotation, gems)
- Error analysis heuristics (context-sensitive ERROR classification)
- Word-internal parsing (13+ WordContent variants, prosody, CA, shortenings)
- File-level orchestration (parse flow, participant resolution, header ordering)
- The public API surface (`parser_api.rs`, fragment helpers)

This is the code that genuinely requires human understanding of the CHAT format.
Everything else is mechanical traversal that the grammar already specifies.

---

## Appendix A: Key File Paths

| File | Purpose | Lines |
|------|---------|-------|
| `grammar/grammar.js` | Grammar definition | 2,045 |
| `grammar/src/grammar.json` | Compiled grammar IR | 5,886 |
| `grammar/src/node-types.json` | CST type schema | 5,218 |
| `grammar/GRAMMAR.md` | Grammar architecture guide | 165 |
| `crates/talkbank-parser/` | Tree-sitter-based parser | 20,978 |
| `crates/talkbank-direct-parser/` | Chumsky-based alternative | 9,443 |
| `crates/talkbank-model/` | Data model + validation | 51,750 |
| `crates/talkbank-parser/src/node_types.rs` | Generated constants | 827 |
| `crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs` | CST helpers | 352 |
| `crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/supertypes/` | Supertype matchers | ~300 |
| `crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/` | ERROR classifiers | ~250 |
| `~/tree-sitter-grammar-utils/` | Grammar analysis toolkit | ~2,000 |

## Appendix B: Quantitative Summary

| Metric | Value |
|--------|-------|
| Grammar rules | 367 |
| Named node types | 361 |
| Grammar supertypes | 6 (+ 3 parser-defined) |
| SEQ rules | 116 |
| CHOICE rules | 28 |
| TOKEN rules | 140 |
| Rules with named fields | 10 |
| Rules with unnamed children | 143 |
| Strict+catch-all patterns | 10 |
| Conflict declarations | 5 |
| Error codes | 198 |
| Parser child-traversal loops | 22 |
| Parser kind-dispatch blocks | 29 |
| Parser expect_child calls | 32 |
| Parser whitespace-skip arms | 19 |
| Parser unexpected-node reports | 50 |
| Parser rejection points | 169 |
| Estimated generatable lines | 8,000-10,000 |
| Estimated remaining hand-written | 11,000-13,000 |

---

## Appendix C: Implementation Notes (updated as work progresses)

### C.1: Code Generation Approach — `quote!` + `prettyplease` (2026-03-22)

The initial codegen in `tree-sitter-node-types/codegen.rs` used raw `write!`/
`writeln!` string building. This was refactored to use:

- **`quote!`** (proc-macro2) for structured Rust token generation
- **`prettyplease`** for deterministic, idiomatic formatting of the output

This was done proactively before the codebase grew. The `quote!` approach:
- Eliminates brace-escaping bugs (`{{` vs `{` in format strings)
- Makes the generated code structure visible in the generator source
- Catches syntax errors at generation time via `syn::parse2()`
- Produces consistently formatted output regardless of generator code style

The error type changed from `std::fmt::Error` to `CodegenError(syn::Error)`.

### C.2: Supertype Predicate Parity Validation (2026-03-22)

Generated `is_*()` predicates from CHAT's `node-types.json` were compared
against talkbank-parser's hand-written supertype matchers:

| Supertype | Generated kinds | Hand-written kinds | Match? |
|-----------|----------------|-------------------|--------|
| `terminator` | 18 | 18 | Exact |
| `linker` | 8 | 8 | Exact |
| `base_annotation` | 19 | 19 | Exact |
| `dependent_tier` | 32 | 32 | Exact |
| `pre_begin_header` | 5 | 5 | Exact |
| `header` | 35 | 38 | Hand-written adds 3 extras |

The `is_header()` hand-written version includes `begin_header`, `end_header`,
and `utf8_header` which are NOT grammar supertypes — they are separate
document-structure nodes. The generated version is grammar-faithful. The
hand-written version encodes CHAT-specific domain knowledge that would be
handled via annotations in the generated pipeline.

### C.3: Rule Shape Analysis — Flattening grammar.json (2026-03-22)

The `rule_shape` module in `tree-sitter-grammar-schema` computes "effective
child sequences" from grammar.json rules. Key design decisions:

**Why flatten?** Grammar.json rules are deeply nested combinator trees.
A traversal generator needs a linear answer: "what children, in what order,
with what cardinality." The `RuleShape` enum provides this.

**Why both grammar.json AND node-types.json?** They carry complementary
information:
- grammar.json: production ORDER (which child comes first) and STRUCTURE
  (SEQ vs CHOICE vs REPEAT)
- node-types.json: TYPES (what node kinds can fill each position) and
  CARDINALITY (required/optional/multiple)

Both are needed. grammar.json alone doesn't know about field cardinality.
node-types.json alone doesn't know about production order.

**CHAT rule distribution after flattening:**

| Top-level shape | Count | Notes |
|-----------------|-------|-------|
| Terminal (TOKEN/STRING/PATTERN) | 212 | No children — leaf extraction only |
| Seq (SEQ) | 116 | Main target for traversal generation |
| Choice (CHOICE) | 30 | Dispatch tables |
| Repeat1 | 8 | Collection loops |
| Symbol | 1 | Alias |

The 116 SEQ rules are where the hand-written parser has the most boilerplate
and where generated traversals will have the most impact.

### C.4: Grammar-Utils Workspace Architecture (2026-03-22)

The `tree-sitter-grammar-utils` project was restructured into a 3-crate
workspace before this implementation work began:

```mermaid
flowchart TD
    subgraph "tree-sitter-grammar-utils workspace"
        schema["tree-sitter-grammar-schema\n(grammar.json types + rule_shape analysis)"]
        nodetypes["tree-sitter-node-types\n(node-types.json types + codegen)"]
        cli["CLI crate\n(prune, extract, audit, optimize,\nminimize, diff, codegen)"]
    end

    grammarjs["grammar.js\n(JS source)"] -->|"oxc parse"| cli
    grammarjson["grammar.json\n(serde)"| -->|"deserialize"| schema
    nodetypesjson["node-types.json\n(serde)"| -->|"deserialize"| nodetypes

    schema -->|"dep"| nodetypes
    schema -->|"dep"| cli
    nodetypes -->|"dep"| cli

    nodetypes -->|"generate_rust()"| generated["Generated Rust\n(structs, enums, TryFrom,\nis_*() predicates, Visitor)"]
```

<!-- Verified against: Cargo.toml (workspace members), crates/*/Cargo.toml (deps),
     crates/tree-sitter-node-types/src/codegen.rs, crates/tree-sitter-grammar-schema/src/lib.rs -->

The dual-level architecture (JS via oxc for source manipulation, JSON via serde
for analysis/codegen) was the resolution of the "oxc breakage" problem
identified in the research phase. New codegen work operates entirely on the
JSON level — no oxc dependency.

### C.5: Traversal Generation Results (2026-03-22)

The traversal generator (`tree-sitter-node-types/traversal.rs`) was tested
against the complete CHAT grammar:

| Metric | Value |
|--------|-------|
| SEQ rules processed | 116 |
| Structs generated (`RawFooChildren`) | 116 |
| Extraction functions generated | 116 |
| Total generated lines (valid Rust) | 21,333 |
| Whitespace skip kind configured | `"whitespaces"` |

Each generated extraction function:
- Iterates children in production order (from grammar.json SEQ members)
- Skips `whitespaces` nodes (configured via `TraversalConfig`)
- Detects MISSING nodes (tree-sitter error recovery placeholders)
- Detects ERROR nodes (tree-sitter parse failures)
- Reports unexpected children via generic `TraversalErrors` trait
- Populates `Option<Node>` fields (all fields are Option because
  required children may still be absent in error-recovery CSTs)

The 21,333 lines of generated traversal code correspond to the ~21,000 lines
of hand-written parser code in `talkbank-parser`. The generated code handles
the mechanical parts (child extraction, whitespace skipping, error detection);
the semantic parts (model construction, cross-node coordination) remain
hand-written as hooks.

### C.6: Trait-Based Architecture — Rejecting Annotation-Driven Actions (2026-03-22)

The initial design proposed a TOML annotation file mapping grammar rules to
handler functions, error codes, and codegen options. This was rejected in favor
of a **trait-based architecture** where the generated code produces a Rust trait
(`SemanticHandler`) and the user implements it.

**Why the annotation approach was wrong:**
- Annotations are a shadow of the code that drifts from reality
- They embed "what to do" in a config file — separating concerns incorrectly
- The compiler can't check that an annotation references a real function
- Adding a new grammar rule requires updating both grammar.js and the annotation
  file, but only the first is enforced

**Why the trait approach is right:**
- The trait IS the interface. The compiler enforces the contract.
- Adding a new SEQ rule to the grammar automatically adds a method to the
  generated trait. If the user's impl is missing the method, it doesn't compile.
- Each trait method's doc comment carries the grammar context: what children
  appear, which are required/optional, which have field names. The implementor
  knows exactly what they're working with.
- No configuration file to maintain. No "actions" embedded in data.

**Annotations remain as optional enrichment** — TOML files can provide error
codes and human-friendly names for generated doc comments. They are not required
for code generation to work.

The generated architecture is now:

```text
grammar.json ──→ rule_shape() ──→ RuleShape::Seq(slots)
                                         │
                                         ▼
                               ┌── FooNode typed wrappers
                               ├── FooChildren struct (NodeSlot fields)
                               └── GrammarTraversal trait (extract methods)
                                         │
                                         ▼
                               User's impl overrides where needed
```

### C.7: Faithful Model — `NodeSlot` Replacing Error Callbacks (2026-03-22)

The per-position error callback approach (hundreds of `on_{rule}_missing_{child}`
methods) was replaced with a **faithful data model**: `NodeSlot<T>` enum that
represents ALL five possibilities at every child position.

```rust
enum NodeSlot<'tree, T> {
    Present(T),                      // valid, kind verified
    Missing(T),                      // MISSING placeholder (kind correct, zero-length)
    Error(tree_sitter::Node<'tree>), // ERROR node (untyped)
    Unexpected(tree_sitter::Node<'tree>), // wrong kind
    Absent,                          // child list too short
}
```

**Why this is more principled than callbacks:**
- No information loss — the struct carries the complete picture
- No temporal coupling — errors aren't reported during extraction; the handler
  decides what to report after seeing all children
- No string matching — the handler pattern-matches typed variants, not
  `parent_kind: &str`
- MISSING nodes are naturally modeled — they have the expected `kind()` so
  the typed wrapper `T` is valid. `Missing(SpeakerNode)` IS a `SpeakerNode`
  with zero-length span.

**Why ERROR nodes are untyped:** ERROR nodes have `kind() == "ERROR"` regardless
of position. They can't be wrapped in a typed `SpeakerNode` because the kind
doesn't match. The `Node` is left raw for domain-specific inspection.

**CHAT integration test results:**
- 281 structs generated (165 typed wrappers + 116 children structs)
- 116 extraction methods in one `GrammarTraversal` trait
- 12,175 lines of valid Rust (down from 20,771 with callbacks — the error
  methods were a significant portion)
- Zero error callback methods — all error information is in `NodeSlot`

### C.8: First Parity Test — Generated Traversal on Real CHAT Data (2026-03-22)

The generated traversal code was wired into `talkbank-parser-tests` and tested
against the 74-file reference corpus:

1. Generated `generated_traversal.rs` (12,175 lines) via the `generate_traversal`
   example binary in tree-sitter-grammar-utils
2. Added `tree-sitter-node-types` as a cross-workspace path dependency
3. Wrote `generated_traversal_parity.rs` integration test

Results:
- **74 files** parsed from `corpus/reference/`
- **297 main_tier nodes** found and extracted
- **297/297** (100%) had `NodeSlot::Present` for all 5 required children
  (star, speaker, colon, tab, tier_body)
- **Speaker text verified** — `SpeakerNode::text(source)` returns correct speaker
  codes (e.g., `"CHI"`)
- The `GrammarTraversal` trait works with a zero-line implementation (`struct
  TestTraversal; impl GrammarTraversal for TestTraversal {}`) — all extraction
  methods use generated defaults

This validates the entire pipeline: grammar.json → rule_shape → codegen →
typed wrappers → NodeSlot fields → extraction methods → real CHAT CST data.

### C.9: Full 116-Method Coverage Test (2026-03-22)

All 116 generated extraction methods were exercised on every matching node
in the 74-file reference corpus:

| Metric | Value |
|--------|-------|
| Files parsed | 74 |
| Total CST nodes | 37,331 |
| Nodes extracted by generated methods | 9,096 |
| Unique rule kinds exercised | 106 of 116 |
| Panics | 0 |
| Speaker parity (generated vs hand-written) | 297/297 (100%) |

The 10 unexercised rule kinds are rare constructs not present in the reference
corpus (e.g., `thumbnail_header`, `sin_groups`, `window_header`). Every rule
kind that appears in real data was successfully extracted.

Top extracted node types:
- `word_with_optional_annotations`: 1,499
- `mor_feature`: 1,270
- `gra_relation`: 831
- `mor_word`: 720
- `main_tier`/`utterance`/`tier_body`/`utterance_end`: 297 each
- `header_sep`: 505
- `tier_sep`: 298

The generated traversal pipeline handles the full complexity of the CHAT
grammar on production data without any failures.

### C.10: Child Role Classification — Auto-Inferring Structural vs. Payload (2026-03-22)

A new `child_role` module in `tree-sitter-grammar-schema` auto-classifies each
child in every SEQ rule as **structural** (discardable delimiter) or **payload**
(semantically meaningful). The classification is fully language-agnostic — it
uses universal heuristics that work on any tree-sitter grammar:

| Heuristic | Signal | Classification |
|-----------|--------|---------------|
| Rule type | `STRING { value }` → literal token | Structural |
| Regex pattern | `[\r\n]+`, `\s+` → whitespace | Structural |
| Naming suffix | `*_prefix`, `*_sep` | Structural |
| Naming suffix | `*_contents`, `*_body`, `*_word` | Payload |
| Named field | `FIELD { name, ... }` | Payload (always) |
| Fan-in | Referenced by >20 SEQ rules | Structural (generic) |
| Regex analysis | `\d+` → Integer, `[^\r\n]+` → FreeText | Type inference |
| CHOICE structure | `[STRING "a", STRING "b", PATTERN ".*"]` | ValidatedEnum |
| TOKEN wrapper | `TOKEN(...)` | OpaqueToken |

Results on CHAT grammar (116 SEQ rules, 409 total children):

| Classification | Count | Percentage |
|---------------|-------|-----------|
| Structural | 294 | 72% |
| Payload | 115 | 28% |

Payload type breakdown:

| Inferred type | Count | What it means |
|---------------|-------|--------------|
| NestedRule | 70 | Reference to another grammar rule (recurse) |
| FreeText | 28 | Extract text directly |
| OpaqueToken | 10 | Delegate to another parser |
| ValidatedEnum | 4 | Strict+catch-all pattern (auto-detected) |
| Integer | 3 | `\d+` pattern |
| Unknown | 0 | — |

**Zero "Unknown" classifications** — the heuristics cover 100% of CHAT's
children without any annotations. The 4 auto-detected ValidatedEnum instances
are the strict+catch-all patterns documented in GRAMMAR.md.

This means a semantic conversion generator could:
1. **Skip** 72% of children (structural)
2. **Extract text** for 28 payload children
3. **Recurse** into 70 nested rules
4. **Generate validation enums** for 4 strict+catch-all fields
5. **Mark 10 as delegated** to word-internal parser (OpaqueToken)
6. **Parse 3 as integers** from `\d+` patterns

Annotations would only be needed to REFINE these defaults — e.g., specifying
that a `FreeText` field should be parsed as a `LanguageCode` (ISO 639-3) rather
than a raw `String`, or that an `OpaqueToken` should delegate to a specific
parser function.

### C.11: Payload Structs — Semantic View Generation (2026-03-22)

The child_role classification now drives code generation of **payload-only
structs** that contain only the semantically meaningful children, with
structural delimiters stripped.

Generated examples from CHAT grammar:

```rust
// 4 children → 2 payload fields (prefix, sep, newline stripped)
pub struct MainTierPayload<'tree> {
    pub speaker: Option<SpeakerNode<'tree>>,       // free text
    pub child_4: Option<TierBodyNode<'tree>>,      // nested rule
}

// 4 children → 1 payload field (prefix, sep, newline stripped)
pub struct DateHeaderPayload<'tree> {
    pub child_2: Option<DateContentsNode<'tree>>,  // nested rule
}

// 5 children → 3 payload fields (pipe separators stripped)
pub struct GraRelationPayload<'tree> {
    pub index: Option<GraIndexNode<'tree>>,        // integer
    pub head: Option<GraHeadNode<'tree>>,           // integer
    pub relation: Option<GraRelationNameNode<'tree>>, // free text
}
```

Total: 361 structs generated (165 typed wrappers + 116 full NodeSlot children
+ 80 payload structs), 12,747 lines, all valid Rust.

The payload structs are the **semantic view** of the grammar — what a consumer
cares about. Combined with the `GrammarTraversal` trait (which provides the
full NodeSlot extraction), consumers can choose their level of detail:
- **Full extraction**: use `FooChildren` for error recovery and detailed analysis
- **Semantic view**: use `FooPayload` for domain conversion (fewer fields to handle)
