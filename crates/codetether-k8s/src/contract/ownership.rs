//! ownership behavior contract.
use super::*;

#[tokio::test]
async fn occupied_lease_prevents_pod_write() {
    let lease = serde_json::json!({"apiVersion":"coordination.k8s.io/v1","kind":"Lease",
        "metadata":{"name":"other","resourceVersion":"1"},
        "spec":{"holderIdentity":"other","leaseDurationSeconds":30,"renewTime":"2999-01-01T00:00:00Z"}});
    let (manager, state) = mock::manager(mock::State {
        occupied: true,
        lease,
        ..Default::default()
    });
    assert!(
        manager
            .spawn_subagent_pod("task", None, Default::default())
            .await
            .is_err()
    );
    assert!(
        state
            .lock()
            .unwrap()
            .requests
            .iter()
            .all(|r| !r.path.contains("/pods"))
    );
}
