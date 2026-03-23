//! Display manifest contents and statistics.

use std::path::PathBuf;

use super::{MediaManifest, resolve_manifest_path};

/// Entry point for the `show-manifest` subcommand.
pub fn run(
    manifest_path: &Option<PathBuf>,
    show_files: bool,
    bank_filter: Option<&str>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let path = resolve_manifest_path(manifest_path);
    let manifest = MediaManifest::load(&path).map_err(Box::new)?;

    println!("Manifest: {}", path.display());
    println!("Generated: {}", manifest.generated_at);
    println!("Source: {}:{}", manifest.source_host, manifest.media_root);
    if manifest.is_stale() {
        println!("WARNING: manifest is older than 7 days — consider refreshing");
    }
    println!();

    for (bank, listing) in &manifest.banks {
        if let Some(filter) = bank_filter {
            if bank != filter {
                continue;
            }
        }
        println!("{bank}: {} files", listing.file_count);
        if show_files {
            for file in &listing.files {
                println!("  {file}");
            }
        }
    }

    println!();
    println!("Total: {} banks, {} files", manifest.banks.len(), manifest.total_files());

    Ok(false)
}
