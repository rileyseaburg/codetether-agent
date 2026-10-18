//! Centered satisfaction popup geometry and widgets.
use crate::content;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
};
/// Draw a yes/no satisfaction prompt using the current selection.
///
/// # Arguments
/// * `frame` - Destination terminal frame.
/// * `selected_yes` - Whether Yes is currently highlighted.
///
/// # Returns
/// Draws the popup without recording a decision or changing goal state.
///
/// # Examples
/// ```no_run
/// # fn draw(frame: &mut ratatui::Frame) {
/// codetether_answer_review_view::render(frame, false);
/// # }
/// ```
pub fn render(frame: &mut Frame, selected_yes: bool) {
    let area = frame.area();
    let width = area.width.min(74);
    let height = area.height.min(6);
    let popup = Rect::new(
        area.x + (area.width - width) / 2,
        area.y + area.height - height,
        width,
        height,
    );
    let lines = content::lines(selected_yes);
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(lines).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Answer satisfaction"),
        ),
        popup,
    );
}
