//! Owned token/cost spans from the live presenter.
use crate::TokenDisplay;
use codetether_theme::{Theme, terminal::validate_theme};
use ratatui::text::Span;

/// Append token-usage spans by cloning `Cow` content to `'static`.
///
/// # Arguments
/// * `spans` - Destination for owned live token and cost spans.
///
/// # Returns
/// Appends status spans from the shared token presenter.
///
/// # Examples
/// ```
/// let mut spans=Vec::new();
/// codetether_token_display::token_spans::push_token_spans(&mut spans);
/// ```
pub fn push_token_spans(spans: &mut Vec<Span<'static>>) {
    let theme = validate_theme(&Theme::default());
    let token_display = TokenDisplay::new();
    let token_line = token_display.create_status_bar(&theme);
    for s in token_line.spans {
        spans.push(Span::styled(s.content.into_owned(), s.style));
    }
}
