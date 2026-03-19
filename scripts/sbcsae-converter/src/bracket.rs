use crate::types::{BracketKind, BracketToken};

/// Tokenize overlap brackets within a single line's content.
///
/// Finds all `[`, `]`, `[N`, `N]`, `$]`, `N$]` tokens, skipping
/// `((...))` environmental comments and `<<...>>` nonvocal sounds.
///
/// Returns tokens with their character offset within `content` and
/// absolute column position.
pub fn tokenize_brackets(
    content: &str,
    line_number: usize,
    content_column: usize,
) -> Vec<BracketToken> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Skip ((...)) environmental comments.
        if chars[i] == '(' && i + 1 < len && chars[i + 1] == '(' {
            i = skip_double_paren(&chars, i);
            continue;
        }

        // Note: do NOT skip <<...>> nonvocal sounds — overlap brackets can
        // appear inside them (e.g., <<CLAP +[4++++ CLAP>>4]).

        // Open bracket: [ or [N
        if chars[i] == '[' {
            let lexical_index = if i + 1 < len && is_overlap_digit(chars[i + 1]) {
                Some(chars[i + 1].to_digit(10).unwrap() as u8)
            } else {
                None
            };

            tokens.push(BracketToken {
                line_number,
                char_offset: i,
                column: content_column + i,
                kind: BracketKind::Open,
                lexical_index,
            });

            i += 1;
            if lexical_index.is_some() {
                i += 1; // Skip the digit.
            }
            continue;
        }

        // Close with digit: N] or N$]
        if is_overlap_digit(chars[i]) {
            if i + 1 < len && chars[i + 1] == ']' {
                tokens.push(BracketToken {
                    line_number,
                    char_offset: i,
                    column: content_column + i,
                    kind: BracketKind::Close,
                    lexical_index: Some(chars[i].to_digit(10).unwrap() as u8),
                });
                i += 2; // Skip digit and ].
                continue;
            }
            if i + 2 < len && chars[i + 1] == '$' && chars[i + 2] == ']' {
                tokens.push(BracketToken {
                    line_number,
                    char_offset: i,
                    column: content_column + i,
                    kind: BracketKind::CloseForced,
                    lexical_index: Some(chars[i].to_digit(10).unwrap() as u8),
                });
                i += 3;
                continue;
            }
        }

        // Forced close without digit: $]
        if chars[i] == '$' && i + 1 < len && chars[i + 1] == ']' {
            tokens.push(BracketToken {
                line_number,
                char_offset: i,
                column: content_column + i,
                kind: BracketKind::CloseForced,
                lexical_index: None,
            });
            i += 2;
            continue;
        }

        // Bare close: ]
        if chars[i] == ']' {
            tokens.push(BracketToken {
                line_number,
                char_offset: i,
                column: content_column + i,
                kind: BracketKind::Close,
                lexical_index: None,
            });
            i += 1;
            continue;
        }

        i += 1;
    }

    tokens
}

fn is_overlap_digit(c: char) -> bool {
    matches!(c, '2'..='9')
}

/// Skip past `((...))`, returning the index after the closing `))`.
fn skip_double_paren(chars: &[char], start: usize) -> usize {
    let mut i = start + 2;
    while i + 1 < chars.len() {
        if chars[i] == ')' && chars[i + 1] == ')' {
            return i + 2;
        }
        i += 1;
    }
    chars.len() // Unclosed — skip to end.
}

/// Skip past `<<...>>`, returning the index after the closing `>>`.
fn skip_double_angle(chars: &[char], start: usize) -> usize {
    let mut i = start + 2;
    while i + 1 < chars.len() {
        if chars[i] == '>' && chars[i + 1] == '>' {
            return i + 2;
        }
        i += 1;
    }
    chars.len() // Unclosed — skip to end.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn offsets(tokens: &[BracketToken]) -> Vec<(usize, BracketKind, Option<u8>)> {
        tokens
            .iter()
            .map(|t| (t.char_offset, t.kind, t.lexical_index))
            .collect()
    }

    #[test]
    fn simple_unnumbered() {
        let tokens = tokenize_brackets("[foo]", 1, 0);
        assert_eq!(
            offsets(&tokens),
            vec![
                (0, BracketKind::Open, None),
                (4, BracketKind::Close, None),
            ]
        );
    }

    #[test]
    fn numbered_pair() {
        let tokens = tokenize_brackets("[2bar2]", 1, 0);
        assert_eq!(
            offsets(&tokens),
            vec![
                (0, BracketKind::Open, Some(2)),
                (5, BracketKind::Close, Some(2)),
            ]
        );
    }

    #[test]
    fn adjacent_brackets() {
        // [4M4][5hm=5]
        let tokens = tokenize_brackets("[4M4][5hm=5]", 1, 0);
        assert_eq!(
            offsets(&tokens),
            vec![
                (0, BracketKind::Open, Some(4)),
                (3, BracketKind::Close, Some(4)),
                (5, BracketKind::Open, Some(5)),
                (10, BracketKind::Close, Some(5)),
            ]
        );
    }

    #[test]
    fn index_mismatch() {
        // [3mhm4] — open with 3, close with 4.
        let tokens = tokenize_brackets("[3mhm4]", 1, 0);
        assert_eq!(
            offsets(&tokens),
            vec![
                (0, BracketKind::Open, Some(3)),
                (5, BracketKind::Close, Some(4)),
            ]
        );
    }

    #[test]
    fn forced_close() {
        let tokens = tokenize_brackets("text$]more", 1, 0);
        assert_eq!(
            offsets(&tokens),
            vec![(4, BracketKind::CloseForced, None)]
        );
    }

    #[test]
    fn forced_close_with_digit() {
        let tokens = tokenize_brackets("text2$]more", 1, 0);
        assert_eq!(
            offsets(&tokens),
            vec![(4, BracketKind::CloseForced, Some(2))]
        );
    }

    #[test]
    fn skip_double_paren_comment() {
        // ((COMMENT [not a bracket])) but [real] is.
        let tokens = tokenize_brackets("((COMMENT [not])) but [real]", 1, 0);
        assert_eq!(
            offsets(&tokens),
            vec![
                (22, BracketKind::Open, None),
                (27, BracketKind::Close, None),
            ]
        );
    }

    #[test]
    fn skip_double_angle_nonvocal() {
        let tokens = tokenize_brackets("<<THUMP>> [overlap]", 1, 0);
        assert_eq!(
            offsets(&tokens),
            vec![
                (10, BracketKind::Open, None),
                (18, BracketKind::Close, None),
            ]
        );
    }

    #[test]
    fn digit_in_normal_text() {
        // "2 dogs" — the 2 is not followed by ] so it's not a bracket.
        let tokens = tokenize_brackets("2 dogs", 1, 0);
        assert!(tokens.is_empty());
    }

    #[test]
    fn column_offset() {
        let tokens = tokenize_brackets("some [text]", 1, 20);
        assert_eq!(tokens[0].column, 25); // 20 + 5
        assert_eq!(tokens[1].column, 30); // 20 + 10
    }

    #[test]
    fn bracket_inside_word() {
        // does[n't] — bracket inside a word.
        let tokens = tokenize_brackets("does[n't]", 1, 0);
        assert_eq!(
            offsets(&tokens),
            vec![
                (4, BracketKind::Open, None),
                (8, BracketKind::Close, None),
            ]
        );
    }

    #[test]
    fn empty_content() {
        let tokens = tokenize_brackets("", 1, 0);
        assert!(tokens.is_empty());
    }
}
