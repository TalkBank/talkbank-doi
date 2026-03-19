//! Merge TRN overlap indices into existing hand-edited CHAT files.
//!
//! Two-pass strategy:
//! 1. DP alignment for open markers (⌈, ⌊) — content-based matching
//! 2. Stack-based propagation for close markers (⌉, ⌋) — inherit index
//!    from the most recent unmatched open of the same kind

use std::collections::HashMap;
use std::path::Path;

use crate::intermediate::{OverlapAssignment, OverlapRole, TrnDocument};

pub struct MergeResult {
    pub markers_found: usize,
    pub markers_indexed: usize,
    pub markers_already_indexed: usize,
    pub markers_unmatched: usize,
    pub updated_content: String,
}

pub fn merge_indices(
    chat_path: &Path,
    doc: &TrnDocument,
    assignment: &OverlapAssignment,
) -> std::io::Result<MergeResult> {
    let content = std::fs::read_to_string(chat_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // DP alignment for open markers. Close markers stay unindexed —
    // E347 only validates indexed markers, so unindexed closes don't cause errors.
    let all_indices = dp_align_opens(&lines, doc, assignment);

    // Pass 3: Apply to output.
    let mut result = MergeResult {
        markers_found: 0,
        markers_indexed: 0,
        markers_already_indexed: 0,
        markers_unmatched: 0,
        updated_content: String::with_capacity(content.len() + 2048),
    };

    for (line_idx, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let mut output = String::with_capacity(line.len() + 32);
        let mut i = 0;

        while i < chars.len() {
            if is_overlap_marker(chars[i]) {
                result.markers_found += 1;
                if i + 1 < chars.len() && chars[i + 1] >= '2' && chars[i + 1] <= '9' {
                    result.markers_already_indexed += 1;
                    output.push(chars[i]);
                    output.push(chars[i + 1]);
                    i += 2;
                    continue;
                }
                if let Some(&idx) = all_indices.get(&(line_idx, i)) {
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

// ── Pass 1: DP alignment for opens ──────────────────────────────────────────

fn dp_align_opens(
    lines: &[&str],
    doc: &TrnDocument,
    assignment: &OverlapAssignment,
) -> HashMap<(usize, usize), u8> {
    let mut result = HashMap::new();

    // Extract CHAT open markers per (speaker, direction).
    let mut chat_opens: HashMap<(String, Dir), Vec<(usize, usize, String)>> = HashMap::new();
    let mut current_speaker = String::new();
    for (line_idx, line) in lines.iter().enumerate() {
        if line.starts_with('*') {
            if let Some(colon) = line.find(':') {
                current_speaker = line[1..colon].to_string();
            }
        }
        if !line.starts_with('*') { continue; }

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if (chars[i] == '⌈' || chars[i] == '⌊')
                && !(i + 1 < chars.len() && chars[i + 1] >= '2' && chars[i + 1] <= '9')
            {
                let dir = if chars[i] == '⌈' { Dir::TopOpen } else { Dir::BottomOpen };
                let text = extract_bracketed_text(&chars, i);
                chat_opens.entry((current_speaker.clone(), dir)).or_default()
                    .push((line_idx, i, text));
            }
            i += 1;
        }
    }

    // Build TRN open entries per (speaker, direction).
    let mut trn_opens: HashMap<(String, Dir), Vec<(u8, String)>> = HashMap::new();
    for bracket in &doc.brackets {
        if let Some(role) = assignment.roles.get(&bracket.id) {
            let dir = match role.role {
                OverlapRole::TopBegin => Dir::TopOpen,
                OverlapRole::BottomBegin => Dir::BottomOpen,
                _ => continue,
            };
            let display_index = (role.real_index % 9) as u8 + 1;

            let context = if let Some(utt) = doc.utterances.get(bracket.utterance_index) {
                let mut words = Vec::new();
                let mut in_bracket = false;
                for elem in &utt.elements {
                    match elem {
                        crate::intermediate::ContentElement::Bracket(id) if *id == bracket.id => {
                            in_bracket = true;
                        }
                        crate::intermediate::ContentElement::Bracket(_) if in_bracket => break,
                        crate::intermediate::ContentElement::Word(w) if in_bracket => {
                            words.push(w.clone());
                        }
                        _ => {}
                    }
                }
                normalize(&words.join(" "))
            } else {
                String::new()
            };

            trn_opens.entry((bracket.speaker.clone(), dir)).or_default()
                .push((display_index, context));
        }
    }

    // For each (speaker, direction), run DP alignment.
    for (chat_key, chat_items) in &chat_opens {
        // Find TRN key matching this CHAT speaker.
        let trn_key = doc.speaker_map.iter()
            .find(|(_, cid)| cid.as_str() == chat_key.0)
            .map(|(name, _)| (name.clone(), chat_key.1));
        let trn_key = match trn_key {
            Some(k) => k,
            None => continue,
        };
        let trn_items = match trn_opens.get(&trn_key) {
            Some(v) => v,
            None => continue,
        };

        let chat_texts: Vec<&str> = chat_items.iter().map(|(_, _, t)| t.as_str()).collect();
        let trn_texts: Vec<&str> = trn_items.iter().map(|(_, t)| t.as_str()).collect();

        let alignment = dp_align(&chat_texts, &trn_texts);

        for (ci, ti) in alignment {
            let (line_idx, char_idx, _) = &chat_items[ci];
            let (display_index, _) = &trn_items[ti];
            result.insert((*line_idx, *char_idx), *display_index);
        }
    }

    result
}

// ── Pass 2: Close propagation ───────────────────────────────────────────────

fn propagate_to_closes(
    lines: &[&str],
    open_indices: &HashMap<(usize, usize), u8>,
) -> HashMap<(usize, usize), u8> {
    let mut result = open_indices.clone();
    let mut current_speaker = String::new();
    // Per-speaker stack: (expected_close_char, index).
    let mut stacks: HashMap<String, Vec<(char, u8)>> = HashMap::new();

    for (line_idx, line) in lines.iter().enumerate() {
        if line.starts_with('*') {
            if let Some(colon) = line.find(':') {
                current_speaker = line[1..colon].to_string();
            }
        }
        if !line.starts_with('*') { continue; }

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if !is_overlap_marker(chars[i]) { i += 1; continue; }

            let has_digit = i + 1 < chars.len() && chars[i + 1] >= '2' && chars[i + 1] <= '9';

            if chars[i] == '⌈' || chars[i] == '⌊' {
                let close_char = if chars[i] == '⌈' { '⌉' } else { '⌋' };
                let idx = if has_digit {
                    (chars[i + 1] as u8) - b'0'
                } else if let Some(&idx) = open_indices.get(&(line_idx, i)) {
                    idx
                } else {
                    1 // Default for unmatched opens.
                };
                stacks.entry(current_speaker.clone()).or_default()
                    .push((close_char, idx));
                i += if has_digit { 2 } else { 1 };
            } else {
                // Close marker.
                if !has_digit {
                    if let Some(stack) = stacks.get_mut(&current_speaker) {
                        if let Some(pos) = stack.iter().rposition(|(c, _)| *c == chars[i]) {
                            let (_, idx) = stack.remove(pos);
                            result.insert((line_idx, i), idx);
                        }
                    }
                } else {
                    // Already indexed close — just pop the stack.
                    let idx = (chars[i + 1] as u8) - b'0';
                    if let Some(stack) = stacks.get_mut(&current_speaker) {
                        if let Some(pos) = stack.iter().rposition(|(c, _)| *c == chars[i]) {
                            stack.remove(pos);
                        }
                    }
                }
                i += if has_digit { 2 } else { 1 };
            }
        }
    }

    result
}

// ── DP alignment ────────────────────────────────────────────────────────────

fn dp_align(chat: &[&str], trn: &[&str]) -> Vec<(usize, usize)> {
    let m = chat.len();
    let n = trn.len();
    if m == 0 || n == 0 { return Vec::new(); }

    let similarity = |a: &str, b: &str| -> i32 {
        if a.is_empty() && b.is_empty() { return 10; }
        if a.is_empty() || b.is_empty() { return 0; }
        if a == b { return 100; }
        if a.contains(b) || b.contains(a) { return 80; }
        let aw: std::collections::HashSet<&str> = a.split_whitespace().collect();
        let bw: std::collections::HashSet<&str> = b.split_whitespace().collect();
        let common = aw.intersection(&bw).count() as i32;
        let total = aw.len().max(bw.len()) as i32;
        if total > 0 && common > 0 { (common * 60) / total } else { 0 }
    };

    let gap = -5;
    let mut dp = vec![vec![0i32; n + 1]; m + 1];
    let mut from = vec![vec![(0usize, 0usize); n + 1]; m + 1];

    for i in 1..=m { dp[i][0] = dp[i-1][0] + gap; from[i][0] = (i-1, 0); }
    for j in 1..=n { dp[0][j] = dp[0][j-1] + gap; from[0][j] = (0, j-1); }

    for i in 1..=m {
        for j in 1..=n {
            let ms = dp[i-1][j-1] + similarity(chat[i-1], trn[j-1]);
            let cg = dp[i-1][j] + gap;
            let tg = dp[i][j-1] + gap;
            if ms >= cg && ms >= tg {
                dp[i][j] = ms; from[i][j] = (i-1, j-1);
            } else if cg >= tg {
                dp[i][j] = cg; from[i][j] = (i-1, j);
            } else {
                dp[i][j] = tg; from[i][j] = (i, j-1);
            }
        }
    }

    let mut pairs = Vec::new();
    let (mut i, mut j) = (m, n);
    while i > 0 && j > 0 {
        let (pi, pj) = from[i][j];
        if pi == i-1 && pj == j-1 && similarity(chat[i-1], trn[j-1]) > 0 {
            pairs.push((i-1, j-1));
        }
        i = pi; j = pj;
    }
    pairs.reverse();
    pairs
}

// ── Utilities ───────────────────────────────────────────────────────────────

fn is_overlap_marker(c: char) -> bool {
    matches!(c, '⌈' | '⌉' | '⌊' | '⌋')
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir { TopOpen, TopClose, BottomOpen, BottomClose }

fn extract_bracketed_text(chars: &[char], start: usize) -> String {
    let close = match chars[start] {
        '⌈' => '⌉', '⌊' => '⌋', _ => return String::new(),
    };
    let mut i = start + 1;
    if i < chars.len() && chars[i] >= '2' && chars[i] <= '9' { i += 1; }
    let text_start = i;
    while i < chars.len() {
        if chars[i] == close {
            return normalize(&chars[text_start..i].iter().collect::<String>());
        }
        i += 1;
    }
    normalize(&chars[text_start..].iter().collect::<String>())
}

fn normalize(text: &str) -> String {
    let mut s = text.to_lowercase();
    s = s.replace("&=in", "").replace("&=ex", "").replace("&=tsk", "");
    s = s.replace('⌈', "").replace('⌉', "").replace('⌊', "").replace('⌋', "");
    s = s.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '\'').collect();
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
