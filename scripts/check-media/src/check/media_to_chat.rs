//! Media → CHAT checks: find media files with no corresponding CHAT file.

use std::collections::BTreeMap;

use crate::diagnostics::{Diagnostic, DiagnosticKind};
use crate::manifest::MediaManifest;

/// Check all media files in the manifest for missing CHAT counterparts.
pub fn check(
    manifest: &MediaManifest,
    chat_by_bank: &BTreeMap<&str, Vec<String>>,
    bank_filter: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (bank, listing) in &manifest.banks {
        if let Some(filter) = bank_filter {
            if bank != filter {
                continue;
            }
        }

        // Build case-insensitive set of known CHAT paths for this bank.
        let chat_set: std::collections::HashSet<String> = chat_by_bank
            .get(bank.as_str())
            .map(|paths| paths.iter().map(|p| p.to_lowercase()).collect())
            .unwrap_or_default();

        for media_path in &listing.files {
            // Skip jpg/gif — those are for %pic, not @Media.
            if media_path.ends_with(".jpg") || media_path.ends_with(".gif")
                || media_path.ends_with(".JPG") || media_path.ends_with(".GIF")
            {
                continue;
            }

            // Derive expected CHAT path.
            let Some(stem_end) = media_path.rfind('.') else {
                continue;
            };
            let expected_chat = format!("{}.cha", &media_path[..stem_end]);
            let expected_lower = expected_chat.to_lowercase();

            if !chat_set.contains(&expected_lower) {
                diagnostics.push(Diagnostic::new(
                    bank,
                    media_path,
                    DiagnosticKind::MissingChat,
                    format!("media file has no corresponding CHAT at {expected_chat}"),
                ));
            }
        }
    }
}
