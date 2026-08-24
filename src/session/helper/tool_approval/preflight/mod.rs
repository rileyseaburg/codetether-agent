//! Side-effect-free input validation before approval execution.

mod files;
mod report;

use crate::tool::ToolResult;
use serde_json::Value;
use std::path::Path;

pub(super) async fn blocked(workspace: &Path, tool: &str, args: &Value) -> Option<ToolResult> {
    let files = match files::collect(workspace, tool, args) {
        Ok(files) => files,
        Err(error) => return Some(report::invalid_input(tool, &error)),
    };
    let _ = files;
    None
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
