//! Batch result aggregation and metadata preservation.

use super::ToolResult;
use serde_json::json;

pub(super) fn build(results: Vec<(usize, String, ToolResult)>) -> ToolResult {
    let mut output_parts = Vec::new();
    let mut call_metadata = Vec::new();
    let mut success_count = 0;
    let mut error_count = 0;
    for (idx, tool_id, result) in results {
        let marker = if result.success {
            success_count += 1;
            "✓"
        } else {
            error_count += 1;
            "✗"
        };
        if !result.metadata.is_empty() {
            call_metadata.push(json!({
                "index": idx,
                "tool": tool_id,
                "success": result.success,
                "metadata": result.metadata,
            }));
        }
        output_parts.push(format!(
            "[{}] {} {}:\n{}",
            idx + 1,
            marker,
            tool_id,
            result.output
        ));
    }
    let summary = format!(
        "Batch complete: {} succeeded, {} failed\n\n{}",
        success_count,
        error_count,
        output_parts.join("\n\n")
    );
    let mut result = if error_count == 0 {
        ToolResult::success(summary).with_metadata("success_count", json!(success_count))
    } else {
        ToolResult::error(summary).with_metadata("error_count", json!(error_count))
    };
    if !call_metadata.is_empty() {
        result = result.with_metadata("calls", json!(call_metadata));
    }
    result
}

#[cfg(test)]
#[path = "batch_summary_tests.rs"]
mod tests;
