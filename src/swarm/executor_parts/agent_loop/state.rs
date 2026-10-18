//! Native tool-registry specialization of shared sub-agent execution state.
use crate::tool::ToolRegistry;
use codetether_swarm_core::agent_loop::AgentState;

pub(super) struct State {
    pub inner: AgentState<ToolRegistry>,
}
pub(super) use codetether_swarm_core::agent_loop::ToolCall;
