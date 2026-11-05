//! Read-only tool metadata and delegation to focused operation modules.
use anyhow::Result;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
mod catalog;
mod execute;
mod input;
mod run;
#[cfg(test)]
mod tests;

/// Built-in official Monarch connector. Construction performs no I/O.
///
/// # Examples
/// ```
/// use codetether_tool_core::Tool;
/// assert_eq!(codetether_monarch::MonarchTool.id(), "monarch");
/// ```
pub struct MonarchTool;

#[async_trait::async_trait]
impl Tool for MonarchTool {
    fn id(&self) -> &str {
        "monarch"
    }
    fn name(&self) -> &str {
        "Monarch"
    }
    fn description(&self) -> &str {
        "Access authorized Monarch budgeting data through its official MCP server. Use list_tools to discover read-only tools, then call with that tool's arguments. Requires codetether auth monarch login. Never provide passwords or web tokens. Account coverage and usage limits are controlled by Monarch."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "action":{"type":"string","enum":["status","list_tools","call"]},
            "tool":{"type":"string"},"arguments":{"type":"object"}
        },"required":["action"],"additionalProperties":false})
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute::execute(args).await
    }
}
