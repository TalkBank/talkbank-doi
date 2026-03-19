//! Auto-number TRN brackets based on inference results.
//!
//! Reads a TrnDocument + OverlapAssignment and produces a report of:
//! 1. Brackets that can be confidently numbered (from inference + alignment)
//! 2. Brackets that are ambiguous and need human review
//!
//! Also produces an updated TRN file with explicit indices where possible.

use std::collections::HashMap;
use std::path::Path;

use crate::intermediate::*;

/// A suggested index annotation for a bracket in the TRN source.
#[derive(Debug, Clone)]
pub struct BracketAnnotation {
    /// The bracket's unique ID.
    pub bracket_id: u32,
    /// Source location.
    pub line_number: usize,
    pub char_offset: usize,
    /// Original lexical index (None = unnumbered).
    pub original_index: Option<u8>,
    /// Suggested index (1-9, where 1 = what was previously unnumbered).
    pub suggested_index: u8,
    /// How confident we are in this suggestion.
    pub confidence: Confidence,
    /// Why we chose this index.
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Confidence {
    /// Numbered in the original TRN — no change needed.
    Original,
    /// Strong evidence: alignment edge + temporal overlap + adjacent lines.
    High,
    /// Moderate evidence: temporal overlap or adjacency, but no alignment edge.
    Medium,
    /// Weak evidence: assigned by inference but could be wrong.
    Low,
    /// No evidence: singleton bracket, needs human review.
    NeedsReview,
}

/// Analyze a document's brackets and produce annotations.
pub fn analyze_brackets(
    doc: &TrnDocument,
    assignment: &OverlapAssignment,
) -> Vec<BracketAnnotation> {
    let mut annotations = Vec::new();

    // Build alignment lookup for confidence assessment.
    let has_alignment: std::collections::HashSet<u32> = doc
        .alignment_edges
        .iter()
        .flat_map(|e| [e.aligned_bracket_id, e.target_bracket_id])
        .collect();

    // Group brackets by their assigned set (from the inference).
    // The set is identified by the real_index of the role.
    // Brackets in the same set share the same overlap group.
    let mut set_members: HashMap<usize, Vec<u32>> = HashMap::new();
    for (bracket_id, role) in &assignment.roles {
        set_members
            .entry(role.real_index)
            .or_default()
            .push(*bracket_id);
    }

    // For each bracket, determine its annotation.
    for bracket in &doc.brackets {
        if bracket.direction != BracketDirection::Open {
            // Only annotate open brackets — close brackets mirror them.
            continue;
        }

        let role = assignment.roles.get(&bracket.id);

        let (suggested_index, confidence, reason) = if let Some(idx) = bracket.lexical_index {
            // Already numbered in the original TRN.
            (idx, Confidence::Original, "numbered in original TRN".to_string())
        } else if let Some(role) = role {
            // Unnumbered but assigned by inference.
            let display_index = (role.real_index % 9) as u8 + 1; // 1-based for display

            // Assess confidence.
            if has_alignment.contains(&bracket.id) {
                (display_index, Confidence::High, format!(
                    "alignment edge + inference (set {})", role.real_index
                ))
            } else {
                // Check if the set has multiple members (paired with another speaker).
                let set_size = set_members
                    .get(&role.real_index)
                    .map_or(0, |v| v.len());
                if set_size >= 2 {
                    (display_index, Confidence::Medium, format!(
                        "temporal overlap + inference (set {}, {} members)",
                        role.real_index, set_size
                    ))
                } else {
                    (display_index, Confidence::Low, format!(
                        "singleton set {} — no partner found", role.real_index
                    ))
                }
            }
        } else {
            // Not assigned by inference at all.
            (1, Confidence::NeedsReview, "no inference result".to_string())
        };

        annotations.push(BracketAnnotation {
            bracket_id: bracket.id,
            line_number: bracket.source.line_number,
            char_offset: bracket.source.char_offset,
            original_index: bracket.lexical_index,
            suggested_index,
            confidence,
            reason,
        });
    }

    annotations
}

/// Generate a report of brackets that need human review.
pub fn review_report(annotations: &[BracketAnnotation]) -> String {
    let mut report = String::new();
    let mut by_confidence = HashMap::new();
    for ann in annotations {
        by_confidence
            .entry(ann.confidence)
            .or_insert_with(Vec::new)
            .push(ann);
    }

    report.push_str("# Bracket Annotation Report\n\n");

    let total = annotations.len();
    let original = by_confidence.get(&Confidence::Original).map_or(0, |v: &Vec<&BracketAnnotation>| v.len());
    let high = by_confidence.get(&Confidence::High).map_or(0, |v: &Vec<&BracketAnnotation>| v.len());
    let medium = by_confidence.get(&Confidence::Medium).map_or(0, |v: &Vec<&BracketAnnotation>| v.len());
    let low = by_confidence.get(&Confidence::Low).map_or(0, |v: &Vec<&BracketAnnotation>| v.len());
    let review = by_confidence.get(&Confidence::NeedsReview).map_or(0, |v: &Vec<&BracketAnnotation>| v.len());

    report.push_str(&format!("Total open brackets: {total}\n"));
    report.push_str(&format!("  Original (already numbered): {original}\n"));
    report.push_str(&format!("  High confidence (alignment + inference): {high}\n"));
    report.push_str(&format!("  Medium confidence (temporal overlap): {medium}\n"));
    report.push_str(&format!("  Low confidence (singleton): {low}\n"));
    report.push_str(&format!("  Needs review: {review}\n\n"));

    if low + review > 0 {
        report.push_str("## Brackets needing attention\n\n");
        for ann in annotations {
            if ann.confidence == Confidence::Low || ann.confidence == Confidence::NeedsReview {
                report.push_str(&format!(
                    "- Line {}, col {}: {:?} — {}\n",
                    ann.line_number, ann.char_offset, ann.confidence, ann.reason
                ));
            }
        }
    }

    report
}

/// Apply auto-numbering to a TRN file.
/// Only applies High and Medium confidence annotations.
/// Low and NeedsReview are left unchanged for human review.
pub fn apply_autonumber(
    trn_content: &str,
    annotations: &[BracketAnnotation],
    min_confidence: Confidence,
) -> String {
    // Build a map of (line_number, char_offset) -> suggested_index for applicable annotations.
    let mut replacements: HashMap<(usize, usize), u8> = HashMap::new();
    for ann in annotations {
        if ann.original_index.is_some() {
            continue; // Already numbered — skip.
        }
        let dominated = match (min_confidence, ann.confidence) {
            (Confidence::High, Confidence::High) => true,
            (Confidence::Medium, Confidence::High | Confidence::Medium) => true,
            (Confidence::Low, Confidence::High | Confidence::Medium | Confidence::Low) => true,
            _ => false,
        };
        if dominated {
            replacements.insert((ann.line_number, ann.char_offset), ann.suggested_index);
        }
    }

    if replacements.is_empty() {
        return trn_content.to_string();
    }

    // Apply replacements line by line.
    let mut result = String::new();
    for (line_idx, line) in trn_content.lines().enumerate() {
        let line_number = line_idx + 1;
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if let Some(&index) = replacements.get(&(line_number, i)) {
                if chars[i] == '[' {
                    // Replace [ with [N
                    result.push('[');
                    if index >= 2 {
                        result.push(char::from(b'0' + index));
                    }
                    i += 1;
                    // Skip existing digit if there was one (shouldn't happen for unnumbered).
                    continue;
                }
            }

            // Check for close bracket that needs numbering.
            // We need to find the matching close and add the index there too.
            // For now, we only number open brackets — the close indices
            // are handled by looking for ] preceded by the bracket content.

            result.push(chars[i]);
            i += 1;
        }
        result.push('\n');
    }

    result
}
