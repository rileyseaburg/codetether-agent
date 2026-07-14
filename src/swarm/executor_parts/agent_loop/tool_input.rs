//! Parsing and policy enforcement for one sub-agent tool call.

use super::{
    super::{path_guard, tool_policy},
    state::State,
};
use crate::provenance::{ExecutionOrigin, ExecutionProvenance};
use crate::session::helper::runtime::enrich_tool_input_with_runtime_context;
use serde_json::Value;

pub(super) async fn prepare(state: &State, name: &str, raw: &str) -> Result<Value, String> {
    let mut args = serde_json::from_str(raw).unwrap_or_else(|error| {
        tracing::warn!(tool = %name, %error, arguments = %raw, "Invalid tool arguments");
        serde_json::json!({})
    });
    if let Some(root) = &state.working_dir {
        path_guard::normalize_tool_args(name, &mut args, root)
            .map_err(|error| format!("Tool path policy denied: {error}"))?;
    }
    if let Some(denial) = tool_policy::runtime_denial(name, &args).await {
        return Err(denial);
    }
    let agent = format!("agent-{}", state.subtask_id);
    let provenance = ExecutionProvenance::for_operation(&agent, ExecutionOrigin::Swarm);
    Ok(enrich_tool_input_with_runtime_context(
        &args,
        state
            .working_dir
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new(".")),
        Some(&state.model),
        &state.subtask_id,
        &agent,
        Some(&provenance),
    ))
}
