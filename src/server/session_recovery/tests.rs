//! Focused storage-error regressions for identity-preserving recovery.
use super::http_error;
use axum::http::StatusCode;
use std::io::{Error, ErrorKind};

mod authority;
mod denial;
mod fixture;
mod identity;
mod missing;
mod paths;
mod storage;

#[test]
fn missing_session_is_not_found_even_with_context() {
    let error = anyhow::Error::new(Error::new(ErrorKind::NotFound, "private/session"))
        .context("loading session");
    assert_eq!(
        http_error(error),
        (StatusCode::NOT_FOUND, "Session not found".into())
    );
}

#[test]
fn corrupt_oversized_and_inaccessible_sessions_are_errors() {
    for error in [
        anyhow::anyhow!("malformed JSON in private/session"),
        anyhow::anyhow!("session file exceeds size limit"),
        anyhow::Error::new(Error::new(ErrorKind::PermissionDenied, "private/session")),
    ] {
        let (status, message) = http_error(error);
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(!message.contains("private/session"));
    }
}

#[test]
fn save_failure_is_never_completion() {
    let error = anyhow::Error::new(Error::new(ErrorKind::StorageFull, "save failed"));
    assert_eq!(http_error(error).0, StatusCode::INTERNAL_SERVER_ERROR);
}
