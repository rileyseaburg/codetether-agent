//! Backend atomic approval-claim boundary for side-effecting tools.

use crate::tool::ToolResult;
use serde_json::Value;

pub(crate) async fn blocked(tool: &str, args: &Value) -> Option<ToolResult> {
    let policy_id = crate::tool::alias::policy_id(tool);
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation(&policy_id, args).await {
        return Some(blocked);
    }
    match crate::approval::use_once::claim(&policy_id, args) {
        Ok(()) => None,
        Err(error) => Some(ToolResult::structured_error(
            "APPROVAL_CLAIM_FAILED", tool, &error.to_string(), None, None,
        )),
    }
}

macro_rules! guard {
    ($tool:expr, $args:expr) => {
        if let Some(blocked) = $crate::tool::orchestration_gate::blocked($tool, $args).await {
            return Ok(blocked);
        }
    };
}

macro_rules! guarded {
    ($tool:expr, $args:expr, $value:expr) => {{
        if let Some(blocked) = $crate::tool::orchestration_gate::blocked($tool, $args).await {
            return Ok(blocked);
        }
        $value
    }};
}

pub(crate) use guard;
pub(crate) use guarded;

#[cfg(test)]
#[path = "direct_mutation_approval_tests.rs"]
mod tests;