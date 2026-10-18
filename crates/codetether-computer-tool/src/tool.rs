use crate::{execution, input, schema};
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;

/// Desktop screenshot and native Windows automation tool.
///
/// # Examples
/// ```
/// use codetether_computer_tool::ComputerUseTool;
/// use codetether_tool_core::Tool;
/// assert_eq!(ComputerUseTool::new().id(), "computer_use");
/// ```
pub struct ComputerUseTool;

impl ComputerUseTool {
    /// Construct the computer-use tool.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ComputerUseTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for ComputerUseTool {
    fn id(&self) -> &str {
        "computer_use"
    }

    fn name(&self) -> &str {
        "Computer Use"
    }

    fn description(&self) -> &str {
        "Desktop screenshots on Linux/Ubuntu (start with status or snapshot); native Windows desktop automation and WinRT OCR in an isolated worker. Linux requires a graphical session and an installed capture utility; input and OCR are Windows-only. Snapshots retain original pixels on disk and attach bounded previews: use preview scaling for coordinates. Windows physical keyboard input requires an explicit foreground hwnd. input_mode=shadow queues HWND messages without physical fallback. Worker failures are not retried because effects may be uncertain."
    }

    fn parameters(&self) -> Value {
        schema::parameters_schema()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let input: input::ComputerUseInput =
            serde_json::from_value(args).context("Invalid computer_use args")?;

        execution::execute(input).await
    }
}
