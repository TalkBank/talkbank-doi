//! Global overlap inference — constraint-based assignment using petgraph.
//!
//! Operates on the entire TrnDocument at once. No incremental state, no
//! sensitivity to utterance boundaries.
//!
//! Algorithm:
//! 1. Pair each Open bracket with its matching Close by speaker + index
//! 2. Build an undirected graph where nodes = pairs, edges = "same overlap set"
//!    (from alignment edges, temporal overlap, line adjacency)
//! 3. Find connected components via union-find → overlap sets
//! 4. Assign top/bottom: first speaker in each component is top, others bottom

use std::collections::{BTreeMap, HashMap};

use petgraph::unionfind::UnionFind;

use crate::intermediate::*;
use crate::types::Diagnostic;

/// Infer overlap roles for all brackets in a TrnDocument.
pub fn infer_overlaps_global(doc: &TrnDocument) -> OverlapAssignment {
    let mut diag_items: Vec<Diagnostic> = Vec::new();

    // Step 1: Pair Open brackets with their matching Close.
    let pairs = pair_brackets(&doc.brackets, &mut diag_items);

    // Step 2: Group pairs into overlap sets via union-find on a constraint graph.
    let sets = group_into_sets(&pairs, doc);

    // Step 3: Assign top/bottom roles.
    let roles = assign_roles(&pairs, &sets);

    OverlapAssignment {
        filename: doc.filename.clone(),
        roles,
        inference_diagnostics: diag_items,
    }
}

// ── Step 1: Pair Open with Close ────────────────────────────────────────────

#[derive(Debug, Clone)]
struct BracketPair {
    open_id: u32,
    close_id: Option<u32>,
    speaker: String,
    lexical_index: Option<u8>,
    open_time: f64,
    close_time: f64,
    open_line: usize,
}

fn pair_brackets(brackets: &[BracketRef], diag: &mut Vec<Diagnostic>) -> Vec<BracketPair> {
    let mut pairs: Vec<BracketPair> = Vec::new();
    let mut open_stack: HashMap<(String, u8), Vec<usize>> = HashMap::new();

    for bracket in brackets {
        let key = (bracket.speaker.clone(), bracket.lexical_index.unwrap_or(0));

        match bracket.direction {
            BracketDirection::Open => {
                let pair_idx = pairs.len();
                pairs.push(BracketPair {
                    open_id: bracket.id,
                    close_id: None,
                    speaker: bracket.speaker.clone(),
                    lexical_index: bracket.lexical_index,
                    open_time: bracket.source.time_range.0,
                    close_time: bracket.source.time_range.1,
                    open_line: bracket.source.line_number,
                });
                open_stack.entry(key).or_default().push(pair_idx);
            }
            BracketDirection::Close | BracketDirection::CloseForced => {
                if let Some(stack) = open_stack.get_mut(&key) {
                    if let Some(pair_idx) = stack.pop() {
                        pairs[pair_idx].close_id = Some(bracket.id);
                        pairs[pair_idx].close_time = bracket.source.time_range.1;
                    } else {
                        diag.push(Diagnostic {
                            severity: crate::types::Severity::Warning,
                            line_number: bracket.source.line_number,
                            column: Some(bracket.source.column),
                            code: crate::types::DiagnosticCode::UnmatchedBracket,
                            message: format!(
                                "Close bracket by '{}' (index {:?}) has no matching open",
                                bracket.speaker, bracket.lexical_index
                            ),
                        });
                    }
                } else {
                    diag.push(Diagnostic {
                        severity: crate::types::Severity::Warning,
                        line_number: bracket.source.line_number,
                        column: Some(bracket.source.column),
                        code: crate::types::DiagnosticCode::UnmatchedBracket,
                        message: format!(
                            "Close bracket by '{}' (index {:?}) has no matching open",
                            bracket.speaker, bracket.lexical_index
                        ),
                    });
                }
            }
        }
    }

    for ((speaker, _), stack) in &open_stack {
        for &pair_idx in stack {
            diag.push(Diagnostic {
                severity: crate::types::Severity::Warning,
                line_number: pairs[pair_idx].open_line,
                column: None,
                code: crate::types::DiagnosticCode::IncompleteTop,
                message: format!("Open bracket by '{}' was never closed", speaker),
            });
        }
    }

    pairs
}

// ── Step 2: Group pairs via union-find ──────────────────────────────────────

#[derive(Debug)]
struct OverlapSet {
    pair_indices: Vec<usize>,
    set_index: usize,
}

/// Build a constraint graph over bracket pairs and find connected components.
///
/// Edge sources:
/// 1. Spatial alignment edges (strongest — transcriber's explicit visual cue)
/// 2. Temporal overlap + same lexical index + different speaker
/// 3. Line adjacency (≤2 lines apart) + same index + different speaker
///
/// Union-find gives transitive closure: A↔B and B↔C → A,B,C in same set.
fn group_into_sets(pairs: &[BracketPair], doc: &TrnDocument) -> Vec<OverlapSet> {
    let n = pairs.len();
    if n == 0 {
        return Vec::new();
    }

    let mut uf = UnionFind::new(n);

    // Lookup: open_bracket_id → pair_index.
    let open_to_pair: HashMap<u32, usize> = pairs
        .iter()
        .enumerate()
        .map(|(i, p)| (p.open_id, i))
        .collect();

    // Edge source 1: Alignment edges.
    for edge in &doc.alignment_edges {
        if let (Some(&a), Some(&b)) = (
            open_to_pair.get(&edge.aligned_bracket_id),
            open_to_pair.get(&edge.target_bracket_id),
        ) {
            if pairs[a].speaker != pairs[b].speaker
                && compatible_indices(pairs[a].lexical_index, pairs[b].lexical_index)
            {
                uf.union(a, b);
            }
        }
    }

    // Edge sources 2 + 3: Temporal overlap and line adjacency.
    for i in 0..n {
        for j in (0..i).rev() {
            // Stop if too far back in the document.
            let line_dist = pairs[i].open_line.abs_diff(pairs[j].open_line);
            if line_dist > 20 {
                break;
            }

            if pairs[i].speaker == pairs[j].speaker {
                continue;
            }

            if !compatible_indices(pairs[i].lexical_index, pairs[j].lexical_index) {
                continue;
            }

            // Edge source 2: Temporal overlap.
            if times_overlap(
                pairs[i].open_time, pairs[i].close_time,
                pairs[j].open_time, pairs[j].close_time,
            ) {
                uf.union(i, j);
                continue;
            }

            // Edge source 3: Line adjacency (≤2 lines, same index).
            if line_dist <= 2 {
                uf.union(i, j);
            }
        }
    }

    // Extract connected components.
    let mut component_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        component_map.entry(root).or_default().push(i);
    }

    // Post-process: split components that have multiple pairs from the same
    // speaker into separate sets. In a valid overlap set, each speaker
    // should appear at most once (one bracket pair per speaker per set).
    let mut split_components: Vec<Vec<usize>> = Vec::new();
    for (_, members) in &component_map {
        let sub_sets = split_same_speaker(members, pairs);
        split_components.extend(sub_sets);
    }
    let component_map: HashMap<usize, Vec<usize>> = split_components
        .into_iter()
        .enumerate()
        .map(|(i, members)| (i, members))
        .collect();

    let mut sets: Vec<OverlapSet> = component_map
        .into_values()
        .enumerate()
        .map(|(idx, mut pair_indices)| {
            pair_indices.sort();
            OverlapSet { pair_indices, set_index: idx }
        })
        .collect();

    sets.sort_by_key(|s| s.pair_indices[0]);
    for (i, set) in sets.iter_mut().enumerate() {
        set.set_index = i;
    }

    sets
}

/// Split a connected component into sub-sets where each speaker appears at most once.
/// Uses a greedy approach: iterate pairs in document order, assign each to the
/// first sub-set that doesn't already have that speaker.
fn split_same_speaker(members: &[usize], pairs: &[BracketPair]) -> Vec<Vec<usize>> {
    let mut sub_sets: Vec<Vec<usize>> = Vec::new();
    let mut sub_set_speakers: Vec<Vec<String>> = Vec::new();

    for &idx in members {
        let speaker = &pairs[idx].speaker;
        // Find the first sub-set that doesn't have this speaker.
        let mut placed = false;
        for (si, ss) in sub_set_speakers.iter_mut().enumerate() {
            if !ss.contains(speaker) {
                sub_sets[si].push(idx);
                ss.push(speaker.clone());
                placed = true;
                break;
            }
        }
        if !placed {
            sub_sets.push(vec![idx]);
            sub_set_speakers.push(vec![speaker.clone()]);
        }
    }

    sub_sets
}

fn compatible_indices(a: Option<u8>, b: Option<u8>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

fn times_overlap(a_start: f64, a_end: f64, b_start: f64, b_end: f64) -> bool {
    let tolerance = 0.2; // 200ms
    a_start < b_end + tolerance && b_start < a_end + tolerance
}

// ── Step 3: Assign top/bottom roles ─────────────────────────────────────────

fn assign_roles(
    pairs: &[BracketPair],
    sets: &[OverlapSet],
) -> BTreeMap<u32, BracketRole> {
    let mut roles = BTreeMap::new();

    for set in sets {
        if set.pair_indices.is_empty() {
            continue;
        }

        // First pair in document order is top; others are bottoms.
        let first_pair_idx = *set.pair_indices.iter().min().unwrap();

        let base_real_index = pairs[first_pair_idx]
            .lexical_index
            .map(|n| (n - 1) as usize)
            .unwrap_or(0);

        for &pair_idx in &set.pair_indices {
            let pair = &pairs[pair_idx];
            let is_top = pair_idx == first_pair_idx;

            let real_index = pair
                .lexical_index
                .map(|n| (n - 1) as usize)
                .unwrap_or(base_real_index);

            roles.insert(pair.open_id, BracketRole {
                bracket_id: pair.open_id,
                role: if is_top { OverlapRole::TopBegin } else { OverlapRole::BottomBegin },
                real_index,
            });

            if let Some(close_id) = pair.close_id {
                roles.insert(close_id, BracketRole {
                    bracket_id: close_id,
                    role: if is_top { OverlapRole::TopEnd } else { OverlapRole::BottomEnd },
                    real_index,
                });
            }
        }
    }

    roles
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_bracket(id: u32, dir: BracketDirection, idx: Option<u8>, speaker: &str, line: usize, time: (f64, f64), col: usize) -> BracketRef {
        BracketRef {
            id,
            direction: dir,
            lexical_index: idx,
            utterance_index: 0,
            element_position: 0,
            speaker: speaker.to_string(),
            source: BracketSource {
                line_number: line,
                char_offset: col,
                column: col,
                time_range: time,
            },
        }
    }

    fn make_doc(brackets: Vec<BracketRef>, alignment_edges: Vec<AlignmentEdge>) -> TrnDocument {
        TrnDocument {
            filename: "test.trn".to_string(),
            format_variant: crate::types::FormatVariant::A,
            total_lines: 100,
            speaker_map: BTreeMap::new(),
            speakers: vec![],
            utterances: vec![],
            brackets,
            alignment_edges,
            parse_diagnostics: vec![],
        }
    }

    #[test]
    fn simple_two_party_overlap() {
        let doc = make_doc(vec![
            make_bracket(0, BracketDirection::Open, None, "JAMIE", 1, (0.0, 6.0), 20),
            make_bracket(1, BracketDirection::Close, None, "JAMIE", 1, (0.0, 6.0), 50),
            make_bracket(2, BracketDirection::Open, None, "HAROLD", 2, (4.0, 6.0), 20),
            make_bracket(3, BracketDirection::Close, None, "HAROLD", 2, (4.0, 6.0), 40),
        ], vec![
            AlignmentEdge {
                aligned_bracket_id: 2,
                target_bracket_id: 0,
                column_delta: 0,
                line_distance: 1,
            },
        ]);

        let assignment = infer_overlaps_global(&doc);
        assert_eq!(assignment.roles[&0].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&1].role, OverlapRole::TopEnd);
        assert_eq!(assignment.roles[&2].role, OverlapRole::BottomBegin);
        assert_eq!(assignment.roles[&3].role, OverlapRole::BottomEnd);
    }

    #[test]
    fn numbered_brackets_separate_sets() {
        let doc = make_doc(vec![
            make_bracket(0, BracketDirection::Open, None, "JAMIE", 1, (0.0, 6.0), 10),
            make_bracket(1, BracketDirection::Close, None, "JAMIE", 1, (0.0, 6.0), 40),
            make_bracket(2, BracketDirection::Open, Some(2), "JAMIE", 1, (0.0, 6.0), 50),
            make_bracket(3, BracketDirection::Close, Some(2), "JAMIE", 1, (0.0, 6.0), 60),
            make_bracket(4, BracketDirection::Open, None, "HAROLD", 2, (4.0, 6.0), 10),
            make_bracket(5, BracketDirection::Close, None, "HAROLD", 2, (4.0, 6.0), 40),
            make_bracket(6, BracketDirection::Open, Some(2), "HAROLD", 2, (4.0, 6.0), 50),
            make_bracket(7, BracketDirection::Close, Some(2), "HAROLD", 2, (4.0, 6.0), 60),
        ], vec![]);

        let assignment = infer_overlaps_global(&doc);
        assert_eq!(assignment.roles[&0].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&4].role, OverlapRole::BottomBegin);
        assert_eq!(assignment.roles[&0].real_index, 0);
        assert_eq!(assignment.roles[&4].real_index, 0);
        assert_eq!(assignment.roles[&2].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&6].role, OverlapRole::BottomBegin);
        assert_eq!(assignment.roles[&2].real_index, 1);
        assert_eq!(assignment.roles[&6].real_index, 1);
    }

    #[test]
    fn temporal_gap_separates_groups() {
        let doc = make_doc(vec![
            make_bracket(0, BracketDirection::Open, None, "JAMIE", 1, (0.0, 6.0), 10),
            make_bracket(1, BracketDirection::Close, None, "JAMIE", 1, (0.0, 6.0), 40),
            make_bracket(2, BracketDirection::Open, None, "HAROLD", 2, (4.0, 6.0), 10),
            make_bracket(3, BracketDirection::Close, None, "HAROLD", 2, (4.0, 6.0), 40),
            make_bracket(4, BracketDirection::Open, None, "JAMIE", 30, (20.0, 26.0), 10),
            make_bracket(5, BracketDirection::Close, None, "JAMIE", 30, (20.0, 26.0), 40),
            make_bracket(6, BracketDirection::Open, None, "MILES", 31, (24.0, 26.0), 10),
            make_bracket(7, BracketDirection::Close, None, "MILES", 31, (24.0, 26.0), 40),
        ], vec![]);

        let assignment = infer_overlaps_global(&doc);
        // Group 1
        assert_eq!(assignment.roles[&0].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&2].role, OverlapRole::BottomBegin);
        // Group 2 (separated by temporal gap + line distance)
        assert_eq!(assignment.roles[&4].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&6].role, OverlapRole::BottomBegin);
    }

    #[test]
    fn transitive_closure_three_party() {
        // A overlaps B, B overlaps C → A,B,C all in same set.
        let doc = make_doc(vec![
            make_bracket(0, BracketDirection::Open, None, "A", 1, (0.0, 6.0), 10),
            make_bracket(1, BracketDirection::Close, None, "A", 1, (0.0, 6.0), 40),
            make_bracket(2, BracketDirection::Open, None, "B", 2, (4.0, 8.0), 10),
            make_bracket(3, BracketDirection::Close, None, "B", 2, (4.0, 8.0), 40),
            make_bracket(4, BracketDirection::Open, None, "C", 3, (6.0, 10.0), 10),
            make_bracket(5, BracketDirection::Close, None, "C", 3, (6.0, 10.0), 40),
        ], vec![]);

        let assignment = infer_overlaps_global(&doc);
        // A is first → top. B and C are bottoms.
        assert_eq!(assignment.roles[&0].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&2].role, OverlapRole::BottomBegin);
        assert_eq!(assignment.roles[&4].role, OverlapRole::BottomBegin);
    }
}
