//! Scan matching archive files and retain event offsets for ordering.
use crate::{ReplayError, ReplayQuery};
use axum::http::StatusCode;
use std::path::Path;
pub(crate) fn events(
    session_dir: &Path,
    query: &ReplayQuery,
) -> Result<Vec<(u64, u64, serde_json::Value)>, ReplayError> {
    let mut all_events = Vec::new();
    let entries = std::fs::read_dir(session_dir)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    for entry in entries {
        let entry = entry.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let path = entry.path();
        let Some((start, end)) = crate::range::offsets(&path) else {
            continue;
        };
        // Filter by byte range if specified
        if let Some(query_start) = query.start_offset
            && end <= query_start
        {
            continue;
        }
        if let Some(query_end) = query.end_offset
            && start >= query_end
        {
            continue;
        }

        crate::read::events(&path, start, end, query, &mut all_events)?;
    }
    Ok(all_events)
}
