//! Construct execution provenance from runtime identity and context.

use super::{AgentIdentity, ExecutionOrigin, ExecutionProvenance};
use chrono::Utc;
use uuid::Uuid;
impl ExecutionProvenance {
    ///
    /// # Examples
    /// ```
    /// use codetether_provenance::{ExecutionProvenance};
    /// let record = ExecutionProvenance::for_session("session", "worker");
    /// assert_eq!(record.session_id.as_deref(),Some("session"));
    /// ```
    pub fn for_session(session_id: &str, agent_name: &str) -> Self {
        Self::new(Some(session_id.to_string()), agent_name, runtime_origin())
    }

    ///
    /// # Examples
    /// ```
    /// use codetether_provenance::{ExecutionProvenance, ExecutionOrigin};
    /// let mut record = ExecutionProvenance::for_operation("worker", ExecutionOrigin::Worker);
    /// assert_eq!(record.identity.origin,ExecutionOrigin::Worker);
    /// ```
    pub fn for_operation(agent_name: &str, origin: ExecutionOrigin) -> Self {
        Self::new(None, agent_name, origin)
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
                agent_identity_id: crate::runtime_agent_identity(),
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
