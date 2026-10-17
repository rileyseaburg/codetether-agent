//! Validate, preview, and dispatch the confirmation decision.
use super::{ToolResult, input::Input};
use anyhow::Result;
use serde_json::Value;
use tokio::fs;
pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let input = match Input::parse(&args) {
        Ok(input) => input,
        Err(error) => return Ok(error),
    };
    let content = fs::read_to_string(&input.path).await?;
    if let Some(error) = super::validate::validate(&input, &content) {
        return Ok(error);
    }
    let new_content = content.replacen(input.old_string.as_str(), &input.new_string, 1);
    let preview = super::diff::preview(&content, &new_content);
    if input.confirm.is_none() {
        return Ok(super::preview::result(&input, &preview));
    }
    if input.confirm == Some(true) {
        super::apply::apply(&input, &content, &new_content, &preview).await
    } else {
        Ok(ToolResult::success("✗ Changes rejected by user"))
    }
}
