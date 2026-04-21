//! Audit view — subagent activity inspector for the TUI.
//!
//! Renders recent entries from the global [`AuditLog`](crate::audit::AuditLog),
//! filtered by default to subagent-relevant categories (`Swarm`,
//! `ToolExecution`, `Cognition`). Lets the operator see *what the
//! sub-agents just did* without leaving the TUI — which tool was
//! called, by which persona, against which session, with what
//! outcome.
//!
//! State is refreshed from the async audit log on each TUI tick via
//! [`refresh_audit_snapshot`]. Rendering itself is synchronous and
//! reads from the cached [`AuditViewState::entries`] snapshot.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::audit::{AuditCategory, AuditEntry, AuditOutcome, try_audit_log};

/// Maximum number of entries the view caches / renders per category.
const SNAPSHOT_LIMIT: usize = 500;

/// Which slice of the audit log the view is currently showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditFilter {
    /// Subagent-relevant only: Swarm + ToolExecution + Cognition.
    Subagents,
    /// Every category.
    All,
}

impl AuditFilter {
    fn matches(self, e: &AuditEntry) -> bool {
        match self {
            Self::All => true,
            Self::Subagents => matches!(
                e.category,
                AuditCategory::Swarm | AuditCategory::ToolExecution | AuditCategory::Cognition
            ),
        }
    }

    /// Short label for the view header.
    pub fn label(self) -> &'static str {
        match self {
            Self::Subagents => "Subagents",
            Self::All => "All",
        }
    }
}

/// State for the audit view: the cached entries plus UI cursor.
#[derive(Debug)]
pub struct AuditViewState {
    pub entries: Vec<AuditEntry>,
    pub selected: usize,
    pub filter: AuditFilter,
    /// Monotonic counter; bumps each refresh so the footer can show
    /// "updated N ticks ago" without pulling chrono into every frame.
    pub refresh_counter: u64,
}

impl Default for AuditViewState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            selected: 0,
            filter: AuditFilter::Subagents,
            refresh_counter: 0,
        }
    }
}

impl AuditViewState {
    /// Move the selection cursor up; saturates at 0.
    pub fn select_prev(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Move the selection cursor down; saturates at `entries.len() - 1`.
    pub fn select_next(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        }
    }

    /// Toggle between [`AuditFilter::Subagents`] and [`AuditFilter::All`].
    pub fn toggle_filter(&mut self) {
        self.filter = match self.filter {
            AuditFilter::Subagents => AuditFilter::All,
            AuditFilter::All => AuditFilter::Subagents,
        };
        self.selected = 0;
    }
}

/// Pull fresh entries from the global audit log into `state`.
///
/// Called on each TUI tick. When the global audit log has not been
/// initialized (e.g. running without the server), this is a cheap no-op.
pub async fn refresh_audit_snapshot(state: &mut AuditViewState) {
    let Some(log) = try_audit_log() else {
        return;
    };
    let recent = log.recent(SNAPSHOT_LIMIT).await;
    state.entries = recent
        .into_iter()
        .filter(|e| state.filter.matches(e))
        .collect();
    if state.selected >= state.entries.len() {
        state.selected = state.entries.len().saturating_sub(1);
    }
    state.refresh_counter = state.refresh_counter.wrapping_add(1);
}

/// Render the audit view: list on the left, detail on the right.
pub fn render_audit_view(f: &mut Frame, state: &mut AuditViewState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(f, state, chunks[0]);
    render_body(f, state, chunks[1]);
    render_footer(f, state, chunks[2]);
}

fn render_header(f: &mut Frame, state: &AuditViewState, area: Rect) {
    let line = Line::from(vec![
        "Audit".bold(),
        " · ".dim(),
        Span::styled(
            state.filter.label(),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        " · ".dim(),
        format!("{} entries", state.entries.len()).into(),
    ]);
    f.render_widget(Paragraph::new(line), area);
}

fn render_body(f: &mut Frame, state: &mut AuditViewState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    render_list(f, state, chunks[0]);
    render_detail(f, state, chunks[1]);
}

fn render_list(f: &mut Frame, state: &mut AuditViewState, area: Rect) {
    let items: Vec<ListItem> = state
        .entries
        .iter()
        .map(|e| ListItem::new(format_row(e)))
        .collect();

    let mut list_state = ListState::default();
    if !state.entries.is_empty() {
        list_state.select(Some(state.selected));
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Events "))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_detail(f: &mut Frame, state: &AuditViewState, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Detail ");
    let Some(entry) = state.entries.get(state.selected) else {
        f.render_widget(
            Paragraph::new("No audit entries yet.".dim()).block(block),
            area,
        );
        return;
    };
    let detail_json = entry
        .detail
        .as_ref()
        .map(|v| serde_json::to_string_pretty(v).unwrap_or_else(|_| v.to_string()))
        .unwrap_or_else(|| "(none)".to_string());

    let body = vec![
        kv_line("id", &entry.id),
        kv_line("time", &entry.timestamp.format("%H:%M:%S%.3f").to_string()),
        kv_line("category", &format!("{:?}", entry.category)),
        kv_line("action", &entry.action),
        kv_line(
            "outcome",
            match entry.outcome {
                AuditOutcome::Success => "success",
                AuditOutcome::Failure => "failure",
                AuditOutcome::Denied => "denied",
            },
        ),
        kv_line(
            "principal",
            entry.principal.as_deref().unwrap_or("-"),
        ),
        kv_line(
            "session",
            entry.session_id.as_deref().unwrap_or("-"),
        ),
        kv_line(
            "duration_ms",
            &entry
                .duration_ms
                .map(|d| d.to_string())
                .unwrap_or_else(|| "-".into()),
        ),
        Line::from(""),
        "detail:".dim().into(),
        Line::from(detail_json),
    ];

    f.render_widget(Paragraph::new(body).block(block).wrap(Wrap { trim: false }), area);
}

fn render_footer(f: &mut Frame, _state: &AuditViewState, area: Rect) {
    let hint = Line::from(vec![
        "↑/↓".bold(),
        " select · ".dim(),
        "f".bold(),
        " toggle filter · ".dim(),
        "Esc".bold(),
        " chat".dim(),
    ]);
    f.render_widget(Paragraph::new(hint), area);
}

fn format_row(e: &AuditEntry) -> Line<'static> {
    let ts = e.timestamp.format("%H:%M:%S").to_string();
    let outcome_color = match e.outcome {
        AuditOutcome::Success => Color::Green,
        AuditOutcome::Failure => Color::Red,
        AuditOutcome::Denied => Color::Yellow,
    };
    let outcome_mark = match e.outcome {
        AuditOutcome::Success => "✓",
        AuditOutcome::Failure => "✗",
        AuditOutcome::Denied => "⊘",
    };
    Line::from(vec![
        Span::styled(ts, Style::default().fg(Color::DarkGray)),
        " ".into(),
        Span::styled(outcome_mark.to_string(), Style::default().fg(outcome_color)),
        " ".into(),
        Span::styled(
            format!("{:<10}", category_short(e.category)),
            Style::default().fg(Color::Cyan),
        ),
        " ".into(),
        Span::raw(truncate(&e.action, 40)),
        " ".into(),
        Span::styled(
            format!("[{}]", e.principal.as_deref().unwrap_or("-")),
            Style::default().fg(Color::DarkGray),
        ),
    ])
}

fn kv_line(key: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{key:>12}: "), Style::default().fg(Color::DarkGray)),
        Span::raw(value.to_string()),
    ])
}

fn category_short(c: AuditCategory) -> &'static str {
    match c {
        AuditCategory::Api => "api",
        AuditCategory::ToolExecution => "tool",
        AuditCategory::Session => "session",
        AuditCategory::Cognition => "cognition",
        AuditCategory::Swarm => "swarm",
        AuditCategory::Auth => "auth",
        AuditCategory::K8s => "k8s",
        AuditCategory::Sandbox => "sandbox",
        AuditCategory::Config => "config",
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{cut}…")
    }
}
