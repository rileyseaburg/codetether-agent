//! Sequential execution of tool calls from one provider turn.

use super::state::{State, ToolCall};
mod host;

pub(super) async fn execute(state: &mut State, calls: Vec<ToolCall>) -> anyhow::Result<bool> {
    codetether_swarm_core::agent_loop::tools::execute(state, calls).await
}
