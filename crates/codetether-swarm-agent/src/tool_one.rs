//! Full lifecycle of one requested sub-agent tool call.

use super::state::{State, ToolCall};
pub(super) use codetether_swarm_core::agent_loop::tools::Completed;
mod host;

pub(super) async fn execute<H: crate::Host>(state: &mut State<H>, call: ToolCall) -> Completed {
    codetether_swarm_core::agent_loop::tools::call::execute(state, call).await
}
