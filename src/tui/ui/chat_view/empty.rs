//! Empty-placeholder line when the chat has no messages.

use ratatui::{style::Stylize, text::Line};

/// Push a welcoming placeholder when no messages exist yet.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::empty::push_empty_placeholder;
/// let mut lines: Vec<ratatui::text::Line<'static>> = Vec::new();
/// push_empty_placeholder(&mut lines);
/// assert_eq!(lines.len(), 1);
/// ```
pub fn push_empty_placeholder(lines: &mut Vec<Line<'static>>) {
    lines.push(Line::from(
        "No messages yet. Type a prompt and press Enter, or use /help.".dim(),
    ));
}
