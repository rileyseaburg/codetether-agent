//! Keybinding hint spans for the status bar.

use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Build the CHAT banner and keybinding hint spans.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::status_hints::header_spans;
/// let spans = header_spans("abc");
/// let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
/// assert!(text.contains("CHAT"));
/// ```
pub fn header_spans(session_label: &str) -> Vec<Span<'static>> {
    vec![
        Span::styled(" CHAT ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" | "),
        kb("?"), Span::raw(": Help | "),
        kb("Tab"), Span::raw(": Complete | "),
        kb("↑↓"), Span::raw(": Scroll | "),
        kb("Shift+↑↓"), Span::raw(": Tools | "),
        kb("Ctrl+↑↓"), Span::raw(": History | "),
        kb("Ctrl+T"), Span::raw(": Symbols | "),
        kb("Esc"), Span::raw(": Back | "),
        kb("Ctrl+C"), Span::raw(": Quit | "),
        Span::styled("session", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(session_label.to_string(), Style::default().fg(Color::Cyan)),
        Span::raw(" | "),
    ]
}

/// Style a keyboard shortcut key in yellow.
fn kb(key: &str) -> Span<'static> {
    Span::styled(key.to_string(), Style::default().fg(Color::Yellow))
}
