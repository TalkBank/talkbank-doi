# CHAT Grammar Static Analysis Report

**Date:** 2026-03-23
**Grammar:** tree-sitter-chat (TalkBank CHAT format)
**Source:** `~/talkbank/talkbank-tools/grammar/src/grammar.json`
**Tool:** `tree-sitter-grammar-utils lint --min-severity high`

## Summary

| Severity | Count | Description |
|---|---|---|
| **High** | 34 | Real bugs: tokens sharing structural context, degenerate rules, broken precedence |
| Medium | 623 | Specific token shadows in separate structural contexts |
| Low | 1084 | Catch-all patterns shadowed (structural noise) |

The 34 high-severity warnings break down into three categories:

| Category | Count | Root cause |
|---|---|---|
| Token shadows | 6 | High-prec tokens subsume low-prec ones in shared contexts |
| Degenerate rules | 5 | `repeat1(choice(...))` can match semantically empty content |
| Precedence non-propagation | 23 | `prec()` on outer rule doesn't reach inner symbols |

---

## Category 1: Token Shadows (6 warnings)

### Bug 1.1: `zero` shadows `sin_word` and `speaker`

**Rules:**
```javascript
zero: $ => '0',                              // prec 3 (via word_prefix context)
sin_word: $ => /[a-zA-Z0-9:_-]+/,           // prec 0
speaker: $ => /[A-Za-z0-9_\'+\-]+/,         // prec 0
```

**Problem:** The literal `0` matches both `sin_word` and `speaker` patterns. Since `zero` has higher precedence, the lexer always produces `zero` instead of `sin_word` or `speaker` when it sees a bare `0`.

**Impact:** A speaker named `0` (numeric speaker ID) or a %sin tier word `0` would be parsed as `zero` instead of their intended types. Whether this is a real problem depends on whether numeric-only speaker IDs or %sin words exist in the corpus.

**Suggested investigation:** Search the corpus for:
- Speaker IDs that are just `0` in `@Participants:` lines
- `%sin` tier words that are just `0`

If these don't exist in real data, this shadow is safe to ignore.

### Bug 1.2: `ethnicity_value` / `ses_code_value` / `ses_combined` shadow `generic_id_ses`

**Rules:**
```javascript
ethnicity_value: $ => token(prec(1, /White|Black|Latino|Asian|Pacific|Native|Multiple|Unknown/)),
ses_code_value: $ => token(prec(1, /WC|UC|MC|LI/)),
ses_combined: $ => token(prec(1, /(White|...|Unknown)[, ](WC|UC|MC|LI)/)),
generic_id_ses: $ => /[^ \t|\r\n]([^|\r\n]*[^ \t|\r\n])?/,
```

**Problem:** `generic_id_ses` is a catch-all for the SES field in `@ID:` headers. The specific patterns (`White`, `WC`, etc.) have prec 1, so they always win. But `generic_id_ses` is also used as a fallback for unrecognized SES values.

**Impact:** If an `@ID:` line has an SES value that happens to match one of the specific patterns but is intended as a generic value, the specific token wins. This is actually **correct behavior** — the specific patterns are preferred over the generic. This is likely a **false positive**.

**Suggested action:** Verify that `generic_id_ses` is only used as a true fallback. If so, suppress this warning via `.grammar-annotations.json`.

### Bug 1.3: `x_dependent_tier` shadows `unsupported_dependent_tier`

**Rules:**
```javascript
x_dependent_tier: $ => token(prec(1, /%x[a-zA-Z][a-zA-Z0-9]*/)),
unsupported_dependent_tier: $ => /%[a-zA-Z][a-zA-Z0-9]*/,
```

**Problem:** Any tier starting with `%x` (like `%xmod`) matches both patterns. Since `x_dependent_tier` has prec 1, it always wins.

**Impact:** This is **intentional** — `x_dependent_tier` is a specific handler for `%x`-prefixed tiers, and `unsupported_dependent_tier` is the fallback for everything else. The higher precedence ensures the specific rule wins. **Not a bug.**

**Suggested action:** Suppress via annotation.

---

## Category 2: Degenerate Rules (5 warnings)

### Bug 2.1: `contents` can match only `[whitespaces]` or `[overlap_point]`

**Rule:**
```javascript
contents: $ => repeat1(choice(
  $.whitespaces, $.overlap_point, $.base_content_item, ...
)),
```

**Problem:** Since `contents` is `repeat1(choice(...))`, a single `whitespaces` or single `overlap_point` is a valid match. This means a main-line like `*CHI:\t⌈` parses as `contents(overlap_point)` — a content region with no actual speech.

**Impact:** **Medium.** The grammar accepts structurally empty main lines. Whether this causes downstream issues depends on how consumers handle `contents` nodes with no text-bearing children.

**Suggested fix:** If empty content is not valid CHAT, change to:
```javascript
contents: $ => seq($.base_content_item, repeat(choice(
  $.whitespaces, $.overlap_point, $.base_content_item, ...
))),
```
This requires at least one `base_content_item` (actual content) before allowing whitespace or overlap points.

### Bug 2.2: `free_text` can match only `[rest_of_line]` or `[continuation]`

**Problem:** `free_text` accepts a bare continuation marker (`+`) as valid free text.

**Impact:** Low — continuation markers in free text are likely always preceded by actual text in real data.

### Bug 2.3: `header_gap` can match only `[space]` or `[tab]`

**Impact:** None — this is **correct behavior**. A header gap IS just spaces/tabs. **Not a bug.**

### Bug 2.4–2.5: `text_with_bullets` / `text_with_bullets_and_pics`

**Problem:** These can match a bare `text_segment` or `continuation` alone.

**Impact:** Same as 2.2 — continuation markers could form valid text blocks. Low risk in practice.

---

## Category 3: Precedence Non-Propagation (23 warnings)

All 23 warnings share two root causes:

### Root Cause A: `standalone_word` (prec 6) → `word_body` (21 warnings)

**Rules:**
```javascript
standalone_word: $ => prec(6, seq(
  optional($.word_prefix),
  $.word_body,                    // <-- intermediate rule
  optional($.word_suffix),
)),
word_body: $ => repeat1(choice(   // prec 0
  $.word_segment, $.shortening, $.stress_marker,
  $.lengthening, $.ca_element, $.ca_delimiter, ...
)),
```

**Problem:** tree-sitter precedence is a **production-level** hint, not a rule-level hint. `prec(6)` on `standalone_word` only helps resolve conflicts at the `standalone_word` production level — it does NOT propagate through the `word_body` intermediate rule.

So when `word_body` competes with another rule for tokens like `lengthening`, `overlap_point`, `ca_element`, etc., the conflict is resolved at `word_body`'s precedence level (0), not `standalone_word`'s (6).

**Affected symbols:** `word_segment`, `shortening`, `stress_marker`, `lengthening`, `ca_element`, `ca_delimiter`, `tilde`, `syllable_pause`, `overlap_point`, `underline_begin`, `underline_end`, `_word_marker`

**Impact:** **This is the most significant structural issue.** If any of these symbols are contested between `word_body` and another rule at the same parser state, `standalone_word`'s prec(6) doesn't help. The grammar relies on the `conflicts` array and structural context to resolve these, but if a conflict arises that isn't declared, the parser may pick the wrong alternative.

**Suggested fixes (choose one):**

1. **Move prec to `word_body`:**
   ```javascript
   word_body: $ => prec(6, repeat1(choice(...))),
   ```
   This makes the precedence apply at the level where the actual conflict occurs.

2. **Inline `word_body` into `standalone_word`:**
   ```javascript
   standalone_word: $ => prec(6, seq(
     optional($.word_prefix),
     repeat1(choice($.word_segment, $.shortening, ...)),
     optional($.word_suffix),
   )),
   ```
   This eliminates the intermediate rule entirely.

3. **Accept and declare conflicts:** If the current behavior is correct (GLR resolves correctly via the `conflicts` array), add a comment documenting that prec(6) is intentionally inert and relies on conflict declarations.

### Root Cause B: `nonword` (prec 1) → `event` (2 warnings)

**Rules:**
```javascript
nonword: $ => prec(1, choice($.event, $.zero)),
event: $ => seq($.event_marker, repeat($.event_segment)),  // prec 0
```

**Problem:** `nonword`'s prec(1) doesn't propagate to `event_marker` or `event_segment` inside `event`.

**Impact:** Lower risk than Root Cause A because `nonword`'s prec(1) is a small advantage and `event` is structurally distinct. But if `event_marker` or `event_segment` is contested with another rule, prec(1) won't help.

**Suggested fix:** Same options as above — move prec to `event` or inline.

---

## Recommendations Summary

| Priority | Bug | Action |
|---|---|---|
| **High** | prec non-propagation: `standalone_word` → `word_body` (21 warnings) | Move prec(6) to `word_body` or inline |
| **Medium** | Degenerate `contents` (matches only `overlap_point`) | Require `base_content_item` first |
| **Low** | prec non-propagation: `nonword` → `event` (2 warnings) | Move prec(1) to `event` or inline |
| **Low** | Degenerate `free_text`, `text_with_bullets*` | Add required first element |
| **None** | `zero` ↔ `sin_word`/`speaker` | Verify corpus; likely safe |
| **None** | `ethnicity_value`/`ses_code_value` ↔ `generic_id_ses` | Intentional; suppress |
| **None** | `x_dependent_tier` ↔ `unsupported_dependent_tier` | Intentional; suppress |
| **None** | `header_gap` degenerate | Correct behavior |

## How to Reproduce

```bash
# Install the lint tool
cargo install --path crates/cli

# Run on the CHAT grammar (high severity only)
tree-sitter-grammar-utils lint src/grammar.json --min-severity high

# Run with all severities
tree-sitter-grammar-utils lint src/grammar.json --min-severity low
```

## Methodology

This analysis was performed by `tree-sitter-grammar-lint`, a static analysis tool that detects bugs in tree-sitter grammars by analyzing `grammar.json`. It uses:

1. **DFA regex intersection** (via `regex-automata` product automaton) to find token pairs whose patterns overlap
2. **Rule shape analysis** (via `rule_shape` flattening) to detect degenerate `repeat1(choice(...))` patterns
3. **Dependency graph tracing** to find precedence values that don't propagate through intermediate rules
4. **Structural context analysis** to classify severity — checking whether shadowed tokens share `CHOICE` alternatives with their shadowers

The tool is open source at `github.com/FranklinChen/prune-tree-sitter-grammar`.
