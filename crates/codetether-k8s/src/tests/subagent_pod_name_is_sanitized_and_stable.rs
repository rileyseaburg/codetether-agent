//! Manager subagent_pod_name_is_sanitized_and_stable regression.

use super::*;

#[test]
fn subagent_pod_name_is_sanitized_and_stable() {
    let pod_name = K8sManager::subagent_pod_name("SubTask_ABC/123");
    assert!(pod_name.starts_with("codetether-subagent-"));
    assert!(pod_name.len() <= 63);
    assert!(
        pod_name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    );

    let pod_name_again = K8sManager::subagent_pod_name("SubTask_ABC/123");
    assert_eq!(pod_name, pod_name_again);
}
