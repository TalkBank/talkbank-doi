mod autonumber;
mod bracket;
mod diagnostics;
mod emit_chat;
mod encoding;
mod format;
mod infer;
mod intermediate;
mod merge;
mod speakers;
mod trn_content;
mod types;

use std::path::PathBuf;

use clap::Parser;

use crate::diagnostics::Diagnostics;

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

    /// Emit CHAT (.cha) files.
    #[arg(long)]
    chat: bool,

    /// Emit intermediate TrnDocument as JSON (no overlap inference).
    #[arg(long)]
    intermediate: bool,

    /// Generate bracket annotation report.
    #[arg(long)]
    report: bool,

    /// Merge TRN indices into existing CHAT files. Requires --chat-dir.
    #[arg(long)]
    merge: bool,

    /// Directory containing hand-edited CHAT files (for --merge).
    #[arg(long)]
    chat_dir: Option<PathBuf>,

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

    for path in &files {
        let mut diag = Diagnostics::new();
        let doc = match intermediate::build_document(path, &mut diag) {
            Some(d) => d,
            None => continue,
        };

        if cli.diagnostics_only {
            for d in &doc.parse_diagnostics {
                eprintln!(
                    "{}: L{}{}: [{:?}] {}",
                    doc.filename,
                    d.line_number,
                    d.column.map_or(String::new(), |c| format!(":{c}")),
                    d.code,
                    d.message,
                );
            }
            continue;
        }

        let stem = path.file_stem().unwrap().to_str().unwrap();

        if cli.merge {
            let chat_dir = cli.chat_dir.as_ref().expect("--merge requires --chat-dir");
            let assignment = infer::infer_overlaps_global(&doc);

            // Find the corresponding CHAT file.
            let num = stem.trim_start_matches("SBC").trim_start_matches('0');
            let chat_name = format!("{:02}.cha", num.parse::<u32>().unwrap_or(0));
            let chat_path = chat_dir.join(&chat_name);

            if !chat_path.exists() {
                eprintln!("  SKIP {}: no CHAT file at {}", doc.filename, chat_path.display());
                continue;
            }

            match merge::merge_indices(&chat_path, &doc, &assignment) {
                Ok(result) => {
                    if cli.verbose {
                        eprintln!(
                            "  {} → {}: {} markers ({} indexed, {} already, {} unmatched)",
                            doc.filename, chat_name,
                            result.markers_found, result.markers_indexed,
                            result.markers_already_indexed, result.markers_unmatched,
                        );
                    }
                    match cli.output_dir {
                        Some(ref dir) => {
                            let out_path = dir.join(&chat_name);
                            std::fs::write(&out_path, &result.updated_content)
                                .expect("Failed to write merged CHAT");
                        }
                        None => print!("{}", result.updated_content),
                    }
                }
                Err(e) => eprintln!("  ERROR {}: {}", chat_name, e),
            }
            continue;
        }

        if cli.report {
            let assignment = infer::infer_overlaps_global(&doc);
            let annotations = autonumber::analyze_brackets(&doc, &assignment);
            let report = autonumber::review_report(&annotations);
            eprintln!("=== {} ===", doc.filename);
            eprint!("{report}");
            continue;
        }

        if cli.intermediate {
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
            continue;
        }

        if cli.chat {
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
            continue;
        }

        // Default: emit TrnDocument JSON.
        let json = serde_json::to_string_pretty(&doc).expect("JSON serialization failed");
        match cli.output_dir {
            Some(ref dir) => {
                let out_path = dir.join(format!("{stem}.doc.json"));
                std::fs::write(&out_path, &json).expect("Failed to write output");
            }
            None => println!("{json}"),
        }
    }

    if cli.verbose {
        eprintln!("Done. {} files processed.", files.len());
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
