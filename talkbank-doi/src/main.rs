mod audit;
mod cdc;
mod datacite;
mod doi;
mod tui;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "talkbank-doi", about = "TalkBank DOI management")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Discover all 0metadata.cdc files and report status.
    Check {
        /// Root directory containing data repos (e.g., ~/0data)
        #[arg(long, default_value = ".")]
        data_dir: PathBuf,
    },
    /// Sync all DOIs with DataCite (mint new, update changed).
    Sync {
        /// Root directory containing data repos
        #[arg(long, default_value = ".")]
        data_dir: PathBuf,
        /// Show what would be done without making changes.
        #[arg(long)]
        dry_run: bool,
    },
    /// Query DataCite for a specific DOI.
    Query {
        /// DOI to query (e.g., 10.21415/T56W31)
        doi: String,
    },
    /// Export all discovered DOIs to CSV.
    Export {
        /// Root directory containing data repos
        #[arg(long, default_value = ".")]
        data_dir: PathBuf,
        /// Output CSV path
        #[arg(long, default_value = "dois.csv")]
        output: PathBuf,
    },
    /// Interactive TUI review: page through all corpora with Brian.
    Review {
        /// Root directory containing data repos (e.g., ~/talkbank/data)
        #[arg(long, default_value = ".")]
        data_dir: PathBuf,
        /// Root directory containing web repos for HTML scanning (optional)
        #[arg(long)]
        web_dir: Option<PathBuf>,
        /// Query DataCite for each known DOI to verify state and detect orphans.
        /// Makes one paginated API call to list all our DOIs. Requires credentials.
        #[arg(long)]
        verify: bool,
    },
    /// Three-source reconciliation report (DataCite, CDC files, HTML).
    /// Prints a summary and writes a decisions CSV you can apply with `apply`.
    Audit {
        /// Root directory containing data repos
        #[arg(long, default_value = ".")]
        data_dir: PathBuf,
        /// Root directory containing web repos for HTML scanning (optional)
        #[arg(long)]
        web_dir: Option<PathBuf>,
        /// Query DataCite to verify DOI states and detect orphans.
        #[arg(long)]
        verify: bool,
        /// Output CSV path for audit results
        #[arg(long, default_value = "doi-audit.csv")]
        output: PathBuf,
    },
}

/// Map a data repo name to its bank name.
pub(crate) fn repo_to_bank(repo_name: &str) -> &str {
    let base = repo_name.strip_suffix("-data").unwrap_or(repo_name);
    match base {
        "childes-eng-na" | "childes-eng-uk" | "childes-romance-germanic" | "childes-other" => {
            "childes"
        }
        "ca-candor" => "ca",
        "phon-eng-french" | "phon-other" => "phon",
        "homebank-public" | "homebank-cougar" | "homebank-bergelson" | "homebank-password" => {
            "homebank"
        }
        other => other,
    }
}

/// Bank domain mapping for URL construction.
fn bank_domain(bank: &str) -> String {
    match bank {
        "talkbank" => "talkbank.org".to_string(),
        _ => format!("{bank}.talkbank.org"),
    }
}

/// Construct the DOI landing page URL from a CDC file path.
pub(crate) fn target_url(cdc_path: &Path, data_dir: &Path) -> Option<String> {
    let relative = cdc_path.parent()?.strip_prefix(data_dir).ok()?;
    let mut components = relative.components();
    let repo_dir = components.next()?.as_os_str().to_str()?;
    let bank = repo_to_bank(repo_dir);
    let domain = bank_domain(bank);

    let corpus_path: PathBuf = components.collect();
    if corpus_path.as_os_str().is_empty() {
        return None;
    }

    Some(format!("https://{domain}/access/{}.html", corpus_path.display()))
}

/// Discover all 0metadata.cdc files under a directory.
fn discover_cdc_files(data_dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(data_dir)
        .into_iter()
        .filter_entry(|e| e.file_name() != ".git")
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "0metadata.cdc")
        .map(|e| e.into_path())
        .collect()
}

fn cmd_check(data_dir: &Path) -> Result<()> {
    let cdc_files = discover_cdc_files(data_dir);
    println!("Found {} corpus directories with 0metadata.cdc\n", cdc_files.len());

    let mut with_doi = 0u32;
    let mut without_doi = 0u32;
    let mut errors = 0u32;
    let mut dois_seen: HashMap<String, PathBuf> = HashMap::new();

    for path in &cdc_files {
        match cdc::parse(path) {
            Ok(meta) => {
                if let Err(e) = meta.validate(path) {
                    eprintln!("  INVALID: {e}");
                    errors += 1;
                    continue;
                }

                if let Some(ref doi) = meta.doi {
                    with_doi += 1;
                    if let Some(prev) = dois_seen.get(doi) {
                        eprintln!(
                            "  DUPLICATE DOI {doi}:\n    {}\n    {}",
                            prev.display(),
                            path.display()
                        );
                        errors += 1;
                    } else {
                        dois_seen.insert(doi.clone(), path.clone());
                    }
                } else {
                    without_doi += 1;
                    println!("  NEW (no DOI): {}", path.display());
                }

                if target_url(path, data_dir).is_none() {
                    eprintln!("  WARN: can't construct URL for {}", path.display());
                }

                let missing: Vec<&str> = [
                    meta.language.is_none().then_some("language"),
                    meta.description.is_none().then_some("description"),
                    meta.country.is_none().then_some("country"),
                ]
                .into_iter()
                .flatten()
                .collect();

                if !missing.is_empty() {
                    let display = path.strip_prefix(data_dir).unwrap_or(path);
                    println!(
                        "  SPARSE: {} missing {}",
                        display.display(),
                        missing.join(", ")
                    );
                }
            }
            Err(e) => {
                eprintln!("  ERROR parsing {}: {e}", path.display());
                errors += 1;
            }
        }
    }

    println!("\nSummary:");
    println!("  With DOI:    {with_doi}");
    println!("  Without DOI: {without_doi} (need minting)");
    println!("  Errors:      {errors}");
    println!("  Total:       {}", cdc_files.len());

    Ok(())
}

fn cmd_query(doi: &str) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.datacite.org/dois/{doi}");
    let resp = client.get(&url).send().context("querying DataCite")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        anyhow::bail!("DataCite query failed ({status}): {body}");
    }

    let body: serde_json::Value = resp.json()?;
    println!("{}", serde_json::to_string_pretty(&body)?);
    Ok(())
}

fn cmd_export(data_dir: &Path, output: &Path) -> Result<()> {
    let cdc_files = discover_cdc_files(data_dir);
    let mut writer = csv::Writer::from_path(output)?;

    writer.write_record([
        "doi", "target", "title", "creators", "publisher", "year", "language", "description",
        "country",
    ])?;

    let mut count = 0u32;
    for path in &cdc_files {
        let meta = cdc::parse(path)?;
        if meta.title.is_none() {
            continue;
        }

        let url = target_url(path, data_dir).unwrap_or_default();
        let creators = meta.creators.join("; ");

        writer.write_record([
            meta.doi.as_deref().unwrap_or(""),
            &url,
            meta.title.as_deref().unwrap_or(""),
            &creators,
            meta.publisher.as_deref().unwrap_or("TalkBank"),
            &meta
                .publication_year()
                .map_or(String::new(), |y| y.to_string()),
            meta.language.as_deref().unwrap_or(""),
            meta.description.as_deref().unwrap_or(""),
            meta.country.as_deref().unwrap_or(""),
        ])?;
        count += 1;
    }

    writer.flush()?;
    println!("Exported {count} records to {}", output.display());
    Ok(())
}

fn cmd_sync(data_dir: &Path, dry_run: bool) -> Result<()> {
    let config = datacite::Config::from_env()?;
    let client = reqwest::blocking::Client::new();
    let cdc_files = discover_cdc_files(data_dir);

    println!(
        "Syncing {} corpora with DataCite ({})",
        cdc_files.len(),
        if dry_run { "DRY RUN" } else { "LIVE" }
    );
    println!("  API: {}", config.api_url);
    println!("  Prefix: {}", config.prefix);
    println!();

    let mut minted = 0u32;
    let mut updated = 0u32;
    let mut skipped = 0u32;
    let mut errors = 0u32;

    for path in &cdc_files {
        let meta = match cdc::parse(path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("  ERROR parsing {}: {e}", path.display());
                errors += 1;
                continue;
            }
        };

        if let Err(e) = meta.validate(path) {
            eprintln!("  SKIP (invalid): {e}");
            skipped += 1;
            continue;
        }

        let Some(url) = target_url(path, data_dir) else {
            eprintln!("  SKIP (no URL): {}", path.display());
            skipped += 1;
            continue;
        };

        let record = datacite::build_record(&meta, &url, &config.prefix)?;

        if meta.doi.is_none() {
            let display = path.strip_prefix(data_dir).unwrap_or(path);
            if dry_run {
                println!("  WOULD MINT: {} → {url}", display.display());
                minted += 1;
            } else {
                match datacite::mint(&client, &config, &record) {
                    Ok(new_doi) => {
                        println!("  MINTED: {new_doi} → {url}");
                        cdc::write_doi(path, &new_doi)?;
                        minted += 1;
                    }
                    Err(e) => {
                        eprintln!("  ERROR minting for {}: {e}", display.display());
                        errors += 1;
                    }
                }
            }
        } else {
            let doi = meta.doi.as_ref().unwrap();
            if dry_run {
                println!("  WOULD UPDATE: {doi}");
                updated += 1;
            } else {
                match datacite::update(&client, &config, doi, &record) {
                    Ok(()) => {
                        updated += 1;
                    }
                    Err(e) => {
                        eprintln!("  ERROR updating {doi}: {e}");
                        errors += 1;
                    }
                }
            }
        }
    }

    println!("\nSummary:");
    println!("  Minted:  {minted}");
    println!("  Updated: {updated}");
    println!("  Skipped: {skipped}");
    println!("  Errors:  {errors}");

    Ok(())
}

fn build_audit_entries(
    data_dir: &Path,
    web_dir: Option<&Path>,
    verify: bool,
) -> Result<Vec<audit::AuditEntry>> {
    let client = reqwest::blocking::Client::new();

    eprintln!("Scanning CDC files in {}…", data_dir.display());
    let mut entries = audit::build_from_cdc(data_dir);
    eprintln!("  Found {} corpora", entries.len());

    if verify {
        let config = datacite::Config::from_env()?;
        eprintln!("Fetching all DOIs from DataCite…");
        let remote = datacite::list_all(&client, &config)?;
        eprintln!("  DataCite returned {} DOIs under our prefix", remote.len());
        audit::apply_datacite(&mut entries, remote);
    }

    if let Some(web) = web_dir {
        eprintln!("Scanning HTML files in {}…", web.display());
        let html_map = audit::scan_html(web);
        eprintln!("  Found {} DOI references in HTML", html_map.len());
        audit::apply_html(&mut entries, html_map);
    }

    Ok(entries)
}

fn cmd_review(data_dir: &Path, web_dir: Option<&Path>, verify: bool) -> Result<()> {
    let entries = build_audit_entries(data_dir, web_dir, verify)?;

    let suspicious = entries.iter().filter(|e| e.status.is_suspicious()).count();
    eprintln!(
        "\n{} corpora loaded  ({} suspicious)\n",
        entries.len(),
        suspicious
    );

    let decided = tui::run(entries)?;

    // Report decisions
    let mut adopted = 0u32;
    let mut retiring = 0u32;
    let mut minting = 0u32;
    let mut other = 0u32;

    for entry in &decided {
        match entry.decision {
            audit::Decision::Adopt => adopted += 1,
            audit::Decision::Retire | audit::Decision::Delete => retiring += 1,
            audit::Decision::Mint | audit::Decision::Publish => minting += 1,
            audit::Decision::Keep | audit::Decision::Skip | audit::Decision::Pending => other += 1,
        }
    }

    println!("\nSession complete:");
    println!("  Adopt (write DOI to CDC):  {adopted}");
    println!("  Retire/delete from DC:     {retiring}");
    println!("  Mint/publish:              {minting}");
    println!("  Keep/skip/pending:         {other}");
    println!("\nTo apply decisions, run: talkbank-doi apply --decisions doi-decisions.csv");
    println!("(decisions not yet saved — apply command coming in next release)");

    Ok(())
}

fn cmd_audit(data_dir: &Path, web_dir: Option<&Path>, verify: bool, output: &Path) -> Result<()> {
    let entries = build_audit_entries(data_dir, web_dir, verify)?;

    let mut writer = csv::Writer::from_path(output)?;
    writer.write_record(["status", "decision", "doi", "title", "bank", "path", "target_url"])?;

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for entry in &entries {
        let doi = entry
            .doi()
            .map(|d| d.to_string())
            .unwrap_or_default();
        let title = entry.cdc.title.as_deref().or_else(|| {
            if let audit::DataCiteInfo::Found(r) = &entry.datacite {
                Some(r.title.as_str())
            } else {
                None
            }
        }).unwrap_or("");
        writer.write_record([
            entry.status.label(),
            entry.decision.label().trim(),
            &doi,
            title,
            &entry.bank,
            &entry.display_path,
            entry.target_url.as_deref().unwrap_or(""),
        ])?;
        *counts.entry(entry.status.label()).or_default() += 1;
    }
    writer.flush()?;

    println!("Audit results written to {}", output.display());
    println!("\nBreakdown:");
    let mut pairs: Vec<_> = counts.iter().collect();
    pairs.sort_by_key(|(k, _)| *k);
    for (status, count) in pairs {
        println!("  {status:<16} {count}");
    }
    println!("  {:<16} {}", "TOTAL", entries.len());

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Check { data_dir } => cmd_check(&data_dir),
        Command::Sync { data_dir, dry_run } => cmd_sync(&data_dir, dry_run),
        Command::Query { doi } => cmd_query(&doi),
        Command::Export { data_dir, output } => cmd_export(&data_dir, &output),
        Command::Review { data_dir, web_dir, verify } => {
            cmd_review(&data_dir, web_dir.as_deref(), verify)
        }
        Command::Audit { data_dir, web_dir, verify, output } => {
            cmd_audit(&data_dir, web_dir.as_deref(), verify, &output)
        }
    }
}
