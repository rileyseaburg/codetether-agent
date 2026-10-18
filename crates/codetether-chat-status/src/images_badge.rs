//! Pending images badge for the status line.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

/// Render the pending image count when nonzero.
///
/// # Arguments
/// * `count` - Pending image attachments.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::images_badge::pending_images_badge(1);
/// ```
pub fn pending_images_badge(count: usize) -> Option<Span<'static>> {
    if count == 0 {
        return None;
    }
    Some(Span::styled(
        format!(" 📷 {count} attached "),
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ))
}
