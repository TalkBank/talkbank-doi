//! Three-source DOI reconciliation: DataCite, CDC files, and HTML references.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::cdc;
use crate::datacite::RemoteDoi;
use crate::doi::{Doi, DoiState};

// ── Status ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryStatus {
    /// All available sources agree.
    Ok,
    /// No DOI registered anywhere — needs minting.
    NeedsMinting,
    /// DOI exists in DataCite but not in any CDC file (manually minted via Fabrica,
    /// or write-back never happened).
    ManuallyMinted,
    /// CDC file has a DOI value but DataCite returns 404.
    Unregistered,
    /// HTML references a DOI not found in CDC files or DataCite. Most dangerous:
    /// users may cite a DOI that does not resolve.
    HtmlOnly,
    /// DataCite + CDC agree but HTML hasn't been regenerated yet.
    HtmlStale,
    /// DOI registered in DataCite but the landing URL differs from what the repo
    /// path implies.
    UrlMismatch,
    /// Same DOI value appears in multiple CDC files.
    DuplicateDoi,
    /// DOI exists in DataCite as `draft` state — never published/findable.
    DraftOnly,
    /// DOI registered and consistent, but missing recommended metadata fields.
    Incomplete,
}

impl EntryStatus {
    pub fn is_suspicious(&self) -> bool {
        !matches!(self, EntryStatus::Ok | EntryStatus::Incomplete)
    }

    #[allow(dead_code)]
    pub fn is_error(&self) -> bool {
        matches!(
            self,
            EntryStatus::HtmlOnly | EntryStatus::UrlMismatch | EntryStatus::DuplicateDoi
        )
    }

    pub fn label(&self) -> &'static str {
        match self {
            EntryStatus::Ok => "OK",
            EntryStatus::NeedsMinting => "NEEDS DOI",
            EntryStatus::ManuallyMinted => "MANUAL MINT",
            EntryStatus::Unregistered => "UNREGISTERED",
            EntryStatus::HtmlOnly => "HTML ONLY",
            EntryStatus::HtmlStale => "HTML STALE",
            EntryStatus::UrlMismatch => "URL MISMATCH",
            EntryStatus::DuplicateDoi => "DUPLICATE",
            EntryStatus::DraftOnly => "DRAFT",
            EntryStatus::Incomplete => "INCOMPLETE",
        }
    }
}

// ── Decision ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Pending,
    /// Write the DataCite DOI into this CDC file (resolves ManuallyMinted).
    Adopt,
    /// Move this DOI from `findable` → `registered` (hidden from search).
    Retire,
    /// Delete this DOI from DataCite (only valid for `draft` state).
    Delete,
    /// Mark this entry as intentionally OK — suppress future warnings.
    Keep,
    /// Defer — will appear again next review.
    Skip,
    /// Queue this corpus for minting.
    Mint,
    /// Promote this DOI from `draft` → `findable`.
    Publish,
}

impl Decision {
    pub fn label(&self) -> &'static str {
        match self {
            Decision::Pending => "···",
            Decision::Adopt => "ADO",
            Decision::Retire => "RET",
            Decision::Delete => "DEL",
            Decision::Keep => " OK",
            Decision::Skip => "SKP",
            Decision::Mint => "MNT",
            Decision::Publish => "PUB",
        }
    }
}

// ── Per-source info ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CdcInfo {
    #[allow(dead_code)]
    pub doi: Option<Doi>,
    pub title: Option<String>,
    pub creators: Vec<String>,
    pub date: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    #[allow(dead_code)]
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum DataCiteInfo {
    /// DataCite was not queried (no `--verify` flag).
    NotQueried,
    /// Queried; this DOI is not registered under our prefix.
    NotFound,
    /// Queried; record found.
    Found(RemoteDoi),
}

#[derive(Debug, Clone)]
pub enum HtmlInfo {
    /// Web repos were not scanned (no `--web-dir`).
    NotScanned,
    /// Scanned; no DOI reference found in the access HTML for this corpus.
    NotFound,
    /// Scanned; DOI reference found.
    Found { doi: Doi, path: PathBuf },
}

// ── AuditEntry ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AuditEntry {
    #[allow(dead_code)]
    pub corpus_path: PathBuf,
    pub display_path: String,
    pub bank: String,
    pub target_url: Option<String>,
    pub cdc: CdcInfo,
    pub datacite: DataCiteInfo,
    pub html: HtmlInfo,
    pub status: EntryStatus,
    pub decision: Decision,
}

impl AuditEntry {
    /// Recompute status from current source data (call after applying DataCite or HTML).
    pub fn recompute_status(&mut self) {
        if self.status == EntryStatus::DuplicateDoi {
            return;
        }
        self.status = compute_status(self);
    }

    /// The DOI for this entry, preferring CDC over DataCite.
    pub fn doi(&self) -> Option<&Doi> {
        self.cdc.doi.as_ref().or_else(|| {
            if let DataCiteInfo::Found(r) = &self.datacite {
                Some(&r.doi)
            } else {
                None
            }
        })
    }
}

// ── Build from CDC ────────────────────────────────────────────────────────────

/// Scan all `*-data` repos under `data_dir` for `0metadata.cdc` files.
/// Returns one `AuditEntry` per corpus directory found.
pub fn build_from_cdc(data_dir: &Path) -> Vec<AuditEntry> {
    let cdc_paths: Vec<PathBuf> = WalkDir::new(data_dir)
        .into_iter()
        .filter_entry(|e| e.file_name() != ".git")
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "0metadata.cdc")
        .map(|e| e.into_path())
        .collect();

    let mut doi_to_indices: HashMap<String, Vec<usize>> = HashMap::new();
    let mut entries: Vec<AuditEntry> = Vec::new();

    for cdc_path in &cdc_paths {
        let Ok(meta) = cdc::parse(cdc_path) else {
            continue;
        };

        let parent = cdc_path.parent().unwrap_or(cdc_path);
        let display_path = parent
            .strip_prefix(data_dir)
            .unwrap_or(parent)
            .display()
            .to_string();

        let bank = parent
            .strip_prefix(data_dir)
            .ok()
            .and_then(|p| p.components().next())
            .and_then(|c| c.as_os_str().to_str())
            .map(|r| crate::repo_to_bank(r).to_string())
            .unwrap_or_default();

        let target_url = crate::target_url(cdc_path, data_dir);
        let doi = meta.doi.as_deref().and_then(|s| Doi::parse(s).ok());

        let cdc_info = CdcInfo {
            doi: doi.clone(),
            title: meta.title.clone(),
            creators: meta.creators.clone(),
            date: meta.date.clone(),
            description: meta.description.clone(),
            language: meta.language.clone(),
            path: cdc_path.clone(),
        };

        let idx = entries.len();
        if let Some(ref d) = doi {
            doi_to_indices
                .entry(d.to_key())
                .or_default()
                .push(idx);
        }

        entries.push(AuditEntry {
            corpus_path: parent.to_path_buf(),
            display_path,
            bank,
            target_url,
            cdc: cdc_info,
            datacite: DataCiteInfo::NotQueried,
            html: HtmlInfo::NotScanned,
            status: EntryStatus::Ok, // recomputed below
            decision: Decision::Pending,
        });
    }

    // Mark duplicate DOIs before computing individual statuses
    for indices in doi_to_indices.values() {
        if indices.len() > 1 {
            for &i in indices {
                entries[i].status = EntryStatus::DuplicateDoi;
            }
        }
    }

    for entry in &mut entries {
        if entry.status == EntryStatus::DuplicateDoi {
            continue;
        }
        entry.status = compute_status(entry);
    }

    entries
}

// ── Apply DataCite ────────────────────────────────────────────────────────────

/// Merge DataCite remote DOI data into entries.
///
/// `remote`: map from uppercase DOI key → `RemoteDoi` (from `datacite::list_all`).
///
/// After applying:
/// - Entries whose CDC DOI is found in `remote` get `DataCiteInfo::Found`.
/// - Entries whose CDC DOI is absent from `remote` get `DataCiteInfo::NotFound`.
/// - DOIs in `remote` with no matching CDC file are added as new orphan entries.
pub fn apply_datacite(entries: &mut Vec<AuditEntry>, remote: HashMap<String, RemoteDoi>) {
    let cdc_keys: HashSet<String> = entries
        .iter()
        .filter_map(|e| e.cdc.doi.as_ref())
        .map(|d| d.to_key())
        .collect();

    // Annotate existing entries
    for entry in entries.iter_mut() {
        if entry.status == EntryStatus::DuplicateDoi {
            continue;
        }
        if let Some(ref doi) = entry.cdc.doi {
            entry.datacite = match remote.get(&doi.to_key()) {
                Some(r) => DataCiteInfo::Found(r.clone()),
                None => DataCiteInfo::NotFound,
            };
            entry.recompute_status();
        }
    }

    // Add orphan entries for DataCite DOIs with no CDC counterpart
    for (key, r) in &remote {
        if cdc_keys.contains(key) {
            continue;
        }
        let status = if r.state == DoiState::Draft {
            EntryStatus::DraftOnly
        } else {
            EntryStatus::ManuallyMinted
        };
        entries.push(AuditEntry {
            corpus_path: PathBuf::new(),
            display_path: format!("(DataCite only) {}", r.title),
            bank: String::new(),
            target_url: Some(r.url.clone()),
            cdc: CdcInfo {
                doi: None,
                title: Some(r.title.clone()),
                creators: vec![],
                date: None,
                description: None,
                language: None,
                path: PathBuf::new(),
            },
            datacite: DataCiteInfo::Found(r.clone()),
            html: HtmlInfo::NotScanned,
            status,
            decision: Decision::Pending,
        });
    }
}

// ── HTML scan ────────────────────────────────────────────────────────────────

/// Scan all `*-bank` repos under `web_dir` for DOI values injected by
/// `generate-from-chat` into `access/*.html` files.
///
/// The injection pattern is: `<td> doi:10.21415/XXXX </td>`
///
/// Returns a map from uppercase DOI key → first HTML file path found.
pub fn scan_html(web_dir: &Path) -> HashMap<String, PathBuf> {
    let mut result: HashMap<String, PathBuf> = HashMap::new();

    for entry in WalkDir::new(web_dir)
        .into_iter()
        .filter_entry(|e| e.file_name() != ".git")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("html"))
    {
        let Ok(content) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        for line in content.lines() {
            // Look for the pattern injected by generate-from-chat
            if let Some(pos) = line.find("doi:10.21415/") {
                let rest = &line[pos + 4..]; // skip "doi:"
                let end = rest
                    .find(|c: char| c.is_whitespace() || matches!(c, '<' | '"' | '\''))
                    .unwrap_or(rest.len());
                if let Ok(doi) = Doi::parse(&rest[..end]) {
                    result
                        .entry(doi.to_key())
                        .or_insert_with(|| entry.path().to_path_buf());
                }
            }
        }
    }

    result
}

/// Apply HTML scan results to existing entries, then add HTML-only orphan entries.
pub fn apply_html(entries: &mut Vec<AuditEntry>, html_map: HashMap<String, PathBuf>) {
    for entry in entries.iter_mut() {
        if entry.status == EntryStatus::DuplicateDoi {
            continue;
        }
        let key = entry.cdc.doi.as_ref().map(|d| d.to_key());
        entry.html = match key.as_deref().and_then(|k| html_map.get(k)) {
            Some(path) => HtmlInfo::Found {
                doi: entry.cdc.doi.clone().unwrap(),
                path: path.clone(),
            },
            None if key.is_some() => HtmlInfo::NotFound,
            None => HtmlInfo::NotScanned, // no CDC DOI to look up
        };
        entry.recompute_status();
    }

    // Add HTML-only orphan entries
    let known_keys: HashSet<String> = entries
        .iter()
        .filter_map(|e| e.cdc.doi.as_ref().or_else(|| {
            if let DataCiteInfo::Found(r) = &e.datacite { Some(&r.doi) } else { None }
        }))
        .map(|d| d.to_key())
        .collect();

    for (key, path) in &html_map {
        if known_keys.contains(key) {
            continue;
        }
        if let Ok(doi) = Doi::parse(key) {
            entries.push(AuditEntry {
                corpus_path: PathBuf::new(),
                display_path: format!("(HTML only) {key}"),
                bank: String::new(),
                target_url: None,
                cdc: CdcInfo {
                    doi: None,
                    title: None,
                    creators: vec![],
                    date: None,
                    description: None,
                    language: None,
                    path: PathBuf::new(),
                },
                datacite: DataCiteInfo::NotQueried,
                html: HtmlInfo::Found {
                    doi,
                    path: path.clone(),
                },
                status: EntryStatus::HtmlOnly,
                decision: Decision::Pending,
            });
        }
    }
}

// ── Status computation ────────────────────────────────────────────────────────

fn compute_status(entry: &AuditEntry) -> EntryStatus {
    let has_cdc_doi = entry.cdc.doi.is_some();

    // DataCite found — check for state and URL issues first
    if let DataCiteInfo::Found(r) = &entry.datacite {
        if r.state == DoiState::Draft {
            return EntryStatus::DraftOnly;
        }
        if let Some(local_url) = &entry.target_url {
            if !r.url.is_empty() && local_url != &r.url {
                return EntryStatus::UrlMismatch;
            }
        }
        if !has_cdc_doi {
            return EntryStatus::ManuallyMinted;
        }
    }

    // HTML-only: HTML has a DOI but nothing else does
    if matches!(&entry.html, HtmlInfo::Found { .. })
        && !has_cdc_doi
        && !matches!(&entry.datacite, DataCiteInfo::Found(_))
    {
        return EntryStatus::HtmlOnly;
    }

    // Unregistered: CDC has DOI, DataCite says not found
    if has_cdc_doi && matches!(&entry.datacite, DataCiteInfo::NotFound) {
        return EntryStatus::Unregistered;
    }

    // HTML stale: CDC+DataCite consistent but HTML came back empty (when scanned)
    if has_cdc_doi && matches!(&entry.html, HtmlInfo::NotFound) {
        return EntryStatus::HtmlStale;
    }

    // No DOI anywhere
    if !has_cdc_doi {
        return EntryStatus::NeedsMinting;
    }

    // Has DOI — check metadata completeness
    if entry.cdc.description.is_none() || entry.cdc.language.is_none() {
        return EntryStatus::Incomplete;
    }

    EntryStatus::Ok
}
