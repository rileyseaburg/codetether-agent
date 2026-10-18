//! Application callbacks for provider resolution and read-only execution.
use crate::Run;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::agent_retry::AgentLoopExit;
use std::{future::Future, sync::Arc};
/// Operations bound to the application's provider and agent-loop implementation.
/// # Implementors
/// Hosts must build the same read-only registry used by their agent loop.
pub trait Host {
    /// Loaded provider registry, borrowed for model resolution.
    type Registry: Sync;
    /// Executable read-only tools.
    type Tools: Send + Sync;
    /// Resolve a selector using the installed provider registry.
    /// # Errors
    /// Returns the existing registry's model-resolution error.
    fn resolve(
        registry: &Self::Registry,
        selector: &str,
    ) -> anyhow::Result<(Arc<dyn Provider>, String)>;
    /// Construct the read-only tool surface.
    fn tools() -> Arc<Self::Tools>;
    /// Advertise the same tools passed to execution.
    fn definitions(tools: &Self::Tools) -> Vec<ToolDefinition>;
    /// Execute a bounded reviewer agent.
    /// # Errors
    /// Returns the native loop's execution error.
    fn execute(
        run: Run<'_, Self::Tools>,
    ) -> impl Future<Output = anyhow::Result<(String, usize, usize, AgentLoopExit)>> + Send;
}
