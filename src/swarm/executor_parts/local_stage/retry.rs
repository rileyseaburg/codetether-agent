//! Host adapter for the shared sub-agent retry policy.
use super::job::Job;
use crate::provider::ToolDefinition;
use crate::tool::ToolRegistry;
use async_trait::async_trait;
use codetether_swarm_core::agent_retry::AgentAttempt;
pub(super) use codetether_swarm_core::agent_retry::AgentOutcome as Outcome;
use std::sync::Arc;
struct LocalAttempt<'a> {
    job: &'a Job,
    prompts: &'a (String, String),
    registry: Arc<ToolRegistry>,
    tools: Vec<ToolDefinition>,
}
#[async_trait]
impl AgentAttempt for LocalAttempt<'_> {
    async fn execute(&self) -> Outcome {
        super::retry_attempt::execute(self.job, self.prompts, &self.registry, &self.tools).await
    }
}
pub(super) async fn execute(
    job: &Job,
    prompts: &(String, String),
    registry: Arc<ToolRegistry>,
    tools: Vec<ToolDefinition>,
) -> (Outcome, u32) {
    let agent = LocalAttempt {
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
