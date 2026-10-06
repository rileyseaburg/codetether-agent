//! Unknown sessions must never become newly created recovery sessions.
use super::super::{ResumeRequest, http_error, resume};
use super::storage;
use crate::session::Session;
use axum::http::StatusCode;

#[tokio::test]
async fn missing_resume_never_creates_replacement_storage() {
    if storage::isolated(concat!(
        module_path!(),
        "::missing_resume_never_creates_replacement_storage"
    )) {
        return;
    }
    let id = uuid::Uuid::new_v4().to_string();
    let request = ResumeRequest {
        prompt: None,
        agent: None,
        model: None,
    };
    let workspace = std::env::current_dir().unwrap();
    let (status, message) = http_error(resume(&id, request, &workspace).await.unwrap_err());
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(message, "Session not found");
    assert!(!Session::sessions_dir().unwrap().exists());
}
