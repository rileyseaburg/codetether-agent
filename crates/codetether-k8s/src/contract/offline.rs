//! Initialization and bounded action history without cluster access.
use super::*;
#[tokio::test]
async fn isolated_initialization_and_history() {
    let manager = K8sManager::new().await;
    assert!(!manager.is_available());
    let status = manager.status().await;
    assert_eq!(status.namespace, "fixture-offline");
    assert_eq!(status.pod_name.as_deref(), Some("fixture-pod"));
    for i in 0..1002 {
        manager
            .record_action(DeployAction {
                action: i.to_string(),
                success: true,
                message: String::new(),
                timestamp: String::new(),
            })
            .await;
    }
    let actions = manager.recent_actions(2000).await;
    assert_eq!(actions.len(), 1000);
    assert_eq!(actions[0].action, "1001");
    assert_eq!(actions[999].action, "2");
    assert!(manager.scale(1).await.is_err());
}
