//! Denied recovery must not modify a real durable session, even with overrides.
use super::super::{ResumeRequest, http_error, resume};
use super::storage;
use crate::session::Session;
use axum::http::StatusCode;

#[tokio::test]
async fn denied_resume_preserves_durable_metadata_and_history() {
    if storage::isolated(concat!(
        module_path!(),
        "::denied_resume_preserves_durable_metadata_and_history"
    )) {
        return;
    }
    let session = storage::durable().await;
    let workspace = session.metadata.directory.as_ref().unwrap();
    let before = Session::export_json(&session.id).await.unwrap();
    let locator = Session::session_path(&session.id).unwrap();
    let locator_before = std::fs::read(&locator).unwrap();
    let foreign = workspace.join("foreign");
    std::fs::create_dir(&foreign).unwrap();
    let file = workspace.join("not-a-directory");
    std::fs::write(&file, "fixture").unwrap();
    for binding in [foreign, workspace.join("missing"), file] {
        let request = ResumeRequest {
            prompt: Some("must not execute".into()),
            agent: Some("plan".into()),
            model: Some("must-not-change".into()),
        };
        let (status, message) =
            http_error(resume(&session.id, request, &binding).await.unwrap_err());
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert!(!message.contains(workspace.to_str().unwrap()));
        assert_eq!(Session::export_json(&session.id).await.unwrap(), before);
        assert_eq!(std::fs::read(&locator).unwrap(), locator_before);
    }
}
