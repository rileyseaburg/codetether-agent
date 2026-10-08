//! HTTP adaptation for non-destructive editing of an earlier user prompt.
use axum::{Json, extract::Path, http::StatusCode};
use serde::Deserialize;

#[path = "fork_error.rs"]
mod errors;
#[path = "fork_repository.rs"]
mod repository;
#[path = "fork_service.rs"]
mod service;
#[cfg(test)]
#[path = "fork_tests.rs"]
mod tests;

/// Bind the edit to the exact message observed by the client.
#[derive(Deserialize)]
pub(super) struct ForkRequest {
    before_message: usize,
    expected_text: String,
}

/// Create a separate conversation containing only the context before the edited prompt.
pub(super) async fn fork(
    Path(id): Path<String>,
    Json(request): Json<ForkRequest>,
) -> Result<Json<crate::session::Session>, (StatusCode, String)> {
    if uuid::Uuid::parse_str(&id).is_err() {
        return Err((StatusCode::BAD_REQUEST, "Invalid session ID".into()));
    }
    repository::create(&id, request.before_message, &request.expected_text)
        .await
        .map(Json)
        .map_err(error)
}

/// Translate typed failures without exposing storage paths or server internals.
fn error(failure: service::ForkError) -> (StatusCode, String) {
    match failure {
        service::ForkError::Conflict => (StatusCode::CONFLICT, failure.to_string()),
        service::ForkError::Storage(source) => {
            tracing::error!(error = %source, "Could not fork conversation");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not create edited conversation".into(),
            )
        }
    }
}
