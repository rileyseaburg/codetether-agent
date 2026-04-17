//! Pending images badge for the status line.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::tui::app::state::App;

/// Badge showing the number of pending attached images, if any.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::images_badge::pending_images_badge;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let badge = pending_images_badge(app);
/// // None when no images are attached
/// # }
/// ```
pub fn pending_images_badge(app: &App) -> Option<Span<'static>> {
    let count = app.state.pending_images.len();
    if count == 0 {
        return None;
    }
    Some(Span::styled(
        format!(" 📷 {count} attached "),
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
    ))
}
