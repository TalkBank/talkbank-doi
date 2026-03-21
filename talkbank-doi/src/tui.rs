//! Interactive TUI for reviewing DOI audit results with Brian.
//!
//! Controls:
//!   j/↓       — next entry
//!   k/↑       — previous entry
//!   Tab       — cycle filter (Suspicious → Pending → All)
//!   a         — Adopt  (write DataCite DOI into CDC file)
//!   r         — Retire (move findable → registered / hidden)
//!   d         — Delete (draft DOIs only)
//!   p         — Publish (promote draft → findable)
//!   o         — OK / Keep (suppress future warnings)
//!   m         — Mint (queue for minting)
//!   s         — Skip (defer)
//!   J/PgDn    — scroll detail panel down
//!   K/PgUp    — scroll detail panel up
//!   q / Ctrl-C — quit and save decisions

use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::audit::{AuditEntry, DataCiteInfo, Decision, EntryStatus, HtmlInfo};
use crate::doi::DoiState;

// ── Filter ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    /// Only entries whose status is suspicious (not Ok/Incomplete).
    Suspicious,
    /// Only suspicious entries still awaiting a decision.
    Pending,
    /// All entries.
    All,
}

impl Filter {
    pub fn label(&self) -> &'static str {
        match self {
            Filter::Suspicious => "SUSPICIOUS",
            Filter::Pending => "PENDING",
            Filter::All => "ALL",
        }
    }
    pub fn next(&self) -> Filter {
        match self {
            Filter::Suspicious => Filter::Pending,
            Filter::Pending => Filter::All,
            Filter::All => Filter::Suspicious,
        }
    }
}

// ── App state ─────────────────────────────────────────────────────────────────

pub struct App {
    pub entries: Vec<AuditEntry>,
    filtered: Vec<usize>,
    pub list_state: ListState,
    pub filter: Filter,
    pub status_msg: String,
    pub detail_scroll: u16,
}

impl App {
    pub fn new(entries: Vec<AuditEntry>) -> Self {
        let mut app = Self {
            entries,
            filtered: vec![],
            list_state: ListState::default(),
            filter: Filter::Suspicious,
            status_msg: String::new(),
            detail_scroll: 0,
        };
        app.rebuild_filter();
        if !app.filtered.is_empty() {
            app.list_state.select(Some(0));
        }
        app
    }

    fn rebuild_filter(&mut self) {
        self.filtered = (0..self.entries.len())
            .filter(|&i| match self.filter {
                Filter::All => true,
                Filter::Suspicious => self.entries[i].status.is_suspicious(),
                Filter::Pending => {
                    self.entries[i].status.is_suspicious()
                        && self.entries[i].decision == Decision::Pending
                }
            })
            .collect();

        // Clamp selection to valid range
        match self.list_state.selected() {
            Some(sel) if !self.filtered.is_empty() => {
                self.list_state
                    .select(Some(sel.min(self.filtered.len() - 1)));
            }
            Some(_) => self.list_state.select(None),
            None if !self.filtered.is_empty() => self.list_state.select(Some(0)),
            None => {}
        }
    }

    fn selected_entry_idx(&self) -> Option<usize> {
        let sel = self.list_state.selected()?;
        self.filtered.get(sel).copied()
    }

    pub fn selected_entry(&self) -> Option<&AuditEntry> {
        self.selected_entry_idx()
            .and_then(|i| self.entries.get(i))
    }

    fn selected_entry_mut(&mut self) -> Option<&mut AuditEntry> {
        let i = self.selected_entry_idx()?;
        self.entries.get_mut(i)
    }

    pub fn move_down(&mut self) {
        let n = self.filtered.len();
        if n == 0 {
            return;
        }
        let next = self
            .list_state
            .selected()
            .map_or(0, |i| (i + 1).min(n - 1));
        self.list_state.select(Some(next));
        self.detail_scroll = 0;
    }

    pub fn move_up(&mut self) {
        if self.filtered.is_empty() {
            return;
        }
        let prev = self
            .list_state
            .selected()
            .map_or(0, |i| i.saturating_sub(1));
        self.list_state.select(Some(prev));
        self.detail_scroll = 0;
    }

    pub fn cycle_filter(&mut self) {
        self.filter = self.filter.next();
        self.rebuild_filter();
    }

    pub fn decide(&mut self, d: Decision) {
        let label = format!("{d:?}");
        if let Some(entry) = self.selected_entry_mut() {
            entry.decision = d;
        }
        let pending = self
            .entries
            .iter()
            .filter(|e| e.status.is_suspicious() && e.decision == Decision::Pending)
            .count();
        self.status_msg = format!("→ {label}  ({pending} suspicious still pending)");

        // In Pending filter, rebuild so the decided entry disappears and
        // the next one takes its place
        if self.filter == Filter::Pending {
            let cur = self.list_state.selected().unwrap_or(0);
            self.rebuild_filter();
            if !self.filtered.is_empty() {
                self.list_state
                    .select(Some(cur.min(self.filtered.len() - 1)));
            }
        }
    }

    pub fn suspicious_count(&self) -> usize {
        self.entries.iter().filter(|e| e.status.is_suspicious()).count()
    }

    pub fn pending_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.status.is_suspicious() && e.decision == Decision::Pending)
            .count()
    }

    pub fn shown_count(&self) -> usize {
        self.filtered.len()
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

/// Run the TUI review session. Returns the entries with decisions filled in.
pub fn run(entries: Vec<AuditEntry>) -> Result<Vec<AuditEntry>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(entries);
    let loop_result = event_loop(&mut terminal, &mut app);

    // Always restore terminal even on error
    let _ = disable_raw_mode();
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    let _ = terminal.show_cursor();

    loop_result?;
    Ok(app.entries)
}

// ── Event loop ────────────────────────────────────────────────────────────────

fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };

        match (key.code, key.modifiers) {
            (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,

            (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::NONE) => app.move_down(),
            (KeyCode::Up, _) | (KeyCode::Char('k'), KeyModifiers::NONE) => app.move_up(),
            (KeyCode::Tab, _) => app.cycle_filter(),

            (KeyCode::Char('a'), _) => app.decide(Decision::Adopt),
            (KeyCode::Char('r'), _) => app.decide(Decision::Retire),
            (KeyCode::Char('d'), _) => app.decide(Decision::Delete),
            (KeyCode::Char('p'), _) => app.decide(Decision::Publish),
            (KeyCode::Char('o'), _) => app.decide(Decision::Keep),
            (KeyCode::Char('m'), _) => app.decide(Decision::Mint),
            (KeyCode::Char('s'), _) => app.decide(Decision::Skip),

            (KeyCode::PageDown, _) | (KeyCode::Char('J'), _) => {
                app.detail_scroll = app.detail_scroll.saturating_add(5);
            }
            (KeyCode::PageUp, _) | (KeyCode::Char('K'), _) => {
                app.detail_scroll = app.detail_scroll.saturating_sub(5);
            }

            _ => {}
        }
    }
    Ok(())
}

// ── Rendering ─────────────────────────────────────────────────────────────────

fn ui(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(0),    // body
            Constraint::Length(3), // footer
        ])
        .split(area);

    render_header(f, app, rows[0]);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .split(rows[1]);

    // Snapshot selected entry before the mutable borrow for list rendering
    let selected = app.selected_entry().cloned();
    let scroll = app.detail_scroll;

    render_list(f, app, cols[0]);
    render_detail(f, selected.as_ref(), scroll, cols[1]);
    render_footer(f, &app.status_msg.clone(), rows[2]);
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let text = format!(
        " TalkBank DOI Review  [Tab: {}]  {} suspicious  {} pending  {}/{} shown ",
        app.filter.label(),
        app.suspicious_count(),
        app.pending_count(),
        app.shown_count(),
        app.entries.len(),
    );
    let p = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(p, area);
}

fn status_color(s: &EntryStatus) -> Color {
    match s {
        EntryStatus::Ok => Color::Green,
        EntryStatus::Incomplete | EntryStatus::HtmlStale | EntryStatus::DraftOnly => Color::Yellow,
        EntryStatus::NeedsMinting => Color::DarkGray,
        EntryStatus::ManuallyMinted | EntryStatus::Unregistered => Color::Cyan,
        EntryStatus::HtmlOnly | EntryStatus::UrlMismatch | EntryStatus::DuplicateDoi => Color::Red,
    }
}

fn decision_color(d: &Decision) -> Color {
    match d {
        Decision::Pending => Color::DarkGray,
        Decision::Keep => Color::Green,
        Decision::Retire | Decision::Delete => Color::Red,
        Decision::Adopt | Decision::Publish => Color::Cyan,
        Decision::Mint | Decision::Skip => Color::Yellow,
    }
}

fn render_list(f: &mut Frame, app: &mut App, area: Rect) {
    let max_path = (area.width as usize).saturating_sub(16);

    let items: Vec<ListItem> = app
        .filtered
        .iter()
        .map(|&i| {
            let entry = &app.entries[i];

            // Three source indicators: DataCite · CDC · HTML
            let dc = match &entry.datacite {
                DataCiteInfo::Found(_) => Span::styled("●", Style::default().fg(Color::Green)),
                DataCiteInfo::NotFound => Span::styled("○", Style::default().fg(Color::Red)),
                DataCiteInfo::NotQueried => Span::styled("·", Style::default().fg(Color::DarkGray)),
            };
            let cdc = if entry.cdc.doi.is_some() {
                Span::styled("●", Style::default().fg(Color::Green))
            } else {
                Span::styled("○", Style::default().fg(Color::DarkGray))
            };
            let html = match &entry.html {
                HtmlInfo::Found { .. } => Span::styled("●", Style::default().fg(Color::Green)),
                HtmlInfo::NotFound => Span::styled("○", Style::default().fg(Color::DarkGray)),
                HtmlInfo::NotScanned => Span::styled("·", Style::default().fg(Color::DarkGray)),
            };

            // Truncate path to fit
            let path = &entry.display_path;
            let display = if path.len() > max_path {
                format!("…{}", &path[path.len().saturating_sub(max_path - 1)..])
            } else {
                path.clone()
            };

            let decision_str = entry.decision.label();

            let line = Line::from(vec![
                dc,
                cdc,
                html,
                Span::raw(" "),
                Span::styled(
                    format!("{:<width$}", display, width = max_path),
                    Style::default().fg(status_color(&entry.status)),
                ),
                Span::raw(" "),
                Span::styled(decision_str, Style::default().fg(decision_color(&entry.decision))),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Corpora "))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_detail(f: &mut Frame, entry: Option<&AuditEntry>, scroll: u16, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Detail ");

    let Some(entry) = entry else {
        f.render_widget(
            Paragraph::new("Select an entry to view details.").block(block),
            area,
        );
        return;
    };

    let mut lines: Vec<Line> = Vec::new();

    // ── Heading ──
    let title = entry
        .cdc
        .title
        .as_deref()
        .or_else(|| {
            if let DataCiteInfo::Found(r) = &entry.datacite {
                Some(r.title.as_str())
            } else {
                None
            }
        })
        .unwrap_or("(unknown)");

    let bank_label = if entry.bank.is_empty() {
        String::new()
    } else {
        format!("{} › ", entry.bank.to_uppercase())
    };

    lines.push(Line::from(Span::styled(
        format!("{bank_label}{title}"),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        entry.display_path.clone(),
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    // ── Status ──
    let sc = status_color(&entry.status);
    lines.push(Line::from(vec![
        Span::raw("Status   "),
        Span::styled(
            entry.status.label(),
            Style::default().fg(sc).add_modifier(Modifier::BOLD),
        ),
    ]));
    if let Some(url) = &entry.target_url {
        lines.push(Line::from(vec![
            Span::raw("URL      "),
            Span::styled(url.clone(), Style::default().fg(Color::Blue)),
        ]));
    }
    lines.push(Line::from(""));

    // ── DataCite ──
    lines.push(section_heading("DataCite"));
    match &entry.datacite {
        DataCiteInfo::NotQueried => {
            lines.push(dim_line("  · not queried  (pass --verify to check)"));
        }
        DataCiteInfo::NotFound => {
            lines.push(Line::from(Span::styled(
                "  ○ not found",
                Style::default().fg(Color::Red),
            )));
        }
        DataCiteInfo::Found(r) => {
            let state_color = match r.state {
                DoiState::Findable => Color::Green,
                DoiState::Registered => Color::Yellow,
                DoiState::Draft => Color::Red,
            };
            lines.push(Line::from(vec![
                Span::styled("  ● ", Style::default().fg(Color::Green)),
                Span::styled(r.doi.to_string(), Style::default().fg(Color::Cyan)),
                Span::raw("  "),
                Span::styled(
                    format!("[{}]", r.state.label()),
                    Style::default().fg(state_color),
                ),
            ]));
            lines.push(Line::from(format!("    title   {}", r.title)));
            lines.push(Line::from(vec![
                Span::raw("    url     "),
                Span::styled(r.url.clone(), Style::default().fg(Color::Blue)),
            ]));
        }
    }
    lines.push(Line::from(""));

    // ── CDC file ──
    lines.push(section_heading("0metadata.cdc"));
    let cdc = &entry.cdc;
    if let Some(doi) = &cdc.doi {
        lines.push(Line::from(vec![
            Span::styled("  ● doi  ", Style::default().fg(Color::Green)),
            Span::styled(doi.to_string(), Style::default().fg(Color::Cyan)),
        ]));
    } else {
        lines.push(dim_line("  ○ no DOI field"));
    }
    if let Some(t) = &cdc.title {
        lines.push(Line::from(format!("    title   {t}")));
    }
    if !cdc.creators.is_empty() {
        lines.push(Line::from(format!("    creator {}", cdc.creators.join("; "))));
    }
    if let Some(d) = &cdc.date {
        lines.push(Line::from(format!("    date    {d}")));
    }
    if let Some(l) = &cdc.language {
        lines.push(Line::from(format!("    lang    {l}")));
    }
    match &cdc.description {
        Some(desc) => {
            let s = if desc.len() > 72 {
                format!("{}…", &desc[..69])
            } else {
                desc.clone()
            };
            lines.push(Line::from(format!("    desc    {s}")));
        }
        None => lines.push(Line::from(Span::styled(
            "    desc    (none)",
            Style::default().fg(Color::Yellow),
        ))),
    }
    lines.push(Line::from(""));

    // ── HTML ──
    lines.push(section_heading("HTML reference"));
    match &entry.html {
        HtmlInfo::NotScanned => {
            lines.push(dim_line("  · not scanned  (pass --web-dir to enable)"));
        }
        HtmlInfo::NotFound => {
            lines.push(dim_line("  ○ not found in access HTML"));
        }
        HtmlInfo::Found { doi, path } => {
            lines.push(Line::from(vec![
                Span::styled("  ● ", Style::default().fg(Color::Green)),
                Span::styled(doi.to_string(), Style::default().fg(Color::Cyan)),
            ]));
            lines.push(Line::from(format!("    {}", path.display())));
        }
    }
    lines.push(Line::from(""));

    // ── Suggestion ──
    let suggestion = suggest(&entry.status, &entry.decision);
    if !suggestion.is_empty() {
        lines.push(section_heading("Suggestion"));
        for l in suggestion.lines() {
            lines.push(Line::from(Span::styled(
                format!("  {l}"),
                Style::default().fg(Color::Yellow),
            )));
        }
    }

    let p = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    f.render_widget(p, area);
}

fn render_footer(f: &mut Frame, msg: &str, area: Rect) {
    let text = if msg.is_empty() {
        " [a]dopt  [r]etire  [d]elete  [p]ublish  [o]k  [m]int  [s]kip  [Tab] filter  [j/k] nav  [q] quit "
    } else {
        msg
    };
    let p = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(p, area);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn section_heading(title: &str) -> Line<'_> {
    Line::from(Span::styled(
        format!("── {title} {}", "─".repeat(38usize.saturating_sub(title.len() + 4))),
        Style::default().fg(Color::DarkGray),
    ))
}

fn dim_line(s: &str) -> Line<'_> {
    Line::from(Span::styled(s, Style::default().fg(Color::DarkGray)))
}

fn suggest(status: &EntryStatus, decision: &Decision) -> String {
    if *decision != Decision::Pending {
        return match decision {
            Decision::Adopt => "Queued: will write DOI from DataCite into the CDC file.".into(),
            Decision::Retire => "Queued: will move DOI from findable → registered (hidden from search).".into(),
            Decision::Delete => "Queued: will DELETE this draft DOI from DataCite permanently.".into(),
            Decision::Publish => "Queued: will promote this DOI from draft → findable.".into(),
            Decision::Keep => "Marked OK — will not appear in future Suspicious reviews.".into(),
            Decision::Skip => "Skipped — will reappear next review session.".into(),
            Decision::Mint => "Queued: will mint a new DOI via DataCite API.".into(),
            Decision::Pending => unreachable!(),
        };
    }
    match status {
        EntryStatus::Ok => String::new(),
        EntryStatus::NeedsMinting =>
            "No DOI anywhere for this corpus.\n[m] queue for minting  [s] skip".into(),
        EntryStatus::ManuallyMinted =>
            "DOI exists in DataCite but was never written back to the CDC\nfile — likely minted manually via Fabrica.\n[a] adopt into CDC  [r] retire if corpus is gone".into(),
        EntryStatus::Unregistered =>
            "CDC file records a DOI that DataCite doesn't know about.\nCheck if the DOI value in the file is correct.\n[m] re-sync to DataCite  [s] skip".into(),
        EntryStatus::HtmlOnly =>
            "HTML references a DOI found nowhere else. Users visiting this\ncorpus page see a broken DOI link.\n[r] retire (hide)  [s] skip".into(),
        EntryStatus::HtmlStale =>
            "DataCite + CDC agree but HTML hasn't regenerated yet.\nThis clears automatically on the next deploy.\n[o] mark OK  [s] skip".into(),
        EntryStatus::UrlMismatch =>
            "The URL registered in DataCite differs from the current\ncorpus path. Usually caused by a directory rename.\n[a] update DataCite to current URL".into(),
        EntryStatus::DuplicateDoi =>
            "Two CDC files share this DOI. One is wrong.\nInvestigate before taking action.".into(),
        EntryStatus::DraftOnly =>
            "DOI exists in DataCite as draft — never published publicly.\n[p] publish (make findable)  [d] delete draft".into(),
        EntryStatus::Incomplete =>
            "DOI is registered but missing recommended fields.\nEdit the CDC file to add description and/or language.\n[o] mark OK and suppress this warning".into(),
    }
}
