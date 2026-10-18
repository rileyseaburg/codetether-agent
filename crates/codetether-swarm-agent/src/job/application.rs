//! Connect the shared job lifecycle to registry and agent-loop adapters.
use super::JobHost;
use codetether_provider_core::ToolDefinition;
use codetether_swarm_core::{agent_retry::AgentOutcome, prompt_policy::SystemPromptInput};
use codetether_swarm_local_job::{Host, Job};
use std::{marker::PhantomData, sync::Arc};
pub(super) struct ApplicationJob<H>(PhantomData<fn() -> H>);
impl<H: JobHost> Host for ApplicationJob<H> {
    type Registry = H::Registry;
    fn system_prompt(input: SystemPromptInput<'_>) -> String {
        H::system_prompt(input)
    }
    fn registry(job: &Job) -> (Arc<Self::Registry>, Vec<ToolDefinition>) {
        super::registry::create::<H>(job)
    }
    async fn attempt(
        job: &Job,
        prompts: &(String, String),
        registry: &Arc<Self::Registry>,
        tools: &[ToolDefinition],
    ) -> AgentOutcome {
        super::attempt::execute::<H>(job, prompts, registry, tools).await
    }
}
