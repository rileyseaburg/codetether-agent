//! Preserve success/error status and merge-readiness metadata.

use anyhow::Result;
use codetether_ralph_core::RalphState;
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(crate) fn finish(
    output: String,
    state: &RalphState,
    feature_branch: &str,
    all_passed: bool,
    passed_count: usize,
    total_count: usize,
) -> Result<ToolResult> {
    if all_passed {
        Ok(ToolResult::success(output)
            .with_metadata("status", json!(format!("{:?}", state.status)))
            .with_metadata("passed", json!(passed_count))
            .with_metadata("total", json!(total_count))
            .with_metadata("feature_branch", json!(feature_branch))
            .with_metadata("all_passed", json!(true))
            .with_metadata("ready_to_merge", json!(true)))
    } else {
        Ok(ToolResult::error(output)
            .with_metadata("status", json!(format!("{:?}", state.status)))
            .with_metadata("passed", json!(passed_count))
            .with_metadata("total", json!(total_count))
            .with_metadata("feature_branch", json!(feature_branch))
            .with_metadata("all_passed", json!(false))
            .with_metadata("ready_to_merge", json!(false)))
    }
}
