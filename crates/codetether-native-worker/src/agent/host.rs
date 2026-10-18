//! Native standard and swarm calls retaining worker task settings.
use super::*;
use codetether_a2a_worker_core::session_protocol::policy::PolicySessionResult;
use codetether_session_events::SessionResult;
impl<S, T, W, E> codetether_a2a_worker_core::execution_policy::agent::Host
    for Native<'_, S, T, W, E>
where
    S: Session,
    T: Application,
    W: codetether_worker_tools::Host<Registry = ToolRegistry<T>>,
    E: codetether_swarm_runtime::inline::Executor<Bus = AgentBus>,
{
    async fn swarm(&mut self) -> anyhow::Result<(SessionResult, bool)> {
        crate::swarm_policy::execute::<S, T, W, E>(
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
        crate::session_policy::execute::<S, T, W>(
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
