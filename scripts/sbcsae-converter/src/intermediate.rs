//! Intermediate data model for TRN documents.
//!
//! This is the clean separation point between TRN parsing and CHAT inference.
//! The TRN format has NO concept of overlap top vs bottom — brackets are
//! symmetric. This model preserves that symmetry. Top/bottom assignment is
//! performed by a separate inference step that produces an `OverlapAssignment`.
//!
//! The model also captures spatial (indentation) alignment information that
//! the TRN format uses to visually indicate overlap correspondence.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::types::{Diagnostic, FormatVariant};

// ── Document ────────────────────────────────────────────────────────────────

/// A fully-parsed TRN file, ready for downstream processing.
/// No CHAT-specific inference (top/bottom) has been applied.
/// Serializable to JSON for inspection and tooling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrnDocument {
    /// Source filename (e.g., "SBC002.trn").
    pub filename: String,
    /// Detected format variant.
    pub format_variant: FormatVariant,
    /// Total lines in source file.
    pub total_lines: usize,
    /// TRN full name → CHAT truncated ID.
    pub speaker_map: BTreeMap<String, String>,
    /// Ordered list of speakers as they first appear.
    pub speakers: Vec<String>,
    /// Utterances in document order (before any sorting).
    pub utterances: Vec<TrnUtterance>,
    /// All brackets in document order, with spatial alignment computed.
    pub brackets: Vec<BracketRef>,
    /// Spatial alignment edges between brackets.
    pub alignment_edges: Vec<AlignmentEdge>,
    /// Diagnostics from parsing stages.
    pub parse_diagnostics: Vec<Diagnostic>,
}

// ── Utterance ───────────────────────────────────────────────────────────────

/// A single utterance (one speaker's turn or sub-turn).
///
/// Formed by grouping consecutive TRN lines from the same speaker,
/// splitting at terminators. Contains content elements with bracket
/// positions but NO top/bottom role assignment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrnUtterance {
    /// 0-based index in the document's utterance list.
    pub index: usize,
    /// TRN speaker name (full, e.g., "JAMIE").
    pub speaker: String,
    /// Content elements in order.
    pub elements: Vec<ContentElement>,
    /// How this utterance ends.
    pub terminator: Terminator,
    /// Start time in milliseconds.
    pub start_ms: Option<i64>,
    /// End time in milliseconds.
    pub end_ms: Option<i64>,
    /// Source line range (1-based, inclusive).
    pub source_lines: LineRange,
}

/// Inclusive line range in the source file.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LineRange {
    pub first: usize,
    pub last: usize,
}

// ── Content Elements ────────────────────────────────────────────────────────

/// A content element within an utterance.
///
/// TRN-specific notation has been transformed (= → :, % → ʔ, etc.)
/// but overlap brackets carry NO top/bottom role.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentElement {
    /// A word (may include lengthening colons, glottal ʔ, etc.).
    Word(String),

    /// An overlap bracket — position and index only, no role.
    /// The `bracket_id` references a `BracketRef` in `TrnDocument.brackets`.
    Bracket(u32),

    /// Short pause (..)
    PauseShort,
    /// Medium pause (...)
    PauseMedium,
    /// Timed pause with duration string.
    PauseTimed(String),

    /// &=in
    Inhalation,
    /// &=in &=lengthened
    InhalationLengthened,
    /// &=ex
    Exhalation,
    /// &=tsk
    Click,
    /// &=name (general vocalism, laugh, environmental comment).
    Vocalism(String),
    /// Multiple laughs.
    Laughs(usize),

    /// Long feature scope begin: &{l=LABEL
    LongFeatureBegin(String),
    /// Long feature scope end: &}l=LABEL
    LongFeatureEnd(String),
    /// Nonvocal scope begin: &{n=LABEL
    NonvocalBegin(String),
    /// Nonvocal scope end: &}n=LABEL
    NonvocalEnd(String),
    /// Simple nonvocal: &{n=LABEL}
    NonvocalSimple(String),
    /// Beat within nonvocal.
    NonvocalBeat,

    /// Phonological fragment: /text/
    PhonologicalFragment(String),
    /// Standalone glottal stop.
    Glottal,
    /// Comma (intonation marker).
    Comma,
}

// ── Brackets ────────────────────────────────────────────────────────────────

/// A reference to an overlap bracket within the document.
///
/// This is the core type that enables separation of parsing from inference.
/// It records everything known from the TRN source — position, index,
/// speaker, timing, column — but does NOT assign top/bottom role.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BracketRef {
    /// Unique ID for this bracket within the document (sequential, 0-based).
    pub id: u32,
    /// Open, Close, or CloseForced.
    pub direction: BracketDirection,
    /// Lexical index: None = unnumbered, Some(2..=9) = numbered.
    pub lexical_index: Option<u8>,
    /// Which utterance this bracket appears in.
    pub utterance_index: usize,
    /// 0-based position within the utterance's element list.
    pub element_position: usize,
    /// Speaker who owns this bracket.
    pub speaker: String,

    /// Source location — for diagnostics and spatial alignment.
    pub source: BracketSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BracketDirection {
    Open,
    Close,
    /// $] or N$] — forced close.
    CloseForced,
}

/// Source-file location of a bracket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BracketSource {
    /// 1-based line number in the source file.
    pub line_number: usize,
    /// 0-based character offset within the line's content field.
    pub char_offset: usize,
    /// 0-based absolute column in the original line (includes timestamp +
    /// speaker field widths). This is the spatial alignment signal.
    pub column: usize,
    /// Timestamp range of the line this bracket appears on.
    pub time_range: (f64, f64),
}

// ── Spatial Alignment ───────────────────────────────────────────────────────

/// An edge connecting two brackets that appear spatially aligned in the
/// original TRN file — i.e., their column positions are close enough to
/// suggest the transcriber intended them to correspond.
///
/// These edges are heuristic (decorative, not authoritative — see
/// trn-format-analysis.md §7). They provide a signal for overlap inference
/// but can be wrong. The inference tool should weight them alongside
/// temporal overlap, speaker identity, and numbered indices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentEdge {
    /// The bracket that is spatially aligned (the one with leading spaces).
    pub aligned_bracket_id: u32,
    /// The bracket it appears to align with (in a preceding/adjacent turn).
    pub target_bracket_id: u32,
    /// Column difference (0 = exact alignment).
    pub column_delta: usize,
    /// Confidence: how many lines apart the two brackets are.
    /// Adjacent lines (delta=1) are high confidence; distant lines are low.
    pub line_distance: usize,
}

// ── Terminator ──────────────────────────────────────────────────────────────

/// How an utterance ends in the TRN source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Terminator {
    Period,
    Question,
    /// -- at end, next speaker is different → interruption.
    Interruption,
    /// -- at end, same speaker continues → trailing off.
    TrailOff,
    /// & continuation or comma at end → self-completion.
    SelfCompletion,
    /// No explicit terminator in source. CHAT emitter must decide what to insert.
    Implicit,
}

// ── Overlap Assignment (output of inference, separate from TrnDocument) ─────

/// The result of overlap inference, applied to a TrnDocument.
/// Maps bracket IDs to roles. This is the ONLY place top/bottom exists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlapAssignment {
    /// Source document filename (for verification).
    pub filename: String,
    /// Per-bracket role assignment. Key = BracketRef.id.
    pub roles: BTreeMap<u32, BracketRole>,
    /// Diagnostics from the inference stage.
    pub inference_diagnostics: Vec<Diagnostic>,
}

/// Role assigned to a specific bracket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BracketRole {
    pub bracket_id: u32,
    /// Top or bottom, begin or end.
    pub role: OverlapRole,
    /// Real index (0-based, after wraparound resolution).
    pub real_index: usize,
}

/// The four possible roles for a bracket in CHAT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverlapRole {
    TopBegin,    // ⌈
    TopEnd,      // ⌉
    BottomBegin, // ⌊
    BottomEnd,   // ⌋
}

// ── Alignment Computation ───────────────────────────────────────────────────

/// Compute spatial alignment edges between brackets in a document.
///
/// For each bracket that has leading whitespace (indentation), look for
/// brackets in nearby utterances (within `max_line_distance` lines) whose
/// column position is within `max_column_delta` characters.
pub fn compute_alignment_edges(
    brackets: &[BracketRef],
    max_column_delta: usize,
    max_line_distance: usize,
) -> Vec<AlignmentEdge> {
    let mut edges = Vec::new();

    for (i, bracket) in brackets.iter().enumerate() {
        // Only look at Open brackets with significant indentation
        // (content_column > 0 suggests leading spaces for alignment).
        if bracket.direction != BracketDirection::Open {
            continue;
        }
        if bracket.source.char_offset == 0 {
            // At the start of content — no indentation, skip.
            continue;
        }

        // Search backward for a matching Open bracket from a different speaker.
        for j in (0..i).rev() {
            let candidate = &brackets[j];
            if candidate.direction != BracketDirection::Open {
                continue;
            }
            if candidate.speaker == bracket.speaker {
                continue;
            }

            let line_dist = bracket.source.line_number.saturating_sub(candidate.source.line_number);
            if line_dist > max_line_distance {
                break; // Too far back.
            }

            let col_delta = bracket.source.column.abs_diff(candidate.source.column);
            if col_delta <= max_column_delta {
                edges.push(AlignmentEdge {
                    aligned_bracket_id: bracket.id,
                    target_bracket_id: candidate.id,
                    column_delta: col_delta,
                    line_distance: line_dist,
                });
                break; // Take the first (closest) match.
            }
        }
    }

    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment_edge_exact() {
        let brackets = vec![
            BracketRef {
                id: 0,
                direction: BracketDirection::Open,
                lexical_index: None,
                utterance_index: 0,
                element_position: 2,
                speaker: "JAMIE".to_string(),
                source: BracketSource {
                    line_number: 1,
                    char_offset: 4,
                    column: 20,
                    time_range: (0.0, 6.52),
                },
            },
            BracketRef {
                id: 1,
                direction: BracketDirection::Open,
                lexical_index: None,
                utterance_index: 1,
                element_position: 0,
                speaker: "HAROLD".to_string(),
                source: BracketSource {
                    line_number: 2,
                    char_offset: 4, // indented to match
                    column: 20,     // same column as JAMIE's bracket
                    time_range: (4.43, 5.78),
                },
            },
        ];

        let edges = compute_alignment_edges(&brackets, 2, 5);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].aligned_bracket_id, 1);
        assert_eq!(edges[0].target_bracket_id, 0);
        assert_eq!(edges[0].column_delta, 0);
        assert_eq!(edges[0].line_distance, 1);
    }

    #[test]
    fn no_alignment_same_speaker() {
        let brackets = vec![
            BracketRef {
                id: 0,
                direction: BracketDirection::Open,
                lexical_index: None,
                utterance_index: 0,
                element_position: 0,
                speaker: "JAMIE".to_string(),
                source: BracketSource {
                    line_number: 1,
                    char_offset: 4,
                    column: 20,
                    time_range: (0.0, 5.0),
                },
            },
            BracketRef {
                id: 1,
                direction: BracketDirection::Open,
                lexical_index: None,
                utterance_index: 1,
                element_position: 0,
                speaker: "JAMIE".to_string(), // same speaker — no alignment
                source: BracketSource {
                    line_number: 2,
                    char_offset: 4,
                    column: 20,
                    time_range: (5.0, 8.0),
                },
            },
        ];

        let edges = compute_alignment_edges(&brackets, 2, 5);
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn no_alignment_at_content_start() {
        let brackets = vec![
            BracketRef {
                id: 0,
                direction: BracketDirection::Open,
                lexical_index: None,
                utterance_index: 0,
                element_position: 0,
                speaker: "JAMIE".to_string(),
                source: BracketSource {
                    line_number: 1,
                    char_offset: 0, // at content start — no indentation
                    column: 16,
                    time_range: (0.0, 5.0),
                },
            },
            BracketRef {
                id: 1,
                direction: BracketDirection::Open,
                lexical_index: None,
                utterance_index: 1,
                element_position: 0,
                speaker: "HAROLD".to_string(),
                source: BracketSource {
                    line_number: 2,
                    char_offset: 0, // also at content start — no indentation
                    column: 16,
                    time_range: (4.0, 6.0),
                },
            },
        ];

        // Neither bracket is indented, so no alignment edge.
        let edges = compute_alignment_edges(&brackets, 2, 5);
        assert_eq!(edges.len(), 0);
    }
}
