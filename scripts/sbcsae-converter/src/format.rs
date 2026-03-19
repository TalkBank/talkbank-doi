use crate::diagnostics::Diagnostics;
use crate::types::{DiagnosticCode, FormatVariant, TrnLine};

/// Detect the format variant from the first non-empty line.
pub fn detect_format(first_line: &str) -> FormatVariant {
    let tab_fields: Vec<&str> = first_line.split('\t').collect();

    if tab_fields.len() >= 3 {
        // Check if first field contains two space-separated numbers (Format A).
        let first = tab_fields[0].trim();
        if first.contains(' ') {
            let parts: Vec<&str> = first.split_whitespace().collect();
            if parts.len() == 2 && looks_like_timestamp(parts[0]) && looks_like_timestamp(parts[1]) {
                return FormatVariant::A;
            }
        }
        // Check if first two fields are each a single number (Format B).
        if looks_like_timestamp(tab_fields[0].trim()) && looks_like_timestamp(tab_fields[1].trim()) {
            return FormatVariant::B;
        }
    }

    if tab_fields.len() == 2 {
        // Format C: "start end SPEAKER:\tcontent" — timestamps+speaker space-separated in field 0.
        let first = tab_fields[0].trim();
        if first.contains(' ') {
            let parts: Vec<&str> = first.split_whitespace().collect();
            if parts.len() >= 2 && looks_like_timestamp(parts[0]) && looks_like_timestamp(parts[1]) {
                return FormatVariant::C;
            }
        }
        // Could also be Format B continuation (start\tend\t\tcontent with empty speaker)
        if looks_like_timestamp(tab_fields[0].trim()) && looks_like_timestamp(tab_fields[1].trim()) {
            return FormatVariant::B;
        }
    }

    // Default fallback — try A.
    FormatVariant::A
}

fn looks_like_timestamp(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // Allow digits and at most one dot. Also tolerate trailing ':' (SBC027 anomaly).
    let s = s.trim_end_matches(':').trim();
    let mut dot_count = 0;
    for c in s.chars() {
        if c == '.' {
            dot_count += 1;
            if dot_count > 1 {
                return false;
            }
        } else if !c.is_ascii_digit() {
            return false;
        }
    }
    true
}

/// Parse all lines of a decoded TRN file.
pub fn parse_lines(text: &str, variant: FormatVariant, diag: &mut Diagnostics) -> Vec<TrnLine> {
    let mut lines = Vec::new();
    let mut current_speaker = String::from("???");

    for (idx, raw_line) in text.lines().enumerate() {
        let line_number = idx + 1;

        if raw_line.trim().is_empty() {
            continue;
        }

        match parse_single_line(raw_line, line_number, variant, &current_speaker, diag) {
            Some(trn_line) => {
                if let Some(ref spk) = trn_line.speaker {
                    current_speaker = spk.clone();
                }
                lines.push(trn_line);
            }
            None => {
                // Unparseable line — diagnostic already emitted.
            }
        }
    }

    lines
}

fn parse_single_line(
    raw_line: &str,
    line_number: usize,
    variant: FormatVariant,
    current_speaker: &str,
    diag: &mut Diagnostics,
) -> Option<TrnLine> {
    match variant {
        FormatVariant::A => parse_format_a(raw_line, line_number, current_speaker, diag),
        FormatVariant::B => parse_format_b(raw_line, line_number, current_speaker, diag),
        FormatVariant::C => parse_format_c(raw_line, line_number, current_speaker, diag),
    }
}

/// Format A: "start end\tSPEAKER: \tcontent" or "start end\t        \tcontent"
fn parse_format_a(
    raw_line: &str,
    line_number: usize,
    current_speaker: &str,
    diag: &mut Diagnostics,
) -> Option<TrnLine> {
    let tab_fields: Vec<&str> = raw_line.splitn(3, '\t').collect();
    if tab_fields.len() < 2 {
        // Try space-only fallback (SBC013 broken formatting).
        return parse_broken_format(raw_line, line_number, current_speaker, diag);
    }

    let (start_time, end_time) = parse_timestamp_pair(tab_fields[0].trim(), line_number, diag)?;
    let speaker_field = if tab_fields.len() >= 3 { tab_fields[1] } else { "" };
    let content = if tab_fields.len() >= 3 { tab_fields[2] } else { tab_fields.get(1).unwrap_or(&"") };

    let speaker = extract_speaker(speaker_field, line_number, diag);
    let effective_speaker = speaker.as_deref().unwrap_or(current_speaker).to_string();

    // Calculate content column: timestamps + tab + speaker + tab.
    let content_column = raw_line.len() - content.len();

    if content.trim().is_empty() {
        diag.warn(line_number, None, DiagnosticCode::EmptyContent, "Empty content field");
    }

    Some(TrnLine {
        line_number,
        start_time,
        end_time,
        speaker: speaker.map(|s| s.to_string()),
        effective_speaker,
        raw_content: content.to_string(),
        content_column,
    })
}

/// Format B: "start\tend\tSPEAKER:\tcontent" or "start\tend\t\tcontent"
fn parse_format_b(
    raw_line: &str,
    line_number: usize,
    current_speaker: &str,
    diag: &mut Diagnostics,
) -> Option<TrnLine> {
    let tab_fields: Vec<&str> = raw_line.splitn(4, '\t').collect();
    if tab_fields.len() < 3 {
        diag.error(
            line_number,
            None,
            DiagnosticCode::BrokenTabFormatting,
            format!("Expected at least 3 tab-separated fields, got {}", tab_fields.len()),
        );
        return None;
    }

    let start_str = tab_fields[0].trim().trim_end_matches(':');
    let end_str = tab_fields[1].trim().trim_end_matches(':');
    let (start_time, end_time) = parse_two_timestamps(start_str, end_str, line_number, diag)?;

    let speaker_field = tab_fields[2];
    let content = if tab_fields.len() >= 4 { tab_fields[3] } else { "" };

    let speaker = extract_speaker(speaker_field, line_number, diag);
    let effective_speaker = speaker.as_deref().unwrap_or(current_speaker).to_string();

    let content_column = raw_line.len() - content.len();

    if content.trim().is_empty() && !speaker_field.trim().is_empty() {
        diag.warn(line_number, None, DiagnosticCode::EmptyContent, "Empty content field");
    }

    Some(TrnLine {
        line_number,
        start_time,
        end_time,
        speaker: speaker.map(|s| s.to_string()),
        effective_speaker,
        raw_content: content.to_string(),
        content_column,
    })
}

/// Format C: "start end SPEAKER:  \tcontent" or "start end          \tcontent"
fn parse_format_c(
    raw_line: &str,
    line_number: usize,
    current_speaker: &str,
    diag: &mut Diagnostics,
) -> Option<TrnLine> {
    let tab_fields: Vec<&str> = raw_line.splitn(2, '\t').collect();
    if tab_fields.len() < 2 {
        diag.error(
            line_number,
            None,
            DiagnosticCode::BrokenTabFormatting,
            "Expected tab separator before content",
        );
        return None;
    }

    let prefix = tab_fields[0];
    let content = tab_fields[1];

    // Parse "start end SPEAKER:" or "start end         " from prefix.
    let parts: Vec<&str> = prefix.split_whitespace().collect();
    if parts.len() < 2 {
        diag.error(
            line_number,
            None,
            DiagnosticCode::BrokenTabFormatting,
            "Cannot parse timestamps from prefix",
        );
        return None;
    }

    let (start_time, end_time) = parse_two_timestamps(parts[0], parts[1], line_number, diag)?;

    let speaker = if parts.len() >= 3 {
        extract_speaker(parts[2], line_number, diag)
    } else {
        None
    };
    let effective_speaker = speaker.as_deref().unwrap_or(current_speaker).to_string();

    let content_column = raw_line.len() - content.len();

    Some(TrnLine {
        line_number,
        start_time,
        end_time,
        speaker: speaker.map(|s| s.to_string()),
        effective_speaker,
        raw_content: content.to_string(),
        content_column,
    })
}

/// Fallback for lines with broken tab formatting (SBC013).
fn parse_broken_format(
    raw_line: &str,
    line_number: usize,
    current_speaker: &str,
    diag: &mut Diagnostics,
) -> Option<TrnLine> {
    diag.warn(
        line_number,
        None,
        DiagnosticCode::BrokenTabFormatting,
        "No tab separators found, attempting space-based parsing",
    );

    let parts: Vec<&str> = raw_line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }

    let (start_time, end_time) = parse_two_timestamps(parts[0], parts[1], line_number, diag)?;

    // Check if parts[2] looks like a speaker.
    let (speaker, content_start_idx) = if looks_like_speaker_token(parts[2]) {
        (Some(normalize_speaker(parts[2])), 3)
    } else {
        (None, 2)
    };

    let effective_speaker = speaker.as_deref().unwrap_or(current_speaker).to_string();
    let content = parts[content_start_idx..].join(" ");

    // Rough content column estimate.
    let content_column = raw_line
        .find(parts.get(content_start_idx).unwrap_or(&""))
        .unwrap_or(0);

    Some(TrnLine {
        line_number,
        start_time,
        end_time,
        speaker: speaker.map(|s| s.to_string()),
        effective_speaker,
        raw_content: content,
        content_column,
    })
}

fn parse_timestamp_pair(field: &str, line_number: usize, diag: &mut Diagnostics) -> Option<(f64, f64)> {
    let parts: Vec<&str> = field.split_whitespace().collect();
    if parts.len() < 2 {
        diag.error(
            line_number,
            None,
            DiagnosticCode::TimestampAnomaly,
            format!("Cannot parse timestamp pair from '{field}'"),
        );
        return None;
    }
    parse_two_timestamps(parts[0], parts[1], line_number, diag)
}

fn parse_two_timestamps(start_str: &str, end_str: &str, line_number: usize, diag: &mut Diagnostics) -> Option<(f64, f64)> {
    let start_str = start_str.trim().trim_end_matches(':').trim();
    let end_str = end_str.trim().trim_end_matches(':').trim();

    // Check for zero-timestamp markers.
    if start_str.chars().all(|c| c == '0') && start_str.len() > 4 {
        diag.warn(line_number, None, DiagnosticCode::ZeroTimestamp, "Zero-timestamp line");
        return Some((0.0, 0.0));
    }

    let start = match start_str.parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            diag.error(
                line_number,
                None,
                DiagnosticCode::TimestampAnomaly,
                format!("Cannot parse start timestamp '{start_str}'"),
            );
            return None;
        }
    };

    let end = match end_str.parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            diag.error(
                line_number,
                None,
                DiagnosticCode::TimestampAnomaly,
                format!("Cannot parse end timestamp '{end_str}'"),
            );
            return None;
        }
    };

    if end < start && (start - end).abs() > 0.01 {
        diag.warn(
            line_number,
            None,
            DiagnosticCode::TimestampAnomaly,
            format!("End time ({end}) is before start time ({start})"),
        );
    }

    Some((start, end))
}

/// Extract speaker name from a speaker field, if present.
fn extract_speaker<'a>(field: &'a str, line_number: usize, diag: &mut Diagnostics) -> Option<&'a str> {
    let trimmed = field.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Check if it's all spaces (continuation line in Format A).
    if field.chars().all(|c| c == ' ') {
        return None;
    }

    // Normalize: strip trailing colon if present.
    let name = trimmed.trim_end_matches(':');
    if name.is_empty() {
        return None;
    }

    // Check for missing colon.
    if !trimmed.ends_with(':') && looks_like_speaker_token(trimmed) {
        diag.warn(
            line_number,
            None,
            DiagnosticCode::MissingSpeakerColon,
            format!("Speaker '{name}' missing trailing colon"),
        );
        return Some(name);
    }

    if looks_like_speaker_token(trimmed) {
        Some(name)
    } else {
        None
    }
}

/// Does this token look like a speaker label (with or without colon)?
fn looks_like_speaker_token(s: &str) -> bool {
    let s = s.trim().trim_end_matches(':');
    if s.is_empty() {
        return false;
    }
    // Allow: >PREFIX, #PREFIX, *PREFIX, or uppercase start.
    let s = s.trim_start_matches(|c| c == '>' || c == '#' || c == '*');
    if s.is_empty() {
        return false;
    }
    // Must start with uppercase letter.
    let first = s.chars().next().unwrap();
    if !first.is_ascii_uppercase() {
        // Check for case-inconsistent speakers (e.g., "Doris").
        if first.is_ascii_lowercase() && s.len() > 1 {
            return true; // Allow, will be caught elsewhere.
        }
        return false;
    }
    // Rest: uppercase, digits, underscore, ?, /.
    s.chars().skip(1).all(|c| {
        c.is_ascii_uppercase()
            || c.is_ascii_digit()
            || c == '_'
            || c == '?'
            || c == '/'
            || c.is_ascii_lowercase()
    })
}

/// Normalize speaker name: strip colon and > prefix for comparison.
fn normalize_speaker(s: &str) -> String {
    s.trim().trim_end_matches(':').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::Diagnostics;

    #[test]
    fn detect_format_a() {
        let line = "0.00 9.21\tLENORE: \t... So you don't need to go";
        assert!(matches!(detect_format(line), FormatVariant::A));
    }

    #[test]
    fn detect_format_b() {
        let line = "2.660\t2.805\tJOANNE:\tBut,";
        assert!(matches!(detect_format(line), FormatVariant::B));
    }

    #[test]
    fn detect_format_c() {
        let line = "0.00 2.53 FRED:   \t... Okay.";
        assert!(matches!(detect_format(line), FormatVariant::C));
    }

    #[test]
    fn parse_format_a_speaker_line() {
        let text = "0.00 6.52\tJAMIE:  \tHow [can you teach] tap dance.\n";
        let mut diag = Diagnostics::new();
        let lines = parse_lines(text, FormatVariant::A, &mut diag);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].speaker.as_deref(), Some("JAMIE"));
        assert_eq!(lines[0].start_time, 0.0);
        assert_eq!(lines[0].end_time, 6.52);
        assert!(lines[0].raw_content.starts_with("How"));
    }

    #[test]
    fn parse_format_a_continuation() {
        let text = "0.00 6.52\tJAMIE:  \tHow tap dance.\n6.52 8.00\t        \tReally.\n";
        let mut diag = Diagnostics::new();
        let lines = parse_lines(text, FormatVariant::A, &mut diag);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].speaker.as_deref(), Some("JAMIE"));
        assert_eq!(lines[1].speaker, None);
        assert_eq!(lines[1].effective_speaker, "JAMIE");
    }

    #[test]
    fn parse_format_b_basic() {
        let text = "2.660\t2.805\tJOANNE:\tBut,\n2.805\t4.685\t\tso these slides.\n";
        let mut diag = Diagnostics::new();
        let lines = parse_lines(text, FormatVariant::B, &mut diag);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].speaker.as_deref(), Some("JOANNE"));
        assert_eq!(lines[1].speaker, None);
        assert_eq!(lines[1].effective_speaker, "JOANNE");
    }

    #[test]
    fn timestamp_with_trailing_colon() {
        let mut diag = Diagnostics::new();
        // Real case from SBC027: "78.100 :" — the space+colon is extra junk.
        // After trim() → "78.100 :", trim_end_matches(':') → "78.100 ", trim() → "78.100".
        let result = parse_two_timestamps("77.540", "78.100", 1, &mut diag);
        assert!(result.is_some());
        // With trailing " :" — needs the extra trim in the chain.
        let result2 = parse_two_timestamps("77.540 :", "78.100 :", 1, &mut diag);
        assert!(result2.is_some());
    }

    #[test]
    fn zero_timestamp() {
        let mut diag = Diagnostics::new();
        let result = parse_two_timestamps("000000000", "000000000", 1, &mut diag);
        assert_eq!(result, Some((0.0, 0.0)));
        assert_eq!(diag.len(), 1);
    }
}
