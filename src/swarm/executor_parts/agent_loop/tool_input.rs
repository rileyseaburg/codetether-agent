//! Parsing and policy enforcement for one sub-agent tool call.

use super::{super::tool_policy, state::State};
use serde_json::Value;

struct Native;
impl codetether_swarm_core::agent_loop::tools::input::Policy for Native {
    async fn denial(name: &str, args: &Value) -> Option<String> {
        tool_policy::runtime_denial(name, args).await
    }
}

pub(super) async fn prepare(state: &State, name: &str, raw: &str) -> Result<Value, String> {
    codetether_swarm_core::agent_loop::tools::input::prepare::<Native>(
        state.inner.working_dir.as_deref(),
        &state.inner.model,
        &state.inner.subtask_id,
        name,
        raw,
    )
    .await
}
