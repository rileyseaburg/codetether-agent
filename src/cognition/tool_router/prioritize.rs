//! Tool prioritization for the FunctionGemma prompt.

use crate::provider::ToolDefinition;

/// Common action tools that models often describe without naming explicitly.
const ACTION_TOOLS: &[&str] = &["bash", "read", "write", "grep", "list", "search"];

/// Order tools so the most likely candidates receive full parameter schemas.
///
/// `build_functiongemma_prompt` only serializes the first few tools in full, so
/// tools named in the text rank first, then common action tools, then the rest.
pub(super) fn prioritize(assistant_text: &str, tools: &[ToolDefinition]) -> Vec<ToolDefinition> {
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
