//! Translate fork failures without exposing storage paths.
use axum::http::StatusCode;

/// Translate typed failures without exposing storage paths or server internals.
pub(super) fn error(failure: codetether_session_fork::ForkError) -> (StatusCode, String) {
    match failure {
        codetether_session_fork::ForkError::Conflict => (StatusCode::CONFLICT, failure.to_string()),
        codetether_session_fork::ForkError::Storage(source) => {
            tracing::error!(error = %source, "Could not fork conversation");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not create edited conversation".into(),
            )
        }
    }
}
