//! Current in-flight request timing.
use crate::{LatencyView, format::format_duration};
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>, view: &LatencyView<'_>) {
    if view.processing {
        let elapsed = view
            .current_request_elapsed_ms
            .map(format_duration)
            .unwrap_or_else(|| "running".to_string());
        let ttft = view
            .current_request_first_token_ms
            .map(format_duration)
            .unwrap_or_else(|| "waiting".to_string());
        lines.push(Line::from(format!(
            "Current request: in flight for {elapsed} | TTFT {ttft}"
        )));
    } else {
        lines.push(Line::from("Current request: idle"));
    }
}
