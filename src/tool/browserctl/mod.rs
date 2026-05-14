mod actions;
mod dispatch;
mod helpers;
mod input;
mod response;
mod schema;
mod screenshot;

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct BrowserCtlTool;

impl BrowserCtlTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BrowserCtlTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserCtlTool {
    fn id(&self) -> &str {
        "browserctl"
    }
    fn name(&self) -> &str {
        "Browser Control"
    }
    fn description(&self) -> &str {
        "Control the browser session for navigation, DOM inspection, evaluation, screenshots, tabs, and robust interaction with modern web apps."
    }
    fn parameters(&self) -> Value {
        schema::parameters_schema()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let input: input::BrowserCtlInput =
            serde_json::from_value(args).context("Invalid browserctl args")?;
        if matches!(&input.action, input::BrowserCtlAction::Detect) {
            return Ok(detect_result());
        }
        let result = match dispatch::dispatch(&input).await {
            Ok(output) => response::success_result(&input, output).await?,
            Err(error) => response::error_result(error),
        };
        if matches!(&input.action, input::BrowserCtlAction::Stop) && result.success {
            crate::browser::browser_service().clear();
        }
        Ok(result)
    }
}

fn detect_result() -> ToolResult {
    match crate::browser::detect_browser() {
        Some(path) => {
            let output = serde_json::json!({
                "found": true,
                "executable_path": path.display().to_string(),
                "platform": std::env::consts::OS,
            });
            ToolResult {
                output: serde_json::to_string_pretty(&output).unwrap_or_default(),
                success: true,
                metadata: Default::default(),
            }
        }
        None => {
            let output = serde_json::json!({
                "found": false,
                "platform": std::env::consts::OS,
                "hint": "No external browser executable is required; browserctl uses the native TetherScript backend.",
            });
            ToolResult {
                output: serde_json::to_string_pretty(&output).unwrap_or_default(),
                success: true,
                metadata: Default::default(),
            }
        }
    }
}
