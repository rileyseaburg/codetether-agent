//! HTTP classification of recovery errors without leaking storage paths.
use axum::http::StatusCode;
use std::io::{Error, ErrorKind};

/// Classify access and storage failures without exposing filesystem paths.
/// # Arguments
/// * `error` - Access or durable-storage failure, including context chains.
/// # Returns
/// HTTP status and a client-safe description.
/// # Examples
/// ```
/// let error = codetether_session_access::scope::ScopeError::Forbidden.into();
/// assert_eq!(codetether_session_access::http_error(error).0, axum::http::StatusCode::FORBIDDEN);
/// ```
pub fn http_error(error: anyhow::Error) -> (StatusCode, String) {
    if let Some(scope) = error.downcast_ref::<super::scope::ScopeError>() {
        let status = match scope {
            super::scope::ScopeError::Unauthenticated => StatusCode::UNAUTHORIZED,
            super::scope::ScopeError::Forbidden => StatusCode::FORBIDDEN,
            super::scope::ScopeError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        };
        return (status, scope.to_string());
    }
    let missing = error.chain().any(|cause| {
        cause
            .downcast_ref::<Error>()
            .is_some_and(|io| io.kind() == ErrorKind::NotFound)
    });
    tracing::warn!(error = %error, "Session recovery failed");
    if missing {
        (StatusCode::NOT_FOUND, "Session not found".into())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Session recovery failed".into(),
        )
    }
}
