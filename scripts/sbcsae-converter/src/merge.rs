//! Merge TRN overlap indices into existing hand-edited CHAT files.
//!
//! Strategy: sequential matching per speaker.
//! For each speaker, the Nth overlap marker in the CHAT corresponds to
//! the Nth bracket of the same direction for that speaker in the TRN.
//! This works because Brian didn't add or remove overlap markers — he only
//! stripped the indices and adjusted timing/content.

use std::collections::HashMap;
use std::path::Path;

use crate::intermediate::{BracketDirection, OverlapAssignment, OverlapRole, TrnDocument};

/// Result of merging indices into one CHAT file.
pub struct MergeResult {
    pub markers_found: usize,
    pub markers_indexed: usize,
    pub markers_already_indexed: usize,
    pub markers_unmatched: usize,
    pub updated_content: String,
}

/// Merge TRN overlap indices into an existing CHAT file.
pub fn merge_indices(
    chat_path: &Path,
    doc: &TrnDocument,
    assignment: &OverlapAssignment,
) -> std::io::Result<MergeResult> {
    let content = std::fs::read_to_string(chat_path)?;
    let mut result = MergeResult {
        markers_found: 0,
        markers_indexed: 0,
        markers_already_indexed: 0,
        markers_unmatched: 0,
        updated_content: String::with_capacity(content.len() + 1024),
    };

    // Build per-speaker queues of indices from the TRN assignment.
    // For each speaker, collect the sequence of (direction, index) in document order.
    let speaker_queues = build_speaker_queues(doc, assignment);

    // Track position per (chat_speaker, direction) as we scan the CHAT.
    let mut positions: HashMap<(String, MarkerDir), usize> = HashMap::new();

    for line in content.lines() {
        if !line.starts_with('*') {
            result.updated_content.push_str(line);
            result.updated_content.push('\n');
            continue;
        }

        let chat_speaker = extract_speaker(line);
        // Map CHAT speaker ID back to TRN speaker name.
        let trn_speaker = doc.speaker_map.iter()
            .find(|(_, chat_id)| chat_id.as_str() == chat_speaker)
            .map(|(trn_name, _)| trn_name.clone())
            .unwrap_or_default();

        let updated = process_line(
            line,
            &trn_speaker,
            &speaker_queues,
            &mut positions,
            &mut result,
        );
        result.updated_content.push_str(&updated);
        result.updated_content.push('\n');
    }

    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MarkerDir {
    TopOpen,
    TopClose,
    BottomOpen,
    BottomClose,
}

fn marker_to_dir(c: char) -> Option<MarkerDir> {
    match c {
        '⌈' => Some(MarkerDir::TopOpen),
        '⌉' => Some(MarkerDir::TopClose),
        '⌊' => Some(MarkerDir::BottomOpen),
        '⌋' => Some(MarkerDir::BottomClose),
        _ => None,
    }
}

/// Build per-(speaker, direction) queues of display indices from the TRN.
fn build_speaker_queues(
    doc: &TrnDocument,
    assignment: &OverlapAssignment,
) -> HashMap<(String, MarkerDir), Vec<u8>> {
    let mut queues: HashMap<(String, MarkerDir), Vec<u8>> = HashMap::new();

    // Iterate brackets in document order.
    for bracket in &doc.brackets {
        if let Some(role) = assignment.roles.get(&bracket.id) {
            let dir = match role.role {
                OverlapRole::TopBegin => MarkerDir::TopOpen,
                OverlapRole::TopEnd => MarkerDir::TopClose,
                OverlapRole::BottomBegin => MarkerDir::BottomOpen,
                OverlapRole::BottomEnd => MarkerDir::BottomClose,
            };
            let display_index = (role.real_index % 9) as u8 + 1;
            queues
                .entry((bracket.speaker.clone(), dir))
                .or_default()
                .push(display_index);
        }
    }

    queues
}

fn process_line(
    line: &str,
    trn_speaker: &str,
    queues: &HashMap<(String, MarkerDir), Vec<u8>>,
    positions: &mut HashMap<(String, MarkerDir), usize>,
    result: &mut MergeResult,
) -> String {
    let mut output = String::with_capacity(line.len() + 32);
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if let Some(dir) = marker_to_dir(chars[i]) {
            result.markers_found += 1;

            // Check if already indexed.
            if i + 1 < chars.len() && chars[i + 1] >= '2' && chars[i + 1] <= '9' {
                result.markers_already_indexed += 1;
                output.push(chars[i]);
                output.push(chars[i + 1]);
                i += 2;
                continue;
            }

            // Look up the next index for this speaker + direction.
            let key = (trn_speaker.to_string(), dir);
            let pos = positions.entry(key.clone()).or_insert(0);
            let queue = queues.get(&key);

            if let Some(indices) = queue {
                if *pos < indices.len() {
                    let idx = indices[*pos];
                    *pos += 1;
                    output.push(chars[i]);
                    if idx >= 2 {
                        output.push(char::from(b'0' + idx));
                    }
                    result.markers_indexed += 1;
                } else {
                    // Ran out of TRN brackets for this speaker+direction.
                    output.push(chars[i]);
                    result.markers_unmatched += 1;
                }
            } else {
                // No TRN brackets for this speaker+direction at all.
                output.push(chars[i]);
                result.markers_unmatched += 1;
            }

            i += 1;
            continue;
        }

        output.push(chars[i]);
        i += 1;
    }

    output
}

fn extract_speaker(line: &str) -> String {
    if let Some(rest) = line.strip_prefix('*') {
        if let Some(colon_pos) = rest.find(':') {
            return rest[..colon_pos].to_string();
        }
    }
    String::new()
}
