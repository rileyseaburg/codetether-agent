//! Deadline-bound invocation of a registered tool.

use super::state::State;
pub(super) use codetether_swarm_core::agent_loop::tools::raw::RawResult as Result;
use serde_json::Value;

pub(super) async fn execute<H: crate::Host>(state: &State<H>, name: &str, args: Value) -> Result {
    codetether_swarm_core::agent_loop::tools::raw::execute(
        H::get(&state.inner.registry, name),
        name,
        args,
        state.inner.conversation.deadline,
        state.inner.timeout_secs,
    )
    .await
}
