use tokio::sync::mpsc;

use crate::approval::{LiveApprovalDecision, LiveApprovalRequest};
use crate::session::SessionEvent;

#[tokio::test]
async fn request_event_resumes_when_decided() {
    let (tx, mut rx) = mpsc::channel(1);
    let id = uuid::Uuid::new_v4().to_string();
    let req = LiveApprovalRequest::new(
        id.clone(),
        "call-1".into(),
        "bash".into(),
        "execute".into(),
        "bash:abc".into(),
        "runtime policy".into(),
    );
    let wait = tokio::spawn(async move { super::request(&tx, req).await });
    match rx.recv().await {
        Some(SessionEvent::ApprovalRequest(req)) => assert_eq!(req.approval_id, id),
        other => panic!("expected approval request, got {other:?}"),
    }
    assert!(super::decide(&id, LiveApprovalDecision::Approved));
    assert_eq!(
        wait.await.expect("approval task"),
        LiveApprovalDecision::Approved
    );
}
