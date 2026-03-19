use serde::Serialize;

/// Format variant detected from file structure.
#[derive(Debug, Clone, Copy, Serialize, serde::Deserialize)]
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

/// A diagnostic message.
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub line_number: usize,
    pub column: Option<usize>,
    pub code: DiagnosticCode,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
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
