//! Fail-closed atomic claiming for approval-bearing tool invocations.

use super::ApprovalStore;

pub(crate) fn claim(tool: &str, args: &serde_json::Value) -> anyhow::Result<()> {
    let Some(approval_id) = args
        .get("approval_id")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return Ok(());
    };
    let scope = crate::runtime_policy::invocation_scope::for_tool(tool, args);
    ApprovalStore::open_default()?.claim(
        approval_id,
        tool,
        scope.action,
        &scope.resource,
        "tool-runtime",
    )?;
    Ok(())
}
