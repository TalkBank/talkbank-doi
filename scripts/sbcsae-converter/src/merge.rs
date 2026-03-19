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

    // Build per-speaker entry lists with timing from the TRN.
    let mut speaker_entries = build_speaker_entries(doc, assignment);

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
            &mut speaker_entries,
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

/// An index entry with timing for matching.
struct IndexEntry {
    display_index: u8,
    time_ms: i64, // start time of the bracket's line in ms
    used: bool,
}

/// Build per-(speaker, direction) lists of indices with timing from the TRN.
fn build_speaker_entries(
    doc: &TrnDocument,
    assignment: &OverlapAssignment,
) -> HashMap<(String, MarkerDir), Vec<IndexEntry>> {
    let mut entries: HashMap<(String, MarkerDir), Vec<IndexEntry>> = HashMap::new();

    for bracket in &doc.brackets {
        if let Some(role) = assignment.roles.get(&bracket.id) {
            let dir = match role.role {
                OverlapRole::TopBegin => MarkerDir::TopOpen,
                OverlapRole::TopEnd => MarkerDir::TopClose,
                OverlapRole::BottomBegin => MarkerDir::BottomOpen,
                OverlapRole::BottomEnd => MarkerDir::BottomClose,
            };
            let display_index = (role.real_index % 9) as u8 + 1;
            let time_ms = (bracket.source.time_range.0 * 1000.0) as i64;
            entries
                .entry((bracket.speaker.clone(), dir))
                .or_default()
                .push(IndexEntry { display_index, time_ms, used: false });
        }
    }

    entries
}

fn process_line(
    line: &str,
    trn_speaker: &str,
    entries: &mut HashMap<(String, MarkerDir), Vec<IndexEntry>>,
    result: &mut MergeResult,
) -> String {
    let mut output = String::with_capacity(line.len() + 32);
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    // Extract timing for this line to guide matching.
    let line_time = extract_timing(line).map(|(s, _)| s);

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

            // Find the best matching TRN bracket: closest unused entry by timing.
            let key = (trn_speaker.to_string(), dir);
            let matched_idx = if let Some(entry_list) = entries.get_mut(&key) {
                if let Some(chat_time) = line_time {
                    // Find closest unused entry by timing.
                    let best: Option<(usize, &IndexEntry)> = entry_list.iter()
                        .enumerate()
                        .filter(|(_, e)| !e.used)
                        .min_by_key(|(_, e)| (e.time_ms - chat_time).abs());
                    if let Some((idx, _)) = best {
                        entry_list[idx].used = true;
                        Some(entry_list[idx].display_index)
                    } else {
                        None
                    }
                } else {
                    // No timing — use first unused entry (sequential fallback).
                    let first_unused = entry_list.iter_mut().find(|e| !e.used);
                    if let Some(entry) = first_unused {
                        entry.used = true;
                        Some(entry.display_index)
                    } else {
                        None
                    }
                }
            } else {
                None
            };

            if let Some(idx) = matched_idx {
                output.push(chars[i]);
                if idx >= 2 {
                    output.push(char::from(b'0' + idx));
                }
                result.markers_indexed += 1;
            } else {
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

fn extract_timing(line: &str) -> Option<(i64, i64)> {
    if let Some(start_pos) = line.find('\x15') {
        let rest = &line[start_pos + 1..];
        if let Some(end_pos) = rest.find('\x15') {
            let bullet = &rest[..end_pos];
            if let Some(underscore) = bullet.find('_') {
                let start: i64 = bullet[..underscore].parse().ok()?;
                let end: i64 = bullet[underscore + 1..].parse().ok()?;
                return Some((start, end));
            }
        }
    }
    None
}

fn extract_speaker(line: &str) -> String {
    if let Some(rest) = line.strip_prefix('*') {
        if let Some(colon_pos) = rest.find(':') {
            return rest[..colon_pos].to_string();
        }
    }
    String::new()
}
