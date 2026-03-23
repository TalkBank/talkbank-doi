//! CLI argument definitions for check-media.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

/// Validate CHAT transcript ↔ media file correspondence.
///
/// Replaces the legacy `chatmedia.py` tool with a cached-manifest approach
/// suitable for use as a pre-push hook.
#[derive(Parser)]
#[command(name = "check-media", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validate CHAT ↔ media correspondence (read-only).
    Check {
        /// Data repo paths or directories to check.
        #[arg(required = true)]
        paths: Vec<PathBuf>,

        /// Restrict to a specific bank.
        #[arg(long)]
        bank: Option<String>,

        /// Path to cached media manifest.
        #[arg(long, env = "CHECK_MEDIA_MANIFEST")]
        manifest: Option<PathBuf>,

        /// Output format.
        #[arg(long, default_value = "terminal")]
        format: OutputFormat,

        /// Which checks to run (default: all).
        #[arg(long = "check", value_delimiter = ',')]
        checks: Vec<CheckKind>,

        /// Exit with non-zero status if any issues found.
        #[arg(long)]
        fail_on_error: bool,

        /// Only show errors, not summary.
        #[arg(long)]
        quiet: bool,
    },

    /// Refresh the cached media manifest from the media server.
    RefreshManifest {
        /// SSH target for the media server.
        #[arg(long, env = "MEDIA_HOST", default_value = "macw@net")]
        host: String,

        /// Root path on the media server.
        #[arg(long, env = "MEDIA_ROOT", default_value = "/Users/macw/media")]
        media_root: String,

        /// Only refresh a specific bank.
        #[arg(long)]
        bank: Option<String>,

        /// Output path for the manifest file.
        #[arg(long)]
        output: Option<PathBuf>,
    },

    /// Display manifest contents or statistics.
    ShowManifest {
        /// Path to the manifest file.
        #[arg(long, env = "CHECK_MEDIA_MANIFEST")]
        manifest: Option<PathBuf>,

        /// Show individual file paths (verbose).
        #[arg(long)]
        files: bool,

        /// Restrict to a specific bank.
        #[arg(long)]
        bank: Option<String>,
    },

    /// Apply mutations to CHAT files (no git operations).
    Fix {
        #[command(subcommand)]
        mutation: FixMutation,
    },
}

#[derive(Subcommand)]
pub enum FixMutation {
    /// Add "unlinked" status to @Media headers where media exists but no bullets.
    AddUnlinked {
        /// Data repo paths or directories to fix.
        #[arg(required = true)]
        paths: Vec<PathBuf>,

        /// Path to cached media manifest.
        #[arg(long, env = "CHECK_MEDIA_MANIFEST")]
        manifest: Option<PathBuf>,

        /// Show what would change without writing.
        #[arg(long)]
        dry_run: bool,
    },

    /// Rewrite @ID corpus fields to match directory structure.
    FixCorpus {
        /// Data repo paths or directories to fix.
        #[arg(required = true)]
        paths: Vec<PathBuf>,

        /// Show what would change without writing.
        #[arg(long)]
        dry_run: bool,
    },
}

/// Output format for the check subcommand.
#[derive(Clone, Debug, Default, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable terminal output.
    #[default]
    Terminal,
    /// Machine-readable JSON output.
    Json,
}

/// Individual check kinds that can be selected.
#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum CheckKind {
    /// CHAT references media that does not exist.
    MissingMedia,
    /// Media exists but no corresponding CHAT file.
    MissingChat,
    /// Media or CHAT filename case mismatch.
    CaseMismatch,
    /// @Media filename does not match CHAT file basename.
    FilenameMatch,
    /// Bullet presence inconsistent with @Media status.
    BulletConsistency,
    /// @ID corpus field does not match directory structure.
    CorpusName,
    /// %pic references nonexistent file.
    Pic,
}
