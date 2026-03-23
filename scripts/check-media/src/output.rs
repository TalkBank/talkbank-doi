//! Output formatting: terminal (human) and JSON modes.

use serde::Serialize;

use crate::diagnostics::{Diagnostic, Severity};
use crate::manifest::MediaManifest;

/// Print diagnostics in human-readable terminal format.
pub fn print_terminal(
    diagnostics: &[Diagnostic],
    _manifest: &MediaManifest,
    files_checked: usize,
    quiet: bool,
) {
    if diagnostics.is_empty() {
        if !quiet {
            eprintln!("No issues found ({files_checked} files checked)");
        }
        return;
    }

    // Group by bank.
    let mut by_bank: std::collections::BTreeMap<&str, Vec<&Diagnostic>> =
        std::collections::BTreeMap::new();
    for d in diagnostics {
        by_bank.entry(&d.bank).or_default().push(d);
    }

    for (bank, diags) in &by_bank {
        if !bank.is_empty() {
            println!("=== {bank} ===");
        }
        for d in diags {
            let severity_marker = match d.severity() {
                Severity::Error => "ERROR",
                Severity::Warning => "WARN ",
            };
            println!("  {severity_marker} [{tag}] {path}", tag = d.kind.tag(), path = d.path);
            println!("         {}", d.message);
        }
        println!();
    }

    // Summary.
    if !quiet {
        let errors = diagnostics.iter().filter(|d| d.severity() == Severity::Error).count();
        let warnings = diagnostics.iter().filter(|d| d.severity() == Severity::Warning).count();
        eprintln!(
            "Summary: {errors} error(s), {warnings} warning(s), {files_checked} files checked"
        );
    }
}

/// Print diagnostics as JSON.
pub fn print_json(
    diagnostics: &[Diagnostic],
    manifest: &MediaManifest,
    files_checked: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let errors = diagnostics.iter().filter(|d| d.severity() == Severity::Error).count();
    let warnings = diagnostics.iter().filter(|d| d.severity() == Severity::Warning).count();

    let report = CheckReport {
        manifest_generated_at: manifest.generated_at.to_rfc3339(),
        files_checked,
        diagnostics,
        summary: CheckSummary { errors, warnings },
    };

    let json = serde_json::to_string_pretty(&report)?;
    println!("{json}");
    Ok(())
}

#[derive(Serialize)]
struct CheckReport<'a> {
    manifest_generated_at: String,
    files_checked: usize,
    diagnostics: &'a [Diagnostic],
    summary: CheckSummary,
}

#[derive(Serialize)]
struct CheckSummary {
    errors: usize,
    warnings: usize,
}
