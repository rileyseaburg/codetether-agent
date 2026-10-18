//! Native tool-registry specialization of shared sub-agent execution state.
use codetether_swarm_core::agent_loop::AgentState;

/// Turn state retaining the application's executable tool registry.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_swarm_agent::Host>(inner: codetether_swarm_core::agent_loop::AgentState<H::Registry>) {
/// let _state = codetether_swarm_agent::state::State::<H>::from(inner);
/// # }
/// ```
pub struct State<H: crate::Host> {
    /// Shared conversation, limits, channels and registry ownership.
    pub inner: AgentState<H::Registry>,
}
impl<H: crate::Host> From<AgentState<H::Registry>> for State<H> {
    fn from(inner: AgentState<H::Registry>) -> Self {
        Self { inner }
    }
}
pub use codetether_swarm_core::agent_loop::ToolCall;
