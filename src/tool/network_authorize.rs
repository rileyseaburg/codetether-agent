//! Atomic runtime authorization for direct network-tool execution.

use crate::tool::ToolResult;
use serde_json::Value;

pub(crate) async fn invocation(tool: &str, args: &Value) -> Option<ToolResult> {
    let allowed = crate::tool::network_access::allowed_for(args);
    let mut scoped = args.clone();
    let tool = crate::tool::alias::policy_id(tool);
    crate::tool::network_access::bind(&mut scoped, allowed);
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation(&tool, &scoped).await {
        return Some(blocked);
    }
    crate::approval::use_once::claim(&tool, &scoped)
        .err()
        .map(|error| ToolResult::error(format!("approval claim failed: {error}")))
}

macro_rules! guard {
    ($tool:expr, $args:expr) => {
        if let Some(blocked) = $crate::tool::network_access::invocation($tool, $args).await {
            return Ok(blocked);
        }
    };
}

macro_rules! args {
    ($tool:expr, $args:expr) => {{
        let raw = $args;
        let parsed = serde_json::from_value(raw.clone())?;
        if let Some(blocked) = $crate::tool::network_access::invocation($tool, &raw).await {
            return Ok(blocked);
        }
        parsed
    }};
}


pub(crate) use args;
pub(crate) use guard;

#[cfg(test)]
#[path = "network_authorize_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "network_preflight_tests.rs"]
mod preflight_tests;