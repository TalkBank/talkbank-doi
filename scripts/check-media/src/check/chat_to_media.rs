//! CHAT → media checks: verify that media referenced by CHAT files exist.

use crate::diagnostics::{Diagnostic, DiagnosticKind};
use crate::extract::{ChatFileInfo, MediaStatus, MediaType};
use crate::manifest::MediaManifest;

/// Run CHAT-to-media checks for a single file.
pub fn check(
    bank: &str,
    info: &ChatFileInfo,
    manifest: &MediaManifest,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(ref media) = info.media else {
        return;
    };

    let chat_basename = info.path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let display_path = info.path.display().to_string();

    // Check @Media filename matches CHAT file basename.
    if !media.filename.starts_with('"') && media.filename != chat_basename {
        diagnostics.push(Diagnostic::new(
            bank,
            &display_path,
            DiagnosticKind::FilenameMismatch,
            format!(
                "@Media filename '{}' does not match CHAT basename '{chat_basename}'",
                media.filename
            ),
        ));
    }

    // Determine expected media file path relative to bank root.
    let extension = match media.media_type {
        MediaType::Audio => "mp3",
        MediaType::Video => "mp4",
    };

    let chat_dir = super::repo_relative_path(
        info.path.parent().unwrap_or(info.path.as_path()),
    ).unwrap_or_default();

    let expected_path = if chat_dir.is_empty() {
        format!("{}.{extension}", media.filename)
    } else {
        format!("{chat_dir}/{}.{extension}", media.filename)
    };

    let (exists, case_matches) = manifest.check_media(bank, &expected_path);

    match (&media.status, exists) {
        (Some(MediaStatus::Missing), true) => {
            diagnostics.push(Diagnostic::new(
                bank, &display_path, DiagnosticKind::MarkedMissingButExists,
                format!("@Media marked 'missing' but file exists at {expected_path}"),
            ));
        }
        (Some(MediaStatus::Missing), false) => {}
        (Some(status), false) => {
            diagnostics.push(Diagnostic::new(
                bank, &display_path, DiagnosticKind::MarkedStatusButNoMedia,
                format!("@Media marked '{}' but no file found at {expected_path}", status.as_str()),
            ));
        }
        (Some(_), true) | (None, true) => {
            if !case_matches {
                diagnostics.push(Diagnostic::new(
                    bank, &display_path, DiagnosticKind::MediaCaseMismatch,
                    format!("media file exists but with different case than '{expected_path}'"),
                ));
            }
        }
        (None, false) => {
            diagnostics.push(Diagnostic::new(
                bank, &display_path, DiagnosticKind::MissingMedia,
                format!("no media file found for {extension} '{expected_path}'"),
            ));
        }
    }
}
