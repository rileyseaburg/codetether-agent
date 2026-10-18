//! Completed-request first/last-token badges.
use crate::{RequestTimings, request_span::request_timing_span};
use ratatui::{style::Color, text::Span};
pub(super) fn spans(timings: RequestTimings) -> Option<Vec<Span<'static>>> {
    let mut spans = Vec::new();

    if timings.last_first_token_ms.is_none() && timings.last_last_token_ms.is_none() {
        return None;
    }

    spans.push(request_timing_span(
        "TTFT",
        timings.last_first_token_ms,
        Color::Cyan,
        false,
    ));
    spans.push(Span::raw(" "));
    spans.push(request_timing_span(
        "TTL",
        timings.last_last_token_ms,
        Color::Green,
        false,
    ));
    Some(spans)
}
