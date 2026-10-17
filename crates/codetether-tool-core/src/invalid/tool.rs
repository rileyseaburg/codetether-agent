//! Executable adapter for unknown-tool feedback.
use super::InvalidTool;
use crate::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
#[async_trait]
impl Tool for InvalidTool {
    fn id(&self) -> &str {
        "invalid"
    }

    fn name(&self) -> &str {
        "Invalid Tool Handler"
    }

    fn description(&self) -> &str {
        "Handles invalid or unknown tool calls. Returns helpful error messages with suggestions for valid tools."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "requested_tool": {
                    "type": "string",
                    "description": "The tool that was requested but not found"
                },
                "args": {
                    "type": "object",
                    "description": "The arguments that were passed"
                }
            },
            "required": ["requested_tool"]
        })
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        Ok(ToolResult::error(self.message(&args)))
    }
}
