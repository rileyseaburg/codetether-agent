//! Resolve the configured session event archive directory.
use crate::{ReplayError, ReplayQuery};
use axum::http::StatusCode;
pub(crate) fn session_dir(query: &ReplayQuery) -> Result<std::path::PathBuf, ReplayError> {
    use std::path::PathBuf;

    let base_dir = std::env::var("CODETETHER_EVENT_STREAM_PATH")
        .map(PathBuf::from)
        .ok()
        .ok_or_else(|| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Event stream not configured. Set CODETETHER_EVENT_STREAM_PATH.".to_string(),
            )
        })?;

    let session_dir = base_dir.join(&query.session_id);

    // Check if session directory exists
    if !session_dir.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Session not found: {}", query.session_id),
        ));
    }

    Ok(session_dir)
}
