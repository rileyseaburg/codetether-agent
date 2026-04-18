//! Pending-confirmation helpers for tool results.
//!
//! Edit tools such as `edit`, `multiedit`, and `write` emit *preview* results
//! that must be explicitly confirmed before being applied to disk. This module
//! provides the small set of helpers that decide whether a tool result
//! requires confirmation, how to render the user-facing status message, and
//! how to auto-apply the confirmation when the session has
//! [`SessionMetadata::auto_apply_edits`](super::super::SessionMetadata::auto_apply_edits)
//! enabled.

use anyhow::Result;
use std::collections::HashMap;

use super::edit::build_pending_confirmation_apply_request;
use crate::tool::Tool;
use crate::tool::confirm_edit::ConfirmEditTool;
use crate::tool::confirm_multiedit::ConfirmMultiEditTool;

/// Reminder appended to tool output when `auto_apply_edits` is **off** and the
/// tool produced only a preview.
pub(crate) fn pending_confirmation_tool_result_content(
    tool_name: &str,
    content: &str,
) -> String {
    format!(
        "{content}\n\nStatus: Pending confirmation only. `{tool_name}` has NOT been applied yet. \
         Auto-apply is off. Enable it in TUI Settings or with `/autoapply on` if you want pending \
         edit previews to be confirmed automatically."
    )
}

/// Banner prefix shown when `auto_apply_edits` is **on** and a pending change
/// has been automatically confirmed.
pub(crate) fn auto_apply_pending_confirmation_result_content(
    output: &str,
    success: bool,
) -> String {
    let status = if success {
        "TUI edit auto-apply is enabled. The pending change was automatically confirmed and applied."
    } else {
        "TUI edit auto-apply is enabled, but confirming the pending change failed."
    };
    format!("{status}\n\n{output}")
}

/// Inspects tool metadata for the `requires_confirmation` boolean flag.
pub(crate) fn tool_result_requires_confirmation(
    tool_metadata: Option<&HashMap<String, serde_json::Value>>,
) -> bool {
    tool_metadata
        .and_then(|metadata| metadata.get("requires_confirmation"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}

/// Automatically apply a pending-confirmation edit when the session has
/// `auto_apply_edits` enabled.
///
/// Returns:
/// * `Ok(Some((content, success, metadata)))` when the pending edit was
///   resolved (either successfully applied or explicitly failed).
/// * `Ok(None)` when the tool call cannot be auto-confirmed (the caller should
///   render the pending-confirmation banner instead).
/// * `Err(_)` when the underlying confirm tool returned an error.
pub(crate) async fn auto_apply_pending_confirmation(
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
