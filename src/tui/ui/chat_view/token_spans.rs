//! Push token-usage spans from [`TokenDisplay`] into a span buffer.

use ratatui::text::Span;

use crate::tui::theme::Theme;
use crate::tui::theme_utils::validate_theme;
use crate::tui::token_display::TokenDisplay;

/// Append token-usage spans by cloning `Cow` content to `'static`.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::token_spans::push_token_spans;
/// let mut spans: Vec<ratatui::text::Span<'static>> = Vec::new();
/// push_token_spans(&mut spans);
/// ```
pub fn push_token_spans(spans: &mut Vec<Span<'static>>) {
    let theme = validate_theme(&Theme::default());
    let token_display = TokenDisplay::new();
    let token_line = token_display.create_status_bar(&theme);
    for s in token_line.spans {
        spans.push(Span::styled(s.content.into_owned(), s.style));
    }
}
