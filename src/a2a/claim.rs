use crate::provenance::ClaimProvenance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskClaimResponse {
    pub task_id: String,
    pub worker_id: String,
    pub run_id: Option<String>,
    pub attempt_id: Option<String>,
    pub tenant_id: Option<String>,
    pub user_id: Option<String>,
    pub agent_identity_id: Option<String>,
    #[serde(default)]
    pub task_timeout_seconds: Option<u64>,
    #[serde(default)]
    pub provider_keys: Option<serde_json::Value>,
    #[serde(default)]
    pub provider_key_source: Option<String>,
}

impl TaskClaimResponse {
    pub fn into_provenance(self) -> ClaimProvenance {
        ClaimProvenance {
            worker_id: self.worker_id,
            task_id: self.task_id,
            run_id: self.run_id,
            attempt_id: self.attempt_id,
            tenant_id: self.tenant_id,
            agent_identity_id: self
                .agent_identity_id
                .or_else(|| self.user_id.map(|user_id| format!("user:{user_id}"))),
        }
    }
}

#[cfg(test)]
mod tests {
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
}
