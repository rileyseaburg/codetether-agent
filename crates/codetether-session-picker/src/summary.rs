//! Render workspace session counts and navigation guidance.
use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// Draw session counts and the selected index.
/// # Arguments
/// Receives the frame, panel bounds, session count and selected index.
/// # Returns
/// Updates the supplied frame within the panel bounds.
/// # Examples
/// ```no_run
/// # fn draw(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// codetether_session_picker::summary::render_sessions_summary(frame, area, 3, 0);
/// # }
/// ```
pub fn render_sessions_summary(f: &mut Frame, area: Rect, count: usize, selected: usize) {
    let lines = vec![
        Line::from("Workspace Sessions Summary"),
        Line::from(""),
        Line::from(format!("Sessions found: {count}")),
        Line::from(format!("Selected index: {selected}")),
        Line::from(""),
        Line::from("Use /sessions from chat to open the full session picker."),
    ];

    let widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Sessions"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, area);
}
