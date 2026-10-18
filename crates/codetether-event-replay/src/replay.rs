//! Replay matching events in byte-offset order.
use crate::{ReplayError, ReplayQuery};
use axum::Json;
use axum::extract::Query;
/// Replay archived JSONL events in byte-offset order.
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
/// let _ = codetether_event_replay::replay_session_events(axum::extract::Query(query)).await;
/// # }
/// ```
pub async fn replay_session_events(
    Query(query): Query<ReplayQuery>,
) -> Result<Json<Vec<serde_json::Value>>, ReplayError> {
    let session_dir = crate::directory::session_dir(&query)?;
    let mut all_events = crate::scan::events(&session_dir, &query)?;
    // Sort by start offset
    all_events.sort_by_key(|(s, _, _)| *s);

    // Apply limit
    let limit = query.limit.unwrap_or(1000).min(10000);
    let events: Vec<_> = all_events
        .into_iter()
        .take(limit)
        .map(|(_, _, e)| e)
        .collect();

    Ok(Json(events))
}
