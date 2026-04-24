use crate::tool::{ToolResult, tool_output_budget};

use super::task::KilnRunResult;

pub fn from_run(result: KilnRunResult) -> ToolResult {
    match result {
        KilnRunResult::Finished(Ok(outcome)) => ToolResult {
            output: outcome.output,
            success: outcome.success,
            metadata: [("value".to_string(), outcome.value)].into(),
        }
        .truncate_to(tool_output_budget()),
        KilnRunResult::Finished(Err(error)) => {
            ToolResult::error(format!("Kiln plugin failed: {error}"))
                .truncate_to(tool_output_budget())
        }
        KilnRunResult::Timeout(secs) => {
            ToolResult::error(format!("Kiln plugin timed out after {secs} second(s)"))
        }
    }
}
