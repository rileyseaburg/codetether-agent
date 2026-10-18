use tokio::sync::mpsc;

use crate::{LiveApprovalDecision, LiveApprovalRequest};
use codetether_session_events::SessionEvent;

#[tokio::test]
async fn request_event_resumes_when_decided() {
    let _lock = crate::test_support::lock();
    let (tx, mut rx) = mpsc::channel::<SessionEvent<()>>(1);
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
