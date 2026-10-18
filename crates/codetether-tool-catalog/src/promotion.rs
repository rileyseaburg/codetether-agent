//! Activate an explicitly requested schema without duplicating advertised tools.
use codetether_provider_core::ToolDefinition;

/// Add an explicitly requested schema to the advertised tool list once.
/// # Arguments
/// `tools` is the full catalog; `advertised` is the current advertised subset;
/// `input` supplies an exact, case-insensitive tool name under `tool`.
/// # Returns
/// Updates `advertised` when the requested tool exists and is not already present.
/// # Examples
/// ```
/// let schema = codetether_tool_catalog::list_tools_bootstrap_definition();
/// let mut advertised = vec![];
/// codetether_tool_catalog::promote(&[schema], &mut advertised,
///     &serde_json::json!({"tool": "LIST_TOOLS"}));
/// assert_eq!(advertised.len(), 1);
/// ```
pub fn promote(
    tools: &[ToolDefinition],
    advertised: &mut Vec<ToolDefinition>,
    input: &serde_json::Value,
) {
    let Some(name) = input
        .get("tool")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
    else {
        return;
    };
    if advertised
        .iter()
        .any(|tool| tool.name.eq_ignore_ascii_case(name))
    {
        return;
    }
    if let Some(tool) = tools
        .iter()
        .find(|tool| tool.name.eq_ignore_ascii_case(name))
        .cloned()
    {
        advertised.push(tool);
    }
}
