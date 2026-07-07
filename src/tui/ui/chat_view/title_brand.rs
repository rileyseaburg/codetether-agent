//! Gradient `CodeTether` wordmark spans for the chat-panel title.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::tui::ui::gradient::{NEON_CYAN, NEON_MAGENTA, gradient_spans, rgb_supported};

/// The `CodeTether` wordmark: cyan→magenta gradient on truecolor
/// terminals, bold white elsewhere.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::title::brand::brand_spans;
/// assert!(!brand_spans().is_empty());
/// ```
pub fn brand_spans() -> Vec<Span<'static>> {
    if rgb_supported() {
        gradient_spans("CodeTether", NEON_CYAN, NEON_MAGENTA, true)
    } else {
        vec![Span::styled(
            "CodeTether",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]
    }
}
