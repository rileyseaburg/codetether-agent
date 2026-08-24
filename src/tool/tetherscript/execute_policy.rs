//! Content-bound approval and sandbox preflight for plugin execution.

#[path = "execute_policy_scope.rs"]
mod scope;

use super::runner::ProcessGrant;
use crate::tool::ToolResult;
use serde_json::Value;
use std::path::Path;

pub(super) async fn authorize(
    raw: &Value,
    root: &Path,
    source: &str,
    enabled: bool,
) -> Result<ProcessGrant, ToolResult> {
    let workspace = root.canonicalize().map_err(|error| {
        ToolResult::error(format!("failed to resolve plugin workspace: {error}"))
    })?;
    let allow_network = crate::tool::network_access::allowed_for(raw);
    let grant = ProcessGrant::new(enabled, workspace.clone(), allow_network);
    if enabled {
        crate::tool::sandbox::sandbox_spawn_std::preflight(&grant.policy(), &workspace)
            .map_err(|error| ToolResult::error(format!("plugin sandbox unavailable: {error}")))?;
    }
    let mut scoped = raw.clone();
    scope::bind(&mut scoped, &workspace, allow_network, source);
    if let Some(blocked) =
        crate::runtime_policy::evaluate_tool_invocation("tetherscript_plugin", &scoped).await
    {
        return Err(blocked);
    }
    crate::approval::use_once::claim("tetherscript_plugin", &scoped).map_err(|error| {
        ToolResult::structured_error(
            "APPROVAL_REQUIRED",
            "tetherscript_plugin",
            &error.to_string(),
            None,
            None,
        )
    })?;
    Ok(grant)
}
