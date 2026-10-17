//! Tool prioritization for the FunctionGemma prompt.

use codetether_provider_core::ToolDefinition;

/// Common action tools that models often describe without naming explicitly.
const ACTION_TOOLS: &[&str] = &["bash", "read", "write", "grep", "list", "search"];

/// Order tools so the most likely candidates receive full parameter schemas.
///
/// `build_functiongemma_prompt` only serializes the first few tools in full, so
/// tools named in the text rank first, then common action tools, then the rest.
///
/// # Arguments
/// * `assistant_text` - Assistant output describing intended tool use.
/// * `tools` - Tool definitions in candidate order.
///
/// # Returns
/// Copied definitions sorted by the existing relevance rules.
///
/// # Examples
/// ```
/// # use codetether_provider_core::{ToolDefinition, CompletionResponse};
/// # fn example(assistant_text: &str, tools: &[ToolDefinition]) {
/// let _result = codetether_cognition_core::tool_routing::prioritize(assistant_text, tools);
/// # }
/// ```
pub fn prioritize(assistant_text: &str, tools: &[ToolDefinition]) -> Vec<ToolDefinition> {
    let text_lower = assistant_text.to_lowercase();
    let mut sorted = tools.to_vec();
    sorted.sort_by_key(|t| {
        let name_lower = t.name.to_lowercase();
        if text_lower.contains(&name_lower) {
            0
        } else if ACTION_TOOLS.iter().any(|a| name_lower.contains(a)) {
            1
        } else {
            2
        }
    });
    sorted
}
