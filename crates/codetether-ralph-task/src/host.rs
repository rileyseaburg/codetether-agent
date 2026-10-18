//! Native relay and command-policy verification bindings.
use crate::*;
/// Native services used by a parallel story task.
/// # Implementors
/// Bind the existing provider registry, agent loop, relay and shell policy.
#[async_trait::async_trait]
pub trait Host: Send + Sync + Sized + 'static {
    /// Provider registry retained by each task and borrowed for relay execution.
    type Registry: Send + Sync + 'static;
    /// Native prompt, tool registry and agent loop binding.
    type Agent: codetether_ralph_agent::Host;
    /// Run the existing relay implementation with this task's borrowed inputs.
    /// # Errors
    /// Returns relay preparation or execution failures.
    async fn relay(
        context: &Context<'_, Self>,
        registry: &Arc<Self::Registry>,
        prompt: &str,
    ) -> anyhow::Result<String>;
    /// Run configured quality commands with the native command policy.
    async fn check(directory: &Path, checks: &QualityChecks) -> (bool, String);
    /// Verify explicit story completion criteria through the native command policy.
    /// # Errors
    /// Returns verification failures or command execution errors.
    async fn verify(directory: &Path, story: &UserStory) -> anyhow::Result<()>;
}
