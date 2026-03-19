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

// ── Document Builder ────────────────────────────────────────────────────────

use std::path::Path;

use crate::bracket::tokenize_brackets;
use crate::diagnostics::Diagnostics;
use crate::emit_chat::time_to_ms;
use crate::format::{detect_format, parse_lines};
use crate::speakers::build_speaker_map;
use crate::trn_content::{parse_trn_content, TrnElement};
use crate::types::BracketKind;

/// Build a TrnDocument from a TRN file. This is stages 1–7 of the pipeline:
/// read, decode, parse lines, tokenize brackets, parse content, map speakers,
/// group utterances, compute alignment edges. NO overlap inference.
pub fn build_document(path: &Path, diag: &mut Diagnostics) -> Option<TrnDocument> {
    let filename = path.file_name()?.to_str()?.to_string();

    // Stage 1: Read and decode.
    let text = match crate::encoding::read_and_decode(path, diag) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading {}: {e}", path.display());
            return None;
        }
    };

    // Stage 2: Detect format and parse lines.
    let first_line = text.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    let variant = detect_format(first_line);
    let lines = parse_lines(&text, variant, diag);
    let total_lines = lines.len();

    // Stage 3: Tokenize brackets (positions only, no role inference).
    // Collect all bracket tokens per line for content parsing.
    let mut all_bracket_tokens = Vec::new();
    for line in &lines {
        let tokens = tokenize_brackets(&line.raw_content, line.line_number, line.content_column);
        all_bracket_tokens.push(tokens);
    }

    // Stage 4: Parse TRN content into elements.
    // Pass bracket positions (char offsets only) so the content parser can
    // replace them with placeholders. No role information at this stage.
    let line_elements: Vec<Vec<TrnElement>> = lines
        .iter()
        .zip(all_bracket_tokens.iter())
        .map(|(line, tokens)| {
            // Build position-only bracket list for content parser.
            let positions: Vec<(usize, bool)> = tokens
                .iter()
                .map(|t| (t.char_offset, t.kind == crate::types::BracketKind::Open))
                .collect();
            parse_trn_content(&line.raw_content, &positions)
        })
        .collect();

    // Stage 5: Build speaker map.
    let mut seen = std::collections::HashSet::new();
    let speakers: Vec<String> = lines
        .iter()
        .filter_map(|l| l.speaker.clone())
        .filter(|s| seen.insert(s.clone()))
        .collect();
    let speaker_map = build_speaker_map(&speakers, diag);

    // Stage 6: Group into utterances and assign bracket IDs.
    let mut utterances = Vec::new();
    let mut all_brackets = Vec::new();
    let mut bracket_id_counter: u32 = 0;

    let mut current_elements: Vec<ContentElement> = Vec::new();
    let mut current_speaker: Option<String> = None;
    let mut current_start_ms: Option<i64> = None;
    let mut current_end_ms: Option<i64> = None;
    let mut current_first_line: usize = 1;
    let mut last_terminator: Option<RawTerminator> = None;

    for (line_idx, (line, (trn_elements, bracket_tokens))) in lines
        .iter()
        .zip(line_elements.iter().zip(all_bracket_tokens.iter()))
        .enumerate()
    {
        // Skip zero-timestamp annotator comment lines.
        if line.start_time == 0.0 && line.end_time == 0.0
            && line.raw_content.trim_start().starts_with('$')
        {
            continue;
        }

        let is_new_speaker = line.speaker.is_some()
            && line.speaker.as_deref() != current_speaker.as_deref();

        if is_new_speaker {
            // Flush previous utterance.
            if !current_elements.is_empty() {
                // Compute gap: current utterance end → next line's start.
                let gap = line.start_time - current_end_ms.unwrap_or(0) as f64 / 1000.0;
                let term = resolve_terminator(
                    last_terminator.take().or_else(|| extract_terminator_from_elements(&mut current_elements)),
                    gap,
                );
                let utt_idx = utterances.len();
                for elem in &current_elements {
                    if let ContentElement::Bracket(bid) = elem {
                        if let Some(br) = all_brackets.iter_mut().find(|b: &&mut BracketRef| b.id == *bid) {
                            br.utterance_index = utt_idx;
                        }
                    }
                }
                utterances.push(TrnUtterance {
                    index: utt_idx,
                    speaker: current_speaker.clone().unwrap_or_default(),
                    elements: std::mem::take(&mut current_elements),
                    terminator: term,
                    start_ms: current_start_ms,
                    end_ms: current_end_ms,
                    source_lines: LineRange {
                        first: current_first_line,
                        last: line.line_number.saturating_sub(1).max(current_first_line),
                    },
                });
            }
            current_speaker = line.speaker.clone();
            current_start_ms = Some(time_to_ms(line.start_time));
            current_end_ms = Some(time_to_ms(line.end_time));
            current_first_line = line.line_number;
        } else {
            // Continuation — check if previous line's TRN elements had a terminator.
            // We look at the previous line's elements (not current_elements, which
            // has already absorbed them as ContentElements minus terminators).
            let prev_line_idx = if line_idx > 0 { line_idx - 1 } else { 0 };
            let has_trn_terminator = line_elements.get(prev_line_idx)
                .map_or(false, |elems| elems.iter().any(|e| matches!(
                    e,
                    TrnElement::Period | TrnElement::QuestionMark | TrnElement::Truncation
                )));

            if has_trn_terminator && !current_elements.is_empty() {
                // Flush at terminator boundary.
                let term = resolve_terminator(last_terminator.take().or_else(|| extract_terminator_from_elements(&mut current_elements)), line.start_time - current_end_ms.unwrap_or(0) as f64 / 1000.0);
                let utt_idx = utterances.len();
                for elem in &current_elements {
                    if let ContentElement::Bracket(bid) = elem {
                        if let Some(br) = all_brackets.iter_mut().find(|b: &&mut BracketRef| b.id == *bid) {
                            br.utterance_index = utt_idx;
                        }
                    }
                }
                utterances.push(TrnUtterance {
                    index: utt_idx,
                    speaker: current_speaker.clone().unwrap_or_default(),
                    elements: std::mem::take(&mut current_elements),
                    terminator: term,
                    start_ms: current_start_ms,
                    end_ms: current_end_ms,
                    source_lines: LineRange {
                        first: current_first_line,
                        last: line.line_number,
                    },
                });
                current_start_ms = Some(time_to_ms(line.start_time));
                current_first_line = line.line_number;
            }

            current_end_ms = Some(time_to_ms(line.end_time));
        }

        // Convert TrnElements to ContentElements, creating BracketRefs.
        let mut bracket_token_idx = 0;
        for elem in trn_elements {
            match elem {
                TrnElement::Bracket { .. } => {
                    // This is a bracket placeholder — create a BracketRef.
                    let bt = if bracket_token_idx < bracket_tokens.len() {
                        &bracket_tokens[bracket_token_idx]
                    } else {
                        bracket_token_idx += 1;
                        continue;
                    };
                    bracket_token_idx += 1;

                    let direction = match bt.kind {
                        BracketKind::Open => BracketDirection::Open,
                        BracketKind::Close => BracketDirection::Close,
                        BracketKind::CloseForced => BracketDirection::CloseForced,
                    };

                    let bracket_ref = BracketRef {
                        id: bracket_id_counter,
                        direction,
                        lexical_index: bt.lexical_index,
                        utterance_index: 0, // Will be fixed up when utterance is flushed.
                        element_position: current_elements.len(),
                        speaker: line.effective_speaker.clone(),
                        source: BracketSource {
                            line_number: bt.line_number,
                            char_offset: bt.char_offset,
                            column: bt.column,
                            time_range: (line.start_time, line.end_time),
                        },
                    };

                    current_elements.push(ContentElement::Bracket(bracket_id_counter));
                    all_brackets.push(bracket_ref);
                    bracket_id_counter += 1;
                }
                TrnElement::Word(w) => {
                    current_elements.push(ContentElement::Word(w.clone()));
                }
                TrnElement::PauseShort => current_elements.push(ContentElement::PauseShort),
                TrnElement::PauseMedium => current_elements.push(ContentElement::PauseMedium),
                TrnElement::PauseTimed(v) => current_elements.push(ContentElement::PauseTimed(v.clone())),
                TrnElement::Inhalation => current_elements.push(ContentElement::Inhalation),
                TrnElement::InhalationLengthened => current_elements.push(ContentElement::InhalationLengthened),
                TrnElement::Exhalation => current_elements.push(ContentElement::Exhalation),
                TrnElement::Click => current_elements.push(ContentElement::Click),
                TrnElement::Vocalism(n) => current_elements.push(ContentElement::Vocalism(n.clone())),
                TrnElement::Laugh => current_elements.push(ContentElement::Vocalism("laugh".to_string())),
                TrnElement::Laughs(n) => current_elements.push(ContentElement::Laughs(*n)),
                TrnElement::Comment(n) => current_elements.push(ContentElement::Vocalism(n.clone())),
                TrnElement::LongFeatureBegin(l) => current_elements.push(ContentElement::LongFeatureBegin(l.clone())),
                TrnElement::LongFeatureEnd(l) => current_elements.push(ContentElement::LongFeatureEnd(l.clone())),
                TrnElement::NonvocalBegin(l) => current_elements.push(ContentElement::NonvocalBegin(l.clone())),
                TrnElement::NonvocalEnd(l) => current_elements.push(ContentElement::NonvocalEnd(l.clone())),
                TrnElement::NonvocalSimple(l) => current_elements.push(ContentElement::NonvocalSimple(l.clone())),
                TrnElement::NonvocalBeat => current_elements.push(ContentElement::NonvocalBeat),
                TrnElement::PhonologicalFragment(f) => current_elements.push(ContentElement::PhonologicalFragment(f.clone())),
                TrnElement::Glottal => current_elements.push(ContentElement::Glottal),
                TrnElement::Comma => current_elements.push(ContentElement::Comma),
                // Structural elements: record terminator type, don't add to content.
                // Truncation (--) is stored as a placeholder — the actual CHAT
                // terminator (+/. vs +...) depends on whether the next speaker
                // is the same or different, resolved at flush time.
                TrnElement::Period => { last_terminator = Some(RawTerminator::Period); }
                TrnElement::QuestionMark => { last_terminator = Some(RawTerminator::Question); }
                TrnElement::Truncation => { last_terminator = Some(RawTerminator::Truncation); }
                TrnElement::Linker => { last_terminator = Some(RawTerminator::Linker); }
                TrnElement::Space => {}
            }
        }
    }

    // Flush final utterance — no next speaker, large gap → trail off if truncated.
    if !current_elements.is_empty() {
        let term = resolve_terminator(
            last_terminator.take().or_else(|| extract_terminator_from_elements(&mut current_elements)),
            f64::MAX,
        );
        let utt_idx = utterances.len();
        for elem in &current_elements {
            if let ContentElement::Bracket(bid) = elem {
                if let Some(br) = all_brackets.iter_mut().find(|b: &&mut BracketRef| b.id == *bid) {
                    br.utterance_index = utt_idx;
                }
            }
        }
        utterances.push(TrnUtterance {
            index: utt_idx,
            speaker: current_speaker.unwrap_or_default(),
            elements: current_elements,
            terminator: term,
            start_ms: current_start_ms,
            end_ms: current_end_ms,
            source_lines: LineRange {
                first: current_first_line,
                last: lines.last().map_or(1, |l| l.line_number),
            },
        });
    }

    // Stage 7: Compute alignment edges.
    let alignment_edges = compute_alignment_edges(&all_brackets, 2, 5);

    Some(TrnDocument {
        filename,
        format_variant: variant,
        total_lines,
        speaker_map,
        speakers,
        utterances,
        brackets: all_brackets,
        alignment_edges,
        parse_diagnostics: diag.drain(),
    })
}

/// Raw terminator from TRN — resolved to CHAT terminator at flush time.
#[derive(Debug, Clone, Copy)]
enum RawTerminator {
    Period,
    Question,
    Truncation, // -- : becomes +/. or +... depending on next speaker
    Linker,     // & : becomes +,
}

/// Resolve a raw TRN terminator to a CHAT terminator.
///
/// For truncation (`--`), uses the temporal gap to the next speaker's start:
/// - gap < 0.5s → `+/.` (interrupted: next speaker already started)
/// - gap ≥ 0.5s → `+...` (trail off: significant pause before next speaker)
fn resolve_terminator(raw: Option<RawTerminator>, gap_to_next_seconds: f64) -> Terminator {
    match raw {
        Some(RawTerminator::Period) => Terminator::Period,
        Some(RawTerminator::Question) => Terminator::Question,
        Some(RawTerminator::Truncation) => {
            // Temporal analysis: did the next speaker start before this one stopped?
            // gap < 0: next speaker overlaps → interrupted (+/.)
            // gap >= 0: this speaker stopped first → trail off (+...)
            //
            // Note: Brian's hand-edited CHAT uses a roughly 50/50 split between
            // +/. and +... that doesn't purely follow timing. His decisions were
            // based on listening to the audio. Our temporal approximation skews
            // toward +... because many TRN timestamps are exactly aligned (gap=0).
            // This is documented in TRN-TO-CHAT-TRANSLATION.md.
            if gap_to_next_seconds < 0.0 {
                Terminator::Interruption  // +/.
            } else {
                Terminator::TrailOff  // +...
            }
        }
        Some(RawTerminator::Linker) => Terminator::SelfCompletion,
        None => Terminator::Implicit,
    }
}

/// Extract a terminator from the end of a ContentElement list.
/// Looks for trailing Comma (→ Period) or returns None.
fn extract_terminator_from_elements(elements: &mut Vec<ContentElement>) -> Option<RawTerminator> {
    if let Some(ContentElement::Comma) = elements.last() {
        elements.pop();
        return Some(RawTerminator::Period);
    }
    None
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
