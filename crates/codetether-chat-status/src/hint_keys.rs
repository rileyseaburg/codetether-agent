//! keybinding spans for the status bar.
use super::kb_style::kb;
use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Full chat keyboard-shortcut hints.
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::status_hints::keybinding_spans();
/// ```
pub fn keybinding_spans() -> Vec<Span<'static>> {
    vec![
        Span::styled(" CHAT ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" | "),
        kb("?"),
        Span::raw(": Help | "),
        kb("Tab"),
        Span::raw(": Complete | "),
        kb("↑↓"),
        Span::raw(": Scroll | "),
        kb("Shift+↑↓"),
        Span::raw(": Tools | "),
        kb("Ctrl+↑↓"),
        Span::raw(": History | "),
        kb("Ctrl+T"),
        Span::raw(": Symbols | "),
        kb("Ctrl+Y"),
        Span::raw(": Copy reply | "),
        kb("Ctrl+⇧Y"),
        Span::raw(": Copy all | "),
        kb("Esc"),
        Span::raw(": Back | "),
        kb("Ctrl+C"),
        Span::raw(": Quit"),
    ]
}
