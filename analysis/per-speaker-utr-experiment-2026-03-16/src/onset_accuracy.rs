//! Experiment A validation: proportional onset estimation accuracy.
//!
//! For each cross-speaker overlap pair (⌈ on line N, ⌊ on line N+1) where
//! both utterances have timing bullets, computes:
//! 1. Estimated overlap onset from ⌈ word position (proportional interpolation)
//! 2. Actual overlap onset from ⌊ utterance's start time
//! 3. Error = estimated - actual (signed, in milliseconds)
//!
//! This validates whether the proportional onset estimate is accurate enough
//! to narrow the pass-2 recovery window (target: ±500ms).

use talkbank_model::alignment::helpers::overlap::extract_overlap_info;
use talkbank_model::model::{ChatFile, Line};

/// One cross-speaker overlap pair measurement.
#[derive(Debug)]
pub struct OnsetMeasurement {
    /// File path (for reporting).
    pub file: String,
    /// ⌈ utterance index (0-based among utterances).
    pub top_utt_idx: usize,
    /// ⌊ utterance index.
    pub bottom_utt_idx: usize,
    /// Top speaker code.
    pub top_speaker: String,
    /// Bottom speaker code.
    pub bottom_speaker: String,
    /// Top utterance bullet: (start_ms, end_ms).
    pub top_bullet: (u64, u64),
    /// Bottom utterance bullet: (start_ms, end_ms).
    pub bottom_bullet: (u64, u64),
    /// Onset fraction from ⌈ position (0.0–1.0).
    pub onset_fraction: f64,
    /// Estimated onset ms from proportional interpolation.
    pub estimated_onset_ms: u64,
    /// Actual onset ms (⌊ utterance start time).
    pub actual_onset_ms: u64,
    /// Signed error: estimated - actual (positive = overestimate).
    pub error_ms: i64,
    /// Absolute error.
    pub abs_error_ms: u64,
    /// Would a ±500ms tight window around the estimate capture the actual onset?
    pub within_500ms: bool,
    /// Would a ±1000ms window capture it?
    pub within_1000ms: bool,
    /// Would a ±2000ms window capture it?
    pub within_2000ms: bool,
}

/// Collect onset measurements from a CHAT file.
pub fn measure_file(chat: &ChatFile, path: &str) -> Vec<OnsetMeasurement> {
    let mut measurements = Vec::new();

    // Collect per-utterance info
    struct UttInfo {
        speaker: String,
        bullet: Option<(u64, u64)>,
        onset_fraction: Option<f64>,
        has_bottom_begin: bool,
    }

    let mut utt_infos: Vec<UttInfo> = Vec::new();

    for line in &chat.lines {
        let Line::Utterance(utt) = line else {
            continue;
        };

        let overlap = extract_overlap_info(&utt.main.content.content.0);
        let bullet = utt
            .main
            .content
            .bullet
            .as_ref()
            .map(|b| (b.timing.start_ms, b.timing.end_ms));

        utt_infos.push(UttInfo {
            speaker: utt.main.speaker.to_string(),
            bullet,
            onset_fraction: overlap.top_onset_fraction(),
            has_bottom_begin: overlap.has_bottom_overlap(),
        });
    }

    // Find cross-speaker pairs with timing
    for i in 0..utt_infos.len().saturating_sub(1) {
        let top = &utt_infos[i];
        let bottom = &utt_infos[i + 1];

        // Need: ⌈ on top, ⌊ on bottom, different speakers, both timed
        let Some(onset_fraction) = top.onset_fraction else {
            continue;
        };
        if !bottom.has_bottom_begin {
            continue;
        }
        if top.speaker == bottom.speaker {
            continue;
        }
        let Some(top_bullet) = top.bullet else {
            continue;
        };
        let Some(bottom_bullet) = bottom.bullet else {
            continue;
        };

        // Compute estimated onset
        let top_duration = top_bullet.1.saturating_sub(top_bullet.0);
        let estimated_onset_ms = top_bullet.0 + (onset_fraction * top_duration as f64) as u64;
        let actual_onset_ms = bottom_bullet.0;

        let error_ms = estimated_onset_ms as i64 - actual_onset_ms as i64;
        let abs_error_ms = error_ms.unsigned_abs();

        measurements.push(OnsetMeasurement {
            file: path.to_string(),
            top_utt_idx: i,
            bottom_utt_idx: i + 1,
            top_speaker: top.speaker.clone(),
            bottom_speaker: bottom.speaker.clone(),
            top_bullet,
            bottom_bullet,
            onset_fraction,
            estimated_onset_ms,
            actual_onset_ms,
            error_ms,
            abs_error_ms,
            within_500ms: abs_error_ms <= 500,
            within_1000ms: abs_error_ms <= 1000,
            within_2000ms: abs_error_ms <= 2000,
        });
    }

    measurements
}

/// Print TSV header for per-measurement output.
pub fn print_header() {
    println!(
        "file\ttop_idx\tbot_idx\ttop_spk\tbot_spk\ttop_start\ttop_end\tbot_start\tbot_end\tfraction\test_onset\tactual_onset\terror_ms\tabs_error\t<=500ms\t<=1s\t<=2s"
    );
}

/// Print one measurement as TSV.
pub fn print_measurement(m: &OnsetMeasurement) {
    let name = std::path::Path::new(&m.file)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    println!(
        "{name}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.3}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        m.top_utt_idx,
        m.bottom_utt_idx,
        m.top_speaker,
        m.bottom_speaker,
        m.top_bullet.0,
        m.top_bullet.1,
        m.bottom_bullet.0,
        m.bottom_bullet.1,
        m.onset_fraction,
        m.estimated_onset_ms,
        m.actual_onset_ms,
        m.error_ms,
        m.abs_error_ms,
        if m.within_500ms { "Y" } else { "N" },
        if m.within_1000ms { "Y" } else { "N" },
        if m.within_2000ms { "Y" } else { "N" },
    );
}

/// Summary statistics for a collection of measurements.
pub struct OnsetStats {
    pub count: usize,
    pub within_500ms: usize,
    pub within_1000ms: usize,
    pub within_2000ms: usize,
    pub median_abs_error: u64,
    pub p90_abs_error: u64,
    pub p95_abs_error: u64,
    pub mean_abs_error: f64,
    pub mean_signed_error: f64,
}

/// Compute summary statistics from measurements.
pub fn compute_stats(measurements: &[OnsetMeasurement]) -> OnsetStats {
    if measurements.is_empty() {
        return OnsetStats {
            count: 0,
            within_500ms: 0,
            within_1000ms: 0,
            within_2000ms: 0,
            median_abs_error: 0,
            p90_abs_error: 0,
            p95_abs_error: 0,
            mean_abs_error: 0.0,
            mean_signed_error: 0.0,
        };
    }

    let count = measurements.len();
    let within_500ms = measurements.iter().filter(|m| m.within_500ms).count();
    let within_1000ms = measurements.iter().filter(|m| m.within_1000ms).count();
    let within_2000ms = measurements.iter().filter(|m| m.within_2000ms).count();

    let mut abs_errors: Vec<u64> = measurements.iter().map(|m| m.abs_error_ms).collect();
    abs_errors.sort();

    let median_abs_error = abs_errors[count / 2];
    let p90_abs_error = abs_errors[(count as f64 * 0.9) as usize];
    let p95_abs_error = abs_errors[(count as f64 * 0.95) as usize];

    let mean_abs_error = abs_errors.iter().sum::<u64>() as f64 / count as f64;
    let mean_signed_error =
        measurements.iter().map(|m| m.error_ms as f64).sum::<f64>() / count as f64;

    OnsetStats {
        count,
        within_500ms,
        within_1000ms,
        within_2000ms,
        median_abs_error,
        p90_abs_error,
        p95_abs_error,
        mean_abs_error,
        mean_signed_error,
    }
}

/// Print summary statistics.
pub fn print_stats(label: &str, stats: &OnsetStats) {
    println!("\n=== {label} ({} measurements) ===", stats.count);
    if stats.count == 0 {
        println!("  No measurements available.");
        return;
    }
    println!(
        "  Within ±500ms:  {} ({:.1}%)",
        stats.within_500ms,
        stats.within_500ms as f64 / stats.count as f64 * 100.0
    );
    println!(
        "  Within ±1000ms: {} ({:.1}%)",
        stats.within_1000ms,
        stats.within_1000ms as f64 / stats.count as f64 * 100.0
    );
    println!(
        "  Within ±2000ms: {} ({:.1}%)",
        stats.within_2000ms,
        stats.within_2000ms as f64 / stats.count as f64 * 100.0
    );
    println!("  Median |error|: {}ms", stats.median_abs_error);
    println!("  p90 |error|:    {}ms", stats.p90_abs_error);
    println!("  p95 |error|:    {}ms", stats.p95_abs_error);
    println!("  Mean |error|:   {:.0}ms", stats.mean_abs_error);
    println!("  Mean signed:    {:.0}ms (positive = overestimate)", stats.mean_signed_error);
}
