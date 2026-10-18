//! Bordered composer text and command-mode indicator.
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// Render composer text; the caller places the application cursor afterward.
/// # Arguments
/// * `f` - Destination frame.
/// * `input` - Current composer text.
/// * `command_mode` - Whether to display the command-mode indicator.
/// * `area` - Composer rectangle.
/// # Returns
/// Draws a bordered composer with untrimmed wrapping.
/// # Examples
/// ```no_run
/// fn draw(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
///     codetether_webview::chat::render_input(f, "hello", false, area);
/// }
/// ```
pub fn render_input(f: &mut Frame, input: &str, command_mode: bool, area: Rect) {
    let mode_label = if command_mode { " [CMD]" } else { "" };
    let title = format!(" Message (Enter to send, / for commands, ! for shell){mode_label} ");
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(Color::Cyan));
    let para = Paragraph::new(input)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(para, area);
}
