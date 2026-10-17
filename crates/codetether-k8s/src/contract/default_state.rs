//! Sparse Kubernetes status defaults.
use super::*;
#[tokio::test]
async fn sparse_pod_defaults() {
    let pod = serde_json::json!({"apiVersion":"v1","kind":"Pod","metadata":{"name":"sparse"}});
    let (manager, _) = mock::manager(mock::State {
        pod,
        ..Default::default()
    });
    let state = manager
        .get_subagent_pod_state("sparse")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.phase, "Unknown");
    assert!(!state.ready);
    assert!(!state.terminated);
    assert_eq!(state.exit_code, None);
    assert_eq!(state.reason, None);
    assert_eq!(state.restart_count, 0);
}

#[tokio::test]
async fn missing_pod_is_none() {
    let (manager, _) = mock::manager(mock::State::default());
    assert!(
        manager
            .get_subagent_pod_state("missing")
            .await
            .unwrap()
            .is_none()
    );
}
