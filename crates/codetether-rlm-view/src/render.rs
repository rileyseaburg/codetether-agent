//! Render the RLM overview with its workspace session summary.
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use codetether_session_picker::summary::render_sessions_summary;

/// Draw the RLM overview and workspace session summary.
/// # Arguments
/// Receives the frame, panel bounds, workspace label, status, session count and selected index.
/// # Returns
/// Updates the supplied frame within the panel bounds.
/// # Examples
/// ```no_run
/// # fn draw(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// codetether_rlm_view::render_rlm(frame, area, ".", "Ready", 3, 0);
/// # }
/// ```
pub fn render_rlm(
    f: &mut Frame,
    area: Rect,
    cwd: &str,
    status: &str,
    session_count: usize,
    selected_session: usize,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(6)])
        .split(area);

    let lines = vec![
        Line::from("Recursive Language Model View"),
        Line::from(""),
        Line::from(format!("Workspace: {cwd}")),
        Line::from(format!("Status: {status}")),
        Line::from(""),
        Line::from("This integrated panel is the future home for large-codebase analysis,"),
        Line::from("summaries, semantic search previews, and recursive traces."),
        Line::from(""),
        Line::from("Use Esc to return to chat."),
    ];

    let widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("RLM"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    render_sessions_summary(f, chunks[1], session_count, selected_session);
}
