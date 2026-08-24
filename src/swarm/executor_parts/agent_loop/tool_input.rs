//! Parsing and policy enforcement for one sub-agent tool call.

use super::{
    super::{path_guard, tool_policy},
    state::State,
};
use crate::provenance::{ExecutionOrigin, ExecutionProvenance};
use crate::session::helper::runtime::enrich_tool_input_with_runtime_context;
use serde_json::Value;

pub(super) async fn prepare(state: &State, name: &str, raw: &str) -> Result<Value, String> {
    if state.registry.get(name).is_none() {
        return Err(format!("Unknown tool: {name}"));
    }
    let mut args = serde_json::from_str(raw).unwrap_or_else(|error| {
        tracing::warn!(tool = %name, %error, arguments = %raw, "Invalid tool arguments");
        serde_json::json!({})
    });
    if let Some(root) = &state.working_dir {
        path_guard::normalize_tool_args(name, &mut args, root)
            .map_err(|error| format!("Tool path policy denied: {error}"))?;
    }
    let agent = format!("agent-{}", state.subtask_id);
    let provenance = ExecutionProvenance::for_operation(&agent, ExecutionOrigin::Swarm);
    let mut enriched = enrich_tool_input_with_runtime_context(
        &args,
        state
            .working_dir
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new(".")),
        Some(&state.model),
        &state.subtask_id,
        &agent,
        Some(&provenance),
    );
    crate::tool::network_access::bind_trusted(&mut enriched, super::network_scope::allowed());
    let workspace = state
        .working_dir
        .as_deref()
        .unwrap_or_else(|| std::path::Path::new("."));
    let enriched = crate::session::helper::runtime::bind_workspace(name, enriched, workspace);
    if let Some(denial) = tool_policy::runtime_denial(name, &enriched).await {
        return Err(denial);
    }
    Ok(enriched)
}
