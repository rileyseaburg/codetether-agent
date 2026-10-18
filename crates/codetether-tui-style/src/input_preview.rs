//! Input-preview panel shared by terminal overview screens.
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

/// Render command-preview text inside a bordered terminal panel.
/// # Arguments
/// Receives the target frame, panel bounds and the current view data.
/// # Returns
/// Draws the panel using the shared input-preview presentation.
/// # Examples
/// ```no_run
/// # fn draw(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// codetether_tui_style::input_preview::render_input_preview(frame, area, "command");
/// # }
/// ```
pub fn render_input_preview(f: &mut Frame, area: Rect, text: &str) {
    let widget = Paragraph::new(text.to_string()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Input Preview"),
    );
    f.render_widget(widget, area);
}
