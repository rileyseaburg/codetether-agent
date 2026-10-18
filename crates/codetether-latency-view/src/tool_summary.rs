//! Aggregate timed tools by name, ranking by average and last duration.
use crate::{LatencyView, tool_stats::ToolLatencySummary};
use std::collections::HashMap;

pub(crate) fn summarize_tool_latencies(
    view: &LatencyView<'_>,
) -> Vec<(String, ToolLatencySummary)> {
    let mut by_tool: HashMap<String, ToolLatencySummary> = HashMap::new();

    for row in &view.tools {
        let name = &row.name;
        let success = &row.success;
        let duration_ms = &row.duration_ms;
        let entry = by_tool.entry(name.clone()).or_default();
        entry.count += 1;
        entry.total_ms += *duration_ms;
        entry.max_ms = entry.max_ms.max(*duration_ms);
        entry.last_ms = *duration_ms;
        if !*success {
            entry.failures += 1;
        }
    }

    let mut summaries: Vec<_> = by_tool.into_iter().collect();
    summaries.sort_by(|a, b| {
        let a_avg = a.1.total_ms / a.1.count.max(1) as u64;
        let b_avg = b.1.total_ms / b.1.count.max(1) as u64;
        b_avg
            .cmp(&a_avg)
            .then_with(|| b.1.last_ms.cmp(&a.1.last_ms))
            .then_with(|| a.0.cmp(&b.0))
    });
    summaries
}
