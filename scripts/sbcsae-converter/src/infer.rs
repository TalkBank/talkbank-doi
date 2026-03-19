//! Global overlap inference — constraint-based assignment.
//!
//! Unlike the Java-ported state machine (overlap.rs), this operates on the
//! entire TrnDocument at once. No incremental state, no sensitivity to
//! utterance boundaries, no `lastSawOverlap` heuristic.
//!
//! Algorithm:
//! 1. Pair each Open bracket with its matching Close by index and speaker
//! 2. Group bracket pairs into overlap sets using temporal overlap + alignment edges
//! 3. Assign top/bottom: first speaker in each set is top, others are bottom

use std::collections::{BTreeMap, HashMap};

use crate::intermediate::*;
use crate::types::Diagnostic;

/// Infer overlap roles for all brackets in a TrnDocument.
pub fn infer_overlaps_global(doc: &TrnDocument) -> OverlapAssignment {
    let mut diag_items: Vec<Diagnostic> = Vec::new();

    // Step 1: Pair Open brackets with their matching Close.
    let pairs = pair_brackets(&doc.brackets, &mut diag_items);

    // Step 2: Group pairs into overlap sets.
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

/// A matched pair of Open + Close brackets from the same speaker and index.
#[derive(Debug, Clone)]
struct BracketPair {
    open_id: u32,
    close_id: Option<u32>,
    speaker: String,
    lexical_index: Option<u8>,
    /// Start time of the line containing the open bracket (seconds).
    open_time: f64,
    /// End time of the line containing the close bracket (seconds).
    close_time: f64,
    /// Source line of the open bracket.
    open_line: usize,
}

fn pair_brackets(brackets: &[BracketRef], diag: &mut Vec<Diagnostic>) -> Vec<BracketPair> {
    let mut pairs: Vec<BracketPair> = Vec::new();
    // Stack of open brackets per (speaker, lexical_index) waiting for a close.
    // Key: (speaker, lexical_index_as_u8_or_0)
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
                // Find the most recent unmatched open for this speaker + index.
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

    // Diagnose unclosed opens.
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

// ── Step 2: Group pairs into overlap sets ───────────────────────────────────

/// An overlap set: a group of bracket pairs that overlap in time.
/// The first pair (by document order) is the top; others are bottoms.
#[derive(Debug)]
struct OverlapSet {
    /// Indices into the `pairs` array.
    pair_indices: Vec<usize>,
    /// Sequential index for this set within its run (for CHAT index emission).
    set_index: usize,
}

fn group_into_sets(pairs: &[BracketPair], doc: &TrnDocument) -> Vec<OverlapSet> {
    // Build alignment lookup: open_bracket_id → target_open_bracket_id.
    let alignment_target: HashMap<u32, u32> = doc
        .alignment_edges
        .iter()
        .map(|e| (e.aligned_bracket_id, e.target_bracket_id))
        .collect();

    // Reverse map: open_bracket_id → pair_index.
    let open_to_pair: HashMap<u32, usize> = pairs
        .iter()
        .enumerate()
        .map(|(i, p)| (p.open_id, i))
        .collect();

    // For each pair, determine which set it belongs to.
    // Strategy: numbered brackets (index 2-9) are grouped by index within
    // a contiguous run. Unnumbered brackets are grouped by temporal overlap
    // and alignment edges.
    let mut pair_to_set: Vec<Option<usize>> = vec![None; pairs.len()];
    let mut sets: Vec<OverlapSet> = Vec::new();
    let mut set_counter = 0;

    for (i, pair) in pairs.iter().enumerate() {
        if pair_to_set[i].is_some() {
            continue; // Already assigned.
        }

        // Check alignment edge: does this pair's open bracket align with
        // another pair's open bracket?
        if let Some(&target_id) = alignment_target.get(&pair.open_id) {
            if let Some(&target_pair_idx) = open_to_pair.get(&target_id) {
                if let Some(existing_set) = pair_to_set[target_pair_idx] {
                    // Join the target's set.
                    sets[existing_set].pair_indices.push(i);
                    pair_to_set[i] = Some(existing_set);
                    continue;
                }
            }
        }

        // Check temporal overlap with recent pairs from different speakers.
        let mut found_set = None;
        for j in (0..i).rev() {
            if pairs[j].speaker == pair.speaker {
                continue; // Same speaker — can't be in the same set.
            }

            // Check temporal overlap: does this pair's time range overlap
            // with pair j's time range?
            let overlap = times_overlap(
                pair.open_time,
                pair.close_time,
                pairs[j].open_time,
                pairs[j].close_time,
            );

            // Require matching lexical index — unnumbered pairs with unnumbered,
            // [2] pairs with [2], etc. This prevents cross-index grouping.
            if pair.lexical_index != pairs[j].lexical_index {
                continue;
            }

            if !overlap {
                // Check proximity: are they on adjacent lines?
                let line_dist = pair.open_line.abs_diff(pairs[j].open_line);
                if line_dist > 3 {
                    break; // Too far back — stop searching.
                }
                // Same index, adjacent lines, different speaker → same set.
            }

            // Found a temporal overlap or adjacent same-index pair.
            if let Some(existing_set) = pair_to_set[j] {
                // Join pair j's set.
                sets[existing_set].pair_indices.push(i);
                pair_to_set[i] = Some(existing_set);
                found_set = Some(existing_set);
                break;
            } else {
                // Create a new set with both pairs.
                let set_idx = sets.len();
                sets.push(OverlapSet {
                    pair_indices: vec![j, i],
                    set_index: set_counter,
                });
                set_counter += 1;
                pair_to_set[j] = Some(set_idx);
                pair_to_set[i] = Some(set_idx);
                found_set = Some(set_idx);
                break;
            }
        }

        if found_set.is_none() {
            // No match found — this pair starts a new set by itself.
            let set_idx = sets.len();
            sets.push(OverlapSet {
                pair_indices: vec![i],
                set_index: set_counter,
            });
            set_counter += 1;
            pair_to_set[i] = Some(set_idx);
        }
    }

    sets
}

fn times_overlap(a_start: f64, a_end: f64, b_start: f64, b_end: f64) -> bool {
    // Two intervals overlap if neither ends before the other starts.
    // Allow a small tolerance (100ms) for near-simultaneous speech.
    let tolerance = 0.1;
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

        // The first pair in document order is the top.
        // All others are bottoms.
        let first_pair_idx = *set.pair_indices.iter().min().unwrap();

        // Determine the real_index for CHAT emission.
        // For numbered brackets, use the lexical index directly.
        // For unnumbered brackets in a set, use 0 (unnumbered).
        // If the set contains mixed indices, use the lowest.
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

            // Assign open bracket role.
            let open_role = if is_top {
                OverlapRole::TopBegin
            } else {
                OverlapRole::BottomBegin
            };
            roles.insert(pair.open_id, BracketRole {
                bracket_id: pair.open_id,
                role: open_role,
                real_index,
            });

            // Assign close bracket role (if paired).
            if let Some(close_id) = pair.close_id {
                let close_role = if is_top {
                    OverlapRole::TopEnd
                } else {
                    OverlapRole::BottomEnd
                };
                roles.insert(close_id, BracketRole {
                    bracket_id: close_id,
                    role: close_role,
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

    #[test]
    fn simple_two_party_overlap() {
        let brackets = vec![
            make_bracket(0, BracketDirection::Open, None, "JAMIE", 1, (0.0, 6.0), 20),
            make_bracket(1, BracketDirection::Close, None, "JAMIE", 1, (0.0, 6.0), 50),
            make_bracket(2, BracketDirection::Open, None, "HAROLD", 2, (4.0, 6.0), 20),
            make_bracket(3, BracketDirection::Close, None, "HAROLD", 2, (4.0, 6.0), 40),
        ];

        let doc = TrnDocument {
            filename: "test.trn".to_string(),
            format_variant: crate::types::FormatVariant::A,
            total_lines: 2,
            speaker_map: BTreeMap::new(),
            speakers: vec!["JAMIE".to_string(), "HAROLD".to_string()],
            utterances: vec![],
            brackets: brackets.clone(),
            alignment_edges: vec![
                AlignmentEdge {
                    aligned_bracket_id: 2,
                    target_bracket_id: 0,
                    column_delta: 0,
                    line_distance: 1,
                },
            ],
            parse_diagnostics: vec![],
        };

        let assignment = infer_overlaps_global(&doc);

        // JAMIE (first in document order) should be top.
        assert_eq!(assignment.roles[&0].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&1].role, OverlapRole::TopEnd);
        // HAROLD should be bottom.
        assert_eq!(assignment.roles[&2].role, OverlapRole::BottomBegin);
        assert_eq!(assignment.roles[&3].role, OverlapRole::BottomEnd);
    }

    #[test]
    fn numbered_brackets_separate_sets() {
        let brackets = vec![
            // [  — unnumbered, JAMIE
            make_bracket(0, BracketDirection::Open, None, "JAMIE", 1, (0.0, 6.0), 10),
            make_bracket(1, BracketDirection::Close, None, "JAMIE", 1, (0.0, 6.0), 40),
            // [2 — numbered, JAMIE
            make_bracket(2, BracketDirection::Open, Some(2), "JAMIE", 1, (0.0, 6.0), 50),
            make_bracket(3, BracketDirection::Close, Some(2), "JAMIE", 1, (0.0, 6.0), 60),
            // [  — unnumbered, HAROLD (bottom of unnumbered)
            make_bracket(4, BracketDirection::Open, None, "HAROLD", 2, (4.0, 6.0), 10),
            make_bracket(5, BracketDirection::Close, None, "HAROLD", 2, (4.0, 6.0), 40),
            // [2 — numbered, HAROLD (bottom of [2])
            make_bracket(6, BracketDirection::Open, Some(2), "HAROLD", 2, (4.0, 6.0), 50),
            make_bracket(7, BracketDirection::Close, Some(2), "HAROLD", 2, (4.0, 6.0), 60),
        ];

        let doc = TrnDocument {
            filename: "test.trn".to_string(),
            format_variant: crate::types::FormatVariant::A,
            total_lines: 2,
            speaker_map: BTreeMap::new(),
            speakers: vec!["JAMIE".to_string(), "HAROLD".to_string()],
            utterances: vec![],
            brackets,
            alignment_edges: vec![],
            parse_diagnostics: vec![],
        };

        let assignment = infer_overlaps_global(&doc);

        // Unnumbered set: JAMIE top, HAROLD bottom.
        assert_eq!(assignment.roles[&0].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&4].role, OverlapRole::BottomBegin);
        assert_eq!(assignment.roles[&0].real_index, 0);
        assert_eq!(assignment.roles[&4].real_index, 0);

        // [2] set: JAMIE top, HAROLD bottom.
        assert_eq!(assignment.roles[&2].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&6].role, OverlapRole::BottomBegin);
        assert_eq!(assignment.roles[&2].real_index, 1);
        assert_eq!(assignment.roles[&6].real_index, 1);
    }

    #[test]
    fn temporal_gap_separates_groups() {
        let brackets = vec![
            // Group 1: JAMIE and HAROLD overlap at t=0-6
            make_bracket(0, BracketDirection::Open, None, "JAMIE", 1, (0.0, 6.0), 10),
            make_bracket(1, BracketDirection::Close, None, "JAMIE", 1, (0.0, 6.0), 40),
            make_bracket(2, BracketDirection::Open, None, "HAROLD", 2, (4.0, 6.0), 10),
            make_bracket(3, BracketDirection::Close, None, "HAROLD", 2, (4.0, 6.0), 40),
            // Group 2: JAMIE and MILES overlap at t=20-26 (big gap)
            make_bracket(4, BracketDirection::Open, None, "JAMIE", 10, (20.0, 26.0), 10),
            make_bracket(5, BracketDirection::Close, None, "JAMIE", 10, (20.0, 26.0), 40),
            make_bracket(6, BracketDirection::Open, None, "MILES", 11, (24.0, 26.0), 10),
            make_bracket(7, BracketDirection::Close, None, "MILES", 11, (24.0, 26.0), 40),
        ];

        let doc = TrnDocument {
            filename: "test.trn".to_string(),
            format_variant: crate::types::FormatVariant::A,
            total_lines: 11,
            speaker_map: BTreeMap::new(),
            speakers: vec![],
            utterances: vec![],
            brackets,
            alignment_edges: vec![],
            parse_diagnostics: vec![],
        };

        let assignment = infer_overlaps_global(&doc);

        // Group 1: JAMIE top, HAROLD bottom.
        assert_eq!(assignment.roles[&0].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&2].role, OverlapRole::BottomBegin);
        // Group 2: JAMIE top, MILES bottom (separate set due to temporal gap).
        assert_eq!(assignment.roles[&4].role, OverlapRole::TopBegin);
        assert_eq!(assignment.roles[&6].role, OverlapRole::BottomBegin);
    }
}
