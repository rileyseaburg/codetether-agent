//! Pulsing `⚡ YOLO` badge shown when full-auto mode is active.
//!
//! On truecolor terminals the badge uses an inverted neon block:
//! black text on a spinning neon background — unmissable.
//! On 8-color terminals it stays bold yellow text.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use codetether_tui_style::gradient::rgb_supported;
use codetether_tui_style::spinner::spinner_color;

/// Render the animated full-auto badge when enabled.
///
/// # Arguments
/// * `enabled` - Whether full-auto edits are enabled.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::yolo_badge::yolo_badge(true);
/// ```
pub fn yolo_badge(enabled: bool) -> Option<Span<'static>> {
    if !enabled {
        return None;
    }
    let style = if rgb_supported() {
        Style::default()
            .fg(Color::Black)
            .bg(spinner_color())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    };
    Some(Span::styled(" ⚡ YOLO ", style))
}
