//! Most recent model round-trip timing.
use crate::{LatencyView, format::format_duration};
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>, view: &LatencyView<'_>) {
    if let Some(latency_ms) = view.last_completion_latency_ms {
        let model = view
            .last_completion_model
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let prompt = view.last_completion_prompt_tokens.unwrap_or_default();
        let output = view.last_completion_output_tokens.unwrap_or_default();
        lines.push(Line::from(format!(
            "Last model round-trip: {model} in {} ({} in / {} out)",
            format_duration(latency_ms),
            prompt,
            output
        )));
    } else {
        lines.push(Line::from("Last model round-trip: no request timing yet"));
    }
}
