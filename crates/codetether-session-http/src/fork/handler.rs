//! Validate the HTTP identity and persist an independent branch.
use super::{error::error, request::ForkRequest};
use axum::{Json, extract::Path, http::StatusCode};

/// Create a separate conversation containing only the context before the edited prompt.
pub(super) async fn fork<H: codetether_session_fork::ForkSession + serde::Serialize>(
    Path(id): Path<String>,
    Json(request): Json<ForkRequest>,
) -> Result<Json<H>, (StatusCode, String)> {
    if uuid::Uuid::parse_str(&id).is_err() {
        return Err((StatusCode::BAD_REQUEST, "Invalid session ID".into()));
    }
    codetether_session_fork::create_persisted::<H>(
        &id,
        request.before_message,
        &request.expected_text,
    )
    .await
    .map(Json)
    .map_err(error)
}
