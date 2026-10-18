//! Goal controller failures mapped without leaking storage details.
use axum::http::StatusCode;
use codetether_goal_core::control;
type HttpError = (StatusCode, String);

/// Map typed controller failures without leaking storage paths or secrets.
pub(super) fn error(error: control::GoalControlError) -> HttpError {
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
