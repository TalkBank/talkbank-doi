//! TRN content AST — lightweight representation of parsed TRN content elements.
//!
//! This is intentionally NOT a CHAT AST. It represents TRN conventions directly,
//! and a separate pass (`emit_chat.rs`) transforms these into CHAT text.

use crate::types::OverlapRole;

/// A single element of TRN content.
#[derive(Debug, Clone)]
pub enum TrnElement {
    /// Plain word text.
    Word(String),
    /// Overlap bracket with classification from the state machine.
    Overlap {
        role: OverlapRole,
        /// 0-based real index from the overlap run.
        real_index: usize,
    },
    /// Short pause `..`
    PauseShort,
    /// Medium pause `...`
    PauseMedium,
    /// Timed pause `...(N.N)` — value in seconds.
    PauseTimed(String),
    /// Lengthening `=` (attached to preceding word, converted during parsing).
    /// Stored as the word with `=` already replaced by `:`.
    // (Handled inline in word parsing, not a separate element.)

    /// Inhalation `(H)`
    Inhalation,
    /// Lengthened inhalation `(H)=`
    InhalationLengthened,
    /// Exhalation `(Hx)`
    Exhalation,
    /// Click `(TSK)`
    Click,
    /// General vocalism `(NAME)` — e.g., SNIFF, THROAT, COUGH.
    Vocalism(String),

    /// Single laugh `@`
    Laugh,
    /// Multiple laughs `@@@` etc.
    Laughs(usize),

    /// Environmental comment `((NAME))`
    Comment(String),

    /// Long feature begin `<NAME`
    LongFeatureBegin(String),
    /// Long feature end `NAME>`
    LongFeatureEnd(String),

    /// Nonvocal begin `<<NAME`
    NonvocalBegin(String),
    /// Nonvocal end `NAME>>`
    NonvocalEnd(String),
    /// Simple nonvocal `<<NAME>>`
    NonvocalSimple(String),
    /// Beat within nonvocal scope `+`
    NonvocalBeat,

    /// Truncation `--`
    Truncation,

    /// Continuation linker `&`
    Linker,

    /// Comma `,`
    Comma,
    /// Period `.`
    Period,
    /// Question mark `?`
    QuestionMark,

    /// Phonological fragment `/_word_/` or `/word/`
    PhonologicalFragment(String),

    /// Pseudograph prefix on next word: `~`, `!`, or `#`.
    /// The prefix is stripped; the word follows as a Word element.
    // (Handled inline — prefix stripped during word parsing.)

    /// Glottal stop `%` — context determines output.
    Glottal,

    /// Whitespace (preserved for spacing).
    Space,
}

/// Parse TRN raw content into a sequence of TrnElements.
///
/// `bracket_classifications` is a list of (char_offset, role, real_index) for
/// overlap brackets on this line, from the overlap state machine.
pub fn parse_trn_content(
    raw: &str,
    bracket_classifications: &[(usize, OverlapRole, usize)],
) -> Vec<TrnElement> {
    let chars: Vec<char> = raw.chars().collect();
    let len = chars.len();
    let mut elements = Vec::new();
    let mut i = 0;
    let mut word_buf = String::new();

    let flush_word = |buf: &mut String, elements: &mut Vec<TrnElement>| {
        if !buf.is_empty() {
            // Apply lengthening: replace = with : in words.
            // Apply glottal: replace % with ʔ in words (ʔuh if standalone).
            let transformed = buf.replace('=', ":").replace('%', "ʔ");
            elements.push(TrnElement::Word(transformed));
            buf.clear();
        }
    };

    while i < len {
        // Check if this position is a classified overlap bracket.
        if let Some(&(_, role, real_index)) = bracket_classifications.iter().find(|(off, _, _)| *off == i) {
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::Overlap { role, real_index });
            // Skip past the bracket and optional digit.
            i += 1;
            if i < len && chars[i].is_ascii_digit() && chars[i] >= '2' && chars[i] <= '9' {
                i += 1; // Skip index digit after [.
            }
            continue;
        }

        // Close bracket: check if classified.
        if chars[i] == ']' {
            // Already handled by bracket_classifications for the close position.
            // If not classified (orphan), skip it.
            flush_word(&mut word_buf, &mut elements);
            i += 1;
            continue;
        }

        // Digit before ] — part of overlap close, skip.
        if i + 1 < len && chars[i] >= '2' && chars[i] <= '9' && chars[i + 1] == ']' {
            if bracket_classifications.iter().any(|(off, _, _)| *off == i) {
                flush_word(&mut word_buf, &mut elements);
                elements.push(TrnElement::Overlap {
                    role: bracket_classifications.iter().find(|(off, _, _)| *off == i).unwrap().1,
                    real_index: bracket_classifications.iter().find(|(off, _, _)| *off == i).unwrap().2,
                });
                i += 2; // digit + ]
                continue;
            }
            // Unclassified close digit — skip both.
            flush_word(&mut word_buf, &mut elements);
            i += 2;
            continue;
        }

        // Forced close: $]
        if chars[i] == '$' && i + 1 < len && chars[i + 1] == ']' {
            if bracket_classifications.iter().any(|(off, _, _)| *off == i) {
                flush_word(&mut word_buf, &mut elements);
                elements.push(TrnElement::Overlap {
                    role: bracket_classifications.iter().find(|(off, _, _)| *off == i).unwrap().1,
                    real_index: bracket_classifications.iter().find(|(off, _, _)| *off == i).unwrap().2,
                });
            }
            i += 2;
            continue;
        }

        // Timed pause: ...(N.N)
        if i + 4 < len && chars[i] == '.' && chars[i + 1] == '.' && chars[i + 2] == '.' && chars[i + 3] == '(' {
            flush_word(&mut word_buf, &mut elements);
            if let Some(end) = chars[i + 3..].iter().position(|&c| c == ')') {
                let val: String = chars[i + 4..i + 3 + end].iter().collect();
                elements.push(TrnElement::PauseTimed(val));
                i = i + 3 + end + 1;
                continue;
            }
        }

        // Medium pause: ...
        if i + 2 < len && chars[i] == '.' && chars[i + 1] == '.' && chars[i + 2] == '.' {
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::PauseMedium);
            i += 3;
            continue;
        }

        // Short pause: ..
        if i + 1 < len && chars[i] == '.' && chars[i + 1] == '.' {
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::PauseShort);
            i += 2;
            continue;
        }

        // Environmental comment: ((NAME))
        if i + 1 < len && chars[i] == '(' && chars[i + 1] == '(' {
            flush_word(&mut word_buf, &mut elements);
            if let Some(end) = find_double_close(&chars, i + 2, ')') {
                let name: String = chars[i + 2..end].iter().collect();
                elements.push(TrnElement::Comment(name));
                i = end + 2;
                continue;
            }
        }

        // Vocalisms: (H), (Hx), (TSK), (NAME)
        if chars[i] == '(' && i + 2 < len && chars[i + 1] != '(' {
            flush_word(&mut word_buf, &mut elements);
            if let Some(close) = chars[i + 1..].iter().position(|&c| c == ')') {
                let inner: String = chars[i + 1..i + 1 + close].iter().collect();
                let element = match inner.as_str() {
                    "H" => TrnElement::Inhalation,
                    "Hx" | "HX" => TrnElement::Exhalation,
                    "TSK" => TrnElement::Click,
                    _ => TrnElement::Vocalism(inner),
                };
                elements.push(element);
                i = i + 1 + close + 1;
                // Check for lengthened: (H)=
                if i < len && chars[i] == '=' {
                    if let Some(last) = elements.last_mut() {
                        if matches!(last, TrnElement::Inhalation) {
                            *last = TrnElement::InhalationLengthened;
                        }
                    }
                    i += 1;
                }
                continue;
            }
        }

        // Nonvocal simple: <<NAME>>
        if i + 3 < len && chars[i] == '<' && chars[i + 1] == '<' {
            flush_word(&mut word_buf, &mut elements);
            if let Some(end) = find_double_close(&chars, i + 2, '>') {
                let name: String = chars[i + 2..end].iter().collect();
                // Check if this is simple (no content after) or begin.
                elements.push(TrnElement::NonvocalSimple(name));
                i = end + 2;
                continue;
            }
            // No close found — treat as begin.
            let rest: String = chars[i + 2..].iter().take_while(|c| c.is_ascii_uppercase() || **c == '_' || **c == '-').collect();
            if !rest.is_empty() {
                elements.push(TrnElement::NonvocalBegin(rest.clone()));
                i += 2 + rest.len();
                continue;
            }
        }

        // Nonvocal end: NAME>>
        // This is tricky — we need to look for UPPERCASE>> pattern.
        // Handled by checking for >> after accumulating a word.

        // Long feature begin: <NAME (single <, not <<)
        if chars[i] == '<' && (i + 1 >= len || chars[i + 1] != '<') {
            flush_word(&mut word_buf, &mut elements);
            let rest: String = chars[i + 1..].iter().take_while(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || **c == '@' || **c == '%').collect();
            if !rest.is_empty() {
                elements.push(TrnElement::LongFeatureBegin(rest.clone()));
                i += 1 + rest.len();
                continue;
            }
            // Not a long feature — pass through.
            word_buf.push(chars[i]);
            i += 1;
            continue;
        }

        // Long feature end: NAME> or @> (single >, not >>)
        if chars[i] == '>' && (i + 1 >= len || chars[i + 1] != '>') {
            // Check if word_buf ends with an uppercase/@ label.
            if !word_buf.is_empty() {
                let label_start = word_buf.rfind(|c: char| !c.is_ascii_uppercase() && !c.is_ascii_digit() && c != '@' && c != '%').map_or(0, |p| p + 1);
                if label_start < word_buf.len() {
                    let label = word_buf[label_start..].to_string();
                    word_buf.truncate(label_start);
                    flush_word(&mut word_buf, &mut elements);
                    elements.push(TrnElement::LongFeatureEnd(label));
                    i += 1;
                    continue;
                }
            }
            // Check if last element was a Laugh — it might be the @ of a long feature end.
            if let Some(TrnElement::Laugh) = elements.last() {
                flush_word(&mut word_buf, &mut elements);
                elements.pop(); // Remove the Laugh
                elements.push(TrnElement::LongFeatureEnd("@".to_string()));
                i += 1;
                continue;
            }
            // Bare > — pass through as word char.
            word_buf.push('>');
            i += 1;
            continue;
        }

        // Truncation: --
        if i + 1 < len && chars[i] == '-' && chars[i + 1] == '-' {
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::Truncation);
            i += 2;
            continue;
        }

        // Multiple laughs: @@@ etc.
        if chars[i] == '@' && (word_buf.is_empty() || word_buf.chars().all(|c| c == '@')) {
            if word_buf.is_empty() {
                // Count consecutive @s.
                let count = chars[i..].iter().take_while(|&&c| c == '@').count();
                if count > 1 {
                    elements.push(TrnElement::Laughs(count));
                    i += count;
                    continue;
                }
                // Single @ — could be laugh or prefix for laughing word.
                if i + 1 < len && chars[i + 1].is_ascii_alphabetic() {
                    // @ prefix on word — laughing word. Push @ into word_buf
                    // and let word accumulation handle it.
                    word_buf.push('@');
                    i += 1;
                    continue;
                }
                flush_word(&mut word_buf, &mut elements);
                elements.push(TrnElement::Laugh);
                i += 1;
                continue;
            }
        }

        // Glottal %
        if chars[i] == '%' {
            // If in a word context, accumulate (in-word glottal → ʔ).
            if !word_buf.is_empty() {
                word_buf.push('%');
                i += 1;
                continue;
            }
            // If followed by > — this is part of a long feature end label (%>).
            // Accumulate into word buffer so the > handler picks it up.
            if i + 1 < len && chars[i + 1] == '>' {
                word_buf.push('%');
                i += 1;
                continue;
            }
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::Glottal);
            i += 1;
            continue;
        }

        // Continuation linker &
        if chars[i] == '&' {
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::Linker);
            i += 1;
            continue;
        }

        // Beat + (within nonvocal scope)
        if chars[i] == '+' {
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::NonvocalBeat);
            i += 1;
            continue;
        }

        // Terminators / punctuation
        if chars[i] == '.' && (i + 1 >= len || chars[i + 1] != '.') {
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::Period);
            i += 1;
            continue;
        }
        if chars[i] == '?' {
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::QuestionMark);
            i += 1;
            continue;
        }
        if chars[i] == ',' {
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::Comma);
            i += 1;
            continue;
        }

        // Pseudograph prefixes: ~, !, # — strip and let word accumulate.
        if (chars[i] == '~' || chars[i] == '!' || chars[i] == '#') && word_buf.is_empty() {
            // Skip the prefix; next chars will be the word.
            i += 1;
            continue;
        }

        // Space
        if chars[i] == ' ' || chars[i] == '\t' {
            flush_word(&mut word_buf, &mut elements);
            elements.push(TrnElement::Space);
            // Collapse multiple spaces.
            while i < len && (chars[i] == ' ' || chars[i] == '\t') {
                i += 1;
            }
            continue;
        }

        // Phonological fragment: /word/
        if chars[i] == '/' {
            flush_word(&mut word_buf, &mut elements);
            if let Some(close) = chars[i + 1..].iter().position(|&c| c == '/') {
                let frag: String = chars[i + 1..i + 1 + close].iter().collect();
                elements.push(TrnElement::PhonologicalFragment(frag));
                i = i + 1 + close + 1;
                continue;
            }
            // No closing / — accumulate.
            word_buf.push('/');
            i += 1;
            continue;
        }

        // Check for LABEL>> (nonvocal end) — uppercase word followed by >>.
        if chars[i] == '>' && i + 1 < len && chars[i + 1] == '>' && !word_buf.is_empty() {
            // Check if word_buf is all uppercase (a nonvocal label).
            let label_start = word_buf.rfind(|c: char| !c.is_ascii_uppercase() && c != '_' && c != '-').map_or(0, |p| p + 1);
            if label_start < word_buf.len() {
                let label = word_buf[label_start..].to_string();
                word_buf.truncate(label_start);
                flush_word(&mut word_buf, &mut elements);
                elements.push(TrnElement::NonvocalEnd(label));
                i += 2; // Skip >>
                continue;
            }
        }

        // Default: accumulate into word.
        word_buf.push(chars[i]);
        i += 1;
    }

    flush_word(&mut word_buf, &mut elements);
    elements
}

fn find_double_close(chars: &[char], start: usize, close_char: char) -> Option<usize> {
    let mut i = start;
    while i + 1 < chars.len() {
        if chars[i] == close_char && chars[i + 1] == close_char {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Vec<TrnElement> {
        parse_trn_content(s, &[])
    }

    fn words(elements: &[TrnElement]) -> Vec<String> {
        elements.iter().filter_map(|e| match e {
            TrnElement::Word(w) => Some(w.clone()),
            _ => None,
        }).collect()
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
}
