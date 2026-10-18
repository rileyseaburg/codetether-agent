//! Keyboard-shortcut key styling shared by status-bar hint builders.

use ratatui::{
    style::{Color, Style},
    text::Span,
};

use codetether_tui_style::gradient::rgb_supported;
use codetether_tui_style::spinner::spinner_color;

/// Style a key name with the current terminal accent.
///
/// # Arguments
/// * `key` - Shortcut name.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::kb_style::kb("Enter");
/// ```
pub fn kb(key: &str) -> Span<'static> {
    let color = if rgb_supported() {
        spinner_color()
    } else {
        Color::Yellow
    };
    Span::styled(key.to_string(), Style::default().fg(color))
}
