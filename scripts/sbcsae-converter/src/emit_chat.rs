//! Emit CHAT text from parsed TRN data.
//!
//! Transforms TRN content elements into CHAT format, producing complete `.cha` files.

use std::collections::BTreeMap;
use std::fmt::Write;

use crate::trn_content::TrnElement;
use crate::types::*;

const MAX_OVERLAPS: usize = 9;

/// Emit a complete CHAT file for one TRN file.
pub fn emit_chat_file(
    file_stem: &str,
    speaker_map: &BTreeMap<String, String>,
    utterances: &[ChatUtterance],
) -> String {
    let mut out = String::new();

    // Headers.
    writeln!(out, "@UTF8").unwrap();
    writeln!(out, "@Begin").unwrap();
    writeln!(out, "@Languages:\teng").unwrap();

    // @Participants
    let parts: Vec<String> = speaker_map
        .iter()
        .map(|(trn_name, chat_id)| {
            let role = if trn_name.starts_with('>') { "Environment" } else { "Speaker" };
            format!("{chat_id} {role}")
        })
        .collect();
    writeln!(out, "@Participants:\t{}", parts.join(", ")).unwrap();

    writeln!(out, "@Options:\tCA").unwrap();

    // @ID lines.
    for (trn_name, chat_id) in speaker_map {
        let role = if trn_name.starts_with('>') { "Environment" } else { "Speaker" };
        writeln!(out, "@ID:\teng|SBCSAE|{chat_id}|||||{role}|||").unwrap();
    }

    // @Media
    writeln!(out, "@Media:\t{file_stem}, audio").unwrap();

    // Sort utterances by start time for monotonic timestamps.
    let mut sorted_utterances: Vec<&ChatUtterance> = utterances.iter().collect();
    sorted_utterances.sort_by(|a, b| {
        let a_start = a.start_ms.unwrap_or(0);
        let b_start = b.start_ms.unwrap_or(0);
        a_start.cmp(&b_start)
    });

    // Utterances.
    for utt in &sorted_utterances {
        let chat_id = speaker_map
            .get(&utt.speaker)
            .map(|s| s.as_str())
            .unwrap_or(&utt.speaker);

        let prefix = if utt.speaker.starts_with('>') {
            format!("%com:\t")
        } else {
            format!("*{chat_id}:\t")
        };

        let content = emit_utterance_content(&utt.elements);
        let terminator = emit_terminator(&utt.terminator);
        let bullet = emit_bullet(utt.start_ms, utt.end_ms);

        write!(out, "{prefix}{content}").unwrap();
        if !terminator.is_empty() {
            write!(out, " {terminator}").unwrap();
        }
        if let Some(ref b) = bullet {
            write!(out, " {b}").unwrap();
        }
        writeln!(out).unwrap();
    }

    writeln!(out, "@End").unwrap();
    out
}

/// An utterance ready for CHAT emission.
pub struct ChatUtterance {
    pub speaker: String,
    pub elements: Vec<TrnElement>,
    pub terminator: ChatTerminator,
    pub start_ms: Option<i64>,
    pub end_ms: Option<i64>,
}

#[derive(Debug, Clone)]
pub enum ChatTerminator {
    Period,
    Question,
    /// +/. (interruption by other speaker)
    Interruption,
    /// +... (trailing off, same speaker continues)
    TrailOff,
    /// +, (self-completion / continuation)
    SelfCompletion,
    /// No explicit terminator — insert period.
    Inserted,
}

fn emit_utterance_content(elements: &[TrnElement]) -> String {
    let mut out = String::new();
    let mut need_space = false;

    for elem in elements {
        let (text, space_before, space_after) = emit_element(elem);
        if text.is_empty() {
            continue;
        }

        if need_space && space_before {
            out.push(' ');
        }
        out.push_str(&text);
        need_space = space_after;
    }

    out
}

/// Returns (text, needs_space_before, provides_space_after).
fn emit_element(elem: &TrnElement) -> (String, bool, bool) {
    match elem {
        TrnElement::Word(w) => {
            // Handle @ prefix (laughing word): @Word → Word [% laugh]
            if let Some(rest) = w.strip_prefix('@') {
                (format!("{rest} [% laugh]"), true, true)
            } else {
                (w.clone(), true, true)
            }
        }
        TrnElement::Overlap { role, real_index } => {
            let marker = match role {
                OverlapRole::TopBegin => "⌈",
                OverlapRole::TopEnd => "⌉",
                OverlapRole::BottomBegin => "⌊",
                OverlapRole::BottomEnd => "⌋",
            };
            let idx = real_index % MAX_OVERLAPS;
            let index_str = if idx == 0 {
                String::new()
            } else {
                format!("{}", idx + 1)
            };
            // Overlap markers get spaces around them in CHAT.
            (format!("{marker}{index_str}"), true, true)
        }
        TrnElement::PauseShort => ("(..)".to_string(), true, true),
        TrnElement::PauseMedium => ("(...)".to_string(), true, true),
        TrnElement::PauseTimed(val) => (format!("({val})"), true, true),
        TrnElement::Inhalation => ("&=in".to_string(), true, true),
        TrnElement::InhalationLengthened => ("&=in &=lengthened".to_string(), true, true),
        TrnElement::Exhalation => ("&=ex".to_string(), true, true),
        TrnElement::Click => ("&=tsk".to_string(), true, true),
        TrnElement::Vocalism(name) => {
            // Handle % prefix: (%Hx) → ʔuh &=ex, (%NAME) → ʔuh &=NAME
            if let Some(rest) = name.strip_prefix('%') {
                let vocalism = match rest {
                    "Hx" | "HX" => "&=ex".to_string(),
                    _ => format!("&={}", rest.to_lowercase()),
                };
                (format!("ʔuh {vocalism}"), true, true)
            } else {
                (format!("&={}", name.to_lowercase()), true, true)
            }
        }
        TrnElement::Laugh => ("&=laugh".to_string(), true, true),
        TrnElement::Laughs(n) => {
            let text = (0..*n).map(|_| "&=laugh").collect::<Vec<_>>().join(" ");
            (text, true, true)
        }
        TrnElement::Comment(name) => {
            // Sanitize: commas → _AND_, periods → _POINT_ (matching Brian's convention).
            let sanitized = name.replace(',', "_AND").replace('.', "_POINT_");
            (format!("&={sanitized}"), true, true)
        }
        TrnElement::LongFeatureBegin(label) => (format!("&{{l={label}"), true, true),
        TrnElement::LongFeatureEnd(label) => (format!("&}}l={label}"), true, true),
        TrnElement::NonvocalBegin(label) => (format!("&{{n={label}"), true, true),
        TrnElement::NonvocalEnd(label) => (format!("&}}n={label}"), true, true),
        TrnElement::NonvocalSimple(label) => (format!("&={{n={label}}}"), true, true),
        TrnElement::NonvocalBeat => ("&=nonvocal".to_string(), true, true),
        TrnElement::Truncation => {
            // Truncation is handled at the utterance level as a terminator.
            // If it appears mid-content, emit as word-level truncation.
            (String::new(), false, false)
        }
        TrnElement::Linker => {
            // Handled at utterance grouping level.
            (String::new(), false, false)
        }
        TrnElement::Comma => {
            // Commas in TRN are intonation markers — emit as-is for now.
            (",".to_string(), false, true)
        }
        TrnElement::Period => {
            // Handled as terminator at utterance level.
            (String::new(), false, false)
        }
        TrnElement::QuestionMark => {
            // Handled as terminator at utterance level.
            (String::new(), false, false)
        }
        TrnElement::PhonologicalFragment(text) => (format!("/{text}/"), true, true),
        TrnElement::Glottal => {
            // Standalone glottal → ʔuh. In-word glottal handled during word parsing.
            ("ʔuh".to_string(), true, true)
        }
        TrnElement::Space => (String::new(), false, false),
    }
}

fn emit_terminator(term: &ChatTerminator) -> &'static str {
    match term {
        ChatTerminator::Period => ".",
        ChatTerminator::Question => "?",
        ChatTerminator::Interruption => "+/.",
        ChatTerminator::TrailOff => "+...",
        ChatTerminator::SelfCompletion => "+,",
        ChatTerminator::Inserted => ".",
    }
}

fn emit_bullet(start_ms: Option<i64>, end_ms: Option<i64>) -> Option<String> {
    match (start_ms, end_ms) {
        (Some(s), Some(e)) => Some(format!("\x15{s}_{e}\x15")),
        _ => None,
    }
}

/// Convert TRN timestamp (seconds as f64) to milliseconds.
pub fn time_to_ms(t: f64) -> i64 {
    (t * 1000.0).round() as i64
}

/// Group parsed TRN lines into CHAT utterances.
///
/// A TRN turn (speaker line + continuation lines) becomes one or more CHAT
/// utterances, split at terminators. The last line's timestamp becomes the bullet.
pub fn group_into_utterances(
    lines: &[TrnLine],
    line_elements: &[Vec<TrnElement>],
) -> Vec<ChatUtterance> {
    let mut utterances = Vec::new();
    let mut current_elements: Vec<TrnElement> = Vec::new();
    let mut current_speaker: Option<String> = None;
    let mut current_start_ms: Option<i64> = None;
    let mut current_end_ms: Option<i64> = None;

    for (line, elements) in lines.iter().zip(line_elements.iter()) {
        // Skip zero-timestamp annotator comment lines ($ lines).
        if line.start_time == 0.0 && line.end_time == 0.0
            && line.raw_content.trim_start().starts_with('$')
        {
            continue;
        }
        let is_new_speaker = line.speaker.is_some()
            && line.speaker.as_deref() != current_speaker.as_deref();

        if is_new_speaker {
            // Flush previous utterance.
            if !current_elements.is_empty() {
                let term = extract_terminator(&mut current_elements);
                utterances.push(ChatUtterance {
                    speaker: current_speaker.clone().unwrap_or_default(),
                    elements: std::mem::take(&mut current_elements),
                    terminator: term,
                    start_ms: current_start_ms,
                    end_ms: current_end_ms,
                });
            }
            current_speaker = line.speaker.clone();
            current_start_ms = Some(time_to_ms(line.start_time));
            current_end_ms = Some(time_to_ms(line.end_time));
            current_elements.extend(elements.iter().cloned());
        } else {
            // Continuation or same speaker — check for internal terminators.
            // If the previous content ended with a terminator, flush and start new.
            let has_terminator = current_elements.iter().any(|e| {
                matches!(
                    e,
                    TrnElement::Period | TrnElement::QuestionMark | TrnElement::Truncation
                )
            });

            if has_terminator && !elements.is_empty() {
                let term = extract_terminator(&mut current_elements);
                let next_is_same_speaker = line.speaker.is_none()
                    || line.speaker.as_deref() == current_speaker.as_deref();

                // Determine terminator type for truncation.
                let final_term = if matches!(term, ChatTerminator::Inserted) && next_is_same_speaker {
                    ChatTerminator::SelfCompletion
                } else {
                    term
                };

                utterances.push(ChatUtterance {
                    speaker: current_speaker.clone().unwrap_or_default(),
                    elements: std::mem::take(&mut current_elements),
                    terminator: final_term,
                    start_ms: current_start_ms,
                    end_ms: current_end_ms,
                });
                current_start_ms = Some(time_to_ms(line.start_time));
            }

            current_end_ms = Some(time_to_ms(line.end_time));
            current_elements.extend(elements.iter().cloned());
        }
    }

    // Flush final utterance.
    if !current_elements.is_empty() {
        let term = extract_terminator(&mut current_elements);
        utterances.push(ChatUtterance {
            speaker: current_speaker.unwrap_or_default(),
            elements: current_elements,
            terminator: term,
            start_ms: current_start_ms,
            end_ms: current_end_ms,
        });
    }

    utterances
}

/// Remove the last terminator-like element from the list and return the
/// appropriate ChatTerminator. If none found, return Inserted (period).
fn extract_terminator(elements: &mut Vec<TrnElement>) -> ChatTerminator {
    // Find last terminator element (ignoring trailing spaces).
    let mut term_idx = None;
    for (i, elem) in elements.iter().enumerate().rev() {
        match elem {
            TrnElement::Space => continue,
            TrnElement::Period => {
                term_idx = Some((i, ChatTerminator::Period));
                break;
            }
            TrnElement::QuestionMark => {
                term_idx = Some((i, ChatTerminator::Question));
                break;
            }
            TrnElement::Truncation => {
                // Truncation → interruption by default; caller may override.
                term_idx = Some((i, ChatTerminator::Interruption));
                break;
            }
            TrnElement::Comma => {
                // Comma at end of turn → period.
                term_idx = Some((i, ChatTerminator::Period));
                break;
            }
            TrnElement::Linker => {
                // & at end → self-completion.
                term_idx = Some((i, ChatTerminator::SelfCompletion));
                break;
            }
            _ => break,
        }
    }

    if let Some((idx, term)) = term_idx {
        elements.remove(idx);
        // Also remove trailing spaces after removal.
        while elements.last().map_or(false, |e| matches!(e, TrnElement::Space)) {
            elements.pop();
        }
        term
    } else {
        ChatTerminator::Inserted
    }
}
