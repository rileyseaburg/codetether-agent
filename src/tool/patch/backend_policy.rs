//! Central runtime-policy check for direct patch backends.

use crate::tool::ToolResult;
use serde_json::Value;

pub(super) async fn blocked(args: &Value) -> Option<ToolResult> {
    let tool = crate::tool::alias::policy_id("apply_patch");
    crate::runtime_policy::evaluate_tool_invocation(&tool, args).await
}
