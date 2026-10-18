//! Dispatch catalog requests to full-schema lookup or compact listing.
use codetether_provider_core::ToolDefinition;
use serde_json::Value;

/// Render a full schema lookup or a filtered, sorted catalog listing.
/// # Arguments
/// `tools` is the available catalog; `tool_input` may contain `tool` or `query`.
/// # Returns
/// Pretty-printed response JSON with a serialization fallback string.
/// # Examples
/// ```
/// let output = codetether_tool_catalog::list_tools_bootstrap_output(
///     &[], &serde_json::json!({})
/// );
/// let value: serde_json::Value = serde_json::from_str(&output).unwrap();
/// assert_eq!(value["count"], 0);
/// ```
pub fn list_tools_bootstrap_output(tools: &[ToolDefinition], tool_input: &Value) -> String {
    let requested_tool = tool_input
        .get("tool")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty());

    if let Some(name) = requested_tool {
        return crate::lookup::lookup(tools, name);
    }
    crate::listing::list(tools, tool_input)
}
