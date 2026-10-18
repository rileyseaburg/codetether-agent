//! Native provider and agent-loop operations for verification.
use super::Run;
use codetether_goal_core::GoalStatus;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::agent_retry::AgentLoopExit;
use std::{future::Future, path::Path, sync::Arc};
/// Execution operations supplied by the native verification runtime.
/// # Implementors
/// Applications preserve provider initialization, verification-only tools and
/// their existing agent loop while the shared runtime records identity/order.
pub trait Host {
    /// Provider registry kept alive for the duration of each review.
    type Providers: Send;
    /// Native executable tool registry.
    type Tools: Send + Sync;
    /// Resolve a model through the existing provider initialization path.
    /// # Errors
    /// Returns provider initialization or model-resolution errors.
    fn resolve(
        requested: &str,
    ) -> impl Future<Output = anyhow::Result<(Self::Providers, Arc<dyn Provider>, String)>> + Send;
    /// Build the verification-only registry rooted at the workspace.
    fn tools(workspace: &Path, provider: &Arc<dyn Provider>, model: &str) -> Arc<Self::Tools>;
    /// Read advertised definitions from the same registry being executed.
    fn definitions(tools: &Self::Tools) -> Vec<ToolDefinition>;
    /// Build a system prompt using the existing native runtime constraints.
    fn system_prompt(workspace: &Path, model: &str, claimed: GoalStatus) -> String;
    /// Run the native agent loop with the supplied prompts, limits and tools.
    /// # Errors
    /// Returns provider or agent-loop setup/execution errors.
    fn execute(
        run: Run<'_, Self::Tools>,
    ) -> impl Future<Output = anyhow::Result<(String, usize, usize, AgentLoopExit)>> + Send;
}
