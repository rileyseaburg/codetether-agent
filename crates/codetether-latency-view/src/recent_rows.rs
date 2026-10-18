//! Format recent timed tool entries in reverse chronological order.
use crate::{LatencyView, format::format_duration};
pub(crate) fn recent_tool_latencies(view: &LatencyView<'_>, limit: usize) -> Vec<String> {
    view.tools
        .iter()
        .rev()
        .map(|row| {
            let icon = if row.success { "OK" } else { "ER" };
            let name = &row.name;
            format!("[{icon}] {name} {}", format_duration(row.duration_ms))
        })
        .take(limit)
        .collect()
}
