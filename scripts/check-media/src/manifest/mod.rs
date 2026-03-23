//! Media manifest: cached inventory of media files on the media server.
//!
//! The manifest is a JSON file listing all media files per bank. It is
//! refreshed explicitly via `check-media refresh-manifest` (SSH to the media
//! server) and used for offline validation by `check-media check`.

pub mod refresh;
pub mod show;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::config;

/// Root manifest structure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaManifest {
    /// When this manifest was generated.
    pub generated_at: DateTime<Utc>,
    /// SSH target used to generate this manifest.
    pub source_host: String,
    /// Root path on the media server.
    pub media_root: String,
    /// Per-bank file listings.
    pub banks: BTreeMap<String, BankMediaListing>,
}

/// All media files for one bank.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BankMediaListing {
    /// Number of media files in this bank.
    pub file_count: usize,
    /// Relative paths from bank root, sorted.
    pub files: Vec<String>,
}

/// Days before the manifest is considered stale.
const STALENESS_DAYS: i64 = 7;

impl MediaManifest {
    /// Load a manifest from a JSON file.
    pub fn load(path: &Path) -> Result<Self, ManifestError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ManifestError::Read { path: path.to_owned(), source: e })?;
        let manifest: Self = serde_json::from_str(&content)
            .map_err(|e| ManifestError::Parse { path: path.to_owned(), source: e })?;
        Ok(manifest)
    }

    /// Save the manifest to a JSON file. Creates parent directories as needed.
    pub fn save(&self, path: &Path) -> Result<(), ManifestError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ManifestError::Write { path: path.to_owned(), source: e })?;
        }
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| ManifestError::Serialize(e))?;
        std::fs::write(path, content)
            .map_err(|e| ManifestError::Write { path: path.to_owned(), source: e })?;
        Ok(())
    }

    /// Whether the manifest is older than the staleness threshold.
    pub fn is_stale(&self) -> bool {
        let age = Utc::now() - self.generated_at;
        age.num_days() >= STALENESS_DAYS
    }

    /// Total number of media files across all banks.
    pub fn total_files(&self) -> usize {
        self.banks.values().map(|b| b.file_count).sum()
    }

    /// Look up a media file by bank and relative path (case-insensitive).
    ///
    /// Returns the actual (case-preserved) path if found.
    pub fn find_media(&self, bank: &str, relative_path: &str) -> Option<&str> {
        let listing = self.banks.get(bank)?;
        let lower = relative_path.to_lowercase();
        listing.files.iter()
            .find(|f| f.to_lowercase() == lower)
            .map(String::as_str)
    }

    /// Check whether a media file exists for the given bank and path.
    /// Returns `(exists, case_matches)`.
    pub fn check_media(&self, bank: &str, relative_path: &str) -> (bool, bool) {
        match self.find_media(bank, relative_path) {
            Some(actual) => (true, actual == relative_path),
            None => (false, false),
        }
    }
}

impl BankMediaListing {
    /// Build a case-insensitive lookup map for this bank's files.
    #[allow(dead_code)] // Available for bulk lookups in future checks.
    pub fn case_insensitive_map(&self) -> BTreeMap<String, &str> {
        self.files.iter()
            .map(|f| (f.to_lowercase(), f.as_str()))
            .collect()
    }
}

/// Resolve the manifest path from an optional CLI argument.
pub fn resolve_manifest_path(explicit: &Option<PathBuf>) -> PathBuf {
    explicit.clone().unwrap_or_else(config::default_manifest_path)
}

/// Errors that can occur when working with manifests.
#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    #[error("failed to read manifest at {path}: {source}")]
    Read { path: PathBuf, source: std::io::Error },

    #[error("failed to parse manifest at {path}: {source}")]
    Parse { path: PathBuf, source: serde_json::Error },

    #[error("failed to write manifest to {path}: {source}")]
    Write { path: PathBuf, source: std::io::Error },

    #[error("failed to serialize manifest: {0}")]
    Serialize(serde_json::Error),

    #[error("SSH command failed: {0}")]
    Ssh(String),
}
