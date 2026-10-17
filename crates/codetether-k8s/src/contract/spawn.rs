//! Pod manifest and successful mutation contract.
use super::*;
#[tokio::test]
async fn manifest_and_history() {
    let (manager, state) = mock::manager(mock::State::default());
    let spec = SubagentPodSpec {
        image: Some("fixture:v1".into()),
        env_vars: std::collections::HashMap::from([("EXAMPLE".into(), "yes".into())]),
        labels: std::collections::HashMap::from([("app".into(), "custom".into())]),
        command: Some(vec!["agent".into()]),
        args: Some(vec!["run".into()]),
    };
    let action = manager
        .spawn_subagent_pod_with_spec("Task_A/123", spec)
        .await
        .unwrap();
    assert_eq!(action.action, "spawn_subagent:Task_A/123");
    assert!(action.success);
    assert_eq!(manager.recent_actions(10).await.len(), 1);
    let state = state.lock().unwrap();
    let request = state
        .requests
        .iter()
        .find(|r| r.method == "POST" && r.path.ends_with("/pods"))
        .unwrap();
    let body = &request.body;
    super::manifest::check(body);
    let mut captured = body.clone();
    captured["metadata"]["annotations"]["codetether.run/lease-holder"] = "fixture-owner".into();
    println!("MANIFEST={captured}");
}
