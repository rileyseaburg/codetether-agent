use crate::tool::{ToolResult, tool_output_budget};

use super::task::TetherScriptRunResult;

pub fn from_run(result: TetherScriptRunResult) -> ToolResult {
    match result {
        TetherScriptRunResult::Finished(Ok(outcome)) => ToolResult {
            output: outcome.output,
            success: outcome.success,
            metadata: [("value".to_string(), outcome.value)].into(),
        }
        .truncate_to(tool_output_budget()),
        TetherScriptRunResult::Finished(Err(error)) => {
            ToolResult::error(format!("TetherScript plugin failed: {error}"))
                .truncate_to(tool_output_budget())
        }
        TetherScriptRunResult::Timeout(secs) => ToolResult::error(format!(
            "TetherScript plugin timed out after {secs} second(s)"
        )),
    }
}
