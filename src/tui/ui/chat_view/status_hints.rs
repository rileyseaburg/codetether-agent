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
    let mut spans = keybinding_spans();
    spans.push(Span::raw(" | "));
    spans.extend(session_label_spans(session_label));
    spans.push(Span::raw(" | "));
    spans
}

/// Keybinding hint spans without the trailing session badge.
///
/// Used when the status bar stacks across multiple rows so that
/// session/badge info can live on its own line.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::status_hints::keybinding_spans;
/// let spans = keybinding_spans();
/// let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
/// assert!(text.contains("Help"));
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
        kb("Esc"),
        Span::raw(": Back | "),
        kb("Ctrl+C"),
        Span::raw(": Quit"),
    ]
}

/// Session label spans (`session: <label>`).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::status_hints::session_label_spans;
/// let spans = session_label_spans("abc");
/// let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
/// assert!(text.contains("abc"));
/// ```
pub fn session_label_spans(session_label: &str) -> Vec<Span<'static>> {
    vec![
        Span::styled("session", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(session_label.to_string(), Style::default().fg(Color::Cyan)),
    ]
}

/// Style a keyboard shortcut key in yellow.
fn kb(key: &str) -> Span<'static> {
    Span::styled(key.to_string(), Style::default().fg(Color::Yellow))
}
