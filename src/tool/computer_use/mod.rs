//! Computer use tool for OS-level GUI automation
//!
//! Provides native desktop automation capabilities including app discovery,
//! screen capture, and input simulation. Currently supports Windows only.

pub mod input;
pub mod response;
pub mod schema;

mod platform;

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
        "Native Windows desktop automation. ALWAYS prefer this over bash/powershell for screenshots, clicking, typing, and window management. Actions: snapshot (saves JPEG to temp file), window_snapshot (specific window), list_apps, bring_to_front, click, right_click, double_click, drag, type_text, press_key, scroll, wait_ms."
    }

    fn parameters(&self) -> Value {
        schema::parameters_schema()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let input: input::ComputerUseInput =
            serde_json::from_value(args).context("Invalid computer_use args")?;

        platform::dispatch(&input).await
    }
}
