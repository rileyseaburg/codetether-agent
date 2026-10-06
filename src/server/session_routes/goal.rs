//! Authenticated HTTP adaptation for native session-goal controls.

use crate::session::tasks::{GoalEdit, control};
use axum::{Json, extract::Path, http::StatusCode};
use serde_json::Value;

type HttpError = (StatusCode, String);

/// Read native goal state through the existing session auth/policy layers.
pub(super) async fn get(Path(id): Path<String>) -> Result<Json<Value>, HttpError> {
    session(&id).await?;
    control::read(&id).await.map(Json).map_err(error)
}

/// Apply one explicit user edit, with native revision and lifecycle checks.
pub(super) async fn post(
    Path(id): Path<String>,
    Json(edit): Json<GoalEdit>,
) -> Result<Json<Value>, HttpError> {
    session(&id).await?;
    control::update_user(&id, edit)
        .await
        .map(Json)
        .map_err(error)
}

/// Reject unsafe identifiers and unknown sessions before touching task logs.
async fn session(id: &str) -> Result<(), HttpError> {
    if id.is_empty()
        || !id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err((StatusCode::BAD_REQUEST, "invalid session ID".into()));
    }
    crate::session::Session::load(id)
        .await
        .map(|_| ())
        .map_err(|_| (StatusCode::NOT_FOUND, "session not found".into()))
}

/// Map typed controller failures without leaking storage paths or secrets.
fn error(error: control::GoalControlError) -> HttpError {
    use control::GoalControlError::{Conflict, Invalid, Storage};
    match error {
        Invalid(message) => (StatusCode::BAD_REQUEST, message.into()),
        Conflict(message) => (StatusCode::CONFLICT, message.into()),
        Storage(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "native goal storage unavailable".into(),
        ),
    }
}
