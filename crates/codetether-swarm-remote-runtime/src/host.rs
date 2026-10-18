//! Application bindings for remote provider and tool execution.
use crate::Resources;
use anyhow::Result;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::{agent_retry::AgentOutcome, remote_protocol::RemoteSubtaskPayload};
use std::{future::Future, path::PathBuf, sync::Arc};

/// Supplies application resources and executes the configured agent loop.
/// # Implementors
/// Application hosts bind their provider registry, tool registry, session
/// instructions, and native agent loop. The shared runtime owns sequencing.
pub trait Host {
    /// Registry retained throughout the remote process lifecycle.
    type Registry: Send;
    /// Executable workspace tool registry.
    type Tools: Send + Sync;
    /// Resolve the payload's provider and capability-scoped tools.
    /// # Errors
    /// Returns credential, provider-selection, or resource-setup errors.
    fn resolve(
        payload: &RemoteSubtaskPayload,
        working_dir: &PathBuf,
    ) -> impl Future<Output = Result<Resources<Self::Registry, Self::Tools>>> + Send;
    /// Append application session instructions to the system prompt.
    fn append_session_instructions(prompt: String) -> String;
    /// Invoke the agent with the payload's limits and prepared prompts.
    /// # Errors
    /// Returns errors from the application agent loop.
    #[allow(clippy::too_many_arguments)]
    fn invoke(
        payload: &RemoteSubtaskPayload,
        working_dir: &PathBuf,
        provider: Arc<dyn Provider>,
        tool_defs: Vec<ToolDefinition>,
        tool_registry: Arc<Self::Tools>,
        system_prompt: &str,
        user_prompt: &str,
    ) -> impl Future<Output = AgentOutcome> + Send;
}
