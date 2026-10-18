//! Pending approval popup composition and queue projection.
use codetether_approval_queue as approval_queue;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Clear},
};
/// Render the current pending approval unless the user is composing feedback.
/// # Arguments
/// * `f` - Terminal frame to draw into.
/// * `area` - Available popup area.
/// * `input` - Composer text used to detect feedback composition.
/// * `scroll` - Preview offset, clamped to the rendered content.
/// # Returns
/// Draws the popup and updates the offset and queued preview bounds.
/// # Panics
/// Panics if the shared approval queue mutex was poisoned.
/// # Examples
/// ```no_run
/// # fn draw(frame: &mut ratatui::Frame) {
/// let mut scroll = 0;
/// let area = frame.area();
/// codetether_approval_view::render(frame, area, "", &mut scroll);
/// # }
/// ```

pub fn render(f: &mut Frame, area: Rect, input: &str, scroll: &mut u16) {
    if approval_queue::feedback_input(input) {
        return;
    }
    let Some(item) = approval_queue::active() else {
        return;
    };
    let popup = crate::layout::popup(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Approval · {} pending", approval_queue::len()));
    let inner = block.inner(popup);
    let footer_height = crate::footer::height(inner.width).min(inner.height.saturating_sub(3));
    let [preview, footer] =
        Layout::vertical([Constraint::Min(3), Constraint::Length(footer_height)]).areas(inner);
    f.render_widget(Clear, popup);
    f.render_widget(block, popup);
    crate::preview::render(f, preview, &item, scroll);
    crate::footer::render(f, footer);
}
