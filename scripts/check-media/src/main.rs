//! check-media: validate CHAT transcript ↔ media file correspondence.
//!
//! Replaces the legacy `staging/scripts/chatmedia.py` with a Rust tool that
//! uses a cached media manifest (refreshed explicitly) and typed CHAT header
//! parsing from `talkbank-direct-parser`. Designed to run as a pre-push hook.

mod check;
mod cli;
mod config;
mod diagnostics;
mod extract;
mod fix;
mod manifest;
mod output;

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Commands};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Check {
            paths,
            bank,
            manifest,
            format,
            checks,
            fail_on_error,
            quiet,
        } => check::run(&paths, bank.as_deref(), &manifest, &format, &checks, fail_on_error, quiet),

        Commands::RefreshManifest {
            host,
            media_root,
            bank,
            output,
        } => manifest::refresh::run(&host, &media_root, bank.as_deref(), &output),

        Commands::ShowManifest {
            manifest,
            files,
            bank,
        } => manifest::show::run(&manifest, files, bank.as_deref()),

        Commands::Fix { mutation } => fix::run(mutation),
    };

    match result {
        Ok(has_errors) => {
            if has_errors {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}
