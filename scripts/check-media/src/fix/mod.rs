//! Fix subcommand: apply mutations to CHAT files.
//!
//! All mutations are file-local. No git operations are performed.

mod add_unlinked;
mod fix_corpus;

use crate::cli::FixMutation;

/// Entry point for the `fix` subcommand.
pub fn run(mutation: FixMutation) -> Result<bool, Box<dyn std::error::Error>> {
    match mutation {
        FixMutation::AddUnlinked { paths, manifest, dry_run } => {
            add_unlinked::run(&paths, &manifest, dry_run)
        }
        FixMutation::FixCorpus { paths, dry_run } => {
            fix_corpus::run(&paths, dry_run)
        }
    }
}
