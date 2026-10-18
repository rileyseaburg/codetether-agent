//! Native prompt, workspace-registry and agent-loop callbacks.
use crate::Job;
use codetether_provider_core::ToolDefinition;
use codetether_swarm_core::{agent_retry::AgentOutcome, prompt_policy::SystemPromptInput};
use std::{future::Future, sync::Arc};
/// Application operations invoked by the shared local-job lifecycle.
/// # Implementors
/// Keep native workspace capabilities and the existing agent-loop argument mapping.
pub trait Host: Send + Sync + 'static {
    /// Executable native tools for this workspace.
    type Registry: Send + Sync;
    /// Assemble runtime-aware system instructions.
    fn system_prompt(input: SystemPromptInput<'_>) -> String;
    /// Construct capabilities and advertised tools for the job.
    fn registry(job: &Job) -> (Arc<Self::Registry>, Vec<ToolDefinition>);
    /// Execute one attempt through the native agent loop.
    fn attempt(
        job: &Job,
        prompts: &(String, String),
        registry: &Arc<Self::Registry>,
        tools: &[ToolDefinition],
    ) -> impl Future<Output = AgentOutcome> + Send;
}
