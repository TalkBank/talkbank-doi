//! Overlap consistency audit (Experiment B).
//!
//! Parses CHAT files with CA overlap markers (⌈⌉⌊⌋) and reports:
//! 1. Pairing quality: how many markers are fully paired vs onset-only
//! 2. Cross-speaker consistency: do ⌈ markers on line N have matching ⌊ on line N+1?
//! 3. Temporal consistency: for files with timing bullets, do paired overlaps
//!    actually overlap in time?
//!
//! This is a diagnostic tool — it does not modify files.

use std::collections::BTreeMap;
use std::path::Path;

use talkbank_model::alignment::helpers::overlap::{OverlapRegionKind, extract_overlap_info};
use talkbank_model::model::{ChatFile, Line};

/// Per-utterance overlap summary.
#[derive(Debug)]
struct UttOverlapInfo {
    /// Speaker code.
    speaker: String,
    /// Has ⌈ (top overlap begin).
    has_top_begin: bool,
    /// Has ⌉ (top overlap end).
    has_top_end: bool,
    /// Has ⌊ (bottom overlap begin).
    has_bottom_begin: bool,
    /// Has ⌋ (bottom overlap end).
    has_bottom_end: bool,
    /// Utterance timing bullet (start_ms, end_ms) if present.
    bullet: Option<(u64, u64)>,
}

/// Per-file audit results.
#[derive(Debug)]
pub struct FileAuditResult {
    /// File path (for display).
    pub path: String,
    /// Total utterances in the file.
    pub total_utterances: usize,
    /// Utterances with any CA overlap marker.
    pub utterances_with_markers: usize,
    /// Count of ⌈ markers found.
    pub top_begin_count: usize,
    /// Count of ⌉ markers found.
    pub top_end_count: usize,
    /// Count of ⌊ markers found.
    pub bottom_begin_count: usize,
    /// Count of ⌋ markers found.
    pub bottom_end_count: usize,
    /// Number of cross-speaker pairs found (⌈ on line N, ⌊ on line N+1).
    pub cross_speaker_pairs: usize,
    /// Of those pairs, how many have timing bullets on both utterances.
    pub timed_pairs: usize,
    /// Of timed pairs, how many are temporally consistent (overlapping in time).
    pub temporally_consistent: usize,
    /// Pairing quality category.
    pub quality: PairingQuality,
}

/// Classification of a file's overlap marker pairing quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairingQuality {
    /// Both ⌈⌉ and ⌊⌋ are present and reasonably balanced.
    FullyPaired,
    /// Some markers are paired, some are not.
    Mixed,
    /// Only opening markers (⌈⌊ without ⌉⌋).
    OpenOnly,
    /// No overlap markers at all.
    None,
}

impl std::fmt::Display for PairingQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PairingQuality::FullyPaired => write!(f, "fully_paired"),
            PairingQuality::Mixed => write!(f, "mixed"),
            PairingQuality::OpenOnly => write!(f, "open_only"),
            PairingQuality::None => write!(f, "none"),
        }
    }
}

/// Audit a single CHAT file for overlap marker consistency.
pub fn audit_file(chat: &ChatFile, path: &str) -> FileAuditResult {
    let mut utt_infos: Vec<UttOverlapInfo> = Vec::new();

    for line in &chat.lines {
        let Line::Utterance(utt) = line else {
            continue;
        };

        let overlap = extract_overlap_info(&utt.main.content.content.0);
        let speaker = utt.main.speaker.to_string();
        let bullet = utt
            .main
            .content
            .bullet
            .as_ref()
            .map(|b| (b.timing.start_ms, b.timing.end_ms));

        // Only track utterances that have overlap markers
        let has_top_begin = overlap.has_top_overlap();
        let has_top_end = overlap.regions.iter().any(|r| {
            r.kind == OverlapRegionKind::Top && r.end_at_word.is_some()
        });
        let has_bottom_begin = overlap.has_bottom_overlap();
        let has_bottom_end = overlap.regions.iter().any(|r| {
            r.kind == OverlapRegionKind::Bottom && r.end_at_word.is_some()
        });

        utt_infos.push(UttOverlapInfo {
            speaker,
            has_top_begin,
            has_top_end,
            has_bottom_begin,
            has_bottom_end,
            bullet,
        });
    }

    // Count markers
    let top_begin_count = utt_infos.iter().filter(|u| u.has_top_begin).count();
    let top_end_count = utt_infos.iter().filter(|u| u.has_top_end).count();
    let bottom_begin_count = utt_infos.iter().filter(|u| u.has_bottom_begin).count();
    let bottom_end_count = utt_infos.iter().filter(|u| u.has_bottom_end).count();
    let utterances_with_markers = utt_infos
        .iter()
        .filter(|u| u.has_top_begin || u.has_top_end || u.has_bottom_begin || u.has_bottom_end)
        .count();

    // Find cross-speaker pairs: ⌈ on utterance N, ⌊ on utterance N+1 from different speaker
    let mut cross_speaker_pairs = 0;
    let mut timed_pairs = 0;
    let mut temporally_consistent = 0;

    for i in 0..utt_infos.len().saturating_sub(1) {
        let top = &utt_infos[i];
        let bottom = &utt_infos[i + 1];

        if !top.has_top_begin || !bottom.has_bottom_begin {
            continue;
        }
        if top.speaker == bottom.speaker {
            continue; // Same speaker — not a cross-speaker pair
        }

        cross_speaker_pairs += 1;

        // Check temporal consistency if both have timing
        if let (Some(top_bullet), Some(bottom_bullet)) = (top.bullet, bottom.bullet) {
            timed_pairs += 1;

            // Temporal consistency: bottom utterance's start should fall within
            // or near the top utterance's time range. Allow 2s tolerance.
            let tolerance_ms = 2000;
            let top_start = top_bullet.0.saturating_sub(tolerance_ms);
            let top_end = top_bullet.1 + tolerance_ms;

            if bottom_bullet.0 <= top_end && bottom_bullet.1 >= top_start {
                temporally_consistent += 1;
            }
        }
    }

    // Classify pairing quality
    let quality = if utterances_with_markers == 0 {
        PairingQuality::None
    } else if top_end_count == 0 && bottom_end_count == 0 {
        PairingQuality::OpenOnly
    } else {
        let balance = (top_begin_count as i64 - bottom_begin_count as i64).unsigned_abs();
        if balance <= 2 && top_end_count > 0 && bottom_end_count > 0 {
            PairingQuality::FullyPaired
        } else {
            PairingQuality::Mixed
        }
    };

    FileAuditResult {
        path: path.to_string(),
        total_utterances: utt_infos.len(),
        utterances_with_markers,
        top_begin_count,
        top_end_count,
        bottom_begin_count,
        bottom_end_count,
        cross_speaker_pairs,
        timed_pairs,
        temporally_consistent,
        quality,
    }
}

/// Per-corpus aggregate statistics.
#[derive(Debug, Default)]
pub struct CorpusStats {
    pub total_files: usize,
    pub files_with_markers: usize,
    pub fully_paired: usize,
    pub mixed: usize,
    pub open_only: usize,
    pub total_cross_pairs: usize,
    pub total_timed_pairs: usize,
    pub total_temporally_consistent: usize,
}

/// Print TSV header for per-file results.
pub fn print_header() {
    println!(
        "file\tutterances\twith_markers\t⌈\t⌉\t⌊\t⌋\tcross_pairs\ttimed_pairs\ttemporal_ok\tquality"
    );
}

/// Print one file's result as TSV.
pub fn print_result(r: &FileAuditResult) {
    let name = Path::new(&r.path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    println!(
        "{name}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        r.total_utterances,
        r.utterances_with_markers,
        r.top_begin_count,
        r.top_end_count,
        r.bottom_begin_count,
        r.bottom_end_count,
        r.cross_speaker_pairs,
        r.timed_pairs,
        r.temporally_consistent,
        r.quality,
    );
}

/// Print corpus-level summary.
pub fn print_corpus_summary(name: &str, stats: &CorpusStats) {
    println!("\n=== {name} ===");
    println!("  Files total:        {}", stats.total_files);
    println!("  Files with markers: {}", stats.files_with_markers);
    println!(
        "  Fully paired:       {} ({:.0}%)",
        stats.fully_paired,
        if stats.files_with_markers > 0 {
            stats.fully_paired as f64 / stats.files_with_markers as f64 * 100.0
        } else {
            0.0
        }
    );
    println!("  Mixed:              {}", stats.mixed);
    println!("  Open only:          {}", stats.open_only);
    println!("  Cross-speaker pairs: {}", stats.total_cross_pairs);
    println!("  Timed pairs:        {}", stats.total_timed_pairs);
    if stats.total_timed_pairs > 0 {
        println!(
            "  Temporally consistent: {} ({:.0}%)",
            stats.total_temporally_consistent,
            stats.total_temporally_consistent as f64 / stats.total_timed_pairs as f64 * 100.0
        );
    }
}

/// Accumulate a file result into corpus stats.
pub fn accumulate(stats: &mut CorpusStats, result: &FileAuditResult) {
    stats.total_files += 1;
    if result.utterances_with_markers > 0 {
        stats.files_with_markers += 1;
    }
    match result.quality {
        PairingQuality::FullyPaired => stats.fully_paired += 1,
        PairingQuality::Mixed => stats.mixed += 1,
        PairingQuality::OpenOnly => stats.open_only += 1,
        PairingQuality::None => {}
    }
    stats.total_cross_pairs += result.cross_speaker_pairs;
    stats.total_timed_pairs += result.timed_pairs;
    stats.total_temporally_consistent += result.temporally_consistent;
}

/// Group file results by subcorpus directory.
///
/// Finds the longest common prefix of all paths and strips it, then groups by
/// the first 1-2 remaining path components. For example, if all files are under
/// `/Users/chen/talkbank/data/ca-data/`, the subcorpus names will be
/// `CallFriend/spa`, `MOVIN`, `SBCSAE`, etc.
pub fn group_by_subcorpus(results: &[FileAuditResult]) -> BTreeMap<String, Vec<&FileAuditResult>> {
    if results.is_empty() {
        return BTreeMap::new();
    }

    // Find common prefix of all paths
    let paths: Vec<&str> = results.iter().map(|r| r.path.as_str()).collect();
    let common_prefix = {
        let first = paths[0];
        let mut prefix_len = first.len();
        for path in &paths[1..] {
            let common = first
                .bytes()
                .zip(path.bytes())
                .take_while(|(a, b)| a == b)
                .count();
            prefix_len = prefix_len.min(common);
        }
        // Trim to last directory separator
        let prefix = &first[..prefix_len];
        match prefix.rfind('/') {
            Some(pos) => pos + 1,
            None => 0,
        }
    };

    let mut groups: BTreeMap<String, Vec<&FileAuditResult>> = BTreeMap::new();
    for r in results {
        let relative = &r.path[common_prefix..];
        let components: Vec<&str> = relative.split('/').collect();
        // Take up to 2 directory components (subcorpus/sub-subcorpus)
        let subcorpus = if components.len() >= 3 {
            format!("{}/{}", components[0], components[1])
        } else if components.len() >= 2 {
            components[0].to_string()
        } else {
            "root".to_string()
        };
        groups.entry(subcorpus).or_default().push(r);
    }
    groups
}
