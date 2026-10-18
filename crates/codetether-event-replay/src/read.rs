//! Parse and filter JSONL event records without changing archive files.
use crate::{ReplayError, ReplayQuery};
use axum::http::StatusCode;
use std::path::Path;
pub(crate) fn events(
    path: &Path,
    start: u64,
    end: u64,
    query: &ReplayQuery,
    all_events: &mut Vec<(u64, u64, serde_json::Value)>,
) -> Result<(), ReplayError> {
    // Read and parse events from this file
    let content = std::fs::read_to_string(&path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(event) = serde_json::from_str::<serde_json::Value>(line) {
            // Filter by tool name if specified
            if let Some(ref tool_filter) = query.tool_name {
                if let Some(event_tool) = event.get("tool_name").and_then(|v| v.as_str()) {
                    if event_tool != tool_filter {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            all_events.push((start, end, event));
        }
    }
    Ok(())
}
