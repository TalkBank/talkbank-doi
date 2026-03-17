//! Stage decomposition: analyze how UTR output affects FA grouping.
//!
//! For a given CHAT file + ASR tokens, shows what happens at each pipeline
//! stage under different UTR strategies:
//!
//! 1. UTR injection (how many utterances get bullets, which ones)
//! 2. Boundary estimation (how untimed utterances get estimated timing)
//! 3. FA grouping (how utterances are split into alignment groups)
//!
//! This does NOT run actual FA inference — it shows the grouping that
//! would be sent to the FA worker, which is where the cascade originates.

use batchalign_chat_ops::fa::{
    GlobalUtr, TwoPassOverlapUtr, UtrStrategy,
    count_utterance_timing, group_utterances,
    utr::AsrTimingToken,
};
use talkbank_model::model::Line;

/// Summary of one UTR + grouping stage decomposition.
#[derive(Debug)]
pub struct StageReport {
    pub strategy_name: String,
    pub utr_injected: usize,
    pub utr_skipped: usize,
    pub utr_unmatched: usize,
    pub timed_after_utr: usize,
    pub untimed_after_utr: usize,
    pub num_fa_groups: usize,
    pub total_fa_words: usize,
    pub utterances_in_groups: usize,
    /// Per-group: (start_ms, end_ms, num_utterances, num_words)
    pub groups: Vec<(u64, u64, usize, usize)>,
    /// Per-utterance: (speaker, has_bullet, is_overlap, text_preview)
    pub utterance_bullets: Vec<(String, bool, bool, String)>,
}

/// Run stage decomposition for both strategies on the same input.
pub fn decompose(
    chat_text: &str,
    asr_tokens: &[AsrTimingToken],
    total_audio_ms: Option<u64>,
    max_group_ms: u64,
) -> (StageReport, StageReport) {
    let global_report = decompose_strategy(
        "global",
        chat_text,
        asr_tokens,
        total_audio_ms,
        max_group_ms,
        &GlobalUtr,
    );

    let two_pass_report = decompose_strategy(
        "two-pass",
        chat_text,
        asr_tokens,
        total_audio_ms,
        max_group_ms,
        &TwoPassOverlapUtr::new(),
    );

    (global_report, two_pass_report)
}

fn decompose_strategy(
    name: &str,
    chat_text: &str,
    asr_tokens: &[AsrTimingToken],
    total_audio_ms: Option<u64>,
    max_group_ms: u64,
    strategy: &dyn UtrStrategy,
) -> StageReport {
    // Parse fresh
    let (mut chat, _) = batchalign_chat_ops::parse::parse_lenient(chat_text);

    // Stage 1: UTR injection
    let utr_result = strategy.inject(&mut chat, asr_tokens);

    let (timed, untimed) = count_utterance_timing(&chat);

    // Collect per-utterance bullet info
    let mut utterance_bullets = Vec::new();
    for line in &chat.lines {
        if let Line::Utterance(utt) = line {
            let has_bullet = utt.main.content.bullet.is_some();
            let is_overlap = utt
                .main
                .content
                .linkers
                .0
                .iter()
                .any(|l| matches!(l, talkbank_model::model::Linker::LazyOverlapPrecedes));
            let text: String = utt.main.to_string().chars().take(60).collect();
            utterance_bullets.push((
                utt.main.speaker.to_string(),
                has_bullet,
                is_overlap,
                text,
            ));
        }
    }

    // Stage 2 + 3: FA grouping (includes estimate_untimed_boundaries internally)
    let groups = group_utterances(&chat, max_group_ms, total_audio_ms);

    let total_fa_words: usize = groups.iter().map(|g| g.words.len()).sum();
    let utterances_in_groups: usize = groups.iter().map(|g| g.utterance_indices.len()).sum();

    let group_info: Vec<(u64, u64, usize, usize)> = groups
        .iter()
        .map(|g| {
            (
                g.audio_start_ms(),
                g.audio_end_ms(),
                g.utterance_indices.len(),
                g.words.len(),
            )
        })
        .collect();

    StageReport {
        strategy_name: name.to_string(),
        utr_injected: utr_result.injected,
        utr_skipped: utr_result.skipped,
        utr_unmatched: utr_result.unmatched,
        timed_after_utr: timed + utr_result.injected,
        untimed_after_utr: untimed.saturating_sub(utr_result.injected),
        num_fa_groups: groups.len(),
        total_fa_words,
        utterances_in_groups,
        groups: group_info,
        utterance_bullets,
    }
}

/// Print a comparison of two stage reports.
pub fn print_comparison(a: &StageReport, b: &StageReport) {
    println!("=== Stage Decomposition: {} vs {} ===", a.strategy_name, b.strategy_name);
    println!();

    // UTR stage
    println!("Stage 1: UTR Injection");
    println!(
        "  {:>10}  injected={:<4} skipped={:<4} unmatched={:<4}  → timed={} untimed={}",
        a.strategy_name, a.utr_injected, a.utr_skipped, a.utr_unmatched,
        a.timed_after_utr, a.untimed_after_utr,
    );
    println!(
        "  {:>10}  injected={:<4} skipped={:<4} unmatched={:<4}  → timed={} untimed={}",
        b.strategy_name, b.utr_injected, b.utr_skipped, b.utr_unmatched,
        b.timed_after_utr, b.untimed_after_utr,
    );

    // Diff utterance bullets
    let mut bullet_diffs = 0;
    let mut overlap_diffs = 0;
    for (i, (ua, ub)) in a.utterance_bullets.iter().zip(b.utterance_bullets.iter()).enumerate() {
        if ua.1 != ub.1 {
            bullet_diffs += 1;
            let marker = if ua.2 || ub.2 { " [+<]" } else { "" };
            println!(
                "    utt {}: {} bullet: {} → {}{}  ({})",
                i, ua.0,
                if ua.1 { "YES" } else { "no " },
                if ub.1 { "YES" } else { "no " },
                marker,
                &ua.3[..ua.3.len().min(50)],
            );
        }
        if ua.2 && ua.1 != ub.1 {
            overlap_diffs += 1;
        }
    }
    println!("  Bullet diffs: {} total, {} on +< utterances", bullet_diffs, overlap_diffs);

    // FA grouping stage
    println!();
    println!("Stage 2+3: FA Grouping");
    println!(
        "  {:>10}  groups={:<4} words={:<6} utts_in_groups={}",
        a.strategy_name, a.num_fa_groups, a.total_fa_words, a.utterances_in_groups,
    );
    println!(
        "  {:>10}  groups={:<4} words={:<6} utts_in_groups={}",
        b.strategy_name, b.num_fa_groups, b.total_fa_words, b.utterances_in_groups,
    );

    if a.num_fa_groups != b.num_fa_groups {
        println!("  *** GROUP COUNT DIFFERS: {} vs {} ***", a.num_fa_groups, b.num_fa_groups);
    }
    if a.utterances_in_groups != b.utterances_in_groups {
        println!(
            "  *** UTTERANCES IN GROUPS DIFFERS: {} vs {} (delta={}) ***",
            a.utterances_in_groups,
            b.utterances_in_groups,
            (a.utterances_in_groups as i64 - b.utterances_in_groups as i64).abs(),
        );
    }

    // Group-by-group comparison
    let max_groups = a.groups.len().max(b.groups.len());
    if max_groups > 0 && a.groups.len() != b.groups.len() {
        println!();
        println!("  Group details (showing first 10 of each):");
        for (label, groups) in [(&a.strategy_name, &a.groups), (&b.strategy_name, &b.groups)] {
            println!("  {}:", label);
            for (i, (start, end, utts, words)) in groups.iter().enumerate().take(10) {
                println!(
                    "    G{}: {}-{}ms ({} utts, {} words)",
                    i, start, end, utts, words
                );
            }
            if groups.len() > 10 {
                println!("    ... ({} more groups)", groups.len() - 10);
            }
        }
    }
}
