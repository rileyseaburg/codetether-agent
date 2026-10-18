//! Per-tool summary presentation.
use crate::tool_summary::summarize_tool_latencies;
use crate::{
    LatencyView,
    format::{format_duration, section_heading},
};
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>, view: &LatencyView<'_>) {
    lines.push(Line::from(""));
    lines.push(section_heading("Tool Latency In Chat"));

    let tool_summaries = summarize_tool_latencies(view);
    if tool_summaries.is_empty() {
        lines.push(Line::from(
            "  No timed tool executions recorded in the current chat buffer.",
        ));
    } else {
        for (tool_name, stats) in tool_summaries.iter().take(6) {
            let avg_ms = stats.total_ms / stats.count.max(1) as u64;
            lines.push(Line::from(format!(
                "  {}: avg {} | max {} | {} runs | {} failures",
                tool_name,
                format_duration(avg_ms),
                format_duration(stats.max_ms),
                stats.count,
                stats.failures
            )));
        }
    }
}
