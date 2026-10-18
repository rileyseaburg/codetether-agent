//! Empty-state banner shown when the chat has no messages.

use ratatui::text::Line;

use crate::tui::ui::banner::push_welcome_banner;

/// Push the neon welcome banner when no messages exist yet.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::empty::push_empty_placeholder;
/// let mut lines: Vec<ratatui::text::Line<'static>> = Vec::new();
/// push_empty_placeholder(&mut lines);
/// assert!(lines.len() >= 5);
/// ```
pub fn push_empty_placeholder(lines: &mut Vec<Line<'static>>) {
    push_welcome_banner(lines);
}
