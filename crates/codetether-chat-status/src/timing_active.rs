//! In-flight first-token and elapsed-time badges.
use crate::{RequestTimings, format_duration_ms, request_span::request_timing_span};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};
pub(super) fn spans(
    timings: RequestTimings,
    elapsed: impl FnOnce() -> Option<u64>,
) -> Option<Vec<Span<'static>>> {
    let mut spans = Vec::new();

    spans.push(request_timing_span(
        "TTFT",
        timings.current_first_token_ms,
        Color::Cyan,
        true,
    ));
    if let Some(elapsed_ms) = elapsed() {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            format!(" ELAPSED {} ", format_duration_ms(elapsed_ms)),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    }
    Some(spans)
}
