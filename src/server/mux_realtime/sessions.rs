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
        .map(|session| {
            let windows = session
                .windows
                .into_iter()
                .map(|window| {
                    serde_json::json!({
                        "id": window.id,
                        "title": window.title,
                        "workspace": window.workspace,
                    })
                })
                .collect::<Vec<_>>();
            serde_json::json!({
                "name": session.name,
                "pid": session.pid,
                "active_window": session.active_window,
                "windows": windows,
                "reachable": session.reachable,
            })
        })
        .collect();
    Ok(Json(values))
}
