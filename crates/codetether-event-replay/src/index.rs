//! List metadata for archived session-event files.
use crate::{EventFileMeta, ReplayError, ReplayQuery};
use axum::{Json, extract::Query, http::StatusCode};
/// List archived JSONL file metadata in byte-offset order.
/// # Arguments
/// * `query` - Session ID and optional event replay filters.
/// # Returns
/// The requested archive data as JSON.
/// # Errors
/// Returns unavailable if no archive is configured, not found for a missing
/// session, or an internal-server error when directory or file access fails.
/// # Examples
/// ```no_run
/// # async fn example(query: codetether_event_replay::ReplayQuery) {
/// let _ = codetether_event_replay::list_session_event_files(axum::extract::Query(query)).await;
/// # }
/// ```
pub async fn list_session_event_files(
    Query(query): Query<ReplayQuery>,
) -> Result<Json<Vec<EventFileMeta>>, ReplayError> {
    let session_dir = crate::directory::session_dir(&query)?;
    let mut files = Vec::new();
    let entries = std::fs::read_dir(&session_dir)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    for entry in entries {
        let entry = entry.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let path = entry.path();
        let Some((start, end)) = crate::range::offsets(&path) else {
            continue;
        };
        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let metadata = std::fs::metadata(&path)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        files.push(EventFileMeta {
            filename: filename.to_string(),
            start_offset: start,
            end_offset: end,
            size_bytes: metadata.len(),
        });
    }
    files.sort_by_key(|f| f.start_offset);
    Ok(Json(files))
}
