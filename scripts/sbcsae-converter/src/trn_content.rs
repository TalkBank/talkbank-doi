//! TRN content parser using winnow.
//!
//! Parses TRN raw content into a sequence of `TrnElement` values.
//! The parser handles context-sensitive tokens (%, @, =) correctly
//! by trying alternatives in priority order.

use crate::types::OverlapRole;
use winnow::combinator::{alt, opt, peek, repeat};
use winnow::prelude::*;
use winnow::token::{any, one_of, take_while};

/// A single element of TRN content.
#[derive(Debug, Clone)]
pub enum TrnElement {
    Word(String),
    Overlap { role: OverlapRole, real_index: usize },
    PauseShort,
    PauseMedium,
    PauseTimed(String),
    Inhalation,
    InhalationLengthened,
    Exhalation,
    Click,
    Vocalism(String),
    Laugh,
    Laughs(usize),
    Comment(String),
    LongFeatureBegin(String),
    LongFeatureEnd(String),
    NonvocalBegin(String),
    NonvocalEnd(String),
    NonvocalSimple(String),
    NonvocalBeat,
    Truncation,
    Linker,
    Comma,
    Period,
    QuestionMark,
    PhonologicalFragment(String),
    Glottal,
    Space,
}

/// State passed through the parser for overlap bracket context.
pub struct ParseContext<'a> {
    /// (char_offset, role, real_index) for brackets on this line.
    pub brackets: &'a [(usize, OverlapRole, usize)],
    /// Total byte length of the preprocessed string.
    pub start_len: usize,
    /// Map from char index to byte offset in preprocessed string.
    pub char_to_byte: &'a [usize],
    /// Total number of chars.
    pub total_chars: usize,
}

/// Parse TRN raw content into elements.
pub fn parse_trn_content(
    raw: &str,
    bracket_classifications: &[(usize, OverlapRole, usize)],
) -> Vec<TrnElement> {
    // Pre-process: replace bracket tokens with placeholder chars so the
    // winnow parser doesn't need to coordinate with bracket offsets.
    // Work with chars (not bytes) since bracket.rs uses char offsets.
    let mut char_vec: Vec<char> = raw.chars().collect();
    let mut bracket_at: Vec<(usize, OverlapRole, usize)> = Vec::new();

    for &(char_offset, role, real_index) in bracket_classifications {
        if char_offset >= char_vec.len() {
            continue;
        }
        // Determine how many chars this bracket token spans.
        let span = bracket_char_span(&char_vec, char_offset);
        // Replace with \x01 (open) or \x02 (close) + spaces for remaining chars.
        let marker = match role {
            OverlapRole::TopBegin | OverlapRole::BottomBegin => '\x01',
            OverlapRole::TopEnd | OverlapRole::BottomEnd => '\x02',
        };
        char_vec[char_offset] = marker;
        for i in 1..span {
            if char_offset + i < char_vec.len() {
                char_vec[char_offset + i] = ' ';
            }
        }
        bracket_at.push((char_offset, role, real_index));
    }

    let preprocessed: String = char_vec.iter().collect();
    // Build a char-offset → byte-offset mapping for the preprocessed string.
    let char_to_byte: Vec<usize> = preprocessed.char_indices().map(|(i, _)| i).collect();
    let ctx = ParseContext {
        brackets: &bracket_at,
        start_len: preprocessed.len(), // byte length
        char_to_byte: &char_to_byte,
        total_chars: char_vec.len(),
    };
    let mut elements = Vec::new();
    let mut input: &str = &preprocessed;

    while !input.is_empty() {
        let byte_offset = ctx.start_len - input.len();
        // Convert byte offset to char offset for bracket lookup.
        let char_offset = ctx.char_to_byte.iter().position(|&b| b == byte_offset).unwrap_or(0);

        // Check for bracket placeholders (\x01 = open, \x02 = close).
        if input.starts_with('\x01') || input.starts_with('\x02') {
            if let Some(&(_off, role, real_index)) = ctx.brackets.iter().find(|(o, _, _)| *o == char_offset) {
                elements.push(TrnElement::Overlap { role, real_index });
            }
            // Skip the placeholder char.
            input = &input[1..];
            continue;
        }

        // Try each element parser in priority order.
        if let Ok((rest, elem)) = parse_element.parse_peek(input) {
            elements.push(elem);
            input = rest;
        } else {
            // Fallback: consume one char as part of a word.
            let ch = input.chars().next().unwrap();
            // Merge into previous word if possible.
            if let Some(TrnElement::Word(w)) = elements.last_mut() {
                w.push(ch);
            } else {
                elements.push(TrnElement::Word(ch.to_string()));
            }
            input = &input[ch.len_utf8()..];
        }
    }

    // Post-process: apply lengthening (= → :) and glottal (% → ʔ) in words.
    for elem in &mut elements {
        if let TrnElement::Word(w) = elem {
            *w = w.replace('=', ":").replace('%', "ʔ");
        }
    }

    elements
}

/// Determine how many chars an overlap bracket token spans at the given char offset.
fn bracket_char_span(chars: &[char], offset: usize) -> usize {
    if offset >= chars.len() {
        return 1;
    }
    match chars[offset] {
        '[' => {
            if offset + 1 < chars.len() && chars[offset + 1] >= '2' && chars[offset + 1] <= '9' {
                2
            } else {
                1
            }
        }
        '2'..='9' => {
            if offset + 1 < chars.len() && chars[offset + 1] == ']' {
                2
            } else if offset + 2 < chars.len() && chars[offset + 1] == '$' && chars[offset + 2] == ']' {
                3
            } else {
                1
            }
        }
        '$' => {
            if offset + 1 < chars.len() && chars[offset + 1] == ']' {
                2
            } else {
                1
            }
        }
        ']' => 1,
        _ => 1,
    }
}

/// Skip a bracket open token: [ or [N (digit 2-9).
fn parse_bracket_open(input: &mut &str) -> ModalResult<()> {
    '['.parse_next(input)?;
    opt(one_of('2'..='9')).parse_next(input)?;
    Ok(())
}

/// Skip a bracket close token: ], N], $], or N$].
fn parse_bracket_close(input: &mut &str) -> ModalResult<()> {
    alt((
        // N$]
        (one_of('2'..='9'), '$', ']').void(),
        // $]
        ('$', ']').void(),
        // N]
        (one_of('2'..='9'), ']').void(),
        // bare ]
        ']'.void(),
    ))
    .parse_next(input)
}

/// Parse one TRN content element (not overlap brackets — those are handled separately).
/// Split into nested alt groups to stay within winnow's tuple size limit.
fn parse_element(input: &mut &str) -> ModalResult<TrnElement> {
    alt((
        alt((
            parse_space,
            parse_comment,
            parse_nonvocal,
            parse_long_feature,
            parse_timed_pause,
        )),
        alt((
            parse_medium_pause,
            parse_short_pause,
            parse_truncation,
            parse_vocalism,
            parse_laughs,
        )),
        alt((
            parse_laugh_or_word,
            parse_phonological,
            parse_linker,
            parse_nonvocal_beat,
            parse_glottal,
        )),
        alt((
            parse_question,
            parse_period,
            parse_comma,
            parse_pseudograph,
            parse_word,
        )),
    ))
    .parse_next(input)
}

fn parse_space(input: &mut &str) -> ModalResult<TrnElement> {
    take_while(1.., |c: char| c == ' ' || c == '\t').parse_next(input)?;
    Ok(TrnElement::Space)
}

/// `((NAME))` — environmental comment.
fn parse_comment(input: &mut &str) -> ModalResult<TrnElement> {
    "((". parse_next(input)?;
    let name: String = take_while(1.., |c: char| c != ')').parse_next(input)?.to_string();
    "))".parse_next(input)?;
    Ok(TrnElement::Comment(name))
}

/// `<<LABEL>>` (simple) or `<<LABEL` (begin) — nonvocal.
fn parse_nonvocal(input: &mut &str) -> ModalResult<TrnElement> {
    "<<".parse_next(input)?;
    let label: String = take_while(1.., |c: char| c.is_ascii_uppercase() || c == '_' || c == '-')
        .parse_next(input)?
        .to_string();
    // Check for simple close >>.
    if input.starts_with(">>") {
        ">>".parse_next(input)?;
        Ok(TrnElement::NonvocalSimple(label))
    } else {
        Ok(TrnElement::NonvocalBegin(label))
    }
}

/// `<LABEL` (begin) or detect `LABEL>` (end) — long feature.
fn parse_long_feature(input: &mut &str) -> ModalResult<TrnElement> {
    alt((parse_long_feature_begin, parse_long_feature_end)).parse_next(input)
}

fn parse_long_feature_begin(input: &mut &str) -> ModalResult<TrnElement> {
    // < not followed by < (that's nonvocal).
    '<'.parse_next(input)?;
    // Peek: must not be <.
    if input.starts_with('<') {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }
    let label: String =
        take_while(1.., |c: char| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '@' || c == '%')
            .parse_next(input)?
            .to_string();
    Ok(TrnElement::LongFeatureBegin(label))
}

/// Detect `LABEL>` or `LABEL>>` where LABEL is uppercase/digit/@/%/_/-.
/// Single `>` = long feature end. Double `>>` = nonvocal end.
fn parse_long_feature_end(input: &mut &str) -> ModalResult<TrnElement> {
    let label: String =
        take_while(1.., |c: char| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '@' || c == '%' || c == '_' || c == '-')
            .parse_next(input)?
            .to_string();
    // Must be followed by > or >>.
    '>'.parse_next(input)?;
    if input.starts_with('>') {
        '>'.parse_next(input)?;
        return Ok(TrnElement::NonvocalEnd(label));
    }
    Ok(TrnElement::LongFeatureEnd(label))
}

/// `...(N.N)` — timed pause.
fn parse_timed_pause(input: &mut &str) -> ModalResult<TrnElement> {
    "...(".parse_next(input)?;
    let val: String = take_while(1.., |c: char| c.is_ascii_digit() || c == '.')
        .parse_next(input)?
        .to_string();
    ')'.parse_next(input)?;
    Ok(TrnElement::PauseTimed(val))
}

/// `...` — medium pause (not followed by `(` + digit, which is timed pause).
fn parse_medium_pause(input: &mut &str) -> ModalResult<TrnElement> {
    "...".parse_next(input)?;
    // Must not be followed by ( + digit (that's timed pause `...(1.2)`).
    // But ...(H) is medium pause + vocalism, not timed pause.
    if input.starts_with('(') {
        let after_paren = input.get(1..2).unwrap_or("");
        if after_paren.starts_with(|c: char| c.is_ascii_digit()) {
            return Err(winnow::error::ErrMode::Backtrack(
                winnow::error::ContextError::new(),
            ));
        }
    }
    Ok(TrnElement::PauseMedium)
}

/// `..` — short pause.
fn parse_short_pause(input: &mut &str) -> ModalResult<TrnElement> {
    "..".parse_next(input)?;
    // Must not be followed by . (that's medium pause).
    if input.starts_with('.') {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }
    Ok(TrnElement::PauseShort)
}

/// `--` — truncation.
fn parse_truncation(input: &mut &str) -> ModalResult<TrnElement> {
    "--".parse_next(input)?;
    Ok(TrnElement::Truncation)
}

/// `(H)`, `(H)=`, `(Hx)`, `(TSK)`, or `(NAME)` — vocalism.
fn parse_vocalism(input: &mut &str) -> ModalResult<TrnElement> {
    '('.parse_next(input)?;
    // Must not be ( (that's comment).
    if input.starts_with('(') {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }
    let inner: String = take_while(1.., |c: char| c != ')')
        .parse_next(input)?
        .to_string();
    ')'.parse_next(input)?;

    let elem = match inner.as_str() {
        "H" => {
            // Check for lengthened: (H)=
            if input.starts_with('=') {
                let _ = '='.parse_next(input)?;
                TrnElement::InhalationLengthened
            } else {
                TrnElement::Inhalation
            }
        }
        "Hx" | "HX" => TrnElement::Exhalation,
        "TSK" => TrnElement::Click,
        _ => TrnElement::Vocalism(inner),
    };
    Ok(elem)
}

/// `@@@` — multiple laughs.
fn parse_laughs(input: &mut &str) -> ModalResult<TrnElement> {
    let ats: &str = take_while(2.., |c: char| c == '@').parse_next(input)?;
    Ok(TrnElement::Laughs(ats.len()))
}

/// `@` alone (laugh) or `@word` (laughing word — @ prefix kept in Word).
fn parse_laugh_or_word(input: &mut &str) -> ModalResult<TrnElement> {
    '@'.parse_next(input)?;
    // If followed by a letter, it's a laughing word — accumulate @+word.
    if input.starts_with(|c: char| c.is_ascii_alphabetic()) {
        let word: &str =
            take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '\'' || c == '=' || c == '%')
                .parse_next(input)?;
        Ok(TrnElement::Word(format!("@{word}")))
    } else {
        Ok(TrnElement::Laugh)
    }
}

/// `/word/` — phonological fragment.
fn parse_phonological(input: &mut &str) -> ModalResult<TrnElement> {
    '/'.parse_next(input)?;
    let word: String = take_while(1.., |c: char| c != '/')
        .parse_next(input)?
        .to_string();
    '/'.parse_next(input)?;
    Ok(TrnElement::PhonologicalFragment(word))
}

/// `&` — continuation linker.
fn parse_linker(input: &mut &str) -> ModalResult<TrnElement> {
    '&'.parse_next(input)?;
    Ok(TrnElement::Linker)
}

/// `+` — nonvocal beat.
fn parse_nonvocal_beat(input: &mut &str) -> ModalResult<TrnElement> {
    '+'.parse_next(input)?;
    Ok(TrnElement::NonvocalBeat)
}

/// `%` standalone — glottal stop.
fn parse_glottal(input: &mut &str) -> ModalResult<TrnElement> {
    '%'.parse_next(input)?;
    // If followed by > or by a letter (word context), backtrack — let word parser handle.
    if input.starts_with('>') || input.starts_with(|c: char| c.is_ascii_alphabetic()) {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }
    Ok(TrnElement::Glottal)
}

fn parse_question(input: &mut &str) -> ModalResult<TrnElement> {
    '?'.parse_next(input)?;
    Ok(TrnElement::QuestionMark)
}

/// `.` not followed by `.` — period terminator.
fn parse_period(input: &mut &str) -> ModalResult<TrnElement> {
    '.'.parse_next(input)?;
    if input.starts_with('.') {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }
    Ok(TrnElement::Period)
}

fn parse_comma(input: &mut &str) -> ModalResult<TrnElement> {
    ','.parse_next(input)?;
    Ok(TrnElement::Comma)
}

/// `~word`, `!word`, `#word` — pseudograph prefix (stripped).
fn parse_pseudograph(input: &mut &str) -> ModalResult<TrnElement> {
    one_of(['~', '!', '#']).parse_next(input)?;
    // Must be followed by a letter (the proper name).
    if !input.starts_with(|c: char| c.is_ascii_alphabetic()) {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }
    let word: &str =
        take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '\'' || c == '=' || c == '_')
            .parse_next(input)?;
    Ok(TrnElement::Word(word.to_string()))
}

/// A plain word — letters, digits, hyphens, apostrophes, =, %, underscore, non-ASCII.
fn parse_word(input: &mut &str) -> ModalResult<TrnElement> {
    let word: &str = take_while(1.., |c: char| {
        c.is_ascii_alphanumeric()
            || c == '-'
            || c == '\''
            || c == '='
            || c == '%'
            || c == '_'
            || c > '\x7f' // Allow ISO-8859-1 / non-ASCII chars
    })
    .parse_next(input)?;
    Ok(TrnElement::Word(word.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Vec<TrnElement> {
        parse_trn_content(s, &[])
    }

    fn words(elements: &[TrnElement]) -> Vec<String> {
        elements
            .iter()
            .filter_map(|e| match e {
                TrnElement::Word(w) => Some(w.clone()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn simple_words() {
        let elems = parse("How are you");
        assert_eq!(words(&elems), vec!["How", "are", "you"]);
    }

    #[test]
    fn lengthening() {
        let elems = parse("ta=p da=nce");
        assert_eq!(words(&elems), vec!["ta:p", "da:nce"]);
    }

    #[test]
    fn pauses() {
        let elems = parse(".. hello ... world");
        assert!(matches!(elems[0], TrnElement::PauseShort));
        assert!(matches!(elems[4], TrnElement::PauseMedium));
    }

    #[test]
    fn timed_pause() {
        let elems = parse("...(1.2)");
        assert!(matches!(&elems[0], TrnElement::PauseTimed(v) if v == "1.2"));
    }

    #[test]
    fn inhalation() {
        let elems = parse("(H) word");
        assert!(matches!(elems[0], TrnElement::Inhalation));
    }

    #[test]
    fn inhalation_lengthened() {
        let elems = parse("(H)=");
        assert!(matches!(elems[0], TrnElement::InhalationLengthened));
    }

    #[test]
    fn comment() {
        let elems = parse("((DOOR_SLAM))");
        assert!(matches!(&elems[0], TrnElement::Comment(s) if s == "DOOR_SLAM"));
    }

    #[test]
    fn truncation() {
        let elems = parse("word --");
        assert!(matches!(elems[2], TrnElement::Truncation));
    }

    #[test]
    fn laughs() {
        let elems = parse("@@@");
        assert!(matches!(elems[0], TrnElement::Laughs(3)));
    }

    #[test]
    fn laughing_word() {
        let elems = parse("@Yeah");
        assert_eq!(words(&elems), vec!["@Yeah"]);
    }

    #[test]
    fn long_feature_pair() {
        let elems = parse("<X hello X>");
        assert!(matches!(&elems[0], TrnElement::LongFeatureBegin(s) if s == "X"));
        assert_eq!(words(&elems), vec!["hello"]);
        assert!(matches!(&elems[4], TrnElement::LongFeatureEnd(s) if s == "X"));
    }

    #[test]
    fn long_feature_with_percent() {
        // <% word %> — % is a label, not glottal. Spaces separate label from content.
        let elems = parse("<% word %>");
        assert!(matches!(&elems[0], TrnElement::LongFeatureBegin(s) if s == "%"));
        assert_eq!(words(&elems), vec!["word"]);
        assert!(matches!(&elems[4], TrnElement::LongFeatureEnd(s) if s == "%"));
    }

    #[test]
    fn long_feature_with_at() {
        let elems = parse("<@ laughing @>");
        assert!(matches!(&elems[0], TrnElement::LongFeatureBegin(s) if s == "@"));
        assert!(matches!(&elems[4], TrnElement::LongFeatureEnd(s) if s == "@"));
    }

    #[test]
    fn nonvocal_simple() {
        let elems = parse("<<THUMP>>");
        assert!(matches!(&elems[0], TrnElement::NonvocalSimple(s) if s == "THUMP"));
    }

    #[test]
    fn nonvocal_begin_end() {
        let elems = parse("<<CLAP stuff CLAP>>");
        assert!(matches!(&elems[0], TrnElement::NonvocalBegin(s) if s == "CLAP"));
        // The end CLAP>> is parsed as LongFeatureEnd which falls back to NonvocalEnd.
        let has_end = elems.iter().any(|e| matches!(e, TrnElement::NonvocalEnd(s) if s == "CLAP"));
        assert!(has_end);
    }

    #[test]
    fn phonological_fragment() {
        let elems = parse("/hello/");
        assert!(matches!(&elems[0], TrnElement::PhonologicalFragment(s) if s == "hello"));
    }

    #[test]
    fn glottal_standalone() {
        let elems = parse("% --");
        assert!(matches!(elems[0], TrnElement::Glottal));
    }

    #[test]
    fn glottal_in_word() {
        let elems = parse("a%b");
        // % in word → ʔ
        assert_eq!(words(&elems), vec!["aʔb"]);
    }

    #[test]
    fn pseudograph_tilde() {
        let elems = parse("~John");
        assert_eq!(words(&elems), vec!["John"]);
    }

    #[test]
    fn exhalation() {
        let elems = parse("(Hx)");
        assert!(matches!(elems[0], TrnElement::Exhalation));
    }

    #[test]
    fn click() {
        let elems = parse("(TSK)");
        assert!(matches!(elems[0], TrnElement::Click));
    }
}
