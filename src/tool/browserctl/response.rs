use crate::browser::{BrowserError, BrowserOutput};
use crate::tool::ToolResult;
use serde_json::json;
use std::collections::HashMap;

pub(super) fn error_result(error: BrowserError) -> ToolResult {
    ToolResult::error(error.to_string()).with_metadata("browser_error", json!(error.to_string()))
}

pub(super) async fn success_result(
    input: &super::input::BrowserCtlInput,
    output: BrowserOutput,
) -> anyhow::Result<ToolResult> {
    let mut metadata = HashMap::new();
    let body = match output {
        BrowserOutput::Json(value) => value,
        BrowserOutput::Ack(value) => serde_json::to_value(value)?,
        BrowserOutput::Eval(value) => serde_json::to_value(value)?,
        BrowserOutput::Html(value) => serde_json::to_value(value)?,
        BrowserOutput::Snapshot(value) => {
            metadata.insert("url".into(), json!(value.url));
            metadata.insert("title".into(), json!(value.title));
            serde_json::to_value(value)?
        }
        BrowserOutput::Tabs(value) => serde_json::to_value(value)?,
        BrowserOutput::Text(value) => serde_json::to_value(value)?,
        BrowserOutput::Toggle(value) => serde_json::to_value(value)?,
        BrowserOutput::Screenshot(value) => {
            super::screenshot::write(input, value, &mut metadata).await?
        }
    };
    Ok(ToolResult {
        output: serde_json::to_string_pretty(&body)?,
        success: true,
        metadata,
    })
}
