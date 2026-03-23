//! Refresh the media manifest by SSHing to the media server.
//!
//! Runs a single `find -L <media_root> -type f -print` over SSH, filters
//! excluded extensions/directories, and builds a [`MediaManifest`].

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;

use chrono::Utc;

use super::{BankMediaListing, ManifestError, MediaManifest, resolve_manifest_path};
use crate::config;

/// Entry point for the `refresh-manifest` subcommand.
pub fn run(
    host: &str,
    media_root: &str,
    bank_filter: Option<&str>,
    output: &Option<PathBuf>,
) -> Result<bool, Box<dyn std::error::Error>> {
    eprintln!("Refreshing manifest from {host}:{media_root} ...");

    let raw_paths = fetch_file_list(host, media_root)?;
    eprintln!("  fetched {} raw paths", raw_paths.len());

    let manifest = build_manifest(host, media_root, &raw_paths, bank_filter);

    let out_path = resolve_manifest_path(output);
    manifest.save(&out_path).map_err(Box::new)?;
    eprintln!("  manifest written to {}", out_path.display());

    // Print summary.
    eprintln!();
    eprintln!("Summary ({} banks, {} total files):", manifest.banks.len(), manifest.total_files());
    for (bank, listing) in &manifest.banks {
        eprintln!("  {bank:20} {}", listing.file_count);
    }

    Ok(false)
}

/// SSH to the media server and list all files.
fn fetch_file_list(host: &str, media_root: &str) -> Result<Vec<String>, ManifestError> {
    let output = Command::new("ssh")
        .arg(host)
        .arg(format!("find -L {media_root} -type f -print"))
        .output()
        .map_err(|e| ManifestError::Ssh(format!("failed to spawn ssh: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ManifestError::Ssh(format!(
            "ssh exited with {}: {stderr}",
            output.status
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(String::from).collect())
}

/// Build a manifest from raw absolute paths.
fn build_manifest(
    host: &str,
    media_root: &str,
    raw_paths: &[String],
    bank_filter: Option<&str>,
) -> MediaManifest {
    let prefix = if media_root.ends_with('/') {
        media_root.to_string()
    } else {
        format!("{media_root}/")
    };

    let mut banks: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for path in raw_paths {
        let Some(relative) = path.strip_prefix(&prefix) else {
            continue;
        };

        // Split into bank / rest.
        let Some((bank, rest)) = relative.split_once('/') else {
            continue;
        };

        // Filter to specific bank if requested.
        if let Some(filter) = bank_filter {
            if bank != filter {
                continue;
            }
        }

        // Skip unknown banks.
        if !config::BANKS.contains(&bank) {
            continue;
        }

        // Skip excluded top-level paths.
        if config::EXCLUDED_TOP_PATHS.iter().any(|exc| relative.starts_with(exc)) {
            continue;
        }

        // Skip excluded directory components.
        if path_contains_excluded_dir(rest) {
            continue;
        }

        // Skip excluded extensions.
        if has_excluded_extension(rest) {
            continue;
        }

        banks.entry(bank.to_string()).or_default().push(rest.to_string());
    }

    // Sort file lists and build final structure.
    let banks = banks.into_iter().map(|(bank, mut files)| {
        files.sort();
        let file_count = files.len();
        (bank, BankMediaListing { file_count, files })
    }).collect();

    MediaManifest {
        generated_at: Utc::now(),
        source_host: host.to_string(),
        media_root: media_root.to_string(),
        banks,
    }
}

/// Whether any path component is an excluded directory name.
fn path_contains_excluded_dir(path: &str) -> bool {
    for component in path.split('/') {
        if config::EXCLUDED_DIRS.contains(&component) {
            return true;
        }
    }
    false
}

/// Whether the file has an excluded extension.
fn has_excluded_extension(path: &str) -> bool {
    let Some(dot_pos) = path.rfind('.') else {
        return false;
    };
    let ext = &path[dot_pos + 1..];
    config::EXCLUDED_EXTENSIONS.contains(&ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excluded_extension() {
        assert!(has_excluded_extension("foo/bar.wav"));
        assert!(has_excluded_extension("test.DS_Store"));
        assert!(!has_excluded_extension("foo/bar.mp3"));
        assert!(!has_excluded_extension("foo/bar.mp4"));
    }

    #[test]
    fn test_excluded_dir() {
        assert!(path_contains_excluded_dir("Eng-NA/0video/something.mp4"));
        assert!(path_contains_excluded_dir("0textgrid/foo.TextGrid"));
        assert!(!path_contains_excluded_dir("Eng-NA/MacWhinney/010600a.mp3"));
    }

    #[test]
    fn test_build_manifest() {
        let paths = vec![
            "/Users/macw/media/aphasia/English/test.mp3".to_string(),
            "/Users/macw/media/aphasia/English/test.wav".to_string(),
            "/Users/macw/media/childes/Eng-NA/Brown/adam01.mp3".to_string(),
            "/Users/macw/media/unknown_bank/foo.mp3".to_string(),
        ];
        let manifest = build_manifest("macw@net", "/Users/macw/media", &paths, None);

        assert_eq!(manifest.banks.len(), 2);
        assert_eq!(manifest.banks["aphasia"].file_count, 1); // .wav excluded
        assert_eq!(manifest.banks["childes"].file_count, 1);
    }
}
