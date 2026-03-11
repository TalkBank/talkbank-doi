# Tree-sitter Parser UTF-8 Text Extraction Audit (Annotated)

This document lists every occurrence of `utf8_text(...)` and `extract_utf8_text(...)` in `talkbank-parser`, with a best-guess classification and the target data-model construct.

Total occurrences: 139

## rust/crates/talkbank-parser/src/parser/chat_file_parser/dependent_tier_dispatch/helpers.rs:50

- Best-guess leaf status: **Non-leaf**

- Model construct: **Dependent tier content (DependentTier)**

```rust
    47 |         }
    48 |     };
    49 | 
>   50 |     let text = match content_node.utf8_text(source.as_bytes()) {
    51 |         Ok(t) => t,
    52 |         Err(e) => {
    53 |             errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/dependent_tier_dispatch/unparsed.rs:20

- Best-guess leaf status: **Unknown**

- Model construct: **Dependent tier content (DependentTier)**

```rust
    17 |     }
    18 | 
    19 |     let tier_name = if let Some(name_node) = tier_node.child(1u32) {
>   20 |         match name_node.utf8_text(input.as_bytes()) {
    21 |             Ok(text) => text,
    22 |             Err(_) => {
    23 |                 errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/dependent_tier_dispatch/unparsed.rs:67

- Best-guess leaf status: **Unknown**

- Model construct: **Dependent tier content (DependentTier)**

```rust
    64 |             let mut cursor = tier_node.walk();
    65 |             for child in tier_node.children(&mut cursor) {
    66 |                 if child.kind() == ANYTHING {
>   67 |                     content_text = match child.utf8_text(input.as_bytes()) {
    68 |                         Ok(text) => text,
    69 |                         Err(_) => {
    70 |                             errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/dependent_tier_dispatch/user_defined.rs:24

- Best-guess leaf status: **Unknown**

- Model construct: **Dependent tier content (DependentTier)**

```rust
    21 | 
    22 |     // Grammar for x_dependent_tier: seq('%', 'x_tier_code', 'x_tier_label', tier_sep, text_with_bullets, '\n')
    23 |     let tier_label = match find_child_by_kind(tier_node, X_TIER_LABEL) {
>   24 |         Some(n) => match n.utf8_text(input.as_bytes()) {
    25 |             Ok(text) => text,
    26 |             Err(_) => {
    27 |                 errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/dependent_tier_dispatch/user_defined.rs:50

- Best-guess leaf status: **Unknown**

- Model construct: **Dependent tier content (DependentTier)**

```rust
    47 |     };
    48 | 
    49 |     let content_text = match find_child_by_kind(tier_node, TEXT_WITH_BULLETS) {
>   50 |         Some(n) => match n.utf8_text(input.as_bytes()) {
    51 |             Ok(text) => text,
    52 |             Err(_) => {
    53 |                 errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_dispatch/parse.rs:75

- Best-guess leaf status: **Non-leaf**

- Model construct: **Header dispatch (Header enum / Unknown header)**

```rust
    72 |                         header_node.end_byte()
    73 |                     ),
    74 |                 ));
>   75 |                 let text = match header_node.utf8_text(wrapped.as_bytes()) {
    76 |                     Ok(text) => text.to_string(),
    77 |                     Err(_) => String::new(),
    78 |                 };
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_dispatch/parse.rs:159

- Best-guess leaf status: **Non-leaf**

- Model construct: **Header dispatch (Header enum / Unknown header)**

```rust
   156 |                     TYPES_HEADER => parse_types_header(header_node, wrapped, &error_sink),
   157 |                     T_HEADER => parse_t_header(header_node, wrapped, &error_sink),
   158 |                     unknown => {
>  159 |                         let text = match header_node.utf8_text(wrapped.as_bytes()) {
   160 |                             Ok(text) => text.to_string(),
   161 |                             Err(_) => String::new(),
   162 |                         };
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_dispatch/parse.rs:221

- Best-guess leaf status: **Non-leaf**

- Model construct: **Header dispatch (Header enum / Unknown header)**

```rust
   218 | 
   219 | fn find_child_text(node: Node, input: &str, kind: &str) -> String {
   220 |     match find_child_by_kind(node, kind) {
>  221 |         Some(child) => match child.utf8_text(input.as_bytes()) {
   222 |             Ok(text) => text.to_string(),
   223 |             Err(_) => String::new(),
   224 |         },
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/dispatch/simple.rs:12

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
     9 | };
    10 | 
    11 | use super::super::helpers::get_required_content_by_kind;
>   12 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
    13 | 
    14 | pub(super) fn parse_simple_header(
    15 |     header_kind: &str,
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/dispatch/simple.rs:38

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    35 |             // Child layout: [0]=prefix, [1]=sep, [2]=geometry, [3]=newline
    36 |             // Content at position 2
    37 |             let geometry = match header_actual.child(2u32) {
>   38 |                 Some(child) => extract_utf8_text(child, input, errors, WINDOW_HEADER, ""),
    39 |                 None => "",
    40 |             };
    41 |             Some(Header::Window {
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/helpers.rs:5

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
     2 | 
     3 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
     4 | use crate::model::{self, ChatOptionFlag};
>    5 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     6 | use tree_sitter::Node;
     7 | use tree_sitter_talkbank::node_types::{CONTINUATION, OPTION_NAME, OPTIONS_CONTENTS, REST_OF_LINE};
     8 | 
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/helpers.rs:20

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    17 |         let mut cursor = option_list_node.walk();
    18 |         for child in option_list_node.children(&mut cursor) {
    19 |             if child.kind() == OPTION_NAME
>   20 |                 && let Ok(text) = child.utf8_text(input.as_bytes())
    21 |             {
    22 |                 // Tree-sitter provides clean text for option_name nodes - no trim needed
    23 |                 if let Some(flag) = ChatOptionFlag::from_text(text) {
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/helpers.rs:32

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    29 |         }
    30 | 
    31 |         if flags.is_empty()
>   32 |             && let Ok(text) = option_list_node.utf8_text(input.as_bytes())
    33 |             && !text.is_empty()
    34 |         {
    35 |             // If option_list node exists with content but no recognized flags, report error
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/helpers.rs:65

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    62 |     header_kind: &str,
    63 | ) -> String {
    64 |     if let Some(child) = find_child_by_kind(node, kind) {
>   65 |         extract_utf8_text(child, input, errors, header_kind, "").to_string()
    66 |     } else {
    67 |         errors.report(ParseError::new(
    68 |             ErrorCode::TreeParsingError,
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/helpers.rs:93

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    90 |     for child in node.children(&mut cursor) {
    91 |         match child.kind() {
    92 |             REST_OF_LINE => {
>   93 |                 if let Ok(text) = child.utf8_text(input.as_bytes()) {
    94 |                     if !text.is_empty() {
    95 |                         label.push_str(text);
    96 |                         saw_text = true;
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/pre_begin.rs:10

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
     7 | use tree_sitter_talkbank::node_types::*;
     8 | 
     9 | use crate::parser::tree_parsing::header::parse_pid_header;
>   10 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
    11 | 
    12 | pub fn handle_pre_begin_header(
    13 |     node: tree_sitter::Node,
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/pre_begin.rs:32

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    29 |             // Content at position 2
    30 |             let geometry = match node.child(2u32) {
    31 |                 Some(child) => {
>   32 |                     extract_utf8_text(child, input, errors, "window_geometry", "").to_string()
    33 |                 }
    34 |                 None => String::new(),
    35 |             };
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/pre_begin.rs:49

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    46 |             // Content at position 2
    47 |             let colors = match node.child(2u32) {
    48 |                 Some(child) => {
>   49 |                     extract_utf8_text(child, input, errors, "color_word_list", "").to_string()
    50 |                 }
    51 |                 None => String::new(),
    52 |             };
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/header_parser/pre_begin.rs:65

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    62 |             // Child layout: [0]=prefix, [1]=sep, [2]=font_spec, [3]=newline
    63 |             // Content at position 2
    64 |             let font = match node.child(2u32) {
>   65 |                 Some(child) => extract_utf8_text(child, input, errors, "font_spec", "").to_string(),
    66 |                 None => String::new(),
    67 |             };
    68 |             lines.push(Line::header_with_span(
```

## rust/crates/talkbank-parser/src/parser/chat_file_parser/single_item/parse_utterance.rs:54

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    51 | 
    52 |     // Extract the actual main tier line from CST for error reporting
    53 |     // The main_tier_node contains the actual tier content
>   54 |     let main_tier_line = match main_tier_node.utf8_text(to_parse.as_bytes()) {
    55 |         Ok(text) => text,
    56 |         Err(_) => input,
    57 |     };
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/dependent_tier.rs:149

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
   146 |             0 => {} // % symbol
   147 |             1 => {
   148 |                 // tier_name node
>  149 |                 if let Ok(text) = child.utf8_text(source.as_bytes()) {
   150 |                     tier_type = text.to_string();
   151 |                 }
   152 |             }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/dependent_tier.rs:157

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
   154 |             3 => {} // tab
   155 |             4 => {
   156 |                 // content node (or multiple content nodes)
>  157 |                 if let Ok(text) = child.utf8_text(source.as_bytes()) {
   158 |                     content = text.to_string();
   159 |                 }
   160 |             }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/dependent_tier.rs:163

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
   160 |             }
   161 |             _ => {
   162 |                 // Additional content nodes - append
>  163 |                 if let Ok(text) = child.utf8_text(source.as_bytes()) {
   164 |                     if !content.is_empty() && !text.is_empty() {
   165 |                         content.push(' ');
   166 |                     }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:7

- Best-guess leaf status: **Unknown**

- Model construct: **%gra tier (Gra relations)**

```rust
     4 | use talkbank_model::model::borrowed::GrammaticalRelationBorrowed;
     5 | use tree_sitter::Node;
     6 | 
>    7 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     8 | 
     9 | /// Parse a single grammatical relation from tree-sitter node
    10 | ///
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:28

- Best-guess leaf status: **Unknown**

- Model construct: **%gra tier (Gra relations)**

```rust
    25 |     errors: &impl ErrorSink,
    26 | ) -> GrammaticalRelationBorrowed<'a> {
    27 |     let index_text = match node.child(0u32) {
>   28 |         Some(n) => extract_utf8_text(n, source, errors, "gra_index", "0"),
    29 |         None => {
    30 |             errors.report(ParseError::new(
    31 |                 ErrorCode::MalformedGrammarRelation,
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:75

- Best-guess leaf status: **Unknown**

- Model construct: **%gra tier (Gra relations)**

```rust
    72 |     };
    73 | 
    74 |     let head_text = match node.child(2u32) {
>   75 |         Some(n) => extract_utf8_text(n, source, errors, "gra_head", "0"),
    76 |         None => {
    77 |             errors.report(ParseError::new(
    78 |                 ErrorCode::MalformedGrammarRelation,
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/gra/relation.rs:109

- Best-guess leaf status: **Unknown**

- Model construct: **%gra tier (Gra relations)**

```rust
   106 |     };
   107 | 
   108 |     let relation_text = match node.child(4u32) {
>  109 |         Some(n) => extract_utf8_text(n, source, errors, "gra_relation_name", ""),
   110 |         None => {
   111 |             errors.report(ParseError::new(
   112 |                 ErrorCode::MalformedGrammarRelation,
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/segments.rs:11

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
     8 | use tree_sitter_talkbank::node_types as kind;
     9 | 
    10 | use crate::parser::tree_parsing::helpers::unexpected_node_error;
>   11 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
    12 | 
    13 | pub fn parse_mor_prefixes_borrowed<'a>(
    14 |     node: Node<'a>,
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/segments.rs:27

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
    24 |             match child.kind() {
    25 |                 kind::MOR_PREFIX => {
    26 |                     if let Some(segment) = child.child(0u32) {
>   27 |                         let text = extract_utf8_text(segment, source, errors, "mor_prefix", "");
    28 |                         prefixes.push(Cow::Borrowed(text));
    29 |                     }
    30 |                 }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/segments.rs:56

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
    53 |                 kind::MOR_FUSIONAL_SUFFIX => {
    54 |                     if let Some(segment) = child.child(1u32) {
    55 |                         let text =
>   56 |                             extract_utf8_text(segment, source, errors, "mor_fusional_suffix", "");
    57 |                         suffixes.push(MorSuffixBorrowed::Fusional(Cow::Borrowed(text)));
    58 |                     }
    59 |                 }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/segments.rs:62

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
    59 |                 }
    60 |                 kind::MOR_SUFFIX => {
    61 |                     if let Some(segment) = child.child(1u32) {
>   62 |                         let text = extract_utf8_text(segment, source, errors, "mor_suffix", "");
    63 |                         suffixes.push(MorSuffixBorrowed::Hyphen(Cow::Borrowed(text)));
    64 |                     }
    65 |                 }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/segments.rs:69

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
    66 |                 kind::MOR_COLON_SUFFIX => {
    67 |                     if let Some(segment) = child.child(1u32) {
    68 |                         let text =
>   69 |                             extract_utf8_text(segment, source, errors, "mor_colon_suffix", "");
    70 |                         suffixes.push(MorSuffixBorrowed::Colon(Cow::Borrowed(text)));
    71 |                     }
    72 |                 }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/segments.rs:97

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
    94 |         if let Some(child) = node.child(idx as u32) {
    95 |             match child.kind() {
    96 |                 kind::MOR_ENGLISH_WORD => {
>   97 |                     let text = extract_utf8_text(child, source, errors, "mor_english_word", "");
    98 |                     translations.push(Cow::Borrowed(text));
    99 |                 }
   100 |                 kind::EQUALS | kind::SLASH => {}
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/segments.rs:143

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
   140 |         if let Some(child) = node.child(idx as u32) {
   141 |             match child.kind() {
   142 |                 kind::MOR_CATEGORY => {
>  143 |                     if let Ok(text) = child.utf8_text(source.as_bytes()) {
   144 |                         category = Cow::Borrowed(text);
   145 |                     }
   146 |                 }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/segments.rs:167

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
   164 |                     // Skip colon separator
   165 |                 }
   166 |                 kind::MOR_SUBCATEGORY => {
>  167 |                     if let Ok(text) = child.utf8_text(source.as_bytes()) {
   168 |                         subcategories.push(Cow::Borrowed(text));
   169 |                     }
   170 |                 }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/tier.rs:13

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
    10 | use super::item::parse_mor_content_borrowed;
    11 | use crate::parser::tree_parsing::helpers::unexpected_node_error;
    12 | use crate::parser::tree_parsing::parser_helpers::{
>   13 |     check_not_missing, expect_child_at, extract_utf8_text, is_terminator,
    14 | };
    15 | 
    16 | /// Parse %mor tier
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/tier.rs:64

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
    61 |                 }
    62 |                 _ if is_terminator(kind) || kind == kind::TERMINATOR => {
    63 |                     // Extract terminator as optional field, not as item
>   64 |                     let text = extract_utf8_text(child, source, errors, "mor_terminator", ".");
    65 |                     terminator = Some(if text.is_empty() {
    66 |                         Cow::Borrowed(".")
    67 |                     } else {
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/word.rs:14

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
    11 |     parse_mor_prefixes_borrowed, parse_mor_w_borrowed, parse_part_of_speech_borrowed,
    12 | };
    13 | use crate::parser::tree_parsing::helpers::unexpected_node_error;
>   14 | use crate::parser::tree_parsing::parser_helpers::{check_not_missing, extract_utf8_text};
    15 | 
    16 | pub fn parse_mor_word_borrowed<'a>(
    17 |     node: Node<'a>,
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/mor/word.rs:46

- Best-guess leaf status: **Unknown**

- Model construct: **%mor tier (MorTier items)**

```rust
    43 |                 }
    44 |                 kind::PIPE => {}
    45 |                 kind::STEM => {
>   46 |                     stem = Cow::Borrowed(extract_utf8_text(child, source, errors, "mor_stem", ""));
    47 |                 }
    48 |                 kind::MOR_W => {
    49 |                     suffixes = parse_mor_w_borrowed(child, source, errors);
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/pho/cst.rs:121

- Best-guess leaf status: **Unknown**

- Model construct: **%pho tier (PhoTier)**

```rust
   118 |     node: Node<'a>,
   119 |     source: &'a str,
   120 | ) -> Vec<PhoItemBorrowed<'a>> {
>  121 |     let text = node.utf8_text(source.as_bytes()).unwrap_or_default();
   122 |     // DEFAULT: If the CST node contains invalid UTF-8, treat the fallback as empty.
   123 |     if !text.is_empty() {
   124 |         vec![PhoItemBorrowed::Word(PhoWordBorrowed::new(Cow::Borrowed(
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/pho/groups.rs:10

- Best-guess leaf status: **Unknown**

- Model construct: **%pho tier (PhoTier)**

```rust
     7 | 
     8 | use super::cst::{build_group_from_words, fallback_group_as_text};
     9 | use crate::parser::tree_parsing::helpers::unexpected_node_error;
>   10 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
    11 | 
    12 | /// Extract items from a pho_group node
    13 | ///
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/pho/groups.rs:31

- Best-guess leaf status: **Unknown**

- Model construct: **%pho tier (PhoTier)**

```rust
    28 |         match first_child.kind() {
    29 |             kind::PHO_WORDS => {
    30 |                 // Extract text from pho_words (handles pho_word + '+' + pho_word structure)
>   31 |                 let text = extract_utf8_text(first_child, source, errors, "pho_words", "");
    32 |                 if !text.is_empty() {
    33 |                     vec![PhoItemBorrowed::Word(PhoWordBorrowed::new(Cow::Borrowed(
    34 |                         text,
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/pho/groups.rs:85

- Best-guess leaf status: **Unknown**

- Model construct: **%pho tier (PhoTier)**

```rust
    82 |         if let Some(child) = node.child(idx as u32) {
    83 |             match child.kind() {
    84 |                 kind::PHO_WORDS => {
>   85 |                     let text = extract_utf8_text(child, source, errors, "pho_words", "");
    86 |                     if !text.is_empty() {
    87 |                         words.push(Cow::Borrowed(text));
    88 |                     }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/pho/unparsed.rs:56

- Best-guess leaf status: **Unknown**

- Model construct: **%pho tier (PhoTier)**

```rust
    53 |     // Position 3 should be the "anything" node with the content
    54 |     let content_text = node
    55 |         .child(3u32)
>   56 |         .and_then(|child| child.utf8_text(source.as_bytes()).ok())
    57 |         .unwrap_or_default();
    58 |     // DEFAULT: Missing content node or invalid UTF-8 yields empty content to keep parsing.
    59 | 
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/sin/groups.rs:9

- Best-guess leaf status: **Unknown**

- Model construct: **%sin tier (SinTier)**

```rust
     6 | };
     7 | 
     8 | use crate::parser::tree_parsing::helpers::unexpected_node_error;
>    9 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
    10 | 
    11 | pub(super) fn extract_sin_group_items(
    12 |     node: Node,
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/sin/groups.rs:19

- Best-guess leaf status: **Unknown**

- Model construct: **%sin tier (SinTier)**

```rust
    16 |     if let Some(first_child) = node.child(0u32) {
    17 |         match first_child.kind() {
    18 |             SIN_WORD => {
>   19 |                 let text = extract_utf8_text(first_child, source, errors, "sin_word", "");
    20 |                 if !text.is_empty() {
    21 |                     vec![SinItem::Token(SinToken::new_unchecked(text))]
    22 |                 } else {
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/sin/groups.rs:58

- Best-guess leaf status: **Unknown**

- Model construct: **%sin tier (SinTier)**

```rust
    55 |                 }
    56 |             }
    57 |             _ => {
>   58 |                 let text = extract_utf8_text(node, source, errors, "sin_item", "");
    59 |                 if !text.is_empty() {
    60 |                     vec![SinItem::Token(SinToken::new_unchecked(text))]
    61 |                 } else {
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/sin/groups.rs:84

- Best-guess leaf status: **Unknown**

- Model construct: **%sin tier (SinTier)**

```rust
    81 |         if let Some(child) = node.child(idx as u32) {
    82 |             match child.kind() {
    83 |                 SIN_WORD => {
>   84 |                     let text = extract_utf8_text(child, source, errors, "sin_word", "");
    85 |                     if !text.is_empty() {
    86 |                         tokens.push(SinToken::new_unchecked(text));
    87 |                     }
```

## rust/crates/talkbank-parser/src/parser/tier_parsers/wor.rs:86

- Best-guess leaf status: **Leaf**

- Model construct: **%wor tier (WorTier)**

```rust
    83 |             }
    84 |             LANGCODE => {
    85 |                 // Parse language code - extract text from node
>   86 |                 if let Ok(text) = child.utf8_text(source.as_bytes()) {
    87 |                     // langcode node is just the code itself (e.g., "spa"), not the brackets
    88 |                     language_code = Some(LanguageCode::new(text));
    89 |                 }
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/inline_bullet.rs:5

- Best-guess leaf status: **Unknown**

- Model construct: **Bullet content (BulletContent)**

```rust
     2 | use crate::parser::tree_parsing::parser_helpers::cst_assertions::{
     3 |     assert_child_count_exact, expect_child,
     4 | };
>    5 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     6 | use tree_sitter::Node;
     7 | use tree_sitter_talkbank::node_types::{BULLET_END, NATURAL_NUMBER, UNDERSCORE};
     8 | 
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/inline_bullet.rs:52

- Best-guess leaf status: **Unknown**

- Model construct: **Bullet content (BulletContent)**

```rust
    49 |     let start_ms = if let Some(start_node) =
    50 |         expect_child(node, 1, NATURAL_NUMBER, source, errors, "inline_bullet")
    51 |     {
>   52 |         let start_str = extract_utf8_text(start_node, source, errors, "inline_bullet_start", "0");
    53 |         match start_str.parse::<u64>() {
    54 |             Ok(ms) => ms,
    55 |             Err(_) => {
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/inline_bullet.rs:87

- Best-guess leaf status: **Unknown**

- Model construct: **Bullet content (BulletContent)**

```rust
    84 |     let end_ms = if let Some(end_node) =
    85 |         expect_child(node, 3, NATURAL_NUMBER, source, errors, "inline_bullet")
    86 |     {
>   87 |         let end_str = extract_utf8_text(end_node, source, errors, "inline_bullet_end", "0");
    88 |         match end_str.parse::<u64>() {
    89 |             Ok(ms) => ms,
    90 |             Err(_) => {
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/inline_pic.rs:2

- Best-guess leaf status: **Unknown**

- Model construct: **Bullet content (BulletContent)**

```rust
     1 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
>    2 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     3 | use tree_sitter::Node;
     4 | 
     5 | /// Parse inline_pic node
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/inline_pic.rs:37

- Best-guess leaf status: **Leaf**

- Model construct: **Bullet content (BulletContent)**

```rust
    34 | ) -> Option<String> {
    35 |     // Position 3: filename
    36 |     let filename = if let Some(filename_node) = node.child(3u32) {
>   37 |         extract_utf8_text(filename_node, source, errors, "pic_filename", "").to_string()
    38 |     } else {
    39 |         errors.report(ParseError::new(
    40 |             ErrorCode::TreeParsingError,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/parse.rs:2

- Best-guess leaf status: **Unknown**

- Model construct: **Bullet content (BulletContent)**

```rust
     1 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
>    2 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     3 | use smallvec::SmallVec;
     4 | use talkbank_model::model::{BulletContent, BulletContentSegment};
     5 | use tree_sitter::Node;
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/bullet_content/parse.rs:85

- Best-guess leaf status: **Unknown**

- Model construct: **Bullet content (BulletContent)**

```rust
    82 |         match child_kind {
    83 |             TEXT_SEGMENT => {
    84 |                 // Extract plain text from text_segment node
>   85 |                 let text = extract_utf8_text(child, source, errors, "text_segment", "");
    86 |                 if !text.is_empty() {
    87 |                     segments.push(BulletContentSegment::text(text));
    88 |                 }
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/freecode/mod.rs:4

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
     1 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
     2 | use crate::model::{Freecode, UtteranceContent};
     3 | use crate::parser::tree_parsing::helpers::unexpected_node_error;
>    4 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     5 | use tree_sitter::Node;
     6 | 
     7 | /// Parse freecode node [^ text] into UtteranceContent.
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/freecode/mod.rs:32

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    29 |             let kind = child.kind();
    30 |             match kind {
    31 |                 "bracketed_content" => {
>   32 |                     text = extract_utf8_text(child, source, errors, "freecode_content", "")
    33 |                         .to_string();
    34 |                 }
    35 |                 "freecode_prefix" | "right_bracket" | "space" => {}
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/id/helpers.rs:35

- Best-guess leaf status: **Unknown**

- Model construct: **@ID header fields (IDHeader)**

```rust
    32 |     context: &str,
    33 | ) -> Option<String> {
    34 |     let node = node_opt?;
>   35 |     match node.utf8_text(source.as_bytes()) {
    36 |         Ok(text) => Some(text.to_string()),
    37 |         Err(e) => {
    38 |             errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/languages.rs:69

- Best-guess leaf status: **Leaf**

- Model construct: **@Languages header (Language list)**

```rust
    66 |         && let Some(child) = contents.child(idx as u32)
    67 |     {
    68 |         if child.kind() == LANGUAGE_CODE {
>   69 |             if let Ok(code) = child.utf8_text(source.as_bytes()) {
    70 |                 codes.push(LanguageCode::new(code));
    71 |             }
    72 |             idx += 1;
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/languages.rs:158

- Best-guess leaf status: **Leaf**

- Model construct: **@Languages header (Language list)**

```rust
   155 |         // Check for language_code
   156 |         if let Some(child) = contents.child(idx as u32) {
   157 |             if child.kind() == LANGUAGE_CODE {
>  158 |                 if let Ok(code) = child.utf8_text(source.as_bytes()) {
   159 |                     codes.push(LanguageCode::new(code));
   160 |                 }
   161 |                 idx += 1;
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/media.rs:5

- Best-guess leaf status: **Unknown**

- Model construct: **@Media header (MediaHeader)**

```rust
     2 | use tree_sitter_talkbank::node_types::*;
     3 | 
     4 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
>    5 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     6 | use talkbank_model::model::{Header, MediaHeader, MediaStatus, MediaType};
     7 | 
     8 | /// Parse Media header from tree-sitter node
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/media.rs:59

- Best-guess leaf status: **Leaf**

- Model construct: **@Media header (MediaHeader)**

```rust
    56 |     // Extract filename from position 0
    57 |     let filename = if let Some(child) = contents.child(0u32) {
    58 |         if child.kind() == MEDIA_FILENAME {
>   59 |             extract_utf8_text(child, source, errors, "media_filename", "").to_string()
    60 |         } else {
    61 |             String::new()
    62 |         }
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/media.rs:70

- Best-guess leaf status: **Leaf**

- Model construct: **@Media header (MediaHeader)**

```rust
    67 |     // Extract media_type from position 3 (after comma and whitespace)
    68 |     let media_type = if let Some(child) = contents.child(3u32) {
    69 |         if child.kind() == MEDIA_TYPE {
>   70 |             let type_text = extract_utf8_text(child, source, errors, "media_type", "");
    71 |             match type_text {
    72 |                 "audio" => MediaType::Audio,
    73 |                 "video" => MediaType::Video,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/media.rs:95

- Best-guess leaf status: **Leaf**

- Model construct: **@Media header (MediaHeader)**

```rust
    92 |     // Extract optional status from position 6 (after second comma and whitespace at positions 4 and 5)
    93 |     let status = if let Some(child) = contents.child(6) {
    94 |         if child.kind() == MEDIA_STATUS {
>   95 |             let status_text = extract_utf8_text(child, source, errors, "media_status", "");
    96 |             match status_text {
    97 |                 "missing" => Some(MediaStatus::Missing),
    98 |                 "unlinked" => Some(MediaStatus::Unlinked),
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/pid.rs:5

- Best-guess leaf status: **Unknown**

- Model construct: **@PID header (PidHeader)**

```rust
     2 | use tree_sitter_talkbank::node_types::{ANYTHING, PID_HEADER};
     3 | 
     4 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
>    5 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     6 | use talkbank_model::model::{Header, PidValue};
     7 | 
     8 | /// Parse PID header from tree-sitter node
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/pid.rs:35

- Best-guess leaf status: **Leaf**

- Model construct: **@PID header (PidHeader)**

```rust
    32 | 
    33 |     // Grammar: seq(prefix, header_sep, anything, newline)
    34 |     let pid = if let Some(child) = find_child_by_kind(node, ANYTHING) {
>   35 |         extract_utf8_text(child, source, errors, "pid_value", "").to_string()
    36 |     } else {
    37 |         errors.report(ParseError::new(
    38 |             ErrorCode::TreeParsingError,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/situation.rs:5

- Best-guess leaf status: **Unknown**

- Model construct: **@Situation header (SituationHeader)**

```rust
     2 | use tree_sitter_talkbank::node_types::{ANYTHING, SITUATION_HEADER};
     3 | 
     4 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
>    5 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     6 | use talkbank_model::model::{Header, SituationDescription};
     7 | 
     8 | /// Parse Situation header from tree-sitter node
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/situation.rs:35

- Best-guess leaf status: **Unknown**

- Model construct: **@Situation header (SituationHeader)**

```rust
    32 | 
    33 |     // Grammar: seq(prefix, header_sep, anything, newline)
    34 |     let text = if let Some(child) = find_child_by_kind(node, ANYTHING) {
>   35 |         extract_utf8_text(child, source, errors, "situation_text", "").to_string()
    36 |     } else {
    37 |         errors.report(ParseError::new(
    38 |             ErrorCode::TreeParsingError,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/t_header.rs:5

- Best-guess leaf status: **Unknown**

- Model construct: **@T header (THeader)**

```rust
     2 | use tree_sitter_talkbank::node_types::{ANYTHING, T_HEADER};
     3 | 
     4 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
>    5 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     6 | use talkbank_model::model::{Header, TDescription};
     7 | 
     8 | /// Parse @T header
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/t_header.rs:31

- Best-guess leaf status: **Unknown**

- Model construct: **@T header (THeader)**

```rust
    28 | 
    29 |     // Grammar: seq(prefix, header_sep, anything, newline)
    30 |     let text = if let Some(child) = find_child_by_kind(node, ANYTHING) {
>   31 |         extract_utf8_text(child, source, errors, "t_header_text", "").to_string()
    32 |     } else {
    33 |         errors.report(ParseError::new(
    34 |             ErrorCode::TreeParsingError,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/metadata/types.rs:72

- Best-guess leaf status: **Unknown**

- Model construct: **@Types header (TypesHeader)**

```rust
    69 |         return String::new();
    70 |     };
    71 | 
>   72 |     match child.utf8_text(source.as_bytes()) {
    73 |         Ok(text) => text.to_string(),
    74 |         Err(e) => {
    75 |             errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:219

- Best-guess leaf status: **Unknown**

- Model construct: **@Participants header (Participants)**

```rust
   216 | fn parse_participant_entry(node: Node, source: &str, _errors: &impl ErrorSink) -> ParticipantEntry {
   217 |     // Extract speaker code (first child)
   218 |     let speaker_code = match node.child(0u32) {
>  219 |         Some(child) => match child.utf8_text(source.as_bytes()) {
   220 |             Ok(text) => text.to_string(),
   221 |             Err(_) => String::new(),
   222 |         },
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/header/participants.rs:231

- Best-guess leaf status: **Unknown**

- Model construct: **@Participants header (Participants)**

```rust
   228 |         let mut cursor = node.walk();
   229 |         node.children(&mut cursor)
   230 |             .filter(|n| n.kind() == PARTICIPANT_WORD)
>  231 |             .filter_map(|n| n.utf8_text(source.as_bytes()).ok())
   232 |             .map(|s| s.to_string())
   233 |             .collect()
   234 |     };
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/helpers.rs:14

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    11 | /// This function is used to transform tree-sitter's internal "ERROR" nodes
    12 | /// into actionable, user-friendly error messages that don't expose parser internals.
    13 | pub(crate) fn analyze_error_node(node: Node, source: &str, context: &str) -> ParseError {
>   14 |     let error_text = node.utf8_text(source.as_bytes()).unwrap_or_default();
    15 |     // DEFAULT: Invalid UTF-8 in the error node is treated as empty for analysis.
    16 |     let error_text_clean = error_text;
    17 |     let is_whitespace_only = error_text.chars().all(|c| c.is_whitespace());
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/helpers.rs:4

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier annotations (Annotation models)**

```rust
     1 | //! Helper utilities for annotation parsing
     2 | 
     3 | use crate::error::ErrorSink;
>    4 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     5 | use tree_sitter::Node;
     6 | // Note: Many punctuation/anonymous tokens don't have constants in node_types
     7 | // because they're not "named" nodes in tree-sitter's node-types.json.
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/helpers.rs:28

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier annotations (Annotation models)**

```rust
    25 |             match child.kind() {
    26 |                 // Content nodes - extract their text
    27 |                 "bracketed_content" | "word_segment" | "annotation_text" => {
>   28 |                     if let Ok(text) = child.utf8_text(source_bytes) {
    29 |                         text_parts.push(text);
    30 |                     }
    31 |                 }
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/helpers.rs:54

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier annotations (Annotation models)**

```rust
    51 |                 _unknown => {
    52 |                     // Unknown but non-error node - extract text as fallback
    53 |                     // This handles cases where grammar changes added new content node types
>   54 |                     if let Ok(text) = child.utf8_text(source_bytes) {
    55 |                         // Include non-empty text from unknown nodes
    56 |                         if !text.is_empty() {
    57 |                             text_parts.push(text);
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/helpers.rs:89

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier annotations (Annotation models)**

```rust
    86 |                 | tree_sitter_talkbank::node_types::HASH
    87 |                 | tree_sitter_talkbank::node_types::SPACE
    88 |         ) {
>   89 |             let text = extract_utf8_text(child, source, errors, kind, "");
    90 |             if !text.is_empty() {
    91 |                 time_parts.push(text);
    92 |             }
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/overlap.rs:91

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier annotations (Annotation models)**

```rust
    88 |                 }
    89 |                 // The index is captured as overlap_marker_index
    90 |                 OVERLAP_MARKER_INDEX => {
>   91 |                     if let Ok(text) = child.utf8_text(source_bytes) {
    92 |                         if let Ok(value) = text.parse::<u8>() {
    93 |                             if (1..=9).contains(&value) {
    94 |                                 return Some(OverlapMarkerIndex::new(value));
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/scoped/symbols.rs:3

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier annotations (Annotation models)**

```rust
     1 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
     2 | use crate::model::{ScopedAnnotation, ScopedUnknown};
>    3 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     4 | use tree_sitter::Node;
     5 | use tree_sitter_talkbank::node_types::{
     6 |     SCOPED_BEST_GUESS, SCOPED_CONTRASTIVE_STRESSING, SCOPED_STRESSING, SCOPED_UNCERTAIN,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/annotations/scoped/symbols.rs:22

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier annotations (Annotation models)**

```rust
    19 |             SCOPED_BEST_GUESS => Some(ScopedAnnotation::ScopedBestGuess),
    20 |             SCOPED_UNCERTAIN => Some(ScopedAnnotation::ScopedUncertain),
    21 |             _ => {
>   22 |                 let text = extract_utf8_text(symbol_node, source, errors, "scoped_symbol", "?");
    23 |                 Some(ScopedAnnotation::Unknown(ScopedUnknown {
    24 |                     marker: text.to_string(),
    25 |                     text: String::new(),
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/internal_bullet.rs:8

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
     5 | use crate::parser::tree_parsing::parser_helpers::cst_assertions::{
     6 |     assert_child_count_exact, expect_child,
     7 | };
>    8 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     9 | use tree_sitter::Node;
    10 | use tree_sitter_talkbank::node_types::{BULLET_END, HYPHEN, NATURAL_NUMBER, UNDERSCORE};
    11 | 
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/internal_bullet.rs:52

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
    49 |         true
    50 |     };
    51 | 
>   52 |     let start_text = extract_utf8_text(start_node, source, errors, "internal_bullet_start", "0");
    53 |     let start_ms: u32 = match start_text.parse() {
    54 |         Ok(ms) => ms,
    55 |         Err(_) => {
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/internal_bullet.rs:71

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
    68 |         }
    69 |     };
    70 | 
>   71 |     let end_text = extract_utf8_text(end_node, source, errors, "internal_bullet_end", "0");
    72 |     let end_ms: u32 = match end_text.parse() {
    73 |         Ok(ms) => ms,
    74 |         Err(_) => {
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/long_feature.rs:8

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
     5 | use crate::parser::tree_parsing::parser_helpers::cst_assertions::{
     6 |     assert_child_count_exact, assert_child_kind_one_of, expect_child, expect_child_at,
     7 | };
>    8 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     9 | use tree_sitter::Node;
    10 | use tree_sitter_talkbank::node_types::{
    11 |     AMPERSAND, LONG_FEATURE_BEGIN, LONG_FEATURE_BEGIN_MARKER, LONG_FEATURE_END,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/long_feature.rs:72

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
    69 |                 "long_feature_begin",
    70 |             )?;
    71 |             let label_text =
>   72 |                 extract_utf8_text(label_node, source, errors, "long_feature_begin_label", "");
    73 |             let span = Span::new(
    74 |                 feature_child.start_byte() as u32,
    75 |                 feature_child.end_byte() as u32,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/long_feature.rs:114

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
   111 |                 "long_feature_end",
   112 |             )?;
   113 |             let label_text =
>  114 |                 extract_utf8_text(label_node, source, errors, "long_feature_end_label", "");
   115 |             let span = Span::new(
   116 |                 feature_child.start_byte() as u32,
   117 |                 feature_child.end_byte() as u32,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/nonvocal.rs:8

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
     5 | use crate::parser::tree_parsing::parser_helpers::cst_assertions::{
     6 |     assert_child_count_exact, assert_child_kind_one_of, expect_child, expect_child_at,
     7 | };
>    8 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     9 | use tree_sitter::Node;
    10 | use tree_sitter_talkbank::node_types::{
    11 |     AMPERSAND, LONG_FEATURE_LABEL, NONVOCAL_BEGIN, NONVOCAL_BEGIN_MARKER, NONVOCAL_END,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/nonvocal.rs:72

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
    69 |                 "nonvocal_begin",
    70 |             )?;
    71 |             let label_text =
>   72 |                 extract_utf8_text(label_node, source, errors, "nonvocal_begin_label", "");
    73 |             let span = Span::new(
    74 |                 nonvocal_child.start_byte() as u32,
    75 |                 nonvocal_child.end_byte() as u32,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/nonvocal.rs:106

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
   103 |                 "nonvocal_end",
   104 |             )?;
   105 |             let label_text =
>  106 |                 extract_utf8_text(label_node, source, errors, "nonvocal_end_label", "");
   107 |             let span = Span::new(
   108 |                 nonvocal_child.start_byte() as u32,
   109 |                 nonvocal_child.end_byte() as u32,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/nonvocal.rs:157

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
   154 |                 "nonvocal_simple",
   155 |             )?;
   156 |             let label_text =
>  157 |                 extract_utf8_text(label_node, source, errors, "nonvocal_simple_label", "");
   158 |             let span = Span::new(
   159 |                 nonvocal_child.start_byte() as u32,
   160 |                 nonvocal_child.end_byte() as u32,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/other_spoken.rs:6

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
     3 | use crate::parser::tree_parsing::parser_helpers::cst_assertions::{
     4 |     assert_child_count_exact, expect_child,
     5 | };
>    6 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     7 | use tree_sitter::Node;
     8 | use tree_sitter_talkbank::node_types::{AMPERSAND, COLON, SPEAKER, STANDALONE_WORD, STAR};
     9 | 
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/other_spoken.rs:46

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
    43 |         "other_spoken_event",
    44 |     )?;
    45 | 
>   46 |     let speaker_text = extract_utf8_text(speaker_node, source, errors, "speaker", "");
    47 |     let text = extract_utf8_text(text_node, source, errors, "other_spoken_text", "");
    48 | 
    49 |     let event = OtherSpokenEvent::new(speaker_text, text);
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/other_spoken.rs:47

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
    44 |     )?;
    45 | 
    46 |     let speaker_text = extract_utf8_text(speaker_node, source, errors, "speaker", "");
>   47 |     let text = extract_utf8_text(text_node, source, errors, "other_spoken_text", "");
    48 | 
    49 |     let event = OtherSpokenEvent::new(speaker_text, text);
    50 |     Some(UtteranceContent::OtherSpokenEvent(event))
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/base/overlap_point.rs:27

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
    24 |     errors: &impl ErrorSink,
    25 | ) -> Option<UtteranceContent> {
    26 |     // Extract text from atomic token
>   27 |     let text = match node.utf8_text(source.as_bytes()) {
    28 |         Ok(t) => t,
    29 |         Err(e) => {
    30 |             errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/errors.rs:8

- Best-guess leaf status: **Non-leaf**

- Model construct: **Main tier content (UtteranceContent)**

```rust
     5 | 
     6 | /// Analyze ERROR node in word/content context to generate specific error code
     7 | pub(crate) fn analyze_word_error(error_node: Node, source: &str) -> ParseError {
>    8 |     let error_text = error_node.utf8_text(source.as_bytes()).unwrap_or_default();
     9 |     // DEFAULT: Invalid UTF-8 yields empty error text for analysis.
    10 | 
    11 |     // E311: Unclosed replacement bracket (PRIORITY 1)
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/content/nonword.rs:59

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier content (UtteranceContent)**

```rust
    56 |                     if let Some(segment_child) = nonword_type.child(1)
    57 |                         && segment_child.kind() == EVENT_SEGMENT
    58 |                     {
>   59 |                         if let Ok(event_type) = segment_child.utf8_text(source.as_bytes()) {
    60 |                             parsed_nonword = Some(ParsedNonword::Event(
    61 |                                 Event::new(event_type).with_span(span),
    62 |                                 span,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/body.rs:65

- Best-guess leaf status: **Leaf**

- Model construct: **Unknown**

```rust
    62 |                 let mut langcode_cursor = child.walk();
    63 |                 for langcode_child in child.children(&mut langcode_cursor) {
    64 |                     if langcode_child.kind() == LANGUAGE_CODE {
>   65 |                         if let Ok(code_text) = langcode_child.utf8_text(source.as_bytes()) {
    66 |                             language_code = Some(code_text.to_string());
    67 |                         }
    68 |                         break;
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/convert/body.rs:160

- Best-guess leaf status: **Leaf**

- Model construct: **Unknown**

```rust
   157 |                 let mut langcode_cursor = child.walk();
   158 |                 for langcode_child in child.children(&mut langcode_cursor) {
   159 |                     if langcode_child.kind() == LANGUAGE_CODE {
>  160 |                         if let Ok(code_text) = langcode_child.utf8_text(source.as_bytes()) {
   161 |                             language_code = Some(code_text.to_string());
   162 |                         }
   163 |                         break;
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/structure/errors.rs:36

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    33 |     }
    34 | 
    35 |     if node.is_error() {
>   36 |         // Use the wrapped_source (not original_input) for node.utf8_text() to work correctly
    37 |         let mut error = analyze_error_node(node, _wrapped_source, "parse tree");
    38 | 
    39 |         // Calculate adjusted offsets for original_input
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/content.rs:6

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier word (Word model)**

```rust
     3 | use crate::parser::tree_parsing::helpers::unexpected_node_error;
     4 | use crate::parser::tree_parsing::parser_helpers::cst_assertions::expect_child_at;
     5 | use crate::parser::tree_parsing::parser_helpers::{
>    6 |     extract_utf8_text, parse_ca_delimiter_node, parse_ca_element_node, parse_overlap_point_node,
     7 | };
     8 | use std::borrow::Cow;
     9 | use tree_sitter::Node;
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/content.rs:162

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier word (Word model)**

```rust
   159 |             }
   160 |         }
   161 |         kind::INITIAL_WORD_SEGMENT | kind::WORD_SEGMENT => {
>  162 |             let text = extract_utf8_text(node, source, errors, "word_segment", "");
   163 |             content.push(WordContentBorrowed::Text(WordTextBorrowed::new(
   164 |                 Cow::Borrowed(text),
   165 |             )));
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/content.rs:241

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier word (Word model)**

```rust
   238 |             content.push(WordContentBorrowed::CADelimiter(ca));
   239 |         }
   240 |         other if is_ca_intonation(other) => {
>  241 |             let text = extract_utf8_text(node, source, errors, "ca_intonation", "");
   242 |             content.push(WordContentBorrowed::Text(WordTextBorrowed::new(
   243 |                 Cow::Borrowed(text),
   244 |             )));
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/content.rs:276

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier word (Word model)**

```rust
   273 |     while idx < child_count {
   274 |         if let Some(child) = node.child(idx as u32) {
   275 |             if child.kind() == kind::WORD_SEGMENT || child.kind() == kind::INITIAL_WORD_SEGMENT {
>  276 |                 return Cow::Borrowed(extract_utf8_text(child, source, errors, "shortening", ""));
   277 |             }
   278 |             idx += 1;
   279 |         } else {
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/form.rs:3

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier word (Word model)**

```rust
     1 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
     2 | use crate::model::FormType;
>    3 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     4 | use tree_sitter::Node;
     5 | 
     6 | pub fn parse_form_marker(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/form.rs:16

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier word (Word model)**

```rust
    13 | ) -> Option<FormType> {
    14 |     // Extract form_marker_token child node (e.g., "@u", "@b", etc.)
    15 |     let token_node = node.child_by_field_name("").or_else(|| node.child(0))?;
>   16 |     let token_text = extract_utf8_text(token_node, source, errors, "form_marker_token", "");
    17 | 
    18 |     // Grammar guarantees token starts with '@'; FormType::parse accepts with or without '@'.
    19 |     if let Some(ft) = FormType::parse(token_text) {
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/form.rs:51

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier word (Word model)**

```rust
    48 |     let content_node = node
    49 |         .child_by_field_name("special_form_content")
    50 |         .or_else(|| node.named_child(0))?;
>   51 |     let label = extract_utf8_text(content_node, source, errors, "special_form_content", "");
    52 | 
    53 |     if !label.is_empty() {
    54 |         Some(FormType::UserDefined(label.to_string()))
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:16

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier word (Word model)**

```rust
    13 | };
    14 | use crate::model::{FormType, Word, WordCategory, content::word::UntranscribedStatus};
    15 | use crate::parser::tree_parsing::helpers::unexpected_node_error;
>   16 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
    17 | use std::borrow::Cow;
    18 | use tree_sitter::Node;
    19 | use tree_sitter_talkbank::node_types as kind;
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:36

- Best-guess leaf status: **Leaf**

- Model construct: **Main tier word (Word model)**

```rust
    33 |     match lang_node.kind() {
    34 |         kind::LANGUAGE_CODE => {
    35 |             // Simple single language code
>   36 |             let code = extract_utf8_text(lang_node, source, errors, "language_code", "");
    37 |             Some(WordLanguageBorrowed::explicit(Cow::Borrowed(code)))
    38 |         }
    39 |         kind::MULTIPLE_LANGS => {
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:48

- Best-guess leaf status: **Leaf**

- Model construct: **Main tier word (Word model)**

```rust
    45 |             // First named child MUST be a language_code
    46 |             match lang_node.named_child(0) {
    47 |                 Some(first_child) if first_child.kind() == kind::LANGUAGE_CODE => {
>   48 |                     let code = extract_utf8_text(first_child, source, errors, "language_code", "");
    49 |                     codes.push(code.to_string().into());
    50 |                 }
    51 |                 Some(child) => {
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:78

- Best-guess leaf status: **Leaf**

- Model construct: **Main tier word (Word model)**

```rust
    75 |             let mut i = 1;
    76 |             while let Some(child) = lang_node.named_child(i) {
    77 |                 if child.kind() == kind::LANGUAGE_CODE {
>   78 |                     let code = extract_utf8_text(child, source, errors, "language_code", "");
    79 |                     codes.push(code.to_string().into());
    80 |                 } else {
    81 |                     errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:114

- Best-guess leaf status: **Leaf**

- Model construct: **Main tier word (Word model)**

```rust
   111 |             // First named child MUST be a language_code
   112 |             match lang_node.named_child(0) {
   113 |                 Some(first_child) if first_child.kind() == kind::LANGUAGE_CODE => {
>  114 |                     let code = extract_utf8_text(first_child, source, errors, "language_code", "");
   115 |                     codes.push(code.to_string().into());
   116 |                 }
   117 |                 Some(child) => {
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:144

- Best-guess leaf status: **Leaf**

- Model construct: **Main tier word (Word model)**

```rust
   141 |             let mut i = 1;
   142 |             while let Some(child) = lang_node.named_child(i) {
   143 |                 if child.kind() == kind::LANGUAGE_CODE {
>  144 |                     let code = extract_utf8_text(child, source, errors, "language_code", "");
   145 |                     codes.push(code.to_string().into());
   146 |                 } else {
   147 |                     errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:226

- Best-guess leaf status: **Unknown**

- Model construct: **Main tier word (Word model)**

```rust
   223 |     }
   224 | 
   225 |     let source_bytes = source.as_bytes();
>  226 |     let original_input = extract_utf8_text(node, source, errors, "word", "");
   227 |     // Use the word node's start byte instead of scanning the full source.
   228 |     let word_offset_in_wrapped = node.start_byte();
   229 | 
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/main_tier/word/mod.rs:242

- Best-guess leaf status: **Leaf**

- Model construct: **Main tier word (Word model)**

```rust
   239 | 
   240 |     while idx < child_count {
   241 |         if let Some(child) = node.child(idx as u32) {
>  242 |             let child_text = extract_utf8_text(child, source, errors, "word_child", "");
   243 |             match child.kind() {
   244 |                 kind::WORD_LANGS => {
   245 |                     // Grammar: word_langs = '@s' optional(colon language_code_or_variant)
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/media_bullet.rs:128

- Best-guess leaf status: **Unknown**

- Model construct: **Media bullet (Bullet)**

```rust
   125 |     };
   126 | 
   127 |     // Extract start time from position 1
>  128 |     let start_str = match start_node.utf8_text(source.as_bytes()) {
   129 |         Ok(s) => s,
   130 |         Err(e) => {
   131 |             errors.push(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/media_bullet.rs:160

- Best-guess leaf status: **Unknown**

- Model construct: **Media bullet (Bullet)**

```rust
   157 |     };
   158 | 
   159 |     // Extract end time from position 3
>  160 |     let end_str = match end_node.utf8_text(source.as_bytes()) {
   161 |         Ok(s) => s,
   162 |         Err(e) => {
   163 |             errors.push(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/media_bullet.rs:190

- Best-guess leaf status: **Unknown**

- Model construct: **Media bullet (Bullet)**

```rust
   187 |     let span = Span::new(node.start_byte() as u32, node.end_byte() as u32);
   188 | 
   189 |     if start_ms == 0 && end_ms == 0 {
>  190 |         let bullet_text = node.utf8_text(source.as_bytes()).unwrap_or({
   191 |             // DEFAULT: Invalid UTF-8 should still surface a placeholder in error reporting.
   192 |             "<invalid>"
   193 |         });
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:318

- Best-guess leaf status: **Unknown**

- Model construct: **Parser helper utilities**

```rust
   315 | ///
   316 | /// # Example
   317 | /// ```ignore
>  318 | /// let text = extract_utf8_text(node, source, errors, "word_text", "");
   319 | /// // If UTF-8 fails, error is reported and fallback is returned
   320 | /// ```
   321 | pub fn extract_utf8_text<'a>(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:321

- Best-guess leaf status: **Unknown**

- Model construct: **Parser helper utilities**

```rust
   318 | /// let text = extract_utf8_text(node, source, errors, "word_text", "");
   319 | /// // If UTF-8 fails, error is reported and fallback is returned
   320 | /// ```
>  321 | pub fn extract_utf8_text<'a>(
   322 |     node: Node,
   323 |     source: &'a str,
   324 |     errors: &impl ErrorSink,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/cst_assertions.rs:328

- Best-guess leaf status: **Unknown**

- Model construct: **Parser helper utilities**

```rust
   325 |     context: &str,
   326 |     fallback: &'a str,
   327 | ) -> &'a str {
>  328 |     match node.utf8_text(source.as_bytes()) {
   329 |         Ok(text) => text,
   330 |         Err(e) => {
   331 |             errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/dependent_tier.rs:9

- Best-guess leaf status: **Non-leaf**

- Model construct: **Parse error analysis context**

```rust
     6 |     source: &str,
     7 |     tier_type: Option<&str>,
     8 | ) -> ParseError {
>    9 |     let error_text = error_node.utf8_text(source.as_bytes()).unwrap_or_default();
    10 |     // DEFAULT: Invalid UTF-8 yields empty error text for analysis.
    11 |     let start = error_node.start_byte();
    12 |     let end = error_node.end_byte();
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs:2

- Best-guess leaf status: **Unknown**

- Model construct: **Parse error analysis context**

```rust
     1 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
>    2 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     3 | use tree_sitter::Node;
     4 | 
     5 | /// Analyze an ERROR node at file level to determine specific error code
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/file.rs:7

- Best-guess leaf status: **Unknown**

- Model construct: **Parse error analysis context**

```rust
     4 | 
     5 | /// Analyze an ERROR node at file level to determine specific error code
     6 | pub(crate) fn analyze_error_node(node: Node, source: &str, errors: &impl ErrorSink) {
>    7 |     let error_text = extract_utf8_text(node, source, errors, "file_error", "");
     8 |     let start = node.start_byte();
     9 |     let end = node.end_byte();
    10 | 
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/header.rs:2

- Best-guess leaf status: **Unknown**

- Model construct: **Parse error analysis context**

```rust
     1 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
>    2 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     3 | use tree_sitter::Node;
     4 | use tree_sitter_talkbank::node_types::{
     5 |     DATE_HEADER, HEADER, ID_HEADER, LANGUAGES_HEADER, MEDIA_HEADER, PARTICIPANTS_HEADER,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/header.rs:15

- Best-guess leaf status: **Unknown**

- Model construct: **Parse error analysis context**

```rust
    12 |     source: &str,
    13 |     errors: &impl ErrorSink,
    14 | ) {
>   15 |     let error_text = extract_utf8_text(error_node, source, errors, "header_error", "");
    16 |     let start = error_node.start_byte();
    17 |     let end = error_node.end_byte();
    18 | 
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/line.rs:2

- Best-guess leaf status: **Unknown**

- Model construct: **Parse error analysis context**

```rust
     1 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
>    2 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     3 | use crate::parser::tree_parsing::parser_helpers::{is_header, is_pre_begin_header};
     4 | use tree_sitter::Node;
     5 | use tree_sitter_talkbank::node_types::{HEADER, LINE, UTTERANCE};
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/line.rs:17

- Best-guess leaf status: **Unknown**

- Model construct: **Parse error analysis context**

```rust
    14 |     source: &str,
    15 |     errors: &impl ErrorSink,
    16 | ) {
>   17 |     let error_text = extract_utf8_text(error_node, source, errors, "line_error", "");
    18 |     let start = error_node.start_byte();
    19 |     let end = error_node.end_byte();
    20 | 
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/utterance.rs:2

- Best-guess leaf status: **Unknown**

- Model construct: **Parse error analysis context**

```rust
     1 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
>    2 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     3 | use tree_sitter::Node;
     4 | 
     5 | /// Analyze ERROR node in utterance context
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/utterance.rs:12

- Best-guess leaf status: **Non-leaf**

- Model construct: **Parse error analysis context**

```rust
     9 |     source: &str,
    10 |     errors: &impl ErrorSink,
    11 | ) {
>   12 |     let _line_text = extract_utf8_text(line_node, source, errors, "utterance_line", "");
    13 |     let error_text = extract_utf8_text(error_node, source, errors, "utterance_error", "");
    14 |     let start = error_node.start_byte();
    15 |     let end = error_node.end_byte();
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/error_analysis/utterance.rs:13

- Best-guess leaf status: **Unknown**

- Model construct: **Parse error analysis context**

```rust
    10 |     errors: &impl ErrorSink,
    11 | ) {
    12 |     let _line_text = extract_utf8_text(line_node, source, errors, "utterance_line", "");
>   13 |     let error_text = extract_utf8_text(error_node, source, errors, "utterance_error", "");
    14 |     let start = error_node.start_byte();
    15 |     let end = error_node.end_byte();
    16 | 
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/mod.rs:21

- Best-guess leaf status: **Unknown**

- Model construct: **Parser helper utilities**

```rust
    18 | #[allow(unused_imports)]
    19 | pub(crate) use cst_assertions::{
    20 |     assert_child_count_exact, assert_child_count_min, assert_child_kind, assert_child_kind_one_of,
>   21 |     check_not_missing, expect_child, expect_child_at, extract_utf8_text,
    22 | };
    23 | pub(crate) use error_analysis::{
    24 |     analyze_dependent_tier_error, analyze_error_node, analyze_line_error,
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/overlap.rs:23

- Best-guess leaf status: **Unknown**

- Model construct: **Parser helper utilities**

```rust
    20 |     errors: &impl ErrorSink,
    21 | ) -> Option<OverlapPoint> {
    22 |     // Extract text from atomic token
>   23 |     let text = match node.utf8_text(source.as_bytes()) {
    24 |         Ok(t) => t,
    25 |         Err(e) => {
    26 |             errors.report(ParseError::new(
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/pause.rs:6

- Best-guess leaf status: **Unknown**

- Model construct: **Parser helper utilities**

```rust
     3 | };
     4 | use crate::model::{Pause, PauseDuration, PauseTimedDuration};
     5 | use crate::parser::tree_parsing::parser_helpers::cst_assertions::{
>    6 |     assert_child_count_exact, assert_child_kind_one_of, expect_child, extract_utf8_text,
     7 | };
     8 | use tree_sitter::Node;
     9 | use tree_sitter_talkbank::node_types::{
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/pause.rs:95

- Best-guess leaf status: **Unknown**

- Model construct: **Parser helper utilities**

```rust
    92 |                     }
    93 |                 };
    94 |                 let duration_text =
>   95 |                     extract_utf8_text(duration_node, source, errors, "pause_timed duration", "0.0");
    96 |                 Pause::new(PauseDuration::Timed(PauseTimedDuration::new(
    97 |                     duration_text.to_string(),
    98 |                 )))
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/parser_helpers/node_dispatch/separator.rs:140

- Best-guess leaf status: **Unknown**

- Model construct: **Parser helper utilities**

```rust
   137 |     match node.kind() {
   138 |         tree_sitter_talkbank::node_types::SEPARATOR => {
   139 |             if node.child_count() == 0 {
>  140 |                 if let Ok(text) = node.utf8_text(source.as_bytes()) {
   141 |                     return match text {
   142 |                         ":" => Some(Separator::Colon {}),
   143 |                         "," => Some(Separator::Comma {}),
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/postcode/mod.rs:4

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
     1 | use crate::error::{ErrorCode, ErrorContext, ErrorSink, ParseError, Severity, SourceLocation};
     2 | use crate::model::Postcode;
     3 | use crate::parser::tree_parsing::helpers::unexpected_node_error;
>    4 | use crate::parser::tree_parsing::parser_helpers::extract_utf8_text;
     5 | use tree_sitter::Node;
     6 | 
     7 | /// Parse a single postcode node [+ text].
```

## rust/crates/talkbank-parser/src/parser/tree_parsing/postcode/mod.rs:28

- Best-guess leaf status: **Unknown**

- Model construct: **Unknown**

```rust
    25 |             let kind = child.kind();
    26 |             match kind {
    27 |                 "bracketed_content" => {
>   28 |                     text = extract_utf8_text(child, source, errors, "postcode_content", "")
    29 |                         .to_string();
    30 |                 }
    31 |                 "postcode_prefix" | "right_bracket" | "space" => {}
```
