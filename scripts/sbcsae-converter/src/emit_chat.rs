//! Emit CHAT text from a TrnDocument + OverlapAssignment.

use std::fmt::Write;

use crate::intermediate::{
    ContentElement, OverlapAssignment,
    OverlapRole as DocOverlapRole,
    TrnDocument, TrnUtterance, Terminator,
};

const MAX_OVERLAPS_EMIT: usize = 9;

/// Emit a CHAT file from a TrnDocument + OverlapAssignment.
pub fn emit_chat_from_doc(
    file_stem: &str,
    doc: &TrnDocument,
    assignment: &OverlapAssignment,
) -> String {
    let mut out = String::new();

    // Headers.
    writeln!(out, "@UTF8").unwrap();
    writeln!(out, "@Begin").unwrap();
    writeln!(out, "@Languages:\teng").unwrap();

    let parts: Vec<String> = doc.speaker_map
        .iter()
        .map(|(trn_name, chat_id)| {
            let role = if trn_name.starts_with('>') { "Environment" } else { "Speaker" };
            format!("{chat_id} {role}")
        })
        .collect();
    writeln!(out, "@Participants:\t{}", parts.join(", ")).unwrap();
    writeln!(out, "@Options:\tCA").unwrap();

    for (trn_name, chat_id) in &doc.speaker_map {
        let role = if trn_name.starts_with('>') { "Environment" } else { "Speaker" };
        writeln!(out, "@ID:\teng|SBCSAE|{chat_id}|||||{role}|||").unwrap();
    }

    writeln!(out, "@Media:\t{file_stem}, audio").unwrap();

    // Sort utterances by start time.
    let mut sorted: Vec<&TrnUtterance> = doc.utterances.iter().collect();
    sorted.sort_by_key(|u| u.start_ms.unwrap_or(0));

    for utt in &sorted {
        let chat_id = doc.speaker_map
            .get(&utt.speaker)
            .map(|s| s.as_str())
            .unwrap_or(&utt.speaker);

        let prefix = if utt.speaker.starts_with('>') {
            "%com:\t".to_string()
        } else {
            format!("*{chat_id}:\t")
        };

        let content = emit_content(&utt.elements, assignment);
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

/// Convert TRN timestamp (seconds as f64) to milliseconds.
pub fn time_to_ms(t: f64) -> i64 {
    (t * 1000.0).round() as i64
}

fn emit_content(
    elements: &[ContentElement],
    assignment: &OverlapAssignment,
) -> String {
    let mut out = String::new();
    let mut need_space = false;

    for elem in elements {
        let (text, space_before, space_after) = emit_element(elem, assignment);
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

fn emit_element(
    elem: &ContentElement,
    assignment: &OverlapAssignment,
) -> (String, bool, bool) {
    match elem {
        ContentElement::Word(w) => {
            if let Some(rest) = w.strip_prefix('@') {
                (format!("{rest} [% laugh]"), true, true)
            } else {
                (w.clone(), true, true)
            }
        }
        ContentElement::Bracket(bracket_id) => {
            if let Some(role) = assignment.roles.get(bracket_id) {
                let marker = match role.role {
                    DocOverlapRole::TopBegin => "⌈",
                    DocOverlapRole::TopEnd => "⌉",
                    DocOverlapRole::BottomBegin => "⌊",
                    DocOverlapRole::BottomEnd => "⌋",
                };
                let idx = role.real_index % MAX_OVERLAPS_EMIT;
                let index_str = if idx == 0 { String::new() } else { format!("{}", idx + 1) };
                (format!("{marker}{index_str}"), true, true)
            } else {
                ("⌈?".to_string(), true, true)
            }
        }
        ContentElement::PauseShort => ("(..)".to_string(), true, true),
        ContentElement::PauseMedium => ("(...)".to_string(), true, true),
        ContentElement::PauseTimed(val) => (format!("({val})"), true, true),
        ContentElement::Inhalation => ("&=in".to_string(), true, true),
        ContentElement::InhalationLengthened => ("&=in &=lengthened".to_string(), true, true),
        ContentElement::Exhalation => ("&=ex".to_string(), true, true),
        ContentElement::Click => ("&=tsk".to_string(), true, true),
        ContentElement::Vocalism(name) => {
            if let Some(rest) = name.strip_prefix('%') {
                let vocalism = match rest {
                    "Hx" | "HX" => "&=ex".to_string(),
                    _ => format!("&={}", rest.to_lowercase()),
                };
                (format!("ʔuh {vocalism}"), true, true)
            } else {
                let sanitized = name.replace(',', "_AND").replace('.', "_POINT_").to_lowercase();
                (format!("&={sanitized}"), true, true)
            }
        }
        ContentElement::Laughs(n) => {
            let text = (0..*n).map(|_| "&=laugh").collect::<Vec<_>>().join(" ");
            (text, true, true)
        }
        ContentElement::LongFeatureBegin(label) => (format!("&{{l={label}"), true, true),
        ContentElement::LongFeatureEnd(label) => (format!("&}}l={label}"), true, true),
        ContentElement::NonvocalBegin(label) => (format!("&{{n={label}"), true, true),
        ContentElement::NonvocalEnd(label) => (format!("&}}n={label}"), true, true),
        ContentElement::NonvocalSimple(label) => (format!("&={{n={label}}}"), true, true),
        ContentElement::NonvocalBeat => ("&=nonvocal".to_string(), true, true),
        ContentElement::PhonologicalFragment(text) => (format!("/{text}/"), true, true),
        ContentElement::Glottal => ("ʔuh".to_string(), true, true),
        ContentElement::Comma => (",".to_string(), false, true),
    }
}

fn emit_terminator(term: &Terminator) -> &'static str {
    match term {
        Terminator::Period => ".",
        Terminator::Question => "?",
        Terminator::Interruption => "+/.",
        Terminator::TrailOff => "+...",
        Terminator::SelfCompletion => "+,",
        Terminator::Implicit => ".",
    }
}

fn emit_bullet(start_ms: Option<i64>, end_ms: Option<i64>) -> Option<String> {
    match (start_ms, end_ms) {
        (Some(s), Some(e)) => Some(format!("\x15{s}_{e}\x15")),
        _ => None,
    }
}
