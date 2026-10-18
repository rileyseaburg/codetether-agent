//! Native tool-registry specialization of shared sub-agent execution state.
use crate::tool::ToolRegistry;
use codetether_swarm_core::agent_loop::AgentState;

pub(super) struct State {
    pub inner: AgentState<ToolRegistry>,
}
impl From<AgentState<ToolRegistry>> for State {
    fn from(inner: AgentState<ToolRegistry>) -> Self {
        Self { inner }
    }
}
pub(super) use codetether_swarm_core::agent_loop::ToolCall;
