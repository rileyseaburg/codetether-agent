//! Compact keybinding hints for the status bar.
//!
//! Shows only the four most important shortcuts so the bar stays readable;
//! the full list lives in the `?` help overlay. Key names cycle with the
//! live spinner hue on truecolor terminals via [`super::kb_style::kb`].

use ratatui::{
    style::{Color, Style},
    text::Span,
};

use super::kb_style::kb;

/// Compact chat keyboard-shortcut hints.
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::compact_hints::compact_keybinding_spans();
/// ```
pub fn compact_keybinding_spans() -> Vec<Span<'static>> {
    vec![
        Span::styled(" CHAT ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" | "),
        kb("?"),
        Span::raw(": Help | "),
        kb("Tab"),
        Span::raw(": Complete | "),
        kb("Esc"),
        Span::raw(": Back | "),
        kb("Ctrl+C"),
        Span::raw(": Quit"),
    ]
}
