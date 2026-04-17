//! Auto-apply ON/OFF badge spans.

use ratatui::{
    style::{Color, Style},
    text::Span,
};

use crate::tui::app::state::App;

/// `auto-apply: ON/OFF` badge with color coding.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::auto_apply::auto_apply_spans;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let spans = auto_apply_spans(app);
/// assert!(spans.len() >= 3);
/// # }
/// ```
pub fn auto_apply_spans(app: &App) -> Vec<Span<'static>> {
    vec![
        Span::styled("auto-apply", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(
            if app.state.auto_apply_edits {
                "ON"
            } else {
                "OFF"
            },
            Style::default().fg(if app.state.auto_apply_edits {
                Color::Green
            } else {
                Color::Yellow
            }),
        ),
    ]
}
