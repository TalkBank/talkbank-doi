//! Diagnostic types for check-media validation results.
//!
//! Each diagnostic represents a single issue found during checking.
//! Diagnostics carry a kind (what went wrong), severity, location, and
//! human-readable message.

use serde::Serialize;

/// Severity of a diagnostic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Must be fixed.
    Error,
    /// Should be reviewed.
    Warning,
}

/// All diagnostic kinds produced by check-media.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticKind {
    // ── Media existence ──
    /// CHAT references media not present in the manifest (and not marked `missing`).
    MissingMedia,
    /// Media file in manifest has no corresponding CHAT file.
    MissingChat,
    /// @Media says `missing` but the file exists in the manifest.
    MarkedMissingButExists,
    /// @Media says `unlinked`/`notrans` but no media exists in the manifest.
    MarkedStatusButNoMedia,

    // ── Case and naming ──
    /// Media file exists but with different case than CHAT expects.
    MediaCaseMismatch,
    /// CHAT file exists but with different case than derived from media.
    #[allow(dead_code)] // Will be used when media_to_chat detects case mismatches.
    ChatCaseMismatch,
    /// @Media filename does not match the CHAT file's basename.
    FilenameMismatch,

    // ── Bullet consistency ──
    /// Has timing bullets but @Media is marked unlinked.
    BulletsButMarkedUnlinked,
    /// Has timing bullets but @Media is marked notrans.
    BulletsButMarkedNotrans,
    /// Media exists, no bullets, and not marked unlinked.
    NoBulletsNeedUnlinked,
    /// No media found, no bullets, and not marked missing.
    NoBulletsMediaMissing,

    // ── Corpus name ──
    /// @ID corpus field does not match the directory-derived corpus name.
    CorpusNameMismatch,
    /// No `0metadata.cdc` found in any ancestor directory.
    NoCorpusDetected,

    // ── %pic ──
    /// %pic reference points to a file that does not exist.
    MissingPic,

    // ── Manifest ──
    /// The manifest is older than the staleness threshold.
    StaleManifest,
}

impl DiagnosticKind {
    /// Default severity for this diagnostic kind.
    pub fn severity(&self) -> Severity {
        match self {
            Self::MarkedMissingButExists
            | Self::MarkedStatusButNoMedia
            | Self::MediaCaseMismatch
            | Self::ChatCaseMismatch
            | Self::NoCorpusDetected
            | Self::StaleManifest => Severity::Warning,

            Self::MissingMedia
            | Self::MissingChat
            | Self::FilenameMismatch
            | Self::BulletsButMarkedUnlinked
            | Self::BulletsButMarkedNotrans
            | Self::NoBulletsNeedUnlinked
            | Self::NoBulletsMediaMissing
            | Self::CorpusNameMismatch
            | Self::MissingPic => Severity::Error,
        }
    }

    /// Short tag for terminal output (e.g., `"missing-media"`).
    pub fn tag(&self) -> &'static str {
        match self {
            Self::MissingMedia => "missing-media",
            Self::MissingChat => "missing-chat",
            Self::MarkedMissingButExists => "marked-missing-exists",
            Self::MarkedStatusButNoMedia => "status-no-media",
            Self::MediaCaseMismatch => "media-case",
            Self::ChatCaseMismatch => "chat-case",
            Self::FilenameMismatch => "filename-mismatch",
            Self::BulletsButMarkedUnlinked => "bullets-unlinked",
            Self::BulletsButMarkedNotrans => "bullets-notrans",
            Self::NoBulletsNeedUnlinked => "no-bullets-unlinked",
            Self::NoBulletsMediaMissing => "no-bullets-missing",
            Self::CorpusNameMismatch => "corpus-mismatch",
            Self::NoCorpusDetected => "no-corpus",
            Self::MissingPic => "missing-pic",
            Self::StaleManifest => "stale-manifest",
        }
    }
}

/// A single diagnostic produced by a check.
#[derive(Clone, Debug, Serialize)]
pub struct Diagnostic {
    /// Which bank this relates to.
    pub bank: String,
    /// Path relative to the repo root (or manifest source).
    pub path: String,
    /// What went wrong.
    pub kind: DiagnosticKind,
    /// Human-readable explanation.
    pub message: String,
}

impl Diagnostic {
    /// Convenience constructor.
    pub fn new(
        bank: impl Into<String>,
        path: impl Into<String>,
        kind: DiagnosticKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            bank: bank.into(),
            path: path.into(),
            kind,
            message: message.into(),
        }
    }

    /// Severity derived from the diagnostic kind.
    pub fn severity(&self) -> Severity {
        self.kind.severity()
    }
}
