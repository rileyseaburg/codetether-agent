//! Application lookup, runtime policy and parent-goal accounting.
use codetether_tool_core::Tool;
use std::sync::Arc;
/// Application operations used by the shared sub-agent loop.
/// # Implementors
/// Supply the existing registry lookup, tool denial and parent usage accounting.
pub trait Host:
    codetether_provider_step::controlled::Accounting
    + codetether_swarm_core::agent_loop::tools::input::Policy
    + Send
    + Sync
    + 'static
{
    /// Shared registry retained for the entire sub-agent loop.
    type Registry: Send + Sync;
    /// Look up a tool without changing registry ownership.
    fn get(registry: &Self::Registry, name: &str) -> Option<Arc<dyn Tool>>;
}
