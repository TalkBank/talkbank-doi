//! Check orchestration: walk data repos, extract CHAT info, run checks,
//! collect diagnostics.

mod bullet;
mod chat_to_media;
mod corpus_name;
mod media_to_chat;
mod pic;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use walkdir::WalkDir;

use crate::cli::{CheckKind, OutputFormat};
use crate::config;
use crate::diagnostics::{Diagnostic, DiagnosticKind, Severity};
use crate::extract::{self, ChatFileInfo};
use crate::manifest::{self, MediaManifest};
use crate::output;

/// Entry point for the `check` subcommand.
///
/// Returns `Ok(true)` if errors were found and `--fail-on-error` was set,
/// `Ok(false)` otherwise.
pub fn run(
    paths: &[PathBuf],
    bank_filter: Option<&str>,
    manifest_path: &Option<PathBuf>,
    format: &OutputFormat,
    checks: &[CheckKind],
    fail_on_error: bool,
    quiet: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let manifest_file = manifest::resolve_manifest_path(manifest_path);
    let manifest = MediaManifest::load(&manifest_file).map_err(Box::new)?;

    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    // Warn about stale manifest.
    if manifest.is_stale() {
        diagnostics.push(Diagnostic::new(
            "",
            manifest_file.display().to_string(),
            DiagnosticKind::StaleManifest,
            format!(
                "manifest is {} days old — consider running `check-media refresh-manifest`",
                (chrono::Utc::now() - manifest.generated_at).num_days()
            ),
        ));
    }

    // Discover all .cha files and their repo context.
    let chat_files = discover_chat_files(paths, bank_filter)?;

    // Extract info from all CHAT files in parallel.
    let extracted: Vec<(String, String, ChatFileInfo)> = chat_files
        .into_par_iter()
        .filter_map(|(bank, repo_name, path)| {
            match extract::extract_chat_info(&path) {
                Ok(info) => Some((bank, repo_name, info)),
                Err(e) => {
                    eprintln!("warning: {e}");
                    None
                }
            }
        })
        .collect();

    let should_run = |kind: &CheckKind| -> bool {
        checks.is_empty() || checks.contains(kind)
    };

    // Run CHAT → media checks.
    if should_run(&CheckKind::MissingMedia)
        || should_run(&CheckKind::CaseMismatch)
        || should_run(&CheckKind::FilenameMatch)
    {
        for (bank, _repo, info) in &extracted {
            chat_to_media::check(bank, info, &manifest, &mut diagnostics);
        }
    }

    // Run bullet consistency checks.
    if should_run(&CheckKind::BulletConsistency) {
        for (bank, _repo, info) in &extracted {
            bullet::check(bank, info, &manifest, &mut diagnostics);
        }
    }

    // Run corpus name checks.
    if should_run(&CheckKind::CorpusName) {
        for (bank, _repo, info) in &extracted {
            corpus_name::check(bank, info, &mut diagnostics);
        }
    }

    // Run %pic checks.
    if should_run(&CheckKind::Pic) {
        for (bank, _repo, info) in &extracted {
            pic::check(bank, info, &mut diagnostics);
        }
    }

    // Run media → CHAT checks (reverse direction).
    if should_run(&CheckKind::MissingChat) {
        let mut chat_by_bank: BTreeMap<&str, Vec<String>> = BTreeMap::new();
        for (bank, _repo, info) in &extracted {
            if info.media.is_some() {
                let chat_stem = info.path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                let chat_dir = repo_relative_path(
                    info.path.parent().unwrap_or(info.path.as_path()),
                );
                if let Some(dir) = chat_dir {
                    let relative = if dir.is_empty() {
                        format!("{chat_stem}.cha")
                    } else {
                        format!("{dir}/{chat_stem}.cha")
                    };
                    chat_by_bank.entry(bank.as_str()).or_default().push(relative);
                }
            }
        }
        media_to_chat::check(&manifest, &chat_by_bank, bank_filter, &mut diagnostics);
    }

    // Output results.
    let has_errors = diagnostics.iter().any(|d| d.severity() == Severity::Error);

    match format {
        OutputFormat::Terminal => output::print_terminal(&diagnostics, &manifest, extracted.len(), quiet),
        OutputFormat::Json => output::print_json(&diagnostics, &manifest, extracted.len())?,
    }

    Ok(fail_on_error && has_errors)
}

/// Discover all `.cha` files under the given paths, with bank and repo context.
fn discover_chat_files(
    paths: &[PathBuf],
    bank_filter: Option<&str>,
) -> Result<Vec<(String, String, PathBuf)>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();

    for base in paths {
        let canonical = std::fs::canonicalize(base)
            .map_err(|e| format!("{}: {e}", base.display()))?;

        for entry in WalkDir::new(&canonical)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| e.file_name() != ".git")
        {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("cha")) {
                if let Some((bank, repo_name, _repo_path)) = config::detect_repo_from_path(path) {
                    if let Some(filter) = bank_filter {
                        if bank != filter {
                            continue;
                        }
                    }
                    results.push((bank.to_string(), repo_name, path.to_owned()));
                }
            }
        }
    }

    Ok(results)
}

/// Extract the repo-relative path from an absolute path.
/// Looks for a `*-data` component and returns everything after it.
pub fn repo_relative_path(path: &Path) -> Option<String> {
    let mut components = Vec::new();
    let mut found_repo = false;

    for component in path.components() {
        if found_repo {
            if let Some(s) = component.as_os_str().to_str() {
                components.push(s.to_string());
            }
        } else if let Some(s) = component.as_os_str().to_str() {
            if s.ends_with("-data") {
                found_repo = true;
            }
        }
    }

    if found_repo {
        Some(components.join("/"))
    } else {
        None
    }
}
