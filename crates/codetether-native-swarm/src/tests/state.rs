//! Native policy specialization for sub-agent integration tests.
pub(super) type State =
    codetether_swarm_agent::state::State<crate::AgentHost<super::fixture::Application>>;
pub(super) use codetether_swarm_agent::state::ToolCall;
