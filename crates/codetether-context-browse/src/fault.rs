//! Attach shared fault codes and descriptions to failed browse requests.
use codetether_context_policy::fault::Fault;
use codetether_tool_core::ToolResult;
use serde_json::json;

pub(super) fn fault_result(fault: Fault, output: impl Into<String>) -> ToolResult {
    let code = fault.code();
    let detail = fault.to_string();
    ToolResult::error(output)
        .with_metadata("fault_code", json!(code))
        .with_metadata("fault_detail", json!(detail))
}
