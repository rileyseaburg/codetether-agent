//! HTTP classification of recovery errors without leaking storage paths.
use axum::http::StatusCode;
use std::io::{Error, ErrorKind};

pub(in crate::server) fn http_error(error: anyhow::Error) -> (StatusCode, String) {
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
