use super::ClaimProvenance;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub tenant_id: Option<String>,
    pub agent_identity_id: Option<String>,
    pub agent_name: String,
    pub origin: ExecutionOrigin,
    pub worker_id: Option<String>,
    pub key_id: Option<String>,
    pub github_installation_id: Option<String>,
    pub github_app_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionOrigin {
    #[default]
    LocalCli,
    Worker,
    Ralph,
    Swarm,
    Relay,
}

impl ExecutionOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalCli => "local_cli",
            Self::Worker => "worker",
            Self::Ralph => "ralph",
            Self::Swarm => "swarm",
            Self::Relay => "relay",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProvenance {
    pub provenance_id: String,
    pub session_id: Option<String>,
    pub task_id: Option<String>,
    pub run_id: Option<String>,
    pub attempt_id: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub identity: AgentIdentity,
}

impl ExecutionProvenance {
    pub fn for_session(session_id: &str, agent_name: &str) -> Self {
        Self::new(Some(session_id.to_string()), agent_name, runtime_origin())
    }

    pub fn for_operation(agent_name: &str, origin: ExecutionOrigin) -> Self {
        Self::new(None, agent_name, origin)
    }

    pub fn set_agent_name(&mut self, agent_name: &str) {
        self.identity.agent_name = agent_name.to_string();
    }

    pub fn apply_worker_task(&mut self, worker_id: &str, task_id: &str) {
        self.identity.origin = ExecutionOrigin::Worker;
        self.identity.worker_id = Some(worker_id.to_string());
        self.task_id = Some(task_id.to_string());
    }

    pub fn apply_claim(&mut self, claim: &ClaimProvenance) {
        self.apply_worker_task(&claim.worker_id, &claim.task_id);
        self.run_id = claim.run_id.clone().or_else(|| self.run_id.clone());
        self.attempt_id = claim.attempt_id.clone().or_else(|| self.attempt_id.clone());
        self.identity.tenant_id = claim
            .tenant_id
            .clone()
            .or_else(|| self.identity.tenant_id.clone());
        self.identity.agent_identity_id = claim
            .agent_identity_id
            .clone()
            .or_else(|| self.identity.agent_identity_id.clone());
    }

    pub fn set_run_id(&mut self, run_id: impl Into<String>) {
        self.run_id = Some(run_id.into());
    }

    fn new(session_id: Option<String>, agent_name: &str, origin: ExecutionOrigin) -> Self {
        Self {
            provenance_id: format!("ctprov_{}", Uuid::new_v4().simple()),
            session_id,
            task_id: std::env::var("CODETETHER_TASK_ID").ok(),
            run_id: std::env::var("CODETETHER_RUN_ID").ok(),
            attempt_id: std::env::var("CODETETHER_ATTEMPT_ID").ok(),
            issued_at: Utc::now(),
            identity: AgentIdentity {
                tenant_id: std::env::var("CODETETHER_TENANT_ID").ok(),
                agent_identity_id: std::env::var("CODETETHER_AGENT_IDENTITY_ID").ok(),
                agent_name: agent_name.to_string(),
                origin,
                worker_id: std::env::var("CODETETHER_WORKER_ID").ok(),
                key_id: std::env::var("CODETETHER_KEY_ID").ok(),
                github_installation_id: std::env::var("CODETETHER_GITHUB_INSTALLATION_ID").ok(),
                github_app_id: std::env::var("CODETETHER_GITHUB_APP_ID").ok(),
            },
        }
    }
}

fn runtime_origin() -> ExecutionOrigin {
    if std::env::var("CODETETHER_WORKER_ID").is_ok() {
        ExecutionOrigin::Worker
    } else {
        ExecutionOrigin::LocalCli
    }
}
