//! Persisted decisions reach the durable active-session event stream.

use crate::approval::{ApprovalStore, LiveApprovalDecision, LiveApprovalRequest};
use crate::session::SessionEvent;
use tokio::sync::mpsc;

#[tokio::test]
async fn persisted_decision_emits_durable_metadata() {
    let _lock = crate::approval::test_env::lock_env();
    let temp = tempfile::tempdir().expect("temp");
    let _env = crate::approval::test_env::ScopedEnv::data_dir_with_access(
        temp.path(),
        crate::config::AccessMode::Ask,
    );
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:abc", "test")
        .expect("request");
    let live = LiveApprovalRequest::new(
        request.id.clone(),
        "call-1".into(),
        request.tool,
        request.action,
        request.resource,
        request.reason,
    );
    let (tx, mut rx) = mpsc::channel(4);
    let wait = tokio::spawn(async move { super::request(&tx, live).await });
    assert!(matches!(
        rx.recv().await,
        Some(SessionEvent::ApprovalRequest(_))
    ));
    store
        .approve(&request.id, "test", "approved")
        .expect("approve");
    assert!(super::decide(&request.id, LiveApprovalDecision::Approved));
    let Some(SessionEvent::ToolCallMetadata { metadata, .. }) = rx.recv().await else {
        panic!("expected decision metadata");
    };
    assert_eq!(metadata["approval_decision"]["request_id"], request.id);
    assert!(
        SessionEvent::ToolCallMetadata {
            tool_call_id: "call-1".into(),
            name: "bash".into(),
            metadata,
        }
        .is_durable()
    );
    assert_eq!(wait.await.expect("wait"), LiveApprovalDecision::Approved);
}
