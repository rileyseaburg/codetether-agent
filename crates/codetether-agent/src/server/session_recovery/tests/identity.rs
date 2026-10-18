//! Authorized recovery retains the durable identity and history.
use super::super::{ResumeRequest, resume};
use super::storage;
use crate::session::Session;

#[tokio::test]
async fn authorized_resume_preserves_identity_and_history() {
    if storage::isolated(concat!(
        module_path!(),
        "::authorized_resume_preserves_identity_and_history"
    )) {
        return;
    }
    let original = storage::durable().await;
    let workspace = original.metadata.directory.as_ref().unwrap();
    let request = ResumeRequest {
        prompt: None,
        agent: Some("plan".into()),
        model: Some("updated-model".into()),
    };
    let response = resume(&original.id, request, &workspace.join("."))
        .await
        .unwrap();
    assert_eq!(response["session_id"], original.id);
    assert_eq!(response["active_session_id"], original.id);
    assert_eq!(response["status"], "ready");
    let recovered = Session::resume(&original.id).await.unwrap();
    assert_eq!(recovered.id, original.id);
    assert_eq!(recovered.messages.len(), original.messages.len());
    assert_eq!(
        serde_json::to_value(&recovered.messages).unwrap(),
        serde_json::to_value(&original.messages).unwrap(),
    );
    assert_eq!(recovered.agent, "plan");
    assert_eq!(recovered.metadata.model.as_deref(), Some("updated-model"));
    assert_eq!(recovered.metadata.directory, original.metadata.directory);
}
