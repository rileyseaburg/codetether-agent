//! Pod-state projection and scoped status contracts.
use super::*;
#[tokio::test]
async fn pod_state_and_deployment_counts() {
    let pod = serde_json::json!({"apiVersion":"v1","kind":"Pod","metadata":{"name":"task"},
        "status":{"phase":"Failed","conditions":[{"type":"Ready","status":"True"}],
        "containerStatuses":[{"name":"agent","image":"x","imageID":"x","ready":false,"restartCount":-1,
            "state":{"terminated":{"exitCode":7,"reason":"Error"}}}]}});
    let (manager, _) = mock::manager(mock::State {
        pod,
        ..Default::default()
    });
    let state = manager
        .get_subagent_pod_state("task")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.phase, "Failed");
    assert!(state.ready);
    assert!(state.terminated);
    assert_eq!(state.exit_code, Some(7));
    assert_eq!(state.reason.as_deref(), Some("Error"));
    assert_eq!(state.restart_count, 0);
    let status = manager.status().await;
    assert_eq!(status.replicas, Some(3));
    assert_eq!(status.available_replicas, Some(2));
    assert_eq!(
        manager.scoped(Some("other"), None).status().await.namespace,
        "other"
    );
    assert_eq!(
        manager.scoped(Some(" "), None).status().await.namespace,
        "fixture"
    );
}
