//! Argument validation for the `write` tool, including the temp-path ban.

use super::super::ToolResult;
use serde_json::{Value, json};

/// A validated `write` request.
#[derive(Debug)]
pub(super) struct WriteArgs<'a> {
    pub path: &'a str,
    pub content: &'a str,
}

impl<'a> WriteArgs<'a> {
    /// Parse and validate `args`, rejecting temp-directory targets.
    ///
    /// # Errors
    ///
    /// Returns a structured [`ToolResult`] when a field is missing or the
    /// target path is inside a banned temporary directory.
    pub(super) fn parse(args: &'a Value) -> Result<Self, ToolResult> {
        let path = required(args, "path")?;
        let content = required(args, "content")?;
        if let Some(blocked) = crate::tool::temp_write_guard::denied_result("write", path) {
            return Err(blocked);
        }
        Ok(Self { path, content })
    }
}

fn required<'a>(args: &'a Value, field: &str) -> Result<&'a str, ToolResult> {
    args[field].as_str().ok_or_else(|| {
        ToolResult::structured_error(
            "INVALID_ARGUMENT",
            "write",
            &format!("{field} is required"),
            Some(vec![field]),
            Some(json!({"path": "src/example.rs", "content": "// file content"})),
        )
    })
}

#[cfg(test)]
#[path = "file_write_args_tests.rs"]
mod tests;
