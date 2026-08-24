//! Review boundary for model-selected network search targets.

use crate::tool::ToolResult;
use serde_json::Value;

pub(super) fn required(tool: &str, args: Value) -> ToolResult {
    ToolResult::structured_error(
        "NESTED_NETWORK_REVIEW_REQUIRED",
        tool,
        &format!(
            "search router selected {tool}; invoke it directly so its network target can be reviewed"
        ),
        None,
        Some(serde_json::json!({"suggested_arguments": args})),
    )
}
