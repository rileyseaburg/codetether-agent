use super::types::{CallToolResult, ToolContent};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;

#[path = "tool_policy_names.rs"]
mod names;

pub(super) fn call_registered(
    name: String,
    tool: Arc<dyn Tool>,
    args: Value,
) -> Result<CallToolResult> {
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            if let Some(blocked) =
                crate::runtime_policy::evaluate_tool_invocation(&name, &args).await
            {
                return Ok(blocked);
            }
            tool.execute(args).await
        })
    });
    Ok(from_tool_result(result.unwrap_or_else(|error| {
        ToolResult::error(error.to_string())
    })))
}

pub(super) fn blocked(name: &str, args: &Value) -> Option<CallToolResult> {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async {
                crate::runtime_policy::evaluate_tool_invocation(names::policy_name(name), args)
                    .await
            })
            .map(from_tool_result)
    })
}

fn from_tool_result(result: ToolResult) -> CallToolResult {
    CallToolResult {
        content: vec![ToolContent::Text {
            text: result.output,
        }],
        is_error: !result.success,
    }
}
