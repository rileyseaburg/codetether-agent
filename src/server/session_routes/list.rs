//! Session list request parsing and response projection.

use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use serde::Deserialize;

/// Optional pagination applied after loading session summaries.
#[derive(Deserialize)]
pub(super) struct ListSessionsQuery {
    limit: Option<usize>,
    offset: Option<usize>,
}

/// Return one bounded page of persisted session summaries.
pub(super) async fn list(
    Query(query): Query<ListSessionsQuery>,
) -> Result<Json<Vec<crate::session::SessionSummary>>, (StatusCode, String)> {
    let sessions = crate::session::list_sessions()
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(100);
    Ok(Json(
        sessions.into_iter().skip(offset).take(limit).collect(),
    ))
}
