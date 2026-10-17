//! Claim identity conversion regression.

use super::TaskClaimResponse;

#[test]
fn maps_user_id_into_agent_identity() {
    let provenance = TaskClaimResponse {
        task_id: "task-1".to_string(),
        worker_id: "worker-1".to_string(),
        run_id: Some("run-1".to_string()),
        attempt_id: Some("run-1:attempt:2".to_string()),
        tenant_id: Some("tenant-1".to_string()),
        user_id: Some("user-1".to_string()),
        agent_identity_id: None,
        task_timeout_seconds: None,
        provider_keys: None,
        provider_key_source: None,
    }
    .into_provenance();
    assert_eq!(provenance.agent_identity_id.as_deref(), Some("user:user-1"));
}
