//! Rewrite @ID corpus fields to match directory structure.

use std::path::PathBuf;
use std::sync::LazyLock;

use regex::Regex;
use walkdir::WalkDir;

use crate::config;
use crate::extract;

/// Regex capturing @ID lines: `@ID:\t<lang>|<corpus>|...`
///
/// Group 1: prefix through first pipe (`@ID:\tlang|`)
/// Group 2: corpus name
static ID_CORPUS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^(@ID:\t[^\|]*\|)([^\|]*)").expect("id corpus regex is valid")
});

/// Run the fix-corpus mutation.
pub fn run(
    paths: &[PathBuf],
    dry_run: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut fixed_count = 0;

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
            if !path.extension().is_some_and(|e| e.eq_ignore_ascii_case("cha")) {
                continue;
            }

            if config::detect_repo_from_path(path).is_none() {
                continue;
            }

            let info = extract::extract_chat_info(path)?;
            if info.id_corpora.is_empty() {
                continue;
            }

            let Some(expected_corpus) = config::detect_corpus(path) else {
                continue;
            };

            let needs_fix = info.id_corpora.iter().any(|c| c != &expected_corpus);
            if !needs_fix {
                continue;
            }

            if dry_run {
                println!(
                    "would fix corpus in {} ({} -> {})",
                    path.display(),
                    info.id_corpora.first().unwrap_or(&String::new()),
                    expected_corpus
                );
            } else {
                let content = std::fs::read_to_string(path)?;
                let new_content = ID_CORPUS_RE.replace_all(
                    &content,
                    format!("${{1}}{expected_corpus}"),
                );
                if new_content != content {
                    std::fs::write(path, new_content.as_bytes())?;
                    println!(
                        "fixed: {} ({} -> {})",
                        path.display(),
                        info.id_corpora.first().unwrap_or(&String::new()),
                        expected_corpus
                    );
                }
            }
            fixed_count += 1;
        }
    }

    eprintln!(
        "{fixed_count} file(s) {}",
        if dry_run { "would be fixed" } else { "fixed" }
    );

    Ok(false)
}
