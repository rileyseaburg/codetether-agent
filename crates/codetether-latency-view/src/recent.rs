//! Most recent timed tool rows, newest first.
use crate::recent_rows::recent_tool_latencies;
use crate::{LatencyView, format::section_heading};
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>, view: &LatencyView<'_>) {
    lines.push(Line::from(""));
    lines.push(section_heading("Recent Timed Tools"));
    let recent = recent_tool_latencies(view, 8);
    if recent.is_empty() {
        lines.push(Line::from("  No recent timed tools yet."));
    } else {
        for entry in recent {
            lines.push(Line::from(format!("  {entry}")));
        }
    }
}
