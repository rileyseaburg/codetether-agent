//! Native prompt, registry and loop operations.
use crate::Invocation;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::agent_retry::AgentOutcome;
use std::{future::Future, path::Path, sync::Arc};
/// Application bindings for Ralph agent execution.
/// # Implementors
/// Preserve the native base prompt, tool registry and agent-loop implementation.
pub trait Host: Send + Sync + 'static {
    /// Native executable tools.
    type Registry: Send + Sync;
    /// Build the repository-aware base system prompt.
    fn system_prompt(workspace: &Path) -> String;
    /// Build the provider-aware executable registry.
    fn registry(provider: Arc<dyn Provider>, model: String) -> Arc<Self::Registry>;
    /// Read advertised definitions before autonomous-tool filtering.
    fn definitions(registry: &Self::Registry) -> Vec<ToolDefinition>;
    /// Run one invocation with the supplied limits and events.
    /// # Errors
    /// Returns native agent-loop failures.
    fn execute(
        request: Invocation<'_, Self::Registry>,
    ) -> impl Future<Output = AgentOutcome> + Send;
}
