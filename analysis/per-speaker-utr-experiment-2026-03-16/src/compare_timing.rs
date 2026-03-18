//! Direct ground-truth timing comparison for UTR strategy evaluation.
//!
//! For each utterance that has a bullet in the ground truth file:
//! - Does the recovered file also have a bullet?
//! - If so, how close are the start/end times?
//!
//! Special focus on ⌊-bearing (overlap) utterances.

use talkbank_model::alignment::helpers::overlap::extract_overlap_info;
use talkbank_model::model::{ChatFile, Line};

/// Per-utterance timing comparison.
struct UttComparison {
    utt_idx: usize,
    speaker: String,
    has_ca_overlap: bool,
    gt_bullet: Option<(u64, u64)>,
    recovered_bullet: Option<(u64, u64)>,
}

/// Summary of timing comparison for one file.
pub struct FileComparison {
    pub file: String,
    /// Total utterances in the file.
    pub total_utts: usize,
    /// Utterances with ground truth timing.
    pub gt_timed: usize,
    /// Utterances with recovered timing.
    pub recovered_timed: usize,
    /// Utterances where both have timing (can compare).
    pub both_timed: usize,
    /// Of both_timed: start errors (absolute, ms).
    pub start_errors: Vec<u64>,
    /// Of both_timed: end errors (absolute, ms).
    pub end_errors: Vec<u64>,

    // Overlap-specific metrics (⌊-bearing utterances only)
    /// ⌊ utterances with ground truth timing.
    pub overlap_gt_timed: usize,
    /// ⌊ utterances with recovered timing.
    pub overlap_recovered_timed: usize,
    /// ⌊ utterances where both have timing.
    pub overlap_both_timed: usize,
    /// Of overlap_both_timed: start errors.
    pub overlap_start_errors: Vec<u64>,
    /// ⌊ utterances with GT timing but NO recovered timing (regression).
    pub overlap_regressions: usize,
    /// ⌊ utterances with recovered timing but NO GT timing (new coverage).
    pub overlap_new_coverage: usize,
}

/// Compare two CHAT files (ground truth vs recovered).
pub fn compare_files(gt: &ChatFile, recovered: &ChatFile, file: &str) -> FileComparison {
    let mut comparisons: Vec<UttComparison> = Vec::new();

    let gt_utts: Vec<_> = gt
        .lines
        .iter()
        .filter_map(|l| if let Line::Utterance(u) = l { Some(u) } else { None })
        .collect();
    let rec_utts: Vec<_> = recovered
        .lines
        .iter()
        .filter_map(|l| if let Line::Utterance(u) = l { Some(u) } else { None })
        .collect();

    // Match utterances by index (files should have same structure)
    let n = gt_utts.len().min(rec_utts.len());
    for i in 0..n {
        let gt_utt = gt_utts[i];
        let rec_utt = rec_utts[i];

        let overlap = extract_overlap_info(&gt_utt.main.content.content.0);
        let gt_bullet = gt_utt
            .main
            .content
            .bullet
            .as_ref()
            .map(|b| (b.timing.start_ms, b.timing.end_ms));
        let rec_bullet = rec_utt
            .main
            .content
            .bullet
            .as_ref()
            .map(|b| (b.timing.start_ms, b.timing.end_ms));

        comparisons.push(UttComparison {
            utt_idx: i,
            speaker: gt_utt.main.speaker.to_string(),
            has_ca_overlap: overlap.has_bottom_overlap(),
            gt_bullet,
            recovered_bullet: rec_bullet,
        });
    }

    let total_utts = comparisons.len();
    let gt_timed = comparisons.iter().filter(|c| c.gt_bullet.is_some()).count();
    let recovered_timed = comparisons
        .iter()
        .filter(|c| c.recovered_bullet.is_some())
        .count();

    let mut start_errors = Vec::new();
    let mut end_errors = Vec::new();
    let mut both_timed = 0;

    let mut overlap_gt_timed = 0;
    let mut overlap_recovered_timed = 0;
    let mut overlap_both_timed = 0;
    let mut overlap_start_errors = Vec::new();
    let mut overlap_regressions = 0;
    let mut overlap_new_coverage = 0;

    for c in &comparisons {
        if let (Some(gt), Some(rec)) = (c.gt_bullet, c.recovered_bullet) {
            both_timed += 1;
            start_errors.push(gt.0.abs_diff(rec.0));
            end_errors.push(gt.1.abs_diff(rec.1));

            if c.has_ca_overlap {
                overlap_both_timed += 1;
                overlap_start_errors.push(gt.0.abs_diff(rec.0));
            }
        }

        if c.has_ca_overlap {
            if c.gt_bullet.is_some() {
                overlap_gt_timed += 1;
            }
            if c.recovered_bullet.is_some() {
                overlap_recovered_timed += 1;
            }
            if c.gt_bullet.is_some() && c.recovered_bullet.is_none() {
                overlap_regressions += 1;
            }
            if c.gt_bullet.is_none() && c.recovered_bullet.is_some() {
                overlap_new_coverage += 1;
            }
        }
    }

    FileComparison {
        file: file.to_string(),
        total_utts,
        gt_timed,
        recovered_timed,
        both_timed,
        start_errors,
        end_errors,
        overlap_gt_timed,
        overlap_recovered_timed,
        overlap_both_timed,
        overlap_start_errors,
        overlap_regressions,
        overlap_new_coverage,
    }
}

/// Print comparison summary.
pub fn print_comparison(label: &str, comparisons: &[FileComparison]) {
    let total_utts: usize = comparisons.iter().map(|c| c.total_utts).sum();
    let gt_timed: usize = comparisons.iter().map(|c| c.gt_timed).sum();
    let recovered_timed: usize = comparisons.iter().map(|c| c.recovered_timed).sum();
    let both_timed: usize = comparisons.iter().map(|c| c.both_timed).sum();

    let mut all_start_errors: Vec<u64> = comparisons
        .iter()
        .flat_map(|c| c.start_errors.iter().copied())
        .collect();
    all_start_errors.sort();

    let mut all_end_errors: Vec<u64> = comparisons
        .iter()
        .flat_map(|c| c.end_errors.iter().copied())
        .collect();
    all_end_errors.sort();

    let overlap_gt: usize = comparisons.iter().map(|c| c.overlap_gt_timed).sum();
    let overlap_rec: usize = comparisons.iter().map(|c| c.overlap_recovered_timed).sum();
    let overlap_both: usize = comparisons.iter().map(|c| c.overlap_both_timed).sum();
    let overlap_reg: usize = comparisons.iter().map(|c| c.overlap_regressions).sum();
    let overlap_new: usize = comparisons.iter().map(|c| c.overlap_new_coverage).sum();

    let mut overlap_starts: Vec<u64> = comparisons
        .iter()
        .flat_map(|c| c.overlap_start_errors.iter().copied())
        .collect();
    overlap_starts.sort();

    println!("\n=== {label} ===");
    println!("  Utterances:   {total_utts}");
    println!(
        "  GT timed:     {gt_timed} ({:.1}%)",
        gt_timed as f64 / total_utts as f64 * 100.0
    );
    println!(
        "  Recovered:    {recovered_timed} ({:.1}%)",
        recovered_timed as f64 / total_utts as f64 * 100.0
    );
    println!("  Both timed:   {both_timed}");

    if !all_start_errors.is_empty() {
        let n = all_start_errors.len();
        let within_500 = all_start_errors.iter().filter(|&&e| e <= 500).count();
        let within_1000 = all_start_errors.iter().filter(|&&e| e <= 1000).count();
        println!("  --- Start time error (vs ground truth) ---");
        println!(
            "    Median:     {}ms",
            all_start_errors[n / 2]
        );
        println!(
            "    Within 500ms: {} ({:.1}%)",
            within_500,
            within_500 as f64 / n as f64 * 100.0
        );
        println!(
            "    Within 1s:    {} ({:.1}%)",
            within_1000,
            within_1000 as f64 / n as f64 * 100.0
        );
        println!(
            "    p90:        {}ms",
            all_start_errors[(n as f64 * 0.9) as usize]
        );
    }

    println!("  --- ⌊ Overlap utterances ---");
    println!("    GT timed:     {overlap_gt}");
    println!("    Recovered:    {overlap_rec}");
    println!("    Both timed:   {overlap_both}");
    println!("    Regressions:  {overlap_reg} (GT had timing, recovered lost it)");
    println!("    New coverage: {overlap_new} (GT had no timing, recovered added it)");

    if !overlap_starts.is_empty() {
        let n = overlap_starts.len();
        let within_500 = overlap_starts.iter().filter(|&&e| e <= 500).count();
        let within_1000 = overlap_starts.iter().filter(|&&e| e <= 1000).count();
        println!("    --- ⌊ Start error vs GT ---");
        println!(
            "      Median:     {}ms",
            overlap_starts[n / 2]
        );
        println!(
            "      Within 500ms: {} ({:.1}%)",
            within_500,
            within_500 as f64 / n as f64 * 100.0
        );
        println!(
            "      Within 1s:    {} ({:.1}%)",
            within_1000,
            within_1000 as f64 / n as f64 * 100.0
        );
    }
}
