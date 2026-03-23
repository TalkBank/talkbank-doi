//! Bank-to-repo mapping and media server configuration.
//!
//! Mirrors the authoritative mapping in `staging/scripts/config.py`.
//! The media server uses the original 16 bank names as top-level directories,
//! while data repos use the post-split 24-repo names.

use std::collections::BTreeMap;
use std::path::Path;

/// The 16 original TalkBank bank names (media server directory names).
pub const BANKS: &[&str] = &[
    "aphasia", "asd", "biling", "ca", "childes", "class", "dementia", "fluency",
    "homebank", "motor", "phon", "psychosis", "rhd", "samtale", "slabank", "tbi",
];

/// Static mapping from bank name to data repo names (without `-data` suffix).
///
/// Mirrors `staging/scripts/config.py:BANK_TO_DATA_REPOS`.
#[allow(dead_code)] // Available for future callers (e.g., media_to_chat expansion).
pub fn bank_to_repos() -> BTreeMap<&'static str, Vec<&'static str>> {
    BTreeMap::from([
        ("aphasia", vec!["aphasia"]),
        ("asd", vec!["asd"]),
        ("biling", vec!["biling"]),
        ("ca", vec!["ca-candor", "ca"]),
        ("childes", vec![
            "childes-eng-na", "childes-eng-uk",
            "childes-romance-germanic", "childes-other",
        ]),
        ("class", vec!["class"]),
        ("dementia", vec!["dementia"]),
        ("fluency", vec!["fluency"]),
        ("homebank", vec![
            "homebank-public", "homebank-cougar",
            "homebank-bergelson", "homebank-password",
        ]),
        ("motor", vec!["motor"]),
        ("phon", vec!["phon-eng-french", "phon-other"]),
        ("psychosis", vec!["psychosis"]),
        ("rhd", vec!["rhd"]),
        ("samtale", vec!["samtale"]),
        ("slabank", vec!["slabank"]),
        ("tbi", vec!["tbi"]),
    ])
}

/// Static mapping from split-repo name (without `-data`) to parent bank name.
///
/// Unsplit repos are not in this table — they map to themselves.
/// Mirrors `staging/scripts/config.py:DATA_REPO_TO_BANK`.
fn split_repo_to_bank() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("childes-eng-na", "childes"),
        ("childes-eng-uk", "childes"),
        ("childes-romance-germanic", "childes"),
        ("childes-other", "childes"),
        ("ca-candor", "ca"),
        ("phon-eng-french", "phon"),
        ("phon-other", "phon"),
        ("homebank-public", "homebank"),
        ("homebank-cougar", "homebank"),
        ("homebank-bergelson", "homebank"),
        ("homebank-password", "homebank"),
    ])
}

/// Derive the bank name from a data repo directory name.
///
/// Strips the `-data` suffix, then checks the split-repo table.
/// Unsplit repos (e.g., `aphasia-data`) map to themselves (`aphasia`).
///
/// Returns `None` if the name doesn't end in `-data` or isn't a known repo.
pub fn repo_to_bank(repo_dir_name: &str) -> Option<&'static str> {
    let stem = repo_dir_name.strip_suffix("-data")?;
    // Check split-repo table first, then fall back to identity.
    if let Some(&bank) = split_repo_to_bank().get(stem) {
        return Some(bank);
    }
    // Unsplit: the stem IS the bank name if it's in BANKS.
    if BANKS.contains(&stem) {
        return Some(BANKS.iter().find(|&&b| b == stem).copied()?);
    }
    None
}

/// Walk up from `path` looking for a directory whose name ends in `-data`
/// and is a known repo. Returns `(bank_name, repo_dir_name, repo_path)`.
pub fn detect_repo_from_path(path: &Path) -> Option<(&'static str, String, &Path)> {
    let mut current = path;
    loop {
        if let Some(name) = current.file_name().and_then(|n| n.to_str()) {
            if name.ends_with("-data") {
                if let Some(bank) = repo_to_bank(name) {
                    return Some((bank, name.to_string(), current));
                }
            }
        }
        current = current.parent()?;
    }
}

/// Name of the metadata file that marks a corpus root directory.
pub const METADATA_FILE: &str = "0metadata.cdc";

/// Walk up from `path` to find the nearest ancestor containing `0metadata.cdc`.
/// Returns the directory name (corpus name) if found.
pub fn detect_corpus(path: &Path) -> Option<String> {
    let mut dir = if path.is_file() {
        path.parent()?
    } else {
        path
    };
    loop {
        if dir.join(METADATA_FILE).is_file() {
            return dir.file_name().and_then(|n| n.to_str()).map(String::from);
        }
        dir = dir.parent()?;
    }
}

/// Default manifest path: `~/.cache/talkbank/media-manifest.json`.
pub fn default_manifest_path() -> std::path::PathBuf {
    let cache = dirs_cache();
    cache.join("media-manifest.json")
}

/// Platform-appropriate cache directory for talkbank tools.
fn dirs_cache() -> std::path::PathBuf {
    if cfg!(target_os = "macos") {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        std::path::PathBuf::from(home).join("Library/Caches/talkbank")
    } else if cfg!(target_os = "linux") {
        let base = std::env::var("XDG_CACHE_HOME")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
                format!("{home}/.cache")
            });
        std::path::PathBuf::from(base).join("talkbank")
    } else {
        // Windows fallback.
        let base = std::env::var("LOCALAPPDATA")
            .unwrap_or_else(|_| std::env::var("HOME").unwrap_or_else(|_| "/tmp".into()));
        std::path::PathBuf::from(base).join("talkbank")
    }
}

/// Extensions to exclude from the media manifest (not real media).
pub const EXCLUDED_EXTENSIONS: &[&str] = &[
    "DS_Store", "aif", "mov", "wav", "zip", "xlsx", "xls", "csv",
    "TextGrid", "arff", "cha", "doc", "docx", "inc", "jar", "pdf",
    "pl", "png", "properties", "sh", "tsv", "txt", "upl",
];

/// Relative directory names to exclude from media scanning.
pub const EXCLUDED_DIRS: &[&str] = &[
    "0video", "0textgrid", "0extra", "0wav", "0webM",
];

/// Top-level paths under the media root to exclude entirely.
pub const EXCLUDED_TOP_PATHS: &[&str] = &[
    "aphasia/Class",
    "rhd/class",
    "tbi/class",
    "homebank/Password/Challenge",
    "homebank/Password/IDSLabel",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsplit_repo_to_bank() {
        assert_eq!(repo_to_bank("aphasia-data"), Some("aphasia"));
        assert_eq!(repo_to_bank("tbi-data"), Some("tbi"));
    }

    #[test]
    fn split_repo_to_bank_mapping() {
        assert_eq!(repo_to_bank("childes-eng-na-data"), Some("childes"));
        assert_eq!(repo_to_bank("homebank-cougar-data"), Some("homebank"));
        assert_eq!(repo_to_bank("ca-candor-data"), Some("ca"));
        assert_eq!(repo_to_bank("phon-other-data"), Some("phon"));
    }

    #[test]
    fn unknown_repo() {
        assert_eq!(repo_to_bank("unknown-data"), None);
        assert_eq!(repo_to_bank("not-a-repo"), None);
    }
}
