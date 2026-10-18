//! Native prompt and workspace-registry construction for local jobs.
use super::super::workspace_registry;
use crate::{provider::ToolDefinition, swarm::tool_policy, tool::ToolRegistry};
use codetether_swarm_core::{agent_retry::AgentOutcome, prompt_policy::SystemPromptInput};
use codetether_swarm_local_job::{Host, Job};
use std::sync::Arc;
mod attempt;
pub(super) struct Native;
impl Host for Native {
    type Registry = ToolRegistry;
    fn system_prompt(input: SystemPromptInput<'_>) -> String {
        tool_policy::system_prompt(input)
    }
    fn registry(job: &Job) -> (Arc<ToolRegistry>, Vec<ToolDefinition>) {
        let capability = workspace_registry::Capability::from_task_flags(
            job.read_only,
            job.verification,
            job.expects_changes,
        );
        workspace_registry::for_agent(
            Arc::clone(&job.provider),
            job.model.clone(),
            &job.workspace,
            capability,
            Arc::clone(&job.result_store),
            job.id.clone(),
        )
    }
    async fn attempt(
        job: &Job,
        prompts: &(String, String),
        registry: &Arc<ToolRegistry>,
        tools: &[ToolDefinition],
    ) -> AgentOutcome {
        attempt::execute(job, prompts, registry, tools).await
    }
}
