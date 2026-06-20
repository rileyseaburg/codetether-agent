//! Text compaction helpers for the summary panel.

use super::{summary::ProtocolSummary, truncate::truncate};

pub(super) fn worker_id(summary: &ProtocolSummary) -> String {
    summary
        .worker_id
        .clone()
        .unwrap_or_else(|| "n/a".to_string())
}

pub(super) fn worker_name(summary: &ProtocolSummary) -> String {
    summary
        .worker_name
        .clone()
        .unwrap_or_else(|| "n/a".to_string())
}

pub(super) fn registered_agents(summary: &ProtocolSummary) -> String {
    if summary.registered_agents.is_empty() {
        "none".to_string()
    } else {
        truncate(&summary.registered_agents.join(", "), 120)
    }
}

pub(super) fn recent_task(summary: &ProtocolSummary) -> String {
    let fallback = "No recent A2A tasks".to_string();
    truncate(&summary.recent_task.clone().unwrap_or(fallback), 120)
}
