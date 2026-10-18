//! Most recent timed tool outcome.
use crate::{LatencyView, format::format_duration};
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>, view: &LatencyView<'_>) {
    if let Some(latency_ms) = view.last_tool_latency_ms {
        let tool = view
            .last_tool_name
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let outcome = if view.last_tool_success.unwrap_or(false) {
            "ok"
        } else {
            "fail"
        };
        lines.push(Line::from(format!(
            "Last tool: {tool} in {} [{outcome}]",
            format_duration(latency_ms)
        )));
    } else {
        lines.push(Line::from("Last tool: no timed tool executions yet"));
    }
}
