//! Runnable snapshots retain configuration without claiming or altering tasks.

use super::task;
use crate::server::task_queue::KnativeTaskQueue;

#[tokio::test]
async fn pending_snapshot_preserves_configuration_and_queue_state() {
    let queue = KnativeTaskQueue::new();
    let statuses = [
        "pending",
        "queued",
        "processing",
        "working",
        "completed",
        "failed",
        "cancelled",
        "unknown",
    ];
    for status in statuses {
        let mut original = task(status);
        original.task_id = status.into();
        original.model = Some("provider/model".into());
        original.metadata = Some(serde_json::json!({"codebase_id": "workspace-1"}));
        queue.push(original).await;
    }
    let before = serde_json::to_value(queue.list().await).unwrap();
    let pending = queue.snapshot_pending().await;
    assert_eq!(pending.len(), 2);
    for (snapshot, status) in pending.iter().zip(["pending", "queued"]) {
        assert_eq!(snapshot.task_id, status);
        assert_eq!(snapshot.status, status);
        assert_eq!(snapshot.model.as_deref(), Some("provider/model"));
        assert_eq!(
            snapshot.metadata.as_ref().unwrap()["codebase_id"],
            "workspace-1"
        );
        assert_eq!(
            serde_json::to_value(snapshot).unwrap(),
            serde_json::to_value(queue.get(status).await.unwrap()).unwrap()
        );
    }
    assert_eq!(serde_json::to_value(queue.list().await).unwrap(), before);
    queue.claim("pending").await.unwrap();
    assert_eq!(pending[0].status, "pending");
    let refreshed = queue.snapshot_pending().await;
    assert_eq!(refreshed.len(), 1);
    assert_eq!(refreshed[0].task_id, "queued");
    queue.update_status("queued", "cancelled").await;
    assert!(queue.snapshot_pending().await.is_empty());
    assert!(KnativeTaskQueue::new().snapshot_pending().await.is_empty());
}
