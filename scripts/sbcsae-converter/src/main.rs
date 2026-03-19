mod bracket;
mod diagnostics;
mod emit_chat;
mod encoding;
mod format;
mod infer;
mod intermediate;
mod overlap;
mod speakers;
mod trn_content;
mod types;

use std::path::{Path, PathBuf};

use clap::Parser;

use crate::bracket::tokenize_brackets;
use crate::diagnostics::Diagnostics;
use crate::emit_chat::{emit_chat_file, group_into_utterances, time_to_ms};
use crate::format::{detect_format, parse_lines};
use crate::overlap::OverlapState;
use crate::speakers::build_speaker_map;
use crate::trn_content::parse_trn_content;
use crate::types::*;

#[derive(Parser)]
#[command(name = "trn-overlap-extract")]
#[command(about = "Extract overlap correspondence data from SBCSAE .trn files")]
struct Cli {
    /// TRN files or directories to process.
    #[arg(required = true)]
    input: Vec<PathBuf>,

    /// Output directory for files. If omitted, prints to stdout.
    #[arg(short, long)]
    output_dir: Option<PathBuf>,

    /// Emit CHAT (.cha) files instead of JSON.
    #[arg(long)]
    chat: bool,

    /// Emit intermediate TrnDocument as JSON (no overlap inference).
    #[arg(long)]
    intermediate: bool,

    /// Emit CHAT via the new pipeline: TrnDocument → OverlapAssignment → CHAT.
    #[arg(long)]
    doc_chat: bool,

    /// Print progress to stderr.
    #[arg(short, long)]
    verbose: bool,

    /// Only emit diagnostics, no data output.
    #[arg(long)]
    diagnostics_only: bool,
}

fn main() {
    let cli = Cli::parse();

    let files = collect_trn_files(&cli.input);

    if files.is_empty() {
        eprintln!("No .trn files found.");
        std::process::exit(1);
    }

    if cli.verbose {
        eprintln!("Processing {} files...", files.len());
    }

    if let Some(ref dir) = cli.output_dir {
        std::fs::create_dir_all(dir).expect("Failed to create output directory");
    }

    let mut total_runs = 0;
    let mut total_diagnostics = 0;

    for path in &files {
        let result = process_file(path, cli.verbose);
        total_runs += result.json_output.overlap_runs.len();
        total_diagnostics += result.json_output.diagnostics.len();

        if cli.diagnostics_only {
            for d in &result.json_output.diagnostics {
                eprintln!(
                    "{}: L{}{}: [{:?}] {}",
                    result.json_output.filename,
                    d.line_number,
                    d.column.map_or(String::new(), |c| format!(":{c}")),
                    d.code,
                    d.message,
                );
            }
            continue;
        }

        let stem = path.file_stem().unwrap().to_str().unwrap();

        if cli.intermediate {
            // Emit TrnDocument JSON — no overlap inference.
            let mut diag = Diagnostics::new();
            if let Some(doc) = intermediate::build_document(path, &mut diag) {
                let json = serde_json::to_string_pretty(&doc).expect("JSON serialization failed");
                match cli.output_dir {
                    Some(ref dir) => {
                        let out_path = dir.join(format!("{stem}.doc.json"));
                        std::fs::write(&out_path, &json).expect("Failed to write output");
                        if cli.verbose {
                            eprintln!(
                                "  {} → {} ({} utterances, {} brackets, {} alignment edges)",
                                path.display(), out_path.display(),
                                doc.utterances.len(), doc.brackets.len(), doc.alignment_edges.len(),
                            );
                        }
                    }
                    None => println!("{json}"),
                }
            }
            continue;
        }

        if cli.doc_chat {
            // New pipeline: TrnDocument → global inference → CHAT.
            let mut diag = Diagnostics::new();
            if let Some(doc) = intermediate::build_document(path, &mut diag) {
                let assignment = infer::infer_overlaps_global(&doc);
                let num = stem.trim_start_matches("SBC").trim_start_matches('0');
                let chat_file_stem = if num.is_empty() { "00" } else { num };
                let chat = emit_chat::emit_chat_from_doc(chat_file_stem, &doc, &assignment);

                match cli.output_dir {
                    Some(ref dir) => {
                        let chat_name = format!("{:02}.cha", num.parse::<u32>().unwrap_or(0));
                        let out_path = dir.join(&chat_name);
                        std::fs::write(&out_path, &chat).expect("Failed to write CHAT output");
                        if cli.verbose {
                            eprintln!(
                                "  {} → {} ({} brackets, {} roles assigned)",
                                path.display(), out_path.display(),
                                doc.brackets.len(), assignment.roles.len(),
                            );
                        }
                    }
                    None => print!("{chat}"),
                }
            }
            continue;
        }

        if cli.chat {
            let chat = &result.chat_output;
            match cli.output_dir {
                Some(ref dir) => {
                    // File number from stem: SBC002 → 02
                    let num = stem.trim_start_matches("SBC").trim_start_matches('0');
                    let chat_name = format!("{:02}.cha", num.parse::<u32>().unwrap_or(0));
                    let out_path = dir.join(&chat_name);
                    std::fs::write(&out_path, chat).expect("Failed to write CHAT output");
                    if cli.verbose {
                        eprintln!("  {} → {}", path.display(), out_path.display());
                    }
                }
                None => print!("{chat}"),
            }
        } else {
            let json = serde_json::to_string_pretty(&result.json_output)
                .expect("JSON serialization failed");
            match cli.output_dir {
                Some(ref dir) => {
                    let out_path = dir.join(format!("{stem}.json"));
                    std::fs::write(&out_path, &json).expect("Failed to write JSON output");
                    if cli.verbose {
                        eprintln!(
                            "  {} → {} ({} runs, {} diagnostics)",
                            path.display(),
                            out_path.display(),
                            result.json_output.overlap_runs.len(),
                            result.json_output.diagnostics.len(),
                        );
                    }
                }
                None => println!("{json}"),
            }
        }
    }

    if cli.verbose {
        eprintln!(
            "Done. {total_runs} overlap runs, {total_diagnostics} diagnostics across {} files.",
            files.len()
        );
    }
}

struct ProcessResult {
    json_output: FileOutput,
    chat_output: String,
}

fn process_file(path: &Path, verbose: bool) -> ProcessResult {
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
    if verbose {
        eprintln!("  Processing {filename}...");
    }

    let mut diag = Diagnostics::new();

    // Stage 1: Read and decode.
    let text = match encoding::read_and_decode(path, &mut diag) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading {}: {e}", path.display());
            return ProcessResult {
                json_output: FileOutput {
                    filename,
                    format_variant: FormatVariant::A,
                    total_lines: 0,
                    speaker_map: Default::default(),
                    overlap_runs: Vec::new(),
                    diagnostics: diag.into_vec(),
                },
                chat_output: String::new(),
            };
        }
    };

    // Stage 2: Detect format and parse lines.
    let first_line = text.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    let variant = detect_format(first_line);
    let lines = parse_lines(&text, variant, &mut diag);

    // Stage 3 + 4: Tokenize brackets, run overlap state machine,
    // and collect per-line bracket classifications.
    let mut state = OverlapState::new();
    let mut current_turn_speaker: Option<String> = None;

    // Per-line: Vec of (char_offset, role, real_index).
    let mut line_bracket_classifications: Vec<Vec<(usize, OverlapRole, usize)>> = Vec::new();

    for line in &lines {
        if line.speaker.is_some() && line.speaker != current_turn_speaker {
            state.reset_seen();
            current_turn_speaker = line.speaker.clone();
        }

        let tokens = tokenize_brackets(&line.raw_content, line.line_number, line.content_column);
        let mut classifications = Vec::new();

        for token in tokens {
            let (role, real_index) = match token.kind {
                BracketKind::Open => {
                    let role = state.add_begin(token.clone(), &line.effective_speaker, &mut diag, &lines);
                    // Determine real_index from the current run's last set.
                    let real_idx = state.last_classified_index();
                    (role, real_idx)
                }
                BracketKind::Close => {
                    let role = state.add_end(token.clone(), &line.effective_speaker, false, &mut diag, &lines);
                    let real_idx = lexical_to_approx_index(token.lexical_index);
                    (role, real_idx)
                }
                BracketKind::CloseForced => {
                    let role = state.add_end(token.clone(), &line.effective_speaker, true, &mut diag, &lines);
                    let real_idx = lexical_to_approx_index(token.lexical_index);
                    (role, real_idx)
                }
            };
            classifications.push((token.char_offset, role, real_index));
        }

        line_bracket_classifications.push(classifications);
    }

    let overlap_runs = state.finish(&mut diag, &lines);

    // Stage 5: Build speaker map.
    let speakers_in_order: Vec<String> = lines
        .iter()
        .filter_map(|l| l.speaker.clone())
        .collect::<Vec<_>>();
    let mut seen = std::collections::HashSet::new();
    let unique_speakers: Vec<String> = speakers_in_order
        .into_iter()
        .filter(|s| seen.insert(s.clone()))
        .collect();
    let speaker_map = build_speaker_map(&unique_speakers, &mut diag);

    // Stage 6: Parse TRN content into elements.
    let line_elements: Vec<Vec<_>> = lines
        .iter()
        .zip(line_bracket_classifications.iter())
        .map(|(line, classifications)| {
            parse_trn_content(&line.raw_content, classifications)
        })
        .collect();

    // Stage 7: Group into utterances and emit CHAT.
    let utterances = group_into_utterances(&lines, &line_elements);

    // Compute file stem for CHAT (e.g., "SBC002" → "02").
    let chat_stem = stem
        .trim_start_matches("SBC")
        .trim_start_matches('0');
    let chat_file_stem = if chat_stem.is_empty() { "00" } else { chat_stem };

    let chat_output = emit_chat_file(chat_file_stem, &speaker_map, &utterances);

    let json_output = FileOutput {
        filename,
        format_variant: variant,
        total_lines: lines.len(),
        speaker_map,
        overlap_runs,
        diagnostics: diag.into_vec(),
    };

    ProcessResult {
        json_output,
        chat_output,
    }
}

/// Approximate real index from lexical index (for close brackets where
/// we don't have direct access to the run state).
fn lexical_to_approx_index(lexical: Option<u8>) -> usize {
    match lexical {
        None => 0,
        Some(n) => (n - 1) as usize,
    }
}

fn collect_trn_files(inputs: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for input in inputs {
        if input.is_dir() {
            if let Ok(entries) = std::fs::read_dir(input) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.extension().is_some_and(|e| e.eq_ignore_ascii_case("trn")) {
                        files.push(p);
                    }
                }
            }
        } else if input.extension().is_some_and(|e| e.eq_ignore_ascii_case("trn")) {
            files.push(input.clone());
        }
    }
    files.sort();
    files
}
