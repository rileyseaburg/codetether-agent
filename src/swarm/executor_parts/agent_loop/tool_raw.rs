//! Deadline-bound invocation of a registered tool.

use super::state::State;
pub(super) use codetether_swarm_core::agent_loop::tools::raw::RawResult as Result;
use serde_json::Value;

pub(super) async fn execute(state: &State, name: &str, args: Value) -> Result {
    codetether_swarm_core::agent_loop::tools::raw::execute(
        state.registry.get(name),
        name,
        args,
        state.deadline,
        state.timeout_secs,
    )
    .await
}
