//! delete behavior contract.
use super::*;

#[tokio::test]
async fn missing_pod_delete_is_successful() {
    let (manager, _) = mock::manager(mock::State {
        missing_delete: true,
        ..Default::default()
    });
    assert!(manager.delete_subagent_pod("gone").await.unwrap().success);
    assert_eq!(
        manager.recent_actions(1).await[0].action,
        "delete_subagent:gone"
    );
}
