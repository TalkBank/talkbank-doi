//! Bullet consistency checks: verify timing bullets match @Media status.

use crate::diagnostics::{Diagnostic, DiagnosticKind};
use crate::extract::{ChatFileInfo, MediaStatus, MediaType};
use crate::manifest::MediaManifest;

/// Check bullet ↔ media status consistency for a single file.
pub fn check(
    bank: &str,
    info: &ChatFileInfo,
    manifest: &MediaManifest,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(ref media) = info.media else {
        return;
    };

    let display_path = info.path.display().to_string();

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

    let (media_exists, _) = manifest.check_media(bank, &expected_path);

    if info.has_bullets {
        match &media.status {
            Some(MediaStatus::Unlinked) => {
                diagnostics.push(Diagnostic::new(
                    bank, &display_path, DiagnosticKind::BulletsButMarkedUnlinked,
                    format!("has timing bullets but @Media is marked 'unlinked' for {expected_path}"),
                ));
            }
            Some(MediaStatus::Notrans) => {
                diagnostics.push(Diagnostic::new(
                    bank, &display_path, DiagnosticKind::BulletsButMarkedNotrans,
                    format!("has timing bullets but @Media is marked 'notrans' for {expected_path}"),
                ));
            }
            _ => {}
        }
    } else {
        match &media.status {
            Some(_) => {}
            None if media_exists => {
                diagnostics.push(Diagnostic::new(
                    bank, &display_path, DiagnosticKind::NoBulletsNeedUnlinked,
                    format!("has media but no timing bullets; should be marked 'unlinked' for {expected_path}"),
                ));
            }
            None => {
                diagnostics.push(Diagnostic::new(
                    bank, &display_path, DiagnosticKind::NoBulletsMediaMissing,
                    format!("no media found and no timing bullets; should be marked 'missing' for {expected_path}"),
                ));
            }
        }
    }
}
