//! Execute a supported explicit confirmation request and merge result metadata.
use crate::auto_apply_pending_confirmation_result_content;
use anyhow::Result;
use codetether_confirm_edit::ConfirmEditTool;
use codetether_confirm_multiedit::ConfirmMultiEditTool;
use codetether_edit_policy::build_pending_confirmation_apply_request;
use codetether_tool_core::Tool;
use std::collections::HashMap;

/// Automatically apply a pending-confirmation edit when the session has
/// `auto_apply_edits` enabled.
///
/// Returns:
/// * `Ok(Some((content, success, metadata)))` when the pending edit was
///   resolved (either successfully applied or explicitly failed).
/// * `Ok(None)` when the tool call cannot be auto-confirmed (the caller should
///   render the pending-confirmation banner instead).
/// * `Err(_)` when the underlying confirm tool returned an error.
/// # Arguments
/// Receives a tool name, original input and optional preview metadata. The caller must already have authorized auto-application.
/// # Returns
/// The formatted result, success and confirmation metadata; None for unsupported tools.
/// # Errors
/// Propagates errors returned by the selected confirmation tool.
/// # Examples
/// ```no_run
/// // Unsupported tools do not execute a confirmation.
/// # async fn example() -> anyhow::Result<()> {
/// assert!(codetether_edit_confirmation::auto_apply_pending_confirmation("read", &serde_json::json!({}), None).await?.is_none());
/// # Ok(())
/// # }
/// ```
pub async fn auto_apply_pending_confirmation(
    tool_name: &str,
    tool_input: &serde_json::Value,
    tool_metadata: Option<&HashMap<String, serde_json::Value>>,
) -> Result<Option<(String, bool, Option<HashMap<String, serde_json::Value>>)>> {
    let Some((confirm_tool_name, confirm_input)) =
        build_pending_confirmation_apply_request(tool_name, tool_input, tool_metadata)
    else {
        return Ok(None);
    };

    let result = match confirm_tool_name.as_str() {
        "confirm_edit" => ConfirmEditTool::new().execute(confirm_input).await?,
        "confirm_multiedit" => ConfirmMultiEditTool::new().execute(confirm_input).await?,
        _ => return Ok(None),
    };

    let metadata = if result.metadata.is_empty() {
        tool_metadata.cloned()
    } else {
        Some(result.metadata)
    };

    Ok(Some((
        auto_apply_pending_confirmation_result_content(&result.output, result.success),
        result.success,
        metadata,
    )))
}
