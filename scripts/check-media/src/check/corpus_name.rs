//! Corpus name check: verify @ID corpus field matches directory structure.

use crate::config;
use crate::diagnostics::{Diagnostic, DiagnosticKind};
use crate::extract::ChatFileInfo;

/// Check @ID corpus field against the directory-derived corpus name.
pub fn check(
    bank: &str,
    info: &ChatFileInfo,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if info.id_corpora.is_empty() {
        return;
    }

    let display_path = info.path.display().to_string();

    let expected = config::detect_corpus(&info.path);

    match expected {
        None => {
            diagnostics.push(Diagnostic::new(
                bank,
                &display_path,
                DiagnosticKind::NoCorpusDetected,
                "no 0metadata.cdc found in any ancestor directory",
            ));
        }
        Some(expected_corpus) => {
            for actual in &info.id_corpora {
                if actual != &expected_corpus {
                    diagnostics.push(Diagnostic::new(
                        bank,
                        &display_path,
                        DiagnosticKind::CorpusNameMismatch,
                        format!(
                            "@ID corpus '{actual}' does not match expected '{expected_corpus}'"
                        ),
                    ));
                    break; // One diagnostic per file is enough.
                }
            }
        }
    }
}
