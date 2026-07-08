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

/// Build the four-shortcut compact keybinding hint spans.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::compact_hints::compact_keybinding_spans;
/// let spans = compact_keybinding_spans();
/// let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
/// assert!(text.contains("Help"));
/// assert!(text.contains("Quit"));
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
