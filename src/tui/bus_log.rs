//! Protocol bus log view for the TUI
//!
//! Displays a real-time, scrollable log of every `BusEnvelope` that
//! travels over the `AgentBus`, giving full visibility into how
//! sub-agents, the gRPC layer, and workers communicate.

use crate::bus::{BusEnvelope, BusMessage};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

// ─── State ───────────────────────────────────────────────────────────────

/// A flattened, display-ready log entry derived from a `BusEnvelope`.
#[derive(Debug, Clone)]
pub struct BusLogEntry {
    /// When the envelope was created (formatted)
    pub timestamp: String,
    /// The routing topic
    pub topic: String,
    /// Who sent it
    pub sender_id: String,
    /// Human-readable message kind label
    pub kind: String,
    /// One-line summary of the payload
    pub summary: String,
    /// Full detail text (shown in detail pane)
    pub detail: String,
    /// Display color for the kind badge
    pub kind_color: Color,
}

impl BusLogEntry {
    /// Build a display entry from a raw envelope.
    pub fn from_envelope(env: &BusEnvelope) -> Self {
        let timestamp = env.timestamp.format("%H:%M:%S%.3f").to_string();
        let topic = env.topic.clone();
        let sender_id = env.sender_id.clone();

        let (kind, summary, detail, kind_color) = match &env.message {
            BusMessage::AgentReady {
                agent_id,
                capabilities,
            } => (
                "READY".to_string(),
                format!("{agent_id} online ({} caps)", capabilities.len()),
                format!(
                    "Agent: {agent_id}\nCapabilities: {}",
                    capabilities.join(", ")
                ),
                Color::Green,
            ),
            BusMessage::AgentShutdown { agent_id } => (
                "SHUTDOWN".to_string(),
                format!("{agent_id} shutting down"),
                format!("Agent: {agent_id}"),
                Color::Red,
            ),
            BusMessage::AgentMessage { from, to, parts } => {
                let text_preview: String = parts
                    .iter()
                    .filter_map(|p| match p {
                        crate::a2a::types::Part::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                let preview = truncate(&text_preview, 80);
                (
                    "MSG".to_string(),
                    format!("{from} → {to}: {preview}"),
                    format!(
                        "From: {from}\nTo: {to}\nParts ({}):\n{text_preview}",
                        parts.len()
                    ),
                    Color::Cyan,
                )
            }
            BusMessage::TaskUpdate {
                task_id,
                state,
                message,
            } => {
                let msg = message.as_deref().unwrap_or("");
                (
                    "TASK".to_string(),
                    format!("{task_id} → {state:?} {}", truncate(msg, 50)),
                    format!("Task: {task_id}\nState: {state:?}\nMessage: {msg}"),
                    Color::Yellow,
                )
            }
            BusMessage::ArtifactUpdate { task_id, artifact } => (
                "ARTIFACT".to_string(),
                format!("task={task_id} parts={}", artifact.parts.len()),
                format!(
                    "Task: {task_id}\nArtifact: {}\nParts: {}",
                    artifact.name.as_deref().unwrap_or("(unnamed)"),
                    artifact.parts.len()
                ),
                Color::Magenta,
            ),
            BusMessage::SharedResult { key, tags, .. } => (
                "RESULT".to_string(),
                format!("key={key} tags=[{}]", tags.join(",")),
                format!("Key: {key}\nTags: {}", tags.join(", ")),
                Color::Blue,
            ),
            BusMessage::ToolRequest {
                request_id,
                agent_id,
                tool_name,
                arguments,
            } => {
                let args_str = serde_json::to_string(arguments).unwrap_or_default();
                (
                    "TOOL→".to_string(),
                    format!("{agent_id} call {tool_name}"),
                    format!(
                        "Request: {request_id}\nAgent: {agent_id}\nTool: {tool_name}\nArgs: {}",
                        truncate(&args_str, 200)
                    ),
                    Color::Yellow,
                )
            }
            BusMessage::ToolResponse {
                request_id,
                agent_id,
                tool_name,
                result,
                success,
            } => {
                let icon = if *success { "✓" } else { "✗" };
                (
                    "←TOOL".to_string(),
                    format!("{icon} {agent_id} {tool_name}"),
                    format!(
                        "Request: {request_id}\nAgent: {agent_id}\nTool: {tool_name}\nSuccess: {success}\nResult: {}",
                        truncate(result, 200)
                    ),
                    if *success { Color::Green } else { Color::Red },
                )
            }
            BusMessage::Heartbeat { agent_id, status } => (
                "BEAT".to_string(),
                format!("{agent_id} [{status}]"),
                format!("Agent: {agent_id}\nStatus: {status}"),
                Color::DarkGray,
            ),
            BusMessage::RalphLearning {
                prd_id,
                story_id,
                iteration,
                learnings,
                ..
            } => (
                "LEARN".to_string(),
                format!("{story_id} iter {iteration} ({} items)", learnings.len()),
                format!(
                    "PRD: {prd_id}\nStory: {story_id}\nIteration: {iteration}\nLearnings:\n{}",
                    learnings.join("\n")
                ),
                Color::Cyan,
            ),
            BusMessage::RalphHandoff {
                prd_id,
                from_story,
                to_story,
                progress_summary,
                ..
            } => (
                "HANDOFF".to_string(),
                format!("{from_story} → {to_story}"),
                format!(
                    "PRD: {prd_id}\nFrom: {from_story}\nTo: {to_story}\nSummary: {progress_summary}"
                ),
                Color::Blue,
            ),
            BusMessage::RalphProgress {
                prd_id,
                passed,
                total,
                iteration,
                status,
            } => (
                "PRD".to_string(),
                format!("{passed}/{total} stories (iter {iteration}) [{status}]"),
                format!(
                    "PRD: {prd_id}\nPassed: {passed}/{total}\nIteration: {iteration}\nStatus: {status}"
                ),
                Color::Yellow,
            ),
        };

        Self {
            timestamp,
            topic,
            sender_id,
            kind,
            summary,
            detail,
            kind_color,
        }
    }
}

/// State for the bus log view.
#[derive(Debug)]
pub struct BusLogState {
    /// All captured log entries (newest last).
    pub entries: Vec<BusLogEntry>,
    /// Current selection index in the list.
    pub selected_index: usize,
    /// Whether showing detail for the selected entry.
    pub detail_mode: bool,
    /// Scroll offset inside the detail pane.
    pub detail_scroll: usize,
    /// Optional topic filter (empty = show all).
    pub filter: String,
    /// Whether auto-scroll is on (follows newest entry).
    pub auto_scroll: bool,
    /// ListState for StatefulWidget rendering.
    pub list_state: ListState,
    /// Maximum entries to keep (ring buffer behaviour).
    pub max_entries: usize,
}

impl Default for BusLogState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            selected_index: 0,
            detail_mode: false,
            detail_scroll: 0,
            filter: String::new(),
            auto_scroll: true,
            list_state: ListState::default(),
            max_entries: 10_000,
        }
    }
}

impl BusLogState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a new entry, trimming old ones if over capacity.
    pub fn push(&mut self, entry: BusLogEntry) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            let excess = self.entries.len() - self.max_entries;
            self.entries.drain(..excess);
            self.selected_index = self.selected_index.saturating_sub(excess);
        }
        if self.auto_scroll && !self.entries.is_empty() {
            self.selected_index = self.filtered_entries().len().saturating_sub(1);
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Ingest a raw bus envelope.
    pub fn ingest(&mut self, env: &BusEnvelope) {
        let entry = BusLogEntry::from_envelope(env);
        self.push(entry);
    }

    /// Get entries filtered by the current topic filter.
    pub fn filtered_entries(&self) -> Vec<&BusLogEntry> {
        if self.filter.is_empty() {
            self.entries.iter().collect()
        } else {
            let f = self.filter.to_lowercase();
            self.entries
                .iter()
                .filter(|e| {
                    e.topic.to_lowercase().contains(&f)
                        || e.kind.to_lowercase().contains(&f)
                        || e.sender_id.to_lowercase().contains(&f)
                        || e.summary.to_lowercase().contains(&f)
                })
                .collect()
        }
    }

    /// Move selection up.
    pub fn select_prev(&mut self) {
        let len = self.filtered_entries().len();
        if len == 0 {
            return;
        }
        self.auto_scroll = false;
        self.selected_index = self.selected_index.saturating_sub(1);
        self.list_state.select(Some(self.selected_index));
    }

    /// Move selection down.
    pub fn select_next(&mut self) {
        let len = self.filtered_entries().len();
        if len == 0 {
            return;
        }
        self.auto_scroll = false;
        self.selected_index = (self.selected_index + 1).min(len - 1);
        self.list_state.select(Some(self.selected_index));
        // Re-enable auto-scroll if at the bottom
        if self.selected_index == len - 1 {
            self.auto_scroll = true;
        }
    }

    /// Enter detail mode for selected entry.
    pub fn enter_detail(&mut self) {
        if !self.filtered_entries().is_empty() {
            self.detail_mode = true;
            self.detail_scroll = 0;
        }
    }

    /// Exit detail mode.
    pub fn exit_detail(&mut self) {
        self.detail_mode = false;
        self.detail_scroll = 0;
    }

    pub fn detail_scroll_down(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_add(amount);
    }

    pub fn detail_scroll_up(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_sub(amount);
    }

    /// Get the currently selected entry (from filtered list).
    pub fn selected_entry(&self) -> Option<&BusLogEntry> {
        let filtered = self.filtered_entries();
        filtered.get(self.selected_index).copied()
    }

    /// Total entry count (unfiltered).
    pub fn total_count(&self) -> usize {
        self.entries.len()
    }

    /// Visible (filtered) entry count.
    pub fn visible_count(&self) -> usize {
        self.filtered_entries().len()
    }
}

// ─── Rendering ───────────────────────────────────────────────────────────

/// Render the bus protocol log view.
pub fn render_bus_log(f: &mut Frame, state: &mut BusLogState, area: Rect) {
    if state.detail_mode {
        render_entry_detail(f, state, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Log list
            Constraint::Length(1), // Key hints
        ])
        .split(area);

    // ── Header ──
    let filter_display = if state.filter.is_empty() {
        String::new()
    } else {
        format!("  filter: \"{}\"", state.filter)
    };
    let scroll_icon = if state.auto_scroll { "⬇" } else { "⏸" };

    let header_line = Line::from(vec![
        Span::styled(
            format!(" {} ", scroll_icon),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(
            format!("{}/{} messages", state.visible_count(), state.total_count()),
            Style::default().fg(Color::White),
        ),
        Span::styled(filter_display, Style::default().fg(Color::Yellow)),
    ]);

    let header = Paragraph::new(header_line).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Protocol Bus Log ")
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(header, chunks[0]);

    // ── Log list ──
    // Build items as owned data, then sync ListState separately to avoid borrow conflicts.
    let (items, filtered_len): (Vec<ListItem>, usize) = {
        let filtered = state.filtered_entries();
        let len = filtered.len();
        let items = filtered
            .iter()
            .map(|entry| {
                let line = Line::from(vec![
                    Span::styled(
                        format!("{} ", entry.timestamp),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(
                        format!("{:<8} ", entry.kind),
                        Style::default()
                            .fg(entry.kind_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("[{}] ", entry.sender_id),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(entry.summary.clone(), Style::default().fg(Color::White)),
                ]);
                ListItem::new(line)
            })
            .collect();
        (items, len)
    };

    // Sync ListState
    if filtered_len > 0 && state.selected_index < filtered_len {
        state.list_state.select(Some(state.selected_index));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Messages (↑↓:select  Enter:detail  /:filter) "),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, chunks[1], &mut state.list_state);

    // ── Key hints ──
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Back  "),
        Span::styled("Enter", Style::default().fg(Color::Yellow)),
        Span::raw(": Detail  "),
        Span::styled("/", Style::default().fg(Color::Yellow)),
        Span::raw(": Filter  "),
        Span::styled("c", Style::default().fg(Color::Yellow)),
        Span::raw(": Clear  "),
        Span::styled("g", Style::default().fg(Color::Yellow)),
        Span::raw(": Bottom"),
    ]));
    f.render_widget(hints, chunks[2]);
}

/// Render full-screen detail for a single bus log entry.
fn render_entry_detail(f: &mut Frame, state: &BusLogState, area: Rect) {
    let entry = match state.selected_entry() {
        Some(e) => e,
        None => {
            let p = Paragraph::new("No entry selected").block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Entry Detail "),
            );
            f.render_widget(p, area);
            return;
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Metadata header
            Constraint::Min(1),    // Detail body
            Constraint::Length(1), // Key hints
        ])
        .split(area);

    // ── Metadata header ──
    let header_lines = vec![
        Line::from(vec![
            Span::styled("Time:   ", Style::default().fg(Color::DarkGray)),
            Span::styled(&entry.timestamp, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Topic:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(&entry.topic, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("Sender: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&entry.sender_id, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Kind: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                &entry.kind,
                Style::default()
                    .fg(entry.kind_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let header = Paragraph::new(header_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Entry: {} ", entry.kind))
            .border_style(Style::default().fg(entry.kind_color)),
    );
    f.render_widget(header, chunks[0]);

    // ── Detail body ──
    let detail_lines: Vec<Line> = entry
        .detail
        .lines()
        .map(|l| Line::from(Span::styled(l, Style::default().fg(Color::White))))
        .collect();

    let body = Paragraph::new(detail_lines)
        .block(Block::default().borders(Borders::ALL).title(" Detail "))
        .wrap(Wrap { trim: false })
        .scroll((state.detail_scroll as u16, 0));
    f.render_widget(body, chunks[1]);

    // ── Key hints ──
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Back  "),
        Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
        Span::raw(": Scroll  "),
        Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Prev/Next entry"),
    ]));
    f.render_widget(hints, chunks[2]);
}

/// Truncate a string for display.
fn truncate(s: &str, max: usize) -> String {
    let flat = s.replace('\n', " ");
    if flat.len() <= max {
        flat
    } else {
        let mut end = max;
        while end > 0 && !flat.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}…", &flat[..end])
    }
}
