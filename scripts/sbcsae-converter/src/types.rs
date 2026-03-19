use serde::Serialize;
use std::collections::BTreeMap;

/// Format variant detected from file structure.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum FormatVariant {
    /// SBC001–013: space-separated timestamps, TAB, padded speaker, TAB, content.
    A,
    /// SBC015–060: TAB-separated timestamps, TAB, speaker, TAB, content.
    B,
    /// SBC014 only: space-separated timestamps+speaker, TAB, content.
    C,
}

/// A parsed TRN line.
#[derive(Debug, Clone, Serialize)]
pub struct TrnLine {
    /// 1-based line number in the source file.
    pub line_number: usize,
    pub start_time: f64,
    pub end_time: f64,
    /// None for continuation lines.
    pub speaker: Option<String>,
    /// The effective speaker (inherited from the most recent speaker line).
    pub effective_speaker: String,
    /// Content field, untouched.
    pub raw_content: String,
    /// 0-based absolute column offset where content begins in the original line.
    pub content_column: usize,
}

/// A bracket token found within a line's content.
#[derive(Debug, Clone, Serialize)]
pub struct BracketToken {
    pub line_number: usize,
    /// 0-based offset within raw_content.
    pub char_offset: usize,
    /// 0-based absolute column in the original line.
    pub column: usize,
    pub kind: BracketKind,
    /// None = unnumbered (logical index 0), Some(2..=9) = numbered.
    pub lexical_index: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BracketKind {
    /// `[` or `[N`
    Open,
    /// `]` or `N]`
    Close,
    /// `$]` or `N$]` (force bottom end)
    CloseForced,
}

/// Classification after overlap inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OverlapRole {
    TopBegin,
    TopEnd,
    BottomBegin,
    BottomEnd,
}

/// How the index appears in the source file.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayIndex {
    Unnumbered,
    Numbered(u8),
}

/// Location of a bracket in the source file.
#[derive(Debug, Clone, Serialize)]
pub struct BracketLocation {
    pub line_number: usize,
    /// Offset within the line's content field.
    pub char_offset: usize,
    /// Absolute column in the original line.
    pub column: usize,
    /// Timestamp range of the line this bracket appears on.
    pub time_range: (f64, f64),
}

/// One participant's bracket span within an overlap set.
#[derive(Debug, Clone, Serialize)]
pub struct OverlapParticipant {
    /// TRN speaker name (full, e.g., "JAMIE").
    pub speaker: String,
    pub begin: Option<BracketLocation>,
    pub end: Option<BracketLocation>,
    /// Text between `[` and `]` for this participant.
    pub bracketed_text: Option<String>,
}

/// An overlap set: one top speaker paired with one or more bottom speakers.
#[derive(Debug, Clone, Serialize)]
pub struct OverlapSetOutput {
    /// 0-based sequential index within the run.
    pub real_index: usize,
    pub display_index: DisplayIndex,
    pub top: OverlapParticipant,
    pub bottoms: Vec<OverlapParticipant>,
    /// True if all begins and ends are matched.
    pub complete: bool,
}

/// A complete overlap run (resets between non-overlapping stretches).
#[derive(Debug, Clone, Serialize)]
pub struct OverlapRunOutput {
    pub run_id: usize,
    pub sets: Vec<OverlapSetOutput>,
    pub first_line: usize,
    pub last_line: usize,
}

/// Per-file output.
#[derive(Debug, Clone, Serialize)]
pub struct FileOutput {
    pub filename: String,
    pub format_variant: FormatVariant,
    pub total_lines: usize,
    /// TRN full name → CHAT truncated ID.
    pub speaker_map: BTreeMap<String, String>,
    pub overlap_runs: Vec<OverlapRunOutput>,
    pub diagnostics: Vec<Diagnostic>,
}

/// A diagnostic message.
#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub line_number: usize,
    pub column: Option<usize>,
    pub code: DiagnosticCode,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DiagnosticCode {
    NulByte,
    Windows1252Char,
    MissingSpeakerColon,
    CaseInconsistentSpeaker,
    BracketIndexMismatch,
    IndexGap,
    UnmatchedBracket,
    TimestampAnomaly,
    ZeroTimestamp,
    EmptyContent,
    BrokenTabFormatting,
    MissingEnvPrefix,
    SameSpeakerOverlap,
    HighOverlapIndex,
    InvalidIndex,
    NoBottom,
    IncompleteTop,
    IncompleteBottom,
    SpeakerMapConflict,
}
