//! Browser tool metadata and action dispatch.
use super::detect::detect_result;
use super::{BrowserCtlTool, Tool, ToolResult, dispatch, input, response, schema};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::Value;

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
            codetether_browser::browser_service().clear();
        }
        Ok(result)
    }
}
