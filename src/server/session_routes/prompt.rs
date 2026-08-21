//! Compatibility handler for blocking session prompts.

use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use serde::Deserialize;

/// User text accepted by the blocking prompt endpoint.
#[derive(Deserialize)]
pub(super) struct PromptRequest {
    message: String,
}

/// Execute one prompt and return only after its session is persisted.
pub(super) async fn prompt(
    Path(id): Path<String>,
    Json(request): Json<PromptRequest>,
) -> Result<Json<crate::session::SessionResult>, (StatusCode, String)> {
    if request.message.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Message cannot be empty".into()));
    }
    tracing::info!(
        session_id = %id,
        message_len = request.message.len(),
        "Received prompt request"
    );
    let mut session = crate::session::Session::load(&id)
        .await
        .map_err(|error| (StatusCode::NOT_FOUND, error.to_string()))?;
    let result = session.prompt(&request.message).await.map_err(|error| {
        tracing::error!(session_id = %id, %error, "Session prompt failed");
        (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    })?;
    session.save().await.map_err(|error| {
        tracing::error!(session_id = %id, %error, "Session save failed");
        (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    })?;
    Ok(Json(result))
}
