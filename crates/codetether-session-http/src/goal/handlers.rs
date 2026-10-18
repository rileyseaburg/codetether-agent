//! Validate session access before invoking the human goal controller.
use super::{GoalHost, error::error};
use axum::{Json, extract::Path, http::StatusCode};
use codetether_goal_core::GoalEdit;
use serde_json::Value;
type HttpError = (StatusCode, String);

/// Read native goal state through the existing session auth/policy layers.
pub(super) async fn get<H: GoalHost>(Path(id): Path<String>) -> Result<Json<Value>, HttpError> {
    session::<H>(&id).await?;
    H::read_goal(&id).await.map(Json).map_err(error)
}

/// Apply one explicit user edit, with native revision and lifecycle checks.
pub(super) async fn post<H: GoalHost>(
    Path(id): Path<String>,
    Json(edit): Json<GoalEdit>,
) -> Result<Json<Value>, HttpError> {
    session::<H>(&id).await?;
    H::update_goal(&id, edit).await.map(Json).map_err(error)
}

/// Reject unsafe identifiers and unknown sessions before touching task logs.
async fn session<H: GoalHost>(id: &str) -> Result<(), HttpError> {
    if id.is_empty()
        || !id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err((StatusCode::BAD_REQUEST, "invalid session ID".into()));
    }
    H::load(id)
        .await
        .map(|_| ())
        .map_err(|_| (StatusCode::NOT_FOUND, "session not found".into()))
}
