//! Parsing and policy enforcement for one sub-agent tool call.

use super::state::State;
use serde_json::Value;

pub(super) async fn prepare<H: crate::Host>(
    state: &State<H>,
    name: &str,
    raw: &str,
) -> Result<Value, String> {
    codetether_swarm_core::agent_loop::tools::input::prepare::<H>(
        state.inner.working_dir.as_deref(),
        &state.inner.model,
        &state.inner.subtask_id,
        name,
        raw,
    )
    .await
}
