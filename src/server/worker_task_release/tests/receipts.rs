//! Accepted receipts survive polling and rejected releases cannot replace them.

use super::{super::service, queue, request};
use crate::{bus::AgentBus, server::task_queue::ReleaseError};

#[tokio::test]
async fn accepted_payload_is_retained_and_duplicate_is_rejected() {
    for status in ["completed", "failed"] {
        let tasks = queue().await;
        let bus = AgentBus::new().into_arc();
        let mut observer = bus.handle("receipt-observer");
        let mut req = request(status);
        req.result = Some("partial or final result".into());
        req.error = Some("worker diagnostic".into());
        req.session_id = Some("original-session".into());
        req.diagnostics = Some(serde_json::json!({"exit_code": 7}));
        assert_eq!(service::release(&tasks, &bus, &req).await.unwrap(), status);
        let accepted = tasks.get("task-1").await.unwrap();
        assert_eq!(accepted.completion.result, req.result);
        assert_eq!(accepted.completion.error, req.error);
        assert_eq!(accepted.completion.session_id, req.session_id);
        assert_eq!(accepted.completion.diagnostics, req.diagnostics);
        let mut duplicate = request("failed");
        duplicate.result = Some("replacement".into());
        duplicate.session_id = Some("replacement-session".into());
        assert!(matches!(
            service::release(&tasks, &bus, &duplicate).await,
            Err(ReleaseError::NotActive)
        ));
        let stored = tasks.get("task-1").await.unwrap();
        assert_eq!(
            serde_json::to_value(stored).unwrap(),
            serde_json::to_value(accepted).unwrap()
        );
        assert!(observer.try_recv().is_some());
        assert!(observer.try_recv().is_none());
    }
}

#[tokio::test]
async fn inactive_release_cannot_write_a_receipt() {
    let tasks = queue().await;
    assert!(tasks.update_status("task-1", "pending").await);
    let mut req = request("completed");
    req.result = Some("must not persist".into());
    req.session_id = Some("must not persist".into());
    let bus = AgentBus::new().into_arc();
    assert!(matches!(
        service::release(&tasks, &bus, &req).await,
        Err(ReleaseError::NotActive)
    ));
    let stored = serde_json::to_value(tasks.get("task-1").await.unwrap()).unwrap();
    assert!(stored.get("result").is_none());
}
