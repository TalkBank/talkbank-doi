//! Add "unlinked" status to @Media headers where media exists but no bullets.

use std::path::PathBuf;
use std::sync::LazyLock;

use regex::Regex;
use walkdir::WalkDir;

use crate::config;
use crate::extract::{self, MediaType};
use crate::manifest::{self, MediaManifest};

/// Regex to capture a complete @Media line for replacement.
static MEDIA_LINE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?m)^(@Media:\t(?:[^" ,]+|"[^"]+") *, *(?:audio|video))"#,
    ).expect("media line regex is valid")
});

/// Run the add-unlinked fix.
pub fn run(
    paths: &[PathBuf],
    manifest_path: &Option<PathBuf>,
    dry_run: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let manifest_file = manifest::resolve_manifest_path(manifest_path);
    let manifest = MediaManifest::load(&manifest_file).map_err(Box::new)?;

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

            let Some((bank, _, _)) = config::detect_repo_from_path(path) else {
                continue;
            };

            let info = extract::extract_chat_info(path)?;

            let Some(ref media) = info.media else { continue };
            if media.status.is_some() || info.has_bullets {
                continue;
            }

            let extension = match media.media_type {
                MediaType::Audio => "mp3",
                MediaType::Video => "mp4",
            };
            let chat_dir = crate::check::repo_relative_path(
                path.parent().unwrap_or(path),
            ).unwrap_or_default();
            let expected_path = if chat_dir.is_empty() {
                format!("{}.{extension}", media.filename)
            } else {
                format!("{chat_dir}/{}.{extension}", media.filename)
            };

            let (exists, _) = manifest.check_media(bank, &expected_path);
            if !exists {
                continue;
            }

            if dry_run {
                println!("would add 'unlinked' to {}", path.display());
            } else {
                let content = std::fs::read_to_string(path)?;
                let new_content = MEDIA_LINE_RE.replace(
                    &content,
                    "${1}, unlinked\n@Comment:\tPlease use the slider at the left to control media playback.",
                );
                if new_content != content {
                    std::fs::write(path, new_content.as_bytes())?;
                    println!("fixed: {}", path.display());
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
