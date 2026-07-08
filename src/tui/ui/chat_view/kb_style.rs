//! Keyboard-shortcut key styling shared by status-bar hint builders.

use ratatui::{
    style::{Color, Style},
    text::Span,
};

use crate::tui::ui::chat_view::spinner::spinner_color;
use crate::tui::ui::gradient::rgb_supported;

/// Style a keyboard shortcut key with the live spinner color (truecolor)
/// or yellow (8-color).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::kb_style::kb;
/// let span = kb("Ctrl+C");
/// assert_eq!(span.content.as_ref(), "Ctrl+C");
/// ```
pub fn kb(key: &str) -> Span<'static> {
    let color = if rgb_supported() {
        spinner_color()
    } else {
        Color::Yellow
    };
    Span::styled(key.to_string(), Style::default().fg(color))
}
