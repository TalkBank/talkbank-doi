//! %pic reference check: verify referenced image files exist locally.

use crate::diagnostics::{Diagnostic, DiagnosticKind};
use crate::extract::ChatFileInfo;

/// Check that all %pic references point to existing files.
pub fn check(
    bank: &str,
    info: &ChatFileInfo,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if info.pic_refs.is_empty() {
        return;
    }

    let chat_dir = info.path.parent().unwrap_or(info.path.as_path());
    let display_path = info.path.display().to_string();

    for pic_filename in &info.pic_refs {
        // %pic files are expected in a 'media' subdirectory relative to the CHAT file.
        let expected = chat_dir.join("media").join(pic_filename);
        if !expected.is_file() {
            diagnostics.push(Diagnostic::new(
                bank,
                &display_path,
                DiagnosticKind::MissingPic,
                format!("%%pic reference 'media/{pic_filename}' does not exist"),
            ));
        }
    }
}
