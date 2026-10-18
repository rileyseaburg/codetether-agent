//! Gradient `CodeTether` wordmark spans for the chat-panel title.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use codetether_tui_style::gradient::{NEON_CYAN, NEON_MAGENTA, gradient_spans, rgb_supported};

/// Render the chat-panel brand wordmark.
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// Gradient or fallback brand spans.
///
/// # Examples
/// ```
/// assert!(!codetether_chat_status::title_brand::brand_spans().is_empty());
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
