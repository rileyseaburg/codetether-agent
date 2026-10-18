//! Session creation request handling.

use axum::Json;
use axum::http::StatusCode;
use serde::Deserialize;

/// Optional metadata accepted when a session is created.
#[derive(Deserialize)]
pub(super) struct CreateSessionRequest {
    title: Option<String>,
    agent: Option<String>,
}

/// Create and persist one empty session.
pub(super) async fn create<H: crate::SessionHost>(
    Json(request): Json<CreateSessionRequest>,
) -> Result<Json<H>, (StatusCode, String)> {
    let mut session = H::new().await.map_err(internal_error)?;
    session.set_title(request.title);
    if let Some(agent) = request.agent {
        session.set_agent_name(agent);
    }
    session.save().await.map_err(internal_error)?;
    Ok(Json(session))
}

/// Convert session construction failures into an HTTP response.
fn internal_error(error: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
