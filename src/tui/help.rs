use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
    },
};

use crate::tui::app::state::AppState;
use crate::tui::token_display::TokenDisplay;

/// Mutable scroll state for the help view, stored externally.
pub struct HelpScrollState {
    pub offset: usize,
}

impl Default for HelpScrollState {
    fn default() -> Self {
        Self { offset: 0 }
    }
}

impl HelpScrollState {
    pub fn scroll_up(&mut self, n: usize) {
        self.offset = self.offset.saturating_sub(n);
    }

    pub fn scroll_down(&mut self, n: usize, max: usize) {
        self.offset = (self.offset + n).min(max);
    }
}

fn heading(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        format!("  {text}"),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}

fn separator() -> Line<'static> {
    Line::from(Span::styled(
        "  ─────────────────────────────────────────────",
        Style::default().fg(Color::DarkGray),
    ))
}

fn key_row(key: &str, desc: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("  {key:<18}"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(desc.to_string()),
    ])
}

fn cmd_row(cmd: &str, alias: &str, desc: &str) -> Line<'static> {
    let mut spans = vec![Span::styled(
        format!("  {cmd:<14}"),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )];
    if !alias.is_empty() {
        spans.push(Span::styled(
            format!("{alias:<6} "),
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        spans.push(Span::raw("       ".to_string()));
    }
    spans.push(Span::raw(desc.to_string()));
    Line::from(spans)
}

fn blank() -> Line<'static> {
    Line::from("")
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn build_help_lines(app_state: &AppState) -> Vec<Line<'static>> {
    let token_display = TokenDisplay::new();
    let session_label = app_state
        .session_id
        .as_deref()
        .map(|id| {
            if id.len() > 20 {
                format!("{}…", &id[..20])
            } else {
                id.to_string()
            }
        })
        .unwrap_or_else(|| "(none)".to_string());

    let mut lines: Vec<Line<'static>> = Vec::with_capacity(80);

    // ── Title ──
    lines.push(Line::from(vec![
        Span::styled(
            "  CodeTether TUI",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "  ·  Help & Reference",
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    lines.push(separator());
    lines.push(blank());

    // ── Session info ──
    heading("SESSION");
    lines.push(heading("SESSION"));
    lines.push(Line::from(vec![
        Span::styled("  Session:   ", Style::default().fg(Color::DarkGray)),
        Span::styled(session_label, Style::default().fg(Color::White)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Directory: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            app_state.cwd_display.clone(),
            Style::default().fg(Color::White),
        ),
    ]));

    // Worker info if connected
    if let Some(ref wid) = app_state.worker_id {
        lines.push(Line::from(vec![
            Span::styled("  Worker:    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                app_state.worker_name.clone().unwrap_or_else(|| wid.clone()),
                Style::default().fg(Color::Green),
            ),
            if app_state.a2a_connected {
                Span::styled("  (A2A connected)", Style::default().fg(Color::Green))
            } else {
                Span::styled("  (disconnected)", Style::default().fg(Color::Red))
            },
        ]));
    }
    lines.push(blank());

    let global = crate::telemetry::TOKEN_USAGE.global_snapshot();
    if global.total.total() > 0 {
        let _cost = token_display.calculate_cost_for_tokens("gpt-4o", global.input, global.output);
        lines.push(Line::from(vec![
            Span::styled("  Tokens: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} total", global.total.total()),
                Style::default().fg(Color::Yellow),
            ),
        ]));
        lines.push(blank());
    }

    // ── Keyboard shortcuts ──
    lines.push(heading("KEYBOARD SHORTCUTS"));
    lines.push(separator());
    lines.push(key_row("Ctrl+C / Ctrl+Q", "Quit"));
    lines.push(key_row("Esc", "Back / close overlay / exit detail"));
    lines.push(key_row("Ctrl+T", "Symbol search (workspace)"));
    lines.push(key_row("Enter", "Send message or run slash command"));
    lines.push(key_row("Tab", "Accept slash autocomplete"));
    lines.push(blank());

    lines.push(heading("TEXT EDITING"));
    lines.push(key_row("Left / Right", "Move cursor"));
    lines.push(key_row("Ctrl+Left/Right", "Move by word"));
    lines.push(key_row("Home / End", "Jump to start / end of input"));
    lines.push(key_row("Backspace", "Delete backward"));
    lines.push(key_row("Delete", "Delete forward"));
    lines.push(key_row("Ctrl+Up / Down", "Command history (in chat)"));
    lines.push(blank());

    lines.push(heading("SCROLLING"));
    lines.push(key_row("Up / Down", "Scroll messages"));
    lines.push(key_row("PgUp / PgDn", "Scroll by page"));
    lines.push(blank());

    // ── Slash commands ──
    lines.push(heading("SLASH COMMANDS"));
    lines.push(separator());
    lines.push(Line::from(Span::styled(
        "  Aliases: type a prefix and Tab to autocomplete",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(blank());

    lines.push(heading("Navigation"));
    lines.push(cmd_row("/chat", "", "Return to chat view"));
    lines.push(cmd_row("/help", "/h /?", "Open this help"));
    lines.push(cmd_row("/sessions", "/s", "Session picker"));
    lines.push(cmd_row("/model", "/m", "Model picker"));
    lines.push(cmd_row("/file", "", "Attach /file <path> to composer"));
    lines.push(cmd_row("/settings", "/set", "Settings panel"));
    lines.push(cmd_row("/new", "", "Start fresh chat buffer"));
    lines.push(blank());

    lines.push(heading("Protocol & Observability"));
    lines.push(cmd_row("/bus", "/b", "Protocol bus log"));
    lines.push(cmd_row("/protocol", "/p", "Protocol bus (alias)"));
    lines.push(cmd_row("/swarm", "/w", "Swarm agent view"));
    lines.push(cmd_row("/ralph", "/r", "Ralph PRD loop view"));
    lines.push(cmd_row("/latency", "", "Provider + tool latency inspector"));
    lines.push(blank());

    lines.push(heading("Development Tools"));
    lines.push(cmd_row("/lsp", "", "LSP diagnostics view"));
    lines.push(cmd_row("/rlm", "", "RLM processing view"));
    lines.push(cmd_row("/symbols", "/sym", "Workspace symbol search"));
    lines.push(cmd_row("/keys", "", "Print all commands to status bar"));
    lines.push(blank());

    // ── View-specific controls ──
    lines.push(heading("SESSION PICKER"));
    lines.push(separator());
    lines.push(key_row("Up / Down", "Navigate sessions"));
    lines.push(key_row("Enter", "Load selected session"));
    lines.push(key_row("Type", "Filter sessions by name/ID"));
    lines.push(key_row("Backspace", "Clear filter character"));
    lines.push(key_row("Esc", "Close picker"));
    lines.push(blank());

    lines.push(heading("PROTOCOL BUS LOG"));
    lines.push(separator());
    lines.push(key_row("Up / Down", "Navigate entries"));
    lines.push(key_row("Enter", "Open detail view"));
    lines.push(key_row("/", "Enter filter mode"));
    lines.push(key_row("c", "Clear filter"));
    lines.push(key_row("g", "Jump to latest entry"));
    lines.push(key_row("Esc", "Close detail or filter"));
    lines.push(blank());

    lines.push(heading("SWARM / RALPH"));
    lines.push(separator());
    lines.push(key_row("Up / Down", "Select sub-agent / story"));
    lines.push(key_row("Enter", "Open detail view"));
    lines.push(key_row("PgUp / PgDn", "Scroll detail content"));
    lines.push(key_row("Esc", "Exit detail"));
    lines.push(blank());

    lines.push(separator());
    lines.push(Line::from(Span::styled(
        "  Press Esc to return to chat",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(blank());

    lines
}

pub fn render_help_overlay_if_needed(f: &mut Frame, app_state: &mut AppState) {
    if !app_state.show_help {
        return;
    }

    let area = centered_rect(60, 60, f.area());
    let lines = build_help_lines(app_state);
    let total_lines = lines.len();
    let content_height = area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(content_height);
    if app_state.help_scroll.offset > max_scroll {
        app_state.help_scroll.offset = max_scroll;
    }
    let scroll_offset = app_state.help_scroll.offset;

    f.render_widget(Clear, area);
    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset as u16, 0));
    f.render_widget(widget, area);

    if total_lines > content_height {
        let mut sb_state = ScrollbarState::new(total_lines).position(scroll_offset);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight),
            area,
            &mut sb_state,
        );
    }
}
