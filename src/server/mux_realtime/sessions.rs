//! JSON projection of the mux registry, ordered as the primary agent roster.

use axum::{Json, http::StatusCode};

pub(super) async fn list() -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let sessions = crate::mux::control::list_sessions()
        .await
        .map_err(|error| {
            tracing::error!(%error, "Failed to list mux realtime sessions");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let values = sessions
        .into_iter()
        .map(super::session_projection::project)
        .collect();
    Ok(Json(values))
}
