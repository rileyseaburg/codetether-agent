//! Computer use tool for OS-level GUI automation
//!
//! Provides native desktop automation capabilities including app discovery,
//! screen capture, and input simulation. Currently supports Windows only.

pub mod input;
pub mod response;
pub mod schema;

include!("platform_modules.rs");

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct ComputerUseTool;

impl ComputerUseTool {
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
        "Native Windows desktop automation and WinRT OCR in an isolated worker. Start with snapshot/list_apps or ocr_status. Snapshots retain original pixels on disk and attach bounded previews: use preview scaling for coordinates. Physical keyboard input requires an explicit foreground hwnd. input_mode=shadow queues HWND messages without physical fallback. Worker failures are not retried because effects may be uncertain."
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
