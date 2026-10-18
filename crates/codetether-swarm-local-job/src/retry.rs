//! Host adapter for the shared sub-agent retry policy.
use super::job::Job;
use crate::Host;
use async_trait::async_trait;
use codetether_provider_core::ToolDefinition;
use codetether_swarm_core::agent_retry::AgentAttempt;
pub(super) use codetether_swarm_core::agent_retry::AgentOutcome as Outcome;
use std::sync::Arc;
struct LocalAttempt<'a, H: Host> {
    job: &'a Job,
    prompts: &'a (String, String),
    registry: Arc<H::Registry>,
    tools: Vec<ToolDefinition>,
}
#[async_trait]
impl<H: Host> AgentAttempt for LocalAttempt<'_, H> {
    async fn execute(&self) -> Outcome {
        H::attempt(self.job, self.prompts, &self.registry, &self.tools).await
    }
}
pub(super) async fn execute<H: Host>(
    job: &Job,
    prompts: &(String, String),
    registry: Arc<H::Registry>,
    tools: Vec<ToolDefinition>,
) -> (Outcome, u32) {
    let agent = LocalAttempt::<H> {
        job,
        prompts,
        registry,
        tools,
    };
    codetether_swarm_core::agent_retry::run(
        &agent,
        &job.id,
        job.max_retries,
        job.base_delay_ms,
        job.max_delay_ms,
    )
    .await
}
