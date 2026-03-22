# Semantic Layer Design: Declarative vs. Procedural Transformation

**Status:** Draft — open question for further review
**Last updated:** 2026-03-22 19:12 EDT

## Context

The grammar-driven traversal generator (`tree-sitter-grammar-utils`) successfully
replaces the **extraction layer** of the hand-written CHAT parser — the ~8,000
lines of child-indexed traversal boilerplate. What remains is the **semantic
layer** — the ~11,000 lines that transform raw CST nodes into `talkbank-model`
domain types.

The question: how much of the semantic layer can be replaced by declarative
mappings, generated code, or pattern-based transformations — and how much must
remain hand-written Rust?

## What the Extraction Layer Replaced

The generated `GrammarTraversal` trait handles:
- Child-indexed sequential traversal (`idx: u32`, `while idx < child_count`)
- Kind-based dispatch (`match child.kind()`)
- Whitespace skipping (`WHITESPACES => { idx += 1; }`)
- MISSING node detection and reporting
- ERROR node detection
- Unexpected child reporting
- Positional type verification (via `NodeSlot<TypedNode>`)

This is purely mechanical — derived entirely from grammar.json production
structure. The generated code is validated against the 74-file reference corpus
with zero panics and 100% speaker parity.

## What the Semantic Layer Does

The remaining hand-written code performs **transformations** from CST nodes to
domain types. These transformations fall into distinct complexity tiers:

### Tier 1: Direct Mapping (~30% of semantic code)

Simple 1:1 mappings where the domain type is essentially the CST node's text
with a newtype wrapper or validation:

```
SpeakerNode.text()        → SpeakerCode (interned string)
DateContentsNode.text()   → date string (validated format)
LanguageCodeNode.text()   → LanguageCode (ISO 639-3 validated)
MorPosNode.text()         → POS tag string
GraIndexNode.text()       → u32 (parsed integer)
OptionNameNode.text()     → ChatOptionFlag (enum from_text)
```

These are highly automatable. The child_role analysis already infers `Integer`,
`FreeText`, `ValidatedEnum`, and `OpaqueToken` types for each payload child.
A generator could produce `from_text()` → newtype conversions for all of these.

### Tier 2: Conditional Variant Selection (~25% of semantic code)

The CST has a single rule (`word_with_optional_annotations`) but the domain
model has three types depending on which optional children are present:

```
word only                    → UtteranceContent::Word(Box<Word>)
word + annotations           → UtteranceContent::AnnotatedWord(Box<Annotated<Word>>)
word + replacement           → UtteranceContent::ReplacedWord(Box<ReplacedWord>)
word + replacement + annots  → UtteranceContent::ReplacedWord (with annotations)
```

This is a pattern: **optional children determine enum variant**. The mapping is
deterministic from the presence/absence of optional slots. A generator could
express this as a decision table:

```
replacement=absent, annotations=absent → Word
replacement=absent, annotations=present → AnnotatedWord
replacement=present → ReplacedWord (with optional annotations)
```

The question: is a decision table notation cleaner than the 15-line Rust `match`
it replaces? For this single rule, probably not. But there are ~20 rules with
this pattern.

### Tier 3: Recursive Collection (~15% of semantic code)

Walking `contents` children to build `Vec<UtteranceContent>`, walking
`mor_contents` to build `Vec<MorWord>`, walking `gra_contents` to build
`Vec<GrammaticalRelation>`. The pattern is:

```
for each child in contents:
    match child.kind():
        "word_with_optional_annotations" → recurse into Tier 2
        "event" → extract event text
        "pause_token" → map to Pause variant
        "separator" → map to Separator variant
        ... (24 UtteranceContent variants)
```

This is a dispatch table where each arm calls a conversion function. The
dispatch itself is automatable (CHOICE rule → match on kind). The per-arm
conversion functions are Tier 1 or Tier 2 logic.

### Tier 4: Cross-Node State (~30% of semantic code)

Logic that depends on state accumulated across multiple nodes:

- **Overlap matching**: `⌈` must pair with `⌉`, tracking indices across utterances
- **Scoped annotation nesting**: `<` opens a scope, `>` closes it, must balance
- **Quotation tracking**: `"` opens, `"` closes, content between is grouped
- **Gem scope tracking**: `@Bg` opens, `@Eg` closes, labels must match
- **Speaker validation**: main tier speaker checked against `@Participants`
- **Error suffix attachment**: ERROR fragments glued to previous word

This cannot be generated from grammar.json. The grammar doesn't encode these
cross-node constraints — they're semantic invariants of the CHAT format. This
code must be hand-written.

## Historical Precedent: ANTLR 3 → ANTLR 4

ANTLR 3 allowed declarative AST construction from parse trees using rewrite
rules:

```
// ANTLR 3: declarative AST rewriting
expr : a=INT '+' b=INT -> ^(ADD $a $b) ;
```

ANTLR 4 **removed this feature** in favor of pure parse trees with visitor
construction:

```
// ANTLR 4: visitor-based (procedural)
@Override
public Expr visitAdd(AddContext ctx) {
    return new Add(visit(ctx.left), visit(ctx.right));
}
```

The reasons for removal (per Terence Parr):
- The rewrite rule syntax was a second language to learn
- Complex transformations couldn't be expressed declaratively
- Debugging was harder (which rule produced this node?)
- The 80/20 problem: simple cases were cleaner, complex cases were worse
- Parse trees with visitors turned out to be "good enough"

## The CHAT Dilemma

CHAT has a particularly large data model:
- `UtteranceContent`: 24 variants
- `WordContent`: 13+ variants
- `Header`: 30+ variants
- `DependentTier`: 26 variants
- Plus `Word`, `MainTier`, `Utterance`, `ChatFile`, alignment types, etc.

Much of this model IS the parse tree with different names and some flattening.
The hand-written parser spends thousands of lines doing essentially:

```rust
// This is ~80% of what most parse functions do:
let text = node.text(source);
let domain_value = DomainType::from_text(text);
```

The pain is not that any single conversion is hard — it's that there are
**hundreds** of them, and each requires:
1. A function signature
2. Child extraction (now generated)
3. Text-to-domain conversion (often trivial)
4. Error handling for MISSING/ERROR children
5. Assembly into the parent type

## Options Under Consideration

### Option A: Accept Procedural — Hand-Write GrammarTraversal Impl

Write the semantic conversions as `GrammarTraversal` method overrides in Rust.
The generated defaults handle extraction; overrides add semantics.

**Pros:**
- No new notation to design
- Full Rust type checking
- Debuggable — it's just Rust
- ANTLR 4's lesson: visitors are "good enough"

**Cons:**
- Still ~5,000-7,000 lines of conversion code
- Each grammar change requires updating the impl
- Boilerplate ratio is high for Tier 1 mappings

### Option B: Generate Tier 1 Automatically, Hand-Write Tiers 2-4

Extend the generator to produce newtype conversions for Tier 1 mappings
(text → domain type). The child_role analysis already identifies which children
are `FreeText`, `Integer`, `ValidatedEnum`, etc. Generate `from_text()`
conversions for these.

Hand-write Tier 2 (conditional variants), Tier 3 (recursive collection), and
Tier 4 (cross-node state).

**Pros:**
- Eliminates ~30% of semantic code automatically
- No new notation — just better inference
- Tier 2-4 remain in Rust where they're debuggable

**Cons:**
- Tier 1 savings are real but not transformative
- The hard code (Tiers 3-4) is still hand-written

### Option C: Decision Table Notation for Tier 2

Design a small DSL (or TOML notation) for conditional variant selection:

```toml
[word_with_optional_annotations]
match = { replacement = "absent", annotations = "absent" }
result = "UtteranceContent::Word(Box::new(word))"

match = { replacement = "absent", annotations = "present" }
result = "UtteranceContent::AnnotatedWord(Box::new(Annotated::new(word).with(annotations)))"

match = { replacement = "present" }
result = "UtteranceContent::ReplacedWord(Box::new(ReplacedWord::new(word, replacement).with(annotations)))"
```

**Pros:**
- Makes the decision logic visible and reviewable
- Could generate the match block automatically
- ~20 rules would benefit

**Cons:**
- It's a new notation — the ANTLR 3 problem
- Complex variant construction still requires Rust expressions
- Debugging: "which TOML rule produced this?" is harder than reading Rust

### Option D: Typed CST as the Data Model

Instead of generating a SEPARATE data model and converting to it, use the
generated typed CST (with `NodeSlot` fields) AS the data model. The typed
wrappers (`SpeakerNode`, `TierBodyNode`) carry the text and span information.
Validation operates on the typed CST directly.

**Pros:**
- Eliminates the conversion layer entirely
- No duplication between CST types and model types
- Changes to grammar automatically change the "model"

**Cons:**
- The generated types are grammar-shaped, not domain-shaped
- `UtteranceContent` with 24 variants is a DOMAIN concept — the grammar
  doesn't have a single rule for it
- Validation, alignment, and serialization code depends on the model's shape
- Would require rewriting all downstream consumers

### Option E: Hybrid — Generated CST + Thin Domain Wrapper

Generate the typed CST. Write a thin domain wrapper that provides domain-
specific accessors and validation but delegates to the CST for storage:

```rust
struct Utterance<'tree> {
    children: UtteranceChildren<'tree>,  // generated
}

impl Utterance<'_> {
    fn speaker(&self) -> Option<&str> {
        // Delegate to generated typed node
        self.children.main_tier()?.speaker()?.text(source)
    }
}
```

**Pros:**
- No data copying — domain type wraps the CST
- Domain accessors are thin and easy to write
- Grammar changes flow through automatically
- Only the domain-specific accessors need maintenance

**Cons:**
- Lifetime management — the CST borrows from the tree, so the domain
  types are also borrowed. Can't store domain values independently.
- Serialization to JSON requires conversion anyway
- The existing ecosystem (batchalign3, CLAN, LSP) depends on `talkbank-model`
  types — migration cost is high

## Current Assessment

No clear winner. The right answer likely combines elements:

- **Generate Tier 1** (Option B) — this is low-risk, high-value
- **Hand-write Tiers 2-4** (Option A) — ANTLR 4's lesson
- **Consider Option E** for new code — don't convert if you can wrap
- **Defer Option C** (decision tables) — the notation design cost may exceed
  the code it replaces
- **Reject Option D** — too disruptive to existing ecosystem

The strongest argument against a fully declarative approach: the CHAT model
already exists and works. 51,750 lines of model code with validation,
alignment, serialization, and downstream consumers. Replacing it with a
generated model is a rewrite, not an optimization.

The strongest argument FOR more generation: the model IS mostly the parse tree
with different names. Every grammar change requires touching both grammar.js
AND the model. Generation would make grammar changes cheaper.

## Open Questions

1. How much of Tier 2 (conditional variants) could be expressed as a decision
   table WITHOUT embedding Rust expressions? Is there a clean notation?

2. Could the typed CST wrappers (Option E) work for NEW features while the
   existing model handles legacy? Incremental adoption rather than rewrite?

3. What would a grammar linter suggest for CHAT? If the grammar were more
   conventional (anonymous delimiters, more field() usage), would the
   generation gap shrink?

4. Is the 5,000-7,000 line estimate for hand-written semantics accurate?
   Measure by implementing a few more conversions and extrapolating.

## Decision

Deferred. Record this analysis and revisit after:
- Implementing 5-10 more semantic conversions to calibrate the effort estimate
- Building the grammar linter to see what grammar improvements are possible
- Testing Option E (thin wrapper) on one vertical slice
