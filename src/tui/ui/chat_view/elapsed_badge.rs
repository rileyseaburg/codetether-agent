//! In-flight elapsed-time badge for streaming previews.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::tui::app::state::AppState;

/// Build an elapsed-time badge span for the streaming header.
pub fn elapsed_badge(state: &AppState) -> Span<'static> {
    let label = state
        .current_request_elapsed_ms()
        .map(elapsed_label)
        .unwrap_or_default();
    Span::styled(
        label,
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::DIM),
    )
}

fn elapsed_label(ms: u64) -> String {
    if ms >= 1_000 {
        format!(" {:.1}s", ms as f64 / 1_000.0)
    } else {
        format!(" {ms}ms")
    }
}
