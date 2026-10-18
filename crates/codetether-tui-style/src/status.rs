//! Render plain status text inside a bordered terminal panel.
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

/// Draw the current status in a bordered panel.
/// # Arguments
/// Receives the frame, panel bounds and status text.
/// # Returns
/// Updates the supplied frame within the panel bounds.
/// # Examples
/// ```no_run
/// # fn draw(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// codetether_tui_style::status::render_status(frame, area, "Ready");
/// # }
/// ```
pub fn render_status(f: &mut Frame, area: Rect, text: &str) {
    let widget = Paragraph::new(text.to_string())
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(widget, area);
}
