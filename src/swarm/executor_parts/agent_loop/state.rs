//! Native policy specialization for sub-agent integration tests.
pub(super) type State = codetether_swarm_agent::state::State<super::application::Native>;
pub(super) use codetether_swarm_agent::state::ToolCall;
