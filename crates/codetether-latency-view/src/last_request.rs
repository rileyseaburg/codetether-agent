//! Most recent completed request timings.
use crate::{LatencyView, format::format_duration};
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>, view: &LatencyView<'_>) {
    match (
        view.last_request_first_token_ms,
        view.last_request_last_token_ms,
    ) {
        (Some(ttft_ms), Some(ttl_ms)) => lines.push(Line::from(format!(
            "Last request: TTFT {} | TTL {}",
            format_duration(ttft_ms),
            format_duration(ttl_ms)
        ))),
        (Some(ttft_ms), None) => lines.push(Line::from(format!(
            "Last request: TTFT {} | TTL unavailable",
            format_duration(ttft_ms)
        ))),
        (None, Some(ttl_ms)) => lines.push(Line::from(format!(
            "Last request: TTFT unavailable | TTL {}",
            format_duration(ttl_ms)
        ))),
        (None, None) => lines.push(Line::from("Last request: no token timing yet")),
    }
}
