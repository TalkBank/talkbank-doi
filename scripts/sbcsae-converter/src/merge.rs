//! Merge TRN overlap indices into existing hand-edited CHAT files.
//!
//! Strategy: content-based matching with DP alignment.
//! For each speaker, extract the sequence of overlap bracket texts from both
//! CHAT and TRN, then align them using dynamic programming (edit distance)
//! to find the best correspondence.

use std::collections::HashMap;
use std::path::Path;

use crate::intermediate::{OverlapAssignment, OverlapRole, TrnDocument};

/// Result of merging indices into one CHAT file.
pub struct MergeResult {
    pub markers_found: usize,
    pub markers_indexed: usize,
    pub markers_already_indexed: usize,
    pub markers_unmatched: usize,
    pub updated_content: String,
}

/// A CHAT overlap marker with its context.
struct ChatMarker {
    line_idx: usize,
    char_idx: usize,
    marker_char: char,
    speaker: String,
    /// Text between this open and its close (if on same line).
    bracketed_text: String,
    /// Has an existing index digit.
    existing_index: Option<u8>,
}

/// A TRN bracket with its assigned index.
struct TrnEntry {
    speaker: String,
    role: OverlapRole,
    display_index: u8,
    context_text: String,
}

pub fn merge_indices(
    chat_path: &Path,
    doc: &TrnDocument,
    assignment: &OverlapAssignment,
) -> std::io::Result<MergeResult> {
    let content = std::fs::read_to_string(chat_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Step 1: Extract all CHAT markers with their bracketed text.
    let chat_markers = extract_chat_markers(&lines);

    // Step 2: Build TRN entries with context text.
    let trn_entries = build_trn_entries(doc, assignment);

    // Step 3: For each (speaker, direction), align CHAT markers with TRN entries
    // using DP-based sequence alignment.
    let index_map = align_markers(&chat_markers, &trn_entries);

    // Step 4: Apply indices to the CHAT content.
    let mut result = MergeResult {
        markers_found: chat_markers.len(),
        markers_indexed: 0,
        markers_already_indexed: 0,
        markers_unmatched: 0,
        updated_content: String::with_capacity(content.len() + 2048),
    };

    for (line_idx, line) in lines.iter().enumerate() {
        let mut output = String::with_capacity(line.len() + 32);
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if is_overlap_marker(chars[i]) {
                // Check for existing index.
                if i + 1 < chars.len() && chars[i + 1] >= '2' && chars[i + 1] <= '9' {
                    result.markers_already_indexed += 1;
                    output.push(chars[i]);
                    output.push(chars[i + 1]);
                    i += 2;
                    continue;
                }

                // Look up the assigned index.
                if let Some(&idx) = index_map.get(&(line_idx, i)) {
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
            } else {
                output.push(chars[i]);
                i += 1;
            }
        }

        result.updated_content.push_str(&output);
        result.updated_content.push('\n');
    }

    Ok(result)
}

fn is_overlap_marker(c: char) -> bool {
    matches!(c, '⌈' | '⌉' | '⌊' | '⌋')
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

fn role_to_dir(role: OverlapRole) -> MarkerDir {
    match role {
        OverlapRole::TopBegin => MarkerDir::TopOpen,
        OverlapRole::TopEnd => MarkerDir::TopClose,
        OverlapRole::BottomBegin => MarkerDir::BottomOpen,
        OverlapRole::BottomEnd => MarkerDir::BottomClose,
    }
}

/// Extract all overlap markers from CHAT lines with their context.
fn extract_chat_markers(lines: &[&str]) -> Vec<ChatMarker> {
    let mut markers = Vec::new();
    let mut current_speaker = String::new();

    for (line_idx, line) in lines.iter().enumerate() {
        if line.starts_with('*') {
            if let Some(colon) = line.find(':') {
                current_speaker = line[1..colon].to_string();
            }
        }
        if !line.starts_with('*') {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if is_overlap_marker(chars[i]) {
                let marker_char = chars[i];
                let existing_index = if i + 1 < chars.len() && chars[i + 1] >= '2' && chars[i + 1] <= '9' {
                    Some((chars[i + 1] as u8) - b'0')
                } else {
                    None
                };

                // Extract bracketed text: text from this marker to its matching close.
                let bracketed = extract_bracketed_text_from_chat(&chars, i);

                markers.push(ChatMarker {
                    line_idx,
                    char_idx: i,
                    marker_char,
                    speaker: current_speaker.clone(),
                    bracketed_text: bracketed,
                    existing_index,
                });

                i += 1;
                if existing_index.is_some() {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
    }

    markers
}

/// Extract the text between an overlap open and its close on the same line.
fn extract_bracketed_text_from_chat(chars: &[char], start: usize) -> String {
    let open_char = chars[start];
    let close_char = match open_char {
        '⌈' => '⌉',
        '⌊' => '⌋',
        '⌉' | '⌋' => return String::new(), // Close markers have no bracketed text.
        _ => return String::new(),
    };

    let mut i = start + 1;
    // Skip index digit.
    if i < chars.len() && chars[i] >= '2' && chars[i] <= '9' {
        i += 1;
    }

    let text_start = i;
    while i < chars.len() {
        if chars[i] == close_char {
            let text: String = chars[text_start..i].iter().collect();
            return normalize_for_matching(&text);
        }
        i += 1;
    }

    // No close found — extract rest of content (cross-utterance span).
    let text: String = chars[text_start..].iter().collect();
    // Remove timing bullet and terminator.
    let text = text.trim_end();
    normalize_for_matching(text)
}

/// Build TRN entries with context text for matching.
fn build_trn_entries(doc: &TrnDocument, assignment: &OverlapAssignment) -> Vec<TrnEntry> {
    let mut entries = Vec::new();

    for bracket in &doc.brackets {
        if let Some(role) = assignment.roles.get(&bracket.id) {
            // Get context: the utterance content near this bracket.
            let context = if let Some(utt) = doc.utterances.get(bracket.utterance_index) {
                // Extract words near this bracket in the utterance.
                let mut words = Vec::new();
                let mut in_bracket = false;
                for elem in &utt.elements {
                    match elem {
                        crate::intermediate::ContentElement::Bracket(id) if *id == bracket.id => {
                            in_bracket = true;
                        }
                        crate::intermediate::ContentElement::Bracket(_) => {
                            if in_bracket {
                                break; // Found the close bracket.
                            }
                        }
                        crate::intermediate::ContentElement::Word(w) if in_bracket => {
                            words.push(w.clone());
                        }
                        _ => {}
                    }
                }
                normalize_for_matching(&words.join(" "))
            } else {
                String::new()
            };

            entries.push(TrnEntry {
                speaker: bracket.speaker.clone(),
                role: role.role,
                display_index: (role.real_index % 9) as u8 + 1,
                context_text: context,
            });
        }
    }

    entries
}

/// Normalize text for fuzzy matching: lowercase, strip markers, collapse whitespace.
fn normalize_for_matching(text: &str) -> String {
    let mut s = text.to_lowercase();
    // Remove CHAT-specific markers.
    s = s.replace("&=in", "").replace("&=ex", "").replace("&=tsk", "");
    // Remove overlap markers.
    s = s.replace('⌈', "").replace('⌉', "").replace('⌊', "").replace('⌋', "");
    // Remove digits that are overlap indices.
    // Remove punctuation except apostrophes.
    s = s.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '\'').collect();
    // Collapse whitespace.
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Align CHAT markers with TRN entries using DP per (speaker, direction).
/// Returns a map of (line_idx, char_idx) → display_index.
fn align_markers(
    chat_markers: &[ChatMarker],
    trn_entries: &[TrnEntry],
) -> HashMap<(usize, usize), u8> {
    let mut result = HashMap::new();

    // Map CHAT speaker IDs to TRN speaker names isn't needed here —
    // both sides use the CHAT speaker ID format.

    // Group by (speaker, direction).
    let mut chat_by_key: HashMap<(String, MarkerDir), Vec<usize>> = HashMap::new();
    for (i, m) in chat_markers.iter().enumerate() {
        if m.existing_index.is_some() {
            continue; // Already indexed — skip.
        }
        if let Some(dir) = marker_to_dir(m.marker_char) {
            chat_by_key.entry((m.speaker.clone(), dir)).or_default().push(i);
        }
    }

    let mut trn_by_key: HashMap<(String, MarkerDir), Vec<usize>> = HashMap::new();
    for (i, e) in trn_entries.iter().enumerate() {
        // Map TRN speaker to CHAT ID.
        let dir = role_to_dir(e.role);
        trn_by_key.entry((e.speaker.clone(), dir)).or_default().push(i);
    }

    // For each (speaker, direction), align the sequences.
    for (key, chat_indices) in &chat_by_key {
        // Find the TRN speaker name that matches this CHAT speaker ID.
        let trn_key_candidates: Vec<_> = trn_by_key.keys()
            .filter(|(_, d)| *d == key.1)
            .filter(|(s, _)| {
                // Match: TRN speaker name truncated to 4 chars == CHAT ID.
                let truncated = if s.len() >= 4 { &s[..4] } else { s.as_str() };
                truncated == key.0
            })
            .cloned()
            .collect();

        let trn_key = if let Some(k) = trn_key_candidates.first() {
            k.clone()
        } else {
            continue;
        };

        let trn_indices = match trn_by_key.get(&trn_key) {
            Some(v) => v,
            None => continue,
        };

        // Extract text sequences for DP alignment.
        let chat_texts: Vec<&str> = chat_indices.iter()
            .map(|&i| chat_markers[i].bracketed_text.as_str())
            .collect();
        let trn_texts: Vec<&str> = trn_indices.iter()
            .map(|&i| trn_entries[i].context_text.as_str())
            .collect();

        // DP alignment: find the best matching between chat_texts and trn_texts.
        let alignment = dp_align(&chat_texts, &trn_texts);

        // Apply the alignment.
        for (chat_pos, trn_pos) in alignment {
            let chat_marker = &chat_markers[chat_indices[chat_pos]];
            let trn_entry = &trn_entries[trn_indices[trn_pos]];
            result.insert(
                (chat_marker.line_idx, chat_marker.char_idx),
                trn_entry.display_index,
            );
        }
    }

    result
}

/// transpositions. Returns pairs of (chat_idx, trn_idx).
fn dp_align(chat: &[&str], trn: &[&str]) -> Vec<(usize, usize)> {
    let m = chat.len();
    let n = trn.len();

    if m == 0 || n == 0 {
        return Vec::new();
    }

    // Cost function: similarity between two strings.
    // Higher = better match. 0 = no match.
    let similarity = |a: &str, b: &str| -> i32 {
        if a.is_empty() && b.is_empty() {
            return 10;
        }
        if a.is_empty() || b.is_empty() {
            return 0;
        }
        if a == b {
            return 100;
        }
        // Substring containment.
        if a.contains(b) || b.contains(a) {
            return 80;
        }
        // Word overlap.
        let a_words: std::collections::HashSet<&str> = a.split_whitespace().collect();
        let b_words: std::collections::HashSet<&str> = b.split_whitespace().collect();
        let common = a_words.intersection(&b_words).count() as i32;
        let total = a_words.len().max(b_words.len()) as i32;
        if total > 0 && common > 0 {
            (common * 60) / total
        } else {
            0
        }
    };

    // DP table: dp[i][j] = best score for aligning chat[0..i] with trn[0..j].
    let mut dp = vec![vec![0i32; n + 1]; m + 1];
    let mut from = vec![vec![(0usize, 0usize); n + 1]; m + 1];

    // Gap penalty.
    let gap = -5;

    for i in 1..=m {
        dp[i][0] = dp[i - 1][0] + gap;
        from[i][0] = (i - 1, 0);
    }
    for j in 1..=n {
        dp[0][j] = dp[0][j - 1] + gap;
        from[0][j] = (0, j - 1);
    }

    for i in 1..=m {
        for j in 1..=n {
            let match_score = dp[i - 1][j - 1] + similarity(chat[i - 1], trn[j - 1]);
            let chat_gap = dp[i - 1][j] + gap;
            let trn_gap = dp[i][j - 1] + gap;

            if match_score >= chat_gap && match_score >= trn_gap {
                dp[i][j] = match_score;
                from[i][j] = (i - 1, j - 1);
            } else if chat_gap >= trn_gap {
                dp[i][j] = chat_gap;
                from[i][j] = (i - 1, j);
            } else {
                dp[i][j] = trn_gap;
                from[i][j] = (i, j - 1);
            }
        }
    }

    // Traceback.
    let mut pairs = Vec::new();
    let mut i = m;
    let mut j = n;
    while i > 0 && j > 0 {
        let (pi, pj) = from[i][j];
        if pi == i - 1 && pj == j - 1 {
            // Match.
            let sim = similarity(chat[i - 1], trn[j - 1]);
            if sim > 0 {
                pairs.push((i - 1, j - 1));
            }
        }
        i = pi;
        j = pj;
    }

    pairs.reverse();
    pairs
}
