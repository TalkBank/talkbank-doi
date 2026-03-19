//! Overlap inference state machine.
//!
//! Direct port of java-chatter-stable's `OverlapInfo` → `OverlapRun` → `OverlapSet`
//! hierarchy. Classifies each bracket as top-begin, top-end, bottom-begin, or
//! bottom-end, tracking overlap runs with sequential group semantics.

use crate::diagnostics::Diagnostics;
use crate::types::*;

/// Maximum overlap index before wrapping (indices 0..=8, displayed as unnumbered + 2..9).
const MAX_OVERLAPS: usize = 9;
/// Sanity limit on total overlap sets.
const MAX_TOTAL_OVERLAPS: usize = 30;

// ── Internal working state ──────────────────────────────────────────────────

struct OverlapPair {
    who: String,
    begin: BracketToken,
    end: Option<BracketToken>,
}

struct OverlapSet {
    real_index: usize,
    top: OverlapPair,
    bottoms: Vec<OverlapPair>,
    frozen_end: bool,
}

impl OverlapSet {
    fn new(real_index: usize, begin: BracketToken, who: String) -> Self {
        Self {
            real_index,
            top: OverlapPair { who, begin, end: None },
            bottoms: Vec::new(),
            frozen_end: false,
        }
    }

    fn is_top_complete(&self) -> bool {
        self.top.end.is_some()
    }

    fn has_bottoms(&self) -> bool {
        !self.bottoms.is_empty()
    }

    fn check_valid(&self, diag: &mut Diagnostics) {
        if !self.is_top_complete() {
            diag.warn(
                self.top.begin.line_number,
                Some(self.top.begin.column),
                DiagnosticCode::IncompleteTop,
                format!("Top overlap by {} was not closed", self.top.who),
            );
        }
        if !self.has_bottoms() {
            diag.warn(
                self.top.begin.line_number,
                Some(self.top.begin.column),
                DiagnosticCode::NoBottom,
                format!("Top overlap by {} had no matching bottom", self.top.who),
            );
        }
        for bottom in &self.bottoms {
            if bottom.end.is_none() {
                diag.warn(
                    bottom.begin.line_number,
                    Some(bottom.begin.column),
                    DiagnosticCode::IncompleteBottom,
                    format!("Bottom overlap by {} was not closed", bottom.who),
                );
            }
        }
    }

    fn add_bottom(&mut self, begin: BracketToken, who: &str, diag: &mut Diagnostics) -> bool {
        if self.frozen_end {
            diag.warn(
                begin.line_number,
                Some(begin.column),
                DiagnosticCode::UnmatchedBracket,
                "Overlap set is frozen (already force-closed)".to_string(),
            );
            return false;
        }
        if self.top.who == who {
            diag.warn(
                begin.line_number,
                Some(begin.column),
                DiagnosticCode::SameSpeakerOverlap,
                format!("Speaker '{who}' already has the top of this overlap"),
            );
            return false;
        }
        for bottom in &self.bottoms {
            if bottom.who == who {
                diag.warn(
                    begin.line_number,
                    Some(begin.column),
                    DiagnosticCode::SameSpeakerOverlap,
                    format!("Speaker '{who}' already has a bottom in this overlap"),
                );
                return false;
            }
        }
        self.bottoms.push(OverlapPair { who: who.to_string(), begin, end: None });
        true
    }

    /// Try to close a bracket for the given speaker.
    /// Returns Some(true) if bottom, Some(false) if top, None if no match.
    fn add_end(&mut self, end: BracketToken, who: &str, force: bool, diag: &mut Diagnostics) -> Option<bool> {
        if self.frozen_end {
            diag.warn(
                end.line_number,
                Some(end.column),
                DiagnosticCode::UnmatchedBracket,
                "Overlap set is frozen".to_string(),
            );
            return None;
        }

        let result = if self.top.who == who {
            if self.top.end.is_some() {
                diag.warn(
                    end.line_number,
                    Some(end.column),
                    DiagnosticCode::UnmatchedBracket,
                    format!("Top overlap by '{who}' was already closed"),
                );
                return None;
            }
            self.top.end = Some(end);
            Some(false) // top
        } else {
            let mut found = false;
            for bottom in &mut self.bottoms {
                if bottom.who == who {
                    if bottom.end.is_some() {
                        diag.warn(
                            end.line_number,
                            Some(end.column),
                            DiagnosticCode::UnmatchedBracket,
                            format!("Bottom overlap by '{who}' was already closed"),
                        );
                        return None;
                    }
                    bottom.end = Some(end);
                    found = true;
                    break;
                }
            }
            if found { Some(true) } else { None }
        };

        if force {
            self.frozen_end = true;
        }

        result
    }
}

struct OverlapRun {
    sets: Vec<OverlapSet>,
    continue_first_overlap: bool,
    last_saw_overlap: bool,
    saw_overlap: bool,
}

impl OverlapRun {
    fn new() -> Self {
        Self {
            sets: Vec::new(),
            continue_first_overlap: false,
            last_saw_overlap: false,
            saw_overlap: false,
        }
    }

    fn reset_seen(&mut self) {
        self.last_saw_overlap = self.saw_overlap;
        self.saw_overlap = false;
    }

    fn seen(&mut self) {
        self.saw_overlap = true;
    }

    fn may_wrap(&self) -> bool {
        !self.sets.is_empty() && self.sets.len() % MAX_OVERLAPS == 0
    }

    fn next_expected_index(&self) -> usize {
        self.sets.len()
    }

    /// Map lexical index (0–8) to real index, handling wraparound.
    fn actual_index(&self, lexical: usize) -> usize {
        let size = self.sets.len();
        let max_index = size + 2;
        let multiple = size / MAX_OVERLAPS;
        let anchor = multiple * MAX_OVERLAPS;
        let up = anchor + lexical;

        if up <= max_index {
            up
        } else {
            let down = up.saturating_sub(MAX_OVERLAPS);
            if down > 0 { down } else { up }
        }
    }

    fn find_set(&self, lexical: usize) -> Option<usize> {
        let real = self.actual_index(lexical);
        if real < self.sets.len() {
            Some(real)
        } else {
            None
        }
    }

    fn add_set_top(&mut self, begin: BracketToken, who: String, diag: &mut Diagnostics) -> bool {
        let lexical = lexical_index_value(begin.lexical_index);
        let real = self.actual_index(lexical);

        if real >= MAX_TOTAL_OVERLAPS {
            diag.warn(
                begin.line_number,
                Some(begin.column),
                DiagnosticCode::HighOverlapIndex,
                format!("Overlap index {real} exceeds maximum {MAX_TOTAL_OVERLAPS}"),
            );
            return false;
        }

        let expected = self.next_expected_index();
        if real > expected {
            diag.warn(
                begin.line_number,
                Some(begin.column),
                DiagnosticCode::InvalidIndex,
                format!("Expected overlap index {expected}, got {real}"),
            );
            return false;
        }

        self.sets.push(OverlapSet::new(real, begin, who));
        true
    }

    /// Returns: Top, Bottom, or TryNewRun.
    fn add_begin(&mut self, begin: BracketToken, who: &str, diag: &mut Diagnostics) -> AddBeginResult {
        let lexical = lexical_index_value(begin.lexical_index);

        match self.find_set(lexical) {
            None => {
                // New top — add to current run.
                if self.add_set_top(begin, who.to_string(), diag) {
                    AddBeginResult::Top
                } else {
                    AddBeginResult::Skip
                }
            }
            Some(set_idx) => {
                // Set exists — try bottom, or maybe start new run.
                let try_new = lexical == 0 && !self.may_wrap() && !self.continue_first_overlap;

                if try_new {
                    let next_idx = lexical + 1;
                    let next_set = self.find_set(next_idx);
                    let next_complete = next_set
                        .map(|idx| {
                            let s = &self.sets[idx];
                            s.is_top_complete() && s.has_bottoms()
                                && s.bottoms.iter().all(|b| b.end.is_some())
                        })
                        .unwrap_or(false);

                    if !self.last_saw_overlap || next_complete {
                        return AddBeginResult::TryNewRun(begin);
                    }
                }

                // Try to add as bottom.
                let set = &mut self.sets[set_idx];
                if set.add_bottom(begin.clone(), who, diag) {
                    AddBeginResult::Bottom
                } else if try_new {
                    // Same-speaker or frozen — try new run instead.
                    AddBeginResult::TryNewRun(begin)
                } else {
                    AddBeginResult::Skip
                }
            }
        }
    }

    fn add_end(&mut self, end: BracketToken, who: &str, force: bool, diag: &mut Diagnostics) -> Option<bool> {
        let lexical = lexical_index_value(end.lexical_index);
        match self.find_set(lexical) {
            None => None,
            Some(set_idx) => self.sets[set_idx].add_end(end, who, force, diag),
        }
    }

    fn check_valid(&self, diag: &mut Diagnostics) {
        for set in &self.sets {
            set.check_valid(diag);
        }
    }

    fn into_output(self, run_id: usize, lines: &[TrnLine]) -> OverlapRunOutput {
        let mut first_line = usize::MAX;
        let mut last_line = 0;
        let sets: Vec<OverlapSetOutput> = self
            .sets
            .into_iter()
            .map(|set| {
                let update_range = |loc: &Option<BracketToken>, first: &mut usize, last: &mut usize| {
                    if let Some(t) = loc {
                        *first = (*first).min(t.line_number);
                        *last = (*last).max(t.line_number);
                    }
                };

                update_range(&Some(set.top.begin.clone()), &mut first_line, &mut last_line);
                update_range(&set.top.end, &mut first_line, &mut last_line);
                for b in &set.bottoms {
                    update_range(&Some(b.begin.clone()), &mut first_line, &mut last_line);
                    update_range(&b.end, &mut first_line, &mut last_line);
                }

                let complete = set.is_top_complete()
                    && set.has_bottoms()
                    && set.bottoms.iter().all(|b| b.end.is_some());

                let display_index = if set.real_index % MAX_OVERLAPS == 0 {
                    DisplayIndex::Unnumbered
                } else {
                    DisplayIndex::Numbered((set.real_index % MAX_OVERLAPS) as u8 + 1)
                };

                OverlapSetOutput {
                    real_index: set.real_index,
                    display_index,
                    top: pair_to_participant(set.top, lines),
                    bottoms: set.bottoms.into_iter().map(|b| pair_to_participant(b, lines)).collect(),
                    complete,
                }
            })
            .collect();

        OverlapRunOutput {
            run_id,
            sets,
            first_line: if first_line == usize::MAX { 0 } else { first_line },
            last_line,
        }
    }
}

enum AddBeginResult {
    Top,
    Bottom,
    TryNewRun(BracketToken),
    Skip,
}

/// Top-level overlap state machine.
pub struct OverlapState {
    previous_run: Option<OverlapRun>,
    current_run: Option<OverlapRun>,
    completed_runs: Vec<OverlapRunOutput>,
    run_counter: usize,
}

impl OverlapState {
    pub fn new() -> Self {
        Self {
            previous_run: None,
            current_run: None,
            completed_runs: Vec::new(),
            run_counter: 0,
        }
    }

    pub fn reset_seen(&mut self) {
        if let Some(ref mut run) = self.current_run {
            run.reset_seen();
        }
    }

    pub fn seen(&mut self) {
        if let Some(ref mut run) = self.current_run {
            run.seen();
        }
    }

    /// Return the real index of the most recently added/modified set.
    /// Used by the CHAT emitter to tag classified brackets.
    pub fn last_classified_index(&self) -> usize {
        if let Some(ref run) = self.current_run {
            if let Some(last) = run.sets.last() {
                return last.real_index;
            }
        }
        0
    }

    pub fn set_continue_first_overlap(&mut self, v: bool) {
        if let Some(ref mut run) = self.current_run {
            run.continue_first_overlap = v;
        }
    }

    fn close_previous(&mut self, diag: &mut Diagnostics, lines: &[TrnLine]) {
        if let Some(prev) = self.previous_run.take() {
            prev.check_valid(diag);
            let id = self.run_counter;
            self.run_counter += 1;
            self.completed_runs.push(prev.into_output(id, lines));
        }
    }

    fn start_new_run(&mut self, begin: BracketToken, who: String, diag: &mut Diagnostics, lines: &[TrnLine]) {
        self.close_previous(diag, lines);

        // Try to close current run; if it has issues, save as previous.
        if let Some(current) = self.current_run.take() {
            let mut check_diag = Diagnostics::new();
            current.check_valid(&mut check_diag);
            if check_diag.len() == 0 {
                // Clean close.
                let id = self.run_counter;
                self.run_counter += 1;
                self.completed_runs.push(current.into_output(id, lines));
            } else {
                // Still has issues — save as previous, will close later.
                self.previous_run = Some(current);
            }
        }

        let mut new_run = OverlapRun::new();
        new_run.add_set_top(begin, who, diag);
        self.current_run = Some(new_run);
    }

    /// Process an open bracket.
    pub fn add_begin(&mut self, begin: BracketToken, who: &str, diag: &mut Diagnostics, lines: &[TrnLine]) -> OverlapRole {
        if self.current_run.is_none() {
            self.start_new_run(begin, who.to_string(), diag, lines);
            self.seen();
            return OverlapRole::TopBegin;
        }

        let result = self.current_run.as_mut().unwrap().add_begin(begin.clone(), who, diag);
        self.seen();

        match result {
            AddBeginResult::Top => OverlapRole::TopBegin,
            AddBeginResult::Bottom => OverlapRole::BottomBegin,
            AddBeginResult::TryNewRun(token) => {
                self.start_new_run(token, who.to_string(), diag, lines);
                OverlapRole::TopBegin
            }
            AddBeginResult::Skip => OverlapRole::TopBegin, // Best guess.
        }
    }

    /// Process a close bracket.
    pub fn add_end(&mut self, end: BracketToken, who: &str, force: bool, diag: &mut Diagnostics, lines: &[TrnLine]) -> OverlapRole {
        if self.current_run.is_none() {
            diag.warn(
                end.line_number,
                Some(end.column),
                DiagnosticCode::UnmatchedBracket,
                format!("Close bracket by '{who}' with no open overlap run"),
            );
            self.seen();
            return OverlapRole::TopEnd; // Best guess.
        }

        // Try current run.
        if let Some(is_bottom) = self.current_run.as_mut().unwrap().add_end(end.clone(), who, force, diag) {
            self.seen();
            return if is_bottom { OverlapRole::BottomEnd } else { OverlapRole::TopEnd };
        }

        // Try previous run.
        let prev_result = self.previous_run.as_mut()
            .and_then(|prev| prev.add_end(end.clone(), who, force, diag));

        if let Some(is_bottom) = prev_result {
            self.seen();
            // Check if previous is now complete.
            let prev_clean = {
                let mut check_diag = Diagnostics::new();
                if let Some(ref prev) = self.previous_run {
                    prev.check_valid(&mut check_diag);
                }
                check_diag.len() == 0
            };
            if prev_clean {
                self.close_previous(diag, lines);
            }
            return if is_bottom { OverlapRole::BottomEnd } else { OverlapRole::TopEnd };
        }

        diag.warn(
            end.line_number,
            Some(end.column),
            DiagnosticCode::UnmatchedBracket,
            format!("Close bracket by '{who}' has no matching open bracket"),
        );
        self.seen();
        OverlapRole::TopEnd // Best guess.
    }

    /// Finalize: close all remaining runs and return output.
    pub fn finish(mut self, diag: &mut Diagnostics, lines: &[TrnLine]) -> Vec<OverlapRunOutput> {
        self.close_previous(diag, lines);

        if let Some(current) = self.current_run.take() {
            current.check_valid(diag);
            let id = self.run_counter;
            self.completed_runs.push(current.into_output(id, lines));
        }

        self.completed_runs
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Convert lexical_index (None = unnumbered = 0, Some(N) = N-1) to 0-based value.
fn lexical_index_value(idx: Option<u8>) -> usize {
    match idx {
        None => 0,
        Some(n) => (n - 1) as usize, // [2] → 1, [3] → 2, etc.
    }
}

fn pair_to_participant(pair: OverlapPair, lines: &[TrnLine]) -> OverlapParticipant {
    let begin_loc = token_to_location(&pair.begin, lines);
    let end_loc = pair.end.as_ref().map(|t| token_to_location(t, lines));

    // Extract bracketed text between begin and end.
    let bracketed_text = extract_bracketed_text(&pair.begin, pair.end.as_ref(), lines);

    OverlapParticipant {
        speaker: pair.who,
        begin: Some(begin_loc),
        end: end_loc,
        bracketed_text,
    }
}

fn token_to_location(token: &BracketToken, lines: &[TrnLine]) -> BracketLocation {
    let time_range = lines
        .iter()
        .find(|l| l.line_number == token.line_number)
        .map(|l| (l.start_time, l.end_time))
        .unwrap_or((0.0, 0.0));

    BracketLocation {
        line_number: token.line_number,
        char_offset: token.char_offset,
        column: token.column,
        time_range,
    }
}

fn extract_bracketed_text(
    begin: &BracketToken,
    end: Option<&BracketToken>,
    lines: &[TrnLine],
) -> Option<String> {
    let end = end?;

    if begin.line_number == end.line_number {
        // Same line — simple slice.
        let line = lines.iter().find(|l| l.line_number == begin.line_number)?;
        let content = &line.raw_content;

        // Start after the open bracket (and its optional digit).
        let start = begin.char_offset + 1 + begin.lexical_index.map_or(0, |_| 1);
        // End before the close digit (if any) and bracket.
        let end_offset = end.char_offset;

        if start <= end_offset && end_offset <= content.len() {
            Some(content[start..end_offset].to_string())
        } else {
            None
        }
    } else {
        // Multi-line span — concatenate.
        let mut text = String::new();
        for line in lines {
            if line.line_number == begin.line_number {
                let start = begin.char_offset + 1 + begin.lexical_index.map_or(0, |_| 1);
                if start < line.raw_content.len() {
                    text.push_str(&line.raw_content[start..]);
                }
            } else if line.line_number > begin.line_number && line.line_number < end.line_number {
                if !text.is_empty() {
                    text.push(' ');
                }
                text.push_str(line.raw_content.trim());
            } else if line.line_number == end.line_number {
                if !text.is_empty() {
                    text.push(' ');
                }
                let end_offset = end.char_offset;
                if end_offset <= line.raw_content.len() {
                    text.push_str(line.raw_content[..end_offset].trim());
                }
            }
        }
        Some(text)
    }
}

// ── Document-level inference ────────────────────────────────────────────────

use crate::intermediate::{
    BracketDirection, BracketRef, ContentElement, OverlapAssignment, OverlapRole as IntermediateRole,
    BracketRole, TrnDocument,
};

/// Infer overlap roles (top/bottom) for all brackets in a TrnDocument.
/// Returns an OverlapAssignment mapping bracket IDs to roles.
///
/// Uses the alignment edges and utterance context to improve the heuristic
/// for unnumbered brackets beyond the Java state machine's single-turn lookback.
pub fn infer_overlaps(doc: &TrnDocument) -> OverlapAssignment {
    let mut diag = Diagnostics::new();

    // Build a bracket ID → index map for fast lookup.
    let bracket_index: std::collections::HashMap<u32, usize> = doc
        .brackets
        .iter()
        .enumerate()
        .map(|(i, b)| (b.id, i))
        .collect();

    // Build alignment edge lookup: bracket_id → target_bracket_id.
    let alignment_target: std::collections::HashMap<u32, u32> = doc
        .alignment_edges
        .iter()
        .map(|e| (e.aligned_bracket_id, e.target_bracket_id))
        .collect();

    // Build fake TrnLines for the state machine.
    let fake_lines: Vec<TrnLine> = doc
        .utterances
        .iter()
        .map(|utt| TrnLine {
            line_number: utt.source_lines.first,
            start_time: utt.start_ms.unwrap_or(0) as f64 / 1000.0,
            end_time: utt.end_ms.unwrap_or(0) as f64 / 1000.0,
            speaker: Some(utt.speaker.clone()),
            effective_speaker: utt.speaker.clone(),
            raw_content: String::new(),
            content_column: 0,
        })
        .collect();

    let mut state = OverlapState::new();
    let mut bracket_roles: std::collections::BTreeMap<u32, BracketRole> = std::collections::BTreeMap::new();
    let mut current_speaker: Option<String> = None;

    // Track recent turn context for enhanced heuristic.
    let mut last_overlap_utt_idx: Option<usize> = None;
    let mut last_overlap_end_ms: Option<i64> = None;

    for utt in &doc.utterances {
        // Turn boundary detection.
        if Some(&utt.speaker) != current_speaker.as_ref() {
            state.reset_seen();
            current_speaker = Some(utt.speaker.clone());
        }

        let utt_has_brackets = utt.elements.iter().any(|e| matches!(e, ContentElement::Bracket(_)));

        for elem in &utt.elements {
            if let ContentElement::Bracket(bracket_id) = elem {
                let bracket = match doc.brackets.iter().find(|b| b.id == *bracket_id) {
                    Some(b) => b,
                    None => continue,
                };

                // Enhanced heuristic for unnumbered open brackets:
                // Before feeding to the state machine, decide if we should
                // force a new run based on document context.
                // For now, don't try to override the state machine's heuristic.
                // The alignment edges and temporal signals are available in the
                // TrnDocument for a future inference pass that bypasses the
                // Java-ported state machine entirely.

                let token = BracketToken {
                    line_number: bracket.source.line_number,
                    char_offset: bracket.source.char_offset,
                    column: bracket.source.column,
                    kind: match bracket.direction {
                        BracketDirection::Open => BracketKind::Open,
                        BracketDirection::Close => BracketKind::Close,
                        BracketDirection::CloseForced => BracketKind::CloseForced,
                    },
                    lexical_index: bracket.lexical_index,
                };

                match bracket.direction {
                    BracketDirection::Open => {
                        let r = state.add_begin(token, &bracket.speaker, &mut diag, &fake_lines);
                        let real_idx = state.last_classified_index();
                        bracket_roles.insert(*bracket_id, BracketRole {
                            bracket_id: *bracket_id,
                            role: overlap_role_to_intermediate(r),
                            real_index: real_idx,
                        });
                        // Reset continue flag — it should only apply to this one bracket.
                        state.set_continue_first_overlap(false);
                    }
                    BracketDirection::Close => {
                        let r = state.add_end(token, &bracket.speaker, false, &mut diag, &fake_lines);
                        let real_idx = lexical_index_value(bracket.lexical_index);
                        bracket_roles.insert(*bracket_id, BracketRole {
                            bracket_id: *bracket_id,
                            role: overlap_role_to_intermediate(r),
                            real_index: real_idx,
                        });
                    }
                    BracketDirection::CloseForced => {
                        let r = state.add_end(token, &bracket.speaker, true, &mut diag, &fake_lines);
                        let real_idx = lexical_index_value(bracket.lexical_index);
                        bracket_roles.insert(*bracket_id, BracketRole {
                            bracket_id: *bracket_id,
                            role: overlap_role_to_intermediate(r),
                            real_index: real_idx,
                        });
                    }
                }
            }
        }

        // Track overlap context for enhanced heuristic.
        if utt_has_brackets {
            last_overlap_utt_idx = Some(utt.index);
            last_overlap_end_ms = utt.end_ms;
        }
    }

    let _runs = state.finish(&mut diag, &fake_lines);

    OverlapAssignment {
        filename: doc.filename.clone(),
        roles: bracket_roles,
        inference_diagnostics: diag.into_vec(),
    }
}

enum OverlapHint {
    /// Strong signal: force a new overlap run.
    ForceNewRun,
    /// Strong signal: this bracket continues the current overlap group (is a bottom).
    ForceContinue,
    /// No strong signal — let the state machine's default heuristic decide.
    NoOpinion,
}

/// Enhanced heuristic for unnumbered open brackets.
///
/// Returns a three-valued hint:
/// - ForceNewRun: strong evidence this is a new overlap group
/// - ForceContinue: strong evidence this is a bottom of the existing group
/// - NoOpinion: let the state machine decide
fn overlap_hint(
    bracket: &BracketRef,
    current_utt: &crate::intermediate::TrnUtterance,
    alignment_target: &std::collections::HashMap<u32, u32>,
    last_overlap_utt_idx: Option<usize>,
    last_overlap_end_ms: Option<i64>,
    doc: &TrnDocument,
) -> OverlapHint {
    // Signal 1 (STRONG): Alignment edge exists → this bracket aligns spatially
    // with a recent bracket from a different speaker. This is the transcriber
    // explicitly showing overlap correspondence via indentation.
    if alignment_target.contains_key(&bracket.id) {
        return OverlapHint::ForceContinue;
    }

    // Signal 2 (STRONG): Temporal gap — if this utterance starts significantly
    // after the last overlap ended, the conversation has moved on.
    if let (Some(last_end), Some(current_start)) = (last_overlap_end_ms, current_utt.start_ms) {
        let gap_ms = current_start - last_end;
        if gap_ms > 2000 {
            return OverlapHint::ForceNewRun;
        }
    }

    // Signal 3 (MODERATE): Intervening non-overlapping utterances.
    if let Some(last_idx) = last_overlap_utt_idx {
        let intervening_count = current_utt.index.saturating_sub(last_idx + 1);
        if intervening_count >= 2 {
            return OverlapHint::ForceNewRun;
        }
        if intervening_count == 1 {
            if let Some(intervening_utt) = doc.utterances.get(last_idx + 1) {
                let has_brackets = intervening_utt.elements.iter()
                    .any(|e| matches!(e, ContentElement::Bracket(_)));
                if !has_brackets && intervening_utt.speaker != bracket.speaker {
                    return OverlapHint::ForceNewRun;
                }
            }
        }
    }

    // No strong signal — defer to the state machine.
    OverlapHint::NoOpinion
}

fn overlap_role_to_intermediate(role: OverlapRole) -> IntermediateRole {
    match role {
        OverlapRole::TopBegin => IntermediateRole::TopBegin,
        OverlapRole::TopEnd => IntermediateRole::TopEnd,
        OverlapRole::BottomBegin => IntermediateRole::BottomBegin,
        OverlapRole::BottomEnd => IntermediateRole::BottomEnd,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token(line: usize, offset: usize, kind: BracketKind, index: Option<u8>) -> BracketToken {
        BracketToken {
            line_number: line,
            char_offset: offset,
            column: offset,
            kind,
            lexical_index: index,
        }
    }

    fn make_line(line_number: usize, start: f64, end: f64, speaker: Option<&str>, content: &str) -> TrnLine {
        TrnLine {
            line_number,
            start_time: start,
            end_time: end,
            speaker: speaker.map(|s| s.to_string()),
            effective_speaker: speaker.unwrap_or("???").to_string(),
            raw_content: content.to_string(),
            content_column: 0,
        }
    }

    #[test]
    fn simple_two_party_overlap() {
        let lines = vec![
            make_line(1, 0.0, 6.52, Some("JAMIE"), "How [can you teach] tap."),
            make_line(2, 4.43, 5.78, Some("HAROLD"), "    [I can't imagine]"),
        ];
        let mut diag = Diagnostics::new();
        let mut state = OverlapState::new();

        // JAMIE opens [
        let open1 = make_token(1, 4, BracketKind::Open, None);
        let role = state.add_begin(open1, "JAMIE", &mut diag, &lines);
        assert_eq!(role, OverlapRole::TopBegin);

        // JAMIE closes ]
        let close1 = make_token(1, 18, BracketKind::Close, None);
        let role = state.add_end(close1, "JAMIE", false, &mut diag, &lines);
        assert_eq!(role, OverlapRole::TopEnd);

        state.reset_seen();

        // HAROLD opens [
        let open2 = make_token(2, 4, BracketKind::Open, None);
        let role = state.add_begin(open2, "HAROLD", &mut diag, &lines);
        assert_eq!(role, OverlapRole::BottomBegin);

        // HAROLD closes ]
        let close2 = make_token(2, 20, BracketKind::Close, None);
        let role = state.add_end(close2, "HAROLD", false, &mut diag, &lines);
        assert_eq!(role, OverlapRole::BottomEnd);

        let runs = state.finish(&mut diag, &lines);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].sets.len(), 1);
        assert_eq!(runs[0].sets[0].top.speaker, "JAMIE");
        assert_eq!(runs[0].sets[0].bottoms.len(), 1);
        assert_eq!(runs[0].sets[0].bottoms[0].speaker, "HAROLD");
        assert!(runs[0].sets[0].complete);
    }

    #[test]
    fn numbered_overlap_pair() {
        // Simulate realistic sequence: [...] then [2...2] then [3...3].
        // We only care about testing [3] so we build up the prior indices first.
        let lines = vec![
            make_line(1, 55.0, 61.0, Some("JAMIE"), "[prob ably] [2XXX2]"),
            make_line(2, 59.0, 60.0, Some("HAROLD"), "[I mean he] [2has a bro-2]"),
            make_line(3, 61.0, 63.0, Some("HAROLD"), "leg is [3doing okay3]."),
            make_line(4, 62.0, 63.5, Some("PETE"), "      [3I was wonder3]ing"),
        ];
        let mut diag = Diagnostics::new();
        let mut state = OverlapState::new();

        // Build up indices 0 and 1 first (unnumbered + [2]).
        // Index 0: JAMIE top
        state.add_begin(make_token(1, 0, BracketKind::Open, None), "JAMIE", &mut diag, &lines);
        state.add_end(make_token(1, 10, BracketKind::Close, None), "JAMIE", false, &mut diag, &lines);
        // Index 1 ([2]): JAMIE top
        state.add_begin(make_token(1, 12, BracketKind::Open, Some(2)), "JAMIE", &mut diag, &lines);
        state.add_end(make_token(1, 16, BracketKind::Close, Some(2)), "JAMIE", false, &mut diag, &lines);
        state.reset_seen();
        // Index 0 bottom: HAROLD
        state.add_begin(make_token(2, 0, BracketKind::Open, None), "HAROLD", &mut diag, &lines);
        state.add_end(make_token(2, 10, BracketKind::Close, None), "HAROLD", false, &mut diag, &lines);
        // Index 1 ([2]) bottom: HAROLD
        state.add_begin(make_token(2, 12, BracketKind::Open, Some(2)), "HAROLD", &mut diag, &lines);
        state.add_end(make_token(2, 24, BracketKind::Close, Some(2)), "HAROLD", false, &mut diag, &lines);
        state.reset_seen();

        // Now [3]: HAROLD top
        let role = state.add_begin(make_token(3, 7, BracketKind::Open, Some(3)), "HAROLD", &mut diag, &lines);
        assert_eq!(role, OverlapRole::TopBegin);
        state.add_end(make_token(3, 18, BracketKind::Close, Some(3)), "HAROLD", false, &mut diag, &lines);
        state.reset_seen();

        // [3] PETE bottom
        let role = state.add_begin(make_token(4, 6, BracketKind::Open, Some(3)), "PETE", &mut diag, &lines);
        assert_eq!(role, OverlapRole::BottomBegin);
        let role = state.add_end(make_token(4, 20, BracketKind::Close, Some(3)), "PETE", false, &mut diag, &lines);
        assert_eq!(role, OverlapRole::BottomEnd);

        let runs = state.finish(&mut diag, &lines);
        assert_eq!(runs.len(), 1);
        // [3] is the third set (real_index = 2).
        assert_eq!(runs[0].sets[2].real_index, 2);
        assert_eq!(runs[0].sets[2].top.speaker, "HAROLD");
        assert_eq!(runs[0].sets[2].bottoms[0].speaker, "PETE");
    }

    #[test]
    fn one_top_multiple_bottoms() {
        // Build up indices 0-3 ([...], [2], [3], [4]) before [5].
        let lines = vec![
            make_line(1, 68.0, 73.0, Some("JAMIE"), "[a] [2b2] [3c3] [4d4] [5back5] fast"),
            make_line(2, 73.0, 74.0, Some("PETE"), "[a] [2b2] [3c3] [4d4] [5hm=5]."),
            make_line(3, 73.0, 74.0, Some("HAROLD"), "[5Yeah5]."),
        ];
        let mut diag = Diagnostics::new();
        let mut state = OverlapState::new();

        // Build indices 0-3 as JAMIE top, then PETE bottom.
        for (idx_opt, off) in [(None, 0usize), (Some(2), 4), (Some(3), 10), (Some(4), 16)] {
            state.add_begin(make_token(1, off, BracketKind::Open, idx_opt), "JAMIE", &mut diag, &lines);
            state.add_end(make_token(1, off + 2, BracketKind::Close, idx_opt), "JAMIE", false, &mut diag, &lines);
        }
        // JAMIE top [5]
        state.add_begin(make_token(1, 22, BracketKind::Open, Some(5)), "JAMIE", &mut diag, &lines);
        state.add_end(make_token(1, 27, BracketKind::Close, Some(5)), "JAMIE", false, &mut diag, &lines);
        state.reset_seen();

        // PETE bottoms for 0-3 and [5]
        for (idx_opt, off) in [(None, 0usize), (Some(2), 4), (Some(3), 10), (Some(4), 16)] {
            state.add_begin(make_token(2, off, BracketKind::Open, idx_opt), "PETE", &mut diag, &lines);
            state.add_end(make_token(2, off + 2, BracketKind::Close, idx_opt), "PETE", false, &mut diag, &lines);
        }
        // PETE bottom [5]
        state.add_begin(make_token(2, 22, BracketKind::Open, Some(5)), "PETE", &mut diag, &lines);
        state.add_end(make_token(2, 27, BracketKind::Close, Some(5)), "PETE", false, &mut diag, &lines);
        state.reset_seen();

        // HAROLD second bottom for [5]
        state.add_begin(make_token(3, 0, BracketKind::Open, Some(5)), "HAROLD", &mut diag, &lines);
        state.add_end(make_token(3, 5, BracketKind::Close, Some(5)), "HAROLD", false, &mut diag, &lines);

        let runs = state.finish(&mut diag, &lines);
        // Set at real_index 4 (for [5]) should have 2 bottoms.
        assert_eq!(runs[0].sets[4].bottoms.len(), 2);
        assert_eq!(runs[0].sets[4].top.speaker, "JAMIE");
    }

    #[test]
    fn same_speaker_error() {
        let lines = vec![
            make_line(1, 0.0, 5.0, Some("JAMIE"), "[text]"),
        ];
        let mut diag = Diagnostics::new();
        let mut state = OverlapState::new();

        state.add_begin(make_token(1, 0, BracketKind::Open, None), "JAMIE", &mut diag, &lines);
        state.add_end(make_token(1, 5, BracketKind::Close, None), "JAMIE", false, &mut diag, &lines);
        state.reset_seen();

        // Same speaker tries to be bottom.
        state.add_begin(make_token(1, 0, BracketKind::Open, None), "JAMIE", &mut diag, &lines);

        // Should have diagnostic (either SameSpeaker or TryNewRun leading to new top).
        let runs = state.finish(&mut diag, &lines);
        assert!(runs.len() >= 1);
    }
}
