//! Status text span (yellow when processing, green when idle).

use ratatui::{
    style::{Color, Style},
    text::Span,
};

use crate::tui::app::state::App;

/// Status text colored yellow (processing) or green (idle).
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::status_text::status_text_span;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let span = status_text_span(app);
/// assert!(!span.content.is_empty() || app.state.status.is_empty());
/// # }
/// ```
pub fn status_text_span(app: &App) -> Span<'static> {
    Span::styled(
        app.state.status.clone(),
        Style::default().fg(if app.state.processing {
            Color::Yellow
        } else {
            Color::Green
        }),
    )
}
