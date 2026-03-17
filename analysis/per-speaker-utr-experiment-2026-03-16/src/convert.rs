//! Convert `&*SPK:word` overlap markers to separate `+<` utterances.
//!
//! Walks the CHAT AST, extracts `OtherSpokenEvent` nodes from host utterances,
//! and inserts new `+<`-linked utterances for each extracted backchannel.
//! The host utterance's content, dependent tiers, and bullet are preserved.

use talkbank_model::model::{
    BracketedItem, ChatFile, Linker, Line, MainTier, OtherSpokenEvent, Terminator, Utterance,
    UtteranceContent, Word,
};
use talkbank_model::Span;

/// Result summary from one conversion pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertResult {
    /// Number of `&*` markers extracted and converted to `+<` utterances.
    pub converted: usize,
    /// Number of host utterances that had `&*` markers removed.
    pub hosts_modified: usize,
}

/// Convert all `&*` overlap markers in a CHAT file to separate `+<` utterances.
///
/// For each `OtherSpokenEvent` found in any utterance's content (including inside
/// groups), the marker is removed from the host and a new utterance is inserted
/// immediately after the host with:
/// - Speaker code from the `&*` marker
/// - `+<` lazy overlap linker
/// - Words split from the underscore-joined text
/// - Period terminator
/// - No dependent tiers or bullet
/// Convert all `&*` overlap markers, adding `+<` linker to new utterances.
pub fn convert_overlaps(chat: &mut ChatFile) -> ConvertResult {
    convert_overlaps_inner(chat, true)
}

/// Convert all `&*` overlap markers WITHOUT adding `+<` linker.
///
/// Produces plain separate utterances with no overlap signal. Used in the
/// experiment to demonstrate what happens when backchannels are separate
/// utterances but the aligner has no way to know they overlap.
pub fn convert_overlaps_no_linker(chat: &mut ChatFile) -> ConvertResult {
    convert_overlaps_inner(chat, false)
}

fn convert_overlaps_inner(chat: &mut ChatFile, add_linker: bool) -> ConvertResult {
    let mut result = ConvertResult {
        converted: 0,
        hosts_modified: 0,
    };

    // Collect insertions: (line_index, vec of new utterances to insert after)
    let mut insertions: Vec<(usize, Vec<Utterance>)> = Vec::new();

    for (line_idx, line) in chat.lines.iter_mut().enumerate() {
        let Line::Utterance(utt) = line else {
            continue;
        };

        // Extract OtherSpokenEvents from the content list (and groups)
        let extracted = extract_and_remove_events(&mut utt.main.content.content.0);

        if extracted.is_empty() {
            continue;
        }

        result.hosts_modified += 1;
        result.converted += extracted.len();

        let new_utts: Vec<Utterance> = extracted
            .iter()
            .map(|event| build_overlap_utterance(event, add_linker))
            .collect();

        insertions.push((line_idx, new_utts));
    }

    // Insert in reverse order to preserve line indices
    for (line_idx, new_utts) in insertions.into_iter().rev() {
        for (offset, utt) in new_utts.into_iter().enumerate() {
            chat.lines
                .insert(line_idx + 1 + offset, Line::Utterance(Box::new(utt)));
        }
    }

    result
}

/// Extract and remove all `OtherSpokenEvent` items from a content list,
/// recursing into groups.
fn extract_and_remove_events(content: &mut Vec<UtteranceContent>) -> Vec<OtherSpokenEvent> {
    let mut extracted = Vec::new();

    // First, recurse into groups to extract events from inside brackets
    for item in content.iter_mut() {
        match item {
            UtteranceContent::Group(group) => {
                extracted.extend(extract_and_remove_from_bracketed(&mut group.content.content.0));
            }
            UtteranceContent::AnnotatedGroup(annotated) => {
                extracted
                    .extend(extract_and_remove_from_bracketed(&mut annotated.inner.content.content.0));
            }
            _ => {}
        }
    }

    // Then extract top-level OtherSpokenEvent items
    let mut i = 0;
    while i < content.len() {
        if matches!(&content[i], UtteranceContent::OtherSpokenEvent(_)) {
            let UtteranceContent::OtherSpokenEvent(event) = content.remove(i) else {
                unreachable!()
            };
            extracted.push(event);
        } else {
            i += 1;
        }
    }

    extracted
}

/// Extract and remove `OtherSpokenEvent` items from inside bracketed content.
fn extract_and_remove_from_bracketed(items: &mut Vec<BracketedItem>) -> Vec<OtherSpokenEvent> {
    let mut extracted = Vec::new();

    // Recurse into nested annotated groups (groups inside brackets must have annotations)
    for item in items.iter_mut() {
        if let BracketedItem::AnnotatedGroup(ann) = item {
            extracted.extend(extract_and_remove_from_bracketed(
                &mut ann.inner.content.content.0,
            ));
        }
    }

    // Extract top-level OtherSpokenEvent items from this bracket level
    let mut i = 0;
    while i < items.len() {
        if matches!(&items[i], BracketedItem::OtherSpokenEvent(_)) {
            let BracketedItem::OtherSpokenEvent(event) = items.remove(i) else {
                unreachable!()
            };
            extracted.push(event);
        } else {
            i += 1;
        }
    }

    extracted
}

/// Build a new utterance from an extracted `OtherSpokenEvent`.
///
/// The event's text field is split on `_` to recover individual words.
/// When `add_linker` is true, the `+<` lazy overlap linker is added.
fn build_overlap_utterance(event: &OtherSpokenEvent, add_linker: bool) -> Utterance {
    let words: Vec<UtteranceContent> = event
        .text
        .split('_')
        .map(|w| UtteranceContent::Word(Box::new(Word::simple(w))))
        .collect();

    let mut main = MainTier::new(
        event.speaker.clone(),
        words,
        Terminator::Period {
            span: Span::DUMMY,
        },
    );

    if add_linker {
        main = main.with_linker(Linker::LazyOverlapPrecedes);
    }

    Utterance::new(main)
}

#[cfg(test)]
mod tests {
    use super::*;
    use talkbank_model::errors::NullErrorSink;

    fn parse_chat(text: &str) -> ChatFile {
        talkbank_parser::parse_chat_file_streaming(text, &NullErrorSink)
    }

    /// Count utterances in a ChatFile.
    fn count_utterances(chat: &ChatFile) -> usize {
        chat.lines
            .iter()
            .filter(|l| matches!(l, Line::Utterance(_)))
            .count()
    }

    /// Get the nth utterance's serialized main tier text.
    fn utt_text(chat: &ChatFile, idx: usize) -> String {
        let mut utt_idx = 0;
        for line in &chat.lines {
            if let Line::Utterance(utt) = line {
                if utt_idx == idx {
                    return utt.main.to_string();
                }
                utt_idx += 1;
            }
        }
        panic!("utterance {idx} not found");
    }

    /// Check that a specific utterance has a `+<` linker.
    fn has_lazy_overlap_linker(chat: &ChatFile, idx: usize) -> bool {
        let mut utt_idx = 0;
        for line in &chat.lines {
            if let Line::Utterance(utt) = line {
                if utt_idx == idx {
                    return utt
                        .main
                        .content
                        .linkers
                        .0
                        .contains(&Linker::LazyOverlapPrecedes);
                }
                utt_idx += 1;
            }
        }
        false
    }

    /// Check that a specific utterance has a bullet.
    fn has_bullet(chat: &ChatFile, idx: usize) -> bool {
        let mut utt_idx = 0;
        for line in &chat.lines {
            if let Line::Utterance(utt) = line {
                if utt_idx == idx {
                    return utt.main.content.bullet.is_some();
                }
                utt_idx += 1;
            }
        }
        false
    }

    fn chat_header() -> &'static str {
        "@UTF8\n@Begin\n@Languages:\teng\n@Participants:\tPAR Participant, INV Investigator\n@ID:\teng|test|PAR|||||Participant|||\n@ID:\teng|test|INV|||||Investigator|||\n"
    }

    fn chat_header_3spk() -> &'static str {
        "@UTF8\n@Begin\n@Languages:\teng\n@Participants:\tPAR Participant, INV Investigator, REL Relative\n@ID:\teng|test|PAR|||||Participant|||\n@ID:\teng|test|INV|||||Investigator|||\n@ID:\teng|test|REL|||||Relative|||\n"
    }

    // === Test 1: single &* at beginning ===
    #[test]
    fn single_beginning() {
        let input = format!("{chat}*PAR:\t&*INV:mhm and then I went .\n@End\n", chat = chat_header());
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 1);
        assert_eq!(result.hosts_modified, 1);
        assert_eq!(count_utterances(&chat), 2);

        // PAR's utterance should no longer contain &*
        let par_text = utt_text(&chat, 0);
        assert!(!par_text.contains("&*"), "PAR should not contain &*: {par_text}");
        assert!(par_text.contains("and then I went"), "PAR words preserved: {par_text}");

        // INV's new utterance should have +< and "mhm"
        let inv_text = utt_text(&chat, 1);
        assert!(inv_text.contains("+<"), "INV should have +<: {inv_text}");
        assert!(inv_text.contains("mhm"), "INV should contain mhm: {inv_text}");
    }

    // === Test 2: single &* in middle ===
    #[test]
    fn single_middle() {
        let input = format!("{chat}*PAR:\tI went &*INV:mhm to the store .\n@End\n", chat = chat_header());
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 1);
        assert_eq!(count_utterances(&chat), 2);

        let par_text = utt_text(&chat, 0);
        assert!(!par_text.contains("&*"), "PAR clean: {par_text}");

        assert!(has_lazy_overlap_linker(&chat, 1));
    }

    // === Test 3: single &* at end ===
    #[test]
    fn single_end() {
        let input = format!("{chat}*PAR:\tI went to the store &*INV:mhm .\n@End\n", chat = chat_header());
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 1);
        assert_eq!(count_utterances(&chat), 2);

        let par_text = utt_text(&chat, 0);
        assert!(!par_text.contains("&*"), "PAR clean: {par_text}");
    }

    // === Test 4: multi-word &* (underscore splitting) ===
    #[test]
    fn multi_word() {
        let input = format!("{chat}*PAR:\tI went &*INV:oh_okay .\n@End\n", chat = chat_header());
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 1);

        let inv_text = utt_text(&chat, 1);
        assert!(inv_text.contains("oh"), "should have 'oh': {inv_text}");
        assert!(inv_text.contains("okay"), "should have 'okay': {inv_text}");
        // Should NOT contain underscore
        assert!(!inv_text.contains('_'), "underscore should be split: {inv_text}");
    }

    // === Test 5: multiple same-speaker (Davida's case) ===
    #[test]
    fn multiple_same_speaker() {
        let input = format!(
            "{chat}*PAR:\tbut I grew up in Princeton &*INV:oh_okay_yeah and came to graduate school &*INV:mhm at Chapel_Hill &*INV:oh in ninety one &*INV:mhm or maybe ninety two .\n@End\n",
            chat = chat_header()
        );
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 4);
        assert_eq!(result.hosts_modified, 1);
        assert_eq!(count_utterances(&chat), 5); // 1 PAR + 4 INV

        // PAR should be clean
        let par_text = utt_text(&chat, 0);
        assert!(!par_text.contains("&*"), "PAR clean: {par_text}");
        assert!(par_text.contains("Princeton"), "PAR content preserved");

        // All 4 INV utterances should have +<
        for i in 1..=4 {
            assert!(has_lazy_overlap_linker(&chat, i), "INV utt {i} should have +<");
        }
    }

    // === Test 6: multiple different speakers ===
    #[test]
    fn multiple_different_speakers() {
        let input = format!(
            "{chat}*PAR:\tI &*INV:mhm went &*REL:yeah .\n@End\n",
            chat = chat_header_3spk()
        );
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 2);
        assert_eq!(count_utterances(&chat), 3);
    }

    // === Test 7: host with timing bullet ===
    #[test]
    fn with_timing_bullet() {
        let input = format!(
            "{chat}@Media:\ttest, audio\n*PAR:\tI &*INV:mhm went . \u{15}1000_5000\u{15}\n@End\n",
            chat = chat_header()
        );
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 1);
        // PAR keeps its bullet
        assert!(has_bullet(&chat, 0), "PAR should keep bullet");
        // INV has no bullet
        assert!(!has_bullet(&chat, 1), "INV should not have bullet");
    }

    // === Test 8: without timing bullet ===
    #[test]
    fn without_timing_bullet() {
        let input = format!("{chat}*PAR:\tI &*INV:mhm went .\n@End\n", chat = chat_header());
        let mut chat = parse_chat(&input);
        convert_overlaps(&mut chat);

        assert!(!has_bullet(&chat, 0));
        assert!(!has_bullet(&chat, 1));
    }

    // === Test 9: no overlap markers (noop) ===
    #[test]
    fn no_overlap_markers() {
        let input = format!("{chat}*PAR:\tI went to the store .\n@End\n", chat = chat_header());
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 0);
        assert_eq!(result.hosts_modified, 0);
        assert_eq!(count_utterances(&chat), 1);
    }

    // === Test 12: consecutive host utterances ===
    #[test]
    fn consecutive_host_utts() {
        let input = format!(
            "{chat}*PAR:\tI went &*INV:mhm .\n*PAR:\tto the store &*INV:yeah .\n@End\n",
            chat = chat_header()
        );
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 2);
        assert_eq!(result.hosts_modified, 2);
        assert_eq!(count_utterances(&chat), 4); // 2 PAR + 2 INV

        // Each INV follows its host PAR
        // Utt 0: PAR, Utt 1: INV (+< mhm), Utt 2: PAR, Utt 3: INV (+< yeah)
        assert!(has_lazy_overlap_linker(&chat, 1));
        assert!(has_lazy_overlap_linker(&chat, 3));
    }

    // === Test 13: reverse direction (PAR embedded in INV) ===
    #[test]
    fn reverse_direction() {
        let input = format!(
            "{chat}*INV:\tdo you &*PAR:yeah know .\n@End\n",
            chat = chat_header()
        );
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 1);
        assert_eq!(count_utterances(&chat), 2);

        // INV is host (idx 0), PAR is new (idx 1)
        let inv_text = utt_text(&chat, 0);
        assert!(!inv_text.contains("&*"), "INV clean: {inv_text}");
        assert!(inv_text.contains("do you"), "INV content preserved");

        let par_text = utt_text(&chat, 1);
        assert!(par_text.contains("yeah"), "PAR has yeah: {par_text}");
        assert!(has_lazy_overlap_linker(&chat, 1));
    }

    // === Test 14: with possessive ===
    #[test]
    fn with_possessive() {
        let input = format!(
            "{chat}*INV:\t&*PAR:oh_my_brother's and .\n@End\n",
            chat = chat_header()
        );
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 1);

        let par_text = utt_text(&chat, 1);
        assert!(par_text.contains("oh"), "has oh: {par_text}");
        assert!(par_text.contains("my"), "has my: {par_text}");
        assert!(par_text.contains("brother's"), "has brother's: {par_text}");
    }

    // === Test 15: preserves dependent tiers ===
    #[test]
    fn preserves_dependent_tiers() {
        let input = format!(
            "{chat}*PAR:\tI &*INV:mhm went .\n%mor:\tpro|I verb|go&PAST .\n@End\n",
            chat = chat_header()
        );
        let mut chat = parse_chat(&input);
        let result = convert_overlaps(&mut chat);

        assert_eq!(result.converted, 1);

        // PAR (idx 0) should still have dependent tiers
        let mut utt_idx = 0;
        for line in &chat.lines {
            if let Line::Utterance(utt) = line {
                if utt_idx == 0 {
                    assert!(
                        !utt.dependent_tiers.is_empty(),
                        "PAR should keep dependent tiers"
                    );
                } else if utt_idx == 1 {
                    assert!(
                        utt.dependent_tiers.is_empty(),
                        "INV should have no dependent tiers"
                    );
                }
                utt_idx += 1;
            }
        }
    }

    // === Test: no-linker mode ===
    #[test]
    fn no_linker_mode() {
        let input = format!("{chat}*PAR:\tI went &*INV:mhm to the store .\n@End\n", chat = chat_header());
        let mut chat = parse_chat(&input);
        let result = convert_overlaps_no_linker(&mut chat);

        assert_eq!(result.converted, 1);
        assert_eq!(count_utterances(&chat), 2);

        // INV's utterance should NOT have +< linker
        assert!(!has_lazy_overlap_linker(&chat, 1), "should not have +< in no-linker mode");

        // But should still have the content
        let inv_text = utt_text(&chat, 1);
        assert!(inv_text.contains("mhm"), "INV should have mhm: {inv_text}");
    }

    // === Test 16: preserves other linkers on host ===
    #[test]
    fn preserves_other_linkers() {
        let input = format!(
            "{chat}*PAR:\t++ I went &*INV:mhm .\n@End\n",
            chat = chat_header()
        );
        let mut chat = parse_chat(&input);
        convert_overlaps(&mut chat);

        // PAR should still have ++ linker
        let mut utt_idx = 0;
        for line in &chat.lines {
            if let Line::Utterance(utt) = line {
                if utt_idx == 0 {
                    assert!(
                        utt.main.content.linkers.0.contains(&Linker::OtherCompletion),
                        "PAR should keep ++ linker"
                    );
                }
                utt_idx += 1;
            }
        }

        // INV should have +< linker
        assert!(has_lazy_overlap_linker(&chat, 1));
    }
}
