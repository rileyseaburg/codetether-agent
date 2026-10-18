//! Interpret the preview confirmation flag supplied by edit tools.
use std::collections::HashMap;

/// Inspects tool metadata for the `requires_confirmation` boolean flag.
/// # Arguments
/// Receives optional metadata returned by a tool.
/// # Returns
/// True only when requires_confirmation is a boolean true.
/// # Examples
/// ```
/// assert!(!codetether_edit_confirmation::tool_result_requires_confirmation(None));
/// ```
pub fn tool_result_requires_confirmation(
    tool_metadata: Option<&HashMap<String, serde_json::Value>>,
) -> bool {
    tool_metadata
        .and_then(|metadata| metadata.get("requires_confirmation"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}
