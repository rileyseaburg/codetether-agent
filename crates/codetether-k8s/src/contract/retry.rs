//! retry behavior contract.
use super::*;

#[tokio::test]
async fn existing_pod_is_replaced_once() {
    let (manager, state) = mock::manager(mock::State {
        conflict_once: true,
        ..Default::default()
    });
    assert!(
        manager
            .spawn_subagent_pod("task", None, Default::default())
            .await
            .unwrap()
            .success
    );
    let state = state.lock().unwrap();
    let methods: Vec<_> = state
        .requests
        .iter()
        .filter(|r| r.path.contains("/pods"))
        .map(|r| r.method.as_str())
        .collect();
    assert_eq!(methods, ["POST", "DELETE", "POST"]);
}
