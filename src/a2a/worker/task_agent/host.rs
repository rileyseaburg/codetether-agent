//! Native session and swarm settings for shared agent selection.
use super::super::{TaskContext, WorkerTaskRuntime};
use crate::session::{Session, SessionResult};
use codetether_a2a_worker_core::session_protocol::policy::PolicySessionResult;
use std::sync::Arc;
pub(super) struct Native<'a> {
    pub session: &'a mut Session,
    pub runtime: &'a WorkerTaskRuntime,
    pub context: &'a TaskContext,
    pub output: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
}
impl codetether_a2a_worker_core::execution_policy::agent::Host for Native<'_> {
    async fn swarm(&mut self) -> anyhow::Result<(SessionResult, bool)> {
        super::super::execute_swarm_with_policy(
            self.session,
            &self.context.prompt,
            self.runtime.auto_approve,
            self.context.model_tier.as_deref(),
            self.context.selected_model.clone(),
            &self.context.metadata,
            self.context.complexity_hint.as_deref(),
            self.context.worker_personality.as_deref(),
            self.context.target_agent_name.as_deref(),
            Some(&self.runtime.bus),
            self.output.clone(),
        )
        .await
    }
    async fn standard(&mut self) -> anyhow::Result<PolicySessionResult> {
        super::super::execute_session_with_policy(
            self.session,
            &self.context.prompt,
            self.runtime.auto_approve,
            self.context.model_tier.as_deref(),
            &self.context.metadata,
            self.output.take(),
        )
        .await
    }
}
