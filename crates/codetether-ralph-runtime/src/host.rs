//! Native services retained outside shared Ralph state ownership.
use codetether_ralph_core::{UserStory, verification::ShellExecutor};
use std::path::Path;
/// Bind providers, session turns, tools and command policy to [crate::RalphLoop].
/// # Implementors
/// Application adapters provide existing runtime services without owning the run state.
#[async_trait::async_trait]
pub trait Host: Send + Sync + 'static {
    /// Shared native provider registry.
    type Registry: Send + Sync + 'static;
    /// Existing prompt, tool registry and agent-loop binding.
    type Agent: codetether_ralph_agent::Host;
    /// Existing session construction and prompting with the same provider registry.
    type Relay: codetether_ralph_relay::Host<Registry = Self::Registry>;
    /// Existing Git invocation-policy evaluator.
    type Git: codetether_ralph_git::Host;
    /// Stateless native shell executor used by quality gates.
    type Quality: ShellExecutor + Default + Send + 'static;
    /// Verify explicit story completion through native command policy.
    /// # Errors
    /// Returns verification failures or native command execution errors.
    async fn verify(root: &Path, story: &UserStory) -> anyhow::Result<()>;
}
