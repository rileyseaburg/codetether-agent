//! inactive release cannot write a receipt.
//! Accepted receipts survive polling and rejected releases cannot replace them.

use super::{queue, request};
use crate::release::service;

use codetether_bus_core::AgentBus;

use codetether_task_queue::ReleaseError;

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
