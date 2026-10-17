//! Tool-definition serialization for the FunctionGemma prompt.

use codetether_provider_core::ToolDefinition;

/// Full parameter schemas are sent only for this many tools, to keep prompt
/// tokens low for the 270M model.
const DETAILED_TOOLS: usize = 5;

/// One-line `name: description` summaries for every tool.
///
/// # Arguments
/// * `tools` - Tool definitions in candidate order.
///
/// # Returns
/// The formatted or selected result described above.
///
/// # Examples
/// ```
/// # use codetether_provider_core::{ToolDefinition, CompletionResponse};
/// # fn example(tools: &[ToolDefinition]) {
/// let _result = codetether_cognition_core::tool_routing::tool_lines(tools);
/// # }
/// ```
pub fn tool_lines(tools: &[ToolDefinition]) -> String {
    tools
        .iter()
        .map(|t| format!("- {}: {}", t.name, t.description))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Pretty-printed JSON definitions for the most likely candidate tools.
///
/// # Arguments
/// * `tools` - Tool definitions in candidate order.
///
/// # Returns
/// The formatted or selected result described above.
///
/// # Examples
/// ```
/// # use codetether_provider_core::{ToolDefinition, CompletionResponse};
/// # fn example(tools: &[ToolDefinition]) {
/// let _result = codetether_cognition_core::tool_routing::tools_json(tools);
/// # }
/// ```
pub fn tools_json(tools: &[ToolDefinition]) -> String {
    let detailed: Vec<serde_json::Value> = tools
        .iter()
        .take(DETAILED_TOOLS)
        .map(|t| {
            serde_json::json!({
                "name": t.name,
                "description": t.description,
                "parameters": t.parameters,
            })
        })
        .collect();
    serde_json::to_string_pretty(&detailed).unwrap_or_else(|_| "[]".to_string())
}
