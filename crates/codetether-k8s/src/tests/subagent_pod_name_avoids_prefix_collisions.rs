//! Manager subagent_pod_name_avoids_prefix_collisions regression.

use super::*;

#[test]
fn subagent_pod_name_avoids_prefix_collisions() {
    let a = K8sManager::subagent_pod_name("subtask-aaaaaaaa-1111");
    let b = K8sManager::subagent_pod_name("subtask-aaaaaaaa-2222");
    assert_ne!(a, b);
}
