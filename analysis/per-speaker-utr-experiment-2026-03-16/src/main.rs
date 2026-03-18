//! Per-speaker UTR simulation experiment tool.
//!
//! Uses the talkbank-parser and talkbank-model crates directly.
//!
//! Subcommands:
//! - `measure`:  Count timed/untimed utterances per speaker in a CHAT file.
//! - `split`:    Split a CHAT file into single-speaker files.
//! - `strip`:    Remove all timing (bullets, inline timing, %wor tiers).
//! - `convert`:  Convert `&*SPK:word` overlap markers to separate `+<` utterances.

mod compare_timing;
mod convert;
mod decompose;
mod onset_accuracy;
mod overlap_audit;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use talkbank_model::alignment::helpers::{for_each_leaf_mut, ContentLeafMut};
use talkbank_model::errors::NullErrorSink;
use talkbank_model::model::dependent_tier::DependentTier;
use talkbank_model::model::{ChatFile, Line};
use talkbank_model::Header;

fn parse_chat(path: &PathBuf) -> ChatFile {
    let source = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("ERROR: cannot read {}: {e}", path.display());
        std::process::exit(1);
    });
    talkbank_parser::parse_chat_file_streaming(&source, &NullErrorSink)
}

fn write_chat(file: &ChatFile, path: &PathBuf) {
    let text = file.to_string();
    fs::write(path, text).unwrap_or_else(|e| {
        eprintln!("ERROR: cannot write {}: {e}", path.display());
        std::process::exit(1);
    });
}

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

#[derive(Parser)]
#[command(name = "utr-experiment", about = "Per-speaker UTR simulation tools")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Count timed/untimed utterances per speaker.
    Measure {
        /// CHAT file(s) or directories to measure.
        #[arg(required = true)]
        paths: Vec<PathBuf>,
    },

    /// Split a CHAT file into single-speaker files.
    Split {
        /// Input CHAT file.
        input: PathBuf,
        /// Output directory (created if needed).
        output_dir: PathBuf,
    },

    /// Strip all timing from a CHAT file (bullets, inline timing, %wor tiers).
    Strip {
        /// Input CHAT file.
        input: PathBuf,
        /// Output CHAT file.
        output: PathBuf,
    },

    /// Compare UTR strategies at each pipeline stage (UTR → grouping → FA groups).
    Decompose {
        /// Input CHAT file (with timing stripped).
        input: PathBuf,
        /// ASR tokens JSON file (from BATCHALIGN_DEBUG_DIR).
        tokens: PathBuf,
        /// Total audio duration in milliseconds (from ffprobe).
        #[arg(long)]
        audio_ms: Option<u64>,
        /// Max FA group duration in milliseconds (default: 15000).
        #[arg(long, default_value_t = 15000)]
        max_group_ms: u64,
    },

    /// Convert &*SPK:word overlap markers to separate +< utterances.
    Convert {
        /// Input CHAT file.
        input: PathBuf,
        /// Output CHAT file.
        output: PathBuf,
        /// Omit the +< linker on converted utterances (plain separate utterances).
        #[arg(long)]
        no_linker: bool,
    },

    /// Audit CA overlap markers (⌈⌉⌊⌋) for pairing quality and temporal consistency.
    OverlapAudit {
        /// CHAT file(s) or directories to audit.
        #[arg(required = true)]
        paths: Vec<PathBuf>,
        /// Print per-file details (default: only print corpus summary).
        #[arg(long)]
        verbose: bool,
    },

    /// Measure proportional onset estimation accuracy (Experiment A validation).
    ///
    /// For each cross-speaker overlap pair with timing, compares the estimated
    /// onset (from ⌈ word position) against the actual onset (⌊ utterance start).
    OnsetAccuracy {
        /// CHAT file(s) or directories to analyze.
        #[arg(required = true)]
        paths: Vec<PathBuf>,
        /// Print per-measurement details (default: only print summary).
        #[arg(long)]
        verbose: bool,
    },

    /// Compare recovered timing against ground truth (per-utterance).
    CompareTiming {
        /// Ground truth directory.
        #[arg(long)]
        gt: PathBuf,
        /// Recovered timing directories to compare (e.g., global and two-pass).
        #[arg(required = true)]
        recovered: Vec<PathBuf>,
    },
}

// ---------------------------------------------------------------------------
// measure
// ---------------------------------------------------------------------------

struct SpeakerCoverage {
    timed: u32,
    untimed: u32,
}

fn measure_file(path: &PathBuf) -> BTreeMap<String, SpeakerCoverage> {
    let chat = parse_chat(path);
    let mut by_speaker: BTreeMap<String, SpeakerCoverage> = BTreeMap::new();

    for line in &chat.lines {
        let Line::Utterance(utt) = line else {
            continue;
        };
        let speaker = utt.main.speaker.to_string();
        let has_bullet = utt.main.content.bullet.is_some();

        let entry = by_speaker
            .entry(speaker)
            .or_insert(SpeakerCoverage { timed: 0, untimed: 0 });
        if has_bullet {
            entry.timed += 1;
        } else {
            entry.untimed += 1;
        }
    }
    by_speaker
}

fn run_measure(paths: &[PathBuf]) {
    let mut files: Vec<PathBuf> = Vec::new();
    for p in paths {
        if p.is_dir() {
            if let Ok(entries) = fs::read_dir(p) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|ext| ext == "cha") {
                        files.push(path);
                    }
                }
            }
        } else {
            files.push(p.clone());
        }
    }
    files.sort();

    println!("file\tspeaker\ttotal\ttimed\tuntimed\tcoverage");
    for path in &files {
        let by_speaker = measure_file(path);
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();

        let mut all_timed: u32 = 0;
        let mut all_untimed: u32 = 0;

        for (spk, cov) in &by_speaker {
            let total = cov.timed + cov.untimed;
            let pct = if total > 0 {
                cov.timed as f64 / total as f64 * 100.0
            } else {
                0.0
            };
            println!("{stem}\t{spk}\t{total}\t{}\t{}\t{pct:.1}%", cov.timed, cov.untimed);
            all_timed += cov.timed;
            all_untimed += cov.untimed;
        }

        let all_total = all_timed + all_untimed;
        let all_pct = if all_total > 0 {
            all_timed as f64 / all_total as f64 * 100.0
        } else {
            0.0
        };
        println!("{stem}\tALL\t{all_total}\t{all_timed}\t{all_untimed}\t{all_pct:.1}%");
    }
}

// ---------------------------------------------------------------------------
// split
// ---------------------------------------------------------------------------

fn run_split(input: &PathBuf, output_dir: &PathBuf) {
    let chat = parse_chat(input);
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();

    // Collect participant codes from @Participants header.
    let mut participant_codes: Vec<String> = Vec::new();
    for line in &chat.lines {
        if let Line::Header { header, .. } = line {
            if let Header::Participants { entries } = header.as_ref() {
                for entry in &entries.0 {
                    participant_codes.push(entry.speaker_code.to_string());
                }
            }
        }
    }

    if participant_codes.is_empty() {
        eprintln!("ERROR: no @Participants header found");
        std::process::exit(1);
    }

    fs::create_dir_all(output_dir).unwrap_or_else(|e| {
        eprintln!("ERROR: cannot create {}: {e}", output_dir.display());
        std::process::exit(1);
    });

    for target_code in &participant_codes {
        let mut out = chat.clone();

        // Filter lines: keep headers (with modifications) + target speaker's utterances.
        out.lines.retain(|line| match line {
            Line::Header { header, .. } => {
                match header.as_ref() {
                    // Drop other speakers' @ID lines.
                    Header::ID(id_header) => id_header.speaker.as_ref() == target_code.as_str(),
                    // Keep all other headers.
                    _ => true,
                }
            }
            Line::Utterance(utt) => utt.main.speaker.as_ref() == target_code.as_str(),
        });

        // Rewrite @Participants to include only the target speaker.
        for line in &mut out.lines {
            if let Line::Header { header, .. } = line {
                if let Header::Participants { entries } = header.as_mut() {
                    entries
                        .0
                        .retain(|e| e.speaker_code.as_ref() == target_code.as_str());
                }
            }
        }

        let out_path = output_dir.join(format!("{stem}_{target_code}.cha"));
        let utt_count = out
            .lines
            .iter()
            .filter(|l| matches!(l, Line::Utterance(_)))
            .count();
        write_chat(&out, &out_path);
        println!("  {target_code}: {utt_count} utterances -> {}", out_path.display());
    }
}

// ---------------------------------------------------------------------------
// strip
// ---------------------------------------------------------------------------

fn run_strip(input: &PathBuf, output: &PathBuf) {
    let mut chat = parse_chat(input);

    let mut stripped_count: u32 = 0;
    let mut total_utts: u32 = 0;

    for line in &mut chat.lines {
        let Line::Utterance(utt) = line else {
            continue;
        };
        total_utts += 1;

        if utt.main.content.bullet.is_some() {
            stripped_count += 1;
        }

        // Strip utterance-level bullet.
        utt.main.content.bullet = None;

        // Strip inline word-level bullets using the content walker.
        for_each_leaf_mut(&mut utt.main.content.content.0, None, &mut |leaf| {
            match leaf {
                ContentLeafMut::Word(w, _) => {
                    w.inline_bullet = None;
                }
                ContentLeafMut::ReplacedWord(r) => {
                    r.word.inline_bullet = None;
                }
                ContentLeafMut::Separator(_) => {}
            }
        });

        // Remove %wor tiers.
        utt.dependent_tiers
            .retain(|t| !matches!(t, DependentTier::Wor(_)));
    }

    write_chat(&chat, output);
    println!(
        "Stripped {stripped_count}/{total_utts} utterance bullets -> {}",
        output.display()
    );
}

// ---------------------------------------------------------------------------
// decompose
// ---------------------------------------------------------------------------

fn run_decompose(input: &PathBuf, tokens_path: &PathBuf, audio_ms: Option<u64>, max_group_ms: u64) {
    let chat_text = fs::read_to_string(input).unwrap_or_else(|e| {
        eprintln!("ERROR: cannot read {}: {e}", input.display());
        std::process::exit(1);
    });
    let tokens_json = fs::read_to_string(tokens_path).unwrap_or_else(|e| {
        eprintln!("ERROR: cannot read {}: {e}", tokens_path.display());
        std::process::exit(1);
    });
    let tokens: Vec<batchalign_chat_ops::fa::utr::AsrTimingToken> =
        serde_json::from_str(&tokens_json).unwrap_or_else(|e| {
            eprintln!("ERROR: invalid tokens JSON: {e}");
            std::process::exit(1);
        });

    let (global, two_pass) = decompose::decompose(&chat_text, &tokens, audio_ms, max_group_ms);
    decompose::print_comparison(&global, &two_pass);
}

// ---------------------------------------------------------------------------
// convert
// ---------------------------------------------------------------------------

fn run_convert(input: &PathBuf, output: &PathBuf, no_linker: bool) {
    let mut chat = parse_chat(input);
    let result = if no_linker {
        convert::convert_overlaps_no_linker(&mut chat)
    } else {
        convert::convert_overlaps(&mut chat)
    };

    write_chat(&chat, output);
    let linker_note = if no_linker { " (no +< linker)" } else { "" };
    println!(
        "Converted {} &* markers from {} utterances{} -> {}",
        result.converted,
        result.hosts_modified,
        linker_note,
        output.display()
    );
}

// ---------------------------------------------------------------------------
// overlap-audit
// ---------------------------------------------------------------------------

fn collect_cha_files(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    for p in paths {
        if p.is_dir() {
            collect_cha_files_recursive(p, &mut files);
        } else if p.extension().is_some_and(|ext| ext == "cha") {
            files.push(p.clone());
        }
    }
    files.sort();
    files
}

fn collect_cha_files_recursive(dir: &PathBuf, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_cha_files_recursive(&path, files);
            } else if path.extension().is_some_and(|ext| ext == "cha") {
                files.push(path);
            }
        }
    }
}

fn run_overlap_audit(paths: &[PathBuf], verbose: bool) {
    let files = collect_cha_files(paths);
    if files.is_empty() {
        eprintln!("No .cha files found");
        std::process::exit(1);
    }

    eprintln!("Auditing {} files for CA overlap markers...", files.len());

    let mut results: Vec<overlap_audit::FileAuditResult> = Vec::new();

    if verbose {
        overlap_audit::print_header();
    }

    for path in &files {
        let chat = parse_chat(path);
        let display_path = path.to_string_lossy().to_string();
        let result = overlap_audit::audit_file(&chat, &display_path);

        // Only include files that actually have overlap markers
        if result.utterances_with_markers > 0 {
            if verbose {
                overlap_audit::print_result(&result);
            }
            results.push(result);
        }
    }

    // Group by subcorpus and print summaries
    let groups = overlap_audit::group_by_subcorpus(&results);
    let mut overall = overlap_audit::CorpusStats::default();

    for (subcorpus, file_results) in &groups {
        let mut stats = overlap_audit::CorpusStats::default();
        for r in file_results {
            overlap_audit::accumulate(&mut stats, r);
            overlap_audit::accumulate(&mut overall, r);
        }
        overlap_audit::print_corpus_summary(subcorpus, &stats);
    }

    overlap_audit::print_corpus_summary("OVERALL", &overall);
}

// ---------------------------------------------------------------------------
// compare-timing
// ---------------------------------------------------------------------------

fn run_compare_timing(gt_dir: &PathBuf, recovered_dirs: &[PathBuf]) {
    let gt_files = collect_cha_files(&[gt_dir.clone()]);
    if gt_files.is_empty() {
        eprintln!("No .cha files found in ground truth dir");
        std::process::exit(1);
    }

    for rec_dir in recovered_dirs {
        let label = rec_dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut comparisons = Vec::new();
        for gt_path in &gt_files {
            let base = gt_path.file_name().unwrap();
            let rec_path = rec_dir.join(base);
            if !rec_path.exists() {
                continue;
            }
            let gt = parse_chat(gt_path);
            let rec = parse_chat(&rec_path);
            let display = base.to_string_lossy().to_string();
            comparisons.push(compare_timing::compare_files(&gt, &rec, &display));
        }

        compare_timing::print_comparison(&label, &comparisons);
    }
}

// ---------------------------------------------------------------------------
// onset-accuracy
// ---------------------------------------------------------------------------

fn run_onset_accuracy(paths: &[PathBuf], verbose: bool) {
    let files = collect_cha_files(paths);
    if files.is_empty() {
        eprintln!("No .cha files found");
        std::process::exit(1);
    }

    eprintln!("Measuring onset accuracy across {} files...", files.len());

    let mut all_measurements: Vec<onset_accuracy::OnsetMeasurement> = Vec::new();

    if verbose {
        onset_accuracy::print_header();
    }

    for path in &files {
        let chat = parse_chat(path);
        let display_path = path.to_string_lossy().to_string();
        let measurements = onset_accuracy::measure_file(&chat, &display_path);

        if verbose {
            for m in &measurements {
                onset_accuracy::print_measurement(m);
            }
        }

        all_measurements.extend(measurements);
    }

    // Overall statistics
    let stats = onset_accuracy::compute_stats(&all_measurements);
    onset_accuracy::print_stats("OVERALL", &stats);

    // Per-file statistics for files with measurements
    if !verbose {
        // Group by file and show per-file summaries
        let mut by_file: std::collections::BTreeMap<String, Vec<&onset_accuracy::OnsetMeasurement>> =
            std::collections::BTreeMap::new();
        for m in &all_measurements {
            let name = std::path::Path::new(&m.file)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            by_file.entry(name).or_default().push(m);
        }

        println!("\n--- Per-file summary ---");
        println!("file\tcount\t<=500ms%\t<=1s%\tmedian_err\tp90_err");
        for (name, measurements) in &by_file {
            let owned: Vec<onset_accuracy::OnsetMeasurement> = measurements
                .iter()
                .map(|m| onset_accuracy::OnsetMeasurement {
                    file: m.file.clone(),
                    top_utt_idx: m.top_utt_idx,
                    bottom_utt_idx: m.bottom_utt_idx,
                    top_speaker: m.top_speaker.clone(),
                    bottom_speaker: m.bottom_speaker.clone(),
                    top_bullet: m.top_bullet,
                    bottom_bullet: m.bottom_bullet,
                    onset_fraction: m.onset_fraction,
                    estimated_onset_ms: m.estimated_onset_ms,
                    actual_onset_ms: m.actual_onset_ms,
                    error_ms: m.error_ms,
                    abs_error_ms: m.abs_error_ms,
                    within_500ms: m.within_500ms,
                    within_1000ms: m.within_1000ms,
                    within_2000ms: m.within_2000ms,
                })
                .collect();
            let s = onset_accuracy::compute_stats(&owned);
            if s.count > 0 {
                println!(
                    "{name}\t{}\t{:.0}%\t{:.0}%\t{}ms\t{}ms",
                    s.count,
                    s.within_500ms as f64 / s.count as f64 * 100.0,
                    s.within_1000ms as f64 / s.count as f64 * 100.0,
                    s.median_abs_error,
                    s.p90_abs_error,
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Measure { paths } => run_measure(paths),
        Command::Split { input, output_dir } => run_split(input, output_dir),
        Command::Strip { input, output } => run_strip(input, output),
        Command::Decompose {
            input,
            tokens,
            audio_ms,
            max_group_ms,
        } => run_decompose(input, tokens, *audio_ms, *max_group_ms),
        Command::Convert {
            input,
            output,
            no_linker,
        } => run_convert(input, output, *no_linker),
        Command::OverlapAudit { paths, verbose } => run_overlap_audit(paths, *verbose),
        Command::OnsetAccuracy { paths, verbose } => run_onset_accuracy(paths, *verbose),
        Command::CompareTiming { gt, recovered } => run_compare_timing(gt, recovered),
    }
}
