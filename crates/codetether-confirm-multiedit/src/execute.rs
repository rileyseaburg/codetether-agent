//! Parse, validate, preview and honor the optional confirmation decision.
use super::ToolResult;
use anyhow::Result;
use serde_json::Value;
pub(super) async fn execute(input: Value) -> Result<ToolResult> {
    let edits = match super::input::parse(&input) {
        Ok(edits) => edits,
        Err(error) => return Ok(error),
    };
    let confirm = input.get("confirm").and_then(|v| v.as_bool());

    if edits.is_empty() {
        return Ok(ToolResult::error("No edits provided"));
    }

    let file_contents = match super::prepare::load(&edits).await? {
        Ok(files) => files,
        Err(error) => return Ok(error),
    };
    let (previews, total_added, total_removed) = super::prepare::previews(&file_contents);
    if confirm.is_none() {
        return Ok(super::review::result(&previews, total_added, total_removed));
    }
    if confirm == Some(true) {
        super::apply::apply(file_contents, previews.len()).await
    } else {
        Ok(ToolResult::success("✗ All changes rejected by user"))
    }
}
